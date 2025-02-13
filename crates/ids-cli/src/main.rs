use clap::Parser;
use ids_core::{
    cli::{Cli, Commands},
    generate_data,
    sampler::IncidenceDensitySampler,
    utils::{configure_logging, load_records, validate_csv_format, MatchingCriteria},
};

use ids_covariates::{
    balance::BalanceChecker,
    loader::CovariateLoader,
    matched_pairs::{is_case, load_matched_pairs},
    storage::CovariateStore,
};
use log::{error, info};
use std::{fs, path::Path, time::Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    // Create output directory if it doesn't exist
    fs::create_dir_all(&cli.output_dir)?;

    // Create log directory
    let log_dir = Path::new(&cli.output_dir).join("log");
    fs::create_dir_all(&log_dir)?;

    // Configure logging
    configure_logging(Some(&format!("{}/cli.log", log_dir.display())))?;

    match &cli.command {
        Commands::Generate {
            output,
            num_records,
            num_cases,
        } => {
            info!("Generating synthetic data...");
            generate_data::generate_pediatric_data(output, *num_records, *num_cases)?;
            info!("Data generation completed. Output saved to: {}", output);
        }
        Commands::Sample {
            input,
            controls,
            birth_window,
            parent_window,
        } => {
            let start = Instant::now();

            info!("Validating input file format...");
            if let Err(e) = validate_csv_format(input) {
                error!("CSV validation failed: {}", e);
                return Err(e.into());
            }

            info!("Reading data from {}...", input);
            let records = load_records(input)?;
            info!("Data loaded in {:?}", start.elapsed());

            let criteria = MatchingCriteria {
                birth_date_window: *birth_window,
                parent_date_window: *parent_window,
            };

            info!("Initializing sampler...");
            let sampler_start = Instant::now();
            let sampler = IncidenceDensitySampler::new(records, criteria)?;
            info!("{}", sampler.get_statistics());
            info!("Sampler initialized in {:?}", sampler_start.elapsed());

            info!("Sampling controls...");
            let sampling_start = Instant::now();
            match sampler.sample_controls(*controls) {
                Ok(case_control_pairs) => {
                    info!(
                        "Sampling completed in {:?}. Found {} matches",
                        sampling_start.elapsed(),
                        case_control_pairs.len()
                    );

                    let quality = sampler.evaluate_matching_quality(&case_control_pairs);
                    info!("{}", quality.format_report());

                    // Generate all plots
                    let plots_dir = Path::new(&cli.output_dir).join("plots");
                    fs::create_dir_all(&plots_dir)?;

                    if let Err(e) = quality.generate_summary_plots(&plots_dir.to_string_lossy()) {
                        error!("Error generating plots: {}", e);
                    }

                    let matches_path = Path::new(&cli.output_dir).join("matched_pairs.csv");
                    if let Err(e) = sampler
                        .save_matches_to_csv(&case_control_pairs, &matches_path.to_string_lossy())
                    {
                        error!("Error saving matches to CSV: {}", e);
                    }

                    let stats_path = Path::new(&cli.output_dir).join("matching_stats.csv");
                    if let Err(e) = sampler.save_matching_statistics(
                        &case_control_pairs,
                        &stats_path.to_string_lossy(),
                    ) {
                        error!("Error saving matching statistics: {}", e);
                    }
                }
                Err(e) => error!("Error sampling controls: {}", e),
            }

            info!("Total execution time: {:?}", start.elapsed());
        }

        Commands::CheckBalance {
            matches_file,
            covariate_dir,
        } => {
            info!("Loading matched pairs from {}...", matches_file);
            let matched_pairs = load_matched_pairs(Path::new(matches_file))?;

            info!("Loading covariate data from {}...", covariate_dir);
            let loader = CovariateLoader::new(
                format!("{}/education.csv", covariate_dir),
                format!("{}/income.csv", covariate_dir),
                format!("{}/occupation.csv", covariate_dir),
            );

            let store = CovariateStore::new();
            store.load_education(loader.load_education()?)?;
            store.load_income(loader.load_income()?)?;
            store.load_occupation(loader.load_occupation()?)?;

            let checker = BalanceChecker::new(store);

            // Extract cases and controls with their index dates
            let (cases, controls): (Vec<_>, Vec<_>) = matched_pairs
                .into_iter()
                .flat_map(|(case_id, case_date, control_ids)| {
                    std::iter::once((case_id, case_date)).chain(
                        control_ids
                            .into_iter()
                            .map(move |control_id| (control_id, case_date)),
                    )
                })
                .partition(|(id, _)| is_case(id));

            info!("Calculating covariate balance...");
            let balance_results = checker.calculate_balance(&cases, &controls)?;

            // Save balance results
            let output_path = Path::new(&cli.output_dir).join("covariate_balance.csv");
            BalanceChecker::save_balance_results(&balance_results, &output_path)?;

            info!("Balance results saved to {}", output_path.display());
        }
    }

    Ok(())
}

use clap::Parser;
use core::{
    sampler::IncidenceDensitySampler,
    utils::{configure_logging, load_records, validate_csv_format, MatchingCriteria},
};
use covariates::{
    balance::BalanceChecker,
    matched_pairs::{is_case, load_matched_pairs},
    storage::CovariateStore,
};
use datagen::{GeneratorConfig, RegisterGenerator};
use loader::ParquetLoader;
use log::{error, info};
use std::{fs, path::Path, time::Instant};
use types::{BaseStore, CombinedStore};

// Import the CLI structs directly from cli.rs
mod cli;
use cli::{Cli, Commands};

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
        Commands::GeneratePediatric {
            output,
            num_records,
            num_cases,
            seed,
        } => {
            info!("Generating synthetic pediatric data...");

            // Create generator with config
            let config = GeneratorConfig::new(*num_records, *num_cases, cli.output_dir);
            let config = if let Some(seed_value) = seed {
                config.with_seed(*seed_value)
            } else {
                config
            };

            let mut generator = RegisterGenerator::new(config)?;

            // Generate pediatric data
            generator.generate_pediatric(output)?;
            info!(
                "Pediatric data generation completed. Output saved to: {}",
                output
            );
        }
        Commands::GenerateRegisters {
            output_dir,
            num_records,
            num_cases,
            start_year,
            end_year,
            seed,
        } => {
            info!("Generating synthetic register data...");
            let config = GeneratorConfig::new(*num_records, *num_cases, output_dir.clone())
                .with_year_range(*start_year, *end_year);

            let config = if let Some(seed_value) = seed {
                config.with_seed(*seed_value)
            } else {
                config
            };

            let mut generator = RegisterGenerator::new(config)?;

            info!("Starting data generation for all registers...");
            generator.generate_all()?;
            info!("Register data generation completed in: {}", output_dir);
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
                return Err(e);
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
            let arrow_store = ParquetLoader::new().load_from_path(covariate_dir.clone())?;
            let store = CombinedStore::new(BaseStore::new(), arrow_store);
            let store = CovariateStore::with_store(Box::new(store));

            let checker = BalanceChecker::new(store);

            // Extract cases and controls with their index dates
            let (cases, controls): (Vec<_>, Vec<_>) = matched_pairs
                .into_iter()
                .flat_map(|(case_id, case_date, control_ids)| {
                    std::iter::once((case_id.clone(), case_date)).chain(
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
            BalanceChecker::save_balance_results(&balance_results.summaries, &output_path)?;

            info!("Balance results saved to {}", output_path.display());
        }
    }

    Ok(())
}

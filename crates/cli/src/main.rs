use chrono::NaiveDate;
use clap::Parser;
use core::{
    sampler::IncidenceDensitySampler,
    utils::{configure_logging, load_records, validate_csv_format, MatchingCriteria},
};
use covariates::{balance::BalanceChecker, matched_pairs::load_matched_pairs};
use datagen::{GeneratorConfig, RegisterGenerator};
use loader::ParquetLoader;
use log::{error, info, warn};
use std::collections::HashSet;
use std::{fs, path::Path, time::Instant};
use types::models::CovariateType;

mod cli;
use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    setup_directories(&cli.output_dir)?;
    configure_logging_with_dir(&cli.output_dir)?;

    match &cli.command {
        Commands::GenerateRegisters {
            output_dir,
            num_records,
            num_cases,
            start_year,
            end_year,
            seed,
        } => handle_generate_registers(
            output_dir,
            *num_records,
            *num_cases,
            *start_year,
            *end_year,
            *seed,
        ),
        Commands::Sample {
            input,
            controls,
            birth_window,
            parent_window,
        } => handle_sampling(
            input,
            *controls,
            *birth_window,
            *parent_window,
            &cli.output_dir,
        ),
        Commands::CheckBalance {
            matches_file,
            covariate_dir,
        } => handle_balance_check(matches_file, covariate_dir, &cli.output_dir),
    }
}

fn setup_directories(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create output directory and log directory
    fs::create_dir_all(output_dir)?;
    fs::create_dir_all(Path::new(output_dir).join("log"))?;

    // Create register subdirectories if needed
    for dir in ["akm", "bef", "ind", "uddf"] {
        fs::create_dir_all(Path::new(output_dir).join(dir))?;
    }

    Ok(())
}

fn configure_logging_with_dir(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = format!("{}/log/cli.log", output_dir);
    configure_logging(Some(&log_path))?;
    Ok(())
}

fn handle_generate_registers(
    output_dir: &str,
    num_records: usize,
    num_cases: usize,
    start_year: i32,
    end_year: i32,
    seed: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Generating synthetic register data...");
    let mut config = GeneratorConfig::new(num_records, num_cases, output_dir.to_string())
        .with_year_range(start_year, end_year);
    if let Some(s) = seed {
        config = config.with_seed(s);
    }

    let mut generator = RegisterGenerator::new(config)?;
    generator.generate_all()?;

    // Use a proper path for the pediatric data
    let pediatric_path = Path::new(output_dir).join("pediatric.csv");
    generator.generate_pediatric(pediatric_path.to_str().unwrap())?;

    info!("Register data generation completed");
    Ok(())
}

fn validate_and_load_data(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    info!("Validating input file format...");
    if let Err(e) = validate_csv_format(input) {
        error!("CSV validation failed: {}", e);
        return Err(e);
    }
    Ok(())
}

fn create_sampler(
    input: &str,
    criteria: MatchingCriteria,
) -> Result<IncidenceDensitySampler, Box<dyn std::error::Error>> {
    info!("Reading data from {}...", input);
    let start = Instant::now();
    let records = load_records(input)?;
    info!("Data loaded in {:?}", start.elapsed());

    info!("Initializing sampler...");
    let sampler_start = Instant::now();
    let sampler = IncidenceDensitySampler::new(records, criteria)?;
    info!("{}", sampler.get_statistics());
    info!("Sampler initialized in {:?}", sampler_start.elapsed());

    Ok(sampler)
}

fn handle_sampling(
    input: &str,
    controls: usize,
    birth_window: i64,
    parent_window: i64,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let start = Instant::now();
    validate_and_load_data(input)?;

    let criteria = MatchingCriteria {
        birth_date_window: birth_window,
        parent_date_window: parent_window,
    };

    let sampler = create_sampler(input, criteria)?;
    process_sampling_results(&sampler, controls, output_dir)?;

    info!("Total execution time: {:?}", start.elapsed());
    Ok(())
}

fn process_sampling_results(
    sampler: &IncidenceDensitySampler,
    controls: usize,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!("Sampling controls...");
    let sampling_start = Instant::now();

    let case_control_pairs = sampler.sample_controls(controls)?;
    info!(
        "Sampling completed in {:?}. Found {} matches",
        sampling_start.elapsed(),
        case_control_pairs.len()
    );

    let matches_path = Path::new(output_dir).join("matched_pairs.csv");
    if let Err(e) =
        sampler.save_matches_to_csv(&case_control_pairs, &matches_path.to_string_lossy())
    {
        error!("Error saving matches to CSV: {}", e);
    }

    let stats_path = Path::new(output_dir).join("matching_stats.csv");
    if let Err(e) =
        sampler.save_matching_statistics(&case_control_pairs, &stats_path.to_string_lossy())
    {
        error!("Error saving matching statistics: {}", e);
    }

    let quality = sampler.evaluate_matching_quality(&case_control_pairs);
    info!("{}", quality.format_report());

    let plots_dir = Path::new(output_dir).join("plots");
    fs::create_dir_all(&plots_dir)?;

    if let Err(e) = quality.generate_summary_plots(&plots_dir.to_string_lossy()) {
        error!("Error generating plots: {}", e);
    }

    Ok(())
}

fn handle_balance_check(
    matches_file: &str,
    covariate_dir: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let matched_pairs = load_matched_pairs(Path::new(matches_file))?;
    info!("Loaded {} matched pairs", matched_pairs.len());

    let arrow_store = ParquetLoader::new().load_from_path(covariate_dir.to_string())?;
    info!("Loaded covariate data from {}", covariate_dir);

    let checker = BalanceChecker::new(arrow_store);

    let (cases, controls) = convert_to_case_control_pairs(&matched_pairs);
    process_balance_results(&checker, &cases, &controls, output_dir)?;

    Ok(())
}

fn convert_to_case_control_pairs(
    matched_pairs: &[(String, NaiveDate, Vec<String>)],
) -> (Vec<(String, NaiveDate)>, Vec<(String, NaiveDate)>) {
    let case_pnrs: HashSet<String> = matched_pairs
        .iter()
        .map(|(case_pnr, _, _)| case_pnr.clone())
        .collect();

    info!("Collected {} unique case IDs", case_pnrs.len());

    matched_pairs
        .iter()
        .flat_map(|(case_pnr, treatment_date, control_pnrs)| {
            std::iter::once((case_pnr.clone(), *treatment_date)).chain(
                control_pnrs
                    .iter()
                    .map(|control_pnr| (control_pnr.clone(), *treatment_date)),
            )
        })
        .partition(|(pnr, _)| case_pnrs.contains(pnr))
}

fn process_balance_results(
    checker: &BalanceChecker,
    cases: &[(String, NaiveDate)],
    controls: &[(String, NaiveDate)],
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    info!(
        "Processing {} cases and {} controls",
        cases.len(),
        controls.len()
    );

    // Sample a few cases and controls to verify data
    for (i, (pnr, date)) in cases.iter().take(5).enumerate() {
        match checker.get_covariate(pnr, CovariateType::Demographics, *date) {
            Ok(Some(covariate)) => {
                info!("Case {} (PNR: {}) demographics: {:?}", i, pnr, covariate);
            }
            Ok(None) => {
                warn!("No demographics found for case {} (PNR: {})", i, pnr);
            }
            Err(e) => {
                warn!(
                    "Failed to get demographics for case {} (PNR: {}): {}",
                    i, pnr, e
                );
            }
        }
    }

    info!("Calculating covariate balance...");
    let balance_results = checker.calculate_balance(cases, controls)?;

    info!(
        "Balance calculation complete. Found {} summaries",
        balance_results.summaries.len()
    );

    // Log some sample results
    for summary in balance_results.summaries.iter().take(5) {
        info!(
            "Balance summary for {}: cases={:.2}, controls={:.2}, std_diff={:.2}",
            summary.variable, summary.mean_cases, summary.mean_controls, summary.std_diff
        );
    }

    let output_path = Path::new(output_dir).join("covariate_balance.csv");
    BalanceChecker::save_balance_results(&balance_results.summaries, &output_path)?;

    info!("Balance results saved to {}", output_path.display());
    Ok(())
}

// fn debug_sample_case(checker: &BalanceChecker, id: &str, date: &NaiveDate) {
//     #[allow(dead_code)]
//     for covariate_type in &[
//         CovariateType::Demographics,
//         CovariateType::Education,
//         CovariateType::Income,
//     ] {
//         match checker.get_covariate(id, *covariate_type, *date) {
//             Ok(Some(covariate)) => {
//                 info!(
//                     "Sample case {:#?} covariate: {:#?}",
//                     covariate_type, covariate
//                 );
//             }
//             Ok(None) => {
//                 warn!(
//                     "No covariate data found for case {} of type {:?}",
//                     id, covariate_type
//                 );
//             }
//             Err(e) => {
//                 warn!("Failed to get covariate for case {}: {}", id, e);
//             }
//         }
//     }
// }

use chrono::NaiveDate;
use clap::Parser;
use core::{
    sampler::IncidenceDensitySampler,
    utils::{configure_logging, load_records, validate_csv_format, MatchingCriteria},
};
use covariates::{balance::BalanceChecker, matched_pairs::load_matched_pairs};
use datagen::{GeneratorConfig, RegisterGenerator};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use loader::ParquetLoader;
use log::{error, info, warn};
use std::collections::HashSet;
use std::{fs, path::Path, time::Instant};
use types::models::CovariateType;

mod cli;
use cli::{Cli, Commands};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging system with progress bars
    let logger = env_logger::Builder::from_env(env_logger::Env::default()).build();
    let level = logger.filter();
    let multi = MultiProgress::new();

    // Connect logger with progress bars
    if let Err(e) = LogWrapper::new(multi.clone(), logger).try_init() {
        eprintln!("Warning: Failed to initialize logger: {}", e);
    }
    log::set_max_level(level);

    // Parse command line arguments
    let cli = Cli::parse();
    
    // Create output directories and configure logging
    setup_directories(&cli.output_dir)?;
    configure_logging_with_dir(&cli.output_dir)?;

    // Execute the requested command
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
    let base_path = Path::new(output_dir);
    
    // Create main output directory and log directory
    fs::create_dir_all(base_path)?;
    fs::create_dir_all(base_path.join("log"))?;
    
    // Create plots directory for visualizations
    fs::create_dir_all(base_path.join("plots"))?;

    // Create register subdirectories for data storage
    let register_dirs = ["akm", "bef", "ind", "uddf"];
    for dir in &register_dirs {
        fs::create_dir_all(base_path.join(dir))?;
    }

    info!("Created output directories in {}", output_dir);
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
    use core::utils::console::{ConsoleOutput, format_duration_short};
    
    ConsoleOutput::section("Synthetic Register Data Generation");
    
    let start = Instant::now();
    
    // Configuration
    ConsoleOutput::subsection("Configuration");
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value("Total records", &ConsoleOutput::format_number(num_records));
    ConsoleOutput::key_value("Case records", &ConsoleOutput::format_number(num_cases));
    ConsoleOutput::key_value("Year range", &format!("{} - {}", start_year, end_year));
    
    if let Some(s) = seed {
        ConsoleOutput::key_value("Random seed", &s.to_string());
    } else {
        ConsoleOutput::key_value("Random seed", "None (using system random)");
    }
    
    // Create configuration
    let mut config = GeneratorConfig::new(num_records, num_cases, output_dir.to_string())
        .with_year_range(start_year, end_year);
    if let Some(s) = seed {
        config = config.with_seed(s);
    }

    // Generation
    ConsoleOutput::subsection("Generating Data");
    let generation_start = Instant::now();
    let mut generator = RegisterGenerator::new(config)?;
    generator.generate_all()?;
    let generation_time = generation_start.elapsed();
    
    ConsoleOutput::key_value("Generation time", &format_duration_short(generation_time));
    
    // Pediatric data
    ConsoleOutput::subsection("Generating Pediatric Data");
    let pediatric_start = Instant::now();
    let pediatric_path = Path::new(output_dir).join("pediatric.csv");
    generator.generate_pediatric(pediatric_path.to_str().unwrap())?;
    let pediatric_time = pediatric_start.elapsed();
    
    ConsoleOutput::key_value("Pediatric data file", &pediatric_path.display().to_string());
    ConsoleOutput::key_value("Pediatric generation time", &format_duration_short(pediatric_time));
    
    // Summary
    let total_time = start.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value("Records generated", &ConsoleOutput::format_number(num_records));
    ConsoleOutput::key_value("Total execution time", &format_duration_short(total_time));
    ConsoleOutput::success("Register data generation completed successfully");
    
    Ok(())
}

fn validate_and_load_data(input: &str) -> Result<(), Box<dyn std::error::Error>> {
    use core::utils::console::ConsoleOutput;
    
    let validation_start = Instant::now();
    match validate_csv_format(input) {
        Ok(_) => {
            let validation_time = validation_start.elapsed();
            ConsoleOutput::success(&format!("CSV format validation completed in {:?}", validation_time));
            Ok(())
        }
        Err(e) => {
            ConsoleOutput::error(&format!("CSV validation failed: {}", e));
            error!("CSV validation failed: {}", e);
            Err(e)
        }
    }
}

fn create_sampler(
    input: &str,
    criteria: MatchingCriteria,
) -> Result<IncidenceDensitySampler, Box<dyn std::error::Error>> {
    use core::utils::console::{ConsoleOutput, format_duration_short};
    
    ConsoleOutput::subsection("Data Loading");
    ConsoleOutput::key_value("Input file", input);
    
    let start = Instant::now();
    let records = load_records(input)?;
    let loading_time = start.elapsed();
    
    ConsoleOutput::key_value("Records loaded", &ConsoleOutput::format_number(records.len()));
    ConsoleOutput::key_value("Loading time", &format_duration_short(loading_time));

    ConsoleOutput::subsection("Sampler Initialization");
    let sampler_start = Instant::now();
    let sampler = IncidenceDensitySampler::new(records, criteria)?;
    let init_time = sampler_start.elapsed();
    
    // Get statistics and display in a more structured way
    let stats = sampler.get_statistics();
    println!("{}", stats);
    
    ConsoleOutput::key_value("Initialization time", &format_duration_short(init_time));

    Ok(sampler)
}

fn handle_sampling(
    input: &str,
    controls: usize,
    birth_window: i64,
    parent_window: i64,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use core::utils::console::{ConsoleOutput, format_duration_short};

    ConsoleOutput::section("Incidence Density Sampling");
    
    let start = Instant::now();
    ConsoleOutput::subsection("Data Validation");
    validate_and_load_data(input)?;

    let criteria = MatchingCriteria {
        birth_date_window: birth_window,
        parent_date_window: parent_window,
    };

    ConsoleOutput::key_value("Birth date window", &format!("{} days", birth_window));
    ConsoleOutput::key_value("Parent date window", &format!("{} days", parent_window));
    
    let sampler = create_sampler(input, criteria)?;
    process_sampling_results(&sampler, controls, output_dir)?;

    let total_time = start.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Input file", input);
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value("Total execution time", &format_duration_short(total_time));
    ConsoleOutput::success("Sampling completed successfully");
    
    Ok(())
}

fn process_sampling_results(
    sampler: &IncidenceDensitySampler,
    controls: usize,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use core::utils::console::{ConsoleOutput, format_duration_short};
    
    ConsoleOutput::subsection("Sampling Controls");
    ConsoleOutput::key_value("Requested controls per case", &controls.to_string());
    
    let sampling_start = Instant::now();
    let case_control_pairs = sampler.sample_controls(controls)?;
    let sampling_time = sampling_start.elapsed();
    
    ConsoleOutput::key_value("Sampling time", &format_duration_short(sampling_time));
    ConsoleOutput::key_value_colored(
        "Matches found", 
        &format!("{}", case_control_pairs.len()), 
        !case_control_pairs.is_empty()
    );

    ConsoleOutput::subsection("Saving Results");
    
    // Save matched pairs
    let matches_path = Path::new(output_dir).join("matched_pairs.csv");
    match sampler.save_matches_to_csv(&case_control_pairs, &matches_path.to_string_lossy()) {
        Ok(_) => {
            ConsoleOutput::success(&format!("Matches saved to {}", matches_path.display()));
        }
        Err(e) => {
            ConsoleOutput::error(&format!("Error saving matches to CSV: {}", e));
            error!("Error saving matches to CSV: {}", e);
        }
    }

    // Save statistics
    let stats_path = Path::new(output_dir).join("matching_stats.csv");
    match sampler.save_matching_statistics(&case_control_pairs, &stats_path.to_string_lossy()) {
        Ok(_) => {
            ConsoleOutput::success(&format!("Statistics saved to {}", stats_path.display()));
        }
        Err(e) => {
            ConsoleOutput::error(&format!("Error saving statistics: {}", e));
            error!("Error saving matching statistics: {}", e);
        }
    }

    // Evaluate and display quality metrics
    ConsoleOutput::subsection("Matching Quality Analysis");
    let quality = sampler.evaluate_matching_quality(&case_control_pairs);
    println!("{}", quality.format_report());

    // Generate plots
    let plots_dir = Path::new(output_dir).join("plots");
    fs::create_dir_all(&plots_dir)?;

    match quality.generate_summary_plots(&plots_dir.to_string_lossy()) {
        Ok(_) => {
            ConsoleOutput::success(&format!("Quality plots generated in {}", plots_dir.display()));
        }
        Err(e) => {
            ConsoleOutput::error(&format!("Error generating plots: {}", e));
            error!("Error generating plots: {}", e);
        }
    }

    Ok(())
}

fn handle_balance_check(
    matches_file: &str,
    covariate_dir: &str,
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use core::utils::console::{ConsoleOutput, format_duration_short};
    
    ConsoleOutput::section("Covariate Balance Analysis");
    
    let start = Instant::now();
    ConsoleOutput::subsection("Loading Data");
    
    // Load matched pairs
    let loading_start = Instant::now();
    let matched_pairs = load_matched_pairs(Path::new(matches_file))?;
    let loading_time = loading_start.elapsed();
    
    ConsoleOutput::key_value("Matched pairs loaded", &matched_pairs.len().to_string());
    ConsoleOutput::key_value("Loading time", &format_duration_short(loading_time));
    
    // Load covariate data
    let covariate_start = Instant::now();
    let arrow_store = ParquetLoader::new().load_from_path(covariate_dir.to_string())?;
    let covariate_time = covariate_start.elapsed();
    
    ConsoleOutput::key_value("Covariate source", covariate_dir);
    ConsoleOutput::key_value("Covariate loading time", &format_duration_short(covariate_time));
    
    // Process data
    ConsoleOutput::subsection("Processing Balance");
    let checker = BalanceChecker::new(arrow_store);
    let (cases, controls) = convert_to_case_control_pairs(&matched_pairs);
    
    ConsoleOutput::key_value("Cases", &cases.len().to_string());
    ConsoleOutput::key_value("Controls", &controls.len().to_string());
    
    process_balance_results(&checker, &cases, &controls, output_dir)?;
    
    // Summary
    let total_time = start.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Matched pairs file", matches_file);
    ConsoleOutput::key_value("Covariate directory", covariate_dir);
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value("Total execution time", &format_duration_short(total_time));
    ConsoleOutput::success("Balance analysis completed successfully");

    Ok(())
}

#[allow(clippy::type_complexity)]
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
    use core::utils::console::{ConsoleOutput, format_duration_short};
    
    ConsoleOutput::key_value("Processing cases", &format!("{}", cases.len()));
    ConsoleOutput::key_value("Processing controls", &format!("{}", controls.len()));

    // Sample data verification
    ConsoleOutput::subsection("Data Verification");
    
    // Sample a few cases to verify data
    let mut sample_rows = Vec::new();
    for (i, (pnr, date)) in cases.iter().take(3).enumerate() {
        match checker.get_covariate(pnr, CovariateType::Demographics, *date) {
            Ok(Some(covariate)) => {
                let covariate_str = format!("{:?}", covariate);
                // Truncate if too long for display
                let display_str = if covariate_str.len() > 60 {
                    format!("{}...", &covariate_str[0..57])
                } else {
                    covariate_str
                };
                sample_rows.push(vec![
                    format!("Case {}", i + 1),
                    pnr.clone(),
                    date.to_string(),
                    display_str
                ]);
            }
            Ok(None) => {
                sample_rows.push(vec![
                    format!("Case {}", i + 1),
                    pnr.clone(),
                    date.to_string(),
                    "No demographics found".to_string()
                ]);
                warn!("No demographics found for case {} (PNR: {})", i, pnr);
            }
            Err(e) => {
                sample_rows.push(vec![
                    format!("Case {}", i + 1),
                    pnr.clone(),
                    date.to_string(),
                    format!("Error: {}", e)
                ]);
                warn!(
                    "Failed to get demographics for case {} (PNR: {}): {}",
                    i, pnr, e
                );
            }
        }
    }
    
    if !sample_rows.is_empty() {
        ConsoleOutput::table(
            &["ID", "PNR", "Date", "Demographics"],
            &sample_rows
        );
    }

    // Calculate balance
    ConsoleOutput::subsection("Balance Calculation");
    let calculation_start = Instant::now();
    let balance_results = checker.calculate_balance(cases, controls)?;
    let calculation_time = calculation_start.elapsed();
    
    ConsoleOutput::key_value("Calculation time", &format_duration_short(calculation_time));
    ConsoleOutput::key_value(
        "Balance summaries", 
        &format!("{}", balance_results.summaries.len())
    );

    // Display sample results
    if !balance_results.summaries.is_empty() {
        use colored::Colorize;
        ConsoleOutput::subsection("Sample Balance Results");
        
        let mut balance_rows = Vec::new();
        for summary in balance_results.summaries.iter().take(5) {
            let std_diff_str = format!("{:.3}", summary.std_diff);
            let std_diff_colored = if summary.std_diff.abs() < 0.1 {
                std_diff_str.green().to_string()
            } else if summary.std_diff.abs() < 0.2 {
                std_diff_str.yellow().to_string() 
            } else {
                std_diff_str.red().to_string()
            };
            
            balance_rows.push(vec![
                summary.variable.clone(),
                format!("{:.2}", summary.mean_cases),
                format!("{:.2}", summary.mean_controls),
                std_diff_colored
            ]);
        }
        
        ConsoleOutput::table(
            &["Variable", "Cases Mean", "Controls Mean", "Std Diff"],
            &balance_rows
        );
    }

    // Save results
    ConsoleOutput::subsection("Saving Results");
    let save_start = Instant::now();
    
    use covariates::reporting::ComprehensiveReport;
    let report = ComprehensiveReport::new(balance_results);
    report.save_to_files(Path::new(output_dir))?;
    
    let save_time = save_start.elapsed();
    ConsoleOutput::key_value("Save time", &format_duration_short(save_time));
    ConsoleOutput::success(&format!("Balance results saved to {}", output_dir));
    
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

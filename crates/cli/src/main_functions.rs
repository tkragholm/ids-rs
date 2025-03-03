use chrono::NaiveDate;
use clap::Parser;
use core::{
    sampler::IncidenceDensitySampler,
    utils::{configure_logging_with_level, load_records, validate_csv_format, MatchingCriteria},
};
use covariates::matched_pairs::load_matched_pair_records;
use covariates::{balance::BalanceChecker, matched_pairs::load_matched_pairs};
use datagen::{GeneratorConfig, RegisterGenerator};
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::{error, info, warn};
use std::collections::HashSet;
use std::{fs, path::Path, time::Instant};
use types::models::CovariateType;

use crate::cli::{Cli, Commands};
use crate::generate_structured_reports;

/// Main entry point for the CLI application
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Check for the most common command line mistake - missing space after --family-file
    for (_i, arg) in std::env::args().enumerate() {
        if arg.starts_with("--family-file") && arg != "--family-file" {
            eprintln!("ERROR: Detected possible command line issue. You provided '{}' without a space.", arg);
            eprintln!("       Did you mean to write: --family-file {}", &arg[13..]);
            eprintln!("       Check other parameters too. Put a space between each flag and its value.");
            std::process::exit(1);
        }
    }

    // Initialize logging system with progress bars
    // Create a custom environment with a modified default filter
    // This allows us to control the logger behavior more precisely
    let env = env_logger::Env::default()
        .filter_or("RUST_LOG", "warn");
    
    // Build the logger with our custom env settings
    let logger = env_logger::Builder::from_env(env)
        .format_timestamp(Some(env_logger::TimestampPrecision::Seconds))
        .format_module_path(false) // Make logs cleaner
        .build();
    
    // Get the filter level to properly set max log level
    let level = logger.filter();
    
    // Create a MultiProgress for use with the LogWrapper
    let multi = MultiProgress::new();

    // Connect logger with progress bars to prevent progress bars from being interrupted by logs
    if let Err(e) = LogWrapper::new(multi.clone(), logger).try_init() {
        eprintln!("Warning: Failed to initialize logger: {}", e);
    }
    
    // Set the global max log level
    log::set_max_level(level);

    // Parse command line arguments
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            eprintln!("\nNOTE: Make sure there is a space between each flag and its value!");
            eprintln!("Example: --family-file data/registers/family.parquet");
            std::process::exit(1);
        }
    };

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
            family_file,
            akm_dir,
            bef_dir,
            ind_dir,
            uddf_dir,
            structured,
        } => handle_balance_check(
            matches_file,
            covariate_dir.as_deref(),
            &cli.output_dir,
            family_file.as_deref(),
            akm_dir.as_deref(),
            bef_dir.as_deref(),
            ind_dir.as_deref(),
            uddf_dir.as_deref(),
            *structured,
        ),
    }
}

fn setup_directories(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(output_dir);

    // Create main output directory and log directory
    fs::create_dir_all(base_path)?;
    fs::create_dir_all(base_path.join("log"))?;

    // Create plots directory for visualizations
    fs::create_dir_all(base_path.join("plots"))?;
    
    // Create report directory for HTML reports
    fs::create_dir_all(base_path.join("report"))?;

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

    // Use more restrictive logging in the console to reduce terminal noise
    // Only show warnings and errors in the console
    let log_level = log::LevelFilter::Warn;
    
    // This will send logs to both console and file, but we're setting
    // the overall level to Warn to reduce console output
    configure_logging_with_level(Some(&log_path), log_level)?;
    
    Ok(())
}

// ... include all other functions from main.rs here ...

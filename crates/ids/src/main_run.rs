/// This module contains the main functionality of the CLI application
use chrono::NaiveDate;
use clap::Parser;
use core::{
    sampler::IncidenceDensitySampler,
    utils::{configure_logging_with_level, load_records, validate_csv_format, MatchingCriteria},
};
use covariates::matched_pairs::load_matched_pair_records;
use covariates::{balance::BalanceChecker, matched_pairs::load_matched_pairs, config::CovariatesConfig};
use datagen::{GeneratorConfig, RegisterGenerator};
use hashbrown;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use log::{error, info, warn};
use std::collections::HashSet;
use std::{fs, path::Path, time::Instant};
use types::models::CovariateType;
use serde_json;

use crate::cli::{Cli, Commands, ConfigCommands};

/// Main entry point for the CLI application
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Check for the most common command line mistake - missing space after --family-file
    for arg in std::env::args() {
        if arg.starts_with("--family-file") && arg != "--family-file" {
            eprintln!(
                "ERROR: Detected possible command line issue. You provided '{}' without a space.",
                arg
            );
            eprintln!("       Did you mean to write: --family-file {}", &arg[13..]);
            eprintln!(
                "       Check other parameters too. Put a space between each flag and its value."
            );
            std::process::exit(1);
        }
    }

    // Initialize logging system with progress bars
    // Create a custom environment with a modified default filter
    // This allows us to control the logger behavior more precisely
    let env = env_logger::Env::default().filter_or("RUST_LOG", "warn");

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
        Commands::Config { command } => handle_config_command(command),
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

fn handle_generate_registers(
    output_dir: &str,
    num_records: usize,
    num_cases: usize,
    start_year: i32,
    end_year: i32,
    seed: Option<u64>,
) -> Result<(), Box<dyn std::error::Error>> {
    use core::utils::console::{format_duration_short, ConsoleOutput};

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
    ConsoleOutput::key_value(
        "Pediatric generation time",
        &format_duration_short(pediatric_time),
    );

    // Summary
    let total_time = start.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value(
        "Records generated",
        &ConsoleOutput::format_number(num_records),
    );
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
            ConsoleOutput::success(&format!(
                "CSV format validation completed in {:?}",
                validation_time
            ));
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
    use core::utils::console::{format_duration_short, ConsoleOutput};

    ConsoleOutput::subsection("Data Loading");
    ConsoleOutput::key_value("Input file", input);

    let start = Instant::now();
    let records = load_records(input)?;
    let loading_time = start.elapsed();

    ConsoleOutput::key_value(
        "Records loaded",
        &ConsoleOutput::format_number(records.len()),
    );
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
    use core::utils::console::{format_duration_short, ConsoleOutput};

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
    use core::utils::console::{format_duration_short, ConsoleOutput};

    ConsoleOutput::subsection("Sampling Controls");
    ConsoleOutput::key_value("Requested controls per case", &controls.to_string());

    let sampling_start = Instant::now();
    let case_control_pairs = sampler.sample_controls(controls)?;
    let sampling_time = sampling_start.elapsed();

    ConsoleOutput::key_value("Sampling time", &format_duration_short(sampling_time));
    ConsoleOutput::key_value_colored(
        "Matches found",
        &format!("{}", case_control_pairs.len()),
        !case_control_pairs.is_empty(),
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
            ConsoleOutput::success(&format!(
                "Quality plots generated in {}",
                plots_dir.display()
            ));
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
    covariate_dir: Option<&str>,
    output_dir: &str,
    family_file: Option<&str>,
    akm_dir: Option<&str>,
    bef_dir: Option<&str>,
    ind_dir: Option<&str>,
    uddf_dir: Option<&str>,
    generate_structured_output: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    use core::utils::console::{format_duration_short, ConsoleOutput};
    use covariates::balance::BalanceChecker;
    use loader::ParquetLoader;

    ConsoleOutput::section("Covariate Balance Analysis");

    let start = Instant::now();

    // Step 1: Load matched pairs
    ConsoleOutput::subsection("Loading Matched Pairs");
    let matches_path = Path::new(matches_file);
    if !matches_path.exists() {
        ConsoleOutput::error(&format!(
            "Matched pairs file not found: {}",
            matches_path.display()
        ));
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Matched pairs file not found: {}", matches_path.display()),
        )
        .into());
    }

    let loading_start = Instant::now();
    let matched_pairs = match load_matched_pairs(matches_path) {
        Ok(pairs) => pairs,
        Err(e) => {
            ConsoleOutput::error(&format!("Failed to load matched pairs: {}", e));
            return Err(e.into());
        }
    };
    let loading_time = loading_start.elapsed();

    // Extract all unique PNRs for diagnostic mode (will be used if register data loading fails)
    let mut all_unique_pnrs = std::collections::HashSet::new();
    for (case_pnr, _, control_pnrs) in &matched_pairs {
        all_unique_pnrs.insert(case_pnr.clone());
        for control_pnr in control_pnrs {
            all_unique_pnrs.insert(control_pnr.clone());
        }
    }
    let unique_pnrs_vec: Vec<String> = all_unique_pnrs.into_iter().collect();

    ConsoleOutput::key_value("Matched pairs loaded", &matched_pairs.len().to_string());
    ConsoleOutput::key_value("Unique PNRs found", &unique_pnrs_vec.len().to_string());
    ConsoleOutput::key_value("Loading time", &format_duration_short(loading_time));

    // Step 2: Set up custom paths with proper resolution
    let mut custom_paths = hashbrown::HashMap::new();
    let mut has_custom_paths = false;

    // Helper function to check if a path exists and log its status
    let check_path_exists = |path: &str, path_type: &str| {
        let path_obj = Path::new(path);
        let exists = path_obj.exists();
        let path_type_str = if path_obj.is_dir() {
            "directory"
        } else if path_obj.is_file() {
            "file"
        } else {
            "path"
        };

        if exists {
            ConsoleOutput::info(&format!(
                "Found {} at {} ({})",
                path_type, path, path_type_str
            ));
            log::debug!("Found {} at {} ({})", path_type, path, path_type_str);
        } else {
            ConsoleOutput::warning(&format!(
                "{} not found at {} - will attempt to find alternative paths",
                path_type, path
            ));
            log::warn!("{} not found at {}", path_type, path);
        }
        exists
    };

    // Helper function to normalize paths with extensive debugging
    let normalize_path = |path: &str, register_type: &str| -> String {
        let path_obj = Path::new(path);

        // Log the input path
        log::debug!("Normalizing {} path: {}", register_type, path);

        if path_obj.is_absolute() {
            // If the path is absolute, use it as-is
            log::debug!("Using absolute path for {}: {}", register_type, path);
            check_path_exists(path, &format!("{} (absolute)", register_type));
            path.to_string()
        } else if let Some(base_dir) = covariate_dir {
            // Check if the path already starts with the base_dir to avoid duplication
            if path.contains(base_dir) {
                log::debug!("Path already contains base_dir ({}): {}", base_dir, path);
                check_path_exists(path, &format!("{} (with base_dir)", register_type));
                path.to_string()
            } else {
                let full_path = Path::new(base_dir).join(path).to_string_lossy().to_string();
                log::debug!(
                    "Combining base_dir and path: {} + {} -> {}",
                    base_dir,
                    path,
                    full_path
                );
                check_path_exists(&full_path, &format!("{} (combined)", register_type));
                full_path
            }
        } else {
            // If no covariate_dir was provided, assume relative to current directory
            let resolved_path = match path_obj.canonicalize() {
                Ok(canonical) => {
                    log::debug!(
                        "Canonicalized path for {}: {} -> {}",
                        register_type,
                        path,
                        canonical.display()
                    );
                    canonical.to_string_lossy().to_string()
                }
                Err(e) => {
                    log::warn!("Failed to canonicalize path {}: {}", path, e);
                    path.to_string()
                }
            };
            check_path_exists(&resolved_path, &format!("{} (relative)", register_type));
            resolved_path
        }
    };

    if let Some(path) = family_file {
        let normalized_path = normalize_path(path, "family");
        custom_paths.insert("family".to_string(), normalized_path.clone());
        ConsoleOutput::key_value("Using custom family file", &normalized_path);
        has_custom_paths = true;
    }

    if let Some(path) = akm_dir {
        let normalized_path = normalize_path(path, "akm");
        custom_paths.insert("akm".to_string(), normalized_path.clone());
        ConsoleOutput::key_value("Using custom AKM directory", &normalized_path);
        has_custom_paths = true;
    }

    if let Some(path) = bef_dir {
        let normalized_path = normalize_path(path, "bef");
        custom_paths.insert("bef".to_string(), normalized_path.clone());
        ConsoleOutput::key_value("Using custom BEF directory", &normalized_path);
        has_custom_paths = true;
    }

    if let Some(path) = ind_dir {
        let normalized_path = normalize_path(path, "ind");
        custom_paths.insert("ind".to_string(), normalized_path.clone());
        ConsoleOutput::key_value("Using custom IND directory", &normalized_path);
        has_custom_paths = true;
    }

    if let Some(path) = uddf_dir {
        let normalized_path = normalize_path(path, "uddf");
        custom_paths.insert("uddf".to_string(), normalized_path.clone());
        ConsoleOutput::key_value("Using custom UDDF directory", &normalized_path);
        has_custom_paths = true;
    }

    // Step 3: Load register data
    ConsoleOutput::subsection("Loading Register Data");
    let loader = ParquetLoader::new();

    // Attempt to load the data based on what paths we have
    let arrow_store_result = if has_custom_paths {
        // If we have custom paths, use them regardless of whether covariate_dir is provided
        let base_dir = match covariate_dir {
            Some(dir) => {
                ConsoleOutput::info(&format!("Using base directory: {}", dir));
                log::debug!("Using base directory with custom paths: {}", dir);

                // Check if the directory exists
                if !Path::new(dir).exists() {
                    ConsoleOutput::warning(&format!(
                        "Base directory doesn't exist: {}. Will try to use custom paths directly.",
                        dir
                    ));
                    log::warn!("Base directory doesn't exist: {}", dir);
                }

                dir.to_string()
            }
            None => {
                // If no base directory is provided, use the current directory
                let current_dir = std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string());

                ConsoleOutput::info(&format!(
                    "No base directory provided, using current directory: {}",
                    current_dir
                ));
                log::debug!("Using current directory as base: {}", current_dir);
                current_dir
            }
        };

        // CRITICAL DEBUG: Check family file first specifically since it's the most important
        if let Some(family_path) = custom_paths.get("family") {
            let path_obj = Path::new(family_path);
            ConsoleOutput::info(&format!(
                "CRITICAL CHECK: Verifying family file at {}",
                path_obj.display()
            ));
            if path_obj.exists() {
                if path_obj.is_file() {
                    // Try to open the file directly to see if we can access it
                    ConsoleOutput::info("Family path exists and is a file - checking access");
                    match std::fs::File::open(path_obj) {
                        Ok(mut file) => {
                            // Try to read the first few bytes
                            let mut buffer = [0; 16];
                            match std::io::Read::read(&mut file, &mut buffer) {
                                Ok(bytes_read) => {
                                    ConsoleOutput::info(&format!(
                                        "Successfully read {} bytes from family file",
                                        bytes_read
                                    ));
                                    // Print the first few bytes in hex for debugging
                                    let hex_bytes: Vec<String> = buffer
                                        .iter()
                                        .take(bytes_read)
                                        .map(|b| format!("{:02x}", b))
                                        .collect();
                                    ConsoleOutput::info(&format!(
                                        "First bytes: {}",
                                        hex_bytes.join(" ")
                                    ));
                                }
                                Err(e) => {
                                    ConsoleOutput::error(&format!(
                                        "Failed to read bytes from family file: {}",
                                        e
                                    ));
                                }
                            }
                        }
                        Err(e) => {
                            ConsoleOutput::error(&format!("Failed to open family file: {}", e));
                        }
                    }

                    // See if we can run pqrs on it
                    ConsoleOutput::info("Testing file with pqrs (shell command)");
                    if let Ok(output) = std::process::Command::new("pqrs")
                        .args(["head", "-n", "1", family_path])
                        .output()
                    {
                        if output.status.success() {
                            let stdout = String::from_utf8_lossy(&output.stdout);
                            ConsoleOutput::info(&format!(
                                "pqrs output: {}",
                                stdout.trim().lines().next().unwrap_or("empty")
                            ));
                        } else {
                            let stderr = String::from_utf8_lossy(&output.stderr);
                            ConsoleOutput::error(&format!("pqrs error: {}", stderr.trim()));
                        }
                    } else {
                        ConsoleOutput::error("Failed to run pqrs command");
                    }
                } else {
                    ConsoleOutput::warning(&format!(
                        "Family path exists but is not a file: {}",
                        path_obj.display()
                    ));
                }
            } else {
                ConsoleOutput::warning(&format!(
                    "Family file not found at: {}",
                    path_obj.display()
                ));
            }
        }

        // Log all custom paths for debugging
        ConsoleOutput::info("Using the following custom register paths:");
        for (reg_type, path) in &custom_paths {
            ConsoleOutput::key_value(&format!("  - {}", reg_type), path);
            log::debug!("Custom path for {}: {}", reg_type, path);

            // Additional checks to help with debugging
            let path_obj = Path::new(path);
            if path_obj.exists() {
                if path_obj.is_dir() {
                    // If it's a directory, list the first few files
                    match std::fs::read_dir(path) {
                        Ok(entries) => {
                            let mut file_list = Vec::new();
                            for (i, entry) in entries.take(5).enumerate() {
                                if let Ok(entry) = entry {
                                    file_list.push(entry.file_name().to_string_lossy().to_string());
                                    if i >= 4 {
                                        file_list.push("...".to_string());
                                        break;
                                    }
                                }
                            }
                            log::debug!("Directory {} contains: {}", path, file_list.join(", "));
                            ConsoleOutput::info(&format!(
                                "Directory {} contains: {}",
                                path,
                                file_list.join(", ")
                            ));
                        }
                        Err(e) => {
                            log::warn!("Failed to read directory {}: {}", path, e);
                        }
                    }
                }
            } else {
                ConsoleOutput::warning(&format!("Path doesn't exist: {}", path));
                log::warn!("Path doesn't exist: {}", path);
            }
        }

        // Make a debugging copy of custom_paths for better error reporting
        let debug_custom_paths = custom_paths.clone();

        let loader_result = loader.load_with_custom_paths_map(base_dir.clone(), custom_paths);

        // Provide additional error context if loading fails
        if let Err(ref e) = loader_result {
            ConsoleOutput::error(&format!("Loading failed with error: {}", e));
            ConsoleOutput::info("Trying to diagnose the issue:");

            // Check file access for each register directory
            for (reg_type, path) in &debug_custom_paths {
                if reg_type == "family" {
                    // Family file should be a file, not a directory
                    let file_path = Path::new(path);
                    if file_path.exists() && file_path.is_file() {
                        ConsoleOutput::info(&format!(
                            "Family file exists at {}",
                            file_path.display()
                        ));

                        // Try to open with standard Rust file API
                        match std::fs::File::open(file_path) {
                            Ok(_) => ConsoleOutput::info(
                                "Family file can be opened with Rust std::fs::File",
                            ),
                            Err(e) => ConsoleOutput::error(&format!(
                                "Failed to open family file with std::fs::File: {}",
                                e
                            )),
                        }
                    }
                } else {
                    // Other register types should be directories
                    let dir_path = Path::new(path);
                    if dir_path.exists() && dir_path.is_dir() {
                        ConsoleOutput::info(&format!("Register directory {} exists", reg_type));

                        // Try to list first parquet file in directory
                        if let Ok(entries) = std::fs::read_dir(dir_path) {
                            let parquet_files: Vec<_> = entries
                                .filter_map(Result::ok)
                                .filter(|e| {
                                    e.path().extension().map_or(false, |ext| ext == "parquet")
                                })
                                .take(1)
                                .collect();

                            if let Some(first_file) = parquet_files.first() {
                                let sample_path = first_file.path();
                                ConsoleOutput::info(&format!(
                                    "Checking sample {} file: {}",
                                    reg_type,
                                    sample_path.display()
                                ));

                                // Try to open file
                                match std::fs::File::open(&sample_path) {
                                    Ok(_) => ConsoleOutput::info(
                                        "Sample file can be opened with Rust std::fs::File",
                                    ),
                                    Err(e) => ConsoleOutput::error(&format!(
                                        "Failed to open sample file with std::fs::File: {}",
                                        e
                                    )),
                                }
                            }
                        }
                    }
                }
            }
        }

        loader_result
    } else if let Some(cov_dir) = covariate_dir {
        // Check if the directory exists and contains register data
        let cov_dir_path = Path::new(cov_dir);
        if !cov_dir_path.exists() || !cov_dir_path.is_dir() {
            ConsoleOutput::warning(&format!("Covariate directory not found: {}", cov_dir));
            log::warn!("Covariate directory not found: {}", cov_dir);
        } else {
            // List contents to help with debugging
            match std::fs::read_dir(cov_dir_path) {
                Ok(entries) => {
                    let mut contents = Vec::new();
                    for entry in entries.take(10) {
                        if let Ok(entry) = entry {
                            contents.push(entry.file_name().to_string_lossy().to_string());
                        }
                    }
                    log::debug!("Covariate directory contents: {}", contents.join(", "));
                    ConsoleOutput::info(&format!(
                        "Covariate directory contains: {}",
                        contents.join(", ")
                    ));
                }
                Err(e) => {
                    log::error!("Failed to read covariate directory: {}", e);
                }
            }
        }

        ConsoleOutput::info(&format!("Checking register data in: {}", cov_dir));
        loader.load_from_path(cov_dir.to_string())
    } else {
        // Neither custom paths nor covariate_dir provided
        ConsoleOutput::warning(
            "No covariate directory or custom paths specified, using diagnostic mode",
        );
        Err(types::error::IdsError::missing_data(
            "No data source specified",
        ))
    };

    // Determine if we need to use diagnostic mode and create the appropriate balance checker
    let (balance_checker, use_diagnostic_mode) = match arrow_store_result {
        Ok(store) => {
            ConsoleOutput::success("Successfully loaded register data");
            (BalanceChecker::new(store), false)
        }
        Err(e) => {
            ConsoleOutput::warning(&format!("Failed to load register data: {}", e));
            log::error!("Detailed register loading error: {:?}", e);
            ConsoleOutput::info("Continuing in diagnostic mode with simulated data");
            ConsoleOutput::info(
                "Note: Results will be based on simulated data, not actual register data",
            );

            // Create a diagnostic checker with actual PNRs from the matched pairs
            (
                BalanceChecker::new_diagnostic_with_pnrs(unique_pnrs_vec),
                true,
            )
        }
    };

    // Step 4: Prepare case-control pairs
    ConsoleOutput::subsection("Preparing Analysis");
    let (mut cases, mut controls) = convert_to_case_control_pairs(&matched_pairs);

    ConsoleOutput::key_value("Total cases", &cases.len().to_string());
    ConsoleOutput::key_value("Total controls", &controls.len().to_string());

    // Print a sample of the case data to help with debugging
    if !cases.is_empty() {
        let sample_size = std::cmp::min(5, cases.len());
        let mut sample_cases = Vec::new();
        for (i, (pnr, date)) in cases.iter().take(sample_size).enumerate() {
            sample_cases.push(vec![
                format!("Case {}", i + 1),
                pnr.clone(),
                date.to_string(),
            ]);
            log::debug!("Sample case {}: PNR {} at date {}", i + 1, pnr, date);
        }
        ConsoleOutput::table(&["ID", "PNR", "Date"], &sample_cases);
    }

    // If we're using the diagnostic mode, limit the number of cases/controls to a smaller set
    if use_diagnostic_mode {
        ConsoleOutput::info("Using a sample of the matched pairs for diagnostic mode");

        // Keep only the first 100 cases and controls
        let case_limit = std::cmp::min(100, cases.len());
        let control_limit = std::cmp::min(100, controls.len());

        cases.truncate(case_limit);
        controls.truncate(control_limit);

        // Store the original PNRs to create duplicates with both formats
        let original_cases = cases.clone();
        let original_controls = controls.clone();

        // Add C/K format IDs for each original PNR
        for (i, (pnr, date)) in original_cases.iter().enumerate() {
            let c_format_pnr = format!("C{:06}", i);
            cases.push((c_format_pnr.clone(), *date));

            // Print a debug message for the first few
            if i < 3 {
                log::debug!(
                    "Created duplicate case entry with C-format: {} (original: {})",
                    c_format_pnr,
                    pnr
                );
                ConsoleOutput::info(&format!("Adding C-format duplicate for case: {}", pnr));
            }
        }

        for (i, (pnr, date)) in original_controls.iter().enumerate() {
            let k_format_pnr = format!("K{:06}", i);
            controls.push((k_format_pnr.clone(), *date));

            // Print a debug message for the first few
            if i < 3 {
                log::debug!(
                    "Created duplicate control entry with K-format: {} (original: {})",
                    k_format_pnr,
                    pnr
                );
                ConsoleOutput::info(&format!("Adding K-format duplicate for control: {}", pnr));
            }
        }

        // Now we have both original PNRs and C/K formats in our list
        ConsoleOutput::key_value(
            "Total case entries (including duplicates)",
            &cases.len().to_string(),
        );
        ConsoleOutput::key_value(
            "Total control entries (including duplicates)",
            &controls.len().to_string(),
        );

        // Print a few sample entries to show the duplicated IDs
        ConsoleOutput::info("Using both original PNRs and C/K formats for maximum compatibility");
    }

    // Step 5: Calculate balance
    ConsoleOutput::subsection("Calculating Balance");
    let balance_start = Instant::now();

    let balance_result = balance_checker.calculate_balance(&cases, &controls);

    match balance_result {
        Ok(balance_results) => {
            let balance_time = balance_start.elapsed();
            ConsoleOutput::key_value(
                "Balance calculation time",
                &format_duration_short(balance_time),
            );

            // Display balance statistics
            let summaries_count = balance_results.summaries.len();
            let details_count = balance_results.matched_pair_details.len();

            ConsoleOutput::key_value("Balance summaries generated", &summaries_count.to_string());
            ConsoleOutput::key_value("Matched pair details processed", &details_count.to_string());

            // Save results
            ConsoleOutput::subsection("Saving Results");
            let save_path = Path::new(output_dir).join("covariate_balance.csv");

            // Create a comprehensive report
            use covariates::reporting::ComprehensiveReport;
            let balance_results_copy = balance_results.clone();
            let report = ComprehensiveReport::new(balance_results);
            report.save_to_files(Path::new(output_dir))?;

            ConsoleOutput::success(&format!("Saved balance results to {}", save_path.display()));

            // If structured output is enabled, generate it
            if generate_structured_output {
                // Load the original matched pair records for structured reports
                let matched_pair_records =
                    match load_matched_pair_records(Path::new(
                        matches_file,
                    )) {
                        Ok(records) => records,
                        Err(e) => {
                            ConsoleOutput::warning(&format!(
                                "Could not load matched pair records for structured reports: {}",
                                e
                            ));
                            Vec::new()
                        }
                    };

                // Generate structured reports
                match generate_structured_reports(
                    &balance_results_copy,
                    &matched_pair_records,
                    output_dir,
                ) {
                    Ok(_) => {
                        ConsoleOutput::success(&format!(
                            "Generated structured reports in {}/report/",
                            output_dir
                        ));
                    }
                    Err(e) => {
                        ConsoleOutput::warning(&format!(
                            "Failed to generate structured reports: {}",
                            e
                        ));
                    }
                }
            }

            // Generate plots
            ConsoleOutput::subsection("Generating Visualizations");
            let plots_dir = Path::new(output_dir).join("plots");
            fs::create_dir_all(&plots_dir)?;

            // Generate plots using the report
            match report.generate_plots(&plots_dir) {
                Ok(_) => {
                    ConsoleOutput::success(&format!("Generated plots in {}", plots_dir.display()));
                }
                Err(e) => {
                    ConsoleOutput::warning(&format!("Failed to generate plots: {}", e));
                }
            }
        }
        Err(e) => {
            ConsoleOutput::error(&format!("Failed to calculate balance: {}", e));
            return Err(e.into());
        }
    }

    // Summary
    let total_time = start.elapsed();
    ConsoleOutput::section("Summary");
    ConsoleOutput::key_value("Matched pairs file", matches_file);
    if let Some(cov_dir) = covariate_dir {
        ConsoleOutput::key_value("Covariate directory", cov_dir);
    }
    ConsoleOutput::key_value("Output directory", output_dir);
    ConsoleOutput::key_value("Total execution time", &format_duration_short(total_time));

    if use_diagnostic_mode {
        ConsoleOutput::warning("Analysis completed in diagnostic mode with simulated data");
    } else {
        ConsoleOutput::success("Covariate balance analysis completed with actual register data");
    }

    Ok(())
}

#[allow(clippy::type_complexity, dead_code)]
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

#[allow(dead_code)]
fn process_balance_results(
    checker: &BalanceChecker,
    cases: &[(String, NaiveDate)],
    controls: &[(String, NaiveDate)],
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use core::utils::console::{format_duration_short, ConsoleOutput};

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
                    display_str,
                ]);
            }
            Ok(None) => {
                sample_rows.push(vec![
                    format!("Case {}", i + 1),
                    pnr.clone(),
                    date.to_string(),
                    "No demographics found".to_string(),
                ]);
                warn!("No demographics found for case {} (PNR: {})", i, pnr);
            }
            Err(e) => {
                sample_rows.push(vec![
                    format!("Case {}", i + 1),
                    pnr.clone(),
                    date.to_string(),
                    format!("Error: {}", e),
                ]);
                warn!(
                    "Failed to get demographics for case {} (PNR: {}): {}",
                    i, pnr, e
                );
            }
        }
    }

    if !sample_rows.is_empty() {
        ConsoleOutput::table(&["ID", "PNR", "Date", "Demographics"], &sample_rows);
    }

    // Calculate balance
    ConsoleOutput::subsection("Balance Calculation");
    let calculation_start = Instant::now();
    let balance_results = checker.calculate_balance(cases, controls)?;
    let calculation_time = calculation_start.elapsed();

    ConsoleOutput::key_value("Calculation time", &format_duration_short(calculation_time));
    ConsoleOutput::key_value(
        "Balance summaries",
        &format!("{}", balance_results.summaries.len()),
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
                std_diff_colored,
            ]);
        }

        ConsoleOutput::table(
            &["Variable", "Cases Mean", "Controls Mean", "Std Diff"],
            &balance_rows,
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

// Helper function to resolve a path - absolute or relative to base
#[allow(dead_code)]
fn resolve_path(base_path: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    if Path::new(path).is_absolute() {
        Ok(path.to_string())
    } else {
        // Join relative path to base path
        let full_path = Path::new(base_path).join(path);
        Ok(full_path.to_string_lossy().to_string())
    }
}

/// Handle configuration commands
fn handle_config_command(cmd: &ConfigCommands) -> Result<(), Box<dyn std::error::Error>> {
    use core::utils::console::ConsoleOutput;
    
    match cmd {
        ConfigCommands::GenerateCovariates { output, force, .. } => {
            ConsoleOutput::section("Generating Covariate Configuration");
            ConsoleOutput::info(&format!("Generating default configuration to {}", output));
            
            // Check if file exists and we're not forcing overwrite
            if Path::new(output).exists() && !force {
                ConsoleOutput::error(&format!("File '{}' already exists. Use --force to overwrite.", output));
                return Err("File already exists".into());
            }
            
            // Create a default configuration
            let default_config = CovariatesConfig::default_config();
            
            // Convert to JSON
            let json = serde_json::to_string_pretty(&default_config)
                .map_err(|e| format!("Failed to serialize configuration: {}", e))?;
            
            // Write to file
            fs::write(output, json)?;
            
            ConsoleOutput::success(&format!("Default covariate configuration written to {}", output));
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    // Tests would go here
}

/// Generate structured reports from balance results and matched pairs data
/// 
/// This function demonstrates the use of the structured output manager
/// to create a more organized, web-friendly output structure.
/// 
/// # Arguments
/// * `balance_results` - The balance calculation results
/// * `matched_pairs` - The matched pairs data
/// * `output_dir` - The directory to save reports to
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
fn generate_structured_reports(
    balance_results: &covariates::balance::results::BalanceResults,
    matched_pairs: &[covariates::matched_pairs::record::MatchedPairRecord],
    output_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    use covariates::reporting::StructuredOutputManager;
    use core::utils::console::ConsoleOutput;
    use std::time::Instant;
    
    // Create structured output manager
    ConsoleOutput::subsection("Generating Structured Reports");
    let start_time = Instant::now();
    
    let output_manager = StructuredOutputManager::new(output_dir)?
        .with_runtime_info("command", "generate-structured-reports")
        .with_runtime_info("timestamp", chrono::Local::now().to_string());
    
    // Output balance results
    output_manager.output_balance_results(balance_results, None)?;
    
    // Output matched pairs data
    output_manager.output_matched_pairs(matched_pairs, None)?;
    
    // Generate HTML reports
    output_manager.generate_index_html()?;
    output_manager.generate_data_quality_report()?;
    
    // Log completion
    ConsoleOutput::success(&format!("Generated structured reports in {} seconds", 
        start_time.elapsed().as_secs()));
    ConsoleOutput::info(&format!("Reports available at: {}/report/", output_dir));
    
    Ok(())
}

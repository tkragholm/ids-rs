pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod utils;

// Re-export key types for convenient access
pub use cli::{Cli, Commands, ConfigCommands};
pub use error::{IdsError, IdsResult};

use clap::Parser;
use indicatif::MultiProgress;
use indicatif_log_bridge::LogWrapper;
use std::process;

/// Main entry function for the library
/// 
/// This is the main entry point for the application. It parses command line arguments,
/// sets up logging, and dispatches to the appropriate command handler.
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
/// 
/// # Errors
/// May return errors during:
/// - Command line parsing
/// - Directory creation
/// - Logging initialization
/// - Command execution
/// - File I/O operations
pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    // Check for the most common command line mistake - missing space after --family-file
    for arg in std::env::args() {
        if arg.starts_with("--family-file") && arg != "--family-file" {
            eprintln!(
                "ERROR: Detected possible command line issue. You provided '{arg}' without a space."
            );
            eprintln!("       Did you mean to write: --family-file {}", &arg[13..]);
            eprintln!(
                "       Check other parameters too. Put a space between each flag and its value."
            );
            process::exit(1);
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
    if let Err(e) = LogWrapper::new(multi, logger).try_init() {
        eprintln!("Warning: Failed to initialize logger: {e}");
    }

    // Set the global max log level
    log::set_max_level(level);

    // Parse command line arguments
    let cli = match Cli::try_parse() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{e}");
            eprintln!("\nNOTE: Make sure there is a space between each flag and its value!");
            eprintln!("Example: --family-file data/registers/family.parquet");
            process::exit(1);
        }
    };

    // Create output directories and configure logging
    utils::setup_directories(&cli.output_dir)?;
    utils::configure_logging_with_dir(&cli.output_dir)?;

    // Execute the requested command
    let result = match &cli.command {
        Commands::Config { command } => commands::handle_config_command(command),
        Commands::GenerateRegisters {
            output_dir,
            num_records,
            num_cases,
            start_year,
            end_year,
            seed,
        } => commands::handle_generate_registers(
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
        } => commands::handle_sampling(
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
        } => {
            let config = commands::balance::BalanceCheckConfig {
                matches_file,
                covariate_dir: covariate_dir.as_deref(),
                output_dir: &cli.output_dir,
                family_file: family_file.as_deref(),
                akm_dir: akm_dir.as_deref(),
                bef_dir: bef_dir.as_deref(),
                ind_dir: ind_dir.as_deref(),
                uddf_dir: uddf_dir.as_deref(),
                generate_structured_output: *structured,
            };
            commands::handle_balance_check(&config)
        }
    };
    
    // Convert custom error to standard Box<dyn std::error::Error>
    match result {
        Ok(()) => Ok(()),
        Err(e) => Err(Box::new(e))
    }
}

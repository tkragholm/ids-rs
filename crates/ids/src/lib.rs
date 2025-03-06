pub mod cli;
pub mod commands;
pub mod core;
pub mod diagnostic;
pub mod prelude;
pub mod utils;

// Re-export from prelude for convenience
pub use prelude::*;

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
    // Install color-eyre for better error reporting
    color_eyre::install()?;
    
    // Parse command line arguments with extended error handling
    let cli = cli::parser::parse_cli_args();

    // Initialize logging with progress bars
    initialize_logging()?;
    
    // Create output directories and configure file logging
    utils::setup_directories(&cli.output_dir)?;
    utils::configure_logging_with_dir(&cli.output_dir)?;

    // Execute the requested command
    let result = dispatch_command(&cli)?;
    
    Ok(result)
}

/// Initialize logging with progress bars
/// 
/// Sets up console logging with progress bar integration.
/// 
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
/// 
/// # Errors
/// * Returns an error if logging initialization fails
fn initialize_logging() -> Result<(), Box<dyn std::error::Error>> {
    use indicatif::MultiProgress;
    use indicatif_log_bridge::LogWrapper;
    
    // Create a custom environment with a modified default filter
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
    
    Ok(())
}

/// Dispatch the command to the appropriate handler
/// 
/// Based on the command specified in the CLI arguments, this function
/// calls the appropriate command handler.
/// 
/// # Arguments
/// * `cli` - The parsed CLI arguments
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
/// 
/// # Errors
/// * Returns an error if command execution fails
fn dispatch_command(cli: &cli::types::Cli) -> IdsResult<()> {
    use cli::types::Commands;
    
    match &cli.command {
        Commands::Config { command } => {
            commands::config::handle_config_command(command)
        },
        
        Commands::GenerateRegisters {
            output_dir,
            num_records,
            num_cases,
            start_year,
            end_year,
            seed,
        } => {
            commands::generate::handle_generate_registers(
                output_dir,
                *num_records,
                *num_cases,
                *start_year,
                *end_year,
                *seed,
            )
        },
        
        Commands::Sample {
            input,
            controls,
            birth_window,
            parent_window,
        } => {
            commands::sample::handle_sampling(
                input,
                *controls,
                *birth_window,
                *parent_window,
                &cli.output_dir,
            )
        },
        
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
            let config = commands::balance::config::BalanceCheckConfig {
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
            commands::balance::handler::handle_balance_check(&config)
        }
    }
}

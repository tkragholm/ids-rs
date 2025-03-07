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

    // Create output directories
    utils::setup_directories(&cli.output_dir)?;

    // Initialize logging with progress bars and file output
    // This replaces both the previous initialize_logging and configure_logging_with_dir calls
    initialize_logging_with_files(&cli.output_dir)?;

    // Execute the requested command
    let result = dispatch_command(&cli)?;

    Ok(result)
}

/// Initialize logging with progress bars and file output
///
/// Sets up a combined logging system with:
/// 1. Console output with progress bar integration via indicatif_log_bridge
/// 2. File output for regular logs using utils::SimpleLogger
///
/// # Arguments
/// * `output_dir` - The base output directory where log files will be stored
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
///
/// # Errors
/// * Returns an error if logging initialization fails
fn initialize_logging_with_files(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    use indicatif::MultiProgress;
    use indicatif_log_bridge::LogWrapper;
    use log::LevelFilter;
    use std::path::Path;

    // We use the SimpleLogger from the utils crate, which already supports
    // logging to both console and file

    // Get the log level from environment or use a default
    let rust_log = std::env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string());
    let log_level = match rust_log.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    // Create MultiProgress for progress bars
    let multi = MultiProgress::new();

    // Set up logger for console with progress bars
    let env_logger = env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp(Some(env_logger::TimestampPrecision::Seconds))
        .format_module_path(false)
        .build();

    // Set up the log wrapper with indicatif for progress bars
    if let Err(e) = LogWrapper::new(multi.clone(), env_logger).try_init() {
        // If we can't initialize the logger, it may already be initialized
        eprintln!("Note: Logger may already be initialized: {}", e);

        // Just ensure we have the right log level set
        log::set_max_level(log_level);
    } else {
        // Make sure the log level is correctly set
        log::set_max_level(log_level);
    }

    // Now the logger is set up for the console, add file output
    // Create a log file in the output directory
    let log_file_path = Path::new(output_dir).join("logs").join("ids.log");

    // Ensure the directory exists
    if let Some(parent) = log_file_path.parent() {
        if !parent.exists() {
            std::fs::create_dir_all(parent)?;
        }
    }

    // Create the file for logging
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true) // Append rather than truncate
        .open(&log_file_path)?;

    // Write an initial entry to the log file
    use std::io::Write;
    writeln!(
        file,
        "\n{} - INFO: Logging initialized at {} level for session",
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
        log_level
    )?;

    // Log that we've started
    log::info!("Application started with output directory: {}", output_dir);
    log::info!("Log file: {}", log_file_path.display());

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
        Commands::Config { command } => commands::config::handle_config_command(command),

        Commands::GenerateRegisters {
            output_dir,
            num_records,
            num_cases,
            start_year,
            end_year,
            seed,
        } => commands::generate::handle_generate_registers(
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
        } => commands::sample::handle_sampling(
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

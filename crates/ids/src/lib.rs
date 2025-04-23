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
    dispatch_command(&cli)?;

    Ok(())
}

/// Initialize logging with progress bars and file output
///
/// Sets up a combined logging system with:
/// 1. Console output with progress bar integration via indicatif_log_bridge
/// 2. File output for regular logs using log4rs
/// 3. Creates a session-specific log file for each run with timestamp
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
    use chrono::Local;
    use indicatif::MultiProgress;
    use indicatif_log_bridge::LogWrapper;
    use log::LevelFilter;
    use log4rs::append::rolling_file::policy::compound::roll::fixed_window::FixedWindowRoller;
    use log4rs::append::rolling_file::policy::compound::trigger::size::SizeTrigger;
    use log4rs::append::rolling_file::policy::compound::CompoundPolicy;
    use log4rs::{
        append::{console::ConsoleAppender, file::FileAppender, rolling_file::RollingFileAppender},
        config::{Appender, Config, Logger, Root},
        encode::pattern::PatternEncoder,
        filter::threshold::ThresholdFilter,
    };
    use std::path::Path;

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

    // Ensure the logs directory exists
    let logs_dir = Path::new(output_dir).join("logs");
    if !logs_dir.exists() {
        std::fs::create_dir_all(&logs_dir)?;
    }

    // Create timestamp for this session
    let timestamp = Local::now().format("%Y%m%d_%H%M%S");

    // Prepare log file paths
    let main_log_path = logs_dir.join("ids.log");
    let session_log_path = logs_dir.join(format!("ids_session_{}.log", timestamp));
    let debug_log_path = logs_dir.join(format!("debug_{}.log", timestamp));

    // Create console appender with colored output
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S)} - {h({l})} [{T}] {t} - {m}{n})}",
        )))
        .build();

    // Create file appender for main log (append mode)
    let main_log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)} - {l} [{T}] {t} - {m}{n}",
        )))
        .build(main_log_path.clone())?;

    // Create file appender for session-specific log
    let session_log_file = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S.%3f)} - {l} [{T}] {t} - {m}{n}",
        )))
        .build(session_log_path.clone())?;

    // Create rolling file appender for debug logs with size trigger (10MB)
    let size_trigger = SizeTrigger::new(10 * 1024 * 1024); // 10MB size trigger
    let window_roller = FixedWindowRoller::builder()
        .build(
            &format!("{}/debug_{}.{{}}.log", logs_dir.display(), timestamp),
            5,
        )
        .unwrap();
    let compound_policy = CompoundPolicy::new(Box::new(size_trigger), Box::new(window_roller));

    let debug_log_file = RollingFileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S.%3f)} [{T}] {l} {M}:{L} - {m}{n}",
        )))
        .build(debug_log_path.clone(), Box::new(compound_policy))?;

    // Configure the logging system
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("main_log", Box::new(main_log_file)))
        .appender(Appender::builder().build("session_log", Box::new(session_log_file)))
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Debug)))
                .build("debug_log", Box::new(debug_log_file)),
        )
        // Set up loggers for specific modules to capture more detailed logs
        .logger(
            Logger::builder()
                .appender("debug_log")
                .additive(false)
                .build("types", LevelFilter::Debug),
        )
        .logger(
            Logger::builder()
                .appender("debug_log")
                .additive(false)
                .build("loader", LevelFilter::Debug),
        )
        // Root logger configuration
        .build(
            Root::builder()
                .appender("stdout")
                .appender("main_log")
                .appender("session_log")
                .appender("debug_log")
                .build(log_level),
        )?;

    // Initialize log4rs with the config
    log4rs::init_config(config)?;

    // Set up the log wrapper with indicatif for progress bars integration
    let env_logger = env_logger::Builder::new()
        .filter_level(log_level)
        .format_timestamp(Some(env_logger::TimestampPrecision::Seconds))
        .format_module_path(false)
        .build();

    // Try to initialize the log wrapper (may fail if logger is already initialized)
    // This is okay because we already set up log4rs above
    if let Err(e) = LogWrapper::new(multi.clone(), env_logger).try_init() {
        // If we can't initialize the wrapper, ensure we have the right log level
        eprintln!("Note: Progress bar integration may be limited: {}", e);
        log::set_max_level(log_level);
    }

    // Log that we've started
    log::info!("Application started with output directory: {}", output_dir);
    log::info!("Main log file: {}", main_log_path.display());
    log::info!("Session log file: {}", session_log_path.display());
    log::info!("Debug log file: {}", debug_log_path.display());
    log::debug!("Debug logging enabled");

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

        // Commands::GenerateRegisters {
        //     output_dir,
        //     num_records,
        //     num_cases,
        //     start_year,
        //     end_year,
        //     seed,
        // } => commands::generate::handle_generate_registers(
        //     output_dir,
        //     *num_records,
        //     *num_cases,
        //     *start_year,
        //     *end_year,
        //     *seed,
        // ),
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

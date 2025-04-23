use crate::core::IdsResult;
use log::{debug, error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::path::Path;

/// Configure logging with output directory for log files
///
/// Sets up logging to both console and file, with the log files
/// being written to the output directory.
///
/// # Arguments
/// * `output_dir` - The base output directory path
///
/// # Returns
/// * `IdsResult<()>` - Success or error
///
/// # Errors
/// * Returns an error if logging configuration fails
pub fn configure_logging_with_dir(output_dir: &str) -> IdsResult<()> {
    let log_path = Path::new(output_dir).join("logs").join("ids.log");
    let debug_log_path = Path::new(output_dir).join("logs").join("debug.log");

    debug!("Setting up logging to {}", log_path.display());

    // Create console appender
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{h({d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n})}",
        )))
        .build();

    // Create file appender for normal logs
    let log_file = match FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {l}: {m}{n}",
        )))
        .build(log_path)
    {
        Ok(appender) => appender,
        Err(e) => {
            error!("Failed to create log file: {}", e);
            return Err(crate::core::IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create log file: {e}"),
            )));
        }
    };

    // Create file appender for debug logs
    let debug_file = match FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} [{T}] {l} {M}:{L} - {m}{n}",
        )))
        .build(debug_log_path)
    {
        Ok(appender) => appender,
        Err(e) => {
            error!("Failed to create debug log file: {}", e);
            return Err(crate::core::IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to create debug log file: {e}"),
            )));
        }
    };

    // Configure the logging system
    let config = match Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("log_file", Box::new(log_file)))
        .appender(Appender::builder().build("debug_file", Box::new(debug_file)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("log_file")
                .appender("debug_file")
                .build(LevelFilter::Debug),
        ) {
        Ok(config) => config,
        Err(e) => {
            error!("Failed to build logging config: {}", e);
            return Err(crate::core::IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to build logging config: {e}"),
            )));
        }
    };

    // Initialize the logging system
    if let Err(e) = log4rs::init_config(config) {
        error!("Failed to initialize logging: {}", e);
        return Err(crate::core::IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to initialize logging: {e}"),
        )));
    }

    info!("Logging initialized. Output directory: {}", output_dir);
    debug!("Debug logging enabled");

    Ok(())
}

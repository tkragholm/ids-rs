use crate::error::IdsError;
use log::{info, warn};
use std::fs;
use std::path::Path;

/// Create necessary output directories for application
///
/// This function creates all needed directories including the main output directory,
/// log directory, plots directory, and report directory.
///
/// # Arguments
/// * `output_dir` - The base directory to create
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub fn setup_directories(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let base_path = Path::new(output_dir);

    // Create main output directory and log directory
    fs::create_dir_all(base_path)?;
    fs::create_dir_all(base_path.join("log"))?;

    // Create plots directory for visualizations
    fs::create_dir_all(base_path.join("plots"))?;
    
    // Create report directory for HTML reports
    fs::create_dir_all(base_path.join("report"))?;

    // Note: We no longer create empty register directories by default
    // They will only be created when actually needed for data generation

    info!("Created output directories in {}", output_dir);
    Ok(())
}

/// Configure logging with custom directory
///
/// Sets up logging to both console and files, with different log levels for each.
///
/// # Arguments
/// * `output_dir` - The directory where log files will be saved
///
/// # Returns
/// * `Result<(), Box<dyn std::error::Error>>` - Success or error
pub fn configure_logging_with_dir(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Generate more obvious log path in current directory and output dir
    let current_log_path = "ids_cli.log"; // Log in current directory for easier discovery
    let output_log_path = format!("{output_dir}/log/ids_cli.log");

    // Make sure file is writable - try to create both
    let log_paths = [current_log_path, &output_log_path];

    // Use more descriptive logging levels
    let console_level = log::LevelFilter::Info; // Show more info in console
    let file_level = log::LevelFilter::Debug; // Detailed logs in file

    // Try to initialize all possible logging locations by using existing logging setup
    for path in &log_paths {
        info!("Setting up logger at: {}", path);
        // Use existing logging setup instead of trying to import utils directly
        if let Err(e) = core::utils::configure_logging_with_level(Some(path), file_level) {
            warn!("Failed to set up logger at {}: {}", path, e);
        }
    }

    // Also configure legacy logging to maintain compatibility
    match core::utils::configure_logging_with_level(Some(&output_log_path), console_level) {
        Ok(_) => {},
        Err(e) => {
            // Convert the error type
            return Err(Box::new(IdsError::other(format!(
                "Failed to configure logging: {}", e
            ))));
        }
    }

    info!(
        "Logs are being saved to: {} and {}",
        current_log_path, output_log_path
    );

    Ok(())
}
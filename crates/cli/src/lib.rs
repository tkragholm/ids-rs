pub mod cli;
pub mod main_run;

// Export CLI types that may be used by other crates
pub use cli::{Cli, Commands, ConfigCommands};

use std::path::Path;
use std::fs;
use log::info;
use core::utils::configure_logging_with_level;

// Create output directories
pub fn setup_directories(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
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

// Configure logging with directory
pub fn configure_logging_with_dir(output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let log_path = format!("{}/log/cli.log", output_dir);

    // Use more restrictive logging in the console to reduce terminal noise
    // Only show warnings and errors in the console
    let log_level = log::LevelFilter::Warn;
    
    // This will send logs to both console and file, but we're setting
    // the overall level to Warn to reduce console output
    configure_logging_with_level(Some(&log_path), log_level)?;
    
    Ok(())
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
pub fn generate_structured_reports(
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

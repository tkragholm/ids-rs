use crate::cli::types::ConfigCommands;
use crate::core::IdsResult;
use core::utils::console::ConsoleOutput;
use covariates::core::config::CovariatesConfig;
use std::fs;
use std::path::Path;

/// Handle config related commands
/// 
/// # Arguments
/// * `cmd` - The specific config command to execute
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
/// 
/// # Errors
/// Returns an error if configuration file generation fails due to:
/// - File system issues (e.g., permission denied, disk full)
/// - File already exists and `force` is not set
/// - Serialization errors
pub fn handle_config_command(cmd: &ConfigCommands) -> IdsResult<()> {
    match cmd {
        ConfigCommands::GenerateCovariates { output, force } => {
            generate_covariates_config(output, *force)
        }
    }
}

/// Generate a default covariates configuration file
/// 
/// # Arguments
/// * `output_path` - Path where the configuration file should be saved
/// * `force` - Whether to overwrite an existing file
/// 
/// # Returns
/// * `IdsResult<()>` - Success or error
fn generate_covariates_config(output_path: &str, force: bool) -> IdsResult<()> {
    ConsoleOutput::section("Generating Covariates Configuration");
    
    // Check if file already exists
    let output_file = Path::new(output_path);
    if output_file.exists() && !force {
        ConsoleOutput::error(&format!(
            "Output file {output_path} already exists. Use --force to overwrite."
        ));
        return Err(crate::core::IdsError::config(format!(
            "Output file {output_path} already exists. Use --force to overwrite."
        )));
    }
    
    // Create parent directories if needed
    if let Some(parent) = output_file.parent() {
        if !parent.exists() {
            ConsoleOutput::info(&format!("Creating directory: {}", parent.display()));
            fs::create_dir_all(parent)?;
        }
    }
    
    // Generate default configuration
    let config = CovariatesConfig::default_config();
    let json = serde_json::to_string_pretty(&config)?;
    
    // Write to file
    fs::write(output_path, json)?;
    ConsoleOutput::success(&format!("Configuration written to {output_path}"));
    
    Ok(())
}
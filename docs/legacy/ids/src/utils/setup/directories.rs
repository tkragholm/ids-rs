use crate::core::IdsResult;
use log::{error, info};
use std::fs;
use std::path::Path;

/// Set up output directories for the application
///
/// Creates the main output directory and subdirectories for reports, plots, etc.
///
/// # Arguments
/// * `output_dir` - The base output directory path
///
/// # Returns
/// * `IdsResult<()>` - Success or error
///
/// # Errors
/// * Returns an error if directory creation fails
pub fn setup_directories(output_dir: &str) -> IdsResult<()> {
    // Ensure output directory exists
    let output_path = Path::new(output_dir);
    if !output_path.exists() {
        info!("Creating output directory: {output_dir}");
        fs::create_dir_all(output_path)?;
    }

    // Create subdirectories
    let paths = [
        output_path.join("report"),
        output_path.join("plots"),
        output_path.join("data"),
        output_path.join("logs"),
    ];

    for path in &paths {
        if !path.exists() {
            info!("Creating directory: {}", path.display());
            if let Err(e) = fs::create_dir_all(path) {
                error!("Failed to create directory {}: {}", path.display(), e);
                return Err(crate::core::IdsError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to create directory {}: {}", path.display(), e),
                )));
            }
        }
    }

    Ok(())
}

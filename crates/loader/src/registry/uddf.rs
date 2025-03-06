use std::collections::HashSet;
use std::path::{Path, PathBuf};
use arrow::record_batch::RecordBatch;
use types::error::IdsError;

use crate::formats::load_parquet_files_parallel;
use crate::schema::uddf_schema;
use crate::ui::LoaderProgress;

/// Load Education Register (UDDF) data from the given path
///
/// This function discovers and loads UDDF (Education Register) Parquet files
/// from the specified directory.
///
/// # Arguments
/// * `base_path` - Path to the directory containing UDDF data
/// * `pnr_filter` - Optional set of PNRs to filter the data by
///
/// # Returns
/// A vector of RecordBatches containing UDDF data or an error
///
/// # Errors
/// Returns an error if:
/// - The path doesn't exist
/// - The files can't be parsed as Parquet
/// - The data doesn't match the expected schema
pub fn load_uddf(
    base_path: &str,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Loading UDDF data from {}", base_path);
    
    // Create a progress tracker
    let progress = LoaderProgress::new();
    progress.set_main_message("Loading UDDF data");
    
    // Normalize path handling for both directory and file scenarios
    let path = Path::new(base_path);
    
    // Determine the actual path to search
    let uddf_path = if path.is_dir() {
        // Check for a "uddf" subdirectory
        let uddf_dir = path.join("uddf");
        if uddf_dir.exists() && uddf_dir.is_dir() {
            uddf_dir
        } else {
            // Check for a "registers" subdirectory with "uddf" under it
            let registers_uddf = path.join("registers").join("uddf");
            if registers_uddf.exists() && registers_uddf.is_dir() {
                registers_uddf
            } else {
                // Fallback to the base path
                path.to_path_buf()
            }
        }
    } else {
        // If path is a file, use its parent directory
        path.parent()
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."))
    };
    
    // Get the schema for UDDF data
    let schema = uddf_schema();
    
    // Load the Parquet files
    let batches = if uddf_path.is_dir() {
        load_parquet_files_parallel(&uddf_path, Some(&schema), pnr_filter, Some(&progress))?
    } else if path.exists() && path.extension().map_or(false, |ext| ext == "parquet") {
        // If the path is a direct Parquet file
        crate::formats::read_parquet(path, Some(&schema), Some(&progress), pnr_filter)?
    } else {
        log::warn!("No UDDF data found at {}", base_path);
        Vec::new()
    };
    
    log::info!("Loaded {} record batches of UDDF data", batches.len());
    Ok(batches)
}
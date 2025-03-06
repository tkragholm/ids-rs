use std::collections::HashSet;
use std::path::{Path, PathBuf};
use arrow::record_batch::RecordBatch;
use types::error::IdsError;

use crate::formats::load_parquet_files_parallel;
use crate::schema::ind_schema;
use crate::ui::LoaderProgress;

/// Load Individual Register (IND) data from the given path
///
/// This function discovers and loads IND (Individual Register) Parquet files
/// from the specified directory.
///
/// # Arguments
/// * `base_path` - Path to the directory containing IND data
/// * `pnr_filter` - Optional set of PNRs to filter the data by
///
/// # Returns
/// A vector of RecordBatches containing IND data or an error
///
/// # Errors
/// Returns an error if:
/// - The path doesn't exist
/// - The files can't be parsed as Parquet
/// - The data doesn't match the expected schema
pub fn load_ind(
    base_path: &str,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Loading IND data from {}", base_path);
    
    // Create a progress tracker
    let progress = LoaderProgress::new();
    progress.set_main_message("Loading IND data");
    
    // Normalize path handling for both directory and file scenarios
    let path = Path::new(base_path);
    
    // Determine the actual path to search
    let ind_path = if path.is_dir() {
        // Check for an "ind" subdirectory
        let ind_dir = path.join("ind");
        if ind_dir.exists() && ind_dir.is_dir() {
            ind_dir
        } else {
            // Check for a "registers" subdirectory with "ind" under it
            let registers_ind = path.join("registers").join("ind");
            if registers_ind.exists() && registers_ind.is_dir() {
                registers_ind
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
    
    // Get the schema for IND data
    let schema = ind_schema();
    
    // Load the Parquet files
    let batches = if ind_path.is_dir() {
        load_parquet_files_parallel(&ind_path, Some(&schema), pnr_filter, Some(&progress))?
    } else if path.exists() && path.extension().map_or(false, |ext| ext == "parquet") {
        // If the path is a direct Parquet file
        crate::formats::read_parquet(path, Some(&schema), Some(&progress), pnr_filter)?
    } else {
        log::warn!("No IND data found at {}", base_path);
        Vec::new()
    };
    
    log::info!("Loaded {} record batches of IND data", batches.len());
    Ok(batches)
}
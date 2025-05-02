use arrow::record_batch::RecordBatch;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use types::error::IdsError;

use crate::formats::load_parquet_files_parallel;
use crate::schema::bef_schema;
use crate::ui::LoaderProgress;

/// Load Population Register (BEF) data from the given path
///
/// This function discovers and loads BEF (Population Register) Parquet files
/// from the specified directory.
///
/// # Arguments
/// * `base_path` - Path to the directory containing BEF data
/// * `pnr_filter` - Optional set of PNRs to filter the data by
///
/// # Returns
/// A vector of `RecordBatches` containing BEF data or an error
///
/// # Errors
/// Returns an error if:
/// - The path doesn't exist
/// - The files can't be parsed as Parquet
/// - The data doesn't match the expected schema
pub fn load_bef(
    base_path: &str,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Loading BEF data from {base_path}");

    // Create a progress tracker
    let progress = LoaderProgress::new();
    progress.set_main_message("Loading BEF data");

    // Normalize path handling for both directory and file scenarios
    let path = Path::new(base_path);

    // Determine the actual path to search
    let bef_path = if path.is_dir() {
        // Check for a "bef" subdirectory
        let bef_dir = path.join("bef");
        if bef_dir.exists() && bef_dir.is_dir() {
            bef_dir
        } else {
            // Check for a "registers" subdirectory with "bef" under it
            let registers_bef = path.join("registers").join("bef");
            if registers_bef.exists() && registers_bef.is_dir() {
                registers_bef
            } else {
                // Fallback to the base path
                path.to_path_buf()
            }
        }
    } else {
        // If path is a file, use its parent directory
        path.parent().map_or_else(|| PathBuf::from("."), PathBuf::from)
    };

    // Get the schema for BEF data
    let schema = bef_schema();

    // Load the Parquet files
    let batches = if bef_path.is_dir() {
        load_parquet_files_parallel(&bef_path, Some(&schema), pnr_filter, Some(&progress))?
    } else if path.exists() && path.extension().is_some_and(|ext| ext == "parquet") {
        // If the path is a direct Parquet file
        crate::formats::read_parquet(path, Some(&schema), Some(&progress), pnr_filter)?
    } else {
        log::warn!("No BEF data found at {base_path}");
        Vec::new()
    };

    log::info!("Loaded {} record batches of BEF data", batches.len());
    Ok(batches)
}

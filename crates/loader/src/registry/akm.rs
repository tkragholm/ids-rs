use arrow::record_batch::RecordBatch;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use types::error::IdsError;

use crate::formats::load_parquet_files_parallel;
use crate::schema::akm_schema;
use crate::ui::LoaderProgress;

/// Load Annual Register (AKM) data from the given path
///
/// This function discovers and loads AKM (Annual Register) Parquet files
/// from the specified directory.
///
/// # Arguments
/// * `base_path` - Path to the directory containing AKM data
/// * `pnr_filter` - Optional set of PNRs to filter the data by
///
/// # Returns
/// A vector of RecordBatches containing AKM data or an error
///
/// # Errors
/// Returns an error if:
/// - The path doesn't exist
/// - The files can't be parsed as Parquet
/// - The data doesn't match the expected schema
pub fn load_akm(
    base_path: &str,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Loading AKM data from {}", base_path);

    // Create a progress tracker
    let progress = LoaderProgress::new();
    progress.set_main_message("Loading AKM data");

    // Normalize path handling for both directory and file scenarios
    let path = Path::new(base_path);

    // Determine the actual path to search
    let akm_path = if path.is_dir() {
        // Check for an "akm" subdirectory
        let akm_dir = path.join("akm");
        if akm_dir.exists() && akm_dir.is_dir() {
            akm_dir
        } else {
            // Check for a "registers" subdirectory with "akm" under it
            let registers_akm = path.join("registers").join("akm");
            if registers_akm.exists() && registers_akm.is_dir() {
                registers_akm
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

    // Get the schema for AKM data
    let schema = akm_schema();

    // Load the Parquet files
    let batches = if akm_path.is_dir() {
        load_parquet_files_parallel(&akm_path, Some(&schema), pnr_filter, Some(&progress))?
    } else if path.exists() && path.extension().is_some_and(|ext| ext == "parquet") {
        // If the path is a direct Parquet file
        crate::formats::read_parquet(path, Some(&schema), Some(&progress), pnr_filter)?
    } else {
        log::warn!("No AKM data found at {}", base_path);
        Vec::new()
    };

    log::info!("Loaded {} record batches of AKM data", batches.len());
    Ok(batches)
}

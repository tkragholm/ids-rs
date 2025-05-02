use arrow::record_batch::RecordBatch;
use std::collections::HashSet;
use std::path::{Path, PathBuf};
use types::error::IdsError;

use crate::formats::read_parquet;
use crate::schema::family_schema;
use crate::ui::LoaderProgress;

/// Load Family Relations data from the given path
///
/// This function discovers and loads Family Relations Parquet file
/// from the specified directory.
///
/// # Arguments
/// * `base_path` - Path to the directory containing family relation data
/// * `pnr_filter` - Optional set of PNRs to filter the data by
///
/// # Returns
/// A vector of `RecordBatches` containing family relation data or an error
///
/// # Errors
/// Returns an error if:
/// - The path doesn't exist
/// - The file can't be parsed as Parquet
/// - The data doesn't match the expected schema
pub fn load_family(
    base_path: &str,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Loading family relation data from {base_path}");

    // Create a progress tracker
    let progress = LoaderProgress::new();
    progress.set_main_message("Loading family relation data");

    // Normalize path handling
    let path = Path::new(base_path);

    // Try to find the family file in different possible locations
    let family_file = if path.is_dir() {
        // Check for a family.parquet directly in the base path
        let direct_file = path.join("family.parquet");
        if direct_file.exists() && direct_file.is_file() {
            direct_file
        } else {
            // Check for a "family" subdirectory with "family.parquet" in it
            let family_dir_file = path.join("family").join("family.parquet");
            if family_dir_file.exists() && family_dir_file.is_file() {
                family_dir_file
            } else {
                // Check for a "registers" subdirectory
                let registers_file = path.join("registers").join("family").join("family.parquet");
                if registers_file.exists() && registers_file.is_file() {
                    registers_file
                } else {
                    PathBuf::from(base_path)
                }
            }
        }
    } else if path.exists() && path.extension().is_some_and(|ext| ext == "parquet") {
        // If the path is directly to a Parquet file
        path.to_path_buf()
    } else {
        PathBuf::from(base_path)
    };

    // Get the schema for family data
    let schema = family_schema();

    // Load the Parquet file
    let batches =
        if family_file.exists() && family_file.extension().is_some_and(|ext| ext == "parquet") {
            read_parquet(&family_file, Some(&schema), Some(&progress), pnr_filter)?
        } else {
            log::warn!("No family relation data found at {base_path}");
            Vec::new()
        };

    log::info!(
        "Loaded {} record batches of family relation data",
        batches.len()
    );
    Ok(batches)
}

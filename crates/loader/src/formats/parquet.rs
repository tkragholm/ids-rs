use crate::ui::LoaderProgress;
use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;

use log::{debug, info, warn};
use parking_lot::Mutex;
use parquet_integration::{ParallelParquetLoader, ParquetConfig, create_optimized_loader};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use types::error::IdsError;

/// Read a Parquet file and return its contents as a vector of `RecordBatches`.
///
/// Uses the high-performance optimized parquet reader for better performance.
///
/// # Arguments
/// * `path` - File path to the Parquet file to be read
/// * `schema` - Optional Arrow Schema for projecting specific columns
/// * `progress` - Optional progress tracker for user feedback
/// * `pnr_filter` - Optional set of PNRs to filter the data by
///
/// # Returns
/// Vector of `RecordBatches` or an error
///
/// # Errors
/// Returns an error if:
/// - The file cannot be opened
/// - The file is not a valid Parquet file
/// - There are issues reading the record batches
pub fn read_parquet(
    path: &Path,
    schema: Option<&Schema>,
    progress: Option<&LoaderProgress>,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    info!("Reading parquet file: {}", path.display());

    // Check if the file exists
    if !path.exists() {
        return Err(IdsError::io_error(format!("File not found: {}", path.display())));
    }

    // Create a progress bar if provided
    let progress_bar = if let Some(progress) = progress {
        let file_size = std::fs::metadata(path)
            .map(|m| m.len())
            .unwrap_or(1000);
            
        let filename = path.file_name()
            .and_then(|f| f.to_str())
            .unwrap_or("unknown");
            
        Some(progress.create_file_progress(file_size, filename))
    } else {
        None
    };

    // Create a configuration for the reader
    let mut config = ParquetConfig::new()
        .with_path(path.to_path_buf())
        .with_progress(progress.is_some());
        
    // Add PNR filter if provided
    if let Some(pnr_filter) = pnr_filter {
        config = config.with_pnr_filter(pnr_filter.clone());
    }
    
    // Get result based on whether we have a schema
    let result = if let Some(schema) = schema {
        // Read with schema projection
        let loader = create_optimized_loader(); // Use optimized settings
        loader.load_with_schema(path, schema)
    } else {
        // Read without schema projection
        parquet_integration::read_parquet_file(path, &config)
    }.map_err(|e| {
        IdsError::invalid_operation(format!("Failed to read parquet file {}: {}", path.display(), e))
    })?;
    
    // Update progress if provided
    if let Some(pb) = progress_bar {
        pb.finish_with_message("Done");
    }
    
    // Log results
    info!(
        "Read {} batches with {} rows from {}",
        result.len(),
        result.iter().map(|b| b.num_rows()).sum::<usize>(),
        path.display()
    );
    
    Ok(result)
}

/// Read a Parquet file with PNR filtering applied.
///
/// This is a convenience wrapper around `read_parquet` that applies a PNR filter.
///
/// # Arguments
/// * `path` - File path to the Parquet file to be read
/// * `schema` - Optional Arrow Schema for projecting specific columns
/// * `pnr_filter` - Set of PNRs to filter the data by
/// * `progress` - Optional progress tracker for user feedback
///
/// # Returns
/// Vector of `RecordBatches` or an error
///
/// # Errors
/// Returns an error if the underlying `read_parquet` function fails
pub fn read_parquet_with_filter(
    path: &Path,
    schema: Option<&Schema>,
    pnr_filter: &HashSet<String>,
    progress: Option<&LoaderProgress>,
) -> Result<Vec<RecordBatch>, IdsError> {
    read_parquet(path, schema, progress, Some(pnr_filter))
}

/// Load Parquet files in parallel from a directory.
///
/// Uses the high-performance optimized parquet reader to scan and load files in parallel.
///
/// # Arguments
/// * `dir_path` - Directory containing Parquet files
/// * `schema` - Optional Arrow Schema for projecting specific columns
/// * `pnr_filter` - Optional set of PNRs to filter by
/// * `progress` - Optional progress tracker for user feedback
///
/// # Returns
/// Vector of `RecordBatches` from all Parquet files or an error
///
/// # Errors
/// Returns an error if:
/// - The directory cannot be read
/// - Any Parquet file cannot be read
pub fn load_parquet_files_parallel(
    dir_path: &Path,
    schema: Option<&Schema>,
    pnr_filter: Option<&HashSet<String>>,
    progress: Option<&LoaderProgress>,
) -> Result<Vec<RecordBatch>, IdsError> {
    info!("Loading Parquet files from directory: {}", dir_path.display());

    // Create a configuration for the reader
    let mut config = ParquetConfig::new()
        .with_path(dir_path.to_path_buf())
        .with_progress(progress.is_some());
        
    // Update progress display if provided
    if let Some(progress) = progress {
        progress.set_main_message(&format!(
            "Loading Parquet files from {}",
            dir_path.display()
        ));
    }
        
    // Add PNR filter if provided
    if let Some(pnr_filter) = pnr_filter {
        config = config.with_pnr_filter(pnr_filter.clone());
    }
    
    // Create an optimized loader
    let loader = create_optimized_loader();
    
    // Load files based on whether we have a schema
    let result = if let Some(schema) = schema {
        // Find files in the directory
        let mut files = match parquet_integration::utils::find_parquet_files(dir_path) {
            Ok(files) => files,
            Err(e) => return Err(IdsError::io_error(format!("Failed to scan directory {}: {}", dir_path.display(), e))),
        };
        
        // Sort files by modification time
        if let Err(e) = parquet_integration::utils::sort_files_by_modification_time(&mut files) {
            warn!("Failed to sort files by modification time: {}", e);
        }
        
        // Load each file with schema projection
        let batches = Arc::new(Mutex::new(Vec::new()));
        
        // Process files in parallel
        files.into_iter().for_each(|path| {
            match loader.load_with_schema(&path, schema) {
                Ok(file_batches) => {
                    let mut all_batches = batches.lock();
                    all_batches.extend(file_batches);
                }
                Err(err) => {
                    warn!("Error reading file {}: {}", path.display(), err);
                }
            }
        });
        
        Ok(Arc::try_unwrap(batches).unwrap_or_else(|arc| arc.lock().clone()).into_inner())
    } else {
        // Use the directory loader with our configuration
        parquet_integration::load_directory(dir_path, &config)
    }.map_err(|e| {
        IdsError::invalid_operation(format!(
            "Failed to load parquet files from directory {}: {}",
            dir_path.display(), e
        ))
    })?;
    
    // Log results
    info!(
        "Successfully loaded {} batches with {} rows from directory {}",
        result.len(),
        result.iter().map(|b| b.num_rows()).sum::<usize>(),
        dir_path.display()
    );
    
    Ok(result)
}
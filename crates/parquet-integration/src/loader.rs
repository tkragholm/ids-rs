use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_array::RecordBatch;
use arrow_schema::Schema;
use log::{debug, info};
use tokio::runtime::Runtime;

use crate::config::ParquetConfig;
use crate::engine::ParquetReader;
use crate::error::{ParquetIntegrationError, Result};
use crate::filter::post_process_batches;
use crate::utils::{self, Timer};

/// Read a single parquet file with the given configuration
///
/// This function uses the optimized parquet reader to efficiently read a parquet file,
/// with adaptive reading strategies based on file characteristics.
///
/// # Arguments
/// * `path` - Path to the parquet file
/// * `config` - Configuration for the reader
///
/// # Returns
/// Vector of RecordBatches or an error
pub fn read_parquet_file(path: &Path, config: &ParquetConfig) -> Result<Vec<RecordBatch>> {
    // Create a timer for performance tracking
    let mut timer = utils::Timer::new();
    
    // Validate the file
    if !path.exists() {
        return Err(ParquetIntegrationError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path.display()),
        )));
    }
    
    if !utils::is_parquet_file(path) {
        return Err(ParquetIntegrationError::OperationError(format!(
            "Not a parquet file: {}",
            path.display()
        )));
    }
    
    // Create runtime for async execution
    let rt = Runtime::new().map_err(|e| {
        ParquetIntegrationError::OperationError(format!(
            "Failed to create runtime: {}",
            e
        ))
    })?;
    
    // Create the reader with the provided configuration
    let reader = ParquetReader::with_config(config.clone());
    timer.checkpoint("reader_created");
    
    // Read the file
    info!("Reading parquet file: {}", path.display());
    let result = rt.block_on(async {
        reader.read_file(path).await
    })?;
    timer.checkpoint("file_read");
    
    // Log performance metrics
    debug!("Parquet file reading performance:\n{}", timer.report());
    debug!(
        "Read {} batches with {} rows from {}", 
        result.batches.len(),
        result.batches.iter().map(|b| b.num_rows()).sum::<usize>(),
        path.display()
    );
    
    Ok(result.batches)
}

/// Load a Parquet file with schema projection
///
/// This function reads a parquet file with a specific schema projection.
///
/// # Arguments
/// * `path` - Path to the parquet file
/// * `schema` - Arrow schema to project
///
/// # Returns
/// Vector of RecordBatches or an error
pub fn load_with_schema(path: &Path, schema: &Schema) -> Result<Vec<RecordBatch>> {
    // Create a reader with default configuration
    let reader = ParquetReader::new();
    
    // Use the schema projection function
    reader.load_with_schema(path, schema)
}

/// Load parquet files from a directory
///
/// This function scans a directory for parquet files and loads them using the optimized
/// parquet reader.
///
/// # Arguments
/// * `dir_path` - Directory containing parquet files
/// * `config` - Configuration for the reader
///
/// # Returns
/// Vector of RecordBatches or an error
pub fn load_directory(dir_path: &Path, config: &ParquetConfig) -> Result<Vec<RecordBatch>> {
    // Create a timer for performance tracking
    let mut timer = utils::Timer::new();
    
    // Validate the directory
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(ParquetIntegrationError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory not found: {}", dir_path.display()),
        )));
    }
    
    // Find parquet files in the directory (recursive if configured)
    let mut files = utils::find_parquet_files_with_options(dir_path, config.recursive)?;
    timer.checkpoint("found_files");
    
    // Sort by modification time (newest first)
    utils::sort_files_by_modification_time(&mut files)?;
    timer.checkpoint("sorted_files");
    
    info!("Found {} parquet files in {}", files.len(), dir_path.display());
    
    // Check if files were found
    if files.is_empty() {
        return Ok(Vec::new());
    }
    
    // Use the parallel loader for multiple files
    if files.len() > 1 && config.parallel {
        // Create a parallel loader with this configuration
        let loader = crate::parallel::ParallelParquetLoader::new(config);
        let batches = loader.load_files(&files)?;
        timer.checkpoint("loaded_files_parallel");
        
        // Log performance
        debug!("Parallel loading performance:\n{}", timer.report());
        
        Ok(batches)
    } else {
        // Load files sequentially
        let mut all_batches = Vec::new();
        
        for file in files {
            match read_parquet_file(&file, config) {
                Ok(batches) => {
                    all_batches.extend(batches);
                }
                Err(e) => {
                    log::warn!("Error reading file {}: {}", file.display(), e);
                }
            }
        }
        
        timer.checkpoint("loaded_files_sequential");
        
        // Log performance
        debug!("Sequential loading performance:\n{}", timer.report());
        
        Ok(all_batches)
    }
}
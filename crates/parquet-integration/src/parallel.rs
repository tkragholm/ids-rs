use std::path::{Path, PathBuf};
use std::sync::Arc;

use arrow_array::RecordBatch;
use arrow_schema::Schema;
use log::{info, warn};
use parking_lot::Mutex;
use rayon::prelude::*;

use crate::config::{ParquetConfig, ReaderMode};
use crate::error::{ParquetIntegrationError, Result};
use crate::loader;
use crate::utils;

/// A high-performance parallel loader for parquet files
#[derive(Debug)]
pub struct ParallelParquetLoader {
    config: ParquetConfig,
}

impl ParallelParquetLoader {
    /// Create a new loader with default configuration
    pub fn new() -> Self {
        Self {
            config: ParquetConfig::default(),
        }
    }
    
    /// Create a new loader with custom configuration
    pub fn with_config(config: ParquetConfig) -> Self {
        Self { config }
    }
    
    /// Create a new loader with provided configuration
    pub fn new(config: &ParquetConfig) -> Self {
        Self { 
            config: config.clone(),
        }
    }
    
    /// Load all parquet files from a directory, processing them in parallel
    pub fn load_from_directory(&self, dir_path: impl AsRef<Path>) -> Result<Vec<RecordBatch>> {
        let dir_path = dir_path.as_ref();
        info!("Loading parquet files from directory: {}", dir_path.display());
        
        // Find all parquet files in the directory
        let mut parquet_files = utils::find_parquet_files_with_options(dir_path, self.config.recursive)?;
        
        if parquet_files.is_empty() {
            warn!("No parquet files found in directory: {}", dir_path.display());
            return Ok(Vec::new());
        }
        
        // Sort by modification time (newest first)
        utils::sort_files_by_modification_time(&mut parquet_files)?;
        
        // Load all files in parallel
        self.load_files(&parquet_files)
    }
    
    /// Load multiple parquet files in parallel
    pub fn load_files(&self, files: &[PathBuf]) -> Result<Vec<RecordBatch>> {
        // Process files in parallel using rayon
        let batches = Arc::new(Mutex::new(Vec::new()));
        let errors = Arc::new(Mutex::new(Vec::new()));
        
        info!("Loading {} parquet files in parallel", files.len());
        
        // Configure thread count for rayon
        let thread_count = if self.config.threads > 0 {
            self.config.threads
        } else {
            num_cpus::get()
        };
        
        // Create a thread pool with the configured number of threads
        rayon::ThreadPoolBuilder::new()
            .num_threads(thread_count)
            .build_global()
            .unwrap_or_else(|_| {
                warn!("Failed to configure thread pool, using default");
            });
        
        // Use rayon for parallel processing
        files.par_iter().for_each(|path| {
            match loader::read_parquet_file(path, &self.config) {
                Ok(file_batches) => {
                    let mut all_batches = batches.lock();
                    all_batches.extend(file_batches);
                }
                Err(err) => {
                    let mut error_list = errors.lock();
                    error_list.push((path.clone(), err));
                }
            }
        });
        
        // Check for errors
        let error_list = errors.lock();
        if !error_list.is_empty() {
            // Log errors but continue with what we have
            for (path, err) in error_list.iter() {
                warn!("Error reading file {}: {}", path.display(), err);
            }
        }
        
        Ok(Arc::try_unwrap(batches).unwrap_or_else(|arc| arc.lock().clone()).into_inner())
    }
    
    /// Load a parquet file with schema projection
    pub fn load_with_schema(
        &self, 
        path: impl AsRef<Path>,
        schema: &Schema,
    ) -> Result<Vec<RecordBatch>> {
        let path = path.as_ref();
        
        // Use the schema projection loader
        loader::load_with_schema(path, schema)
    }
}

/// Create a new parallel loader with optimized settings for the current system
pub fn create_optimized_loader() -> ParallelParquetLoader {
    // Get available system resources
    let cpu_count = num_cpus::get();
    
    // Create an optimized configuration
    let config = ParquetConfig::default()
        .with_threads(cpu_count)
        .with_parallel(true)
        .with_batch_size(262144) // 256K rows per batch
        .with_mode(ReaderMode::Adaptive);
        
    ParallelParquetLoader::with_config(config)
}
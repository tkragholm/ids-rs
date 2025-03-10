use std::path::Path;
use std::sync::Arc;

use log::{debug, info};
use parquet::file::metadata::ParquetMetaData;
use sysinfo::{System, SystemExt};

use crate::config::{ParquetConfig, ReaderMode};
use crate::error::{ParquetIntegrationError, Result};

/// Threshold values for optimization
const SMALL_FILE_THRESHOLD: usize = 100 * 1024 * 1024; // 100MB
const LARGE_FILE_THRESHOLD: usize = 500 * 1024 * 1024; // 500MB
const HUGE_FILE_THRESHOLD: usize = 2 * 1024 * 1024 * 1024; // 2GB

/// Memory thresholds
const LOW_MEMORY_THRESHOLD: u64 = 4 * 1024 * 1024 * 1024; // 4GB
const HIGH_MEMORY_THRESHOLD: u64 = 16 * 1024 * 1024 * 1024; // 16GB

/// Optimizer for reading strategy
pub struct Optimizer {
    /// Configuration
    config: Arc<ParquetConfig>,
    
    /// System information
    system: System,
}

impl Optimizer {
    /// Create a new optimizer
    pub fn new(config: Arc<ParquetConfig>) -> Self {
        Self {
            config,
            system: System::new_all(),
        }
    }
    
    /// Choose the best reading mode for a file
    pub async fn choose_read_mode(&self, path: &Path, file_size: usize) -> Result<ReaderMode> {
        // If mode is explicitly set, use that
        if self.config.mode != ReaderMode::Adaptive {
            return Ok(self.config.mode);
        }
        
        // Refresh system info
        let mut system = self.system.clone();
        system.refresh_all();
        
        // Get available memory
        let available_memory = system.available_memory();
        
        // Choose based on file size and available memory
        let mode = if file_size < SMALL_FILE_THRESHOLD {
            // Small files use async mode for best performance
            ReaderMode::Async
        } else if file_size < LARGE_FILE_THRESHOLD {
            // Medium files use sync or async based on memory
            if available_memory < LOW_MEMORY_THRESHOLD {
                ReaderMode::Sync
            } else {
                ReaderMode::Async
            }
        } else if file_size < HUGE_FILE_THRESHOLD {
            // Large files use row group mode for memory efficiency
            if available_memory < HIGH_MEMORY_THRESHOLD {
                ReaderMode::RowGroup
            } else {
                ReaderMode::Sync
            }
        } else {
            // Huge files always use row group mode
            ReaderMode::RowGroup
        };
        
        debug!(
            "Optimizing read mode: file_size={}, available_memory={}, chosen_mode={:?}",
            file_size, available_memory, mode
        );
        
        Ok(mode)
    }
    
    /// Calculate optimal batch size based on file characteristics
    pub fn calculate_optimal_batch_size(&self, metadata: &ParquetMetaData) -> usize {
        // If batch size is explicitly set, use that
        if self.config.batch_size > 0 {
            return self.config.batch_size;
        }
        
        // Get total rows from metadata
        let total_rows = metadata.row_groups()
            .iter()
            .map(|rg| rg.num_rows())
            .sum::<i64>() as usize;
            
        // Calculate rows per row group
        let row_groups = metadata.row_groups().len();
        let avg_rows_per_group = if row_groups > 0 {
            total_rows / row_groups
        } else {
            10000 // Default if no row groups (unlikely)
        };
        
        // Default to 128K rows per batch for large datasets
        let default_batch_size = 131072;
        
        // For very small datasets, use a smaller batch size
        if total_rows < 10000 {
            return total_rows;
        }
        
        // For medium datasets, consider row group size
        if avg_rows_per_group < default_batch_size {
            // Use a multiple of the row group size for better alignment
            let multiple = (default_batch_size + avg_rows_per_group - 1) / avg_rows_per_group;
            return avg_rows_per_group * multiple.min(4); // Up to 4x row group size
        }
        
        // For large datasets, use the default
        default_batch_size
    }
}
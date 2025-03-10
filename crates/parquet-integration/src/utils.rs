use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use log::debug;
use once_cell::sync::Lazy;
use sysinfo::{System, SystemExt};

use crate::error::{ParquetIntegrationError, Result};

/// Check if a file exists and is a parquet file
pub fn is_parquet_file(path: &Path) -> bool {
    path.exists() && path.is_file() && path.extension().map_or(false, |ext| ext == "parquet")
}

/// Find all parquet files in a directory
pub fn find_parquet_files(dir_path: &Path) -> Result<Vec<PathBuf>> {
    find_parquet_files_with_options(dir_path, true)
}

/// Find all parquet files in a directory with recursive option
pub fn find_parquet_files_with_options(dir_path: &Path, recursive: bool) -> Result<Vec<PathBuf>> {
    if !dir_path.exists() || !dir_path.is_dir() {
        return Err(ParquetIntegrationError::IoError(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory not found: {}", dir_path.display()),
        )));
    }

    let mut parquet_files = Vec::new();
    for entry in std::fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() && is_parquet_file(&path) {
            parquet_files.push(path);
        } else if recursive && path.is_dir() {
            // Recurse into subdirectories if requested
            match find_parquet_files_with_options(&path, true) {
                Ok(mut sub_files) => parquet_files.append(&mut sub_files),
                Err(e) => debug!("Error reading subdirectory {}: {}", path.display(), e),
            }
        }
    }

    Ok(parquet_files)
}

/// Sort parquet files by modification time (newest first)
pub fn sort_files_by_modification_time(files: &mut [PathBuf]) -> Result<()> {
    files.sort_by(|a, b| {
        let a_meta = match std::fs::metadata(a) {
            Ok(meta) => meta,
            Err(_) => return std::cmp::Ordering::Equal,
        };
        let b_meta = match std::fs::metadata(b) {
            Ok(meta) => meta,
            Err(_) => return std::cmp::Ordering::Equal,
        };
        b_meta
            .modified()
            .unwrap_or_else(|_| std::time::SystemTime::now())
            .cmp(
                &a_meta
                    .modified()
                    .unwrap_or_else(|_| std::time::SystemTime::now()),
            )
    });

    Ok(())
}

/// Timer utility for measuring performance
pub struct Timer {
    start: Instant,
    checkpoints: Vec<(String, Duration)>,
}

impl Timer {
    /// Create a new timer
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            checkpoints: Vec::new(),
        }
    }

    /// Add a checkpoint with a label
    pub fn checkpoint(&mut self, label: &str) {
        self.checkpoints.push((label.to_string(), self.start.elapsed()));
    }

    /// Get the elapsed time since the start
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Get all checkpoints
    pub fn checkpoints(&self) -> &[(String, Duration)] {
        &self.checkpoints
    }

    /// Print a performance report
    pub fn report(&self) -> String {
        let mut report = String::new();
        report.push_str(&format!("Total time: {:?}\n", self.elapsed()));
        report.push_str("Checkpoints:\n");
        
        let mut prev_time = Duration::from_secs(0);
        for (i, (label, time)) in self.checkpoints.iter().enumerate() {
            let delta = if i == 0 {
                *time
            } else {
                *time - prev_time
            };
            
            report.push_str(&format!("  {}: {:?} (+{:?})\n", label, time, delta));
            prev_time = *time;
        }
        
        report
    }
}

/// Global memory tracker for monitoring memory usage
static MEMORY_PEAK: Lazy<AtomicUsize> = Lazy::new(|| AtomicUsize::new(0));

/// Memory tracking utility
pub struct MemoryTracker {
    /// System information
    system: System,
    
    /// Original memory usage at creation
    initial_usage: u64,
}

impl MemoryTracker {
    /// Create a new memory tracker
    pub fn new() -> Self {
        let mut system = System::new_all();
        system.refresh_memory();
        
        let initial_usage = system.used_memory();
        
        Self {
            system,
            initial_usage,
        }
    }
    
    /// Get the current memory usage
    pub fn current(&mut self) -> usize {
        self.system.refresh_memory();
        let current = self.system.used_memory();
        
        // Update peak if needed
        let usage = current.saturating_sub(self.initial_usage) as usize;
        let peak = MEMORY_PEAK.load(Ordering::Relaxed);
        if usage > peak {
            MEMORY_PEAK.store(usage, Ordering::Relaxed);
        }
        
        usage
    }
    
    /// Get the peak memory usage
    pub fn peak(&self) -> usize {
        MEMORY_PEAK.load(Ordering::Relaxed)
    }
    
    /// Reset the peak memory usage
    pub fn reset_peak() {
        MEMORY_PEAK.store(0, Ordering::Relaxed);
    }
}
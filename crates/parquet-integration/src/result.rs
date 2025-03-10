use std::time::{Duration, Instant};

use arrow_array::RecordBatch;

/// Statistics collected during a read operation
#[derive(Debug, Clone)]
pub struct ReadStatistics {
    /// Total number of rows read
    pub rows_read: usize,
    
    /// Total number of batches read
    pub batches_read: usize,
    
    /// Total duration of the read operation
    pub duration: Duration,
    
    /// Number of row groups in the file
    pub row_groups: usize,
    
    /// File size in bytes
    pub file_size: usize,
    
    /// Read throughput in rows per second
    pub rows_per_second: f64,
    
    /// Read throughput in bytes per second
    pub bytes_per_second: f64,
    
    /// Memory usage peak in bytes
    pub memory_peak: usize,
}

impl Default for ReadStatistics {
    fn default() -> Self {
        Self {
            rows_read: 0,
            batches_read: 0,
            duration: Duration::from_secs(0),
            row_groups: 0,
            file_size: 0,
            rows_per_second: 0.0,
            bytes_per_second: 0.0,
            memory_peak: 0,
        }
    }
}

impl ReadStatistics {
    /// Calculates performance metrics based on the given data
    pub fn calculate_metrics(&mut self) {
        if !self.duration.is_zero() {
            let duration_secs = self.duration.as_secs_f64();
            if duration_secs > 0.0 {
                self.rows_per_second = self.rows_read as f64 / duration_secs;
                self.bytes_per_second = self.file_size as f64 / duration_secs;
            }
        }
    }
    
    /// Create a new statistics object with the start time
    pub fn new() -> Self {
        Self::default()
    }
}

/// Result of a read operation
#[derive(Debug)]
pub struct ReadResult {
    /// Record batches read from the file
    pub batches: Vec<RecordBatch>,
    
    /// Statistics collected during the read operation
    pub stats: ReadStatistics,
    
    /// Start time of the read operation
    start_time: Instant,
}

impl ReadResult {
    /// Create a new read result
    pub fn new() -> Self {
        Self {
            batches: Vec::new(),
            stats: ReadStatistics::new(),
            start_time: Instant::now(),
        }
    }
    
    /// Finish the read operation and calculate statistics
    pub fn finish(&mut self) {
        self.stats.duration = self.start_time.elapsed();
        self.stats.batches_read = self.batches.len();
        self.stats.rows_read = self.batches.iter().map(|b| b.num_rows()).sum();
        self.stats.calculate_metrics();
    }
    
    /// Set file size
    pub fn with_file_size(mut self, file_size: usize) -> Self {
        self.stats.file_size = file_size;
        self
    }
    
    /// Set row groups
    pub fn with_row_groups(mut self, row_groups: usize) -> Self {
        self.stats.row_groups = row_groups;
        self
    }
    
    /// Add batches to the result
    pub fn add_batches(&mut self, batches: Vec<RecordBatch>) {
        self.batches.extend(batches);
    }
}

impl Default for ReadResult {
    fn default() -> Self {
        Self::new()
    }
}
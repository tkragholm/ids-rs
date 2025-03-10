use std::collections::HashSet;
use std::path::PathBuf;
use std::sync::Arc;

/// Column selection methods
#[derive(Debug, Clone)]
pub enum ColumnSelection {
    /// Select columns by index
    ByIndex(Vec<usize>),
    
    /// Select columns by name
    ByName(Vec<String>),
}

/// Reader modes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ReaderMode {
    /// Synchronous reading
    Sync,
    
    /// Asynchronous reading
    Async,
    
    /// Row group based reading
    RowGroup,
    
    /// Metadata focused reading
    Metadata,
    
    /// Adaptive mode that chooses the best strategy based on file characteristics
    Adaptive,
}

/// Configuration for Parquet reading operations
#[derive(Debug, Clone)]
pub struct ParquetConfig {
    /// Number of rows to read in each batch
    pub batch_size: usize,
    
    /// Reader mode
    pub mode: ReaderMode,
    
    /// Maximum number of rows to read (0 = no limit)
    pub limit: usize,
    
    /// Column selection
    pub columns: Option<ColumnSelection>,
    
    /// Optional PNR filter for data filtering
    pub pnr_filter: Option<Arc<HashSet<String>>>,
    
    /// Show progress information
    pub show_progress: bool,
    
    /// Enable parallel processing
    pub parallel: bool,
    
    /// Process directories recursively
    pub recursive: bool,
    
    /// Optional path to the parquet file or directory
    pub path: Option<PathBuf>,
    
    /// Number of CPU threads to use (0 = auto)
    pub threads: usize,
    
    /// Maximum memory usage in bytes
    pub max_memory_usage: Option<usize>,
    
    /// Use memory mapped I/O
    pub use_mmap: bool,
}

impl Default for ParquetConfig {
    fn default() -> Self {
        // Get environment variables or use defaults
        let batch_size = std::env::var("IDS_BATCH_SIZE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(131072); // 128K default batch size
            
        let threads = std::env::var("IDS_MAX_THREADS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or_else(|| num_cpus::get());
            
        Self {
            batch_size,
            mode: ReaderMode::Adaptive,
            limit: 0,
            columns: None,
            pnr_filter: None,
            show_progress: true,
            parallel: true,
            recursive: true,
            path: None,
            threads,
            max_memory_usage: None,
            use_mmap: true,
        }
    }
}

impl ParquetConfig {
    /// Create a new configuration with default values
    pub fn new() -> Self {
        Self::default()
    }
    
    /// Configure batch size
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }
    
    /// Configure reader mode
    pub fn with_mode(mut self, mode: ReaderMode) -> Self {
        self.mode = mode;
        self
    }
    
    /// Configure column selection by index
    pub fn with_column_indices(mut self, indices: Vec<usize>) -> Self {
        self.columns = Some(ColumnSelection::ByIndex(indices));
        self
    }
    
    /// Configure column selection by name
    pub fn with_column_names(mut self, names: Vec<String>) -> Self {
        self.columns = Some(ColumnSelection::ByName(names));
        self
    }
    
    /// Configure PNR filter
    pub fn with_pnr_filter(mut self, pnr_filter: HashSet<String>) -> Self {
        self.pnr_filter = Some(Arc::new(pnr_filter));
        self
    }
    
    /// Configure progress display
    pub fn with_progress(mut self, show_progress: bool) -> Self {
        self.show_progress = show_progress;
        self
    }
    
    /// Configure file path
    pub fn with_path(mut self, path: PathBuf) -> Self {
        self.path = Some(path);
        self
    }
    
    /// Configure parallel processing
    pub fn with_parallel(mut self, parallel: bool) -> Self {
        self.parallel = parallel;
        self
    }
    
    /// Configure maximum memory usage
    pub fn with_max_memory(mut self, max_memory: Option<usize>) -> Self {
        self.max_memory_usage = max_memory;
        self
    }
    
    /// Configure CPU threads
    pub fn with_threads(mut self, threads: usize) -> Self {
        self.threads = threads;
        self
    }
    
    /// Configure recursive directory scanning
    pub fn with_recursive(mut self, recursive: bool) -> Self {
        self.recursive = recursive;
        self
    }
    
    /// Configure row limit
    pub fn with_limit(mut self, limit: usize) -> Self {
        self.limit = limit;
        self
    }
    
    /// Configure memory mapping
    pub fn with_mmap(mut self, use_mmap: bool) -> Self {
        self.use_mmap = use_mmap;
        self
    }
    
    // Getters
    
    /// Get the PNR filter
    pub fn pnr_filter(&self) -> Option<&Arc<HashSet<String>>> {
        self.pnr_filter.as_ref()
    }
    
    /// Check if progress should be shown
    pub fn show_progress(&self) -> bool {
        self.show_progress
    }
    
    /// Get the file path
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }
}
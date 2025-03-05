use std::path::{Path, PathBuf};
use std::collections::HashSet;
use hashbrown::HashMap;
use crate::IdsError;

/// Configuration for customizing register file paths
#[derive(Clone, Debug)]
pub struct RegisterPathConfig {
    pub base_path: String,
    pub custom_paths: HashMap<String, String>,
}

impl RegisterPathConfig {
    /// Create a new configuration with a base path
    pub fn new(base_path: String) -> Self {
        Self {
            base_path,
            custom_paths: HashMap::new(),
        }
    }
    
    /// Add a custom path for a specific register type
    pub fn with_custom_path(mut self, register_type: &str, path: &str) -> Self {
        self.custom_paths.insert(register_type.to_string(), path.to_string());
        self
    }
    
    /// Resolve all paths, handling relative and absolute paths correctly
    pub fn resolve_paths(&self) -> Result<HashMap<String, PathBuf>, crate::IdsError> {
        let mut resolved = HashMap::new();
        let base_path_obj = Path::new(&self.base_path);
        
        if !base_path_obj.exists() {
            return Err(IdsError::invalid_operation(format!(
                "Base path does not exist: {}", self.base_path
            )));
        }
        
        // Normalize base_path for easier comparison
        let normalized_base_path = if let Ok(canonical) = base_path_obj.canonicalize() {
            canonical.to_string_lossy().to_string()
        } else {
            self.base_path.clone()
        };
        
        for (key, path) in &self.custom_paths {
            let path_obj = Path::new(path);
            
            // If path is already absolute, use it as-is
            if path_obj.is_absolute() {
                resolved.insert(key.clone(), path_obj.to_path_buf());
                continue;
            }
            
            // Check if path already includes the base_path
            if path.contains(&self.base_path) || path.contains(&normalized_base_path) {
                resolved.insert(key.clone(), path_obj.to_path_buf());
                continue;
            }
            
            // Prepend base_path for relative paths
            let full_path = base_path_obj.join(path_obj);
            resolved.insert(key.clone(), full_path);
        }
        
        Ok(resolved)
    }
    
    /// Validate that all custom paths exist
    pub fn validate(&self) -> Result<(), crate::IdsError> {
        let resolved = self.resolve_paths()?;
        let mut invalid_paths = Vec::new();
        
        for (key, path) in resolved {
            if !path.exists() {
                invalid_paths.push(format!("{} ({})", key, path.display()));
            }
        }
        
        if invalid_paths.is_empty() {
            Ok(())
        } else {
            Err(IdsError::invalid_operation(format!(
                "The following paths do not exist: {}", 
                invalid_paths.join(", ")
            )))
        }
    }
}

/// Complete loader configuration with additional options
#[derive(Clone, Debug)]
pub struct LoaderConfig {
    pub path_config: RegisterPathConfig,
    pub batch_size: usize,
    pub max_threads: usize, 
    pub filter_by_pnr: Option<HashSet<String>>,
    pub use_family_filtering: bool,
    pub use_polars: bool,
}

impl LoaderConfig {
    /// Create a new loader configuration with default settings
    pub fn new(base_path: String) -> Self {
        Self {
            path_config: RegisterPathConfig::new(base_path),
            batch_size: Self::get_batch_size(),
            max_threads: Self::get_max_threads(),
            filter_by_pnr: None,
            use_family_filtering: Self::should_use_family_filtering(),
            use_polars: Self::should_use_polars(),
        }
    }
    
    /// Set a custom path for a specific register type
    pub fn with_custom_path(mut self, register_type: &str, path: &str) -> Self {
        self.path_config = self.path_config.with_custom_path(register_type, path);
        self
    }
    
    /// Filter by a set of PNR values
    pub fn with_pnr_filter(mut self, pnr_set: HashSet<String>) -> Self {
        self.filter_by_pnr = Some(pnr_set);
        self
    }
    
    /// Filter by PNR values from a file (one PNR per line)
    pub fn with_pnr_filter_file(mut self, filter_file: &str) -> Result<Self, crate::IdsError> {
        let content = std::fs::read_to_string(filter_file)
            .map_err(|e| IdsError::invalid_operation(format!(
                "Failed to read PNR filter file: {}", e
            )))?;
            
        let pnr_set: HashSet<String> = content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
            
        self.filter_by_pnr = Some(pnr_set);
        Ok(self)
    }
    
    /// Enable or disable family-based filtering
    pub fn with_family_filtering(mut self, enabled: bool) -> Self {
        self.use_family_filtering = enabled;
        self
    }
    
    /// Enable or disable Polars backend
    pub fn with_polars(mut self, enabled: bool) -> Self {
        self.use_polars = enabled;
        self
    }
    
    /// Set batch size
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }
    
    /// Set maximum number of threads
    pub fn with_max_threads(mut self, max_threads: usize) -> Self {
        self.max_threads = max_threads;
        self
    }
    
    /// Get batch size from environment or use default
    fn get_batch_size() -> usize {
        std::env::var("IDS_BATCH_SIZE")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(65536) // 64K rows default for better performance
    }
    
    /// Get max threads from environment or use system CPU count
    fn get_max_threads() -> usize {
        std::env::var("IDS_MAX_THREADS")
            .ok()
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or_else(num_cpus::get)
            .max(2) // At least 2 threads for parallelism
    }
    
    /// Check if family filtering should be enabled
    fn should_use_family_filtering() -> bool {
        std::env::var("IDS_USE_FAMILY_FILTERING")
            .ok()
            .map(|s| s.to_lowercase() == "true" || s == "1")
            .unwrap_or(false)
    }
    
    /// Check if Polars backend should be used
    fn should_use_polars() -> bool {
        std::env::var("IDS_USE_POLARS")
            .ok()
            .map(|s| s.to_lowercase() == "true" || s == "1")
            .unwrap_or(false)
    }
}
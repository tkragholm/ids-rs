use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use types::error::IdsError;

use super::env::{get_batch_size, get_max_threads, should_use_family_filtering};

/// Configuration for customizing register file paths
#[derive(Clone, Debug)]
pub struct RegisterPathConfig {
    /// Base directory for register files
    pub base_path: String,
    
    /// Custom paths for specific register types
    pub custom_paths: HashMap<String, String>,
}

impl RegisterPathConfig {
    /// Create a new configuration with a base path
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing register files
    #[must_use]
    pub fn new(base_path: String) -> Self {
        Self {
            base_path,
            custom_paths: HashMap::new(),
        }
    }

    /// Add a custom path for a specific register type
    ///
    /// # Arguments
    /// * `register_type` - Register type (e.g., "akm", "bef")
    /// * `path` - Path to the register files
    #[must_use]
    pub fn with_custom_path(mut self, register_type: &str, path: &str) -> Self {
        self.custom_paths
            .insert(register_type.to_string(), path.to_string());
        self
    }

    /// Resolve all paths, handling relative and absolute paths correctly
    ///
    /// # Returns
    /// A HashMap of register names to their resolved paths
    ///
    /// # Errors
    /// Returns an error if the base path doesn't exist
    pub fn resolve_paths(&self) -> Result<HashMap<String, PathBuf>, IdsError> {
        let mut resolved = HashMap::new();
        let base_path_obj = Path::new(&self.base_path);

        if !base_path_obj.exists() {
            return Err(IdsError::invalid_operation(format!(
                "Base path does not exist: {}",
                self.base_path
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
    ///
    /// # Returns
    /// Ok if all paths exist, otherwise an error
    ///
    /// # Errors
    /// Returns an error if any of the paths don't exist
    pub fn validate(&self) -> Result<(), IdsError> {
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
    /// Path configuration for register files
    pub path_config: RegisterPathConfig,
    
    /// Batch size for loading data
    pub batch_size: usize,
    
    /// Maximum number of threads to use
    pub max_threads: usize,
    
    /// Optional set of PNRs to filter by
    pub filter_by_pnr: Option<HashSet<String>>,
    
    /// Whether to use family-based filtering
    pub use_family_filtering: bool,
}

impl LoaderConfig {
    /// Create a new loader configuration with default settings
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing register files
    #[must_use]
    pub fn new(base_path: String) -> Self {
        Self {
            path_config: RegisterPathConfig::new(base_path),
            batch_size: get_batch_size(),
            max_threads: get_max_threads(),
            filter_by_pnr: None,
            use_family_filtering: should_use_family_filtering(),
        }
    }

    /// Set a custom path for a specific register type
    ///
    /// # Arguments
    /// * `register_type` - Register type (e.g., "akm", "bef")
    /// * `path` - Path to the register files
    #[must_use]
    pub fn with_custom_path(mut self, register_type: &str, path: &str) -> Self {
        self.path_config = self.path_config.with_custom_path(register_type, path);
        self
    }

    /// Filter by a set of PNR values
    ///
    /// # Arguments
    /// * `pnr_set` - Set of PNRs to filter by
    #[must_use]
    pub fn with_pnr_filter(mut self, pnr_set: HashSet<String>) -> Self {
        self.filter_by_pnr = Some(pnr_set);
        self
    }

    /// Filter by PNR values from a file (one PNR per line)
    ///
    /// # Arguments
    /// * `filter_file` - Path to the file containing PNRs
    ///
    /// # Returns
    /// Updated configuration or an error
    ///
    /// # Errors
    /// Returns an error if the file cannot be read
    pub fn with_pnr_filter_file(mut self, filter_file: &str) -> Result<Self, IdsError> {
        let content = std::fs::read_to_string(filter_file).map_err(|e| {
            IdsError::invalid_operation(format!("Failed to read PNR filter file: {}", e))
        })?;

        let pnr_set: HashSet<String> = content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();

        self.filter_by_pnr = Some(pnr_set);
        Ok(self)
    }

    /// Enable or disable family-based filtering
    ///
    /// # Arguments
    /// * `enabled` - Whether to enable family-based filtering
    #[must_use]
    pub fn with_family_filtering(mut self, enabled: bool) -> Self {
        self.use_family_filtering = enabled;
        self
    }

    /// Set batch size
    ///
    /// # Arguments
    /// * `batch_size` - Batch size for loading data
    #[must_use]
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.batch_size = batch_size;
        self
    }

    /// Set maximum number of threads
    ///
    /// # Arguments
    /// * `max_threads` - Maximum number of threads to use
    #[must_use]
    pub fn with_max_threads(mut self, max_threads: usize) -> Self {
        self.max_threads = max_threads;
        self
    }
}
// Configuration module for global settings and defaults
//
// This module handles global configuration for the IDS application

/// Global environment variables used by the application
pub struct EnvironmentVariables {
    /// Maximum number of threads to use for parallel processing
    pub max_threads: usize,

    /// Batch size for data processing
    pub batch_size: usize,

    /// Enable/disable parallel BEF data loading
    pub parallel_bef: bool,

    /// Enable/disable parallel AKM data loading
    pub parallel_akm: bool,

    /// Enable/disable parallel IND data loading
    pub parallel_ind: bool,

    /// Cache type for data loading
    pub cache_type: String,
}

impl Default for EnvironmentVariables {
    fn default() -> Self {
        // Determine the number of available CPU cores
        let num_cpus = num_cpus::get();

        Self {
            max_threads: std::env::var("IDS_MAX_THREADS")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(num_cpus),

            batch_size: std::env::var("IDS_BATCH_SIZE")
                .ok()
                .and_then(|s| s.parse().ok())
                .unwrap_or(16384),

            parallel_bef: std::env::var("IDS_PARALLEL_BEF")
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(true),

            parallel_akm: std::env::var("IDS_PARALLEL_AKM")
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(true),

            parallel_ind: std::env::var("IDS_PARALLEL_IND")
                .map(|s| s.to_lowercase() == "true")
                .unwrap_or(true),

            cache_type: std::env::var("IDS_CACHE_TYPE").unwrap_or_else(|_| "lru".to_string()),
        }
    }
}

/// Load environment variables and return configured settings
#[must_use] pub fn load_environment_config() -> EnvironmentVariables {
    EnvironmentVariables::default()
}

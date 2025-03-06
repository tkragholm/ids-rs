use types::error::IdsError;
use types::storage::ArrowBackend as ArrowStore;

use crate::config::RegisterPathConfig;

/// Core trait for store loader implementations
///
/// This trait defines the required methods for loading data into an `ArrowStore`.
pub trait StoreLoader {
    /// Load data from a specified base path
    ///
    /// # Arguments
    /// * `base_path` - Path to the directory containing register data
    ///
    /// # Returns
    /// A populated `ArrowStore` or an error
    ///
    /// # Errors
    /// Returns an error if loading fails
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError>;

    /// Load data with custom paths for different register types
    ///
    /// # Arguments
    /// * `config` - Configuration specifying paths for different register types
    ///
    /// # Returns
    /// A populated `ArrowStore` or an error
    ///
    /// # Errors
    /// Returns an error if loading fails
    fn load_with_custom_paths(config: RegisterPathConfig) -> Result<ArrowStore, IdsError>;
}
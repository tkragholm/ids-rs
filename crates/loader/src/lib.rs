// Organize modules to improve codebase structure
mod config;
mod loaders;
mod progress;
mod readers;
mod schema;
mod utils;

// Re-export core types
pub use config::{LoaderConfig, RegisterPathConfig};
pub use loaders::{ParallelLoader, ParquetLoader, PolarsLoader};
pub use progress::LoaderProgress;
pub use readers::{DataReader, FileReader, CustomPathReader};
pub use types::{
    error::IdsError,
    family::FamilyRelations,
    models::*,
    storage::{ArrowBackend as ArrowStore, DataStore as UnifiedStore, Storage as Store},
};

// Public trait for loader implementations
pub trait StoreLoader {
    /// Load data from a specified base path
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError>;
    
    /// Load data with custom paths for different register types
    fn load_with_custom_paths(config: RegisterPathConfig) -> Result<ArrowStore, IdsError>;
}

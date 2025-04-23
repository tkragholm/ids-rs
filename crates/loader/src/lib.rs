// Root modules with public re-exports
mod config;
// All modules - make formats and registry public to support testing
pub mod formats;
mod loaders;
mod readers;
pub mod registry;
mod schema;
mod ui;

// Re-export configuration related types
pub use config::{
    env::{get_batch_size, get_max_threads, should_use_family_filtering, use_parallel_loading},
    LoaderConfig, RegisterPathConfig,
};

// Re-export core loader implementations
pub use loaders::{ParallelLoader, SequentialLoader, StoreLoader};

// Re-export reader related types
pub use readers::{CustomPathReader, DataReader, FileReader};

// Re-export UI related types
pub use ui::{
    console::{print_section, print_success, print_warning},
    LoaderProgress,
};

// Re-export format utilities
pub use formats::{load_parquet_files_parallel, read_parquet, read_parquet_with_filter};

// Re-export registry loading functions
pub use registry::{load_akm, load_bef, load_family, load_ind, load_uddf};

// Re-export schema functions
pub use schema::{akm_schema, bef_schema, family_schema, ind_schema, uddf_schema};

// Re-export core types from types crate
pub use types::{
    error::IdsError, family::FamilyRelations, models::*,
    storage::arrow::backend::ArrowBackend as ArrowStore, store::DataStore as UnifiedStore,
    traits::Store,
};

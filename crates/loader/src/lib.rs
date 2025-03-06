// Root modules with public re-exports
mod config;
mod formats;
mod loaders;
mod readers;
mod registry;
mod schema;
mod ui;

// Re-export configuration related types
pub use config::{
    LoaderConfig, 
    RegisterPathConfig,
    env::{get_batch_size, get_max_threads, should_use_family_filtering, use_parallel_loading}
};

// Re-export core loader implementations
pub use loaders::{
    StoreLoader,
    ParallelLoader,
    SequentialLoader,
};

// Re-export reader related types
pub use readers::{
    DataReader,
    FileReader,
    CustomPathReader,
};

// Re-export UI related types
pub use ui::{
    LoaderProgress,
    console::{print_section, print_success, print_warning},
};

// Re-export format utilities
pub use formats::{
    read_parquet,
    read_parquet_with_filter,
    load_parquet_files_parallel,
};

// Re-export registry loading functions
pub use registry::{
    load_akm,
    load_bef,
    load_family,
    load_ind,
    load_uddf,
};

// Re-export schema functions
pub use schema::{
    akm_schema,
    bef_schema,
    ind_schema,
    uddf_schema,
    family_schema,
};

// Re-export core types from types crate
pub use types::{
    error::IdsError,
    family::FamilyRelations,
    models::*,
    storage::{ArrowBackend as ArrowStore, DataStore as UnifiedStore},
    traits::Store,
};

mod data_store;
pub use data_store::{DataStore, CacheKey};

// We're consolidating on the storage::arrow::backend implementation
pub mod time_varying_backend;

pub use crate::storage::arrow::backend::ArrowBackend;
pub use time_varying_backend::TimeVaryingBackend;

/// Backend trait marker for storage implementations
pub trait Backend: crate::traits::Store {
    // All methods are already in Store trait
}
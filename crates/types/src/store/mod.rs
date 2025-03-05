mod data_store;
pub use data_store::{DataStore, CacheKey};

pub mod arrow_backend;
pub mod time_varying_backend;

pub use arrow_backend::ArrowBackend;
pub use time_varying_backend::TimeVaryingBackend;

/// Backend trait marker for storage implementations
pub trait Backend: crate::traits::Store {
    // All methods are already in Store trait
}
// This file exists for backwards compatibility only.
// Use the store module instead.

pub use crate::store::{
    ArrowBackend,
    TimeVaryingBackend,
    DataStore, CacheKey,
};

// Re-export the Storage trait for backwards compatibility
pub use crate::traits::Store as Storage;
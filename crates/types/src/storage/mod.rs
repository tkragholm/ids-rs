//! Storage abstractions for different data backends.
//!
//! This module provides storage abstractions for working with different data backends,
//! including Arrow, in-memory storage, and time-varying data.
//!
//! The main types in this module are:
//!
//! - `DataStore`: The central data store that manages multiple backends
//! - `Backend`: Trait for implementing different storage backends
//! - `ArrowBackend`: Arrow-based storage backend implementation
//! - `MemoryBackend`: Simple in-memory storage backend
//! - `TimeVaryingBackend`: Backend for time-varying data
//!
//! Additionally, this module provides concurrency utilities for thread-safe
//! access to storage backends:
//!
//! - `ThreadSafeStore`: Thread-safe wrapper for any Store implementation
//! - `ShardedCache`: High-performance sharded cache for concurrent access
//! - `CovariateCache`: Optimized cache for covariates with low contention

use chrono::NaiveDate;
use crate::models::CovariateType;

/// Common cache key for covariate lookups
/// Used across various caching implementations
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct CacheKey {
    pub pnr: String,
    pub covariate_type: CovariateType,
    pub date: NaiveDate,
}

impl CacheKey {
    /// Create a new cache key
    #[must_use]
    pub fn new(pnr: &str, covariate_type: CovariateType, date: NaiveDate) -> Self {
        Self {
            pnr: pnr.to_string(),
            covariate_type,
            date,
        }
    }
}

pub mod arrow;
pub mod backends;
pub mod concurrency;

// Re-export public types
pub use crate::store::DataStore;
pub use crate::traits::Store as Backend;

// Export the consolidated backends
pub use arrow::backend::ArrowBackend;
pub use crate::store::time_varying_backend::TimeVaryingBackend;

// Export concurrency utilities
pub use concurrency::{ThreadSafeStore, ShardedCache, CovariateCache};
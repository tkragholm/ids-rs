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
///
/// Optimized implementation that uses a string interner pool to minimize
/// memory usage when many PNRs are used in cache keys.
#[derive(Debug, Hash, Eq, PartialEq, Clone)]
pub struct CacheKey {
    /// PNR identifier string, using an Arc-string for memory-efficient cloning
    pub pnr: std::sync::Arc<str>,
    /// Type of covariate for this cache entry
    pub covariate_type: CovariateType,
    /// Reference date for this covariate
    pub date: NaiveDate,
}

impl CacheKey {
    /// Create a new cache key with efficient memory usage
    ///
    /// Uses a thread-local cache of PNR strings to minimize memory allocations
    /// when creating many cache keys with the same PNRs.
    ///
    /// # Arguments
    /// * `pnr` - The PNR identifier
    /// * `covariate_type` - The type of covariate
    /// * `date` - The reference date
    ///
    /// # Returns
    /// A new cache key with optimized memory usage
    #[must_use]
    pub fn new(pnr: &str, covariate_type: CovariateType, date: NaiveDate) -> Self {
        use std::sync::Arc;
        
        // Use a thread-local cache for PNRs to avoid duplicate allocations
        thread_local! {
            static PNR_CACHE: std::cell::RefCell<dashmap::DashMap<String, Arc<str>>> = 
                std::cell::RefCell::new(dashmap::DashMap::with_capacity(1000));
        }
        
        // Try to get the PNR from the cache first
        let pnr_arc = PNR_CACHE.with(|cache| {
            let cache = cache.borrow();
            if let Some(cached) = cache.get(pnr) {
                cached.clone()
            } else {
                // Not in cache, create new Arc and add to cache
                let pnr_arc = Arc::from(pnr);
                cache.insert(pnr.to_string(), pnr_arc.clone());
                pnr_arc
            }
        });
        
        Self {
            pnr: pnr_arc,
            covariate_type,
            date,
        }
    }
    
    /// Create a new cache key with a pre-allocated Arc<str>
    ///
    /// This is useful when you already have an Arc<str> from another source,
    /// avoiding the need to go through the cache lookup.
    ///
    /// # Arguments
    /// * `pnr` - The PNR identifier as an Arc<str>
    /// * `covariate_type` - The type of covariate
    /// * `date` - The reference date
    ///
    /// # Returns
    /// A new cache key using the provided Arc<str>
    #[must_use]
    pub fn from_arc(pnr: std::sync::Arc<str>, covariate_type: CovariateType, date: NaiveDate) -> Self {
        Self {
            pnr,
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
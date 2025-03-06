//! Cacheable trait for objects that support caching operations
//!
//! This trait provides a standardized interface for caching operations,
//! supporting both in-memory caching and potentially other storage backends.

use crate::error::IdsError;

/// Trait for cacheable operations
///
/// This trait defines a standard interface for objects that perform
/// cacheable operations, with methods for retrieving, computing, and
/// invalidating cached values.
///
/// # Type Parameters
///
/// * `K` - The key type for cache lookups
/// * `V` - The value type stored in the cache
pub trait Cacheable<K, V> {
    /// Get a value from the cache or compute it if not present
    ///
    /// This method tries to retrieve a value from the cache using the provided key.
    /// If the value is not found, it calls the compute function to generate the value,
    /// stores it in the cache, and returns it.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up in the cache
    /// * `compute_fn` - Function to generate the value if not found in cache
    ///
    /// # Returns
    ///
    /// * `Result<V, IdsError>` - The cached or computed value, or an error
    fn get_or_compute<F>(&self, key: K, compute_fn: F) -> Result<V, IdsError>
    where
        F: FnOnce() -> Result<V, IdsError>;
        
    /// Prefetch multiple values into the cache
    ///
    /// This method can be used to load multiple values into the cache
    /// in a single operation, which may be more efficient than individual lookups.
    ///
    /// # Arguments
    ///
    /// * `keys` - Slice of keys to prefetch
    ///
    /// # Returns
    ///
    /// * `Result<usize, IdsError>` - Number of items successfully prefetched, or an error
    fn prefetch(&self, keys: &[K]) -> Result<usize, IdsError>;
    
    /// Invalidate a cached value
    ///
    /// This method removes a value from the cache, forcing it to be
    /// recomputed on the next access.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to invalidate
    ///
    /// # Returns
    ///
    /// * `bool` - True if the key was found and removed, false if not found
    fn invalidate(&self, key: &K) -> bool;
    
    /// Clear all cached values
    ///
    /// This method removes all values from the cache.
    fn clear(&self);
}
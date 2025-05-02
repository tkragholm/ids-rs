//! Concurrency utilities for thread-safe data access.
//!
//! This module provides standardized concurrency primitives and patterns
//! for ensuring thread-safe access to data stores and caches. It consolidates
//! the various concurrency approaches used throughout the codebase into a
//! consistent, optimized set of utilities.

use crate::error::{IdsError, Result};
use crate::models::{Covariate, CovariateType, TimeVaryingValue};
use crate::traits::Store;
use chrono::NaiveDate;
use dashmap::DashMap;
use parking_lot::{Mutex, RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::hash::{BuildHasher, Hash, RandomState};
use std::sync::Arc;

/// Standard thread-safe wrapper for store implementations.
///
/// This wrapper provides consistent thread-safe access to any store implementation
/// using high-performance `RwLock` for concurrent read access and exclusive write access.
#[derive(Debug, Clone)]
pub struct ThreadSafeStore<S: Store + 'static> {
    inner: Arc<RwLock<S>>,
}

impl<S: Store + 'static> ThreadSafeStore<S> {
    /// Creates a new thread-safe store wrapper around the provided store.
    ///
    /// # Arguments
    ///
    /// * `store` - The store to wrap
    ///
    /// # Returns
    ///
    /// A new thread-safe store wrapper
    #[must_use]
    pub fn new(store: S) -> Self {
        Self {
            inner: Arc::new(RwLock::new(store)),
        }
    }

    /// Access the inner store with shared read access.
    ///
    /// This method acquires a read lock on the store, enabling multiple concurrent
    /// readers but blocking writes until all read locks are released.
    ///
    /// # Returns
    ///
    /// A guard that provides shared access to the store
    #[must_use]
    pub fn read(&self) -> RwLockReadGuard<'_, S> {
        self.inner.read()
    }

    /// Access the inner store with exclusive write access.
    ///
    /// This method acquires a write lock on the store, blocking all other access
    /// until the write lock is released.
    ///
    /// # Returns
    ///
    /// A guard that provides exclusive access to the store
    #[must_use]
    pub fn write(&self) -> RwLockWriteGuard<'_, S> {
        self.inner.write()
    }

    /// Get a covariate with minimal locking time.
    ///
    /// This method acquires the lock only for the duration of the covariate retrieval.
    ///
    /// # Arguments
    ///
    /// * `pnr` - The person identification number
    /// * `covariate_type` - The type of covariate to retrieve
    /// * `date` - The date for which to retrieve the covariate
    ///
    /// # Returns
    ///
    /// A Result containing the covariate (if found) or an error
    pub fn covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>> {
        let mut store = self.inner.write();
        store.covariate(pnr, covariate_type, date)
    }

    /// Gets the inner store implementation directly.
    ///
    /// # Returns
    ///
    /// A reference to the Arc-wrapped `RwLock` containing the store
    #[must_use]
    pub fn inner(&self) -> &Arc<RwLock<S>> {
        &self.inner
    }
}

/// Implementation of the Store trait for `ThreadSafeStore`.
///
/// This allows `ThreadSafeStore` to be used anywhere a Store is expected.
impl<S: Store + 'static> Store for ThreadSafeStore<S> {
    fn covariate(
        &mut self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>> {
        let mut store = self.inner.write();
        store.covariate(pnr, covariate_type, date)
    }

    fn family_relations(&self, _pnr: &str) -> Option<&crate::family::FamilyRelations> {
        // This operation requires holding the read lock for the entire method call
        // which means we can't return a reference to something inside the lock.
        // Instead, we need to clone the data or restructure the API.
        // For now, this returns None to avoid deadlocks, but the API needs to be changed.
        None
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<()> {
        let mut store = self.inner.write();
        store.load_data(data)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// High-performance sharded cache for improved concurrency.
///
/// This optimized cache implementation uses efficient data sharding based on key hashing
/// to minimize contention and maximize throughput in highly concurrent scenarios.
/// It eliminates redundant locks by leveraging `DashMap`'s built-in concurrency.
pub struct ShardedCache<K, V> {
    /// Array of `DashMap` instances, each responsible for a shard of the keyspace
    shards: Vec<dashmap::DashMap<K, V>>,
    /// Number of shards for distributing keys
    num_shards: usize,
    /// Hash function state for key distribution
    hasher: RandomState,
}

impl<K, V> ShardedCache<K, V>
where
    K: Hash + Eq + Clone,
    V: Clone,
{
    /// Create a new sharded cache with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The total approximate capacity of the cache
    /// * `num_shards` - The number of shards to use (defaults to the number of logical CPU cores)
    ///
    /// # Returns
    ///
    /// A new sharded cache instance
    #[must_use]
    pub fn new(capacity: usize, num_shards: Option<usize>) -> Self {
        // Determine optimal shard count based on CPU cores or provided value
        let num_shards = num_shards.unwrap_or_else(|| {
            std::thread::available_parallelism()
                .map(std::num::NonZero::get)
                .unwrap_or(16)
                .max(4)
        });

        // Calculate per-shard capacity, ensuring even distribution
        let per_shard_capacity = (capacity / num_shards) + 1;

        // Create shards with pre-allocated capacity
        let shards: Vec<DashMap<K, V>> = (0..num_shards)
            .map(|_| DashMap::with_capacity_and_hasher(per_shard_capacity, RandomState::new()))
            .collect();

        Self {
            shards,
            num_shards,
            hasher: RandomState::new(),
        }
    }

    /// Get the shard index for a key using consistent hashing.
    ///
    /// Uses a high-quality hash function to distribute keys evenly across shards.
    #[inline]
    fn shard_idx<Q: Hash>(&self, key: &Q) -> usize {
        (self.hasher.hash_one(key) % self.num_shards as u64) as usize
    }

    /// Get a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// The value if present, otherwise None
    pub fn get(&self, key: &K) -> Option<V> {
        let shard_idx = self.shard_idx(key);
        self.shards[shard_idx].get(key).map(|v| v.clone())
    }

    /// Insert a value into the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to insert
    /// * `value` - The value to insert
    pub fn insert(&self, key: K, value: V) {
        let shard_idx = self.shard_idx(&key);
        self.shards[shard_idx].insert(key, value);
    }

    /// Check if the cache contains a key.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to check
    ///
    /// # Returns
    ///
    /// True if the key is present, otherwise false
    pub fn contains_key(&self, key: &K) -> bool {
        let shard_idx = self.shard_idx(key);
        self.shards[shard_idx].contains_key(key)
    }

    /// Clear all entries from the cache.
    pub fn clear(&self) {
        self.shards.iter().for_each(dashmap::DashMap::clear);
    }

    /// Get the approximate number of entries in the cache.
    ///
    /// # Returns
    ///
    /// The approximate number of entries
    #[must_use] pub fn len(&self) -> usize {
        self.shards.iter().map(dashmap::DashMap::len).sum()
    }

    /// Check if the cache is empty.
    ///
    /// # Returns
    ///
    /// True if the cache is empty, otherwise false
    #[must_use] pub fn is_empty(&self) -> bool {
        self.shards.iter().all(dashmap::DashMap::is_empty)
    }

    /// Perform a bulk insertion with minimal contention.
    ///
    /// This method pre-sorts entries by shard to minimize cross-shard operations and
    /// maximize insertion throughput in concurrent scenarios.
    ///
    /// # Arguments
    ///
    /// * `entries` - The entries to insert
    pub fn insert_batch(&self, entries: Vec<(K, V)>) {
        // Group entries by shard for efficient insertion
        let mut sharded_entries: Vec<Vec<(K, V)>> = vec![Vec::new(); self.num_shards];

        // Distribute entries to their target shards
        for (key, value) in entries {
            let idx = self.shard_idx(&key);
            sharded_entries[idx].push((key, value));
        }

        // Process each shard in parallel using rayon if available
        #[cfg(feature = "parallel")]
        {
            use rayon::prelude::*;
            sharded_entries
                .into_par_iter()
                .enumerate()
                .for_each(|(idx, entries)| {
                    if !entries.is_empty() {
                        let shard = &self.shards[idx];
                        entries.into_iter().for_each(|(key, value)| {
                            shard.insert(key, value);
                        });
                    }
                });
        }

        // Sequential fallback when parallel feature is not enabled
        #[cfg(not(feature = "parallel"))]
        {
            for (idx, entries) in sharded_entries.into_iter().enumerate() {
                if !entries.is_empty() {
                    let shard = &self.shards[idx];
                    for (key, value) in entries {
                        shard.insert(key, value);
                    }
                }
            }
        }
    }

    /// Remove an entry from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to remove
    ///
    /// # Returns
    ///
    /// The removed value if it was present
    pub fn remove(&self, key: &K) -> Option<V> {
        let shard_idx = self.shard_idx(key);
        self.shards[shard_idx].remove(key).map(|(_, v)| v)
    }

    /// Get or compute a value in the cache.
    ///
    /// If the key exists in the cache, returns the existing value.
    /// Otherwise, computes a new value using the provided function and inserts it.
    ///
    /// # Arguments
    ///
    /// * `key` - The key to look up
    /// * `f` - Function to compute a new value if key is not present
    ///
    /// # Returns
    ///
    /// The existing or newly computed value
    pub fn get_or_insert_with<F>(&self, key: K, f: F) -> V
    where
        F: FnOnce() -> V,
    {
        let shard_idx = self.shard_idx(&key);
        self.shards[shard_idx].entry(key).or_insert_with(f).clone()
    }
}

/// A high-performance cache for covariates.
///
/// This cache implementation provides low-contention access to covariate data
/// using a sharded approach with fine-grained locking.
pub struct CovariateCache {
    cache: ShardedCache<crate::storage::CacheKey, Option<Covariate>>,
    bulk_lock: Mutex<()>,
}

impl CovariateCache {
    /// Create a new covariate cache with the specified capacity.
    ///
    /// # Arguments
    ///
    /// * `capacity` - The approximate capacity of the cache
    ///
    /// # Returns
    ///
    /// A new `CovariateCache` instance
    #[must_use]
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: ShardedCache::<crate::storage::CacheKey, Option<Covariate>>::new(capacity, None),
            bulk_lock: Mutex::new(()),
        }
    }

    /// Get a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to look up
    ///
    /// # Returns
    ///
    /// The value if present, otherwise None
    pub fn get(&self, key: &crate::storage::CacheKey) -> Option<Option<Covariate>> {
        self.cache.get(key)
    }

    /// Insert a value into the cache.
    ///
    /// # Arguments
    ///
    /// * `key` - The cache key to insert
    /// * `value` - The value to insert
    pub fn insert(&self, key: crate::storage::CacheKey, value: Option<Covariate>) {
        self.cache.insert(key, value);
    }

    /// Clear the cache.
    pub fn clear(&self) {
        let _guard = self.bulk_lock.lock();
        self.cache.clear();
    }

    /// Get the number of entries in the cache.
    ///
    /// # Returns
    ///
    /// The number of entries
    pub fn len(&self) -> usize {
        self.cache.len()
    }

    /// Check if the cache is empty.
    ///
    /// # Returns
    ///
    /// True if the cache is empty, otherwise false
    pub fn is_empty(&self) -> bool {
        self.cache.len() == 0
    }

    /// Get or load a value from the cache.
    ///
    /// # Arguments
    ///
    /// * `store` - The store to load from if the value is not in the cache
    /// * `key` - The key to look up
    ///
    /// # Returns
    ///
    /// A Result containing the value or an error
    pub fn get_or_load(
        &self,
        store: &mut impl Store,
        key: crate::storage::CacheKey,
    ) -> Result<Option<Covariate>> {
        // First check if the value is in the cache
        if let Some(value) = self.cache.get(&key) {
            return Ok(value);
        }

        // Not in cache, load from store
        let pnr = &key.pnr;
        let cov_type = key.covariate_type;
        let date = key.date;

        let value = store.covariate(pnr, cov_type, date)?;

        // Cache the result
        self.cache.insert(key, value.clone());

        Ok(value)
    }

    /// Bulk load values into the cache.
    ///
    /// # Arguments
    ///
    /// * `store` - The store to load from
    /// * `pnrs` - The PNRs to load for
    /// * `covariate_types` - The covariate types to load
    /// * `dates` - The dates to load for
    ///
    /// # Returns
    ///
    /// A Result containing the number of entries loaded or an error
    pub fn bulk_load(
        &self,
        store: &mut impl Store,
        pnrs: &[String],
        covariate_types: &[CovariateType],
        dates: &[NaiveDate],
    ) -> Result<usize> {
        // For large bulk operations, acquire the bulk lock
        let _bulk_guard = self.bulk_lock.lock();

        // Create all cache keys
        let total_keys = pnrs.len() * covariate_types.len() * dates.len();
        let mut keys = Vec::with_capacity(total_keys);

        for pnr in pnrs {
            for &cov_type in covariate_types {
                for &date in dates {
                    let key = crate::storage::CacheKey::new(pnr, cov_type, date);
                    if !self.cache.contains_key(&key) {
                        keys.push((key, pnr.clone(), cov_type, date));
                    }
                }
            }
        }

        // Load missing values
        let mut loaded_entries = Vec::with_capacity(keys.len());

        for (key, pnr, cov_type, date) in keys {
            match store.covariate(&pnr, cov_type, date) {
                Ok(value) => {
                    loaded_entries.push((key, value.clone()));
                }
                Err(e) => {
                    return Err(IdsError::invalid_operation(format!(
                        "Failed to load covariate: {e}"
                    )));
                }
            }
        }

        // Bulk insert into cache
        let entry_count = loaded_entries.len();
        if !loaded_entries.is_empty() {
            self.cache.insert_batch(loaded_entries);
        }

        Ok(entry_count)
    }
}

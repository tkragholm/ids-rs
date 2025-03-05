use chrono::NaiveDate;
use dashmap::DashMap;
use parking_lot::{Mutex, RwLock};
use rayon::prelude::*;
use hashbrown::HashMap;
use std::hash::Hash;
use std::sync::Arc;
use types::models::{Covariate, CovariateType};
use types::traits::Store;
use types::IdsError;

#[derive(Hash, Eq, PartialEq, Clone)]
pub struct CacheKey {
    pub pnr: String,
    pub covariate_type: CovariateType,
    pub date: NaiveDate,
}

impl CacheKey {
    pub fn new(pnr: &str, covariate_type: CovariateType, date: NaiveDate) -> Self {
        // Use smaller string allocation strategy - most PNRs are Danish CPR numbers of consistent length
        let pnr = pnr.to_string();
        
        Self {
            pnr,
            covariate_type,
            date,
        }
    }

    // Fast equality check for common case of matching PNRs
    #[inline]
    #[allow(dead_code)]
    pub fn matches(&self, pnr: &str, covariate_type: CovariateType, date: NaiveDate) -> bool {
        self.pnr == pnr && self.covariate_type == covariate_type && self.date == date
    }
}

/// Number of shards to use for the cache to reduce contention
const NUM_SHARDS: usize = 32;

/// Sharded cache implementation for better performance under concurrent access
struct ShardedCache<K, V> {
    shards: Vec<RwLock<HashMap<K, V>>>,
    hasher: ahash::RandomState,
}

impl<K, V> ShardedCache<K, V> 
where 
    K: Hash + Eq + Clone,
    V: Clone,
{
    fn new(capacity: usize) -> Self {
        let per_shard_capacity = (capacity / NUM_SHARDS) + 1;
        let mut shards = Vec::with_capacity(NUM_SHARDS);
        
        for _ in 0..NUM_SHARDS {
            shards.push(RwLock::new(HashMap::with_capacity(per_shard_capacity)));
        }
        
        Self {
            shards,
            hasher: ahash::RandomState::new(),
        }
    }
    
    /// Get the shard index for a key
    #[inline]
    fn shard_idx<Q: Hash>(&self, key: &Q) -> usize {
        (self.hasher.hash_one(key) as usize) % NUM_SHARDS
    }
    
    /// Get a reference to the shard for a key
    #[inline]
    fn shard<Q: Hash>(&self, key: &Q) -> &RwLock<HashMap<K, V>> {
        &self.shards[self.shard_idx(key)]
    }
    
    /// Get a value from the cache
    fn get(&self, key: &K) -> Option<V> {
        let shard = self.shard(key);
        let cache = shard.read();
        
        cache.get(key).cloned()
    }
    
    /// Insert a value into the cache
    fn insert(&self, key: K, value: V) {
        let shard = self.shard(&key);
        let mut cache = shard.write();
        cache.insert(key, value);
    }
    
    /// Insert multiple values with a single lock acquisition per shard
    fn insert_batch(&self, entries: Vec<(K, V)>) {
        // Group entries by shard to minimize lock acquisitions
        let mut sharded_entries: Vec<Vec<(K, V)>> = vec![Vec::new(); NUM_SHARDS];
        
        for (key, value) in entries {
            let idx = self.shard_idx(&key);
            sharded_entries[idx].push((key, value));
        }
        
        // Insert entries into each shard with a single lock acquisition per shard
        for (idx, entries) in sharded_entries.into_iter().enumerate() {
            if !entries.is_empty() {
                let mut cache = self.shards[idx].write();
                for (key, value) in entries {
                    cache.insert(key, value);
                }
            }
        }
    }
    
    /// Check if the cache contains a key
    fn contains_key(&self, key: &K) -> bool {
        let shard = self.shard(key);
        let cache = shard.read();
        
        cache.contains_key(key)
    }
    
    /// Clear all entries from the cache
    fn clear(&self) {
        for shard in &self.shards {
            let mut cache = shard.write();
            cache.clear();
        }
    }
    
    /// Get the total number of entries in the cache
    fn len(&self) -> usize {
        self.shards.iter()
            .map(|shard| shard.read().len())
            .sum()
    }
}

pub struct CovariateCache {
    // Primary cache: sharded RwLock for better concurrent read/write performance
    primary_cache: Arc<ShardedCache<CacheKey, Option<Covariate>>>,
    // Secondary cache: DashMap for operations that need lock-free concurrent access
    secondary_cache: DashMap<CacheKey, Option<Covariate>>,
    // Bulk operation lock to coordinate large operations
    bulk_lock: Mutex<()>,
}

impl CovariateCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            primary_cache: Arc::new(ShardedCache::new(capacity)),
            secondary_cache: DashMap::with_capacity(capacity / 4), // Smaller secondary cache
            bulk_lock: Mutex::new(()),
        }
    }

    pub fn get(&self, key: &CacheKey) -> Option<Option<Covariate>> {
        // First check the primary sharded cache (faster for reads)
        if let Some(value) = self.primary_cache.get(key) {
            return Some(value);
        }
        
        // Fall back to DashMap if not in primary cache
        self.secondary_cache.get(key).map(|v| v.clone())
    }

    pub fn insert(&self, key: CacheKey, value: Option<Covariate>) {
        // Insert into primary cache first
        self.primary_cache.insert(key.clone(), value.clone());
        
        // Also update secondary cache for consistency
        self.secondary_cache.insert(key, value);
    }

    pub fn clear(&self) {
        // Acquire bulk lock to prevent concurrent operations during clear
        let _guard = self.bulk_lock.lock();
        
        // Clear both caches
        self.primary_cache.clear();
        self.secondary_cache.clear();
    }

    pub fn len(&self) -> usize {
        // We primarily track size through the primary cache
        self.primary_cache.len()
    }
    
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn get_or_load(
        &self,
        store: &impl Store,
        key: CacheKey,
    ) -> Result<Option<Covariate>, IdsError> {
        // First check the primary cache (sharded for better read performance)
        if let Some(value) = self.primary_cache.get(&key) {
            return Ok(value);
        }
        
        // Then check the secondary cache
        if let Some(entry) = self.secondary_cache.get(&key) {
            return Ok(entry.clone());
        }
        
        // Not in cache, need to load it
        let pnr = &key.pnr;
        let cov_type = key.covariate_type;
        let date = key.date;
        
        // Load from store
        match store.get_covariate(pnr, cov_type, date) {
            Ok(value) => {
                // Cache the result in both caches
                self.insert(key, value.clone());
                Ok(value)
            }
            Err(err) => {
                // Don't cache errors - let caller decide what to do
                Err(err)
            }
        }
    }
    
    /// Bulk loading method to pre-populate cache for better performance
    /// This implementation uses sharding to minimize lock contention
    pub fn bulk_load(
        &self,
        store: &impl Store,
        pnrs: &[String], 
        covariate_types: &[CovariateType],
        dates: &[NaiveDate],
    ) -> Result<usize, IdsError> {
        // For large bulk operations, we use a lock to prevent concurrent bulk loads
        // which could cause duplicate work and contention
        let _bulk_guard = self.bulk_lock.lock();
        
        // Create all cache keys first to reduce allocations
        let total_keys = pnrs.len() * covariate_types.len() * dates.len();
        let mut keys = Vec::with_capacity(total_keys);
        
        for pnr in pnrs {
            for &cov_type in covariate_types {
                for &date in dates {
                    keys.push((
                        CacheKey::new(pnr, cov_type, date),
                        pnr.clone(),
                        cov_type,
                        date
                    ));
                }
            }
        }
        
        // Filter out keys that are already in the cache
        // First check primary cache
        let mut missing_keys: Vec<_> = keys.into_iter()
            .filter(|(key, _, _, _)| !self.primary_cache.contains_key(key))
            .collect();
            
        // Then filter by secondary cache
        missing_keys.retain(|(key, _, _, _)| !self.secondary_cache.contains_key(key));
        
        // Process the missing keys in parallel chunks
        const CHUNK_SIZE: usize = 1000;
        
        // Use rayon for parallel processing of chunks
        let loaded_entries: Vec<(CacheKey, Option<Covariate>)> = missing_keys
            .par_chunks(CHUNK_SIZE)
            .flat_map(|chunk| {
                let mut batch_results = Vec::with_capacity(chunk.len());
                
                for (key, pnr, cov_type, date) in chunk {
                    if let Ok(value) = store.get_covariate(pnr, *cov_type, *date) {
                        batch_results.push((key.clone(), value.clone()));
                    }
                }
                
                batch_results
            })
            .collect();
        
        let entry_count = loaded_entries.len();
        
        // Bulk insert into the primary cache with minimal lock contention
        if !loaded_entries.is_empty() {
            // Group by shard and insert with minimal lock acquisitions
            self.primary_cache.insert_batch(loaded_entries.clone());
            
            // Also update the secondary cache
            for (key, value) in &loaded_entries {
                self.secondary_cache.insert(key.clone(), value.clone());
            }
        }
        
        Ok(entry_count)
    }
    
    /// Prefetch all covariates for a set of subjects on specific dates
    /// Optimized for parallel fetching with minimal lock contention
    pub fn prefetch_for_subjects(
        &self,
        store: &impl Store,
        pnrs: &[String],
        covariate_types: &[CovariateType],
        dates: &[NaiveDate],
    ) -> Result<usize, IdsError> {
        self.bulk_load(store, pnrs, covariate_types, dates)
    }
}
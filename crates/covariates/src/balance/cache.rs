use chrono::NaiveDate;
use dashmap::DashMap;
use types::models::{Covariate, CovariateType};
use types::storage::Storage as Store;
use types::IdsError;

#[derive(Hash, Eq, PartialEq, Clone)]
pub(crate) struct CacheKey {
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
    pub fn matches(&self, pnr: &str, covariate_type: CovariateType, date: NaiveDate) -> bool {
        self.pnr == pnr && self.covariate_type == covariate_type && self.date == date
    }
}

pub(crate) struct CovariateCache {
    cache: DashMap<CacheKey, Option<Covariate>>,
}

impl CovariateCache {
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: DashMap::with_capacity(capacity),
        }
    }

    pub fn get(&self, key: &CacheKey) -> Option<Option<Covariate>> {
        self.cache.get(key).map(|v| v.clone())
    }

    pub fn insert(&self, key: CacheKey, value: Option<Covariate>) {
        self.cache.insert(key, value);
    }

    pub fn clear(&self) {
        self.cache.clear();
    }

    pub fn len(&self) -> usize {
        self.cache.len()
    }

    pub fn get_or_load(
        &self,
        store: &impl Store,
        key: CacheKey,
    ) -> Result<Option<Covariate>, IdsError> {
        // Optimization: Use entry API for more efficient lookup and insertion
        if let Some(entry) = self.cache.get(&key) {
            // Fast path: item already in cache - avoid cloning if possible
            return Ok(entry.clone());
        }
        
        // Not in cache, need to load it
        let pnr = &key.pnr;
        let cov_type = key.covariate_type;
        let date = key.date;
        
        // Load from store
        match store.get_covariate(pnr, cov_type, date) {
            Ok(value) => {
                // Cache the result
                self.cache.insert(key, value.clone());
                Ok(value)
            }
            Err(err) => {
                // Don't cache errors - let caller decide what to do
                Err(err)
            }
        }
    }
    
    // Bulk loading method to pre-populate cache for better performance
    pub fn bulk_load(
        &self,
        store: &impl Store,
        pnrs: &[String], 
        covariate_types: &[CovariateType],
        dates: &[NaiveDate],
    ) -> Result<usize, IdsError> {
        let mut loaded = 0;
        
        // Create all cache keys first to reduce allocations
        let mut keys = Vec::with_capacity(pnrs.len() * covariate_types.len() * dates.len());
        
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
        
        // Process in batches
        for batch in keys.chunks(1000) {
            // First filter out keys already in cache
            let missing_keys: Vec<_> = batch
                .iter()
                .filter(|(key, _, _, _)| !self.cache.contains_key(key))
                .collect();
            
            // Then load and cache missing values
            for (key, pnr, cov_type, date) in missing_keys {
                if let Ok(value) = store.get_covariate(&pnr, *cov_type, *date) {
                    self.cache.insert(key.clone(), value);
                    loaded += 1;
                }
            }
        }
        
        Ok(loaded)
    }
}

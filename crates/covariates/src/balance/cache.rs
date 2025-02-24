use chrono::NaiveDate;
use dashmap::DashMap;
use types::models::{Covariate, CovariateType};
use types::store::Store;
use types::IdsError;

#[derive(Hash, Eq, PartialEq, Clone)]
pub(crate) struct CacheKey {
    pub pnr: String,
    pub covariate_type: CovariateType,
    pub date: NaiveDate,
}

impl CacheKey {
    pub fn new(pnr: &str, covariate_type: CovariateType, date: NaiveDate) -> Self {
        Self {
            pnr: pnr.to_string(),
            covariate_type,
            date,
        }
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
        if let Some(cached) = self.get(&key) {
            return Ok(cached);
        }

        let value = store.get_covariate(&key.pnr, key.covariate_type, key.date)?;
        self.insert(key, value.clone());
        Ok(value)
    }
}

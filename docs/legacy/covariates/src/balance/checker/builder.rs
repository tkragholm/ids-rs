use super::BalanceChecker;
use crate::balance::metrics::BalanceMetrics;
use types::{
    error::{IdsError, Result},
    storage::{arrow::backend::ArrowBackend as ArrowStore, CovariateCache, ThreadSafeStore},
};

/// Builder for `BalanceChecker` with configurable settings
pub struct BalanceCheckerBuilder {
    store: Option<ArrowStore>,
    cache_capacity: usize,
    debug_mode: bool,
}

impl Default for BalanceCheckerBuilder {
    fn default() -> Self {
        Self {
            store: None,
            cache_capacity: 100_000, // Default cache capacity
            debug_mode: false,
        }
    }
}

impl BalanceCheckerBuilder {
    /// Create a new builder with default settings
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the data store for the `BalanceChecker`
    #[must_use] pub fn with_store(mut self, store: ArrowStore) -> Self {
        self.store = Some(store);
        self
    }

    /// Set the cache capacity for the `BalanceChecker`
    #[must_use] pub fn with_cache_capacity(mut self, capacity: usize) -> Self {
        self.cache_capacity = capacity;
        self
    }

    /// Enable or disable debug mode for the `BalanceChecker`
    #[must_use] pub fn with_debug_mode(mut self, debug: bool) -> Self {
        self.debug_mode = debug;
        self
    }

    /// Build the `BalanceChecker` with the configured settings
    ///
    /// # Errors
    ///
    /// Returns an error if no store was provided
    pub fn build(self) -> Result<BalanceChecker> {
        let store = self.store.ok_or_else(|| {
            IdsError::invalid_operation("Cannot build BalanceChecker without a store".to_string())
        })?;

        Ok(BalanceChecker {
            store: ThreadSafeStore::new(store),
            cache: CovariateCache::new(self.cache_capacity),
            metrics: BalanceMetrics::new(),
            results: None,
        })
    }
}

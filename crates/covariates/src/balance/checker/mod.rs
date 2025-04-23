// Core structure for the BalanceChecker with essential methods
mod balance_calculation;
pub mod builder;
mod paired_analysis;
mod performance;

use crate::balance::{metrics::BalanceMetrics, results::BalanceResults};
use crate::models::{CovariateSummary, MatchedPairDetail};
use chrono::NaiveDate;
use std::collections::HashMap;
use types::storage::{CacheKey, CovariateCache, ThreadSafeStore};
use types::{
    error::Result,
    models::{Covariate, CovariateType},
    storage::arrow::backend::ArrowBackend as ArrowStore,
    traits::Store,
};

// Re-export important types and the builder
pub use self::builder::BalanceCheckerBuilder;

/// Main balance checker for analyzing covariate balance between case and control groups
pub struct BalanceChecker {
    pub(crate) store: ThreadSafeStore<ArrowStore>,
    pub(crate) cache: CovariateCache,
    pub(crate) metrics: BalanceMetrics,
    pub(crate) results: Option<BalanceResults>,
}

impl BalanceChecker {
    /// Creates a new BalanceChecker with the provided data store
    #[must_use]
    pub fn new(store: ArrowStore) -> Self {
        Self {
            store: ThreadSafeStore::new(store),
            cache: CovariateCache::new(100_000),
            metrics: BalanceMetrics::new(),
            results: None,
        }
    }

    /// Returns a builder for creating a BalanceChecker with custom settings
    pub fn builder() -> BalanceCheckerBuilder {
        BalanceCheckerBuilder::new()
    }

    /// Get a covariate value for a specific PNR, type, and date
    pub fn covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>> {
        let key = CacheKey::new(pnr, covariate_type, date);

        // First check the cache
        if let Some(value) = self.cache.get(&key) {
            return Ok(value);
        }

        // Not in cache, get from store
        let mut store = self.store.write();
        let value = store.covariate(pnr, covariate_type, date)?;

        // Cache the result
        self.cache.insert(key, value.clone());

        Ok(value)
    }

    /// Backward compatibility method, deprecated
    #[deprecated(note = "Use covariate method instead")]
    pub fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>> {
        self.covariate(pnr, covariate_type, date)
    }

    /// Clears the covariate cache
    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    /// Returns the current size of the covariate cache
    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    /// Alias for cache_size for backward compatibility
    pub fn cache_len(&self) -> usize {
        self.cache.len()
    }

    /// Add a value to the cache (used for testing)
    pub fn add_to_cache(&self, key: CacheKey, value: Option<types::models::Covariate>) {
        self.cache.insert(key, value);
    }

    /// Get the summary for a specific variable from the results
    pub fn get_variable_summary(&self, variable: &str) -> Option<&CovariateSummary> {
        self.results
            .as_ref()
            .and_then(|r| r.summaries.iter().find(|s| s.variable == variable))
    }

    /// Get a reference to the results (mainly for testing)
    pub fn results(&self) -> Option<&BalanceResults> {
        self.results.as_ref()
    }

    /// Get matched pair details for a specific case PNR
    pub fn get_matched_pair_details(&self, case_pnr: &str) -> Vec<&MatchedPairDetail> {
        self.results
            .as_ref()
            .map(|r| {
                r.matched_pair_details
                    .iter()
                    .filter(|d| d.case_pnr == case_pnr)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Summarizes standardized differences across matched pairs
    pub fn summarize_std_differences(&self) -> HashMap<String, (f64, f64, f64)> {
        let mut summaries = HashMap::new();

        if let Some(results) = &self.results {
            for detail in &results.matched_pair_details {
                let stats = summaries
                    .entry(detail.variable.clone())
                    .or_insert((0.0, 0.0, 0));

                stats.0 += detail.std_diff;
                stats.1 += detail.std_diff.powi(2);
                stats.2 += 1;
            }
        }

        summaries
            .into_iter()
            .map(|(var, (sum, sum_sq, n))| {
                let n = n as f64;
                let mean = sum / n;
                let variance = (sum_sq / n) - mean.powi(2);
                (var, (mean, variance.sqrt(), n))
            })
            .collect()
    }
}

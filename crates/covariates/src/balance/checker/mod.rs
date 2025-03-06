// Core structure for the BalanceChecker with essential methods
pub mod builder;
mod balance_calculation;
mod paired_analysis;
mod performance;

use crate::balance::{
    legacy_cache::{CacheKey, CovariateCache},
    metrics::BalanceMetrics,
    results::BalanceResults,
};
use crate::models::{CovariateSummary, MatchedPairDetail};
use chrono::NaiveDate;
use std::{collections::HashMap, sync::Arc};
use types::{
    error::IdsError,
    models::{Covariate, CovariateType},
    storage::arrow::backend::ArrowBackend as ArrowStore,
};

// Re-export important types and the builder
pub use self::builder::BalanceCheckerBuilder;

/// Main balance checker for analyzing covariate balance between case and control groups
pub struct BalanceChecker {
    pub(crate) store: Arc<ArrowStore>,
    pub(crate) cache: CovariateCache,
    pub(crate) metrics: BalanceMetrics,
    pub(crate) results: Option<BalanceResults>,
}

impl BalanceChecker {
    /// Creates a new BalanceChecker with the provided data store
    #[must_use]
    pub fn new(store: ArrowStore) -> Self {
        Self {
            store: Arc::new(store),
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
    ) -> Result<Option<Covariate>, IdsError> {
        let key = CacheKey::new(pnr, covariate_type, date);
        self.cache.get_or_load(&*self.store, key)
    }
    
    /// Backward compatibility method, deprecated
    #[deprecated(note = "Use covariate method instead")]
    pub fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
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
    pub fn add_to_cache(&self, key: crate::balance::legacy_cache::CacheKey, value: Option<types::models::Covariate>) {
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
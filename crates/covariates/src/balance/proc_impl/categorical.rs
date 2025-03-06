use super::super::BalanceChecker;
use super::super::memory::MemoryGuard;
use super::OptimizationStrategy;
use super::date_grouping::DateGroupingParams;
use super::progress::create_progress_style;
use super::config::ProcessorConfig;
use chrono::NaiveDate;
use hashbrown::HashMap;
use indicatif::ParallelProgressIterator;
use rayon::prelude::*;
use std::time::Instant;
use types::models::{Covariate, CovariateType};

/// Processor for categorical covariate values
pub struct CategoricalProcessor {
    config: ProcessorConfig,
}

impl CategoricalProcessor {
    /// Create a new categorical processor with the given configuration
    pub fn new(config: ProcessorConfig) -> Self {
        Self { config }
    }

    /// Collect categorical values for a list of subjects
    pub fn collect_categorical_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
    ) -> (Vec<String>, usize)
    where
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        // Get the optimal chunk size for this workload
        let chunk_size = self.config.get_optimal_chunk_size(subjects.len());

        // Extract unique dates for prefetching and improved locality
        let dates: Vec<_> = subjects
            .iter()
            .map(|(_, date)| *date)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        log::debug!(
            "Processing {} subjects with {} unique dates for categorical values, chunk size {}",
            subjects.len(),
            dates.len(),
            chunk_size
        );

        // Prefetch data if we have a good locality pattern (few dates, many subjects)
        if dates.len() < 5 && subjects.len() > 1000 {
            // Extract unique PNRs to avoid duplicates
            let pnrs: Vec<_> = subjects.iter().map(|(pnr, _)| pnr.clone()).collect();

            // Prefetch data for all combinations in a single batch
            let covariate_types = [covariate_type];
            log::info!(
                "Prefetching {} categorical data points ({}×{}×{})",
                pnrs.len() * covariate_types.len() * dates.len(),
                pnrs.len(),
                covariate_types.len(),
                dates.len()
            );
            checker.prefetch_data(&pnrs, &covariate_types, &dates);
        }

        // Create styled progress bar
        let style = create_progress_style(covariate_type);
        let progress = subjects
            .par_chunks(chunk_size)
            .progress_with_style(style)
            .with_prefix("Categorical Values")
            .with_message(format!(
                "Processing {} categorical covariates...",
                subjects.len()
            ));

        // Set global thread pool configuration (if not already set)
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.config.thread_count)
            .build_global()
            .ok(); // Ignore error if already initialized

        // Process chunks in parallel with better memory locality
        let results: Vec<_> = progress
            .map(|chunk| self.process_categorical_chunk(chunk, covariate_type, checker, extractor))
            .collect();

        // Calculate total capacity for the combined results
        let total_capacity: usize = results.iter().map(|(v, _)| v.len()).sum();

        // Create memory guard for the combined results
        // String sizes are variable, so estimate 32 bytes per string on average
        const AVG_STRING_SIZE: usize = 32;
        let guard_id = format!(
            "combined_categorical_{}_{}",
            covariate_type as u8, total_capacity
        );
        let _memory_guard = MemoryGuard::new(&guard_id, total_capacity * AVG_STRING_SIZE);

        // Allocate the result vector
        let mut all_values = Vec::with_capacity(total_capacity);
        let mut total_missing = 0;

        // Combine all partial results efficiently
        for (values, missing) in results {
            all_values.extend(values);
            total_missing += missing;
        }

        (all_values, total_missing)
    }

    /// Process a single chunk of subjects for categorical values - extracted for better code organization
    #[inline]
    fn process_categorical_chunk<F>(
        &self,
        chunk: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
    ) -> (Vec<String>, usize)
    where
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        // Reserve capacity to avoid reallocations
        let mut values = Vec::with_capacity(chunk.len());
        let mut missing = 0;

        // Track cache hits and misses for optimization
        let mut cache_hits = 0;
        let mut cache_misses = 0;

        // Process based on the selected optimization strategy
        match self.config.optimization_strategy {
            OptimizationStrategy::Safe => {
                // Safe mode: always process linearly without date grouping
                for (pnr, date) in chunk {
                    let start = std::time::Instant::now();
                    self.process_single_categorical_item(
                        pnr,
                        *date,
                        covariate_type,
                        checker,
                        extractor,
                        &mut values,
                        &mut missing,
                        &mut cache_hits,
                        &mut cache_misses,
                        start,
                    );
                }
            }
            OptimizationStrategy::Balanced => {
                // In balanced mode, only use date grouping for small chunks
                if chunk.len() < 500 {
                    // For small chunks, try the date grouping optimization
                    self.process_with_date_grouping_categorical(
                        DateGroupingParams {
                            chunk,
                            covariate_type, 
                            checker,
                            extractor,
                            values: &mut values,
                            missing: &mut missing,
                            cache_hits: &mut cache_hits,
                            cache_misses: &mut cache_misses,
                        }
                    );
                } else {
                    // For larger chunks, process linearly to avoid memory issues
                    for (pnr, date) in chunk {
                        let start = std::time::Instant::now();
                        self.process_single_categorical_item(
                            pnr,
                            *date,
                            covariate_type,
                            checker,
                            extractor,
                            &mut values,
                            &mut missing,
                            &mut cache_hits,
                            &mut cache_misses,
                            start,
                        );
                    }
                }
            }
            OptimizationStrategy::Performance => {
                // Performance mode: use date grouping optimization for all chunk sizes
                self.process_with_date_grouping_categorical(
                    DateGroupingParams {
                        chunk,
                        covariate_type,
                        checker,
                        extractor,
                        values: &mut values,
                        missing: &mut missing,
                        cache_hits: &mut cache_hits,
                        cache_misses: &mut cache_misses,
                    }
                );
            }
        }

        // Log cache performance statistics for larger chunks
        if chunk.len() > 1000 && (cache_hits + cache_misses > 0) {
            let hit_rate = (cache_hits as f64 / (cache_hits + cache_misses) as f64) * 100.0;
            log::debug!(
                "Cache performance for {} categorical items: {:.1}% hit rate ({} hits, {} misses)",
                chunk.len(),
                hit_rate,
                cache_hits,
                cache_misses
            );
        }

        (values, missing)
    }

    /// Process a single subject for categorical values
    #[inline]
    fn process_single_categorical_item<F>(
        &self,
        pnr: &str,
        date: NaiveDate,
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
        values: &mut Vec<String>,
        missing: &mut usize,
        cache_hits: &mut usize,
        cache_misses: &mut usize,
        start: Instant,
    ) where
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        // Get covariate from store, update cache metrics
        let covariate_result = checker.get_covariate(pnr, covariate_type, date);

        if covariate_result.is_ok() {
            let lookup_time = start.elapsed();
            if lookup_time.as_micros() < 50 {
                *cache_hits += 1;
            } else {
                *cache_misses += 1;
            }
        }

        // Handle the result
        match covariate_result {
            Ok(Some(covariate)) => {
                if let Some(value) = extractor(&covariate) {
                    values.push(value);
                } else {
                    *missing += 1;
                }
            }
            _ => *missing += 1,
        }
    }

    /// Process a chunk of subjects with date grouping optimization for categorical values
    #[inline]
    fn process_with_date_grouping_categorical<F>(
        &self,
        params: DateGroupingParams<'_, F, String>,
    ) where
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        // Create a memory reservation for the temporary date grouping
        let guard_id = format!(
            "date_group_categorical_{}_{}",
            params.covariate_type as u8,
            params.chunk.len()
        );
        let estimated_size =
            params.chunk.len() * (std::mem::size_of::<String>() + std::mem::size_of::<NaiveDate>() * 2);
        let _memory_guard = MemoryGuard::new(&guard_id, estimated_size);

        // Use date grouping for better cache locality
        let mut date_groups = HashMap::with_capacity(params.chunk.len() / 10);

        // Group by date first
        for (pnr, date) in params.chunk {
            date_groups
                .entry(*date)
                .or_insert_with(Vec::new)
                .push(pnr.as_str());
        }

        // Process each date group
        for (date, pnrs) in date_groups {
            for pnr in pnrs {
                let start = std::time::Instant::now();
                self.process_single_categorical_item(
                    pnr,
                    date,
                    params.covariate_type,
                    params.checker,
                    params.extractor,
                    params.values,
                    params.missing,
                    params.cache_hits,
                    params.cache_misses,
                    start,
                );
            }
        }
    }
}
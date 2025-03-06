use super::BalanceChecker;
use chrono::NaiveDate;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use types::models::{Covariate, CovariateType};

use super::memory::{memory_manager, MemoryGuard};

/// Controls how data is processed during balance checking
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptimizationStrategy {
    /// Safe mode: no date grouping, linear processing only
    /// Best for systems with limited memory or when stability is critical
    #[default]
    Safe,

    /// Balanced mode: uses date grouping only for small datasets
    /// Good default for most systems
    Balanced,

    /// Performance mode: uses date grouping optimization extensively
    /// Best for high-memory systems (32+ GB) when speed is critical
    Performance,
}

impl OptimizationStrategy {
    /// Maps memory tier to optimization strategy
    pub fn from_memory_tier(tier: super::memory::MemoryTier) -> Self {
        match tier {
            super::memory::MemoryTier::VeryHigh => OptimizationStrategy::Performance,
            super::memory::MemoryTier::High => OptimizationStrategy::Performance,
            super::memory::MemoryTier::Medium => OptimizationStrategy::Balanced,
            super::memory::MemoryTier::Low => OptimizationStrategy::Safe,
        }
    }
}

pub(crate) struct ValueProcessor {
    thread_count: usize,
    chunk_size_multiplier: usize,
    optimization_strategy: OptimizationStrategy,
}

/// Parameters for processing a chunk of data with date grouping
struct DateGroupingParams<'a, F, V> {
    chunk: &'a [(String, NaiveDate)],
    covariate_type: CovariateType,
    checker: &'a BalanceChecker,
    extractor: &'a F,
    values: &'a mut Vec<V>,
    missing: &'a mut usize,
    cache_hits: &'a mut usize,
    cache_misses: &'a mut usize,
}

impl ValueProcessor {
    pub fn new() -> Self {
        // Use the memory manager to determine optimal settings
        let mem_manager = memory_manager();

        Self {
            thread_count: mem_manager.get_max_parallel_tasks(),
            chunk_size_multiplier: 1,
            optimization_strategy: OptimizationStrategy::from_memory_tier(mem_manager.get_tier()),
        }
    }

    /// Create a new ValueProcessor with custom configuration
    #[allow(dead_code)]
    pub fn with_config(
        thread_count: Option<usize>,
        chunk_size_multiplier: Option<usize>,
        optimization_strategy: Option<OptimizationStrategy>,
    ) -> Self {
        let mem_manager = memory_manager();

        Self {
            thread_count: thread_count.unwrap_or_else(|| mem_manager.get_max_parallel_tasks()),
            chunk_size_multiplier: chunk_size_multiplier.unwrap_or(1),
            optimization_strategy: optimization_strategy
                .unwrap_or_else(|| OptimizationStrategy::from_memory_tier(mem_manager.get_tier())),
        }
    }

    /// Configure optimization strategy
    #[allow(dead_code)]
    pub fn with_optimization_strategy(mut self, strategy: OptimizationStrategy) -> Self {
        self.optimization_strategy = strategy;
        self
    }

    /// Automatically select optimization strategy based on system resources
    #[allow(dead_code)]
    pub fn auto_configure(mut self) -> Self {
        // Get memory manager for system resources
        let mem_manager = memory_manager();

        // Select strategy based on available memory
        self.optimization_strategy = OptimizationStrategy::from_memory_tier(mem_manager.get_tier());

        log::info!(
            "Auto-configured optimization strategy: {:?} (detected memory tier: {:?})",
            self.optimization_strategy,
            mem_manager.get_tier()
        );

        self
    }

    /// Get the optimal chunk size based on workload and system capabilities
    fn get_optimal_chunk_size(&self, total_items: usize) -> usize {
        // Use memory manager for base chunk size determination
        let mem_manager = memory_manager();
        let base_chunk_size = mem_manager.get_optimal_chunk_size(total_items);

        // Apply the multiplier to allow tuning
        let chunk_size = base_chunk_size * self.chunk_size_multiplier;

        // Round to nearest hundred for cleaner numbers
        (chunk_size / 100) * 100
    }

    /// Create a progress style with a custom template and dynamic covariate type display
    fn create_progress_style(covariate_type: CovariateType) -> ProgressStyle {
        ProgressStyle::default_bar()
            .template(
                "{prefix:.bold.dim} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} \
                 ({percent}%) {msg}\n\
                 ⏱️  ETA: {eta_precise:.dim} | 🚀 {per_sec:.green} records/sec | \
                 📊 Processing: {covariate_type}",
            )
            .unwrap()
            .with_key(
                "covariate_type",
                move |_state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
                    write!(w, "{:?}", covariate_type).unwrap()
                },
            )
    }

    pub fn collect_numeric_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
    ) -> (Vec<f64>, usize)
    where
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
    {
        // Get the optimal chunk size for this workload
        let chunk_size = self.get_optimal_chunk_size(subjects.len());

        // Extract unique dates for prefetching
        let dates: Vec<_> = subjects
            .iter()
            .map(|(_, date)| *date)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        log::debug!(
            "Processing {} subjects with {} unique dates using chunk size {}",
            subjects.len(),
            dates.len(),
            chunk_size
        );

        // Prefetch data if we have a good locality pattern (few dates, many subjects)
        if dates.len() < 5 && subjects.len() > 1000 {
            // Extract unique PNRs to avoid duplicates in prefetching
            let pnrs: Vec<_> = subjects.iter().map(|(pnr, _)| pnr.clone()).collect();

            // Prefetch all data in a single batch
            let covariate_types = [covariate_type];
            log::info!(
                "Prefetching {} numeric data points ({}×{}×{})",
                pnrs.len() * covariate_types.len() * dates.len(),
                pnrs.len(),
                covariate_types.len(),
                dates.len()
            );
            checker.prefetch_data(&pnrs, &covariate_types, &dates);
        }

        // Create styled progress bar
        let style = Self::create_progress_style(covariate_type);
        let progress = subjects
            .par_chunks(chunk_size)
            .progress_with_style(style)
            .with_prefix("Numeric Values")
            .with_message(format!(
                "Processing {} numeric covariates...",
                subjects.len()
            ));

        // Set up rayon thread pool configuration (if needed)
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.thread_count)
            .build_global()
            .ok(); // Ignore error if already initialized

        // Process chunks in parallel with better memory locality
        let results: Vec<_> = progress
            .map(|chunk| self.process_numeric_chunk(chunk, covariate_type, checker, extractor))
            .collect();

        // Calculate total capacity and reserve memory for the combined results
        let total_capacity: usize = results.iter().map(|(v, _)| v.len()).sum();

        // Create a memory guard for the combined results
        let guard_id = format!(
            "combined_numeric_{}_{}",
            covariate_type as u8, total_capacity
        );
        let _memory_guard =
            MemoryGuard::new(&guard_id, total_capacity * std::mem::size_of::<f64>());

        // Allocate the result vector
        let mut all_values = Vec::with_capacity(total_capacity);
        let mut total_missing = 0;

        // Combine all partial results
        for (values, missing) in results {
            all_values.extend(values);
            total_missing += missing;
        }

        (all_values, total_missing)
    }

    /// Process a single chunk of subjects for numeric values - extracted for better code organization
    #[inline]
    fn process_numeric_chunk<F>(
        &self,
        chunk: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
    ) -> (Vec<f64>, usize)
    where
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
    {
        // Reserve capacity to avoid reallocations
        let mut values = Vec::with_capacity(chunk.len());
        let mut missing = 0;

        // Track cache hits and misses for optimization
        let mut cache_hits = 0;
        let mut cache_misses = 0;

        // Process based on the selected optimization strategy
        match self.optimization_strategy {
            OptimizationStrategy::Safe => {
                // Safe mode: always process linearly without date grouping
                for (pnr, date) in chunk {
                    let start = std::time::Instant::now();
                    self.process_single_numeric_item(
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
                // In balanced mode, only use date grouping for very small chunks
                if chunk.len() < 500 {
                    // For small chunks, try the date grouping optimization
                    self.process_with_date_grouping_numeric(
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
                        self.process_single_numeric_item(
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
                self.process_with_date_grouping_numeric(
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

        // Report cache performance data for optimization
        if chunk.len() > 1000 {
            let hit_rate = if cache_hits + cache_misses > 0 {
                (cache_hits as f64 / (cache_hits + cache_misses) as f64) * 100.0
            } else {
                0.0
            };

            log::debug!(
                "Cache performance for {} items: {:.1}% hit rate ({} hits, {} misses)",
                chunk.len(),
                hit_rate,
                cache_hits,
                cache_misses
            );
        }

        (values, missing)
    }

    /// Process a single item for numeric values - extracted for better inlining
    #[inline]
    #[allow(clippy::too_many_arguments)]
    fn process_single_numeric_item<F>(
        &self,
        pnr: &str,
        date: NaiveDate,
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
        values: &mut Vec<f64>,
        missing: &mut usize,
        cache_hits: &mut usize,
        cache_misses: &mut usize,
        start: std::time::Instant,
    ) where
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
    {
        match checker.get_covariate(pnr, covariate_type, date) {
            Ok(Some(covariate)) => {
                // Track if this was a fast cache hit (less than 50 microseconds)
                if start.elapsed().as_micros() < 50 {
                    *cache_hits += 1;
                } else {
                    *cache_misses += 1;
                }

                match extractor(&covariate) {
                    Some(value) => values.push(value),
                    None => *missing += 1,
                }
            }
            _ => *missing += 1,
        }
    }

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
        let chunk_size = self.get_optimal_chunk_size(subjects.len());

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
        let style = Self::create_progress_style(covariate_type);
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
            .num_threads(self.thread_count)
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
        match self.optimization_strategy {
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

    /// Process a single item for categorical values - extracted for better inlining
    #[inline]
    #[allow(clippy::too_many_arguments)]
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
        start: std::time::Instant,
    ) where
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        match checker.get_covariate(pnr, covariate_type, date) {
            Ok(Some(covariate)) => {
                // Track if this was a fast cache hit (less than 50 microseconds)
                if start.elapsed().as_micros() < 50 {
                    *cache_hits += 1;
                } else {
                    *cache_misses += 1;
                }

                match extractor(&covariate) {
                    Some(value) => values.push(value),
                    None => *missing += 1,
                }
            }
            _ => *missing += 1,
        }
    }


    /// Process a chunk of subjects with date grouping optimization for numeric values
    #[inline]
    fn process_with_date_grouping_numeric<F>(
        &self,
        params: DateGroupingParams<'_, F, f64>,
    ) where
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
    {
        // Create a memory reservation for the temporary date grouping
        let guard_id = format!(
            "date_group_numeric_{}_{}",
            params.covariate_type as u8,
            params.chunk.len()
        );
        let estimated_size =
            params.chunk.len() * (std::mem::size_of::<String>() + std::mem::size_of::<NaiveDate>() * 2);
        let _memory_guard = MemoryGuard::new(&guard_id, estimated_size);

        // Use date grouping for better cache locality
        let mut date_groups = hashbrown::HashMap::with_capacity(params.chunk.len() / 10);

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
                self.process_single_numeric_item(
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
        let mut date_groups = hashbrown::HashMap::with_capacity(params.chunk.len() / 10);

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

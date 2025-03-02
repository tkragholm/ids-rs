use super::checker::BalanceChecker;
use chrono::NaiveDate;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use types::models::{Covariate, CovariateType};

pub(crate) struct ValueProcessor {
    thread_count: usize,
    chunk_size_multiplier: usize,
}

impl ValueProcessor {
    pub fn new() -> Self {
        Self {
            thread_count: num_cpus::get(),
            chunk_size_multiplier: 1,
        }
    }

    /// Create a new ValueProcessor with custom configuration
    pub fn with_config(thread_count: Option<usize>, chunk_size_multiplier: Option<usize>) -> Self {
        Self {
            thread_count: thread_count.unwrap_or_else(num_cpus::get),
            chunk_size_multiplier: chunk_size_multiplier.unwrap_or(1),
        }
    }

    /// Get the optimal chunk size based on workload and system capabilities
    fn get_optimal_chunk_size(&self, total_items: usize) -> usize {
        // Base chunk size parameters
        const MIN_BATCH_SIZE: usize = 2_000;
        const TARGET_BATCH_SIZE: usize = 50_000;
        
        // Calculate optimal chunk size based on available cores
        let items_per_thread = (total_items / self.thread_count).max(MIN_BATCH_SIZE);
        let base_chunk_size = items_per_thread.min(TARGET_BATCH_SIZE);
        
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
        let dates: Vec<_> = subjects.iter()
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
            let pnrs: Vec<_> = subjects.iter()
                .map(|(pnr, _)| pnr.clone())
                .collect();
                
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
            .with_message(format!("Processing {} numeric covariates...", subjects.len()));

        // Set up rayon thread pool configuration (if needed)
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.thread_count)
            .build_global()
            .ok(); // Ignore error if already initialized
            
        // Process chunks in parallel with better memory locality
        let results: Vec<_> = progress
            .map(|chunk| {
                self.process_numeric_chunk(chunk, covariate_type, checker, extractor)
            })
            .collect();

        // Combine results efficiently with pre-allocated capacity
        let total_capacity: usize = results.iter().map(|(v, _)| v.len()).sum();
        let mut all_values = Vec::with_capacity(total_capacity);
        let mut total_missing = 0;

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

        // Use an adaptive batching approach for small chunks
        if chunk.len() < 100 {
            // For small chunks, process items one by one
            for (pnr, date) in chunk {
                let start = std::time::Instant::now();
                self.process_single_numeric_item(
                    pnr, *date, covariate_type, checker, extractor,
                    &mut values, &mut missing, &mut cache_hits, &mut cache_misses,
                    start
                );
            }
        } else {
            // For larger chunks, batch items by date for better cache locality
            let mut date_groups: std::collections::HashMap<NaiveDate, Vec<&str>> = 
                std::collections::HashMap::new();
                
            // Group by date first
            for (pnr, date) in chunk {
                date_groups.entry(*date).or_default().push(pnr);
            }
            
            // Process each date group
            for (date, pnrs) in date_groups {
                for pnr in pnrs {
                    let start = std::time::Instant::now();
                    self.process_single_numeric_item(
                        pnr, date, covariate_type, checker, extractor,
                        &mut values, &mut missing, &mut cache_hits, &mut cache_misses,
                        start
                    );
                }
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
    )
    where
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
            },
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
        let dates: Vec<_> = subjects.iter()
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
            let pnrs: Vec<_> = subjects.iter()
                .map(|(pnr, _)| pnr.clone())
                .collect();
                
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
            .with_message(format!("Processing {} categorical covariates...", subjects.len()));

        // Set global thread pool configuration (if not already set)
        rayon::ThreadPoolBuilder::new()
            .num_threads(self.thread_count)
            .build_global()
            .ok(); // Ignore error if already initialized
            
        // Process chunks in parallel with better memory locality
        let results: Vec<_> = progress
            .map(|chunk| {
                self.process_categorical_chunk(chunk, covariate_type, checker, extractor)
            })
            .collect();

        // Calculate total sizes for pre-allocation
        let total_capacity: usize = results.iter().map(|(v, _)| v.len()).sum();
        let mut all_values = Vec::with_capacity(total_capacity);
        let mut total_missing = 0;

        // Collect results efficiently
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

        // Use an adaptive approach based on chunk size
        if chunk.len() < 100 {
            // For small chunks, process items linearly
            for (pnr, date) in chunk {
                let start = std::time::Instant::now();
                self.process_single_categorical_item(
                    pnr, *date, covariate_type, checker, extractor,
                    &mut values, &mut missing, &mut cache_hits, &mut cache_misses,
                    start
                );
            }
        } else {
            // For larger chunks, group by date for better locality
            let mut date_groups: std::collections::HashMap<NaiveDate, Vec<&str>> = 
                std::collections::HashMap::new();
                
            // Group by date first for better cache locality
            for (pnr, date) in chunk {
                date_groups.entry(*date).or_default().push(pnr);
            }
            
            // Process each date group (maximizes cache hits)
            for (date, pnrs) in date_groups {
                for pnr in pnrs {
                    let start = std::time::Instant::now();
                    self.process_single_categorical_item(
                        pnr, date, covariate_type, checker, extractor,
                        &mut values, &mut missing, &mut cache_hits, &mut cache_misses,
                        start
                    );
                }
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
    )
    where
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
            },
            _ => *missing += 1,
        }
    }
}

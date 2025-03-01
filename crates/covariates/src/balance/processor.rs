use super::checker::BalanceChecker;
use chrono::NaiveDate;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use types::models::{Covariate, CovariateType};

pub(crate) struct ValueProcessor;

impl ValueProcessor {
    pub fn new() -> Self {
        Self
    }

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
        // Optimize for larger batch sizes on modern CPUs
        const MIN_BATCH_SIZE: usize = 2_000; // Minimum batch size
        const TARGET_BATCH_SIZE: usize = 50_000; // Target items per thread
        
        // Calculate optimal chunk size based on available cores
        let num_threads = rayon::current_num_threads();
        let items_per_thread = (subjects.len() / num_threads).max(MIN_BATCH_SIZE);
        let chunk_size = (items_per_thread.min(TARGET_BATCH_SIZE) / 100) * 100; // Round to hundreds
        
        // Pre-populate common dates to improve cache locality
        let dates: Vec<_> = subjects.iter()
            .map(|(_, date)| *date)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
            
        if dates.len() < 5 && subjects.len() > 1000 {
            // If we have few unique dates and many subjects, prefetch data
            let pnrs: Vec<_> = subjects.iter()
                .map(|(pnr, _)| pnr.clone())
                .collect();
                
            // Prefetch data directly without spawning a thread
            // This avoids lifetime issues with the checker reference
            let covariate_types = [covariate_type];
            log::info!("Prefetching numeric data for {} PNRs across {} date(s)", pnrs.len(), dates.len());
            checker.prefetch_data(&pnrs, &covariate_types, &dates);
        }

        // Create styled progress bar
        let style = Self::create_progress_style(covariate_type);
        let progress = subjects
            .par_chunks(chunk_size)
            .progress_with_style(style)
            .with_prefix("Numeric Values")
            .with_message(format!("Processing {} numeric covariates...", subjects.len()));

        // Process chunks in parallel with reservation hints
        let results: Vec<_> = progress
            .map(|chunk| {
                // Reserve capacity to avoid reallocations
                let mut values = Vec::with_capacity(chunk.len());
                let mut missing = 0;
                
                // Track cache hits and misses for optimization
                let mut cache_hits = 0;
                let mut cache_misses = 0;

                // Process each subject
                for (pnr, date) in chunk {
                    let start = std::time::Instant::now();
                    match checker.get_covariate(pnr, covariate_type, *date) {
                        Ok(Some(covariate)) => {
                            // Track if this was a fast cache hit
                            if start.elapsed().as_micros() < 50 {
                                cache_hits += 1;
                            } else {
                                cache_misses += 1;
                            }
                            
                            match extractor(&covariate) {
                                Some(value) => values.push(value),
                                None => missing += 1,
                            }
                        },
                        _ => missing += 1,
                    }
                }
                
                // Report cache performance data for optimization
                if chunk.len() > 1000 {
                    log::debug!(
                        "Cache performance for {} items: {}% hit rate ({} hits, {} misses)",
                        chunk.len(),
                        (cache_hits as f64 / (cache_hits + cache_misses) as f64) * 100.0,
                        cache_hits,
                        cache_misses
                    );
                }

                (values, missing)
            })
            .collect();

        // Combine results efficiently
        let total_capacity: usize = results.iter().map(|(v, _)| v.len()).sum();
        let mut all_values = Vec::with_capacity(total_capacity);
        let mut total_missing = 0;

        // Use extend_from_slice when possible for better performance
        for (values, missing) in results {
            all_values.extend(values);
            total_missing += missing;
        }

        (all_values, total_missing)
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
        // Optimize for larger batch sizes on modern CPUs
        const MIN_BATCH_SIZE: usize = 2_000; // Minimum batch size
        const TARGET_BATCH_SIZE: usize = 50_000; // Target items per thread
        
        // Calculate optimal chunk size based on available cores
        let num_threads = rayon::current_num_threads();
        let items_per_thread = (subjects.len() / num_threads).max(MIN_BATCH_SIZE);
        let chunk_size = (items_per_thread.min(TARGET_BATCH_SIZE) / 100) * 100; // Round to hundreds
        
        // Pre-populate common dates to improve cache locality
        let dates: Vec<_> = subjects.iter()
            .map(|(_, date)| *date)
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();
            
        if dates.len() < 5 && subjects.len() > 1000 {
            // If we have few unique dates and many subjects, prefetch data
            let pnrs: Vec<_> = subjects.iter()
                .map(|(pnr, _)| pnr.clone())
                .collect();
                
            // Prefetch data directly without using a background thread
            // This resolves the lifetime issues with borrowed references
            let covariate_types = [covariate_type];
            log::info!("Prefetching categorical data for {} PNRs across {} date(s)", pnrs.len(), dates.len());
            checker.prefetch_data(&pnrs, &covariate_types, &dates);
        }

        // Create styled progress bar
        let style = Self::create_progress_style(covariate_type);
        let progress = subjects
            .par_chunks(chunk_size)
            .progress_with_style(style)
            .with_prefix("Categorical Values")
            .with_message(format!("Processing {} categorical covariates...", subjects.len()));

        // Process chunks in parallel
        let results: Vec<_> = progress
            .map(|chunk| {
                // Reserve capacity up front
                let mut values = Vec::with_capacity(chunk.len());
                let mut missing = 0;
                
                // Track cache performance
                let mut cache_hits = 0;
                let mut cache_misses = 0;

                for (pnr, date) in chunk {
                    // Time the lookup to detect cache hits vs misses
                    let start = std::time::Instant::now();
                    match checker.get_covariate(pnr, covariate_type, *date) {
                        Ok(Some(covariate)) => {
                            // Track cache performance
                            if start.elapsed().as_micros() < 50 {
                                cache_hits += 1;
                            } else {
                                cache_misses += 1;
                            }
                            
                            match extractor(&covariate) {
                                Some(value) => values.push(value),
                                None => missing += 1,
                            }
                        },
                        _ => missing += 1,
                    }
                }
                
                // Log cache performance for chunks
                if chunk.len() > 1000 {
                    log::debug!(
                        "Cache performance for {} categorical items: {}% hit rate",
                        chunk.len(),
                        (cache_hits as f64 / (cache_hits + cache_misses) as f64) * 100.0
                    );
                }

                (values, missing)
            })
            .collect();

        // Calculate total sizes
        let total_capacity: usize = results.iter().map(|(v, _)| v.len()).sum();
        let mut all_values = Vec::with_capacity(total_capacity);
        let mut total_missing = 0;

        // Collect results more efficiently
        for (values, missing) in results {
            all_values.extend(values);
            total_missing += missing;
        }

        (all_values, total_missing)
    }
}

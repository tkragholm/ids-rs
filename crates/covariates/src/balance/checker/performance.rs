use super::BalanceChecker;
use types::error::IdsError;

impl BalanceChecker {
    /// Process data in parallel with optimal chunk sizing and workload balancing
    pub fn process_data_in_parallel<T, R, F>(
        &self,
        data: &[T],
        processor: F,
    ) -> Result<Vec<R>, IdsError>
    where
        T: Send + Sync,
        R: Send,
        F: Fn(&T) -> Result<R, IdsError> + Send + Sync,
    {
        use rayon::prelude::*;
        
        // Determine optimal chunk size based on number of items and available threads
        let num_threads = rayon::current_num_threads();
        let items_per_thread = div_ceil(data.len(), num_threads);
        let chunk_size = items_per_thread.clamp(1, 1000); // At least 1, at most 1000
        
        // Process data in parallel chunks
        data.par_chunks(chunk_size)
            .flat_map(|chunk| {
                chunk.iter()
                    .map(&processor)
                    .collect::<Vec<Result<R, IdsError>>>()
            })
            .collect::<Result<Vec<R>, IdsError>>()
    }
    
    /// Get or compute a value with caching
    pub fn get_or_compute<K, V, F>(&self, _key: K, compute_fn: F) -> Result<V, IdsError>
    where
        K: std::hash::Hash + Eq + Clone,
        V: Clone,
        F: FnOnce() -> Result<V, IdsError>,
    {
        // This is a simplified implementation that doesn't use time-based caching
        // This would need to be expanded with thread-safe storage in a real implementation
        
        // For now, we'll just compute the value directly
        compute_fn()
    }

    /// Prefetch data for multiple PNRs and covariates to improve performance
    pub fn prefetch_data(
        &self,
        pnrs: &[String],
        covariate_types: &[types::models::CovariateType],
        dates: &[chrono::NaiveDate],
    ) -> usize {
        // Skip if the dataset is too small to benefit from prefetching
        if pnrs.len() * covariate_types.len() * dates.len() < 100 {
            return 0;
        }

        log::info!(
            "Prefetching data for {} PNRs, {} covariate types, and {} dates ({} total combinations)",
            pnrs.len(),
            covariate_types.len(),
            dates.len(),
            pnrs.len() * covariate_types.len() * dates.len()
        );

        let start = std::time::Instant::now();

        // Get a write lock on the store for exclusive access during bulk operations
        let mut store = self.store.write();
        
        match self
            .cache
            .bulk_load(&mut *store, pnrs, covariate_types, dates)
        {
            Ok(count) => {
                let elapsed = start.elapsed();
                log::info!(
                    "Prefetched {} covariate values in {:.2?} ({:.1} values/sec)",
                    count,
                    elapsed,
                    count as f64 / elapsed.as_secs_f64()
                );
                count
            }
            Err(e) => {
                log::warn!("Error during data prefetching: {}", e);
                0
            }
        }
    }
}

// Helper function for integer division with ceiling
fn div_ceil(a: usize, b: usize) -> usize {
    if b == 0 {
        panic!("Division by zero");
    }
    let d = a / b;
    let r = a % b;
    if r > 0 && b > 0 {
        d + 1
    } else {
        d
    }
}
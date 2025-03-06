use super::{
    legacy_cache::{CacheKey, CovariateCache},
    metrics::BalanceMetrics,
    results::BalanceResults,
};
use crate::models::{CovariateSummary, MatchedPairDetail};
use chrono::NaiveDate;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::debug;
use std::{collections::HashMap, sync::Arc};
use types::{
    error::IdsError,
    models::{Covariate, CovariateType},
    storage::ArrowBackend as ArrowStore,
};

/// Performance metrics for cache analysis
#[derive(Debug, Clone)]
pub struct CachePerformanceMetrics {
    pub total_entries: usize,
    pub hit_ratio: f64,
    pub memory_usage: usize,
    pub access_pattern: String,
}

pub struct BalanceChecker {
    store: Arc<ArrowStore>,
    cache: CovariateCache,
    metrics: BalanceMetrics,
    //processor: ValueProcessor,
    results: Option<BalanceResults>,
}

/// Builder for BalanceChecker with configurable settings
pub struct BalanceCheckerBuilder {
    store: Option<ArrowStore>,
    cache_capacity: usize,
    debug_mode: bool,
}

impl BalanceCheckerBuilder {
    pub fn new() -> Self {
        Self {
            store: None,
            cache_capacity: 100_000, // Default cache capacity
            debug_mode: false,
        }
    }
    
    pub fn with_store(mut self, store: ArrowStore) -> Self {
        self.store = Some(store);
        self
    }
    
    pub fn with_cache_capacity(mut self, capacity: usize) -> Self {
        self.cache_capacity = capacity;
        self
    }
    
    pub fn with_debug_mode(mut self, debug: bool) -> Self {
        self.debug_mode = debug;
        self
    }
    
    pub fn build(self) -> Result<BalanceChecker, IdsError> {
        let store = self.store.ok_or_else(|| 
            IdsError::invalid_operation("Cannot build BalanceChecker without a store".to_string())
        )?;
        
        Ok(BalanceChecker {
            store: Arc::new(store),
            cache: CovariateCache::new(self.cache_capacity),
            metrics: BalanceMetrics::new(),
            results: None,
        })
    }
}

impl BalanceChecker {
    #[must_use]
    pub fn new(store: ArrowStore) -> Self {
        Self {
            store: Arc::new(store),
            cache: CovariateCache::new(100_000),
            metrics: BalanceMetrics::new(),
            //processor: ValueProcessor::new(),
            results: None,
        }
    }

    /// Create a new checker with an empty store (for diagnostic mode)
    #[must_use]
    pub fn new_diagnostic() -> Self {
        let empty_store = ArrowStore::new_empty();

        // Create a checker with an empty store but with a cache that simulates having some data
        let checker = Self {
            store: Arc::new(empty_store),
            cache: CovariateCache::new(1000),
            metrics: BalanceMetrics::new(),
            results: None,
        };

        // Populate the cache with some placeholder data for diagnostic purposes
        checker.populate_diagnostic_cache();

        checker
    }

    /// Create a new diagnostic checker using actual PNRs from matched pairs
    #[must_use]
    pub fn new_diagnostic_with_pnrs(pnrs: Vec<String>) -> Self {
        let empty_store = ArrowStore::new_empty();

        // Create a checker with an empty store but with a cache that simulates having some data
        let checker = Self {
            store: Arc::new(empty_store),
            cache: CovariateCache::new(100_000), // Larger cache for real PNRs
            metrics: BalanceMetrics::new(),
            results: None,
        };

        // Populate the cache with data using the actual PNRs from matched pairs
        checker.populate_diagnostic_cache_with_pnrs(pnrs);

        checker
    }

    /// Populate the cache with some placeholder data for diagnostic purposes using standard test data
    fn populate_diagnostic_cache(&self) {
        use chrono::NaiveDate;
        use log::debug;
        use types::models::{Covariate, CovariateType};

        // Get real treatment dates to use
        let treatment_dates = [
            NaiveDate::from_ymd_opt(2008, 1, 30).unwrap(), // From the matched pairs file
            NaiveDate::from_ymd_opt(2010, 5, 15).unwrap(),
            NaiveDate::from_ymd_opt(2015, 10, 20).unwrap(),
            NaiveDate::from_ymd_opt(2020, 3, 10).unwrap(),
        ];

        debug!("Populating diagnostic cache with placeholder data");

        // Create example covariates for different types
        for i in 0..1000 {
            // Generate unique IDs - use both the simulated C/K format and actual PNRs
            // from the matched pairs file to help with debugging real data
            let case_id = format!("C{:06}", i);
            let control_id = format!("K{:06}", i);

            // Add actual PNR formats from the matched pairs file
            let real_pnrs = [
                "290903-8391", // This is an actual PNR from the matched pairs file
                "010903-8131",
                "190903-9483",
                "200903-3841",
                "081003-0785",
                // Add some formatted like our generated PNRs
                &format!(
                    "{:02}{:02}{:02}-{:04}",
                    (1 + (i % 30)),
                    (1 + (i % 12)),
                    (90 + (i % 10)),
                    1000 + (i % 9000)
                ),
            ];

            // Also try similar PNR formats to help with actual debugging
            let real_pnr_1 = format!("{:06}-{:04}", 100000 + (i % 300000), 1000 + (i % 9000));

            // Combine all ID formats for maximum coverage
            let mut all_ids = Vec::new();
            all_ids.push(case_id.clone());
            all_ids.push(control_id.clone());
            all_ids.push(real_pnr_1.clone());
            all_ids.extend(real_pnrs.iter().map(|s| s.to_string()));

            // Debug first few entries to make sure we're generating what we expect
            if i < 3 {
                debug!("Example diagnostic PNRs: {:?}", all_ids);
            }

            // Use all ID formats to provide maximum coverage
            for id in &all_ids {
                // Prepare common data that we'll use for each date
                // Demographics data
                let family_size = 3 + (i % 5) as i32;  // Family size 3-7
                let municipality = 101 + (i % 100) as i32;  // Municipality
                let family_type = format!("{}", 1 + (i % 9));  // Family type
                let civil_status = format!("{}", "G".chars().nth(i % 4).unwrap_or('G'));
                let gender = if i % 2 == 0 { "M".to_string() } else { "K".to_string() };
                let citizenship = "DK".to_string();
                let age = 20 + (i % 80) as i32;
                let children_count = (i % 4) as i32;

                // Income data
                let income_amount = 250000.0 + (i as f64 * 1000.0);
                let wage_income = 200000.0 + (i as f64 * 800.0); // LOENMV_13
                let employment_status = 1 + (i % 5) as i32;      // BESKST13

                // Education data
                let edu_level = 10 + (i % 20);
                let isced_level = 1 + (i % 8);
                let edu_years = 3.5 + (i % 10) as f32 / 2.0;
                
                // Occupation data
                let socio13_codes = [
                    "110", "111", "112", "113", "114", "120", 
                    "131", "132", "133", "134", "135", "139", 
                    "210", "220", "310", "321", "322", "323", "330"
                ];
                let socio13_code = socio13_codes[i % socio13_codes.len()];
                let socio = 100 + (i % 50) as i32; // SOCIO
                let socio02 = 200 + (i % 30) as i32; // SOCIO02
                let pre_socio = 10 + (i % 20) as i32; // PRE_SOCIO

                // Add to cache for different dates including treatment dates
                for date in &treatment_dates {
                    // Create fresh builder instances for each date iteration
                    
                    // Demographics
                    let demographic = Covariate::demographics(
                        family_size,
                        municipality,
                        family_type.clone(),
                    )
                    .with_civil_status(civil_status.clone())
                    .with_gender(gender.clone())
                    .with_citizenship(citizenship.clone())
                    .with_age(age)
                    .with_children_count(children_count);
                    let key = CacheKey::new(id, CovariateType::Demographics, *date);
                    self.cache.insert(key, Some(demographic.build()));

                    // Income
                    let income = Covariate::income(
                        income_amount,
                        "DKK".to_string(),
                        "PERINDKIALT_13".to_string()
                    )
                    .with_wage_income(wage_income)
                    .with_employment_status(employment_status);
                    let key = CacheKey::new(id, CovariateType::Income, *date);
                    self.cache.insert(key, Some(income.build()));

                    // Education
                    let education = Covariate::education(format!("{}", edu_level))
                        .with_isced_code(format!("{}", isced_level))
                        .with_years(edu_years);
                    let key = CacheKey::new(id, CovariateType::Education, *date);
                    self.cache.insert(key, Some(education.build()));
                    
                    // Occupation
                    let occupation = Covariate::occupation(
                        socio13_code.to_string(),
                        "SOCIO13".to_string()
                    )
                    .with_socio(socio)
                    .with_socio02(socio02)
                    .with_pre_socio(pre_socio);
                    let key = CacheKey::new(id, CovariateType::Occupation, *date);
                    self.cache.insert(key, Some(occupation.build()));
                }
            }
        }

        // Add some deliberate missing values - around 10%
        for i in 0..100 {
            let case_id = format!("C{:06}", i);
            let key = CacheKey::new(&case_id, CovariateType::Education, treatment_dates[0]);
            self.cache.insert(key, None);
        }

        debug!(
            "Diagnostic cache populated with {} entries",
            self.cache.len()
        );
    }

    /// Populate the cache with data using actual PNRs from matched pairs
    fn populate_diagnostic_cache_with_pnrs(&self, pnrs: Vec<String>) {
        use chrono::{Datelike, NaiveDate};
        use log::{debug, info};
        use rand::{Rng, rngs::StdRng, SeedableRng};
        use types::models::{Covariate, CovariateType};

        info!(
            "Initializing diagnostic cache with {} real PNRs from matched pairs",
            pnrs.len()
        );

        // Get treatment dates to use - include more recent years to cover more cases
        let treatment_dates = [
            NaiveDate::from_ymd_opt(2008, 1, 30).unwrap(), // Common from matched pairs
            NaiveDate::from_ymd_opt(2010, 5, 15).unwrap(),
            NaiveDate::from_ymd_opt(2015, 10, 20).unwrap(),
            NaiveDate::from_ymd_opt(2020, 3, 10).unwrap(),
            NaiveDate::from_ymd_opt(2022, 6, 1).unwrap(), // More recent
            NaiveDate::from_ymd_opt(2023, 9, 15).unwrap(), // More recent
        ];

        let mut rng = StdRng::from_os_rng();

        // Show the first few PNRs we're using
        if !pnrs.is_empty() {
            let sample_size = std::cmp::min(5, pnrs.len());
            info!(
                "Sample PNRs (first {} of {}): {:?}",
                sample_size,
                pnrs.len(),
                &pnrs[0..sample_size]
            );

            // Show detailed format info for the first PNR to help with debugging
            if !pnrs.is_empty() {
                let first_pnr = &pnrs[0];
                debug!("First PNR format analysis:");
                debug!("  Value: '{}'", first_pnr);
                debug!("  Length: {}", first_pnr.len());
                debug!("  Contains hyphen: {}", first_pnr.contains('-'));
                debug!(
                    "  First 6 chars: '{}'",
                    if first_pnr.len() >= 6 {
                        &first_pnr[0..6]
                    } else {
                        first_pnr
                    }
                );
                if first_pnr.len() > 6 {
                    debug!("  Last chars: '{}'", &first_pnr[6..]);
                }
            }
        }

        // Generate synthetic data for all the real PNRs
        for (i, pnr) in pnrs.iter().enumerate() {
            // Also generate and add the C/K format IDs to ensure compatibility
            let additional_ids = if i % 2 == 0 {
                // This is a case
                vec![format!("C{:06}", i)]
            } else {
                // This is a control
                vec![format!("K{:06}", i)]
            };

            // Add alternate formats of the PNR (with and without hyphen)
            let mut alternate_formats = Vec::new();
            if pnr.contains('-') {
                // Add version without hyphen
                alternate_formats.push(pnr.replace('-', ""));
            } else if pnr.len() > 6 {
                // Add version with hyphen
                alternate_formats.push(format!("{}-{}", &pnr[0..6], &pnr[6..]));
            }

            // Combine all ID formats for maximum coverage
            let all_ids = vec![pnr.clone()]
                .into_iter()
                .chain(additional_ids.into_iter())
                .chain(alternate_formats.into_iter())
                .collect::<Vec<_>>();

            // For the first few entries, show what alternate IDs we're generating
            if i < 3 {
                debug!(
                    "PNR '{}' has these additional formats for lookup: {:?}",
                    pnr, all_ids
                );
            }

            // Create realistic covariates with some randomization
            for id in &all_ids {
                // Generate common components reused for each date
                let family_size = 2 + rng.random_range(1..=5);                // Family size 3-7
                let municipality = 100 + rng.random_range(1..=100);           // Municipality
                let family_type = format!("{}", 1 + rng.random_range(1..=9)); // Family type
                
                let income_amount = 200000.0 + rng.random_range(0..800000) as f64;
                let education_level = rng.random_range(10..=30);
                
                // Generate ISCED level (1-8), with higher levels less common
                let isced_distribution: [(i32, f64); 8] = [
                    (1, 0.05),  // 5% at ISCED level 1
                    (2, 0.10),  // 10% at ISCED level 2
                    (3, 0.35),  // 35% at ISCED level 3
                    (4, 0.10),  // 10% at ISCED level 4
                    (5, 0.15),  // 15% at ISCED level 5
                    (6, 0.15),  // 15% at ISCED level 6
                    (7, 0.08),  // 8% at ISCED level 7
                    (8, 0.02),  // 2% at ISCED level 8
                ];
                
                // Weighted random selection for ISCED level
                let mut cdf = 0.0;
                let roll: f64 = rng.random();
                let isced = {
                    let mut selected = 3; // Default to level 3 if selection fails
                    for (level, weight) in &isced_distribution {
                        cdf += weight;
                        if roll <= cdf {
                            selected = *level;
                            break;
                        }
                    }
                    selected
                };
                
                // Generate education years based on ISCED level
                let education_years = 3.5 + (rng.random_range(0..10) as f32 / 2.0);
                
                // Create occupation covariates with SOCIO13 codes
                // Use values from the socio13.json mapping with weighted distribution
                let socio13_codes = [
                    ("110", 0.05), ("111", 0.02), ("112", 0.02), ("113", 0.03), ("114", 0.08), 
                    ("120", 0.01), ("131", 0.10), ("132", 0.15), ("133", 0.15), 
                    ("134", 0.20), ("135", 0.05), ("139", 0.02), 
                    ("210", 0.03), ("220", 0.02), ("310", 0.03), 
                    ("321", 0.01), ("322", 0.01), ("323", 0.01), ("330", 0.01)
                ];
                
                // Weighted random selection for SOCIO13 code
                let mut socio_cdf = 0.0;
                let socio_roll: f64 = rng.random();
                let socio13_code = {
                    let mut selected = "134"; // Default to employment at basic level
                    for (code, weight) in &socio13_codes {
                        socio_cdf += weight;
                        if socio_roll <= socio_cdf {
                            selected = code;
                            break;
                        }
                    }
                    selected
                };

                // Add to cache for different dates including treatment dates
                for date in &treatment_dates {
                    // Create fresh builders for each date
                    // Demographics
                    let demographic = Covariate::demographics(
                        family_size,
                        municipality,
                        family_type.clone(),
                    );
                    let key = CacheKey::new(id, CovariateType::Demographics, *date);
                    self.cache.insert(key, Some(demographic.build()));

                    // Income
                    let income = Covariate::income(
                        income_amount,
                        "DKK".to_string(),
                        "PERINDKIALT_13".to_string(),
                    );
                    let key = CacheKey::new(id, CovariateType::Income, *date);
                    self.cache.insert(key, Some(income.build()));

                    // Education
                    let education = Covariate::education(format!("{}", education_level))
                        .with_isced_code(format!("{}", isced))
                        .with_years(education_years);
                    let key = CacheKey::new(id, CovariateType::Education, *date);
                    self.cache.insert(key, Some(education.build()));
                    
                    // Occupation (SOCIO13)
                    let occupation = Covariate::occupation(
                        socio13_code.to_string(),
                        "SOCIO13".to_string(),
                    );
                    let key = CacheKey::new(id, CovariateType::Occupation, *date);
                    self.cache.insert(key, Some(occupation.build()));

                    // Add more date coverage to increase chance of hits
                    // Generate data for each quarter of each year from 2008 to 2023
                    for year in 2008..=2023 {
                        for &month in &[3, 6, 9, 12] {
                            if let Some(extra_date) = NaiveDate::from_ymd_opt(year, month, 15) {
                                if year != date.year() || month != date.month() {
                                    // Add entries for quarterly snapshots - create fresh builders each time
                                    let quarterly_demographic = Covariate::demographics(
                                        family_size, 
                                        municipality, 
                                        family_type.clone()
                                    );
                                    let key = CacheKey::new(id, CovariateType::Demographics, extra_date);
                                    self.cache.insert(key, Some(quarterly_demographic.build()));

                                    let quarterly_income = Covariate::income(
                                        income_amount,
                                        "DKK".to_string(),
                                        "PERINDKIALT_13".to_string(),
                                    );
                                    let key = CacheKey::new(id, CovariateType::Income, extra_date);
                                    self.cache.insert(key, Some(quarterly_income.build()));

                                    let quarterly_education = Covariate::education(format!("{}", education_level))
                                        .with_isced_code(format!("{}", isced))
                                        .with_years(education_years);
                                    let key = CacheKey::new(id, CovariateType::Education, extra_date);
                                    self.cache.insert(key, Some(quarterly_education.build()));
                                    
                                    let quarterly_occupation = Covariate::occupation(
                                        socio13_code.to_string(),
                                        "SOCIO13".to_string(),
                                    );
                                    let key = CacheKey::new(id, CovariateType::Occupation, extra_date);
                                    self.cache.insert(key, Some(quarterly_occupation.build()));
                                }
                            }
                        }
                    }
                }
            }

            // Print progress for large sets
            if i > 0 && i % 5000 == 0 {
                info!("Populated {} of {} PNRs in diagnostic cache", i, pnrs.len());
            }
        }

        // Add some deliberate missing values - around a smaller 2% of entries
        let missing_count = (pnrs.len() * 2) / 100;
        for i in 0..missing_count {
            if i < pnrs.len() {
                let key = CacheKey::new(&pnrs[i], CovariateType::Education, treatment_dates[0]);
                self.cache.insert(key, None);
            }
        }

        info!(
            "Diagnostic cache populated with {} entries for {} real PNRs",
            self.cache.len(),
            pnrs.len()
        );
        info!(
            "Average entries per PNR: {:.1}",
            if !pnrs.is_empty() {
                self.cache.len() as f64 / pnrs.len() as f64
            } else {
                0.0
            }
        );
    }

    pub fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        let key = CacheKey::new(pnr, covariate_type, date);
        self.cache.get_or_load(&*self.store, key)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to get covariate for PNR {}: {}", pnr, e)))
    }

    /// Prefetch data for multiple PNRs and covariates to improve performance
    pub fn prefetch_data(
        &self,
        pnrs: &[String],
        covariate_types: &[CovariateType],
        dates: &[NaiveDate],
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
        

        match self
            .cache
            .bulk_load(&*self.store, pnrs, covariate_types, dates)
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

    pub fn calculate_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<BalanceResults, IdsError> {
        debug!(
            "Starting balance calculation for {} cases and {} controls",
            cases.len(),
            controls.len()
        );

        let multi_progress = MultiProgress::new();
        let overall_style = ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap();

        let overall_pb = multi_progress.add(ProgressBar::new(3)); // 3 steps: demographics, income, education
        overall_pb.set_style(overall_style);
        overall_pb.set_message("Calculating balance...");

        let mut results = BalanceResults::new();

        // Calculate overall balance
        self.add_all_balances(&mut results, cases, controls, &overall_pb)?;

        // Calculate matched pair details
        overall_pb.set_message("Processing matched pairs...");
        self.add_matched_pair_details(&mut results, cases, controls)?;
        overall_pb.finish_with_message("Balance calculation complete");

        self.log_balance_statistics(&results);
        Ok(results)
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    pub fn get_variable_summary(&self, variable: &str) -> Option<&CovariateSummary> {
        self.results
            .as_ref()
            .and_then(|r| r.summaries.iter().find(|s| s.variable == variable))
    }

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

impl BalanceChecker {
    /// Calculate balance metrics for all covariate types
    fn add_all_balances(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        overall_pb: &ProgressBar,
    ) -> Result<(), IdsError> {
        // Update progress bar to account for occupation processing
        let total_steps = 4; // demographics, income, education, occupation
        overall_pb.set_length(total_steps);
        
        overall_pb.set_message("Processing demographics...");
        self.calculate_demographic_balance(results, cases, controls)?;
        overall_pb.inc(1);

        overall_pb.set_message("Processing income...");
        self.calculate_income_balance(results, cases, controls)?;
        overall_pb.inc(1);

        overall_pb.set_message("Processing education...");
        self.calculate_education_balance(results, cases, controls)?;
        overall_pb.inc(1);
        
        overall_pb.set_message("Processing occupation...");
        self.calculate_occupation_balance(results, cases, controls)?;
        overall_pb.inc(1);

        Ok(())
    }
    
    /// Analyze cache performance and provide detailed metrics
    pub fn analyze_cache_performance(&self) -> CachePerformanceMetrics {
        CachePerformanceMetrics {
            total_entries: self.cache.len(),
            hit_ratio: 0.0, // Would need to track hits/misses for this
            memory_usage: self.estimate_memory_usage(),
            access_pattern: "Unknown".to_string(),
        }
    }
    
    /// Estimate memory usage of the cached data
    fn estimate_memory_usage(&self) -> usize {
        // Rough estimate: average covariate size * number of covariates
        // Plus overhead for cache structures
        const AVG_COVARIATE_SIZE: usize = 256; // bytes
        const CACHE_OVERHEAD: usize = 50; // bytes per entry
        
        self.cache.len() * (AVG_COVARIATE_SIZE + CACHE_OVERHEAD)
    }
    
    /// Improved logging for balance analysis diagnostic information
    pub fn log_diagnostic_information(&self) {
        debug!("Balance checker diagnostic information:");
        debug!("Cache size: {} entries", self.cache.len());
        debug!("Estimated memory usage: {} bytes", self.estimate_memory_usage());
        
        if let Some(results) = &self.results {
            debug!("Results summary: {} variables analyzed", results.summaries.len());
            
            // Log top 5 variables with highest standardized differences
            let mut sorted_summaries: Vec<_> = results.summaries.iter().collect();
            sorted_summaries.sort_by(|a, b| b.std_diff.abs().partial_cmp(&a.std_diff.abs()).unwrap());
            
            if !sorted_summaries.is_empty() {
                debug!("Top variables with highest imbalance:");
                for (i, summary) in sorted_summaries.iter().take(5).enumerate() {
                    debug!(
                        "  {}. {}: std_diff = {:.3}, case_mean = {:.2}, control_mean = {:.2}",
                        i + 1, summary.variable, summary.std_diff, summary.mean_cases, summary.mean_controls
                    );
                }
            }
        }
    }
    
    /// Process data in parallel with optimal chunk sizing and workload balancing
    fn process_data_in_parallel<T, R, F>(
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
        let items_per_thread = data.len().div_ceil(num_threads);
        let chunk_size = items_per_thread.max(1).min(1000); // At least 1, at most 1000
        
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
    fn get_or_compute<K, V, F>(&self, _key: K, compute_fn: F) -> Result<V, IdsError>
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
    

    
    fn calculate_demographic_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // Original variables
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Family Size",
            |covariate| covariate.get_family_size().map(|val| val as f64),
        )?;
        results.add_summary(summary);
        results.add_missing_rate("Family Size".to_string(), missing_rates.0, missing_rates.1);

        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Municipality",
            |covariate| covariate.get_municipality().map(|val| val as f64),
        )?;
        results.add_summary(summary);
        results.add_missing_rate("Municipality".to_string(), missing_rates.0, missing_rates.1);

        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Family Type",
            |covariate| covariate.get_family_type().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Family Type".to_string(), missing_rates.0, missing_rates.1);

        // New variables from TROUBLE.md (BEF register)
        
        // Civil status (CIVST) - Categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Civil Status",
            |covariate| covariate.get_civil_status().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Civil Status".to_string(), missing_rates.0, missing_rates.1);
        
        // Gender (KOEN) - Categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Gender",
            |covariate| covariate.get_gender().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Gender".to_string(), missing_rates.0, missing_rates.1);
        
        // Citizenship (STATSB) - Categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Citizenship",
            |covariate| covariate.get_citizenship().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Citizenship".to_string(), missing_rates.0, missing_rates.1);
        
        // Age (ALDER) - Numeric
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Age",
            |covariate| covariate.get_age().map(|val| val as f64),
        )?;
        results.add_summary(summary);
        results.add_missing_rate("Age".to_string(), missing_rates.0, missing_rates.1);
        
        // Children count (ANTBOERNF/ANTBOERNH) - Numeric
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Demographics,
            "Children Count",
            |covariate| covariate.get_children_count().map(|val| val as f64),
        )?;
        results.add_summary(summary);
        results.add_missing_rate("Children Count".to_string(), missing_rates.0, missing_rates.1);

        Ok(())
    }

    fn calculate_income_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // Original income variable
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Income,
            "Income",
            |covariate| covariate.get_income_amount(),
        )?;

        results.add_summary(summary);
        results.add_missing_rate("Income".to_string(), missing_rates.0, missing_rates.1);

        // New variables from TROUBLE.md (IND register)
        
        // Wage income (LOENMV_13) - Numeric
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Income,
            "Wage Income",
            |covariate| covariate.get_wage_income(),
        )?;

        results.add_summary(summary);
        results.add_missing_rate("Wage Income".to_string(), missing_rates.0, missing_rates.1);
        
        // Employment status (BESKST13) - Numeric categorical
        let (summary, missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Income,
            "Employment Status",
            |covariate| covariate.get_employment_status().map(|val| val as f64),
        )?;

        results.add_summary(summary);
        results.add_missing_rate("Employment Status".to_string(), missing_rates.0, missing_rates.1);
        
        // Also add as categorical for better representation
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Income,
            "Employment Status Category",
            |covariate| covariate.get_employment_status().map(|val| val.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Employment Status Category".to_string(), missing_rates.0, missing_rates.1);

        Ok(())
    }

    fn calculate_education_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // 1. Process education levels as categorical variables
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Education,
            "Education Level",
            |covariate| covariate.get_education_level().map(|s| s.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "Education Level".to_string(),
            missing_rates.0,
            missing_rates.1,
        );
        
        // 2. Process ISCED codes as a separate categorical variable
        // Only if ISCED codes are available in the data
        let (isced_summaries, isced_missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Education,
            "ISCED Level",
            |covariate| covariate.get_isced_code().map(|s| s.to_string()),
        )?;

        for summary in isced_summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "ISCED Level".to_string(),
            isced_missing_rates.0,
            isced_missing_rates.1,
        );
        
        // 3. Process education years as a numeric variable (if available)
        let (years_summary, years_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Education,
            "Education Years",
            |covariate| covariate.get_education_years().map(|y| y as f64),
        )?;
        
        results.add_summary(years_summary);
        results.add_missing_rate(
            "Education Years".to_string(),
            years_missing_rates.0,
            years_missing_rates.1,
        );

        Ok(())
    }
    
    fn calculate_occupation_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // 1. Process SOCIO13 codes as categorical variables
        let (code_summaries, code_missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO13 Code",
            |covariate| covariate.get_occupation_code().map(|s| s.to_string()),
        )?;

        for summary in code_summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "SOCIO13 Code".to_string(),
            code_missing_rates.0,
            code_missing_rates.1,
        );
        
        // 2. Process SOCIO13 codes as a numeric variable for standardized difference calculation
        let (socio_summary, socio_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO13 Value",
            |covariate| {
                covariate.get_occupation_code().clone()
                    .and_then(|code| code.parse::<f64>().ok())
            },
        )?;
        
        results.add_summary(socio_summary);
        results.add_missing_rate(
            "SOCIO13 Value".to_string(),
            socio_missing_rates.0,
            socio_missing_rates.1,
        );
        
        // 3. Process occupation classification system as a separate categorical variable
        // This might be used for different versions or systems (DISCO, ISCO, etc.)
        let (class_summaries, class_missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "Classification System",
            |covariate| covariate.get_classification().map(|s| s.to_string()),
        )?;

        for summary in class_summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "Classification System".to_string(),
            class_missing_rates.0,
            class_missing_rates.1,
        );
        
        // New variables from TROUBLE.md (AKM register)
        
        // SOCIO - older socioeconomic classification
        let (socio_summary, socio_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO",
            |covariate| covariate.get_socio().map(|val| val as f64),
        )?;
        
        results.add_summary(socio_summary);
        results.add_missing_rate(
            "SOCIO".to_string(),
            socio_missing_rates.0,
            socio_missing_rates.1,
        );
        
        // Also as categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO Category",
            |covariate| covariate.get_socio().map(|val| val.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "SOCIO Category".to_string(),
            missing_rates.0,
            missing_rates.1,
        );
        
        // SOCIO02 - another socioeconomic classification
        let (socio02_summary, socio02_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO02",
            |covariate| covariate.get_socio02().map(|val| val as f64),
        )?;
        
        results.add_summary(socio02_summary);
        results.add_missing_rate(
            "SOCIO02".to_string(),
            socio02_missing_rates.0,
            socio02_missing_rates.1,
        );
        
        // Also as categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "SOCIO02 Category",
            |covariate| covariate.get_socio02().map(|val| val.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "SOCIO02 Category".to_string(),
            missing_rates.0,
            missing_rates.1,
        );
        
        // PRE_SOCIO - previous socioeconomic status
        let (pre_socio_summary, pre_socio_missing_rates) = self.metrics.calculate_numeric_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "Previous Socioeconomic Status",
            |covariate| covariate.get_pre_socio().map(|val| val as f64),
        )?;
        
        results.add_summary(pre_socio_summary);
        results.add_missing_rate(
            "Previous Socioeconomic Status".to_string(),
            pre_socio_missing_rates.0,
            pre_socio_missing_rates.1,
        );
        
        // Also as categorical
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Occupation,
            "Previous Socioeconomic Category",
            |covariate| covariate.get_pre_socio().map(|val| val.to_string()),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "Previous Socioeconomic Category".to_string(),
            missing_rates.0,
            missing_rates.1,
        );

        Ok(())
    }

    fn add_matched_pair_details(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        use rayon::prelude::*;
        use parking_lot::Mutex;
        use std::sync::Arc;
        
        // Group cases by date for better batch processing
        let mut cases_by_date: HashMap<NaiveDate, Vec<&str>> = HashMap::new();
        for (case_pnr, case_date) in cases {
            cases_by_date.entry(*case_date).or_default().push(case_pnr);
        }
        
        // Do the same for controls
        let mut controls_by_date: HashMap<NaiveDate, Vec<&str>> = HashMap::new();
        for (control_pnr, control_date) in controls {
            controls_by_date.entry(*control_date).or_default().push(control_pnr);
        }
        
        // Determine optimal chunk size based on number of pairs
        let total_pairs: usize = cases_by_date.iter()
            .map(|(date, case_pnrs)| {
                let control_count = controls_by_date.get(date).map_or(0, |c| c.len());
                case_pnrs.len() * control_count
            })
            .sum();
            
        let num_threads = rayon::current_num_threads();
        let chunk_size = (total_pairs / num_threads).clamp(100, 5000);
        
        log::debug!(
            "Processing {} matched pairs for {} cases and {} controls using chunk size {}", 
            total_pairs, cases.len(), controls.len(), chunk_size
        );
        
        // Use a thread-safe container for collecting results
        let pair_details = Arc::new(Mutex::new(Vec::with_capacity(total_pairs * 4)));
        
        // Define the variables we'll use for prefetching - include Occupation
        let covariate_types = [
            CovariateType::Demographics,
            CovariateType::Income, 
            CovariateType::Education,
            CovariateType::Occupation,
        ];
        
        // Process each date group in parallel
        cases_by_date.par_iter().for_each(|(date, case_pnrs)| {
            // Get matching controls for this date
            let control_pnrs = match controls_by_date.get(date) {
                Some(pnrs) => pnrs,
                None => return, // No controls for this date
            };
            
            // For large enough groups, prefetch all the data we'll need
            if case_pnrs.len() * control_pnrs.len() > 100 {
                // Collect all PNRs for prefetching (both cases and controls)
                let mut all_pnrs = Vec::with_capacity(case_pnrs.len() + control_pnrs.len());
                all_pnrs.extend(case_pnrs.iter().map(|p| p.to_string()));
                all_pnrs.extend(control_pnrs.iter().map(|p| p.to_string()));
                
                // Prefetch all data for this date group
                self.prefetch_data(&all_pnrs, &covariate_types, &[*date]);
            }
            
            // Process each case-control pair
            for case_pnr in case_pnrs {
                for control_pnr in control_pnrs {
                    let mut batch_details = Vec::new();
                    
                    // --- DEMOGRAPHICS ---
                    
                    // Original variables
                    
                    // Family Size
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Demographics,
                        "Family Size",
                        |cov| cov.get_family_size().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // Municipality
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Demographics,
                        "Municipality",
                        |cov| cov.get_municipality().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // New demographics variables
                    
                    // Age
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Demographics,
                        "Age",
                        |cov| cov.get_age().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // Children Count
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Demographics,
                        "Children Count",
                        |cov| cov.get_children_count().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // --- INCOME ---
                    
                    // Original income variable
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Income,
                        "Income",
                        |cov| cov.get_income_amount(),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // New income variables
                    
                    // Wage Income
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Income,
                        "Wage Income",
                        |cov| cov.get_wage_income(),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // Employment Status
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Income,
                        "Employment Status",
                        |cov| cov.get_employment_status().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // --- EDUCATION ---
                    
                    // Education Level - treated as a numeric value 
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Education,
                        "Education Level",
                        |cov| {
                            cov.get_education_level()
                                .and_then(|level| level.parse::<f64>().ok())
                        },
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // ISCED Level - convert from string code to numeric value for comparison
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Education,
                        "ISCED Level",
                        |cov| {
                            cov.get_isced_code()
                                .and_then(|code| {
                                    // Extract the first character which should be the ISCED level
                                    if !code.is_empty() {
                                        code[0..1].parse::<f64>().ok()
                                    } else {
                                        None
                                    }
                                })
                        },
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // Education Years - already a numeric value
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Education,
                        "Education Years",
                        |cov| cov.get_education_years().map(|y| y as f64),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // --- OCCUPATION ---
                    
                    // Original occupation variables
                    
                    // SOCIO13 Occupation Code - convert directly to numeric 
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "SOCIO13 Value",
                        |cov| {
                            cov.get_occupation_code()
                                .and_then(|code| code.parse::<f64>().ok())
                        },
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // Classification System - treat as categorical but convert to numeric
                    // This is retained for compatibility with any non-SOCIO13 classification systems
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "Classification System",
                        |cov| {
                            cov.get_classification().map(|class| {
                                // Simple hash to create a numeric value for comparison
                                let mut hash = 0.0;
                                for (i, c) in class.chars().enumerate() {
                                    hash += (c as u32 as f64) * (i + 1) as f64;
                                }
                                hash
                            })
                        },
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // New occupation variables
                    
                    // SOCIO
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "SOCIO",
                        |cov| cov.get_socio().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // SOCIO02
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "SOCIO02",
                        |cov| cov.get_socio02().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // PRE_SOCIO
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "Previous Socioeconomic Status",
                        |cov| cov.get_pre_socio().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }
                    
                    // Add all details at once to minimize lock contention
                    if !batch_details.is_empty() {
                        let mut details = pair_details.lock();
                        details.extend(batch_details);
                    }
                }
            }
        });
        
        // Add all collected pair details to the results
        let collected_details = match Arc::try_unwrap(pair_details) {
            Ok(mutex) => mutex.into_inner(),
            Err(arc) => {
                let guard = arc.lock();
                guard.clone()
            }
        };
            
        log::debug!("Collected {} matched pair details", collected_details.len());
        
        for detail in collected_details {
            results.add_pair_detail(detail);
        }

        Ok(())
    }

    fn process_matched_pair(
        &self,
        case_pnr: &str,
        control_pnr: &str,
        date: NaiveDate,
        covariate_type: CovariateType,
        variable_name: &str,
        value_extractor: impl Fn(&Covariate) -> Option<f64>,
    ) -> Result<Option<MatchedPairDetail>, IdsError> {
        let case_value = self
            .get_covariate(case_pnr, covariate_type, date)?
            .as_ref()
            .and_then(&value_extractor);

        let control_value = self
            .get_covariate(control_pnr, covariate_type, date)?
            .as_ref()
            .and_then(&value_extractor);

        match (case_value, control_value) {
            (Some(case_val), Some(ctrl_val)) => Ok(Some(MatchedPairDetail::new(
                case_pnr.to_string(),
                vec![control_pnr.to_string()],
                date,
                variable_name.to_string(),
                case_val,
                ctrl_val,
                MatchedPairDetail::calculate_std_diff(case_val, ctrl_val),
            ))),
            _ => Ok(None),
        }
    }

    fn log_balance_statistics(&self, results: &BalanceResults) {
        debug!("Balance calculation completed:");
        debug!("Total summaries: {}", results.summaries.len());
        debug!(
            "Total matched pair details: {}",
            results.matched_pair_details.len()
        );

        for summary in &results.summaries {
            if summary.std_diff.abs() > 0.1 {
                debug!(
                    "Large imbalance detected for {}: std_diff = {:.3}",
                    summary.variable, summary.std_diff
                );
            }
        }
    }
}

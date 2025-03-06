use chrono::{Datelike, NaiveDate};
use covariates::balance::BalanceChecker;
use log::{debug, info};
use rand::Rng;
use std::sync::Arc;
use types::models::{Covariate, CovariateType};

/// Performance metrics for cache analysis
#[derive(Debug, Clone)]
pub struct CachePerformanceMetrics {
    pub total_entries: usize,
    pub hit_ratio: f64,
    pub memory_usage: usize,
    pub access_pattern: String,
}

/// Extension trait for BalanceChecker that provides diagnostic capabilities for testing
pub trait BalanceCheckerDiagnostics {
    /// Create a new checker with an empty store (for diagnostic mode)
    fn new_diagnostic() -> BalanceChecker;

    /// Create a new diagnostic checker using actual PNRs from matched pairs
    fn new_diagnostic_with_pnrs(pnrs: Vec<String>) -> BalanceChecker;

    /// Analyze cache performance and provide detailed metrics
    fn analyze_cache_performance(&self) -> CachePerformanceMetrics;

    /// Improved logging for balance analysis diagnostic information
    fn log_diagnostic_information(&self);
}

impl BalanceCheckerDiagnostics for BalanceChecker {
    /// Create a new checker with an empty store (for diagnostic mode)
    fn new_diagnostic() -> Self {
        let empty_store = types::storage::arrow::backend::ArrowBackend::new_empty();

        // Create a checker with an empty store but with a cache that simulates having some data
        // Create a checker builder
        let checker = BalanceChecker::builder()
            .with_store(empty_store)
            .with_cache_size(1000)
            .build();

        // Populate the cache with some placeholder data for diagnostic purposes
        populate_diagnostic_cache(&checker);

        checker
    }

    /// Create a new diagnostic checker using actual PNRs from matched pairs
    fn new_diagnostic_with_pnrs(pnrs: Vec<String>) -> Self {
        let empty_store = types::storage::arrow::backend::ArrowBackend::new_empty();

        // Create a checker with an empty store but with a cache that simulates having some data
        let checker = Self {
            store: Arc::new(empty_store),
            cache: covariates::balance::legacy_cache::CovariateCache::new(100_000), // Larger cache for real PNRs
            metrics: covariates::balance::metrics::BalanceMetrics::new(),
            results: None,
        };

        // Populate the cache with data using the actual PNRs from matched pairs
        populate_diagnostic_cache_with_pnrs(&checker, pnrs);

        checker
    }

    /// Analyze cache performance and provide detailed metrics
    fn analyze_cache_performance(&self) -> CachePerformanceMetrics {
        CachePerformanceMetrics {
            total_entries: self.cache.len(),
            hit_ratio: 0.0, // Would need to track hits/misses for this
            memory_usage: estimate_memory_usage(self),
            access_pattern: "Unknown".to_string(),
        }
    }

    /// Improved logging for balance analysis diagnostic information
    fn log_diagnostic_information(&self) {
        debug!("Balance checker diagnostic information:");
        debug!("Cache size: {} entries", self.cache.len());
        debug!(
            "Estimated memory usage: {} bytes",
            estimate_memory_usage(self)
        );

        if let Some(results) = &self.results {
            debug!(
                "Results summary: {} variables analyzed",
                results.summaries.len()
            );

            // Log top 5 variables with highest standardized differences
            let mut sorted_summaries: Vec<_> = results.summaries.iter().collect();
            sorted_summaries.sort_by(|a, b| {
                b.std_diff
                    .abs()
                    .partial_cmp(&a.std_diff.abs())
                    .unwrap_or(std::cmp::Ordering::Equal)
            });

            if !sorted_summaries.is_empty() {
                debug!("Top variables with highest imbalance:");
                for (i, summary) in sorted_summaries.iter().take(5).enumerate() {
                    debug!(
                        "  {}. {}: std_diff = {:.3}, case_mean = {:.2}, control_mean = {:.2}",
                        i + 1,
                        summary.variable,
                        summary.std_diff,
                        summary.mean_cases,
                        summary.mean_controls
                    );
                }
            }
        }
    }
}

/// Estimate memory usage of the cached data
fn estimate_memory_usage(checker: &BalanceChecker) -> usize {
    // Rough estimate: average covariate size * number of covariates
    // Plus overhead for cache structures
    const AVG_COVARIATE_SIZE: usize = 256; // bytes
    const CACHE_OVERHEAD: usize = 50; // bytes per entry

    checker.cache.len() * (AVG_COVARIATE_SIZE + CACHE_OVERHEAD)
}

/// Populate the cache with some placeholder data for diagnostic purposes using standard test data
fn populate_diagnostic_cache(checker: &BalanceChecker) {
    use covariates::balance::legacy_cache::CacheKey;
    use log::debug;

    // Get real treatment dates to use
    let treatment_dates = [
        NaiveDate::from_ymd_opt(2008, 1, 30)
            .expect("Invalid date 2008-01-30 - this is a static date that should never fail"),
        NaiveDate::from_ymd_opt(2010, 5, 15)
            .expect("Invalid date 2010-05-15 - this is a static date that should never fail"),
        NaiveDate::from_ymd_opt(2015, 10, 20)
            .expect("Invalid date 2015-10-20 - this is a static date that should never fail"),
        NaiveDate::from_ymd_opt(2020, 3, 10)
            .expect("Invalid date 2020-03-10 - this is a static date that should never fail"),
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
            let family_size = 3 + (i % 5) as i32; // Family size 3-7
            let municipality = 101 + (i % 100) as i32; // Municipality
            let family_type = format!("{}", 1 + (i % 9)); // Family type
            let civil_status = format!("{}", "G".chars().nth(i % 4).unwrap_or('G'));
            let gender = if i % 2 == 0 {
                "M".to_string()
            } else {
                "K".to_string()
            };
            let citizenship = "DK".to_string();
            let age = 20 + (i % 80) as i32;
            let children_count = (i % 4) as i32;

            // Income data
            let income_amount = 250000.0 + (i as f64 * 1000.0);
            let wage_income = 200000.0 + (i as f64 * 800.0); // LOENMV_13
            let employment_status = 1 + (i % 5) as i32; // BESKST13

            // Education data
            let edu_level = 10 + (i % 20);
            let isced_level = 1 + (i % 8);
            let edu_years = 3.5 + (i % 10) as f32 / 2.0;

            // Occupation data
            let socio13_codes = [
                "110", "111", "112", "113", "114", "120", "131", "132", "133", "134", "135", "139",
                "210", "220", "310", "321", "322", "323", "330",
            ];
            let socio13_code = socio13_codes[i % socio13_codes.len()];
            let socio = 100 + (i % 50) as i32; // SOCIO
            let socio02 = 200 + (i % 30) as i32; // SOCIO02
            let pre_socio = 10 + (i % 20) as i32; // PRE_SOCIO

            // Add to cache for different dates including treatment dates
            for date in &treatment_dates {
                // Create fresh builder instances for each date iteration

                // Demographics
                let demographic =
                    Covariate::demographics(family_size, municipality, family_type.clone())
                        .with_civil_status(civil_status.clone())
                        .with_gender(gender.clone())
                        .with_citizenship(citizenship.clone())
                        .with_age(age)
                        .with_children_count(children_count);
                let key = CacheKey::new(id, CovariateType::Demographics, *date);
                checker.cache.insert(key, Some(demographic.build()));

                // Income
                let income = Covariate::income(
                    income_amount,
                    "DKK".to_string(),
                    "PERINDKIALT_13".to_string(),
                )
                .with_wage_income(wage_income)
                .with_employment_status(employment_status);
                let key = CacheKey::new(id, CovariateType::Income, *date);
                checker.cache.insert(key, Some(income.build()));

                // Education
                let education = Covariate::education(format!("{}", edu_level))
                    .with_isced_code(format!("{}", isced_level))
                    .with_years(edu_years);
                let key = CacheKey::new(id, CovariateType::Education, *date);
                checker.cache.insert(key, Some(education.build()));

                // Occupation
                let occupation =
                    Covariate::occupation(socio13_code.to_string(), "SOCIO13".to_string())
                        .with_socio(socio)
                        .with_socio02(socio02)
                        .with_pre_socio(pre_socio);
                let key = CacheKey::new(id, CovariateType::Occupation, *date);
                checker.cache.insert(key, Some(occupation.build()));
            }
        }
    }

    // Add some deliberate missing values - around 10%
    for i in 0..100 {
        let case_id = format!("C{:06}", i);
        let key = CacheKey::new(&case_id, CovariateType::Education, treatment_dates[0]);
        checker.cache.insert(key, None);
    }

    debug!(
        "Diagnostic cache populated with {} entries",
        checker.cache.len()
    );
}

/// Populate the cache with data using actual PNRs from matched pairs
fn populate_diagnostic_cache_with_pnrs(checker: &BalanceChecker, pnrs: Vec<String>) {
    use covariates::balance::legacy_cache::CacheKey;

    info!(
        "Initializing diagnostic cache with {} real PNRs from matched pairs",
        pnrs.len()
    );

    // Get treatment dates to use - include more recent years to cover more cases
    let treatment_dates = [
        NaiveDate::from_ymd_opt(2008, 1, 30)
            .expect("Invalid date 2008-01-30 - this is a static date that should never fail"), // Common from matched pairs
        NaiveDate::from_ymd_opt(2010, 5, 15)
            .expect("Invalid date 2010-05-15 - this is a static date that should never fail"),
        NaiveDate::from_ymd_opt(2015, 10, 20)
            .expect("Invalid date 2015-10-20 - this is a static date that should never fail"),
        NaiveDate::from_ymd_opt(2020, 3, 10)
            .expect("Invalid date 2020-03-10 - this is a static date that should never fail"),
        NaiveDate::from_ymd_opt(2022, 6, 1)
            .expect("Invalid date 2022-06-01 - this is a static date that should never fail"), // More recent
        NaiveDate::from_ymd_opt(2023, 9, 15)
            .expect("Invalid date 2023-09-15 - this is a static date that should never fail"), // More recent
    ];

    let mut rng = rand::thread_rng();

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
            let family_size = 2 + rng.gen_range(1..=5); // Family size 3-7
            let municipality = 100 + rng.gen_range(1..=100); // Municipality
            let family_type = format!("{}", 1 + rng.gen_range(1..=9)); // Family type

            let income_amount = 200000.0 + rng.gen_range(0.0..800000.0);
            let education_level = rng.gen_range(10..=30);

            // Generate ISCED level (1-8), with higher levels less common
            let isced_distribution: [(i32, f64); 8] = [
                (1, 0.05), // 5% at ISCED level 1
                (2, 0.10), // 10% at ISCED level 2
                (3, 0.35), // 35% at ISCED level 3
                (4, 0.10), // 10% at ISCED level 4
                (5, 0.15), // 15% at ISCED level 5
                (6, 0.15), // 15% at ISCED level 6
                (7, 0.08), // 8% at ISCED level 7
                (8, 0.02), // 2% at ISCED level 8
            ];

            // Weighted random selection for ISCED level
            let mut cdf = 0.0;
            let roll: f64 = rng.gen_range(0.0..1.0);
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
            let education_years = 3.5 + (rng.gen_range(0.0..5.0) as f32);

            // Create occupation covariates with SOCIO13 codes
            // Use values from the socio13.json mapping with weighted distribution
            let socio13_codes = [
                ("110", 0.05),
                ("111", 0.02),
                ("112", 0.02),
                ("113", 0.03),
                ("114", 0.08),
                ("120", 0.01),
                ("131", 0.10),
                ("132", 0.15),
                ("133", 0.15),
                ("134", 0.20),
                ("135", 0.05),
                ("139", 0.02),
                ("210", 0.03),
                ("220", 0.02),
                ("310", 0.03),
                ("321", 0.01),
                ("322", 0.01),
                ("323", 0.01),
                ("330", 0.01),
            ];

            // Weighted random selection for SOCIO13 code
            let mut socio_cdf = 0.0;
            let socio_roll: f64 = rng.gen_range(0.0..1.0);
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
                let demographic =
                    Covariate::demographics(family_size, municipality, family_type.clone());
                let key = CacheKey::new(id, CovariateType::Demographics, *date);
                checker.cache.insert(key, Some(demographic.build()));

                // Income
                let income = Covariate::income(
                    income_amount,
                    "DKK".to_string(),
                    "PERINDKIALT_13".to_string(),
                );
                let key = CacheKey::new(id, CovariateType::Income, *date);
                checker.cache.insert(key, Some(income.build()));

                // Education
                let education = Covariate::education(format!("{}", education_level))
                    .with_isced_code(format!("{}", isced))
                    .with_years(education_years);
                let key = CacheKey::new(id, CovariateType::Education, *date);
                checker.cache.insert(key, Some(education.build()));

                // Occupation (SOCIO13)
                let occupation =
                    Covariate::occupation(socio13_code.to_string(), "SOCIO13".to_string());
                let key = CacheKey::new(id, CovariateType::Occupation, *date);
                checker.cache.insert(key, Some(occupation.build()));

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
                                    family_type.clone(),
                                );
                                let key =
                                    CacheKey::new(id, CovariateType::Demographics, extra_date);
                                checker
                                    .cache
                                    .insert(key, Some(quarterly_demographic.build()));

                                let quarterly_income = Covariate::income(
                                    income_amount,
                                    "DKK".to_string(),
                                    "PERINDKIALT_13".to_string(),
                                );
                                let key = CacheKey::new(id, CovariateType::Income, extra_date);
                                checker.cache.insert(key, Some(quarterly_income.build()));

                                let quarterly_education =
                                    Covariate::education(format!("{}", education_level))
                                        .with_isced_code(format!("{}", isced))
                                        .with_years(education_years);
                                let key = CacheKey::new(id, CovariateType::Education, extra_date);
                                checker.cache.insert(key, Some(quarterly_education.build()));

                                let quarterly_occupation = Covariate::occupation(
                                    socio13_code.to_string(),
                                    "SOCIO13".to_string(),
                                );
                                let key = CacheKey::new(id, CovariateType::Occupation, extra_date);
                                checker
                                    .cache
                                    .insert(key, Some(quarterly_occupation.build()));
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
            checker.cache.insert(key, None);
        }
    }

    info!(
        "Diagnostic cache populated with {} entries for {} real PNRs",
        checker.cache.len(),
        pnrs.len()
    );
    info!(
        "Average entries per PNR: {:.1}",
        if !pnrs.is_empty() {
            checker.cache.len() as f64 / pnrs.len() as f64
        } else {
            0.0
        }
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_diagnostic_checker() {
        let checker = BalanceChecker::new_diagnostic();
        assert!(
            checker.cache.len() > 0,
            "Diagnostic cache should be populated"
        );
    }

    #[test]
    fn test_create_diagnostic_with_pnrs() {
        let pnrs = vec![
            "010101-1234".to_string(),
            "020202-5678".to_string(),
            "030303-9012".to_string(),
        ];
        let checker = BalanceChecker::new_diagnostic_with_pnrs(pnrs);
        assert!(
            checker.cache.len() > 0,
            "Diagnostic cache should be populated with PNRs"
        );
    }

    #[test]
    fn test_analyze_performance() {
        let checker = BalanceChecker::new_diagnostic();
        let metrics = checker.analyze_cache_performance();
        assert!(metrics.total_entries > 0, "Should have cache entries");
        assert!(
            metrics.memory_usage > 0,
            "Should have estimated memory usage"
        );
    }
}

use super::checker::BalanceChecker;
use super::processor::ValueProcessor;
use super::stats::StatisticalCalculations;
use crate::models::CovariateSummary;
use chrono::NaiveDate;
use hashbrown::HashMap;
use types::{
    error::IdsError,
    models::{Covariate, CovariateType},
};

pub(crate) struct BalanceMetrics {
    processor: ValueProcessor,
}

impl BalanceMetrics {
    pub fn new() -> Self {
        Self {
            processor: ValueProcessor::new(),
        }
    }

    pub fn calculate_numeric_balance<F>(
        &self,
        checker: &BalanceChecker,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        name: &str,
        extractor: F,
    ) -> Result<(CovariateSummary, (f64, f64)), IdsError>
    where
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
    {
        // Print some diagnostic info about the input
        if !cases.is_empty() {
            log::debug!(
                "Sample cases: {} pairs, first case PNR: {}, date: {}", 
                cases.len(),
                cases[0].0,
                cases[0].1
            );
        }
        
        // Add debug log for cache size in the balance checker
        log::debug!("Balance checker cache size: {}", checker.cache_size());
        
        // Sample the first few cases to diagnose PNR format and matching issues
        for (i, (case_pnr, case_date)) in cases.iter().enumerate().take(5) {
            log::debug!("Checking case entry {}: PNR {} at date {}", i, case_pnr, case_date);
            
            // Try both original PNR and the C/K format just to be extra sure
            match checker.get_covariate(case_pnr, covariate_type, *case_date) {
                Ok(Some(_)) => {
                    // Found a value - this is good
                    log::debug!("✓ Successfully found covariate for case PNR: {}", case_pnr);
                }
                Ok(None) => {
                    // No value found - log with PNR format info for diagnosis
                    if case_pnr.starts_with('C') {
                        log::debug!("✗ No covariate found for C-format case PNR: {}", case_pnr);
                    } else {
                        log::debug!("✗ No covariate found for regular case PNR: {}", case_pnr);
                    }
                    
                    // If this is the first few, dump out diagnostic info about exactly what we're looking for
                    if i < 2 {
                        log::debug!("Dumping cache key details for debugging:");
                        log::debug!("PNR: '{}', Type: {:?}, Date: {}", case_pnr, covariate_type, case_date);
                        
                        // Also try searching for it in a different format to diagnose case sensitivity issues
                        let alternate_pnr = if case_pnr.contains('-') {
                            case_pnr.replace('-', "")
                        } else if case_pnr.len() > 6 {
                            format!("{}-{}", &case_pnr[0..6], &case_pnr[6..])
                        } else {
                            case_pnr.clone()
                        };
                        
                        if alternate_pnr != *case_pnr {
                            log::debug!("Also trying alternate PNR format: '{}'", alternate_pnr);
                            match checker.get_covariate(&alternate_pnr, covariate_type, *case_date) {
                                Ok(Some(_)) => log::debug!("✓ Found with alternate format!"),
                                _ => log::debug!("✗ Still not found with alternate format"),
                            }
                        }
                    }
                }
                Err(e) => {
                    // Error accessing covariate - this is unexpected
                    log::warn!("✗ Error accessing covariate for PNR {}: {}", case_pnr, e);
                }
            }
        }
        
        let (case_values, case_missing) =
            self.processor
                .collect_numeric_values(cases, covariate_type, checker, &extractor);
        let (control_values, control_missing) =
            self.processor
                .collect_numeric_values(controls, covariate_type, checker, &extractor);

        // Log the actual values found for debugging
        log::debug!(
            "Found {} numeric values for cases and {} for controls, missing {} and {} respectively", 
            case_values.len(), control_values.len(), case_missing, control_missing
        );
        
        let missing_rates = (
            case_missing as f64 / cases.len() as f64,
            control_missing as f64 / controls.len() as f64,
        );

        if case_values.is_empty() || control_values.is_empty() {
            return Ok((
                CovariateSummary {
                    variable: name.to_string(),
                    mean_cases: 0.0,
                    mean_controls: 0.0,
                    std_diff: 0.0,
                    variance_ratio: 1.0,
                },
                missing_rates,
            ));
        }

        // Calculate all statistics in one pass for each group
        let case_summary = StatisticalCalculations::calculate_summary(&case_values);
        let control_summary = StatisticalCalculations::calculate_summary(&control_values);

        // Use the pre-calculated statistics
        Ok((
            CovariateSummary {
                variable: name.to_string(),
                mean_cases: case_summary.mean,
                mean_controls: control_summary.mean,
                std_diff: StatisticalCalculations::calculate_standardized_difference_from_summaries(
                    &case_summary,
                    &control_summary,
                ),
                variance_ratio: StatisticalCalculations::calculate_variance_ratio_from_summaries(
                    &case_summary,
                    &control_summary,
                ),
            },
            missing_rates,
        ))
    }

    pub fn calculate_categorical_balance<F>(
        &self,
        checker: &BalanceChecker,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        name: &str,
        extractor: F,
    ) -> Result<(Vec<CovariateSummary>, (f64, f64)), IdsError>
    where
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        let (case_values, case_missing) =
            self.processor
                .collect_categorical_values(cases, covariate_type, checker, &extractor);
        let (control_values, control_missing) = self.processor.collect_categorical_values(
            controls,
            covariate_type,
            checker,
            &extractor,
        );

        let missing_rates = (
            case_missing as f64 / cases.len() as f64,
            control_missing as f64 / controls.len() as f64,
        );

        let mut summaries = Vec::new();

        // Calculate frequencies for each category
        let mut case_freqs = HashMap::new();
        let mut control_freqs = HashMap::new();

        for value in case_values {
            *case_freqs.entry(value).or_insert(0) += 1;
        }
        for value in control_values {
            *control_freqs.entry(value).or_insert(0) += 1;
        }

        // Calculate summary statistics for each category
        for (category, count) in &case_freqs {
            let case_prop = *count as f64 / cases.len() as f64;
            let control_prop = control_freqs
                .get(category)
                .map_or(0.0, |&count| count as f64 / controls.len() as f64);

            let std_diff = if case_prop == 0.0 && control_prop == 0.0 {
                0.0
            } else {
                (case_prop - control_prop)
                    / (case_prop.mul_add(1.0 - case_prop, control_prop * (1.0 - control_prop))
                        / 2.0)
                        .sqrt()
            };

            summaries.push(CovariateSummary {
                variable: format!("{name} - {category}"),
                mean_cases: case_prop,
                mean_controls: control_prop,
                std_diff,
                variance_ratio: 1.0, // Not applicable for categorical variables
            });
        }

        Ok((summaries, missing_rates))
    }
}

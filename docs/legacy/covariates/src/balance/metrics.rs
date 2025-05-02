use super::stats::StatisticalCalculations;
use super::BalanceChecker;
use crate::models::CovariateSummary;
use chrono::NaiveDate;
use log::{debug, info, warn};
use types::error::Result as IdsResult;
use types::models::{Covariate, CovariateType};

/// Main balance metrics calculator
pub struct BalanceMetrics {
    processor: super::processor::ValueProcessor,
}

impl BalanceMetrics {
    /// Create a new balance metrics calculator
    #[must_use] pub fn new() -> Self {
        Self {
            processor: super::processor::ValueProcessor::new(),
        }
    }

    /// Calculate balance metrics for a numeric variable
    pub fn calculate_numeric_balance(
        &self,
        checker: &BalanceChecker,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        variable_name: &str,
        extractor: impl Fn(&Covariate) -> Option<f64> + Send + Sync,
    ) -> IdsResult<(CovariateSummary, (f64, f64))> {
        // Log diagnostics for first call
        if variable_name == "Family Size" {
            self.log_diagnostics(cases, controls, checker, true);
        }

        debug!(
            "Calculating balance for numeric variable {variable_name} (type: {covariate_type:?})"
        );

        // Extract values
        let (case_values, case_missing) =
            self.processor
                .collect_numeric_values(cases, covariate_type, checker, &extractor);

        let (control_values, control_missing) =
            self.processor
                .collect_numeric_values(controls, covariate_type, checker, &extractor);

        // Calculate missing rates
        let case_missing_rate = self.calculate_missing_rate(case_missing, cases.len());
        let control_missing_rate = self.calculate_missing_rate(control_missing, controls.len());

        // Calculate statistics
        let case_stats = StatisticalCalculations::calculate_summary(&case_values);
        let control_stats = StatisticalCalculations::calculate_summary(&control_values);

        // Calculate standardized mean difference
        let std_diff = StatisticalCalculations::calculate_standardized_difference_from_summaries(
            &case_stats,
            &control_stats,
        );

        // Calculate variance ratio
        let variance_ratio = StatisticalCalculations::calculate_variance_ratio_from_summaries(
            &case_stats,
            &control_stats,
        );

        // Create summary
        let summary = CovariateSummary::new(
            variable_name.to_string(),
            case_stats.mean,
            control_stats.mean,
            std_diff,
            variance_ratio,
        );

        Ok((summary, (case_missing_rate, control_missing_rate)))
    }

    /// Calculate balance metrics for a categorical variable
    pub fn calculate_categorical_balance(
        &self,
        checker: &BalanceChecker,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        variable_name: &str,
        extractor: impl Fn(&Covariate) -> Option<String> + Send + Sync,
    ) -> IdsResult<(Vec<CovariateSummary>, (f64, f64))> {
        debug!(
            "Calculating balance for categorical variable {variable_name} (type: {covariate_type:?})"
        );

        // Extract values
        let (case_values, case_missing) =
            self.processor
                .collect_categorical_values(cases, covariate_type, checker, &extractor);

        let (control_values, control_missing) = self.processor.collect_categorical_values(
            controls,
            covariate_type,
            checker,
            &extractor,
        );

        // Calculate missing rates
        let case_missing_rate = self.calculate_missing_rate(case_missing, cases.len());
        let control_missing_rate = self.calculate_missing_rate(control_missing, controls.len());

        // Calculate category counts and proportions
        let case_counts = self.count_categories(&case_values);
        let control_counts = self.count_categories(&control_values);

        // Combine categories from both groups
        let all_categories: Vec<_> = case_counts
            .keys()
            .chain(control_counts.keys())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .cloned()
            .collect();

        // Calculate proportions and standardized differences for each category
        let mut results = Vec::new();
        for category in all_categories {
            let case_count = case_counts.get(&category).copied().unwrap_or(0);
            let case_prop = if case_values.is_empty() {
                0.0
            } else {
                case_count as f64 / case_values.len() as f64
            };

            let control_count = control_counts.get(&category).copied().unwrap_or(0);
            let control_prop = if control_values.is_empty() {
                0.0
            } else {
                control_count as f64 / control_values.len() as f64
            };

            // Calculate standardized difference for categorical variables
            let pooled_var =
                (case_prop * (1.0 - case_prop) + control_prop * (1.0 - control_prop)) / 2.0;
            let pooled_sd = pooled_var.sqrt();

            let std_diff = if pooled_sd == 0.0 {
                0.0
            } else {
                (case_prop - control_prop) / pooled_sd
            };

            // For categorical variables, variance ratio is set to 1.0
            let variance_ratio = 1.0;

            // Create summary for this category
            let category_name = if variable_name.is_empty() {
                category.clone()
            } else {
                format!("{variable_name}: {category}")
            };

            let summary = CovariateSummary::new(
                category_name,
                case_prop,
                control_prop,
                std_diff,
                variance_ratio,
            );

            results.push(summary);
        }

        Ok((results, (case_missing_rate, control_missing_rate)))
    }

    // Helper methods

    /// Log diagnostic information about the data
    fn log_diagnostics(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        _checker: &BalanceChecker,
        verbose: bool,
    ) {
        // Count unique cases and controls for logging
        let unique_cases: std::collections::HashSet<&str> =
            cases.iter().map(|(s, _)| s.as_str()).collect();
        let unique_controls: std::collections::HashSet<&str> =
            controls.iter().map(|(s, _)| s.as_str()).collect();

        // Log case & control counts
        info!(
            "Case-control balance: {} cases, {} controls ({:.1}x), {} unique cases, {} unique controls",
            cases.len(),
            controls.len(),
            controls.len() as f64 / cases.len() as f64,
            unique_cases.len(),
            unique_controls.len(),
        );

        if verbose {
            // Diagnostics for PNR formats
            let dash_count = cases
                .iter()
                .map(|(pnr, _)| pnr)
                .filter(|pnr| pnr.contains('-'))
                .count();

            if dash_count > 0 {
                warn!(
                    "{dash_count} PNRs contain dashes, which might affect matching"
                );
            }
        }
    }

    /// Calculate missing data rate
    fn calculate_missing_rate(&self, missing: usize, total: usize) -> f64 {
        if total == 0 {
            0.0
        } else {
            missing as f64 / total as f64
        }
    }

    /// Count occurrences of each category
    fn count_categories(&self, values: &[String]) -> std::collections::HashMap<String, usize> {
        let mut counts = std::collections::HashMap::new();
        for value in values {
            *counts.entry(value.clone()).or_insert(0) += 1;
        }
        counts
    }
}

impl Default for BalanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

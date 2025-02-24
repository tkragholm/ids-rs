use super::checker::BalanceChecker;
use super::processor::ValueProcessor;
use super::stats::StatisticalCalculations;
use crate::models::CovariateSummary;
use chrono::NaiveDate;
use std::collections::HashMap;
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
        let (case_values, case_missing) =
            self.processor
                .collect_numeric_values(cases, covariate_type, checker, &extractor);
        let (control_values, control_missing) =
            self.processor
                .collect_numeric_values(controls, covariate_type, checker, &extractor);

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

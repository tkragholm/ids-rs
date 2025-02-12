use crate::{error::CovariateError, models::*, storage::CovariateStore};
use chrono::NaiveDate;
//use rayon::prelude::*;
use statrs::statistics::Statistics;
use std::path::Path;

pub struct BalanceChecker {
    store: CovariateStore,
}

impl BalanceChecker {
    pub fn new(store: CovariateStore) -> Self {
        Self { store }
    }

    pub fn calculate_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<Vec<CovariateSummary>, CovariateError> {
        let mut summaries = Vec::new();

        // Calculate balance for education
        summaries.extend(self.calculate_education_balance(cases, controls)?);

        // Calculate balance for income
        summaries.extend(self.calculate_income_balance(cases, controls)?);

        // Calculate balance for occupation
        summaries.extend(self.calculate_occupation_balance(cases, controls)?);

        Ok(summaries)
    }

    fn calculate_standardized_difference(case_values: &[f64], control_values: &[f64]) -> f64 {
        let case_mean = case_values.mean();
        let control_mean = control_values.mean();

        let case_var = case_values.variance();
        let control_var = control_values.variance();

        let pooled_sd = ((case_var + control_var) / 2.0).sqrt();

        (case_mean - control_mean) / pooled_sd
    }

    fn calculate_variance_ratio(case_values: &[f64], control_values: &[f64]) -> f64 {
        let case_var = case_values.variance();
        let control_var = control_values.variance();

        case_var / control_var
    }

    fn calculate_education_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<Vec<CovariateSummary>, CovariateError> {
        let mut case_values = Vec::new();
        let mut control_values = Vec::new();

        // Collect values
        for (pnr, date) in cases {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                if let Some(education) = covariates.education.last() {
                    if let Some(value) = education.value.to_numeric_value() {
                        case_values.push(value);
                    }
                }
            }
        }

        for (pnr, date) in controls {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                if let Some(education) = covariates.education.last() {
                    if let Some(value) = education.value.to_numeric_value() {
                        control_values.push(value);
                    }
                }
            }
        }

        // Calculate all statistics before consuming the vectors
        let std_diff = Self::calculate_standardized_difference(&case_values, &control_values);
        let var_ratio = Self::calculate_variance_ratio(&case_values, &control_values);
        let mean_cases = case_values.mean();
        let mean_controls = control_values.mean();

        Ok(vec![CovariateSummary {
            variable: "Education (years)".to_string(),
            mean_cases,
            mean_controls,
            std_diff,
            variance_ratio: var_ratio,
        }])
    }

    // Similar implementations for income and occupation balance
    // Add missing methods
    fn calculate_income_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<Vec<CovariateSummary>, CovariateError> {
        let mut case_values = Vec::new();
        let mut control_values = Vec::new();

        // Collect values
        for (pnr, date) in cases {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                if let Some(income) = covariates.income.last() {
                    case_values.push(income.value.to_numeric_value());
                }
            }
        }

        for (pnr, date) in controls {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                if let Some(income) = covariates.income.last() {
                    control_values.push(income.value.to_numeric_value());
                }
            }
        }

        // Calculate all statistics before consuming the vectors
        let std_diff = Self::calculate_standardized_difference(&case_values, &control_values);
        let var_ratio = Self::calculate_variance_ratio(&case_values, &control_values);
        let mean_cases = case_values.mean();
        let mean_controls = control_values.mean();

        Ok(vec![CovariateSummary {
            variable: "Income".to_string(),
            mean_cases,
            mean_controls,
            std_diff,
            variance_ratio: var_ratio,
        }])
    }

    fn calculate_occupation_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<Vec<CovariateSummary>, CovariateError> {
        let mut case_values = Vec::new();
        let mut control_values = Vec::new();

        // Similar implementation as education balance but for occupation
        for (pnr, date) in cases {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                if let Some(occupation) = covariates.occupation.last() {
                    // Convert occupation code to numeric value for analysis
                    case_values.push(occupation.value.code.parse::<f64>().unwrap_or(0.0));
                }
            }
        }

        for (pnr, date) in controls {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                if let Some(occupation) = covariates.occupation.last() {
                    control_values.push(occupation.value.code.parse::<f64>().unwrap_or(0.0));
                }
            }
        }

        // Calculate all statistics before consuming the vectors
        let std_diff = Self::calculate_standardized_difference(&case_values, &control_values);
        let var_ratio = Self::calculate_variance_ratio(&case_values, &control_values);
        let mean_cases = case_values.mean();
        let mean_controls = control_values.mean();

        Ok(vec![CovariateSummary {
            variable: "Occupation".to_string(),
            mean_cases: mean_cases,
            mean_controls: mean_controls,
            std_diff: std_diff,
            variance_ratio: var_ratio,
        }])
    }

    pub fn save_balance_results(
        results: &[CovariateSummary],
        output_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let checker = BalanceChecker::new(CovariateStore::new());
        checker.save_results(results, output_path)?;
        Ok(())
    }

    pub fn save_results(
        &self,
        results: &[CovariateSummary],
        output_path: &Path,
    ) -> Result<(), CovariateError> {
        let mut wtr = csv::Writer::from_path(output_path).map_err(CovariateError::Csv)?;

        wtr.write_record([
            "Variable",
            "Mean (Cases)",
            "Mean (Controls)",
            "Standardized Difference",
            "Variance Ratio",
        ])
        .map_err(CovariateError::Csv)?;

        for result in results {
            wtr.write_record([
                &result.variable,
                &result.mean_cases.to_string(),
                &result.mean_controls.to_string(),
                &result.std_diff.to_string(),
                &result.variance_ratio.to_string(),
            ])
            .map_err(CovariateError::Csv)?;
        }

        wtr.flush().map_err(|e| CovariateError::Io(e))?;
        Ok(())
    }
}

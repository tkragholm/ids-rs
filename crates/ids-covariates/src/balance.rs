use crate::{error::CovariateError, models::*, storage::CovariateStore};
use chrono::NaiveDate;
use rayon::prelude::*;
use statrs::statistics::Statistics;

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
        let control_values = Vec::new();

        for (pnr, date) in cases {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                if let Some(education) = covariates.education.last() {
                    if let Some(value) = education.value.to_numeric_value() {
                        case_values.push(value);
                    }
                }
            }
        }

        // Similar for controls...

        // Clone values before moving
        let case_values_clone = case_values.clone();
        let control_values_clone = control_values.clone();

        Ok(vec![CovariateSummary {
            variable: "Education (years)".to_string(),
            mean_cases: case_values.mean(),
            mean_controls: control_values.mean(),
            std_diff: Self::calculate_standardized_difference(
                &case_values_clone,
                &control_values_clone,
            ),
            variance_ratio: Self::calculate_variance_ratio(
                &case_values_clone,
                &control_values_clone,
            ),
        }])
    }

    // Similar implementations for income and occupation balance
    // Add missing methods
    fn calculate_income_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<Vec<CovariateSummary>, CovariateError> {
        // Implementation
        todo!()
    }

    fn calculate_occupation_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<Vec<CovariateSummary>, CovariateError> {
        // Implementation
        todo!()
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

        wtr.flush().map_err(CovariateError::Csv)?;
        Ok(())
    }
}

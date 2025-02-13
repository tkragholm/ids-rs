use crate::converter::IntoSnapshot;
use crate::error::CovariateError;
use crate::models::*;
use crate::storage::CovariateStore;
use chrono::NaiveDate;
use ids_arrow::CovariateSnapshot;
use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::path::Path;

pub struct BalanceChecker {
    store: CovariateStore,
}

pub struct BalanceResults {
    pub summaries: Vec<CovariateSummary>,
    pub missing_data_rates: HashMap<String, (f64, f64)>, // (case_rate, control_rate)
}

impl BalanceChecker {
    pub fn new(store: CovariateStore) -> Self {
        Self { store }
    }

    fn add_numeric_balance<F>(
        &self,
        summaries: &mut Vec<CovariateSummary>,
        missing_rates: &mut HashMap<String, (f64, f64)>,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        name: &str,
        extractor: F,
    ) -> Result<(), CovariateError>
    where
        F: Fn(&CovariateSnapshot) -> Option<f64>,
    {
        let (case_values, case_missing) = self.collect_numeric_values(cases, &extractor)?;
        let (control_values, control_missing) =
            self.collect_numeric_values(controls, &extractor)?;

        missing_rates.insert(
            name.to_string(),
            (
                case_missing as f64 / cases.len() as f64,
                control_missing as f64 / controls.len() as f64,
            ),
        );

        if !case_values.is_empty() && !control_values.is_empty() {
            let case_stats = case_values.clone();
            let control_stats = control_values.clone();

            summaries.push(CovariateSummary {
                variable: name.to_string(),
                mean_cases: case_stats.mean(),
                mean_controls: control_stats.mean(),
                std_diff: Self::calculate_standardized_difference(&case_values, &control_values),
                variance_ratio: Self::calculate_variance_ratio(&case_values, &control_values),
            });
        }

        Ok(())
    }

    fn collect_numeric_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        extractor: &F,
    ) -> Result<(Vec<f64>, usize), CovariateError>
    where
        F: Fn(&CovariateSnapshot) -> Option<f64>,
    {
        let mut values = Vec::new();
        let mut missing = 0;

        for (pnr, date) in subjects {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                let snapshot = covariates.into_snapshot(*date);
                match extractor(&snapshot) {
                    Some(value) => values.push(value),
                    None => missing += 1,
                }
            }
        }

        Ok((values, missing))
    }

    fn add_categorical_balance<F>(
        &self,
        summaries: &mut Vec<CovariateSummary>,
        missing_rates: &mut HashMap<String, (f64, f64)>,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        name: &str,
        extractor: F,
    ) -> Result<(), CovariateError>
    where
        F: for<'a> Fn(&'a CovariateSnapshot) -> Option<String>,
    {
        let (case_values, case_missing) = self.collect_categorical_values(cases, &extractor)?;
        let (control_values, control_missing) =
            self.collect_categorical_values(controls, &extractor)?;

        missing_rates.insert(
            name.to_string(),
            (
                case_missing as f64 / cases.len() as f64,
                control_missing as f64 / controls.len() as f64,
            ),
        );

        // Calculate frequencies for each category
        let mut case_freqs = HashMap::new();
        let mut control_freqs = HashMap::new();

        for value in case_values {
            *case_freqs.entry(value).or_insert(0) += 1;
        }
        for value in control_values {
            *control_freqs.entry(value).or_insert(0) += 1;
        }

        // Add summary statistics for categorical variables
        for (category, count) in &case_freqs {
            let case_prop = *count as f64 / cases.len() as f64;
            let control_prop = control_freqs
                .get(category)
                .map(|&count| count as f64 / controls.len() as f64)
                .unwrap_or(0.0);

            summaries.push(CovariateSummary {
                variable: format!("{} - {}", name, category),
                mean_cases: case_prop,
                mean_controls: control_prop,
                std_diff: (case_prop - control_prop)
                    / ((case_prop * (1.0 - case_prop) + control_prop * (1.0 - control_prop)) / 2.0)
                        .sqrt(),
                variance_ratio: 1.0, // Not applicable for categorical variables
            });
        }

        Ok(())
    }

    fn collect_categorical_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        extractor: &F,
    ) -> Result<(Vec<String>, usize), CovariateError>
    where
        F: Fn(&CovariateSnapshot) -> Option<String>,
    {
        let mut values = Vec::new();
        let mut missing = 0;

        for (pnr, date) in subjects {
            if let Some(covariates) = self.store.get_covariates_at_date(pnr, *date) {
                let snapshot = covariates.into_snapshot(*date);
                match extractor(&snapshot) {
                    Some(value) => values.push(value),
                    None => missing += 1,
                }
            }
        }

        Ok((values, missing))
    }

    pub fn calculate_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<BalanceResults, CovariateError> {
        let mut summaries = Vec::new();
        let mut missing_rates = HashMap::new();

        // Personal characteristics
        self.add_numeric_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            "Income",
            |snap| snap.income,
        )?;

        self.add_numeric_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            "Socioeconomic Status",
            |snap| snap.socioeconomic_status.map(|s| s as f64),
        )?;

        // Parent characteristics
        self.add_numeric_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            "Father's Income",
            |snap| snap.father_income,
        )?;

        self.add_numeric_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            "Mother's Income",
            |snap| snap.mother_income,
        )?;

        // Categorical variables
        self.add_categorical_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            "Education",
            |snap| snap.education.clone(), // Remove the Some() wrapper
        )?;

        self.add_categorical_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            "Family Type",
            |snap| snap.family_type.clone(), // Remove the Some() wrapper
        )?;

        Ok(BalanceResults {
            summaries,
            missing_data_rates: missing_rates,
        })
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

    pub fn save_to_files(&self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Save summaries
        let summaries_path = base_path.with_file_name("covariate_balance.csv");
        let results = self.calculate_balance(cases, controls)?;
        BalanceChecker::save_balance_results(&results.summaries, &summaries_path)?;

        // Save missing data rates
        let missing_rates_path = base_path.with_file_name("missing_data_rates.csv");
        let mut wtr = csv::Writer::from_path(missing_rates_path)?;

        wtr.write_record(&["Variable", "Case Missing Rate", "Control Missing Rate"])?;

        for (var, (case_rate, control_rate)) in &results.missing_data_rates {
            wtr.write_record(&[var, &case_rate.to_string(), &control_rate.to_string()])?;
        }

        wtr.flush()?;
        Ok(())
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

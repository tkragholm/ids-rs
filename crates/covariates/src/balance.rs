use crate::models::CovariateSummary;
use chrono::NaiveDate;
use log::debug;
use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::path::Path;
use types::{
    error::IdsError,
    models::{Covariate, CovariateType, CovariateValue},
    store::{ArrowStore, Store},
};

pub struct BalanceChecker {
    store: ArrowStore,
}

pub struct BalanceResults {
    pub summaries: Vec<CovariateSummary>,
    pub missing_data_rates: HashMap<String, (f64, f64)>, // (case_rate, control_rate)
}

#[allow(clippy::cast_precision_loss)]
impl BalanceChecker {
    #[must_use]
    pub const fn new(store: ArrowStore) -> Self {
        Self { store }
    }

    fn add_numeric_balance<F>(
        &self,
        summaries: &mut Vec<CovariateSummary>,
        missing_rates: &mut HashMap<String, (f64, f64)>,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        name: &str,
        extractor: F,
    ) -> Result<(), IdsError>
    where
        F: Fn(&Covariate) -> Option<f64>,
    {
        let (case_values, case_missing) =
            self.collect_numeric_values(cases, covariate_type, &extractor);
        let (control_values, control_missing) =
            self.collect_numeric_values(controls, covariate_type, &extractor);

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
        covariate_type: CovariateType,
        extractor: &F,
    ) -> (Vec<f64>, usize)
    where
        F: Fn(&Covariate) -> Option<f64>,
    {
        let mut values = Vec::new();
        let mut missing = 0;

        for (pnr, date) in subjects {
            match self.store.get_covariate(pnr, covariate_type, *date) {
                Ok(Some(covariate)) => match extractor(&covariate) {
                    Some(value) => values.push(value),
                    None => missing += 1,
                },
                _ => missing += 1,
            }
        }

        (values, missing)
    }

    fn add_categorical_balance<F>(
        &self,
        summaries: &mut Vec<CovariateSummary>,
        missing_rates: &mut HashMap<String, (f64, f64)>,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        name: &str,
        extractor: F,
    ) -> Result<(), IdsError>
    where
        F: Fn(&Covariate) -> Option<String>,
    {
        let (case_values, case_missing) =
            self.collect_categorical_values(cases, covariate_type, &extractor);
        let (control_values, control_missing) =
            self.collect_categorical_values(controls, covariate_type, &extractor);

        // Log the counts of values we're finding
        log::debug!(
            "Found {} case values and {} control values for {}",
            case_values.len(),
            control_values.len(),
            name
        );

        // Add debug logging for the first few values
        if !case_values.is_empty() {
            log::debug!(
                "Sample {} case values: {:?}",
                name,
                &case_values[..std::cmp::min(5, case_values.len())]
            );
        }

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

        // Log the frequency distributions
        log::debug!("Case frequencies for {}: {:?}", name, case_freqs);
        log::debug!("Control frequencies for {}: {:?}", name, control_freqs);

        // Add summary statistics for categorical variables
        for (category, count) in &case_freqs {
            let case_prop = f64::from(*count) / cases.len() as f64;
            let control_prop = control_freqs
                .get(category)
                .map_or(0.0, |&count| f64::from(count) / controls.len() as f64);

            summaries.push(CovariateSummary {
                variable: format!("{name} - {category}"),
                mean_cases: case_prop,
                mean_controls: control_prop,
                std_diff: (case_prop - control_prop)
                    / (case_prop.mul_add(1.0 - case_prop, control_prop * (1.0 - control_prop))
                        / 2.0)
                        .sqrt(),
                variance_ratio: 1.0, // Not applicable for categorical variables
            });
        }

        Ok(())
    }

    fn collect_categorical_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        extractor: &F,
    ) -> (Vec<String>, usize)
    where
        F: Fn(&Covariate) -> Option<String>,
    {
        let mut values = Vec::new();
        let mut missing = 0;

        for (pnr, date) in subjects {
            match self.store.get_covariate(pnr, covariate_type, *date) {
                Ok(Some(covariate)) => match extractor(&covariate) {
                    Some(value) => values.push(value),
                    None => missing += 1,
                },
                _ => missing += 1,
            }
        }

        (values, missing)
    }

    /// Calculate balance metrics between cases and controls
    ///
    /// # Errors
    /// Returns an error if there are issues accessing covariate data
    pub fn calculate_balance(
        &self,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<BalanceResults, IdsError> {
        let mut summaries = Vec::new();
        let mut missing_rates = HashMap::new();

        debug!(
            "Starting balance calculation for {} cases and {} controls",
            cases.len(),
            controls.len()
        );

        // Demographics and basic metrics
        self.add_numeric_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            CovariateType::Demographics,
            "Family Size",
            |covariate| match covariate {
                Covariate::Demographics { family_size, .. } => Some(*family_size as f64),
                _ => None,
            },
        )?;

        self.add_numeric_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            CovariateType::Demographics,
            "Municipality",
            |covariate| match covariate {
                Covariate::Demographics { municipality, .. } => Some(*municipality as f64),
                _ => None,
            },
        )?;

        // Add family type as categorical
        self.add_categorical_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            CovariateType::Demographics,
            "Family Type",
            |covariate| match covariate {
                Covariate::Demographics { family_type, .. } => Some(family_type.clone()),
                _ => None,
            },
        )?;

        // IND data
        self.add_numeric_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            CovariateType::Income,
            "Income",
            |covariate| match covariate {
                Covariate::Income { amount, .. } => Some(*amount),
                _ => None,
            },
        )?;

        // UDDF data
        self.add_categorical_balance(
            &mut summaries,
            &mut missing_rates,
            cases,
            controls,
            CovariateType::Education,
            "Education Level",
            |covariate| match covariate {
                Covariate::Education { level, .. } => Some(level.clone()),
                _ => None,
            },
        )?;

        debug!("Generated {} balance summaries", summaries.len());
        for summary in &summaries {
            debug!(
                "Summary for {}: case mean = {}, control mean = {}, std diff = {}",
                summary.variable, summary.mean_cases, summary.mean_controls, summary.std_diff
            );
        }

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

    /// Save balance results to files
    ///
    /// # Errors
    /// Returns an error if there are issues writing to the output files
    pub fn save_to_files(
        &self,
        base_path: &Path,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Save summaries
        let summaries_path = base_path.with_file_name("covariate_balance.csv");
        let results = self.calculate_balance(cases, controls)?;
        Self::save_balance_results(&results.summaries, &summaries_path)?;

        // Save missing data rates
        let missing_rates_path = base_path.with_file_name("missing_data_rates.csv");
        let mut wtr = csv::Writer::from_path(missing_rates_path)?;

        wtr.write_record(["Variable", "Case Missing Rate", "Control Missing Rate"])?;

        for (var, (case_rate, control_rate)) in &results.missing_data_rates {
            wtr.write_record([var, &case_rate.to_string(), &control_rate.to_string()])?;
        }

        wtr.flush()?;
        Ok(())
    }

    /// Save balance results to a CSV file
    ///
    /// # Errors
    /// Returns an error if there are issues writing the results to the output file
    pub fn save_balance_results(
        results: &[CovariateSummary],
        output_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let checker = Self::new(ArrowStore::new());
        checker.save_results(results, output_path)?;
        Ok(())
    }

    /// Save covariate summaries to a CSV file
    ///
    /// # Errors
    /// Returns an error if there are issues writing to the CSV file
    pub fn save_results(
        &self,
        results: &[CovariateSummary],
        output_path: &Path,
    ) -> Result<(), IdsError> {
        let mut wtr = csv::Writer::from_path(output_path).map_err(IdsError::Csv)?;

        wtr.write_record([
            "Variable",
            "Mean (Cases)",
            "Mean (Controls)",
            "Standardized Difference",
            "Variance Ratio",
        ])
        .map_err(IdsError::Csv)?;

        for result in results {
            wtr.write_record([
                &result.variable,
                &result.mean_cases.to_string(),
                &result.mean_controls.to_string(),
                &result.std_diff.to_string(),
                &result.variance_ratio.to_string(),
            ])
            .map_err(IdsError::Csv)?;
        }

        wtr.flush().map_err(IdsError::Io)?;
        Ok(())
    }
}

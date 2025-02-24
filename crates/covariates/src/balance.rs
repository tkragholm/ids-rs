use crate::models::CovariateSummary;
use crate::models::MatchedPairDetail;
use chrono::NaiveDate;
use dashmap::DashMap;
use log::debug;
use rayon::prelude::*;
use statrs::statistics::Statistics;
use std::collections::HashMap;
use std::sync::Arc;
use types::{
    error::IdsError,
    models::{Covariate, CovariateType, CovariateValue},
    store::{ArrowStore, Store},
};

#[derive(Hash, Eq, PartialEq, Clone)]
struct CacheKey {
    pnr: String,
    covariate_type: CovariateType,
    date: NaiveDate,
}

pub struct BalanceChecker {
    store: Arc<ArrowStore>,
    cache: DashMap<CacheKey, Option<Covariate>>,
}

pub struct BalanceResults {
    pub summaries: Vec<CovariateSummary>,
    pub missing_data_rates: HashMap<String, (f64, f64)>, // (case_rate, control_rate)
    pub matched_pair_details: Vec<MatchedPairDetail>,
}

#[derive(Debug, Clone)]
pub struct MatchedPairSummary {
    pub case_pnr: String,
    pub control_pnrs: Vec<String>,
    pub treatment_date: NaiveDate,
    pub summaries: Vec<CovariateSummary>,
    pub missing_rates: HashMap<String, (f64, f64)>,
}

#[allow(clippy::cast_precision_loss)]
impl BalanceChecker {
    #[must_use]
    pub fn new(store: ArrowStore) -> Self {
        Self {
            store: Arc::new(store),
            cache: DashMap::with_capacity(100_000),
        }
    }

    pub fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        let key = CacheKey {
            pnr: pnr.to_string(),
            covariate_type,
            date,
        };

        Ok(match self.cache.get(&key) {
            Some(cached) => cached.clone(),
            None => {
                let value = self.store.get_covariate(pnr, covariate_type, date)?;
                self.cache.insert(key, value.clone());
                value
            }
        })
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
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
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
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
    {
        const BATCH_SIZE: usize = 10_000;
        let chunk_size = (subjects.len() / rayon::current_num_threads()).max(BATCH_SIZE);

        let results: Vec<_> = subjects
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut values = Vec::with_capacity(chunk.len());
                let mut missing = 0;

                for (pnr, date) in chunk {
                    match self.get_covariate(pnr, covariate_type, *date) {
                        Ok(Some(covariate)) => match extractor(&covariate) {
                            Some(value) => values.push(value),
                            None => missing += 1,
                        },
                        _ => missing += 1,
                    }
                }

                (values, missing)
            })
            .collect();

        let total_capacity: usize = results.iter().map(|(v, _)| v.len()).sum();
        let mut all_values = Vec::with_capacity(total_capacity);
        let mut total_missing = 0;

        for (values, missing) in results {
            all_values.extend(values);
            total_missing += missing;
        }

        (all_values, total_missing)
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
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
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
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        const BATCH_SIZE: usize = 10_000;
        let chunk_size = (subjects.len() / rayon::current_num_threads()).max(BATCH_SIZE);

        let results: Vec<_> = subjects
            .par_chunks(chunk_size)
            .map(|chunk| {
                let mut values = Vec::with_capacity(chunk.len());
                let mut missing = 0;

                for (pnr, date) in chunk {
                    match self.get_covariate(pnr, covariate_type, *date) {
                        Ok(Some(covariate)) => match extractor(&covariate) {
                            Some(value) => values.push(value),
                            None => missing += 1,
                        },
                        _ => missing += 1,
                    }
                }

                (values, missing)
            })
            .collect();

        let total_capacity: usize = results.iter().map(|(v, _)| v.len()).sum();
        let mut all_values = Vec::with_capacity(total_capacity);
        let mut total_missing = 0;

        for (values, missing) in results {
            all_values.extend(values);
            total_missing += missing;
        }

        (all_values, total_missing)
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
        let mut matched_pair_details = Vec::new();

        debug!(
            "Starting balance calculation for {} cases and {} controls",
            cases.len(),
            controls.len()
        );

        // Calculate overall balance
        self.add_demographic_balance(&mut summaries, &mut missing_rates, cases, controls)?;
        self.add_income_balance(&mut summaries, &mut missing_rates, cases, controls)?;
        self.add_education_balance(&mut summaries, &mut missing_rates, cases, controls)?;

        // Calculate matched pair-specific balance
        for (case_pnr, case_date) in cases {
            let case_controls: Vec<_> = controls
                .iter()
                .filter(|(_, ctrl_date)| ctrl_date == case_date)
                .collect();

            if !case_controls.is_empty() {
                let mut pair_summaries = Vec::new();
                let mut pair_missing_rates = HashMap::new();

                let case = vec![(case_pnr.clone(), *case_date)];
                let ctrl_pairs: Vec<_> = case_controls
                    .iter()
                    .map(|(pnr, date)| (pnr.clone(), *date))
                    .collect();

                self.add_demographic_balance(
                    &mut pair_summaries,
                    &mut pair_missing_rates,
                    &case,
                    &ctrl_pairs,
                )?;
                self.add_income_balance(
                    &mut pair_summaries,
                    &mut pair_missing_rates,
                    &case,
                    &ctrl_pairs,
                )?;
                self.add_education_balance(
                    &mut pair_summaries,
                    &mut pair_missing_rates,
                    &case,
                    &ctrl_pairs,
                )?;

                for summary in pair_summaries {
                    matched_pair_details.push(MatchedPairDetail {
                        case_pnr: case_pnr.clone(),
                        control_pnrs: ctrl_pairs.iter().map(|(pnr, _)| pnr.clone()).collect(),
                        treatment_date: *case_date,
                        variable: summary.variable,
                        case_value: summary.mean_cases,
                        control_value: summary.mean_controls,
                        std_diff: summary.std_diff,
                    });
                }
            }
        }

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
            matched_pair_details,
        })
    }

    fn add_demographic_balance(
        &self,
        summaries: &mut Vec<CovariateSummary>,
        missing_rates: &mut HashMap<String, (f64, f64)>,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        self.add_numeric_balance(
            summaries,
            missing_rates,
            cases,
            controls,
            CovariateType::Demographics,
            "Family Size",
            |covariate| match &covariate.value {
                CovariateValue::Demographics { family_size, .. } => Some(*family_size as f64),
                _ => None,
            },
        )?;

        self.add_numeric_balance(
            summaries,
            missing_rates,
            cases,
            controls,
            CovariateType::Demographics,
            "Municipality",
            |covariate| match &covariate.value {
                CovariateValue::Demographics { municipality, .. } => Some(*municipality as f64),
                _ => None,
            },
        )?;

        self.add_categorical_balance(
            summaries,
            missing_rates,
            cases,
            controls,
            CovariateType::Demographics,
            "Family Type",
            |covariate| match &covariate.value {
                CovariateValue::Demographics { family_type, .. } => Some(family_type.clone()),
                _ => None,
            },
        )
    }

    fn add_income_balance(
        &self,
        summaries: &mut Vec<CovariateSummary>,
        missing_rates: &mut HashMap<String, (f64, f64)>,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        self.add_numeric_balance(
            summaries,
            missing_rates,
            cases,
            controls,
            CovariateType::Income,
            "Income",
            |covariate| match &covariate.value {
                CovariateValue::Income { amount, .. } => Some(*amount),
                _ => None,
            },
        )
    }

    fn add_education_balance(
        &self,
        summaries: &mut Vec<CovariateSummary>,
        missing_rates: &mut HashMap<String, (f64, f64)>,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        self.add_categorical_balance(
            summaries,
            missing_rates,
            cases,
            controls,
            CovariateType::Education,
            "Education Level",
            |covariate| match &covariate.value {
                CovariateValue::Education { level, .. } => Some(level.clone()),
                _ => None,
            },
        )
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
    fn calculate_pair_std_diff(case_value: f64, control_value: f64) -> f64 {
        let pooled_var = (case_value.powi(2) + control_value.powi(2)) / 2.0;
        if pooled_var == 0.0 {
            0.0
        } else {
            (case_value - control_value) / pooled_var.sqrt()
        }
    }

    // Add a method to process a single matched pair
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
            .and_then(&value_extractor);

        let control_value = self
            .get_covariate(control_pnr, covariate_type, date)?
            .and_then(&value_extractor);

        match (case_value, control_value) {
            (Some(case_val), Some(ctrl_val)) => Ok(Some(MatchedPairDetail {
                case_pnr: case_pnr.to_string(),
                control_pnrs: vec![control_pnr.to_string()],
                treatment_date: date,
                variable: variable_name.to_string(),
                case_value: case_val,
                control_value: ctrl_val,
                std_diff: Self::calculate_pair_std_diff(case_val, ctrl_val),
            })),
            _ => Ok(None),
        }
    }

    pub fn clear_cache(&self) {
        self.cache.clear();
    }

    pub fn cache_size(&self) -> usize {
        self.cache.len()
    }

    pub fn process_in_batches<T, F>(
        &self,
        items: &[T],
        batch_size: usize,
        mut f: F,
    ) -> Result<(), IdsError>
    where
        F: FnMut(&[T]) -> Result<(), IdsError>,
    {
        for batch in items.chunks(batch_size) {
            f(batch)?;
        }
        Ok(())
    }

    pub fn get_variable_summary(&self, variable: &str) -> Option<&CovariateSummary> {
        self.summaries.iter().find(|s| s.variable == variable)
    }

    pub fn get_matched_pair_details(&self, case_pnr: &str) -> Vec<&MatchedPairDetail> {
        self.matched_pair_details
            .iter()
            .filter(|d| d.case_pnr == case_pnr)
            .collect()
    }

    pub fn summarize_std_differences(&self) -> HashMap<String, (f64, f64, f64)> {
        let mut summaries = HashMap::new();

        for detail in &self.matched_pair_details {
            let stats = summaries
                .entry(detail.variable.clone())
                .or_insert((0.0, 0.0, 0));

            stats.0 += detail.std_diff;
            stats.1 += detail.std_diff.powi(2);
            stats.2 += 1;
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

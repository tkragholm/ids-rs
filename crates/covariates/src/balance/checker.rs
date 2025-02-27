use super::{
    cache::{CacheKey, CovariateCache},
    metrics::BalanceMetrics,
    // processor::ValueProcessor,
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

pub struct BalanceChecker {
    store: Arc<ArrowStore>,
    cache: CovariateCache,
    metrics: BalanceMetrics,
    //processor: ValueProcessor,
    results: Option<BalanceResults>,
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

    pub fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        let key = CacheKey::new(pnr, covariate_type, date);
        self.cache.get_or_load(&*self.store, key)
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
    fn add_all_balances(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
        overall_pb: &ProgressBar,
    ) -> Result<(), IdsError> {
        overall_pb.set_message("Processing demographics...");
        self.add_demographic_balance(results, cases, controls)?;
        overall_pb.inc(1);

        overall_pb.set_message("Processing income...");
        self.add_income_balance(results, cases, controls)?;
        overall_pb.inc(1);

        overall_pb.set_message("Processing education...");
        self.add_education_balance(results, cases, controls)?;
        overall_pb.inc(1);

        Ok(())
    }

    fn add_demographic_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
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
            |covariate| covariate.get_family_type(),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate("Family Type".to_string(), missing_rates.0, missing_rates.1);

        Ok(())
    }

    fn add_income_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
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

        Ok(())
    }

    fn add_education_balance(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        let (summaries, missing_rates) = self.metrics.calculate_categorical_balance(
            self,
            cases,
            controls,
            CovariateType::Education,
            "Education Level",
            |covariate| covariate.get_education_level(),
        )?;

        for summary in summaries {
            results.add_summary(summary);
        }
        results.add_missing_rate(
            "Education Level".to_string(),
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
        for (case_pnr, case_date) in cases {
            let matching_controls: Vec<_> = controls
                .iter()
                .filter(|(_, ctrl_date)| ctrl_date == case_date)
                .collect();

            for (control_pnr, _) in &matching_controls {
                // Family Size
                if let Some(detail) = self.process_matched_pair(
                    case_pnr,
                    control_pnr,
                    *case_date,
                    CovariateType::Demographics,
                    "Family Size",
                    |cov| cov.get_family_size().map(|val| val as f64),
                )? {
                    results.add_pair_detail(detail);
                }

                // Municipality
                if let Some(detail) = self.process_matched_pair(
                    case_pnr,
                    control_pnr,
                    *case_date,
                    CovariateType::Demographics,
                    "Municipality",
                    |cov| cov.get_municipality().map(|val| val as f64),
                )? {
                    results.add_pair_detail(detail);
                }

                // Income
                if let Some(detail) = self.process_matched_pair(
                    case_pnr,
                    control_pnr,
                    *case_date,
                    CovariateType::Income,
                    "Income",
                    |cov| cov.get_income_amount(),
                )? {
                    results.add_pair_detail(detail);
                }

                // Education Level
                if let Some(detail) = self.process_matched_pair(
                    case_pnr,
                    control_pnr,
                    *case_date,
                    CovariateType::Education,
                    "Education Level",
                    |cov| {
                        cov.get_education_level()
                            .and_then(|level| level.parse::<f64>().ok())
                    },
                )? {
                    results.add_pair_detail(detail);
                }
            }
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

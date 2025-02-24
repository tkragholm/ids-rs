use super::checker::BalanceChecker;
use chrono::NaiveDate;
use indicatif::{ParallelProgressIterator, ProgressStyle};
use rayon::prelude::*;
use types::models::{Covariate, CovariateType};

pub(crate) struct ValueProcessor;

impl ValueProcessor {
    pub fn new() -> Self {
        Self
    }

    fn create_progress_style(covariate_type: CovariateType) -> ProgressStyle {
        ProgressStyle::default_bar()
            .template(
                "{prefix:.bold.dim} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} \
                 ({percent}%) {msg}\n\
                 ⏱️  ETA: {eta_precise:.dim} | 🚀 {per_sec:.green} records/sec | \
                 📊 Processing: {covariate_type}",
            )
            .unwrap()
            .with_key(
                "covariate_type",
                move |_state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
                    write!(w, "{:?}", covariate_type).unwrap()
                },
            )
    }

    pub fn collect_numeric_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
    ) -> (Vec<f64>, usize)
    where
        F: Fn(&Covariate) -> Option<f64> + Send + Sync,
    {
        const BATCH_SIZE: usize = 10_000;
        let chunk_size = (subjects.len() / rayon::current_num_threads()).max(BATCH_SIZE);

        let style = Self::create_progress_style(covariate_type);
        let progress = subjects
            .par_chunks(chunk_size)
            .progress_with_style(style)
            .with_prefix("Numeric Values")
            .with_message("Processing numeric covariates...");

        let results: Vec<_> = progress
            .map(|chunk| {
                let mut values = Vec::with_capacity(chunk.len());
                let mut missing = 0;

                for (pnr, date) in chunk {
                    match checker.get_covariate(pnr, covariate_type, *date) {
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

    pub fn collect_categorical_values<F>(
        &self,
        subjects: &[(String, NaiveDate)],
        covariate_type: CovariateType,
        checker: &BalanceChecker,
        extractor: &F,
    ) -> (Vec<String>, usize)
    where
        F: Fn(&Covariate) -> Option<String> + Send + Sync,
    {
        const BATCH_SIZE: usize = 10_000;
        let chunk_size = (subjects.len() / rayon::current_num_threads()).max(BATCH_SIZE);

        let style = Self::create_progress_style(covariate_type);
        let progress = subjects
            .par_chunks(chunk_size)
            .progress_with_style(style)
            .with_prefix("Categorical Values")
            .with_message("Processing categorical covariates...");

        let results: Vec<_> = progress
            .map(|chunk| {
                let mut values = Vec::with_capacity(chunk.len());
                let mut missing = 0;

                for (pnr, date) in chunk {
                    match checker.get_covariate(pnr, covariate_type, *date) {
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
}

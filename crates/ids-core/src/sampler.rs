use crate::errors::SamplingError;
use crate::utils::{DateData, MatchingCriteria};
use chrono::NaiveDate;
use colored::Colorize;
use fastrand::Rng;
use humantime::format_duration;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use smallvec::SmallVec;
use std::sync::Arc;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub pnr: String,
    #[serde(with = "crate::utils::date_format")]
    pub bday: NaiveDate,
    #[serde(with = "crate::utils::optional_date_format")]
    pub treatment_date: Option<NaiveDate>,
    #[serde(with = "crate::utils::date_format")]
    pub mother_bday: NaiveDate,
    #[serde(with = "crate::utils::date_format")]
    pub father_bday: NaiveDate,
}

pub struct IncidenceDensitySampler {
    dates: Vec<DateData>,
    records: Arc<Vec<Record>>,
    criteria: MatchingCriteria,
    cases: Vec<usize>,
    sorted_controls: Vec<usize>,
    birth_date_index: Arc<FxHashMap<i64, SmallVec<[usize; 16]>>>,
}

impl IncidenceDensitySampler {
    pub fn new(records: Vec<Record>, criteria: MatchingCriteria) -> Result<Self, SamplingError> {
        criteria.validate()?;
        let n_records = records.len();
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

        let mut birth_date_index =
            FxHashMap::with_capacity_and_hasher(n_records / 365, Default::default());
        let mut dates = Vec::with_capacity(n_records);
        let mut cases = Vec::with_capacity(50_000);
        let mut controls = Vec::with_capacity(n_records - 50_000);

        let processed_data: Vec<_> = records
            .par_iter()
            .enumerate()
            .map(|(idx, record)| {
                let birth = (record.bday - epoch).num_days();
                let mother = (record.mother_bday - epoch).num_days();
                let father = (record.father_bday - epoch).num_days();
                let treatment = record.treatment_date.map(|d| (d - epoch).num_days());

                (
                    idx,
                    DateData {
                        birth,
                        mother,
                        father,
                    },
                    treatment,
                )
            })
            .collect();

        for (idx, date_data, treatment) in processed_data {
            dates.push(date_data);

            birth_date_index
                .entry(date_data.birth)
                .or_insert_with(|| SmallVec::with_capacity(16))
                .push(idx);

            if treatment.is_some() {
                cases.push(idx);
            } else {
                controls.push(idx);
            }
        }

        controls.sort_unstable();

        Ok(Self {
            dates,
            records: Arc::new(records),
            criteria,
            cases,
            sorted_controls: controls,
            birth_date_index: Arc::new(birth_date_index),
        })
    }

    fn select_random_controls(
        rng: &mut Rng,
        eligible: &[usize],
        n_controls: usize,
    ) -> SmallVec<[usize; 8]> {
        if eligible.len() <= n_controls {
            return SmallVec::from_vec(eligible.to_vec());
        }

        let mut selected = SmallVec::with_capacity(n_controls);
        let mut indices = (0..eligible.len()).collect::<SmallVec<[usize; 32]>>();

        for _ in 0..n_controls {
            let idx = rng.usize(..indices.len());
            selected.push(eligible[indices[idx]]);
            indices.swap_remove(idx);
        }

        selected
    }

    fn print_header(text: &str) {
        println!("\n{}", text.bold().blue());
        println!("{}", "=".repeat(text.len()).blue());
    }

    fn format_percentage(value: f64, total: f64) -> String {
        format!(
            "{:.1}% ({}/{})",
            (value / total * 100.0),
            value as u64,
            total as u64
        )
    }

    pub fn get_statistics(&self) -> String {
        let total_records = self.records.len();
        let total_cases = self.cases.len();
        let total_controls = self.sorted_controls.len();

        format!(
            "{}\n\
                │ {} {}\n\
                │ {} {}\n\
                │ {} {}\n\
                │ {} {:.2}\n\
                └────────────────────────────",
            "Dataset Statistics:".bold().green(),
            "Total Records:".bold(),
            total_records.to_string().yellow(),
            "Cases:".bold(),
            Self::format_percentage(total_cases as f64, total_records as f64).yellow(),
            "Controls:".bold(),
            Self::format_percentage(total_controls as f64, total_records as f64).yellow(),
            "Case/Control Ratio:".bold(),
            (total_controls as f64 / total_cases as f64)
        )
    }

    pub fn sample_controls(
        &self,
        n_controls: usize,
    ) -> Result<Vec<(usize, SmallVec<[usize; 8]>)>, SamplingError> {
        Self::print_header("Sampling Controls");
        const BATCH_SIZE: usize = 1024;

        let start_time = std::time::Instant::now();
        let total_cases = self.cases.len();
        let total_chunks = total_cases.div_ceil(BATCH_SIZE);

        println!(
            "{}",
            format!(
                "│ {} {}\n│ {} {}",
                "Total Cases:".bold(),
                total_cases.to_string().yellow(),
                "Batch Size:".bold(),
                BATCH_SIZE.to_string().yellow()
            )
        );

        let mp = MultiProgress::new();
        let pb = mp.add(ProgressBar::new(total_chunks as u64));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} chunks ({per_sec}) {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        let case_control_pairs: Vec<_> = self
            .cases
            .par_chunks(BATCH_SIZE)
            .map(|case_chunk| {
                let mut rng = Rng::new();
                let mut local_pairs = Vec::with_capacity(case_chunk.len());
                let mut eligible_buffer = SmallVec::<[usize; 32]>::new();

                for &case_idx in case_chunk {
                    let case_date = self.dates[case_idx];
                    let window_start = case_date.birth - self.criteria.birth_date_window;
                    let window_end = case_date.birth + self.criteria.birth_date_window;

                    eligible_buffer.clear();

                    for birth_date in window_start..=window_end {
                        if let Some(controls) = self.birth_date_index.get(&birth_date) {
                            for &control_idx in controls {
                                if self.sorted_controls.binary_search(&control_idx).is_ok() {
                                    let control_date = self.dates[control_idx];
                                    let mother_diff =
                                        (case_date.mother - control_date.mother).abs();
                                    let father_diff =
                                        (case_date.father - control_date.father).abs();

                                    if mother_diff <= self.criteria.parent_date_window
                                        && father_diff <= self.criteria.parent_date_window
                                    {
                                        eligible_buffer.push(control_idx);
                                    }
                                }
                            }
                        }
                    }

                    if !eligible_buffer.is_empty() {
                        let selected =
                            Self::select_random_controls(&mut rng, &eligible_buffer, n_controls);
                        if !selected.is_empty() {
                            local_pairs.push((case_idx, selected));
                        }
                    }
                }

                pb.inc(1);
                pb.set_message(format!("{} matches", local_pairs.len()));

                local_pairs
            })
            .flatten()
            .collect();

        pb.finish_with_message("Complete");

        let total_elapsed = start_time.elapsed();
        println!(
            "\n{}\n│ {} {}\n│ {} {}\n│ {} {:.2}\n└────────────────────────────",
            "Sampling Results:".bold().green(),
            "Time Elapsed:".bold(),
            format_duration(total_elapsed).to_string().yellow(),
            "Total Matches:".bold(),
            case_control_pairs.len().to_string().yellow(),
            "Speed:".bold(),
            (total_cases as f64 / total_elapsed.as_secs_f64())
        );

        if case_control_pairs.is_empty() {
            return Err(SamplingError::NoEligibleControls);
        }

        Ok(case_control_pairs)
    }

    pub fn evaluate_matching_quality(
        &self,
        case_control_pairs: &[(usize, SmallVec<[usize; 8]>)],
    ) -> crate::matching_quality::MatchingQuality {
        let total_cases = self.cases.len();
        let matched_cases = case_control_pairs.len();
        let total_controls: usize = case_control_pairs
            .iter()
            .map(|(_, controls)| controls.len())
            .sum();

        let mut birth_date_differences = Vec::new();
        let mut mother_age_differences = Vec::new();
        let mut father_age_differences = Vec::new();

        for (case_idx, controls) in case_control_pairs {
            let case_dates = self.dates[*case_idx];

            for &control_idx in controls {
                let control_dates = self.dates[control_idx];

                birth_date_differences.push((case_dates.birth - control_dates.birth).abs());
                mother_age_differences.push((case_dates.mother - control_dates.mother).abs());
                father_age_differences.push((case_dates.father - control_dates.father).abs());
            }
        }

        let birth_date_balance = self.calculate_balance_metric(&birth_date_differences);
        let parent_age_balance = (self.calculate_balance_metric(&mother_age_differences)
            + self.calculate_balance_metric(&father_age_differences))
            / 2.0;

        let percentiles = vec![0.25, 0.50, 0.75];
        let birth_date_percentiles =
            crate::matching_quality::MatchingQuality::calculate_percentiles(
                &birth_date_differences,
                &percentiles,
            );
        let mother_age_percentiles =
            crate::matching_quality::MatchingQuality::calculate_percentiles(
                &mother_age_differences,
                &percentiles,
            );
        let father_age_percentiles =
            crate::matching_quality::MatchingQuality::calculate_percentiles(
                &father_age_differences,
                &percentiles,
            );

        crate::matching_quality::MatchingQuality::new(
            total_cases,
            matched_cases,
            total_controls,
            birth_date_differences,
            mother_age_differences,
            father_age_differences,
            birth_date_balance,
            parent_age_balance,
            birth_date_percentiles,
            mother_age_percentiles,
            father_age_percentiles,
        )
    }

    fn calculate_balance_metric(&self, diffs: &[i64]) -> f64 {
        let mean = diffs.iter().sum::<i64>() as f64 / diffs.len() as f64;
        let variance = diffs
            .iter()
            .map(|&x| {
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / (diffs.len() - 1) as f64;

        mean / variance.sqrt()
    }

    pub fn save_matches_to_csv(
        &self,
        case_control_pairs: &[(usize, SmallVec<[usize; 8]>)],
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        log::info!("Saving matches to {}", filename);
        let mut wtr = csv::Writer::from_path(filename)?;

        wtr.write_record([
            "case_id",
            "case_pnr",
            "case_birth_date",
            "case_treatment_date",
            "control_id",
            "control_pnr",
            "control_birth_date",
            "birth_date_diff_days",
            "mother_age_diff_days",
            "father_age_diff_days",
        ])?;

        for (case_idx, controls) in case_control_pairs {
            let case = &self.records[*case_idx];
            let case_dates = self.dates[*case_idx];

            for &control_idx in controls {
                let control = &self.records[control_idx];
                let control_dates = self.dates[control_idx];

                let birth_diff = (case_dates.birth - control_dates.birth).abs();
                let mother_diff = (case_dates.mother - control_dates.mother).abs();
                let father_diff = (case_dates.father - control_dates.father).abs();

                wtr.write_record(&[
                    case_idx.to_string(),
                    case.pnr.clone(),
                    case.bday.to_string(),
                    case.treatment_date
                        .map_or("NA".to_string(), |d| d.to_string()),
                    control_idx.to_string(),
                    control.pnr.clone(),
                    control.bday.to_string(),
                    birth_diff.to_string(),
                    mother_diff.to_string(),
                    father_diff.to_string(),
                ])?;
            }
        }

        wtr.flush()?;
        log::info!("Successfully wrote matches to {}", filename);

        let total_pairs: usize = case_control_pairs
            .iter()
            .map(|(_, controls)| controls.len())
            .sum();
        log::info!("Total case-control pairs written: {}", total_pairs);
        log::info!(
            "Average controls per case: {:.2}",
            total_pairs as f64 / case_control_pairs.len() as f64
        );

        Ok(())
    }

    pub fn save_matching_statistics(
        &self,
        case_control_pairs: &[(usize, SmallVec<[usize; 8]>)],
        filename: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path(filename)?;

        wtr.write_record([
            "case_id",
            "n_controls",
            "avg_birth_diff",
            "max_birth_diff",
            "avg_mother_diff",
            "avg_father_diff",
        ])?;

        for (case_idx, controls) in case_control_pairs {
            let case_dates = self.dates[*case_idx];

            let stats = controls
                .iter()
                .map(|&control_idx| {
                    let control_dates = self.dates[control_idx];
                    (
                        (case_dates.birth - control_dates.birth).abs(),
                        (case_dates.mother - control_dates.mother).abs(),
                        (case_dates.father - control_dates.father).abs(),
                    )
                })
                .fold((0, 0, 0, 0), |acc, (b, m, f)| {
                    (acc.0 + b, acc.1.max(b), acc.2 + m, acc.3 + f)
                });

            let n_controls = controls.len() as f64;

            wtr.write_record(&[
                case_idx.to_string(),
                controls.len().to_string(),
                (stats.0 as f64 / n_controls).to_string(),
                stats.1.to_string(),
                (stats.2 as f64 / n_controls).to_string(),
                (stats.3 as f64 / n_controls).to_string(),
            ])?;
        }

        Ok(())
    }
}

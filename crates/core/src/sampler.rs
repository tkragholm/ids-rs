use crate::errors::SamplingError;
use crate::utils::{DateData, MatchingCriteria};
use chrono::NaiveDate;
use colored::Colorize;
use fastrand::Rng;
use humantime::format_duration;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rayon::prelude::*;
use rustc_hash::{FxBuildHasher, FxHashMap};
use serde::Deserialize;
use smallvec::SmallVec;
use std::sync::Arc;

type ControlGroup = SmallVec<[usize; 8]>;
type CaseControlPair = (usize, ControlGroup);
type MatchResult = Result<Vec<CaseControlPair>, SamplingError>;
type BirthDateIndex = Arc<FxHashMap<i64, SmallVec<[usize; 16]>>>;

#[derive(Debug, Deserialize)]
pub struct Record {
    pub pnr: String,
    #[serde(with = "crate::utils::date_format")]
    pub bday: NaiveDate,
    #[serde(with = "crate::utils::optional_date_format")]
    pub treatment_date: Option<NaiveDate>,
    #[serde(with = "crate::utils::optional_date_format")]
    pub mother_bday: Option<NaiveDate>,
    #[serde(with = "crate::utils::optional_date_format")]
    pub father_bday: Option<NaiveDate>,
}

pub struct IncidenceDensitySampler {
    dates: Vec<DateData>,
    records: Arc<Vec<Record>>,
    criteria: MatchingCriteria,
    cases: Vec<usize>,
    sorted_controls: Vec<usize>,
    birth_date_index: BirthDateIndex,
}

impl IncidenceDensitySampler {
    const BATCH_SIZE: usize = 1024;

    /// Creates a new IncidenceDensitySampler.
    ///
    /// # Panics
    /// Panics if the epoch date (1970-01-01) cannot be created.
    ///
    /// # Errors
    /// Returns a `SamplingError` if the matching criteria are invalid.
    pub fn new(records: Vec<Record>, criteria: MatchingCriteria) -> Result<Self, SamplingError> {
        criteria.validate()?;
        let n_records = records.len();
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();

        let mut birth_date_index =
            FxHashMap::with_capacity_and_hasher(n_records / 365, FxBuildHasher);
        let mut dates = Vec::with_capacity(n_records);
        let mut cases = Vec::with_capacity(50_000);
        let mut controls = Vec::with_capacity(n_records - 50_000);

        let processed_data: Vec<_> = records
            .par_iter()
            .enumerate()
            .map(|(idx, record)| {
                let birth = (record.bday - epoch).num_days();
                let mother = record.mother_bday.map(|d| (d - epoch).num_days());
                let father = record.father_bday.map(|d| (d - epoch).num_days());
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

    const fn is_parent_match(
        case_parent: Option<i64>,
        control_parent: Option<i64>,
        window: i64,
    ) -> bool {
        match (case_parent, control_parent) {
            (None, None) => true, // Match if both are missing
            (Some(case_date), Some(control_date)) => (case_date - control_date).abs() <= window,
            _ => false, // Don't match if one is missing and the other isn't
        }
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

    #[allow(dead_code, clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    fn format_percentage(value: f64, total: f64) -> String {
        format!(
            "{:.1}% ({}/{})",
            (value / total * 100.0),
            value as u64,
            total as u64
        )
    }

    #[must_use]
    pub fn get_statistics(&self) -> String {
        use crate::utils::console::ConsoleOutput;
        
        let total_records = self.records.len();
        let total_cases = self.cases.len();
        let total_controls = self.sorted_controls.len();
        let case_pct = total_cases as f64 / total_records as f64 * 100.0;
        let control_pct = total_controls as f64 / total_records as f64 * 100.0;

        let mut stats = String::new();
        
        stats.push_str(&format!("\n{}\n", "Dataset Statistics".green().bold()));
        stats.push_str(&format!("{}\n", "═".repeat(18).green()));
        
        stats.push_str(&format!(
            "│ {} {}\n",
            "Total Records:".bold(),
            ConsoleOutput::format_number(total_records).yellow()
        ));
        
        stats.push_str(&format!(
            "│ {} {} ({:.1}%)\n",
            "Cases:".bold(),
            ConsoleOutput::format_number(total_cases).yellow(),
            case_pct
        ));
        
        stats.push_str(&format!(
            "│ {} {} ({:.1}%)\n",
            "Controls:".bold(),
            ConsoleOutput::format_number(total_controls).yellow(),
            control_pct
        ));
        
        let ratio = total_controls as f64 / total_cases as f64;
        let ratio_str = format!("{:.2}", ratio);
        let ratio_colored = if ratio >= 3.0 {
            ratio_str.green()
        } else if ratio >= 1.0 {
            ratio_str.yellow()
        } else {
            ratio_str.red()
        };
        
        stats.push_str(&format!(
            "│ {} {}\n",
            "Case/Control Ratio:".bold(),
            ratio_colored
        ));
        
        stats.push_str(&format!("└{}\n", "─".repeat(30)));
        
        stats
    }

    /// Samples controls for each case according to the matching criteria.
    ///
    /// # Panics
    /// Panics if the progress bar template is invalid.
    ///
    /// # Errors
    /// Returns a `SamplingError` if no eligible controls are found.
    pub fn sample_controls(&self, n_controls: usize) -> MatchResult {
        Self::print_header("Sampling Controls");

        let start_time = std::time::Instant::now();
        let total_cases = self.cases.len();
        let total_chunks = total_cases.div_ceil(Self::BATCH_SIZE);

        println!(
            "│ {} {}\n│ {} {}",
            "Total Cases:".bold(),
            total_cases.to_string().yellow(),
            "Batch Size:".bold(),
            Self::BATCH_SIZE.to_string().yellow()
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
            .par_chunks(Self::BATCH_SIZE)
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

                                    // Check parent matches considering missing values
                                    let mother_match = Self::is_parent_match(
                                        case_date.mother,
                                        control_date.mother,
                                        self.criteria.parent_date_window,
                                    );
                                    let father_match = Self::is_parent_match(
                                        case_date.father,
                                        control_date.father,
                                        self.criteria.parent_date_window,
                                    );

                                    if mother_match && father_match {
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
        #[allow(clippy::cast_precision_loss)]
        let speed = total_cases as f64 / total_elapsed.as_secs_f64();

        println!(
            "\n{}\n│ {} {}\n│ {} {}\n│ {} {:.2}\n└────────────────────────────",
            "Sampling Results:".bold().green(),
            "Time Elapsed:".bold(),
            format_duration(total_elapsed).to_string().yellow(),
            "Total Matches:".bold(),
            case_control_pairs.len().to_string().yellow(),
            "Speed:".bold(),
            speed
        );

        if case_control_pairs.is_empty() {
            return Err(SamplingError::NoEligibleControls);
        }

        Ok(case_control_pairs)
    }

    #[must_use]
    pub fn evaluate_matching_quality(
        &self,
        case_control_pairs: &[CaseControlPair],
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

                if let Some(diff) = calculate_date_diff(case_dates.mother, control_dates.mother) {
                    mother_age_differences.push(diff);
                }

                if let Some(diff) = calculate_date_diff(case_dates.father, control_dates.father) {
                    father_age_differences.push(diff);
                }
            }
        }

        #[allow(clippy::cast_precision_loss)]
        let birth_date_balance = Self::calculate_balance_metric(&birth_date_differences);
        #[allow(clippy::cast_precision_loss)]
        let parent_age_balance = (Self::calculate_balance_metric(&mother_age_differences)
            + Self::calculate_balance_metric(&father_age_differences))
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

        let params = crate::matching_quality::MatchingQualityParams {
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
        };

        crate::matching_quality::MatchingQuality::new(params)
    }

    fn calculate_balance_metric(diffs: &[i64]) -> f64 {
        #[allow(clippy::cast_precision_loss)]
        let mean = diffs.iter().sum::<i64>() as f64 / diffs.len() as f64;
        let variance = diffs
            .iter()
            .map(|&x| {
                #[allow(clippy::cast_precision_loss)]
                let diff = x as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / (diffs.len() - 1) as f64;

        mean / variance.sqrt()
    }

    /// Saves matched case-control pairs to a CSV file.
    ///
    /// # Errors
    /// Returns an error if file writing fails.
    pub fn save_matches_to_csv(
        &self,
        case_control_pairs: &[CaseControlPair],
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

                let mother_diff = match (case_dates.mother, control_dates.mother) {
                    (Some(m1), Some(m2)) => (m1 - m2).abs().to_string(),
                    _ => "NA".to_string(),
                };

                let father_diff = match (case_dates.father, control_dates.father) {
                    (Some(f1), Some(f2)) => (f1 - f2).abs().to_string(),
                    _ => "NA".to_string(),
                };

                wtr.write_record(&[
                    case_idx.to_string(),
                    case.pnr.clone(),
                    case.bday.to_string(),
                    case.treatment_date
                        .map_or("NA".to_string(), |d| d.to_string()),
                    control_idx.to_string(),
                    control.pnr.clone(),
                    control.bday.to_string(),
                    (case_dates.birth - control_dates.birth).abs().to_string(),
                    mother_diff,
                    father_diff,
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

        #[allow(clippy::cast_precision_loss)]
        let avg_controls = total_pairs as f64 / case_control_pairs.len() as f64;
        log::info!("Average controls per case: {:.2}", avg_controls);

        Ok(())
    }

    /// Saves matching statistics to a CSV file.
    ///
    /// # Errors
    /// Returns an error if file writing fails.
    pub fn save_matching_statistics(
        &self,
        case_control_pairs: &[CaseControlPair],
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
                        calculate_date_diff(case_dates.mother, control_dates.mother),
                        calculate_date_diff(case_dates.father, control_dates.father),
                    )
                })
                .fold((0, 0, 0, 0), |acc, (b, m, f)| {
                    (
                        acc.0 + b,
                        acc.1.max(b),
                        acc.2 + m.unwrap_or(0),
                        acc.3 + f.unwrap_or(0),
                    )
                });

            #[allow(clippy::cast_precision_loss)]
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

const fn calculate_date_diff(date1: Option<i64>, date2: Option<i64>) -> Option<i64> {
    match (date1, date2) {
        (Some(d1), Some(d2)) => Some((d1 - d2).abs()),
        _ => None,
    }
}

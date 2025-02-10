use chrono::NaiveDate;
use colored::*;
use humantime::format_duration;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use mimalloc::MiMalloc;
use rayon::prelude::*;
use rustc_hash::FxHashMap;
use serde::Deserialize;
use smallvec::SmallVec;
use std::error::Error;
use std::sync::Arc;
use thiserror::Error;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[derive(Error, Debug)]
pub enum SamplingError {
    #[error("Invalid date format")]
    InvalidDate,
    #[error("No eligible controls found for case")]
    NoEligibleControls,
    #[error("Invalid matching criteria")]
    InvalidCriteria,
}

#[derive(Debug, Deserialize)]
struct Record {
    pnr: String,
    #[serde(with = "date_format")]
    bday: NaiveDate,
    #[serde(with = "optional_date_format")]
    treatment_date: Option<NaiveDate>,
    #[serde(with = "date_format")]
    mother_bday: NaiveDate,
    #[serde(with = "date_format")]
    father_bday: NaiveDate,
}

mod date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(serde::de::Error::custom)
    }
}

mod optional_date_format {
    use chrono::NaiveDate;
    use serde::{self, Deserialize, Deserializer};

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<NaiveDate>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        if s == "NA" {
            Ok(None)
        } else {
            NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
    }
}

#[derive(Debug)]
struct MatchingCriteria {
    birth_date_window: i64,
    parent_date_window: i64,
}

impl MatchingCriteria {
    fn validate(&self) -> Result<(), SamplingError> {
        if self.birth_date_window <= 0 || self.parent_date_window <= 0 {
            return Err(SamplingError::InvalidCriteria);
        }
        Ok(())
    }
}

#[derive(Clone, Copy)]
struct DateData {
    birth: i64,
    mother: i64,
    father: i64,
}

struct IncidenceDensitySampler {
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

        // Process all records in parallel
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

        // Process the collected data sequentially
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

        Ok(IncidenceDensitySampler {
            dates,
            records: Arc::new(records),
            criteria,
            cases,
            sorted_controls: controls,
            birth_date_index: Arc::new(birth_date_index),
        })
    }

    fn select_random_controls(
        rng: &mut fastrand::Rng,
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
        let total_chunks = (total_cases + BATCH_SIZE - 1) / BATCH_SIZE;

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
                let mut rng = fastrand::Rng::new();
                let mut local_pairs = Vec::with_capacity(case_chunk.len());
                let mut eligible_buffer = SmallVec::<[usize; 32]>::new();

                for &case_idx in case_chunk {
                    let case_date = self.dates[case_idx];
                    let window_start = case_date.birth - self.criteria.birth_date_window;
                    let window_end = case_date.birth + self.criteria.birth_date_window;

                    eligible_buffer.clear();

                    for birth_date in window_start..=window_end {
                        if let Some(controls) = self.birth_date_index.get(&birth_date) {
                            for &control_idx in controls.iter() {
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

    pub fn verify_sampling_distribution(
        &self,
        case_control_pairs: &[(usize, SmallVec<[usize; 8]>)],
    ) -> String {
        let total_cases = case_control_pairs.len();
        let total_controls: usize = case_control_pairs
            .iter()
            .map(|(_, controls)| controls.len())
            .sum();

        let avg_controls_per_case = total_controls as f64 / total_cases as f64;
        let control_counts = case_control_pairs
            .iter()
            .map(|(_, controls)| controls.len())
            .collect::<Vec<_>>();

        let min_controls = control_counts.iter().min().unwrap_or(&0);
        let max_controls = control_counts.iter().max().unwrap_or(&0);

        format!(
            "{}\n\
            │ {} {}\n\
            │ {} {}\n\
            │ {} {:.2}\n\
            │ {} {}\n\
            │ {} {}\n\
            └────────────────────────────",
            "Distribution Statistics:".bold().green(),
            "Matched Cases:".bold(),
            total_cases.to_string().yellow(),
            "Total Controls:".bold(),
            total_controls.to_string().yellow(),
            "Controls per Case:".bold(),
            avg_controls_per_case,
            "Min Controls:".bold(),
            min_controls.to_string().yellow(),
            "Max Controls:".bold(),
            max_controls.to_string().yellow()
        )
    }

    pub fn evaluate_matching_quality(
        &self,
        case_control_pairs: &[(usize, SmallVec<[usize; 8]>)],
    ) -> MatchingQuality {
        let total_cases = self.cases.len();
        let matched_cases = case_control_pairs.len();
        let total_controls: usize = case_control_pairs
            .iter()
            .map(|(_, controls)| controls.len())
            .sum();

        // Initialize vectors to collect differences
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

        // Calculate balance metrics
        let birth_date_balance = self.calculate_balance_metric(&birth_date_differences);
        let parent_age_balance = (self.calculate_balance_metric(&mother_age_differences)
            + self.calculate_balance_metric(&father_age_differences))
            / 2.0;

        // Calculate percentiles
        let percentiles = vec![0.25, 0.50, 0.75];
        let birth_date_percentiles =
            MatchingQuality::calculate_percentiles(&birth_date_differences, &percentiles);
        let mother_age_percentiles =
            MatchingQuality::calculate_percentiles(&mother_age_differences, &percentiles);
        let father_age_percentiles =
            MatchingQuality::calculate_percentiles(&father_age_differences, &percentiles);

        MatchingQuality {
            total_cases,
            matched_cases,
            total_controls,
            avg_controls_per_case: total_controls as f64 / matched_cases as f64,
            birth_date_differences,
            mother_age_differences,
            father_age_differences,
            matching_rate: matched_cases as f64 / total_cases as f64,
            birth_date_balance,
            parent_age_balance,
            birth_date_percentiles,
            mother_age_percentiles,
            father_age_percentiles,
        }
    }

    fn calculate_balance_metric(&self, diffs: &[i64]) -> f64 {
        // Standardized mean difference or other balance metric
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
    ) -> Result<(), Box<dyn Error>> {
        println!("Saving matches to {}", filename);
        let mut wtr = csv::Writer::from_path(filename)?;

        // Write header
        wtr.write_record(&[
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

                // Calculate differences using the DateData struct
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
        println!("Successfully wrote matches to {}", filename);

        // Print some basic verification stats
        let total_pairs: usize = case_control_pairs
            .iter()
            .map(|(_, controls)| controls.len())
            .sum();
        println!("Total case-control pairs written: {}", total_pairs);
        println!(
            "Average controls per case: {:.2}",
            total_pairs as f64 / case_control_pairs.len() as f64
        );

        Ok(())
    }

    pub fn save_matching_statistics(
        &self,
        case_control_pairs: &[(usize, SmallVec<[usize; 8]>)],
        filename: &str,
    ) -> Result<(), Box<dyn Error>> {
        let mut wtr = csv::Writer::from_path(filename)?;

        // Write header
        wtr.write_record(&[
            "case_id",
            "n_controls",
            "avg_birth_diff",
            "max_birth_diff",
            "avg_mother_diff",
            "avg_father_diff",
        ])?;

        // Write detailed statistics for each case
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

#[derive(Debug)]
pub struct MatchingQuality {
    // Basic statistics
    total_cases: usize,
    matched_cases: usize,
    total_controls: usize,
    avg_controls_per_case: f64,

    // Distribution metrics
    birth_date_differences: Vec<i64>, // renamed for clarity
    mother_age_differences: Vec<i64>, // renamed for clarity
    father_age_differences: Vec<i64>, // renamed for clarity

    // Quality indicators
    matching_rate: f64,
    birth_date_balance: f64,
    parent_age_balance: f64,

    // Distribution statistics
    birth_date_percentiles: Vec<i64>,
    mother_age_percentiles: Vec<i64>,
    father_age_percentiles: Vec<i64>,
}

impl MatchingQuality {
    fn calculate_percentiles(values: &[i64], percentiles: &[f64]) -> Vec<i64> {
        let mut sorted_values = values.to_vec();
        sorted_values.sort_unstable();

        percentiles
            .iter()
            .map(|&p| {
                let idx = (p * (sorted_values.len() - 1) as f64).round() as usize;
                sorted_values[idx]
            })
            .collect()
    }

    pub fn plot_distributions(&self, output_file: &str) -> Result<(), Box<dyn Error>> {
        use plotters::prelude::*;

        let root = BitMapBackend::new(output_file, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        // Calculate histogram bins
        let max_diff = self
            .birth_date_differences
            .iter()
            .max()
            .copied()
            .unwrap_or(0);

        const N_BINS: usize = 50;
        let bin_size = (max_diff as f64 / N_BINS as f64).ceil() as i64;

        // Create histogram data
        let mut histogram_data = vec![0; N_BINS];
        for &diff in &self.birth_date_differences {
            let bin = ((diff as f64 / bin_size as f64).floor() as usize).min(N_BINS - 1);
            histogram_data[bin] += 1;
        }

        // Calculate y-axis max
        let max_count = *histogram_data.iter().max().unwrap_or(&1) as f64;

        // Create the chart
        let mut chart = ChartBuilder::on(&root)
            .caption("Distribution of Birth Date Differences", ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..N_BINS, 0f64..max_count * 1.1)?;

        // Customize the chart
        chart
            .configure_mesh()
            .x_desc("Difference in Days")
            .y_desc("Frequency")
            .draw()?;

        // Draw the histogram bars
        chart.draw_series(histogram_data.iter().enumerate().map(|(i, &count)| {
            let x0 = i;
            let x1 = i + 1;
            let y0 = 0.0;
            let y1 = count as f64;

            Rectangle::new([(x0, y0), (x1, y1)], RED.mix(0.3).filled())
        }))?;

        // Add labels for bin values
        chart
            .configure_mesh()
            .x_labels(20)
            .x_label_formatter(&|x| format!("{}", (*x as i64) * bin_size))
            .draw()?;

        // Optional: Add statistics to the plot
        let mean = self.birth_date_differences.iter().sum::<i64>() as f64
            / self.birth_date_differences.len() as f64;

        chart.draw_series(vec![Text::new(
            format!("Mean: {:.1} days", mean),
            (5, (max_count * 0.9) as f64),
            ("sans-serif", 20).into_font(),
        )])?;

        root.present()?;

        Ok(())
    }

    // Add helper method to create multiple plots
    pub fn plot_all_distributions(&self, base_filename: &str) -> Result<(), Box<dyn Error>> {
        // Plot birth date differences
        self.plot_distribution(
            &self.birth_date_differences,
            &format!("{}_birth.png", base_filename),
            "Birth Date Differences",
            "Difference in Days",
        )?;

        // Plot mother age differences
        self.plot_distribution(
            &self.mother_age_differences,
            &format!("{}_mother.png", base_filename),
            "Mother Age Differences",
            "Difference in Days",
        )?;

        // Plot father age differences
        self.plot_distribution(
            &self.father_age_differences,
            &format!("{}_father.png", base_filename),
            "Father Age Differences",
            "Difference in Days",
        )?;

        Ok(())
    }

    // Helper method for plotting individual distributions
    fn plot_distribution(
        &self,
        data: &[i64],
        filename: &str,
        title: &str,
        x_label: &str,
    ) -> Result<(), Box<dyn Error>> {
        use plotters::prelude::*;

        let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
        root.fill(&WHITE)?;

        // Calculate histogram bins
        let max_diff = data.iter().max().copied().unwrap_or(0);
        const N_BINS: usize = 50;
        let bin_size = (max_diff as f64 / N_BINS as f64).ceil() as i64;

        // Create histogram data
        let mut histogram_data = vec![0; N_BINS];
        for &value in data {
            let bin = ((value as f64 / bin_size as f64).floor() as usize).min(N_BINS - 1);
            histogram_data[bin] += 1;
        }

        let max_count = *histogram_data.iter().max().unwrap_or(&1) as f64;

        let mut chart = ChartBuilder::on(&root)
            .caption(title, ("sans-serif", 30))
            .margin(5)
            .x_label_area_size(40)
            .y_label_area_size(60)
            .build_cartesian_2d(0..N_BINS, 0f64..max_count * 1.1)?;

        chart
            .configure_mesh()
            .x_desc(x_label)
            .y_desc("Frequency")
            .x_labels(20)
            .x_label_formatter(&|x| format!("{}", (*x as i64) * bin_size))
            .draw()?;

        chart.draw_series(histogram_data.iter().enumerate().map(|(i, &count)| {
            Rectangle::new([(i, 0.0), (i + 1, count as f64)], RED.mix(0.3).filled())
        }))?;

        chart
            .configure_mesh()
            .x_labels(20)
            .x_label_formatter(&|x| format!("{}", (*x as i64) * bin_size))
            .draw()?;

        // Add statistics
        let mean = data.iter().sum::<i64>() as f64 / data.len() as f64;
        chart.draw_series(vec![Text::new(
            format!("Mean: {:.1} days", mean),
            (5, (max_count * 0.9) as f64),
            ("sans-serif", 20).into_font(),
        )])?;

        root.present()?;

        Ok(())
    }

    pub fn format_report(&self) -> String {
        use colored::*;

        let mut report = String::new();

        report.push_str(&format!("\n{}\n", "Matching Quality Report".bold().green()));
        report.push_str(&format!(
            "│ {} {}/{} ({:.1}%)\n",
            "Matching Rate:".bold(),
            self.matched_cases,
            self.total_cases,
            self.matching_rate * 100.0
        ));

        report.push_str(&format!(
            "│ {} {:.2}\n",
            "Average Controls per Case:".bold(),
            self.avg_controls_per_case
        ));

        // Add percentile information
        report.push_str("\nPercentiles (Birth Date Differences):\n");
        report.push_str(&format!(
            "  25th: {} days\n",
            self.birth_date_percentiles[0]
        ));
        report.push_str(&format!(
            "  50th: {} days\n",
            self.birth_date_percentiles[1]
        ));
        report.push_str(&format!(
            "  75th: {} days\n",
            self.birth_date_percentiles[2]
        ));

        // Add balance metrics
        report.push_str("\nBalance Metrics:\n");
        report.push_str(&format!(
            "  Birth Date Balance: {:.3}\n",
            self.birth_date_balance
        ));
        report.push_str(&format!(
            "  Parent Age Balance: {:.3}\n",
            self.parent_age_balance
        ));

        report
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    use std::time::Instant;

    let start = Instant::now();

    println!("Reading data...");
    let mut rdr = csv::Reader::from_path("data.csv")?;
    let records: Vec<Record> = rdr.deserialize().collect::<Result<_, _>>()?;
    println!("Data loaded in {:?}", start.elapsed());

    let criteria = MatchingCriteria {
        birth_date_window: 30,
        parent_date_window: 365,
    };

    println!("Initializing sampler...");
    let sampler_start = Instant::now();
    let sampler = IncidenceDensitySampler::new(records, criteria)?;
    println!("{}", sampler.get_statistics());
    println!("Sampler initialized in {:?}", sampler_start.elapsed());

    println!("Sampling controls...");
    let sampling_start = Instant::now();
    match sampler.sample_controls(4) {
        Ok(case_control_pairs) => {
            println!(
                "Sampling completed in {:?}. Found {} matches",
                sampling_start.elapsed(),
                case_control_pairs.len()
            );

            // Evaluate matching quality
            let quality = sampler.evaluate_matching_quality(&case_control_pairs);
            println!("{}", quality.format_report());

            // Plot distributions
            if let Err(e) = quality.plot_all_distributions("matching_distributions") {
                eprintln!("Error plotting distributions: {}", e);
            }

            println!("\nSaving results...");

            // Save matches to CSV
            if let Err(e) = sampler.save_matches_to_csv(&case_control_pairs, "matched_pairs.csv") {
                eprintln!("Error saving matches to CSV: {}", e);
            }

            // Save matching statistics
            if let Err(e) =
                sampler.save_matching_statistics(&case_control_pairs, "matching_stats.csv")
            {
                eprintln!("Error saving matching statistics: {}", e);
            }
        }
        Err(e) => eprintln!("Error sampling controls: {}", e),
    }

    println!("Total execution time: {:?}", start.elapsed());
    Ok(())
}

use crate::balance::BalanceResults;
use std::collections::HashMap;
use std::path::Path;

pub struct ComprehensiveReport {
    results: BalanceResults,
}

impl ComprehensiveReport {
    #[must_use]
    pub const fn new(results: BalanceResults) -> Self {
        Self { results }
    }

    /// Save all balance results to separate files
    ///
    /// # Errors
    /// Returns an error if there are issues writing to the output files
    pub fn save_to_files(&self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Save overall summaries
        self.save_overall_summaries(base_path)?;

        // Save missing data rates
        self.save_missing_rates(base_path)?;

        // Save matched pair summaries
        self.save_matched_pair_summaries(base_path)?;

        Ok(())
    }

    fn save_overall_summaries(&self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let path = base_path.with_file_name("covariate_balance.csv");
        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record([
            "Variable",
            "Mean (Cases)",
            "Mean (Controls)",
            "Standardized Difference",
            "Variance Ratio",
        ])?;

        for summary in &self.results.summaries {
            wtr.write_record([
                &summary.variable,
                &summary.mean_cases.to_string(),
                &summary.mean_controls.to_string(),
                &summary.std_diff.to_string(),
                &summary.variance_ratio.to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    fn save_missing_rates(&self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let path = base_path.with_file_name("missing_data_rates.csv");
        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record(["Variable", "Case Missing Rate", "Control Missing Rate"])?;

        for (var, (case_rate, control_rate)) in &self.results.missing_data_rates {
            wtr.write_record([var, &case_rate.to_string(), &control_rate.to_string()])?;
        }

        wtr.flush()?;
        Ok(())
    }

    fn save_matched_pair_summaries(
        &self,
        base_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let path = base_path.with_file_name("matched_pair_balance.csv");
        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record([
            "Case PNR",
            "Control PNR",
            "Treatment Date",
            "Variable",
            "Case Value",
            "Control Value",
            "Standardized Difference",
        ])?;

        for detail in &self.results.matched_pair_details {
            wtr.write_record([
                &detail.case_pnr,
                &detail.control_pnrs,
                &detail.treatment_date.to_string(),
                &detail.variable,
                &detail.case_value.to_string(),
                &detail.control_value.to_string(),
                &detail.std_diff.to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    pub fn summarize_matched_pair_balance(&self) -> HashMap<String, Vec<f64>> {
        let mut summaries: HashMap<String, Vec<f64>> = HashMap::new();

        for detail in &self.results.matched_pair_details {
            summaries
                .entry(detail.variable.clone())
                .or_default()
                .push(detail.std_diff);
        }

        summaries
    }

    pub fn save_matched_pair_summary(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record([
            "Variable",
            "Mean Std Diff",
            "Median Std Diff",
            "Max Std Diff",
            "Std Diff > 0.1 (%)",
            "N Pairs",
        ])?;

        let summaries = self.summarize_matched_pair_balance();

        for (variable, std_diffs) in summaries {
            let mean_diff = std_diffs.iter().sum::<f64>() / std_diffs.len() as f64;
            let median_diff = {
                let mut sorted = std_diffs.clone();
                sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
                sorted[sorted.len() / 2]
            };
            let max_diff = std_diffs.iter().fold(0.0f64, |a, &b| a.max(b.abs()));
            let large_diff_pct = std_diffs.iter().filter(|&&x| x.abs() > 0.1).count() as f64
                / std_diffs.len() as f64
                * 100.0;

            wtr.write_record([
                &variable,
                &format!("{:.3}", mean_diff),
                &format!("{:.3}", median_diff),
                &format!("{:.3}", max_diff),
                &format!("{:.1}", large_diff_pct),
                &std_diffs.len().to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
}

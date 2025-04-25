use crate::balance::BalanceResults;
use hashbrown::HashMap;
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

        // Save matched pair summary statistics
        self.save_matched_pair_summary(&base_path.join("matched_pair_summary.csv"))?;

        Ok(())
    }

    /// Generate plots for the balance analysis
    ///
    /// # Errors
    /// Returns an error if there are issues generating the plots
    pub fn generate_plots(&self, plots_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
        use std::fs;

        // Ensure plots directory exists
        fs::create_dir_all(plots_dir)?;

        // Generate standardized difference distribution plot
        self.generate_std_diff_distribution_plot(plots_dir)?;

        // Generate missing data rate plot
        self.generate_missing_data_plot(plots_dir)?;

        Ok(())
    }

    /// Generate a plot showing the distribution of standardized differences
    fn generate_std_diff_distribution_plot(
        &self,
        plots_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // This is a placeholder. In a real implementation, we would use a plotting library
        // like plotters to create visualization of standardized differences.

        // Create a simple text-based plot visualization for now
        let plot_path = plots_dir.join("std_diff_distribution.txt");
        let mut content = String::new();

        content.push_str("Standardized Difference Distribution (ASCII Visualization)\n");
        content.push_str("=====================================================\n\n");

        let summaries = self.summarize_matched_pair_balance();

        for (variable, std_diffs) in summaries {
            // Get basic stats
            let mean = std_diffs.iter().sum::<f64>() / std_diffs.len() as f64;

            // Create a simple histogram
            let mut histogram = [0; 11]; // -0.5 to 0.5 in 0.1 increments

            for &diff in &std_diffs {
                if (-0.5..=0.5).contains(&diff) {
                    let bin = ((diff + 0.5) / 0.1).floor() as usize;
                    let bin = bin.min(10); // Ensure we don't go out of bounds
                    histogram[bin] += 1;
                }
            }

            // Normalize for display
            let max_count = histogram.iter().copied().max().unwrap_or(1);
            let scale = 40.0 / f64::from(max_count);

            content.push_str(&format!("{variable} (mean: {mean:.3}):\n"));

            for (i, &count) in histogram.iter().enumerate() {
                let lower = -0.5 + i as f64 * 0.1;
                let upper = lower + 0.1;
                let bar_len = (f64::from(count) * scale).round() as usize;
                let bar = "#".repeat(bar_len);

                content.push_str(&format!(
                    "[{lower:.1}, {upper:.1}): {bar} {count}\n"
                ));
            }
            content.push('\n');
        }

        // Write to file
        std::fs::write(plot_path, content)?;

        Ok(())
    }

    /// Generate a plot showing missing data rates
    fn generate_missing_data_plot(
        &self,
        plots_dir: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        // Create a simple text-based plot for missing data rates
        let plot_path = plots_dir.join("missing_data_rates.txt");
        let mut content = String::new();

        content.push_str("Missing Data Rates (ASCII Visualization)\n");
        content.push_str("=====================================\n\n");

        for (var, (case_rate, control_rate)) in &self.results.missing_data_rates {
            let case_bar_len = (case_rate * 50.0).round() as usize;
            let control_bar_len = (control_rate * 50.0).round() as usize;

            let case_bar = "#".repeat(case_bar_len);
            let control_bar = "#".repeat(control_bar_len);

            content.push_str(&format!("{var}:\n"));
            content.push_str(&format!(
                "Cases:    {} {:.1}%\n",
                case_bar,
                case_rate * 100.0
            ));
            content.push_str(&format!(
                "Controls: {} {:.1}%\n\n",
                control_bar,
                control_rate * 100.0
            ));
        }

        // Write to file
        std::fs::write(plot_path, content)?;

        Ok(())
    }

    fn save_overall_summaries(&self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let path = base_path.join("covariate_balance.csv");
        let mut wtr = csv::Writer::from_path(path)?;

        // Enhanced CSV with category information
        wtr.write_record([
            "Variable",
            "Category",
            "Register",
            "Register Variable",
            "Mean (Cases)",
            "Mean (Controls)",
            "Standardized Difference",
            "Variance Ratio",
        ])?;

        // Map variables to their categories and register details
        for summary in &self.results.summaries {
            // Determine variable category and register info
            let (category, register, register_variable) =
                super::csv_report::categorize_variable(&summary.variable);

            wtr.write_record([
                &summary.variable,
                category,
                register,
                register_variable,
                &summary.mean_cases.to_string(),
                &summary.mean_controls.to_string(),
                &summary.std_diff.to_string(),
                &summary.variance_ratio.to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }

    // Using the categorize_variable in csv_report instead

    fn save_missing_rates(&self, base_path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let path = base_path.join("missing_data_rates.csv");
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
        let path = base_path.join("matched_pair_details.csv");
        let mut wtr = csv::Writer::from_path(path)?;

        wtr.write_record([
            "Case PNR",
            "Control PNRs",
            "Treatment Date",
            "Variable",
            "Case Value",
            "Control Value",
            "Standardized Difference",
        ])?;

        for detail in &self.results.matched_pair_details {
            wtr.write_record([
                &detail.case_pnr,
                &detail.control_pnrs.join(";"), // Join multiple control PNRs with semicolon
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

    #[must_use] pub fn summarize_matched_pair_balance(&self) -> HashMap<String, Vec<f64>> {
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
                &format!("{mean_diff:.3}"),
                &format!("{median_diff:.3}"),
                &format!("{max_diff:.3}"),
                &format!("{large_diff_pct:.1}"),
                &std_diffs.len().to_string(),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
}

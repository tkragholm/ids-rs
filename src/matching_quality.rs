use crate::plotting::{DefaultPlotter, Plottable};
use std::error::Error;

#[derive(Debug)]
pub struct MatchingQuality {
    pub stats: MatchingStats,
    plotting: Box<dyn Plottable>,
}

#[derive(Debug)]
pub struct MatchingStats {
    pub total_cases: usize,
    pub matched_cases: usize,
    pub total_controls: usize,
    pub avg_controls_per_case: f64,
    pub differences: MatchingDifferences,
    pub percentiles: MatchingPercentiles,
    pub balance: BalanceMetrics,
}

#[derive(Debug)]
pub struct MatchingDifferences {
    pub birth_date: Vec<i64>,
    pub mother_age: Vec<i64>,
    pub father_age: Vec<i64>,
}

#[derive(Debug)]
pub struct MatchingPercentiles {
    pub birth_date: Vec<i64>,
    pub mother_age: Vec<i64>,
    pub father_age: Vec<i64>,
}

#[derive(Debug)]
pub struct BalanceMetrics {
    pub birth_date: f64,
    pub parent_age: f64,
}

impl MatchingQuality {
    pub fn new(
        total_cases: usize,
        matched_cases: usize,
        total_controls: usize,
        birth_date_differences: Vec<i64>,
        mother_age_differences: Vec<i64>,
        father_age_differences: Vec<i64>,
        birth_date_balance: f64,
        parent_age_balance: f64,
        birth_date_percentiles: Vec<i64>,
        mother_age_percentiles: Vec<i64>,
        father_age_percentiles: Vec<i64>,
    ) -> Self {
        let stats = MatchingStats {
            total_cases,
            matched_cases,
            total_controls,
            avg_controls_per_case: total_controls as f64 / matched_cases as f64,
            differences: MatchingDifferences {
                birth_date: birth_date_differences,
                mother_age: mother_age_differences,
                father_age: father_age_differences,
            },
            percentiles: MatchingPercentiles {
                birth_date: birth_date_percentiles,
                mother_age: mother_age_percentiles,
                father_age: father_age_percentiles,
            },
            balance: BalanceMetrics {
                birth_date: birth_date_balance,
                parent_age: parent_age_balance,
            },
        };

        // Create default plotting implementation
        let plotting = Box::new(DefaultPlotter::new());

        MatchingQuality { stats, plotting }
    }

    pub fn calculate_percentiles(values: &[i64], percentiles: &[f64]) -> Vec<i64> {
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

    fn plot_all_distributions(&self, base_filename: &str) -> Result<(), Box<dyn Error>> {
        self.plotting
            .plot_distribution(
                &self.stats.differences.birth_date,
                &format!("{}_birth.png", base_filename),
                "Birth Date Differences",
                "Difference in Days",
            )
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.plotting
            .plot_distribution(
                &self.stats.differences.mother_age,
                &format!("{}_mother.png", base_filename),
                "Mother Age Differences",
                "Difference in Days",
            )
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        self.plotting
            .plot_distribution(
                &self.stats.differences.father_age,
                &format!("{}_father.png", base_filename),
                "Father Age Differences",
                "Difference in Days",
            )
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        Ok(())
    }

    pub fn generate_summary_plots(&self, output_dir: &str) -> Result<(), Box<dyn Error>> {
        std::fs::create_dir_all(output_dir)?;
        self.plot_all_distributions(&format!("{}/distributions", output_dir))?;

        let (utilization_rate, average_reuse) = {
            let total_matched_controls =
                self.stats.matched_cases as f64 * self.stats.avg_controls_per_case;
            let utilization_rate = total_matched_controls / self.stats.total_controls as f64;
            let average_reuse = if self.stats.total_controls > 0 {
                total_matched_controls / self.stats.total_controls as f64
            } else {
                0.0
            };
            (utilization_rate, average_reuse)
        };

        self.plotting
            .plot_utilization_summary(
                &format!("{}/utilization.png", output_dir),
                utilization_rate,
                average_reuse,
            )
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        Ok(())
    }

    pub fn format_report(&self) -> String {
        use colored::*;

        let mut report = String::new();

        report.push_str(&format!("\n{}\n", "Matching Quality Report".bold().green()));
        report.push_str(&format!(
            "│ {} {}/{} ({:.1}%)\n",
            "Matching Rate:".bold(),
            self.stats.matched_cases,
            self.stats.total_cases,
            (self.stats.matched_cases as f64 / self.stats.total_cases as f64) * 100.0
        ));

        report.push_str(&format!(
            "│ {} {}/{} ({:.1}%)\n",
            "Control Utilization:".bold(),
            self.stats.matched_cases * self.stats.avg_controls_per_case as usize,
            self.stats.total_controls,
            (self.stats.matched_cases as f64 * self.stats.avg_controls_per_case
                / self.stats.total_controls as f64)
                * 100.0
        ));

        report.push_str(&format!(
            "│ {} {:.2}\n",
            "Average Controls per Case:".bold(),
            self.stats.avg_controls_per_case
        ));

        report.push_str("\nPercentiles (Birth Date Differences):\n");
        report.push_str(&format!(
            "  25th: {} days\n",
            self.stats.percentiles.birth_date[0]
        ));
        report.push_str(&format!(
            "  50th: {} days\n",
            self.stats.percentiles.birth_date[1]
        ));
        report.push_str(&format!(
            "  75th: {} days\n",
            self.stats.percentiles.birth_date[2]
        ));

        report.push_str("\nPercentiles (Mother Age Differences):\n");
        report.push_str(&format!(
            "  25th: {} days\n",
            self.stats.percentiles.mother_age[0]
        ));
        report.push_str(&format!(
            "  50th: {} days\n",
            self.stats.percentiles.mother_age[1]
        ));
        report.push_str(&format!(
            "  75th: {} days\n",
            self.stats.percentiles.mother_age[2]
        ));

        report.push_str("\nPercentiles (Father Age Differences):\n");
        report.push_str(&format!(
            "  25th: {} days\n",
            self.stats.percentiles.father_age[0]
        ));
        report.push_str(&format!(
            "  50th: {} days\n",
            self.stats.percentiles.father_age[1]
        ));
        report.push_str(&format!(
            "  75th: {} days\n",
            self.stats.percentiles.father_age[2]
        ));

        report.push_str("\nBalance Metrics:\n");
        report.push_str(&format!(
            "  Birth Date Balance: {:.3}\n",
            self.stats.balance.birth_date
        ));
        report.push_str(&format!(
            "  Parent Age Balance: {:.3}\n",
            self.stats.balance.parent_age
        ));

        report
    }
}

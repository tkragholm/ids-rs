#[derive(Debug)]
pub struct MatchingQuality {
    pub stats: MatchingStats,
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

#[derive(Debug)]
pub struct MatchingQualityParams {
    pub total_cases: usize,
    pub matched_cases: usize,
    pub total_controls: usize,
    pub birth_date_differences: Vec<i64>,
    pub mother_age_differences: Vec<i64>,
    pub father_age_differences: Vec<i64>,
    pub birth_date_balance: f64,
    pub parent_age_balance: f64,
    pub birth_date_percentiles: Vec<i64>,
    pub mother_age_percentiles: Vec<i64>,
    pub father_age_percentiles: Vec<i64>,
}

impl MatchingQuality {
    #[must_use]
    pub fn new(params: MatchingQualityParams) -> Self {
        let stats = MatchingStats {
            total_cases: params.total_cases,
            matched_cases: params.matched_cases,
            total_controls: params.total_controls,
            avg_controls_per_case: params.total_controls as f64 / params.matched_cases as f64,
            differences: MatchingDifferences {
                birth_date: params.birth_date_differences,
                mother_age: params.mother_age_differences,
                father_age: params.father_age_differences,
            },
            percentiles: MatchingPercentiles {
                birth_date: params.birth_date_percentiles,
                mother_age: params.mother_age_percentiles,
                father_age: params.father_age_percentiles,
            },
            balance: BalanceMetrics {
                birth_date: params.birth_date_balance,
                parent_age: params.parent_age_balance,
            },
        };

        Self { stats }
    }

    #[must_use]
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

    #[must_use]
    pub fn format_report(&self) -> String {
        use crate::utils::console::ConsoleOutput;
        use colored::Colorize;

        let mut report = String::new();

        // Title
        report.push_str(&format!("\n{}\n", "Matching Quality Report".bold().green()));
        report.push_str(&format!("{}\n", "═".repeat(22).green()));

        // Matching metrics
        let matching_rate = self.stats.matched_cases as f64 / self.stats.total_cases as f64;
        let control_utilization = self.stats.matched_cases as f64
            * self.stats.avg_controls_per_case
            / self.stats.total_controls as f64;

        report.push_str(&format!(
            "│ {} {}/{} ({})\n",
            "Matching Rate:".bold(),
            self.stats.matched_cases.to_string().yellow(),
            self.stats.total_cases,
            ConsoleOutput::format_percentage(matching_rate)
        ));

        report.push_str(&format!(
            "│ {} {}/{} ({})\n",
            "Control Utilization:".bold(),
            (self.stats.matched_cases * self.stats.avg_controls_per_case as usize)
                .to_string()
                .yellow(),
            self.stats.total_controls,
            ConsoleOutput::format_percentage(control_utilization)
        ));

        report.push_str(&format!(
            "│ {} {}\n",
            "Average Controls per Case:".bold(),
            format!("{:.2}", self.stats.avg_controls_per_case).yellow()
        ));

        // Birth date differences
        report.push_str(&format!("\n{}\n", "Birth Date Differences".blue().bold()));
        report.push_str(&format!("{}\n", "─".repeat(21).blue()));
        report.push_str(&format!(
            "  25th percentile: {} days\n",
            self.stats.percentiles.birth_date[0].to_string().yellow()
        ));
        report.push_str(&format!(
            "  50th percentile: {} days\n",
            self.stats.percentiles.birth_date[1].to_string().yellow()
        ));
        report.push_str(&format!(
            "  75th percentile: {} days\n",
            self.stats.percentiles.birth_date[2].to_string().yellow()
        ));

        // Mother age differences
        report.push_str(&format!("\n{}\n", "Mother Age Differences".blue().bold()));
        report.push_str(&format!("{}\n", "─".repeat(21).blue()));
        report.push_str(&format!(
            "  25th percentile: {} days\n",
            self.stats.percentiles.mother_age[0].to_string().yellow()
        ));
        report.push_str(&format!(
            "  50th percentile: {} days\n",
            self.stats.percentiles.mother_age[1].to_string().yellow()
        ));
        report.push_str(&format!(
            "  75th percentile: {} days\n",
            self.stats.percentiles.mother_age[2].to_string().yellow()
        ));

        // Father age differences
        report.push_str(&format!("\n{}\n", "Father Age Differences".blue().bold()));
        report.push_str(&format!("{}\n", "─".repeat(21).blue()));
        report.push_str(&format!(
            "  25th percentile: {} days\n",
            self.stats.percentiles.father_age[0].to_string().yellow()
        ));
        report.push_str(&format!(
            "  50th percentile: {} days\n",
            self.stats.percentiles.father_age[1].to_string().yellow()
        ));
        report.push_str(&format!(
            "  75th percentile: {} days\n",
            self.stats.percentiles.father_age[2].to_string().yellow()
        ));

        // Balance metrics
        report.push_str(&format!("\n{}\n", "Balance Metrics".blue().bold()));
        report.push_str(&format!("{}\n", "─".repeat(14).blue()));

        // Color-code the balance metrics (lower is better)
        let birth_balance_str = format!("{:.3}", self.stats.balance.birth_date);
        let birth_balance = if self.stats.balance.birth_date < 0.1 {
            birth_balance_str.green()
        } else if self.stats.balance.birth_date < 0.2 {
            birth_balance_str.yellow()
        } else {
            birth_balance_str.red()
        };

        let parent_balance_str = format!("{:.3}", self.stats.balance.parent_age);
        let parent_balance = if self.stats.balance.parent_age < 0.1 {
            parent_balance_str.green()
        } else if self.stats.balance.parent_age < 0.2 {
            parent_balance_str.yellow()
        } else {
            parent_balance_str.red()
        };

        report.push_str(&format!("  Birth Date Balance: {birth_balance}\n"));
        report.push_str(&format!("  Parent Age Balance: {parent_balance}\n"));

        report
    }
}

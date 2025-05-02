use crate::balance::BalanceResults;

pub struct BalanceReport {
    results: BalanceResults,
}

impl BalanceReport {
    #[must_use]
    pub const fn new(results: BalanceResults) -> Self {
        Self { results }
    }

    #[must_use] pub fn generate_summary_statistics(&self) -> Vec<ReportRow> {
        self.results
            .summaries
            .iter()
            .map(|summary| {
                let (case_missing, control_missing) = self
                    .results
                    .missing_data_rates
                    .get(&summary.variable)
                    .unwrap_or(&(0.0, 0.0));

                ReportRow {
                    variable: summary.variable.clone(),
                    mean_cases: summary.mean_cases,
                    mean_controls: summary.mean_controls,
                    std_diff: summary.std_diff,
                    variance_ratio: summary.variance_ratio,
                    missing_cases: *case_missing,
                    missing_controls: *control_missing,
                }
            })
            .collect()
    }
}

#[derive(Debug)]
pub struct ReportRow {
    pub variable: String,
    pub mean_cases: f64,
    pub mean_controls: f64,
    pub std_diff: f64,
    pub variance_ratio: f64,
    pub missing_cases: f64,
    pub missing_controls: f64,
}

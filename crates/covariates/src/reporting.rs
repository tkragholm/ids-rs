use crate::balance::BalanceResults;
use std::path::Path;
use types::error::IdsError as CovariateError;

pub struct BalanceReport {
    results: BalanceResults,
}

impl BalanceReport {
    pub fn new(results: BalanceResults) -> Self {
        Self { results }
    }

    pub fn save_to_csv(&self, output_path: &Path) -> Result<(), CovariateError> {
        let mut wtr = csv::Writer::from_path(output_path)?;

        // Write summary statistics
        wtr.write_record([
            "Variable",
            "Mean (Cases)",
            "Mean (Controls)",
            "Std. Difference",
            "Variance Ratio",
            "Missing (Cases)",
            "Missing (Controls)",
        ])?;

        for summary in &self.results.summaries {
            let (case_missing, control_missing) = self
                .results
                .missing_data_rates
                .get(&summary.variable)
                .unwrap_or(&(0.0, 0.0));

            wtr.write_record([
                &summary.variable,
                &format!("{:.2}", summary.mean_cases),
                &format!("{:.2}", summary.mean_controls),
                &format!("{:.3}", summary.std_diff),
                &format!("{:.3}", summary.variance_ratio),
                &format!("{:.1}%", case_missing * 100.0),
                &format!("{:.1}%", control_missing * 100.0),
            ])?;
        }

        Ok(())
    }
}

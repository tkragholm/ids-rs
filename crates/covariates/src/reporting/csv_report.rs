use super::balance_report::BalanceReport;
use std::path::Path;
use types::error::IdsError;

pub trait CsvReport {
    fn save_to_csv(&self, path: &Path) -> Result<(), IdsError>;
}

impl CsvReport for BalanceReport {
    fn save_to_csv(&self, output_path: &Path) -> Result<(), IdsError> {
        let mut wtr = csv::Writer::from_path(output_path)?;

        wtr.write_record([
            "Variable",
            "Mean (Cases)",
            "Mean (Controls)",
            "Std. Difference",
            "Variance Ratio",
            "Missing (Cases)",
            "Missing (Controls)",
        ])?;

        for row in self.generate_summary_statistics() {
            wtr.write_record([
                &row.variable,
                &format!("{:.2}", row.mean_cases),
                &format!("{:.2}", row.mean_controls),
                &format!("{:.3}", row.std_diff),
                &format!("{:.3}", row.variance_ratio),
                &format!("{:.1}%", row.missing_cases * 100.0),
                &format!("{:.1}%", row.missing_controls * 100.0),
            ])?;
        }

        wtr.flush()?;
        Ok(())
    }
}

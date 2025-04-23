use super::balance_report::BalanceReport;
use std::path::Path;
use types::error::IdsError;

pub trait CsvReport {
    fn save_to_csv(&self, path: &Path) -> Result<(), IdsError>;
}

// Helper function to categorize variables - to be reused across modules
pub fn categorize_variable(variable: &str) -> (&'static str, &'static str, &'static str) {
    match variable {
        // Demographics variables (BEF)
        v if v.contains("Family Size") => ("Demographics", "BEF", "ANTPERSF/ANTPERSH"),
        v if v.contains("Municipality") => ("Demographics", "BEF", "KOM"),
        v if v.contains("Family Type") => ("Demographics", "BEF", "FAMILIE_TYPE"),
        v if v.contains("Civil Status") => ("Demographics", "BEF", "CIVST"),
        v if v.contains("Gender") => ("Demographics", "BEF", "KOEN"),
        v if v.contains("Citizenship") => ("Demographics", "BEF", "STATSB"),
        v if v.contains("Age") => ("Demographics", "BEF", "ALDER"),
        v if v.contains("Children Count") => ("Demographics", "BEF", "ANTBOERNF/ANTBOERNH"),

        // Income variables (IND)
        "Income" => ("Income", "IND", "PERINDKIALT_13"),
        v if v.contains("Wage Income") => ("Income", "IND", "LOENMV_13"),
        v if v.contains("Employment Status") => ("Income", "IND", "BESKST13"),

        // Occupation variables (AKM)
        v if v.contains("SOCIO13") => ("Occupation", "AKM", "SOCIO13"),
        v if v == "SOCIO" || v.contains("SOCIO Category") => ("Occupation", "AKM", "SOCIO"),
        v if v.contains("SOCIO02") => ("Occupation", "AKM", "SOCIO02"),
        v if v.contains("Previous Socioeconomic") => ("Occupation", "IND", "PRE_SOCIO"),
        v if v.contains("Classification System") => ("Occupation", "AKM", "Classification"),

        // Education variables
        v if v.contains("Education") || v.contains("ISCED") => ("Education", "UDDA", "Education"),

        // Default case
        _ => ("Other", "", ""),
    }
}

impl CsvReport for BalanceReport {
    fn save_to_csv(&self, output_path: &Path) -> Result<(), IdsError> {
        let mut wtr = csv::Writer::from_path(output_path)?;

        // Enhanced CSV format with categories
        wtr.write_record([
            "Variable",
            "Category",
            "Register",
            "Register Variable",
            "Mean (Cases)",
            "Mean (Controls)",
            "Std. Difference",
            "Variance Ratio",
            "Missing (Cases)",
            "Missing (Controls)",
        ])?;

        for row in self.generate_summary_statistics() {
            // Categorize the variable
            let (category, register, register_var) = categorize_variable(&row.variable);

            wtr.write_record([
                &row.variable,
                category,
                register,
                register_var,
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

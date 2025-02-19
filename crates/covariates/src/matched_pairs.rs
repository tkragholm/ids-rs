use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

/// Represents a matched pair record with case and control information
#[derive(Deserialize, Debug)]
pub struct MatchedPairRecord {
    pub case_id: String,
    pub case_pnr: String,
    #[serde(with = "core::utils::date_format")]
    pub case_birth_date: NaiveDate,
    #[serde(with = "core::utils::date_format")]
    pub case_treatment_date: NaiveDate,
    pub control_id: String,
    pub control_pnr: String,
    #[serde(with = "core::utils::date_format")]
    pub control_birth_date: NaiveDate,
    pub birth_date_diff_days: i64,
    pub mother_age_diff_days: i64,
    pub father_age_diff_days: i64,
}

/// Type alias for the matched pairs result
type MatchedPairsResult = Vec<(String, NaiveDate, Vec<String>)>;

/// Loads matched pairs from a CSV file.
///
/// # Arguments
/// * `path` - Path to the CSV file containing matched pairs data
///
/// # Returns
/// A vector of tuples containing (case_pnr, treatment_date, control_pnrs)
///
/// # Errors
/// Returns an error if:
/// * The file cannot be read
/// * The CSV format is invalid
/// * Required fields are missing
/// * Date parsing fails
pub fn load_matched_pairs(path: &Path) -> Result<MatchedPairsResult, Box<dyn std::error::Error>> {
    let mut reader = csv::Reader::from_path(path)?;
    let mut pairs: HashMap<(String, NaiveDate), Vec<String>> = HashMap::new();

    for result in reader.records() {
        let record = result?;

        // Extract case information
        let case_pnr = record.get(1).ok_or("Missing case_pnr")?.to_string();
        let treatment_date = NaiveDate::parse_from_str(
            record.get(3).ok_or("Missing case_treatment_date")?,
            "%Y-%m-%d",
        )?;

        // Extract control information
        let control_pnr = record.get(5).ok_or("Missing control_pnr")?.to_string();

        // Add to pairs HashMap
        pairs
            .entry((case_pnr.clone(), treatment_date))
            .or_default()
            .push(control_pnr);
    }

    // Convert HashMap to Vec
    Ok(pairs
        .into_iter()
        .map(|((case_pnr, date), controls)| (case_pnr, date, controls))
        .collect())
}

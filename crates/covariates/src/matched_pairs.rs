use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;

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

pub fn load_matched_pairs(
    path: &Path,
) -> Result<Vec<(String, NaiveDate, Vec<String>)>, Box<dyn std::error::Error>> {
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

// fn deserialize_control_ids<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let s: String = serde::Deserialize::deserialize(deserializer)?;
//     Ok(s.split(',').map(String::from).collect())
// }

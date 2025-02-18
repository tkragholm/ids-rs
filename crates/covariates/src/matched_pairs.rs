use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::HashMap;
use std::path::Path;
use types::error::IdsError as CovariateError;

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
    file_path: &Path,
) -> Result<Vec<(String, NaiveDate, Vec<String>)>, CovariateError> {
    let mut rdr = csv::Reader::from_path(file_path)?;
    let mut pairs_map: HashMap<(String, NaiveDate), Vec<String>> = HashMap::new();

    for result in rdr.deserialize() {
        let record: MatchedPairRecord = result?;

        // Use case_id and case_treatment_date as the key
        let key = (record.case_id.clone(), record.case_treatment_date);
        pairs_map.entry(key).or_default().push(record.control_id);
    }

    // Convert the map to the required format
    Ok(pairs_map
        .into_iter()
        .map(|((case_id, treatment_date), control_ids)| (case_id, treatment_date, control_ids))
        .collect())
}

// fn deserialize_control_ids<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
// where
//     D: serde::Deserializer<'de>,
// {
//     let s: String = serde::Deserialize::deserialize(deserializer)?;
//     Ok(s.split(',').map(String::from).collect())
// }

// Add missing functions
pub fn is_case(id: &str) -> bool {
    // Implement based on your ID format
    id.starts_with("C")
}

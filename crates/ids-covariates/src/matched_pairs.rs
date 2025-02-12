use crate::error::CovariateError;
use chrono::NaiveDate;
use serde::Deserialize;
use std::path::Path;

#[derive(Deserialize)]
pub struct MatchedPair {
    pub case_id: String,
    #[serde(with = "ids_core::utils::date_format")]
    pub index_date: NaiveDate,
    #[serde(deserialize_with = "deserialize_control_ids")]
    pub control_ids: Vec<String>,
}

pub fn load_matched_pairs(
    file_path: &Path,
) -> Result<Vec<(String, NaiveDate, Vec<String>)>, CovariateError> {
    let mut rdr = csv::Reader::from_path(file_path).map_err(|e| {
        CovariateError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("Failed to open matched pairs file: {}", e),
        ))
    })?;
    let mut pairs = Vec::new();

    for result in rdr.deserialize() {
        let record: MatchedPair = result?;
        pairs.push((record.case_id, record.index_date, record.control_ids));
    }

    Ok(pairs)
}

fn deserialize_control_ids<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = serde::Deserialize::deserialize(deserializer)?;
    Ok(s.split(',').map(String::from).collect())
}

// Add missing functions
pub fn is_case(id: &str) -> bool {
    // Implement based on your ID format
    id.starts_with("C")
}

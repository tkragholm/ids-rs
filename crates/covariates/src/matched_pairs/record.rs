use chrono::NaiveDate;
use serde::{Deserialize, Deserializer};

#[derive(Deserialize, Debug, Clone)]
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
    #[serde(deserialize_with = "deserialize_optional_i64")]
    pub mother_age_diff_days: i64,
    #[serde(deserialize_with = "deserialize_optional_i64")]
    pub father_age_diff_days: i64,
}

// Function to deserialize columns that could be NA or empty as 0
fn deserialize_optional_i64<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let s: Option<String> = Option::deserialize(deserializer)?;
    match s {
        Some(s) if s.is_empty() || s.to_uppercase() == "NA" => Ok(0),
        Some(s) => s.parse::<i64>().map_err(serde::de::Error::custom),
        None => Ok(0),
    }
}

impl MatchedPairRecord {
    pub fn get_case_info(&self) -> (&str, NaiveDate) {
        (&self.case_pnr, self.case_treatment_date)
    }

    pub fn get_control_info(&self) -> (&str, NaiveDate) {
        (&self.control_pnr, self.case_treatment_date)
    }
}

/// Represents a single control for a case
#[derive(Debug, Clone)]
pub struct Control {
    pub id: String,
    pub pnr: String,
    pub birth_date: NaiveDate,
    pub birth_date_diff: i64,
    pub mother_age_diff: Option<i64>,
    pub father_age_diff: Option<i64>,
}

/// Represents a case with multiple controls
#[derive(Debug, Clone)]
pub struct CaseWithControls {
    pub case_id: String,
    pub case_pnr: String,
    pub case_birth_date: NaiveDate,
    pub case_treatment_date: NaiveDate,
    pub controls: Vec<Control>,
}

impl CaseWithControls {
    /// Convert a list of MatchedPairRecords into a list of CaseWithControls,
    /// grouping by case_id
    pub fn from_matched_pair_records(records: &[MatchedPairRecord]) -> Vec<Self> {
        let mut case_map: hashbrown::HashMap<String, CaseWithControls> = hashbrown::HashMap::new();
        
        for record in records {
            let case_id = record.case_id.clone();
            
            // Convert the control part of the MatchedPairRecord to a Control
            let control = Control {
                id: record.control_id.clone(),
                pnr: record.control_pnr.clone(),
                birth_date: record.control_birth_date,
                birth_date_diff: record.birth_date_diff_days,
                mother_age_diff: Some(record.mother_age_diff_days),
                father_age_diff: Some(record.father_age_diff_days),
            };
            
            // If we've seen this case before, add this control to its list
            if let Some(case_entry) = case_map.get_mut(&case_id) {
                case_entry.controls.push(control);
            } else {
                // Otherwise, create a new CaseWithControls
                let case_entry = CaseWithControls {
                    case_id: record.case_id.clone(),
                    case_pnr: record.case_pnr.clone(),
                    case_birth_date: record.case_birth_date,
                    case_treatment_date: record.case_treatment_date,
                    controls: vec![control],
                };
                case_map.insert(case_id, case_entry);
            }
        }
        
        case_map.into_values().collect()
    }
}

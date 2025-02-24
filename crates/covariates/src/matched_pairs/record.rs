use chrono::NaiveDate;
use serde::Deserialize;

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
    pub mother_age_diff_days: i64,
    pub father_age_diff_days: i64,
}

impl MatchedPairRecord {
    pub fn get_case_info(&self) -> (&str, NaiveDate) {
        (&self.case_pnr, self.case_treatment_date)
    }

    pub fn get_control_info(&self) -> (&str, NaiveDate) {
        (&self.control_pnr, self.case_treatment_date)
    }
}

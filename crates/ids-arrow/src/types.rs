use chrono::NaiveDate;

#[derive(Debug, Clone)]
pub struct CovariateSnapshot {
    pub date: NaiveDate,
    pub income: Option<f64>,
    pub education: Option<String>,
    pub socioeconomic_status: Option<i32>,
    pub family_size: Option<i32>,
    pub municipality: Option<i32>,
    pub family_type: Option<String>,
    pub immigrant_background: Option<String>,
    pub father_income: Option<f64>,
    pub father_education: Option<String>,
    pub father_socioeconomic_status: Option<i32>,
    pub mother_income: Option<f64>,
    pub mother_education: Option<String>,
    pub mother_socioeconomic_status: Option<i32>,
}

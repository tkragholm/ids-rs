use crate::{error::CovariateError, models::*};

pub struct CovariateLoader {
    education_path: String,
    income_path: String,
    occupation_path: String,
}

impl CovariateLoader {
    pub fn new(education_path: String, income_path: String, occupation_path: String) -> Self {
        Self {
            education_path,
            income_path,
            occupation_path,
        }
    }

    pub fn load_education(&self) -> Result<Vec<TimeVaryingValue<Education>>, CovariateError> {
        let mut rdr = csv::Reader::from_path(&self.education_path)?;
        let mut records = Vec::new();

        for result in rdr.deserialize() {
            let record: TimeVaryingValue<Education> = result.map_err(|e| {
                CovariateError::InvalidFormat(format!("Failed to parse education record: {}", e))
            })?;
            records.push(record);
        }

        Ok(records)
    }

    pub fn load_income(&self) -> Result<Vec<TimeVaryingValue<Income>>, CovariateError> {
        let mut rdr = csv::Reader::from_path(&self.income_path)?;
        let mut records = Vec::new();

        for result in rdr.deserialize() {
            let record: TimeVaryingValue<Income> = result.map_err(|e| {
                CovariateError::InvalidFormat(format!("Failed to parse income record: {}", e))
            })?;
            records.push(record);
        }

        Ok(records)
    }

    pub fn load_occupation(&self) -> Result<Vec<TimeVaryingValue<Occupation>>, CovariateError> {
        let mut rdr = csv::Reader::from_path(&self.occupation_path)?;
        let mut records = Vec::new();

        for result in rdr.deserialize() {
            let record: TimeVaryingValue<Occupation> = result.map_err(|e| {
                CovariateError::InvalidFormat(format!("Failed to parse occupation record: {}", e))
            })?;
            records.push(record);
        }

        Ok(records)
    }
}

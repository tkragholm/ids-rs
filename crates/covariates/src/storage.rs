use crate::translation::TranslationMaps;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct CovariateData {
    pub pnr: String,
    pub date: NaiveDate,
    pub covariate_type: CovariateType,
    pub value: CovariateValue,
    pub translated_value: Option<String>,
}

pub fn save_covariates(
    data: &[CovariateData],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, data)?;
    Ok(())
}

pub struct CovariateStorage {
    translations: TranslationMaps,
    data: Vec<CovariateData>,
}

impl CovariateStorage {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            translations: TranslationMaps::new()?,
            data: Vec::new(),
        })
    }

    pub fn add_covariate(&mut self, mut covariate: CovariateData) {
        // Translate values based on covariate type
        covariate.translated_value = match &covariate.value {
            CovariateValue::Demographics { statsb, .. } => self
                .translations
                .translate_statsb(&statsb.to_string())
                .map(String::from),
            // Add more translations as needed
            _ => None,
        };

        self.data.push(covariate);
    }

    pub fn save_to_csv(&self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        let mut writer = csv::Writer::from_path(path)?;

        writer.write_record(&[
            "PNR",
            "Date",
            "Covariate Type",
            "Raw Value",
            "Translated Value",
        ])?;

        for covariate in &self.data {
            writer.write_record(&[
                &covariate.pnr,
                &covariate.date.to_string(),
                &format!("{:?}", covariate.covariate_type),
                &format!("{:?}", covariate.value),
                covariate.translated_value.as_deref().unwrap_or(""),
            ])?;
        }

        writer.flush()?;
        Ok(())
    }
}

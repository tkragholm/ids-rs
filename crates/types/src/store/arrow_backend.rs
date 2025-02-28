use crate::{
    arrow_utils::ArrowAccess,
    error::IdsError,
    family::FamilyRelations,
    family::FamilyStore,
    models::{Covariate, CovariateType, TimeVaryingValue},
    translation::TranslationMaps,
};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ArrowStore {
    family_data: HashMap<String, FamilyRelations>,
    akm_data: HashMap<i32, Vec<RecordBatch>>,
    bef_data: HashMap<String, Vec<RecordBatch>>,
    ind_data: HashMap<i32, Vec<RecordBatch>>,
    uddf_data: HashMap<String, Vec<RecordBatch>>,
    translations: TranslationMaps,
}

impl ArrowStore {
    pub fn new() -> Result<Self, IdsError> {
        // Try to load translation maps, if they fail, use empty maps but log the error
        let translations = match TranslationMaps::new() {
            Ok(maps) => {
                log::info!("Successfully loaded translation maps");
                maps
            },
            Err(e) => {
                log::warn!("Failed to load translation maps: {}. Using empty maps instead.", e);
                log::warn!("This is likely due to missing mapping files - proceeding with empty translations");
                log::info!("Translation map error details: {}", e);
                TranslationMaps::new_empty()
            }
        };

        log::info!("Creating new ArrowStore");
        
        Ok(Self {
            family_data: HashMap::new(),
            akm_data: HashMap::new(),
            bef_data: HashMap::new(),
            ind_data: HashMap::new(),
            uddf_data: HashMap::new(),
            translations,
        })
    }

    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        self.akm_data.insert(year, batches);
    }

    pub fn add_bef_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        self.bef_data.insert(period, batches);
    }

    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        self.ind_data.insert(year, batches);
    }

    pub fn add_uddf_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        self.uddf_data.insert(period, batches);
    }

    fn get_education(&self, pnr: &str, date: NaiveDate) -> Result<Option<Covariate>, IdsError> {
        // Find the closest UDDF data period before the given date
        let period = self.find_closest_period(date, &self.uddf_data)?;

        if let Some(batches) = period.and_then(|p| self.uddf_data.get(p)) {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    let level: Option<String> = self.get_value(batch, "HFAUDD", idx)?;
                    if let Some(level) = level {
                        return Ok(Some(Covariate::education(level, None, None)));
                    }
                }
            }
        }
        Ok(None)
    }

    fn get_income(&self, pnr: &str, date: NaiveDate) -> Result<Option<Covariate>, IdsError> {
        use chrono::Datelike;
        let year = date.year();
        if let Some(batches) = self.ind_data.get(&year) {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    let amount: Option<f64> = self.get_value(batch, "PERINDKIALT_13", idx)?;
                    if let Some(amount) = amount {
                        return Ok(Some(Covariate::income(
                            amount,
                            "DKK".to_string(),
                            "PERINDKIALT_13".to_string(),
                        )));
                    }
                }
            }
        }
        Ok(None)
    }

    fn get_demographics(&self, pnr: &str, date: NaiveDate) -> Result<Option<Covariate>, IdsError> {
        let period = self.find_closest_period(date, &self.bef_data)?;

        if let Some(batches) = period.and_then(|p| self.bef_data.get(p)) {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    let family_size: Option<i32> = self.get_value(batch, "ANTPERSF", idx)?;
                    let municipality: Option<i32> = self.get_value(batch, "KOM", idx)?;
                    let family_type: Option<i32> = self.get_value(batch, "FAMILIE_TYPE", idx)?;
                    let statsb: Option<i32> = self.get_value(batch, "STATSB", idx)?;

                    if let (Some(family_size), Some(municipality), Some(family_type)) =
                        (family_size, municipality, family_type)
                    {
                        let mut covariate = Covariate::demographics(
                            family_size,
                            municipality,
                            family_type.to_string(),
                        );

                        // Add translated values to metadata
                        if let Some(statsb) = statsb {
                            if let Some(translated) = self.translations.translate(
                                crate::translation::TranslationType::Statsb,
                                &statsb.to_string(),
                            ) {
                                covariate.metadata.insert(
                                    "statsb_translated".to_string(),
                                    translated.to_string(),
                                );
                            }
                        }

                        return Ok(Some(covariate));
                    }
                }
            }
        }
        Ok(None)
    }

    fn find_closest_period<'a>(
        &self,
        date: NaiveDate,
        data: &'a HashMap<String, Vec<RecordBatch>>,
    ) -> Result<Option<&'a String>, IdsError> {
        Ok(data
            .keys()
            .filter(|p| {
                let year: i32 = p[0..4].parse().unwrap_or(0);
                let month: i32 = if p.len() > 4 {
                    p[4..6].parse().unwrap_or(0)
                } else {
                    12
                };
                NaiveDate::from_ymd_opt(year, month as u32, 1)
                    .map(|period_date| period_date <= date)
                    .unwrap_or(false)
            })
            .max_by_key(|p| p.len()))
    }

    pub fn load_family_relations(
        &mut self,
        family_batches: Vec<RecordBatch>,
    ) -> Result<(), IdsError> {
        let mut family_store = FamilyStore::new(self.clone());
        family_store.load_family_relations(family_batches)?;
        self.family_data = family_store.relations;
        Ok(())
    }
}

impl super::Store for ArrowStore {
    fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        match covariate_type {
            CovariateType::Education => self.get_education(pnr, date),
            CovariateType::Income => self.get_income(pnr, date),
            CovariateType::Demographics => self.get_demographics(pnr, date),
            CovariateType::Occupation => Ok(None), // Implement if needed
        }
    }

    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.family_data.get(pnr)
    }

    fn load_data(&mut self, _data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        Err(IdsError::InvalidOperation(
            "Cannot load time-varying data into Arrow store".to_string(),
        ))
    }
}

use crate::{
    arrow_utils::traits::{ArrowAccess, ArrowDataHelper},
    arrow_utils::types::ArrowPrimitive,
    error::IdsError,
    family::{FamilyRelations, FamilyStore},
    models::{Education, Income, Occupation, TimeVaryingValue},
    snapshot::CovariateSnapshot,
    traits::{DataAccess, FamilyAccess, Store},
};
use arrow::record_batch::RecordBatch;
use chrono::Datelike;
use chrono::NaiveDate;
use log::{debug, warn};
use std::collections::HashMap;

#[derive(Clone)]
pub struct ArrowStore {
    pub family_store: Option<Box<FamilyStore>>,
    pub akm_data: HashMap<i32, Vec<RecordBatch>>,
    pub bef_data: HashMap<String, Vec<RecordBatch>>,
    pub ind_data: HashMap<i32, Vec<RecordBatch>>,
    pub uddf_data: HashMap<String, Vec<RecordBatch>>,
}

impl Default for ArrowStore {
    fn default() -> Self {
        Self::new()
    }
}

impl ArrowStore {
    #[must_use]
    pub fn new() -> Self {
        Self {
            family_store: None,
            akm_data: HashMap::new(),
            bef_data: HashMap::new(),
            ind_data: HashMap::new(),
            uddf_data: HashMap::new(),
        }
    }

    fn validate_batch_schema(
        &self,
        batch: &RecordBatch,
        expected_columns: &[&str],
    ) -> Result<(), IdsError> {
        let schema = batch.schema();
        debug!("Batch schema: {:?}", schema);

        for &col in expected_columns {
            if !schema.fields().iter().any(|f| f.name() == col) {
                return Err(IdsError::InvalidFormat(format!("Missing column: {col}")));
            }
        }
        Ok(())
    }

    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        for batch in &batches {
            if let Err(e) = self.validate_batch_schema(batch, &["PNR", "SOCIO13"]) {
                log::warn!("Invalid AKM batch schema for year {}: {}", year, e);
                return;
            }
        }
        self.akm_data.insert(year, batches);
    }

    pub fn add_bef_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        for batch in &batches {
            if let Err(e) =
                self.validate_batch_schema(batch, &["PNR", "ANTPERSF", "KOM", "FAMILIE_TYPE"])
            {
                warn!("Invalid BEF batch schema for period {}: {}", period, e);
                return;
            }
        }
        self.bef_data.insert(period, batches);
    }

    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        for batch in &batches {
            if let Err(e) = self.validate_batch_schema(batch, &["PNR", "PERINDKIALT_13"]) {
                warn!("Invalid IND batch schema for year {}: {}", year, e);
                return;
            }
        }
        self.ind_data.insert(year, batches);
    }

    pub fn add_uddf_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        for batch in &batches {
            if let Err(e) = self.validate_batch_schema(batch, &["PNR", "HFAUDD"]) {
                warn!("Invalid UDDF batch schema for period {}: {}", period, e);
                return;
            }
        }
        self.uddf_data.insert(period, batches);
    }

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        let mut store = FamilyStore::new(self.clone());
        store.load_family_relations(batches)?;
        self.family_store = Some(Box::new(store));
        Ok(())
    }
}

impl Store for ArrowStore {
    fn load_education(&self, _data: Vec<TimeVaryingValue<Education>>) -> Result<(), IdsError> {
        Ok(())
    }

    fn load_income(&self, _data: Vec<TimeVaryingValue<Income>>) -> Result<(), IdsError> {
        Ok(())
    }

    fn load_occupation(&self, _data: Vec<TimeVaryingValue<Occupation>>) -> Result<(), IdsError> {
        Ok(())
    }
}

impl FamilyAccess for ArrowStore {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.family_store
            .as_ref()
            .and_then(|store| store.get_family_relations(pnr))
    }

    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)> {
        self.get_family_relations(pnr)
            .map(|relations| (relations.father_id.clone(), relations.mother_id.clone()))
    }

    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate> {
        self.get_family_relations(pnr)
            .map(|relations| relations.birth_date)
    }
}

impl DataAccess for ArrowStore {
    fn get_covariates_at_date(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<CovariateSnapshot, IdsError> {
        use crate::traits::DateHelpers;

        let year = date.year();
        let quarter = date.get_quarter();
        let period = format!("{}{:02}", year, quarter * 3);

        debug!(
            "Looking up covariates for PNR {} at date {} (period {})",
            pnr, date, period
        );

        let mut snapshot = CovariateSnapshot::new(date);

        // Extract AKM data (socioeconomic status)
        if let Some(batches) = self.akm_data.get(&year) {
            log::debug!("Found {} AKM batches for year {}", batches.len(), year);
            for (i, batch) in batches.iter().enumerate() {
                log::debug!("Checking AKM batch {} with {} rows", i, batch.num_rows());
                if let Ok(Some(idx)) = self.find_pnr_index(batch, pnr) {
                    log::debug!("Found PNR in AKM batch {} at index {}", i, idx);
                    if let Ok(Some(socio13)) = self.get_value_at_index::<i32>(batch, "SOCIO13", idx)
                    {
                        log::debug!("Found SOCIO13 value: {}", socio13);
                        snapshot.socioeconomic_status = Some(Occupation {
                            code: socio13.to_string(),
                            classification: "SOCIO13".to_string(),
                        });
                    }
                }
            }
        }

        // Extract IND data (income)
        if let Some(batches) = self.ind_data.get(&year) {
            debug!("Found {} IND batches for year {}", batches.len(), year);
            for batch in batches {
                if let Ok(Some(idx)) = self.find_pnr_index(batch, pnr) {
                    debug!("Found PNR in IND data at index {}", idx);
                    if let Ok(Some(income)) =
                        self.get_value_at_index::<f64>(batch, "PERINDKIALT_13", idx)
                    {
                        snapshot.income = Some(Income {
                            amount: income,
                            currency: "DKK".to_string(),
                            type_code: "PERINDKIALT_13".to_string(),
                        });
                    }
                }
            }
        }

        // Extract education data from UDDF
        if let Some(batches) = self.uddf_data.get(&period) {
            debug!("Found {} UDDF batches for period {}", batches.len(), period);
            for batch in batches {
                if let Ok(Some(idx)) = self.find_pnr_index(batch, pnr) {
                    debug!("Found PNR in UDDF data at index {}", idx);
                    if let Ok(Some(level)) = self.get_value_at_index::<String>(batch, "HFAUDD", idx)
                    {
                        snapshot.education = Some(Education {
                            level,
                            isced_code: None,
                            years: None,
                        });
                    }
                }
            }
        }

        // Extract demographics from BEF
        if let Some(batches) = self.bef_data.get(&period) {
            debug!("Found {} BEF batches for period {}", batches.len(), period);
            for batch in batches {
                if let Ok(Some(idx)) = self.find_pnr_index(batch, pnr) {
                    debug!("Found PNR in BEF data at index {}", idx);
                    if let Ok(Some(family_size)) =
                        self.get_value_at_index::<i32>(batch, "ANTPERSF", idx)
                    {
                        snapshot.family_size = Some(family_size);
                    }
                    if let Ok(Some(municipality)) =
                        self.get_value_at_index::<i32>(batch, "KOM", idx)
                    {
                        snapshot.municipality = Some(municipality);
                    }
                    if let Ok(Some(family_type)) =
                        self.get_value_at_index::<String>(batch, "FAMILIE_TYPE", idx)
                    {
                        snapshot.family_type = Some(family_type);
                    }
                }
            }
        }

        Ok(snapshot)
    }

    fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<CovariateSnapshot>, IdsError> {
        let family = self
            .get_family_relations(pnr)
            .ok_or_else(|| IdsError::MissingData("Family info not found".to_string()))?;

        let person_covariates = self.get_covariates_at_date(pnr, date)?;

        let father_covariates = family
            .father_id
            .as_ref()
            .and_then(|id| self.get_covariates_at_date(id, date).ok());

        let mother_covariates = family
            .mother_id
            .as_ref()
            .and_then(|id| self.get_covariates_at_date(id, date).ok());

        Ok(Some(CovariateSnapshot::combine(
            person_covariates,
            father_covariates,
            mother_covariates,
        )))
    }
}

impl ArrowAccess for ArrowStore {
    fn find_pnr_index(&self, batch: &RecordBatch, pnr: &str) -> Result<Option<usize>, IdsError> {
        let pnr_array = self.get_string_array(batch, "PNR")?;
        log::debug!(
            "Searching for PNR {} in batch with {} rows",
            pnr,
            batch.num_rows()
        );

        // Sample the first few PNRs in the batch
        for i in 0..std::cmp::min(5, batch.num_rows()) {
            log::debug!("Sample PNR at index {}: {}", i, pnr_array.value(i));
        }

        let found_idx = (0..batch.num_rows()).find(|&i| pnr_array.value(i) == pnr);
        if found_idx.is_some() {
            log::debug!("Found PNR {} at index {}", pnr, found_idx.unwrap());
        }
        Ok(found_idx)
    }

    fn get_value_at_index<T: ArrowPrimitive>(
        &self,
        batch: &RecordBatch,
        column: &str,
        index: usize,
    ) -> Result<Option<T>, IdsError> {
        let col = batch
            .column_by_name(column)
            .ok_or_else(|| IdsError::MissingData(format!("Column {column} not found")))?;

        Ok(T::from_array(col, index))
    }
}

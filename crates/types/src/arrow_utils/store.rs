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
use log::{debug, error, warn};
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
        // debug!("Batch schema: {:?}", schema);

        for &col in expected_columns {
            if !schema.fields().iter().any(|f| f.name() == col) {
                return Err(IdsError::InvalidFormat(format!("Missing column: {col}")));
            }
        }
        Ok(())
    }

    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        let mut valid = true;
        for batch in &batches {
            if let Err(e) = self.validate_batch_schema(batch, &["PNR", "SOCIO13"]) {
                warn!("Invalid AKM batch schema for year {}: {}", year, e);
                valid = false;
            }
        }
        if valid {
            self.akm_data.insert(year, batches);
        } else {
            error!("Skipping invalid AKM data for year {}", year);
        }
    }

    pub fn add_bef_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        let mut valid = true;
        for batch in &batches {
            if let Err(e) =
                self.validate_batch_schema(batch, &["PNR", "ANTPERSF", "KOM", "FAMILIE_TYPE"])
            {
                warn!("Invalid BEF batch schema for period {}: {}", period, e);
                valid = false;
            }
        }
        if valid {
            self.bef_data.insert(period, batches);
        } else {
            error!("Skipping invalid BEF data for period {}", period);
        }
    }

    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) {
        let mut valid = true;
        for batch in &batches {
            if let Err(e) = self.validate_batch_schema(batch, &["PNR", "PERINDKIALT_13"]) {
                warn!("Invalid IND batch schema for year {}: {}", year, e);
                valid = false;
            }
        }
        if valid {
            self.ind_data.insert(year, batches);
        } else {
            error!("Skipping invalid IND data for year {}", year);
        }
    }

    pub fn add_uddf_data(&mut self, period: String, batches: Vec<RecordBatch>) {
        let mut valid = true;
        for batch in &batches {
            if let Err(e) = self.validate_batch_schema(batch, &["PNR", "HFAUDD"]) {
                warn!("Invalid UDDF batch schema for period {}: {}", period, e);
                valid = false;
            }
        }
        if valid {
            self.uddf_data.insert(period, batches);
        } else {
            error!("Skipping invalid UDDF data for period {}", period);
        }
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

        // Log available data
        debug!(
            "Available AKM data years: {:?}",
            self.akm_data.keys().collect::<Vec<_>>()
        );
        debug!(
            "Available BEF periods: {:?}",
            self.bef_data.keys().collect::<Vec<_>>()
        );
        debug!(
            "Available IND data years: {:?}",
            self.ind_data.keys().collect::<Vec<_>>()
        );
        debug!(
            "Available UDDF periods: {:?}",
            self.uddf_data.keys().collect::<Vec<_>>()
        );

        // Check BEF data
        if let Some(batches) = self.bef_data.get(&period) {
            log::debug!("Found BEF data for period {}", period);
            for (i, batch) in batches.iter().enumerate() {
                if let Ok(Some(idx)) = self.find_pnr_index(batch, pnr) {
                    log::debug!("Found PNR in BEF batch {} at index {}", i, idx);

                    // Log the actual values being extracted
                    if let Ok(Some(family_size)) =
                        self.get_value_at_index::<i32>(batch, "ANTPERSF", idx)
                    {
                        log::debug!("Found family size: {}", family_size);
                        snapshot.family_size = Some(family_size);
                    }
                    if let Ok(Some(municipality)) =
                        self.get_value_at_index::<i32>(batch, "KOM", idx)
                    {
                        log::debug!("Found municipality: {}", municipality);
                        snapshot.municipality = Some(municipality);
                    }
                    if let Ok(Some(family_type)) =
                        self.get_value_at_index::<String>(batch, "FAMILIE_TYPE", idx)
                    {
                        log::debug!("Found family type: {}", family_type);
                        snapshot.family_type = Some(family_type);
                    }
                }
            }
        } else {
            log::debug!("No BEF data found for period {}", period);
        }

        // Similarly add debugging for IND data
        if let Some(batches) = self.ind_data.get(&year) {
            log::debug!("Found IND data for year {}", year);
            for batch in batches {
                if let Ok(Some(idx)) = self.find_pnr_index(batch, pnr) {
                    log::debug!("Found PNR in IND data at index {}", idx);
                    if let Ok(Some(income)) =
                        self.get_value_at_index::<f64>(batch, "PERINDKIALT_13", idx)
                    {
                        log::debug!("Found income: {}", income);
                        snapshot.income = Some(Income {
                            amount: income,
                            currency: "DKK".to_string(),
                            type_code: "PERINDKIALT_13".to_string(),
                        });
                    }
                }
            }
        }

        // And for UDDF data
        if let Some(batches) = self.uddf_data.get(&period) {
            log::debug!("Found UDDF data for period {}", period);
            for batch in batches {
                if let Ok(Some(idx)) = self.find_pnr_index(batch, pnr) {
                    log::debug!("Found PNR in UDDF data at index {}", idx);
                    if let Ok(Some(level)) = self.get_value_at_index::<String>(batch, "HFAUDD", idx)
                    {
                        log::debug!("Found education level: {}", level);
                        snapshot.education = Some(Education {
                            level,
                            isced_code: None,
                            years: None,
                        });
                    }
                }
            }
        }

        log::debug!("Final snapshot: {:?}", snapshot);
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
        debug!(
            "Searching for PNR {} in batch with {} rows",
            pnr,
            batch.num_rows()
        );

        // Sample the first few PNRs in the batch
        for i in 0..std::cmp::min(5, batch.num_rows()) {
            debug!("Sample PNR at index {}: {}", i, pnr_array.value(i));
        }

        let found_idx = (0..batch.num_rows()).find(|&i| pnr_array.value(i) == pnr);
        if found_idx.is_some() {
            debug!("Found PNR {} at index {}", pnr, found_idx.unwrap());
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

use crate::error::ArrowError;
use arrow::array::{Array, StringArray};

use arrow::record_batch::RecordBatch;
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

#[derive(Clone)]
pub struct FamilyRelations {
    pub pnr: String,
    pub birth_date: NaiveDate,
    pub father_id: Option<String>,
    pub father_birth_date: Option<NaiveDate>,
    pub mother_id: Option<String>,
    pub mother_birth_date: Option<NaiveDate>,
    pub family_id: Option<String>,
}

impl FamilyRelations {
    pub fn from_record_batch(batch: &RecordBatch, index: usize) -> Result<Self, ArrowError> {
        use arrow::array::{Date32Array, StringArray};

        let pnr = batch
            .column_by_name("PNR")
            .and_then(|arr| arr.as_any().downcast_ref::<StringArray>())
            .ok_or_else(|| ArrowError::Schema("Missing PNR column".to_string()))?
            .value(index)
            .to_string();

        let birth_date = batch
            .column_by_name("FOED_DAG")
            .and_then(|arr| arr.as_any().downcast_ref::<Date32Array>())
            .ok_or_else(|| ArrowError::Schema("Missing FOED_DAG column".to_string()))?
            .value(index);

        let father_id = batch
            .column_by_name("FAR_ID")
            .and_then(|arr| arr.as_any().downcast_ref::<StringArray>())
            .map(|arr| arr.value(index).to_string());

        let mother_id = batch
            .column_by_name("MOR_ID")
            .and_then(|arr| arr.as_any().downcast_ref::<StringArray>())
            .map(|arr| arr.value(index).to_string());

        let family_id = batch
            .column_by_name("FAMILIE_ID")
            .and_then(|arr| arr.as_any().downcast_ref::<StringArray>())
            .map(|arr| arr.value(index).to_string());

        let father_birth_date = batch
            .column_by_name("FAR_FDAG")
            .and_then(|arr| arr.as_any().downcast_ref::<Date32Array>())
            .map(|arr| arr.value(index));

        let mother_birth_date = batch
            .column_by_name("MOR_FDAG")
            .and_then(|arr| arr.as_any().downcast_ref::<Date32Array>())
            .map(|arr| arr.value(index));

        let birth_date = NaiveDate::from_ymd_opt(1970, 1, 1)
            .unwrap()
            .checked_add_signed(chrono::Duration::days(birth_date as i64))
            .ok_or_else(|| ArrowError::Schema("Invalid birth date".to_string()))?;

        let father_birth_date = father_birth_date
            .map(|d| {
                NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .checked_add_signed(chrono::Duration::days(d as i64))
            })
            .flatten();

        let mother_birth_date = mother_birth_date
            .map(|d| {
                NaiveDate::from_ymd_opt(1970, 1, 1)
                    .unwrap()
                    .checked_add_signed(chrono::Duration::days(d as i64))
            })
            .flatten();

        Ok(Self {
            pnr,
            birth_date,
            father_id,
            father_birth_date,
            mother_id,
            mother_birth_date,
            family_id,
        })
    }
}

#[derive(Clone)]
pub struct FamilyStore {
    relations: HashMap<String, FamilyRelations>,
    arrow_store: ArrowStore,
}

impl FamilyStore {
    pub fn new(arrow_store: ArrowStore) -> Self {
        Self {
            relations: HashMap::new(),
            arrow_store,
        }
    }

    pub fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.relations.get(pnr)
    }

    pub fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)> {
        self.relations
            .get(pnr)
            .map(|rel| (rel.father_id.clone(), rel.mother_id.clone()))
    }

    pub fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate> {
        self.relations.get(pnr).map(|rel| rel.birth_date)
    }

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), ArrowError> {
        for batch in batches {
            for row_index in 0..batch.num_rows() {
                let relation = FamilyRelations::from_record_batch(&batch, row_index)?;
                self.relations.insert(relation.pnr.clone(), relation);
            }
        }
        Ok(())
    }

    pub fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<CovariateSnapshot>, ArrowError> {
        // Get family relations
        let family = self
            .relations
            .get(pnr)
            .ok_or_else(|| ArrowError::MissingData("Family relations not found".to_string()))?;

        // Get covariates for the person
        let person_covariates = self.arrow_store.get_covariates_at_date(pnr, date)?;

        // Get parent covariates if available
        let father_covariates = family
            .father_id
            .as_ref()
            .and_then(|id| self.arrow_store.get_covariates_at_date(id, date).ok());

        let mother_covariates = family
            .mother_id
            .as_ref()
            .and_then(|id| self.arrow_store.get_covariates_at_date(id, date).ok());

        Ok(Some(CovariateSnapshot::combine(
            person_covariates,
            father_covariates,
            mother_covariates,
        )))
    }
}

#[derive(Clone)]
pub struct ArrowStore {
    family_store: Option<Box<FamilyStore>>,
    akm_data: HashMap<i32, Vec<RecordBatch>>,
    bef_data: HashMap<String, Vec<RecordBatch>>, // Key: YYYY or YYYYMM
    ind_data: HashMap<i32, Vec<RecordBatch>>,
    uddf_data: HashMap<String, Vec<RecordBatch>>,
}

impl ArrowStore {
    pub fn new() -> Self {
        Self {
            family_store: None,
            akm_data: HashMap::new(),
            bef_data: HashMap::new(),
            ind_data: HashMap::new(),
            uddf_data: HashMap::new(),
        }
    }

    pub fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<CovariateSnapshot>, ArrowError> {
        let family = self
            .get_family_info(pnr)
            .ok_or_else(|| ArrowError::MissingData("Family info not found".to_string()))?;

        let person_covariates = self.get_covariates_at_date(pnr, date)?;

        let father_covariates = family
            .father_id
            .as_ref()
            .and_then(|id| self.get_covariates_at_date(id, date).ok());

        let mother_covariates = family
            .mother_id
            .as_ref()
            .and_then(|id| self.get_covariates_at_date(id, date).ok());

        Ok(CovariateSnapshot::combine_family_covariates(
            Some(person_covariates),
            father_covariates,
            mother_covariates,
        ))
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

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), ArrowError> {
        let mut store = FamilyStore::new(self.clone());
        store.load_family_relations(batches)?;
        self.family_store = Some(Box::new(store));
        Ok(())
    }

    pub fn get_family_info(&self, pnr: &str) -> Option<FamilyRelations> {
        self.family_store
            .as_ref()
            .and_then(|store| store.get_family_relations(pnr).cloned())
    }

    pub fn get_covariates_at_date(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<CovariateSnapshot, ArrowError> {
        let year = date.year();
        let quarter = (date.month() as f32 / 3.0).ceil() as u32;
        let period = format!("{}{:02}", year, quarter * 3);

        let socio = self.get_socio_status(pnr, year)?;
        let income = self.get_income(pnr, year)?;
        let demographics = self.get_demographics(pnr, &period)?;
        let education = self.get_education(pnr, date)?;

        Ok(CovariateSnapshot {
            date,
            income,
            education,
            socioeconomic_status: socio,
            family_size: demographics.as_ref().map(|d| d.family_size),
            municipality: demographics.as_ref().map(|d| d.municipality),
            family_type: demographics.as_ref().map(|d| d.family_type.clone()),
            immigrant_background: None,
            // Initialize parent fields as None - they'll be populated by FamilyStore
            father_income: None,
            father_education: None,
            father_socioeconomic_status: None,
            mother_income: None,
            mother_education: None,
            mother_socioeconomic_status: None,
        })
    }

    fn get_socio_status(&self, pnr: &str, year: i32) -> Result<Option<i32>, ArrowError> {
        if let Some(batches) = self.akm_data.get(&year) {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    if let Some(socio_array) = batch
                        .column_by_name("SOCIO13")
                        .and_then(|arr| arr.as_any().downcast_ref::<arrow::array::Int32Array>())
                    {
                        return Ok(Some(socio_array.value(idx)));
                    }
                }
            }
        }
        Ok(None)
    }

    fn find_pnr_index(&self, batch: &RecordBatch, pnr: &str) -> Result<Option<usize>, ArrowError> {
        if let Some(pnr_array) = batch
            .column_by_name("PNR")
            .and_then(|arr| arr.as_any().downcast_ref::<StringArray>())
        {
            for (i, value) in pnr_array.iter().enumerate() {
                if let Some(value) = value {
                    if value == pnr {
                        return Ok(Some(i));
                    }
                }
            }
        }
        Ok(None)
    }

    fn get_income(&self, pnr: &str, year: i32) -> Result<Option<f64>, ArrowError> {
        if let Some(batches) = self.ind_data.get(&year) {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    if let Some(income_array) = batch
                        .column_by_name("PERINDKIALT_13")
                        .and_then(|arr| arr.as_any().downcast_ref::<arrow::array::Float64Array>())
                    {
                        return Ok(Some(income_array.value(idx)));
                    }
                }
            }
        }
        Ok(None)
    }

    fn get_demographics(
        &self,
        pnr: &str,
        period: &str,
    ) -> Result<Option<Demographics>, ArrowError> {
        if let Some(batches) = self.bef_data.get(period) {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    let family_size = batch
                        .column_by_name("ANTPERSF")
                        .and_then(|arr| arr.as_any().downcast_ref::<arrow::array::Int32Array>())
                        .map(|arr| arr.value(idx))
                        .unwrap_or(0);

                    let municipality = batch
                        .column_by_name("KOM")
                        .and_then(|arr| arr.as_any().downcast_ref::<arrow::array::Int32Array>())
                        .map(|arr| arr.value(idx))
                        .unwrap_or(0);

                    let family_type = batch
                        .column_by_name("FAMILIE_TYPE")
                        .and_then(|arr| arr.as_any().downcast_ref::<StringArray>())
                        .and_then(|arr| Some(arr.value(idx).to_string()))
                        .unwrap_or_else(|| "Unknown".to_string());

                    return Ok(Some(Demographics {
                        family_size,
                        municipality,
                        family_type,
                    }));
                }
            }
        }
        Ok(None)
    }

    fn get_education(&self, pnr: &str, _date: NaiveDate) -> Result<Option<String>, ArrowError> {
        // Find the most recent education record before the given date
        for (_period, batches) in &self.uddf_data {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    if let Some(education) = batch
                        .column_by_name("HFAUDD")
                        .and_then(|arr| arr.as_any().downcast_ref::<StringArray>())
                        .and_then(|arr| Some(arr.value(idx)))
                    {
                        return Ok(Some(education.to_string()));
                    }
                }
            }
        }
        Ok(None)
    }
}

#[derive(Debug, Clone)]
pub struct CovariateSnapshot {
    pub date: NaiveDate,
    // Personal characteristics
    pub income: Option<f64>,
    pub education: Option<String>,
    pub socioeconomic_status: Option<i32>,

    // Family characteristics
    pub family_size: Option<i32>,
    pub municipality: Option<i32>,
    pub family_type: Option<String>,
    pub immigrant_background: Option<String>,

    // Parent characteristics
    pub father_income: Option<f64>,
    pub father_education: Option<String>,
    pub father_socioeconomic_status: Option<i32>,
    pub mother_income: Option<f64>,
    pub mother_education: Option<String>,
    pub mother_socioeconomic_status: Option<i32>,
}

impl CovariateSnapshot {
    pub fn combine(
        person: CovariateSnapshot,
        father: Option<CovariateSnapshot>,
        mother: Option<CovariateSnapshot>,
    ) -> Self {
        Self {
            date: person.date,
            income: person.income,
            education: person.education,
            socioeconomic_status: person.socioeconomic_status,
            family_size: person.family_size,
            municipality: person.municipality,
            family_type: person.family_type,
            immigrant_background: person.immigrant_background,
            father_income: father.as_ref().and_then(|f| f.income),
            father_education: father.as_ref().and_then(|f| f.education.clone()),
            father_socioeconomic_status: father.as_ref().and_then(|f| f.socioeconomic_status),
            mother_income: mother.as_ref().and_then(|f| f.income),
            mother_education: mother.as_ref().and_then(|f| f.education.clone()),
            mother_socioeconomic_status: mother.as_ref().and_then(|f| f.socioeconomic_status),
        }
    }
    pub fn combine_family_covariates(
        person: Option<CovariateSnapshot>,
        father: Option<CovariateSnapshot>,
        mother: Option<CovariateSnapshot>,
    ) -> Option<Self> {
        person.map(|p| CovariateSnapshot {
            date: p.date,
            income: p.income,
            education: p.education,
            socioeconomic_status: p.socioeconomic_status,
            family_size: p.family_size,
            municipality: p.municipality,
            family_type: p.family_type,
            immigrant_background: p.immigrant_background,
            father_income: father.as_ref().and_then(|f| f.income),
            father_education: father.as_ref().and_then(|f| f.education.clone()),
            father_socioeconomic_status: father.as_ref().and_then(|f| f.socioeconomic_status),
            mother_income: mother.as_ref().and_then(|f| f.income),
            mother_education: mother.as_ref().and_then(|f| f.education.clone()),
            mother_socioeconomic_status: mother.as_ref().and_then(|f| f.socioeconomic_status),
        })
    }
}

#[derive(Debug)]
pub struct Demographics {
    pub family_size: i32,
    pub municipality: i32,
    pub family_type: String,
}

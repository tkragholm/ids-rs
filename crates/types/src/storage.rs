use arrow::array::{Array, StringArray};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use dashmap::DashMap;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use crate::{
    arrow_utils::{ArrowAccess, ArrowUtils},
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
    translation::TranslationMaps,
};

/// Core storage trait for all data access operations
pub trait Storage: Send + Sync {
    /// Get a covariate for a person at a specific date
    fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError>;

    /// Get family relations for a person
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;

    /// Load time-varying data
    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError>;

    /// Get as any for downcasting
    fn as_any(&self) -> &dyn std::any::Any;

    /// Get as any mut for downcasting
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;

    /// Get multiple covariates for a person at a specific date
    fn get_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<HashMap<CovariateType, Covariate>, IdsError> {
        let mut result = HashMap::new();
        for cov_type in &[
            CovariateType::Demographics,
            CovariateType::Education,
            CovariateType::Income,
            CovariateType::Occupation,
        ] {
            if let Some(cov) = self.get_covariate(pnr, *cov_type, date)? {
                result.insert(*cov_type, cov);
            }
        }
        Ok(result)
    }

    /// Get family covariates for a person at a specific date
    fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<HashMap<CovariateType, Covariate>>, IdsError> {
        if let Some(relations) = self.get_family_relations(pnr) {
            let mut result = HashMap::new();

            // Try to get parent covariates
            if let Some(father_pnr) = &relations.father_id {
                if let Ok(covs) = self.get_covariates(father_pnr, date) {
                    for (cov_type, cov) in covs {
                        result.insert(cov_type, cov);
                    }
                }
            }

            if let Some(mother_pnr) = &relations.mother_id {
                if let Ok(covs) = self.get_covariates(mother_pnr, date) {
                    for (cov_type, cov) in covs {
                        result.insert(cov_type, cov);
                    }
                }
            }

            if !result.is_empty() {
                return Ok(Some(result));
            }
        }

        Ok(None)
    }
}

/// Cache key for covariate lookups
#[derive(Debug, Hash, Eq, PartialEq)]
pub struct CacheKey {
    pub pnr: String,
    pub covariate_type: CovariateType,
    pub date: NaiveDate,
}

/// Combined store implementation with different backend options and caching
pub struct DataStore {
    backend: Arc<dyn Storage>,
    cache: DashMap<CacheKey, Covariate>,
}

impl DataStore {
    /// Create a new DataStore with an ArrowBackend
    pub fn new_arrow() -> Result<Self, IdsError> {
        Ok(Self {
            backend: Arc::new(ArrowBackend::new()?),
            cache: DashMap::new(),
        })
    }

    /// Create a new DataStore with a TimeVaryingBackend
    pub fn new_time_varying() -> Self {
        Self {
            backend: Arc::new(TimeVaryingBackend::new()),
            cache: DashMap::new(),
        }
    }

    /// Get a covariate from the cache
    fn get_from_cache(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Option<Covariate> {
        let key = CacheKey {
            pnr: pnr.to_string(),
            covariate_type,
            date,
        };
        self.cache.get(&key).map(|v| v.clone())
    }

    /// Store a covariate in the cache
    fn store_in_cache(&self, pnr: &str, covariate: Covariate, date: NaiveDate) {
        let key = CacheKey {
            pnr: pnr.to_string(),
            covariate_type: covariate.get_type(),
            date,
        };
        self.cache.insert(key, covariate);
    }

    /// Access the underlying arrow backend (if available)
    pub fn as_arrow_backend(&self) -> Option<&ArrowBackend> {
        self.backend.as_any().downcast_ref::<ArrowBackend>()
    }

    /// Access the underlying arrow backend mutably (if available)
    pub fn as_arrow_backend_mut(&mut self) -> Option<&mut ArrowBackend> {
        Arc::get_mut(&mut self.backend)?
            .as_any_mut()
            .downcast_mut::<ArrowBackend>()
    }

    /// Load family relations data (only for arrow backend)
    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.load_family_relations(batches)
        } else {
            Err(IdsError::invalid_operation(
                "Cannot load family relations into this backend type",
            ))
        }
    }

    /// Add AKM (labor market) data
    pub fn add_akm_data(&mut self, year: i32, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.add_akm_data(year, batches);
            Ok(())
        } else {
            Err(IdsError::invalid_operation(
                "Cannot add AKM data to this backend type",
            ))
        }
    }

    /// Add BEF (population) data
    pub fn add_bef_data(
        &mut self,
        period: String,
        batches: Vec<RecordBatch>,
    ) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.add_bef_data(period, batches);
            Ok(())
        } else {
            Err(IdsError::invalid_operation(
                "Cannot add BEF data to this backend type",
            ))
        }
    }

    /// Add IND (income) data
    pub fn add_ind_data(&mut self, year: i32, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.add_ind_data(year, batches);
            Ok(())
        } else {
            Err(IdsError::invalid_operation(
                "Cannot add IND data to this backend type",
            ))
        }
    }

    /// Add UDDF (education) data
    pub fn add_uddf_data(
        &mut self,
        period: String,
        batches: Vec<RecordBatch>,
    ) -> Result<(), IdsError> {
        if let Some(backend) = self.as_arrow_backend_mut() {
            backend.add_uddf_data(period, batches);
            Ok(())
        } else {
            Err(IdsError::invalid_operation(
                "Cannot add UDDF data to this backend type",
            ))
        }
    }

    /// Save current covariates to CSV (only for time-varying backend)
    pub fn save_to_csv(&self, path: &Path) -> Result<(), IdsError> {
        if let Some(backend) = self.backend.as_any().downcast_ref::<TimeVaryingBackend>() {
            backend.save_to_csv(path)
        } else {
            Err(IdsError::invalid_operation(
                "Cannot save this backend type to CSV",
            ))
        }
    }
}

impl Storage for DataStore {
    fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        // Try cache first
        if let Some(cached) = self.get_from_cache(pnr, covariate_type, date) {
            return Ok(Some(cached));
        }

        // If not in cache, get from backend
        let result = self.backend.get_covariate(pnr, covariate_type, date)?;

        // Store in cache if found
        if let Some(ref covariate) = result {
            self.store_in_cache(pnr, covariate.clone(), date);
        }

        Ok(result)
    }

    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.backend.get_family_relations(pnr)
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        if let Some(backend) = Arc::get_mut(&mut self.backend) {
            backend.load_data(data)
        } else {
            Err(IdsError::invalid_operation(
                "Cannot load data into a shared backend",
            ))
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Backend trait marker for storage implementations
pub trait Backend: Storage {
    // All methods are already in Storage trait
}

/// Arrow-based storage backend
#[derive(Debug, Clone)]
pub struct ArrowBackend {
    family_data: HashMap<String, FamilyRelations>,
    akm_data: HashMap<i32, Vec<RecordBatch>>,
    bef_data: HashMap<String, Vec<RecordBatch>>,
    ind_data: HashMap<i32, Vec<RecordBatch>>,
    uddf_data: HashMap<String, Vec<RecordBatch>>,
    translations: TranslationMaps,
}

impl ArrowBackend {
    pub fn new() -> Result<Self, IdsError> {
        let translations =
            TranslationMaps::new().map_err(|e| IdsError::invalid_format(format!("{}", e)))?;

        Ok(Self {
            family_data: HashMap::new(),
            akm_data: HashMap::new(),
            bef_data: HashMap::new(),
            ind_data: HashMap::new(),
            uddf_data: HashMap::new(),
            translations,
        })
    }

    pub fn add_akm_data(&mut self, year: i32, mut batches: Vec<RecordBatch>) {
        // Validate batches first
        for batch in &batches {
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid AKM batch for year {}: {}", year, e);
            }
        }

        // Optimize batch memory layout
        for batch in &mut batches {
            ArrowUtils::align_batch_buffers(batch);
        }

        self.akm_data.insert(year, batches);
    }

    pub fn add_bef_data(&mut self, period: String, mut batches: Vec<RecordBatch>) {
        // Validate batches first
        for batch in &batches {
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid BEF batch for period {}: {}", period, e);
            }
        }

        // Optimize batch memory layout
        for batch in &mut batches {
            ArrowUtils::align_batch_buffers(batch);
        }

        self.bef_data.insert(period, batches);
    }

    pub fn add_ind_data(&mut self, year: i32, mut batches: Vec<RecordBatch>) {
        // Validate batches first
        for batch in &batches {
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid IND batch for year {}: {}", year, e);
            }
        }

        // Optimize batch memory layout
        for batch in &mut batches {
            ArrowUtils::align_batch_buffers(batch);
        }

        self.ind_data.insert(year, batches);
    }

    pub fn add_uddf_data(&mut self, period: String, mut batches: Vec<RecordBatch>) {
        // Validate batches first
        for batch in &batches {
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid UDDF batch for period {}: {}", period, e);
            }
        }

        // Optimize batch memory layout
        for batch in &mut batches {
            ArrowUtils::align_batch_buffers(batch);
        }

        self.uddf_data.insert(period, batches);
    }

    fn get_education(&self, pnr: &str, date: NaiveDate) -> Result<Option<Covariate>, IdsError> {
        // Find the closest UDDF data period before the given date
        let period = self.find_closest_period(date, &self.uddf_data)?;

        if let Some(batches) = period.and_then(|p| self.uddf_data.get(p)) {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    // Use optimized array data access
                    let hfaudd_array = self.get_string_array(batch, "HFAUDD")?;
                    //let array_data = hfaudd_array.data();

                    if !hfaudd_array.is_null(idx) {
                        let level = hfaudd_array.value(idx).to_string();
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
                    // Get value directly using optimized method
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
                    // Use direct access for better performance - get all values at once
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

    /// Optimize batch operations by slicing when needed
    pub fn optimize_batch(&mut self, batch: &mut RecordBatch) -> Result<(), IdsError> {
        // Align buffers for better memory performance
        ArrowUtils::align_batch_buffers(batch);
        Ok(())
    }

    /// Slice a batch for zero-copy operations
    pub fn slice_batch(
        &self,
        batch: &RecordBatch,
        offset: usize,
        length: usize,
    ) -> Result<RecordBatch, IdsError> {
        let mut columns = Vec::with_capacity(batch.num_columns());

        for i in 0..batch.num_columns() {
            columns.push(ArrowUtils::slice_array(
                batch.column(i).as_ref(),
                offset,
                length,
            ));
        }

        RecordBatch::try_new(batch.schema().clone(), columns).map_err(|e| {
            IdsError::invalid_operation(format!("Failed to create sliced batch: {}", e))
        })
    }

    /// Create an optimized string array
    pub fn create_optimized_string_array(
        &self,
        strings: &[String],
    ) -> Result<StringArray, IdsError> {
        ArrowUtils::create_optimized_string_array(strings, strings.len())
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
        mut family_batches: Vec<RecordBatch>,
    ) -> Result<(), IdsError> {
        use crate::family::FamilyStore;

        // Optimize batches before loading
        for batch in &mut family_batches {
            // Validate batch
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid family relations batch: {}", e);
            }

            // Optimize memory layout
            ArrowUtils::align_batch_buffers(batch);
        }

        let mut family_store = FamilyStore::new();
        family_store.load_family_relations(family_batches)?;
        self.family_data = family_store.get_relations();
        Ok(())
    }
}

impl Storage for ArrowBackend {
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
        Err(IdsError::invalid_operation(
            "Cannot load time-varying data into Arrow store",
        ))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Backend for ArrowBackend {}

/// Time-varying storage backend
#[derive(Debug)]
pub struct TimeVaryingBackend {
    data: DashMap<String, Vec<TimeVaryingValue<Covariate>>>,
    family_data: HashMap<String, FamilyRelations>,
}

impl Default for TimeVaryingBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl TimeVaryingBackend {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
            family_data: HashMap::new(),
        }
    }

    fn get_latest_value(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Option<Covariate> {
        self.data.get(pnr).and_then(|values| {
            values
                .iter()
                .filter(|v| v.date <= date && v.value.get_type() == covariate_type)
                .max_by_key(|v| v.date)
                .map(|v| v.value.clone())
        })
    }

    /// Save to CSV
    pub fn save_to_csv(&self, path: &Path) -> Result<(), IdsError> {
        let mut writer = csv::Writer::from_path(path).map_err(IdsError::Csv)?;

        writer
            .write_record(["PNR", "Date", "Covariate Type", "Value"])
            .map_err(IdsError::Csv)?;

        for entry in self.data.iter() {
            for value in entry.value() {
                writer
                    .write_record([
                        &value.pnr,
                        &value.date.to_string(),
                        &format!("{:?}", value.value.get_type()),
                        &format!("{:?}", value.value),
                    ])
                    .map_err(IdsError::Csv)?;
            }
        }

        writer.flush().map_err(IdsError::Io)?;
        Ok(())
    }
}

impl Storage for TimeVaryingBackend {
    fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError> {
        Ok(self.get_latest_value(pnr, covariate_type, date))
    }

    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.family_data.get(pnr)
    }

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError> {
        for value in data {
            self.data.entry(value.pnr.clone()).or_default().push(value);
        }
        Ok(())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

impl Backend for TimeVaryingBackend {}

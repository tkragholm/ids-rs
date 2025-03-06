use arrow::array::{Array, StringArray};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use hashbrown::HashMap;
use log;

// Note: For initial phase, we'll keep original imports, to be updated in next phase
use crate::{
    arrow::access::ArrowAccess,
    arrow::utils::ArrowUtils,
    error::IdsError,
    family::{FamilyRelations, FamilyStore},
    models::{Covariate, CovariateType, TimeVaryingValue},
    traits::Store,
    translation::TranslationMaps,
};

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
            TranslationMaps::new().map_err(|e| IdsError::invalid_format(format!("{e}")))?;

        Ok(Self {
            family_data: HashMap::new(),
            akm_data: HashMap::new(),
            bef_data: HashMap::new(),
            ind_data: HashMap::new(),
            uddf_data: HashMap::new(),
            translations,
        })
    }

    /// Create a new empty ArrowBackend, used for diagnostic mode when data loading fails
    /// 
    /// # Panics
    /// 
    /// This function will panic if it fails to create valid dates for the synthetic data.
    /// Since this is only used for diagnostic purposes and uses carefully constructed date values,
    /// the panics would indicate a serious programming error rather than a runtime condition.
    #[must_use]
    pub fn new_empty() -> Self {
        // Create a minimal store for diagnostic operations with some synthetic data for debugging
        let mut family_data = HashMap::new();
        let ind_data = HashMap::new();
        let bef_data = HashMap::new();

        // Add synthetic relationships and data for debugging in diagnostic mode
        for i in 0..100 {
            // Add some synthetic family data for diagnostic purposes
            let case_id = format!("C{i:06}");
            let control_id = format!("K{i:06}");

            // Calculate valid date components with safe ranges
            let year = 1990 + (i % 30);
            let month = 1 + (i % 12) as u32;
            let day = 1 + (i % 28) as u32; // Always ≤ 28 to avoid invalid dates
            
            let father_year = 1950 + (i % 30);
            let mother_year = 1955 + (i % 30);

            // Get a birth date based on the index - these are constructed to always be valid
            // We explicitly document the panics here since this is diagnostic code only
            let birth_date = chrono::NaiveDate::from_ymd_opt(year, month, day)
                .expect("Invalid synthetic birth date constructed in diagnostic mode");
                
            let father_birth_date = chrono::NaiveDate::from_ymd_opt(father_year, month, day)
                .expect("Invalid synthetic father birth date constructed in diagnostic mode");
                
            let mother_birth_date = chrono::NaiveDate::from_ymd_opt(mother_year, month, day)
                .expect("Invalid synthetic mother birth date constructed in diagnostic mode");

            // Add family relations for cases and controls
            family_data.insert(
                case_id.clone(),
                FamilyRelations {
                    pnr: case_id.clone(),
                    birth_date,
                    father_id: Some(format!("F{i:06}")),
                    father_birth_date: Some(father_birth_date),
                    mother_id: Some(format!("M{i:06}")),
                    mother_birth_date: Some(mother_birth_date),
                    family_id: Some(format!("FAM{i:06}")),
                },
            );

            family_data.insert(
                control_id.clone(),
                FamilyRelations {
                    pnr: control_id.clone(),
                    birth_date,
                    father_id: Some(format!("F{:06}", i + 1000)),
                    father_birth_date: Some(father_birth_date),
                    mother_id: Some(format!("M{:06}", i + 1000)),
                    mother_birth_date: Some(mother_birth_date),
                    family_id: Some(format!("FAM{:06}", i + 1000)),
                },
            );
        }

        Self {
            family_data,
            akm_data: HashMap::new(),
            bef_data,
            ind_data,
            uddf_data: HashMap::new(),
            translations: TranslationMaps::new_empty(),
        }
    }

    pub fn add_akm_data(&mut self, year: i32, mut batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        // Validate batches first
        for batch in &batches {
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid AKM batch for year {}: {}", year, e);
            }
        }

        // Optimize batch memory layout
        for batch in &mut batches {
            let _ = ArrowUtils::align_batch_buffers(batch);
        }

        self.akm_data.insert(year, batches);
        Ok(())
    }

    pub fn add_bef_data(&mut self, period: String, mut batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        // Validate batches first
        for batch in &batches {
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid BEF batch for period {}: {}", period, e);
            }
        }

        // Optimize batch memory layout
        for batch in &mut batches {
            let _ = ArrowUtils::align_batch_buffers(batch);
        }

        self.bef_data.insert(period, batches);
        Ok(())
    }

    pub fn add_ind_data(&mut self, year: i32, mut batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        // Validate batches first
        for batch in &batches {
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid IND batch for year {}: {}", year, e);
            }
        }

        // Optimize batch memory layout
        for batch in &mut batches {
            let _ = ArrowUtils::align_batch_buffers(batch);
        }

        self.ind_data.insert(year, batches);
        Ok(())
    }

    pub fn add_uddf_data(&mut self, period: String, mut batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        // Validate batches first
        for batch in &batches {
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid UDDF batch for period {}: {}", period, e);
            }
        }

        // Optimize batch memory layout
        for batch in &mut batches {
            let _ = ArrowUtils::align_batch_buffers(batch);
        }

        self.uddf_data.insert(period, batches);
        Ok(())
    }
    
    /// Add family data to the backend
    pub fn add_family_data(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        // Load family relations using existing implementation
        self.load_family_relations(batches)
    }

    fn get_education(&self, pnr: &str, date: NaiveDate) -> Result<Option<Covariate>, IdsError> {
        // Find the closest UDDF data period before the given date
        let period = self.find_closest_period(date, &self.uddf_data)?;

        if let Some(batches) = period.and_then(|p| self.uddf_data.get(p)) {
            for batch in batches {
                if let Some(idx) = self.find_pnr_index(batch, pnr)? {
                    // Use optimized array data access
                    let hfaudd_array = self.get_string_array(batch, "HFAUDD")?;

                    if !hfaudd_array.is_null(idx) {
                        let level = hfaudd_array.value(idx).to_string();
                        return Ok(Some(Covariate::education(level).build()));
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
                            "DKK",
                            "PERINDKIALT_13",
                        ).build()));
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
                    // Fix: Get STATSB as a string value, not an integer
                    let statsb: Option<String> = self.get_value(batch, "STATSB", idx)?;

                    if let (Some(family_size), Some(municipality), Some(family_type)) =
                        (family_size, municipality, family_type)
                    {
                        let mut builder = Covariate::demographics(
                            family_size,
                            municipality,
                            family_type.to_string(),
                        );

                        // Add citizenship if available
                        if let Some(statsb) = statsb {
                            builder = builder.with_citizenship(statsb.clone());
                            
                            // Add translated value to metadata
                            if let Some(translated) = self
                                .translations
                                .translate(crate::translation::TranslationType::Statsb, &statsb)
                            {
                                builder = builder.with_metadata(
                                    "statsb_translated",
                                    translated,
                                );
                            }
                        }

                        return Ok(Some(builder.build()));
                    }
                }
            }
        }
        Ok(None)
    }

    /// Optimize batch operations by slicing when needed
    pub fn optimize_batch(&mut self, batch: &mut RecordBatch) -> Result<(), IdsError> {
        // Align buffers for better memory performance
        let _ = ArrowUtils::align_batch_buffers(batch);
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

        RecordBatch::try_new(batch.schema(), columns).map_err(|e| {
            IdsError::invalid_operation(format!("Failed to create sliced batch: {e}"))
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
                // Safely handle potential parsing errors with defaults
                if p.len() < 4 {
                    return false; // Period string too short for YYYY format
                }
                
                let year: i32 = match p[0..4].parse() {
                    Ok(y) => y,
                    Err(_) => return false, // Skip invalid year format
                };
                
                let month: u32 = if p.len() > 5 {
                    p[4..6].parse().unwrap_or(12) // Default to December for invalid month
                } else {
                    12 // Default to December when no month specified
                };
                
                // Only include periods that can be converted to valid dates and are before or equal to target date
                NaiveDate::from_ymd_opt(year, month, 1)
                    .is_some_and(|period_date| period_date <= date)
            })
            .max_by_key(|p| p.len()))
    }

    pub fn load_family_relations(
        &mut self,
        mut family_batches: Vec<RecordBatch>,
    ) -> Result<(), IdsError> {
        // Optimize batches before loading
        for batch in &mut family_batches {
            // Validate batch
            if let Err(e) = self.validate_batch(batch) {
                log::warn!("Invalid family relations batch: {}", e);
            }

            // Optimize memory layout
            let _ = ArrowUtils::align_batch_buffers(batch);
        }

        let mut family_store = FamilyStore::new();
        family_store.load_family_relations(family_batches)?;
        self.family_data = family_store.get_relations().clone();
        Ok(())
    }
}

impl Store for ArrowBackend {
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
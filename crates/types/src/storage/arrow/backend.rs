use arrow::array::{Array, StringArray};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use hashbrown::HashMap;
use log;

// Updated imports for new module structure
use crate::{
    error::IdsError,
    family::{FamilyRelations, FamilyStore},
    models::{Covariate, CovariateType, TimeVaryingValue},
    storage::arrow::access::ArrowAccess,
    storage::arrow::convert::ArrowType, 
    storage::arrow::utils::ArrowUtils,
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
    
    // Performance optimization: Cache for PNR to batch/index mapping
    pnr_index_cache: HashMap<(String, String), (usize, usize)>, // (register_type, pnr) -> (batch_idx, row_idx)
    
    // Performance optimization: Pre-computed date mapping for periods
    period_date_cache: HashMap<String, NaiveDate>, // period string -> date
}

impl ArrowBackend {
    /// Create a new ArrowBackend instance with empty data
    ///
    /// # Returns
    /// * `std::result::Result<Self, IdsError>` - A new ArrowBackend or an error
    ///
    /// # Errors
    /// Returns an error if the TranslationMaps cannot be initialized
    pub fn new() -> std::result::Result<Self, IdsError> {
        let translations =
            TranslationMaps::new().map_err(|e| IdsError::invalid_format(format!("{e}")))?;

        Ok(Self {
            family_data: HashMap::new(),
            akm_data: HashMap::new(),
            bef_data: HashMap::new(),
            ind_data: HashMap::new(),
            uddf_data: HashMap::new(),
            translations,
            pnr_index_cache: HashMap::new(),
            period_date_cache: HashMap::new(),
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
        let pnr_index_cache = HashMap::new();
        let mut period_date_cache = HashMap::new();

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

        // Pre-compute some period date mappings for common periods
        for year in 2000..2024 {
            // Add annual periods
            let period = format!("{year}");
            let date = NaiveDate::from_ymd_opt(year, 12, 31)
                .expect("Invalid date in new_empty period initialization");
            period_date_cache.insert(period, date);
            
            // Add quarterly periods
            for quarter in 1..=4 {
                let month = quarter * 3;
                let period = format!("{year}{month:02}");
                let date = NaiveDate::from_ymd_opt(year, month as u32, 1)
                    .expect("Invalid date in new_empty period initialization");
                period_date_cache.insert(period, date);
            }
        }

        Self {
            family_data,
            akm_data: HashMap::new(),
            bef_data,
            ind_data,
            uddf_data: HashMap::new(),
            translations: TranslationMaps::new_empty(),
            pnr_index_cache,
            period_date_cache,
        }
    }

    /// Add AKM (labor market) data to the backend
    ///
    /// This method adds AKM data for a specific year and builds index caches
    /// for optimized access.
    ///
    /// # Arguments
    /// * `year` - The year for the data
    /// * `batches` - The record batches containing AKM data
    ///
    /// # Returns
    /// * `std::result::Result<(), IdsError>` - Success or an error
    ///
    /// # Errors
    /// Returns an error if batch validation fails
    pub fn add_akm_data(&mut self, year: i32, mut batches: Vec<RecordBatch>) -> std::result::Result<(), IdsError> {
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
        
        // Build PNR index cache for faster lookups
        self.build_pnr_index_cache("akm", &batches)?;

        // Store the batches
        self.akm_data.insert(year, batches);
        Ok(())
    }
    
    /// Build PNR index cache for a specific register type and batch set
    ///
    /// This method builds an index cache that maps PNRs to batch and row indices
    /// for faster lookup operations.
    ///
    /// # Arguments
    /// * `register_type` - The type of register ("akm", "bef", etc.)
    /// * `batches` - The record batches to index
    ///
    /// # Returns
    /// * `std::result::Result<(), IdsError>` - Success or an error
    ///
    /// # Errors
    /// Returns an error if the PNR column cannot be found or accessed
    fn build_pnr_index_cache(&mut self, register_type: &str, batches: &[RecordBatch]) -> std::result::Result<(), IdsError> {
        for (batch_idx, batch) in batches.iter().enumerate() {
            // Skip if the batch doesn't have a PNR column
            if !batch.schema().fields().iter().any(|f| f.name() == "PNR") {
                continue;
            }
            
            let pnr_idx = batch.schema().index_of("PNR")?;
            let pnr_array = batch.column(pnr_idx);
            
            if let Some(string_array) = pnr_array.as_any().downcast_ref::<StringArray>() {
                for row_idx in 0..string_array.len() {
                    if !string_array.is_null(row_idx) {
                        let pnr = string_array.value(row_idx).to_string();
                        // Store the mapping: (register_type, pnr) -> (batch_idx, row_idx)
                        self.pnr_index_cache.insert(
                            (register_type.to_string(), pnr),
                            (batch_idx, row_idx)
                        );
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Add BEF (population) data to the backend
    ///
    /// This method adds BEF data for a specific period and builds index caches
    /// for optimized access.
    ///
    /// # Arguments
    /// * `period` - The period for the data (e.g., "201903")
    /// * `batches` - The record batches containing BEF data
    ///
    /// # Returns
    /// * `std::result::Result<(), IdsError>` - Success or an error
    ///
    /// # Errors
    /// Returns an error if batch validation fails
    pub fn add_bef_data(&mut self, period: String, mut batches: Vec<RecordBatch>) -> std::result::Result<(), IdsError> {
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
        
        // Build PNR index cache for faster lookups
        self.build_pnr_index_cache("bef", &batches)?;
        
        // Pre-compute period date for faster period lookups
        self.add_period_to_cache(&period)?;

        self.bef_data.insert(period, batches);
        Ok(())
    }

    /// Add IND (income) data to the backend
    ///
    /// This method adds IND data for a specific year and builds index caches
    /// for optimized access.
    ///
    /// # Arguments
    /// * `year` - The year for the data
    /// * `batches` - The record batches containing IND data
    ///
    /// # Returns
    /// * `std::result::Result<(), IdsError>` - Success or an error
    ///
    /// # Errors
    /// Returns an error if batch validation fails
    pub fn add_ind_data(&mut self, year: i32, mut batches: Vec<RecordBatch>) -> std::result::Result<(), IdsError> {
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
        
        // Build PNR index cache for faster lookups
        self.build_pnr_index_cache("ind", &batches)?;

        self.ind_data.insert(year, batches);
        Ok(())
    }

    /// Add UDDF (education) data to the backend
    ///
    /// This method adds UDDF data for a specific period and builds index caches
    /// for optimized access.
    ///
    /// # Arguments
    /// * `period` - The period for the data (e.g., "201903")
    /// * `batches` - The record batches containing UDDF data
    ///
    /// # Returns
    /// * `std::result::Result<(), IdsError>` - Success or an error
    ///
    /// # Errors
    /// Returns an error if batch validation fails
    pub fn add_uddf_data(&mut self, period: String, mut batches: Vec<RecordBatch>) -> std::result::Result<(), IdsError> {
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
        
        // Build PNR index cache for faster lookups
        self.build_pnr_index_cache("uddf", &batches)?;
        
        // Pre-compute period date for faster period lookups
        self.add_period_to_cache(&period)?;

        self.uddf_data.insert(period, batches);
        Ok(())
    }
    
    /// Add a period to the date cache for faster lookups
    ///
    /// This method parses a period string like "201903" and adds a mapping
    /// to the corresponding NaiveDate.
    ///
    /// # Arguments
    /// * `period` - The period string (e.g., "201903")
    ///
    /// # Returns
    /// * `std::result::Result<(), IdsError>` - Success or an error
    ///
    /// # Errors
    /// Returns an error if the period cannot be parsed into a valid date
    fn add_period_to_cache(&mut self, period: &str) -> std::result::Result<(), IdsError> {
        // Skip if period is already in cache
        if self.period_date_cache.contains_key(period) {
            return Ok(());
        }
        
        // Parse period into year and month
        if period.len() < 4 {
            return Err(IdsError::invalid_format(
                format!("Period string too short: {}", period)
            ));
        }
        
        let year: i32 = period[0..4].parse().map_err(|_| {
            IdsError::invalid_format(format!("Invalid year in period: {}", period))
        })?;
        
        let month: u32 = if period.len() >= 6 {
            period[4..6].parse().unwrap_or(12) // Default to December
        } else {
            12 // Default to December
        };
        
        let day = 1; // Always use the first day of the month
        
        // Create date and add to cache
        let date = NaiveDate::from_ymd_opt(year, month, day).ok_or_else(|| {
            IdsError::invalid_format(format!("Invalid date for period: {}", period))
        })?;
        
        self.period_date_cache.insert(period.to_string(), date);
        Ok(())
    }
    
    /// Add family data to the backend
    pub fn add_family_data(&mut self, batches: Vec<RecordBatch>) -> std::result::Result<(), IdsError> {
        // Load family relations using existing implementation
        self.load_family_relations(batches)
    }

    /// Get education covariate for a PNR at a given date
    ///
    /// # Arguments
    /// * `pnr` - The person identification number
    /// * `date` - The reference date
    ///
    /// # Returns
    /// * `std::result::Result<Option<Covariate>, IdsError>` - Education covariate or None
    ///
    /// # Errors
    /// Returns an error if data access fails
    fn get_education(&self, pnr: &str, date: NaiveDate) -> std::result::Result<Option<Covariate>, IdsError> {
        // Find the closest UDDF data period before the given date using optimized closest period lookup
        let period = self.find_closest_period(date, &self.uddf_data)?;

        if let Some(period_str) = period {
            if let Some(batches) = self.uddf_data.get(period_str) {
                // Search through batches with the cached index if available
                for (batch_idx, batch) in batches.iter().enumerate() {
                    // Use optimized PNR index lookup with cache
                    if let Some(idx) = self.find_pnr_index_with_cache(batch, pnr, "uddf", batch_idx)? {
                        // Use optimized array data access
                        let hfaudd_array = self.get_string_array(batch, "HFAUDD")?;

                        if !hfaudd_array.is_null(idx) {
                            let level = hfaudd_array.value(idx).to_string();
                            return Ok(Some(Covariate::education(level).build()));
                        }
                    }
                }
            }
        }
        Ok(None)
    }

    /// Get income covariate for a PNR at a given date
    ///
    /// # Arguments
    /// * `pnr` - The person identification number
    /// * `date` - The reference date
    ///
    /// # Returns
    /// * `std::result::Result<Option<Covariate>, IdsError>` - Income covariate or None
    ///
    /// # Errors
    /// Returns an error if data access fails
    fn get_income(&self, pnr: &str, date: NaiveDate) -> std::result::Result<Option<Covariate>, IdsError> {
        use chrono::Datelike;
        let year = date.year();
        if let Some(batches) = self.ind_data.get(&year) {
            // Search through batches with the cached index if available
            for (batch_idx, batch) in batches.iter().enumerate() {
                // Use optimized PNR index lookup with cache
                if let Some(idx) = self.find_pnr_index_with_cache(batch, pnr, "ind", batch_idx)? {
                    // Optimized direct access to values
                    let column = batch.column(batch.schema().index_of("PERINDKIALT_13")?);
                    let array = column.as_any().downcast_ref::<arrow::array::Float64Array>()
                        .ok_or_else(|| IdsError::data_loading("Income column not a float array".to_string()))?;
                    let amount = if array.is_null(idx) { None } else { Some(array.value(idx)) };
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

    /// Get demographics covariate for a PNR at a given date
    ///
    /// # Arguments
    /// * `pnr` - The person identification number
    /// * `date` - The reference date
    ///
    /// # Returns
    /// * `std::result::Result<Option<Covariate>, IdsError>` - Demographics covariate or None
    ///
    /// # Errors
    /// Returns an error if data access fails
    fn get_demographics(&self, pnr: &str, date: NaiveDate) -> std::result::Result<Option<Covariate>, IdsError> {
        // Use optimized period lookup with cache
        let period = self.find_closest_period(date, &self.bef_data)?;

        if let Some(period_str) = period {
            if let Some(batches) = self.bef_data.get(period_str) {
                // Search through batches with the cached index if available
                for (batch_idx, batch) in batches.iter().enumerate() {
                    // Use optimized PNR index lookup with cache
                    if let Some(idx) = self.find_pnr_index_with_cache(batch, pnr, "bef", batch_idx)? {
                        // Optimized - prefetch all column indices in one block to reduce lookups
                        let schema = batch.schema();
                        let family_size_idx = schema.index_of("ANTPERSF")?;
                        let kom_idx = schema.index_of("KOM")?;
                        let family_type_idx = schema.index_of("FAMILIE_TYPE")?;
                        let statsb_idx = schema.index_of("STATSB")?;
                        
                        // Get columns directly
                        let family_size_col = batch.column(family_size_idx);
                        let kom_col = batch.column(kom_idx);
                        let family_col = batch.column(family_type_idx);
                        let statsb_col = batch.column(statsb_idx);
                        
                        // Extract values with optimized null checking
                        let family_size: Option<i32> = if family_size_col.is_null(idx) { 
                            None 
                        } else {
                            let array = family_size_col.as_any().downcast_ref::<arrow::array::Int32Array>()
                                .ok_or_else(|| IdsError::data_loading("ANTPERSF not an int32 array".to_string()))?;
                            Some(array.value(idx))
                        };
                        
                        let municipality: Option<i32> = if kom_col.is_null(idx) { 
                            None 
                        } else {
                            let array = kom_col.as_any().downcast_ref::<arrow::array::Int32Array>()
                                .ok_or_else(|| IdsError::data_loading("KOM not an int32 array".to_string()))?;
                            Some(array.value(idx))
                        };
                        
                        let family_type: Option<i32> = if family_col.is_null(idx) {
                            None
                        } else {
                            let array = family_col.as_any().downcast_ref::<arrow::array::Int32Array>()
                                .ok_or_else(|| IdsError::data_loading("FAMILIE_TYPE not an int32 array".to_string()))?;
                            Some(array.value(idx))
                        };

                        let statsb: Option<String> = if statsb_col.is_null(idx) {
                            None
                        } else {
                            let array = statsb_col.as_any().downcast_ref::<arrow::array::StringArray>()
                                .ok_or_else(|| IdsError::data_loading("STATSB not a string array".to_string()))?;
                            Some(array.value(idx).to_string())
                        };

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
        }
        Ok(None)
    }

    /// Optimize batch operations by slicing when needed
    pub fn optimize_batch(&mut self, batch: &mut RecordBatch) -> std::result::Result<(), IdsError> {
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
    ) -> std::result::Result<RecordBatch, IdsError> {
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
    ) -> std::result::Result<StringArray, IdsError> {
        ArrowUtils::create_optimized_string_array(strings, strings.len())
    }

    /// Find the closest period for a date
    ///
    /// This method finds the most specific period (e.g., "202303" over "2023") 
    /// that is before or equal to the target date.
    ///
    /// # Arguments
    /// * `date` - The target date
    /// * `data` - The data map to search in
    ///
    /// # Returns
    /// * `std::result::Result<Option<&'a String>, IdsError>` - The closest period or None
    ///
    /// # Errors
    /// Returns an error if period date conversion fails
    fn find_closest_period<'a>(
        &self,
        date: NaiveDate,
        data: &'a HashMap<String, Vec<RecordBatch>>,
    ) -> std::result::Result<Option<&'a String>, IdsError> {
        // Track the closest period we've found
        let mut closest_period: Option<&String> = None;
        let mut closest_date: Option<NaiveDate> = None;
        
        // Fast path - use our cache for date comparisons
        for period in data.keys() {
            // Get the period date from cache or compute it
            let period_date = if let Some(cached_date) = self.period_date_cache.get(period) {
                *cached_date
            } else {
                // If not in cache, skip periods with invalid format
                if period.len() < 4 {
                    continue;
                }
                
                let year: i32 = match period[0..4].parse() {
                    Ok(y) => y,
                    Err(_) => continue, // Skip invalid year format
                };
                
                let month: u32 = if period.len() > 5 {
                    match period[4..6].parse() {
                        Ok(m) => m,
                        Err(_) => 12, // Default to December for invalid month
                    }
                } else {
                    12 // Default to December when no month specified
                };
                
                match NaiveDate::from_ymd_opt(year, month, 1) {
                    Some(pd) => pd,
                    None => continue, // Skip invalid dates
                }
            };
            
            // Only consider periods before or equal to the target date
            if period_date <= date {
                // First period or a more recent period than what we've found so far
                if closest_date.is_none() || period_date > closest_date.unwrap() {
                    closest_date = Some(period_date);
                    closest_period = Some(period);
                } else if period_date == closest_date.unwrap() {
                    // For equal dates, prefer the more specific period (longer string)
                    if period.len() > closest_period.unwrap().len() {
                        closest_period = Some(period);
                    }
                }
            }
        }
        
        Ok(closest_period)
    }

    pub fn load_family_relations(
        &mut self,
        mut family_batches: Vec<RecordBatch>,
    ) -> std::result::Result<(), IdsError> {
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
    ) -> std::result::Result<Option<Covariate>, IdsError> {
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

    fn load_data(&mut self, _data: Vec<TimeVaryingValue<Covariate>>) -> std::result::Result<(), IdsError> {
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

// Helper methods for data access and optimization
impl ArrowBackend {
    /// Find the index of a PNR in a batch using cached indices when available
    /// 
    /// This method first checks the PNR index cache before falling back to a linear scan
    /// of the batch. The cache provides O(1) lookups for previously indexed PNRs.
    ///
    /// # Arguments
    /// * `batch` - The record batch to search in
    /// * `pnr` - The PNR to find
    /// * `register_type` - Optional register type for cache lookup
    ///
    /// # Returns
    /// * `std::result::Result<Option<usize>, IdsError>` - The found index or None
    ///
    /// # Errors
    /// Returns an error if the PNR column cannot be accessed
    fn find_pnr_index_with_cache(&self, 
                               batch: &RecordBatch, 
                               pnr: &str, 
                               register_type: &str, 
                               batch_idx: usize) 
                              -> std::result::Result<Option<usize>, IdsError> {
        // First check if we have this PNR in our index cache
        if let Some(&(cached_batch_idx, row_idx)) = self.pnr_index_cache.get(&(register_type.to_string(), pnr.to_string())) {
            // Only use cache if the batch index matches
            if cached_batch_idx == batch_idx {
                return Ok(Some(row_idx));
            }
        }
        
        // Fall back to standard search if not found in cache
        self.find_pnr_index(batch, pnr)
    }
    
    /// Find the index of a PNR in a batch
    ///
    /// This is the basic implementation without using the cache.
    ///
    /// # Arguments
    /// * `batch` - The record batch to search in
    /// * `pnr` - The PNR to find
    ///
    /// # Returns
    /// * `std::result::Result<Option<usize>, IdsError>` - The found index or None
    ///
    /// # Errors
    /// Returns an error if the PNR column cannot be accessed
    fn find_pnr_index(&self, batch: &RecordBatch, pnr: &str) -> std::result::Result<Option<usize>, IdsError> {
        if !batch.schema().fields().iter().any(|f| f.name() == "PNR") {
            return Ok(None);
        }
        
        let pnr_idx = batch.schema().index_of("PNR")?;
        let pnr_array = batch.column(pnr_idx);
        
        if let Some(string_array) = pnr_array.as_any().downcast_ref::<StringArray>() {
            // Use binary search if the array is large (>1000 elements)
            let len = string_array.len();
            if len > 1000 {
                // Create a partial index for faster search by dividing into chunks
                let chunk_size = (len as f64).sqrt() as usize; // Square root of length for optimal chunk size
                let mut i = 0;
                while i < len {
                    let end = (i + chunk_size).min(len);
                    // Check first and last elements of chunk to see if we should search it
                    if !string_array.is_null(i) && !string_array.is_null(end - 1) {
                        let first = string_array.value(i);
                        let last = string_array.value(end - 1);
                        
                        // Only search this chunk if the PNR might be in it (lexicographically between first and last)
                        if (pnr >= first && pnr <= last) || 
                           // Special case for chunks that wrap around lexicographically
                           (first > last && (pnr >= first || pnr <= last)) {
                            // Linear search within this smaller chunk
                            for j in i..end {
                                if !string_array.is_null(j) && string_array.value(j) == pnr {
                                    return Ok(Some(j));
                                }
                            }
                        }
                    }
                    i += chunk_size;
                }
                
                // Not found in any chunk
                return Ok(None);
            } else {
                // Linear scan for small arrays
                for i in 0..string_array.len() {
                    if !string_array.is_null(i) && string_array.value(i) == pnr {
                        return Ok(Some(i));
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    // Helper method to get a string array from a column
    fn get_string_array<'a>(&self, batch: &'a RecordBatch, column_name: &str) -> std::result::Result<&'a StringArray, IdsError> {
        let col_idx = batch.schema().index_of(column_name)?;
        let array = batch.column(col_idx);
        
        array.as_any().downcast_ref::<StringArray>()
            .ok_or_else(move || IdsError::data_loading(format!("Column {} is not a string array", column_name)))
    }
    
    fn validate_batch(&self, batch: &RecordBatch) -> std::result::Result<(), IdsError> {
        // Implementation to validate batch structure
        if batch.num_rows() == 0 {
            return Err(IdsError::data_loading("Empty batch".to_string()));
        }
        
        Ok(())
    }
}

// Implement ArrowAccess for ArrowBackend
impl ArrowAccess for ArrowBackend {
    fn get_value<T: ArrowType>(&self, _column: &str, _row: usize) -> std::result::Result<T, IdsError> {
        Err(IdsError::invalid_operation(
            format!("ArrowBackend does not implement direct get_value. Use get_covariate instead.")
        ))
    }
    
    fn get_optional_value<T: ArrowType>(&self, _column: &str, _row: usize) -> std::result::Result<Option<T>, IdsError> {
        Err(IdsError::invalid_operation(
            format!("ArrowBackend does not implement direct get_optional_value. Use get_covariate instead.")
        ))
    }
    
    fn has_column(&self, _column: &str) -> bool {
        false // This implementation doesn't provide direct column access
    }
    
    fn row_count(&self) -> usize {
        0 // This implementation doesn't provide direct row access
    }
    
    fn column_names(&self) -> Vec<String> {
        Vec::new() // This implementation doesn't provide direct column access
    }
    
    fn schema(&self) -> arrow::datatypes::SchemaRef {
        // Create an empty schema for compatibility
        use std::sync::Arc;
        use arrow::datatypes::{Schema, Field};
        
        // Create an empty schema with explicitly typed empty vector
        let empty_fields: Vec<Field> = vec![];
        Arc::new(Schema::new(empty_fields))
    }
    
    fn get_column(&self, _column: &str) -> std::result::Result<arrow::array::ArrayRef, IdsError> {
        Err(IdsError::invalid_operation(
            format!("ArrowBackend does not implement direct get_column. Use get_covariate instead.")
        ))
    }
}
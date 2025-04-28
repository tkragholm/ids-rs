//! Severe Chronic Disease (SCD) algorithm implementation
//!
//! This module implements the Severe Chronic Disease (SCD) algorithm for
//! identifying patients with severe chronic diseases based on ICD-10 diagnosis codes.

use arrow::array::{Array, ArrayRef, BooleanArray, Date32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::error::{IdsError, Result};

/// SCD disease categories with their associated ICD-10 codes
pub struct ScdDiseaseCodes {
    // Map of disease category to set of ICD-10 code prefixes
    codes: HashMap<String, HashSet<String>>,
    // Cached flat set of all codes for quick lookup
    all_codes_cache: HashSet<String>,
}

impl ScdDiseaseCodes {
    /// Create a new `ScdDiseaseCodes` instance with predefined disease categories and codes
    #[must_use] pub fn new() -> Self {
        let mut codes = HashMap::with_capacity(10); // Pre-allocate for the 10 categories
        let mut all_codes_cache = HashSet::with_capacity(150); // Approximate total codes
        
        // Helper function to add a category and its codes
        let mut add_category = |name: &str, code_list: &[&str]| {
            let code_set: HashSet<String> = code_list.iter().map(|s| (*s).to_string()).collect();
            all_codes_cache.extend(code_set.iter().cloned());
            codes.insert(name.to_string(), code_set);
        };
        
        // Blood Disorders
        add_category("blood_disorders", &[
            "D55", "D56", "D57", "D58", "D59", "D60", "D61",
            "D64", "D65", "D66", "D67", "D68", "D69", "D70", "D71", "D72", "D73",
            "D76"
        ]);
        
        // Immune System Disorders
        add_category("immune_system", &[
            "D80", "D81", "D82", "D83", "D84", "D86", "D89"
        ]);
        
        // Endocrine Disorders
        add_category("endocrine", &[
            "E22", "E23", "E24", "E25", "E26", "E27", "E31", "E34",
            "E70", "E71", "E72", "E73", "E74", "E75", "E76", "E77", 
            "E78", "E79", "E80", "E83", "E84", "E85", "E88"
        ]);
        
        // Neurological Disorders
        add_category("neurological", &[
            "F84", "G11", "G12", "G13", "G23", "G24", "G25", "G31", 
            "G40", "G41", "G70", "G71", "G72", "G80", "G81", "G82"
        ]);
        
        // Cardiovascular Disorders
        add_category("cardiovascular", &[
            "I27", "I42", "I43", "I50", "I81", "I82", "I83"
        ]);
        
        // Respiratory Disorders
        add_category("respiratory", &[
            "J41", "J42", "J43", "J44", "J45", "J47", "J60", "J61", "J62", 
            "J63", "J64", "J65", "J66", "J67", "J68", "J69", "J70", "J84", "J96"
        ]);
        
        // Gastrointestinal Disorders
        add_category("gastrointestinal", &[
            "K50", "K51", "K73", "K74", "K86", "K87", "K90"
        ]);
        
        // Musculoskeletal Disorders
        add_category("musculoskeletal", &[
            "M05", "M06", "M07", "M08", "M09", "M30", "M31", "M32", "M33",
            "M34", "M35", "M40", "M41", "M42", "M43", "M45", "M46"
        ]);
        
        // Renal Disorders
        add_category("renal", &[
            "N01", "N02", "N03", "N04", "N05", "N06", "N07", "N08", 
            "N11", "N12", "N13", "N14", "N15", "N16", "N18", "N19", 
            "N20", "N21", "N22", "N23", "N24", "N25", "N26", "N27", "N28", "N29"
        ]);
        
        // Congenital Disorders
        add_category("congenital", &[
            "P27", "Q01", "Q02", "Q03", "Q04", "Q05", "Q06", "Q07", 
            "Q20", "Q21", "Q22", "Q23", "Q24", "Q25", "Q26", "Q27", "Q28",
            "Q30", "Q31", "Q32", "Q33", "Q34", "Q35", "Q36", "Q37", 
            "Q38", "Q39", "Q40", "Q41", "Q42", "Q43", "Q44", "Q45", 
            "Q60", "Q61", "Q62", "Q63", "Q64", "Q77", "Q78", "Q79", 
            "Q80", "Q81", "Q82", "Q83", "Q84", "Q85", "Q86", "Q87", "Q89"
        ]);
        
        Self { codes, all_codes_cache }
    }
    
    /// Get all SCD codes as a flat set (returns a reference to pre-computed set)
    #[must_use] pub fn all_codes(&self) -> &HashSet<String> {
        &self.all_codes_cache
    }
    
    /// Helper method to extract normalized prefix from a diagnosis code
    fn extract_prefix(&self, diagnosis: &str) -> Option<String> {
        let diagnosis = diagnosis.trim();
        if diagnosis.is_empty() || diagnosis.len() < 3 {
            return None;
        }
        
        // Fast path for ASCII strings (most common case)
        if diagnosis.is_ascii() {
            let mut prefix = String::with_capacity(3);
            for c in diagnosis[..3].chars() {
                prefix.push(c.to_ascii_uppercase());
            }
            return Some(prefix);
        }
        
        // Fallback for non-ASCII (rare case)
        let prefix_chars: Vec<char> = diagnosis[..3].chars().collect();
        if prefix_chars.len() < 3 {
            return None;
        }
        
        let mut prefix = String::with_capacity(3);
        for c in prefix_chars {
            prefix.push(c.to_ascii_uppercase());
        }
        
        Some(prefix)
    }

    /// Check if a diagnosis code is a SCD code
    #[must_use] pub fn is_scd_code(&self, diagnosis: &str) -> bool {
        if let Some(prefix) = self.extract_prefix(diagnosis) {
            // Check if it's in the SCD codes set - use exact match for O(1) lookup
            self.all_codes_cache.contains(&prefix)
        } else {
            false
        }
    }
    
    /// Determine the disease category for a diagnosis code
    #[must_use] pub fn get_disease_category(&self, diagnosis: &str) -> Option<String> {
        // Use the common prefix extraction logic
        self.extract_prefix(diagnosis).and_then(|prefix| {
            // Find the category with this code (use iterators for early return)
            self.codes.iter()
                .find(|(_, codes)| codes.contains(&prefix))
                .map(|(category, _)| category.clone())
        })
    }
    
    /// Get a map of all disease categories and their codes
    #[must_use] pub fn get_all_categories(&self) -> &HashMap<String, HashSet<String>> {
        &self.codes
    }
}

impl Default for ScdDiseaseCodes {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for the SCD algorithm
pub struct ScdConfig {
    /// Columns containing diagnosis codes to check
    pub diagnosis_columns: Vec<String>,
    /// Column containing the date when the diagnosis was made
    pub date_column: String,
    /// Column containing the patient identifier
    pub patient_id_column: String,
}

impl Default for ScdConfig {
    fn default() -> Self {
        Self {
            diagnosis_columns: vec!["primary_diagnosis".to_string(), "diagnosis".to_string()],
            date_column: "admission_date".to_string(),
            patient_id_column: "patient_id".to_string(),
        }
    }
}

/// Result of the SCD algorithm for a single patient
#[derive(Debug, Clone)]
pub struct ScdResult {
    /// Patient identifier
    pub patient_id: String,
    /// Whether the patient has any SCD diagnosis
    pub is_scd: bool,
    /// Date of the earliest SCD diagnosis
    pub first_scd_date: Option<NaiveDate>,
    /// Map of disease categories to whether the patient has a diagnosis in that category
    pub disease_categories: HashMap<String, bool>,
}

/// Apply the SCD algorithm to a record batch
pub fn apply_scd_algorithm(
    health_data: &RecordBatch, 
    config: &ScdConfig
) -> Result<Vec<ScdResult>> {
    let scd_codes = ScdDiseaseCodes::new();
    
    // Get column indices
    let patient_id_idx = health_data.schema()
        .index_of(&config.patient_id_column)
        .map_err(|e| IdsError::Data(format!("Patient ID column not found: {e}")))?;
    
    let date_idx = health_data.schema()
        .index_of(&config.date_column)
        .map_err(|e| IdsError::Data(format!("Date column not found: {e}")))?;
    
    // Get arrays
    let patient_id_array = health_data.column(patient_id_idx);
    let patient_id_array = patient_id_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("Patient ID column is not a string array".to_string()))?;
    
    let date_array = health_data.column(date_idx);
    let date_array = date_array
        .as_any()
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("Date column is not a date array".to_string()))?;
    
    // Maps to store intermediate results - pre-allocate with estimated capacity
    let total_rows = health_data.num_rows();
    let estimated_capacity = total_rows / 5; // Assuming roughly 20% of patients have SCD codes
    
    let mut patient_scd = HashMap::with_capacity(estimated_capacity);
    let mut patient_first_date = HashMap::with_capacity(estimated_capacity);
    let mut patient_categories = HashMap::with_capacity(estimated_capacity);
    
    // Get diagnosis column indices
    let mut diag_indices = Vec::new();
    for diag_col in &config.diagnosis_columns {
        if let Ok(idx) = health_data.schema().index_of(diag_col) {
            diag_indices.push(idx);
        } else {
            log::warn!("Diagnosis column not found: {diag_col}");
        }
    }
    
    if diag_indices.is_empty() {
        return Err(IdsError::Data("No valid diagnosis columns found".to_string()));
    }
    
    // Pre-fetch diagnosis arrays for better cache locality
    let mut diag_arrays = Vec::with_capacity(diag_indices.len());
    for &idx in &diag_indices {
        let array = health_data.column(idx);
        if let Some(string_array) = array.as_any().downcast_ref::<StringArray>() {
            diag_arrays.push(string_array);
        } else {
            log::warn!("Diagnosis column is not a string array");
            continue;
        }
    }
    
    // Process in chunks for better cache efficiency
    const CHUNK_SIZE: usize = 10000;
    let total_chunks = total_rows.div_ceil(CHUNK_SIZE);
    
    for chunk_index in 0..total_chunks {
        let start_idx = chunk_index * CHUNK_SIZE;
        let end_idx = (start_idx + CHUNK_SIZE).min(total_rows);
        
        // Process each row in this chunk
        for row_idx in start_idx..end_idx {
            // Skip if patient ID is null
            if patient_id_array.is_null(row_idx) {
                continue;
            }
            
            let patient_id = patient_id_array.value(row_idx).to_string();
            let mut is_scd_row = false;
            let mut categories = HashSet::new();
            
            // Check each diagnosis column
            for diag_array in &diag_arrays {
                // Skip if diagnosis is null
                if diag_array.is_null(row_idx) {
                    continue;
                }
                
                let diagnosis = diag_array.value(row_idx);
                
                // Check if it's a SCD code
                if scd_codes.is_scd_code(diagnosis) {
                    is_scd_row = true;
                    
                    // Add disease category if present
                    if let Some(category) = scd_codes.get_disease_category(diagnosis) {
                        categories.insert(category);
                    }
                }
            }
            
            // If SCD code found in this row, update patient results
            if is_scd_row {
                // Set the patient as having SCD
                patient_scd.insert(patient_id.clone(), true);
                
                // Get the date for this row
                if !date_array.is_null(row_idx) {
                    let days_since_epoch = date_array.value(row_idx);
                    let date = NaiveDate::from_num_days_from_ce_opt(days_since_epoch + 719163)
                        .unwrap_or_default();
                    
                    // Update first SCD date if this is earlier or not set yet
                    if let Some(existing_date) = patient_first_date.get(&patient_id) {
                        if date < *existing_date {
                            patient_first_date.insert(patient_id.clone(), date);
                        }
                    } else {
                        patient_first_date.insert(patient_id.clone(), date);
                    }
                }
                
                // Update disease categories
                let patient_cats = patient_categories.entry(patient_id).or_insert_with(HashSet::new);
                for category in categories {
                    patient_cats.insert(category);
                }
            }
        }
        
        // Log progress every few chunks
        if chunk_index % 10 == 0 || chunk_index == total_chunks - 1 {
            log::debug!("Processed SCD chunk {}/{} ({:.1}%)", 
                        chunk_index + 1, total_chunks, 
                        (chunk_index + 1) as f64 * 100.0 / total_chunks as f64);
        }
    }
    
    // Create disease category map for each patient
    let all_categories: HashSet<String> = scd_codes.get_all_categories().keys().cloned().collect();
    
    // Create a set of patients that have SCD
    let scd_patient_ids: HashSet<String> = patient_scd.keys().cloned().collect();
    
    // Build unique patient set more efficiently in a single pass
    let mut unique_patients = HashSet::with_capacity(total_rows / 2);
    for i in 0..patient_id_array.len() {
        if !patient_id_array.is_null(i) {
            unique_patients.insert(patient_id_array.value(i).to_string());
        }
    }
    
    // Pre-allocate results vector
    let mut results = Vec::with_capacity(unique_patients.len());
    
    // Convert to ScdResult objects for patients with SCD
    for (patient_id, is_scd) in patient_scd {
        let mut disease_categories = HashMap::with_capacity(all_categories.len());
        
        // Initialize all categories to false
        for category in &all_categories {
            disease_categories.insert(category.clone(), false);
        }
        
        // Set the patient's categories to true
        if let Some(categories) = patient_categories.get(&patient_id) {
            for category in categories {
                disease_categories.insert(category.clone(), true);
            }
        }
        
        let first_scd_date = patient_first_date.get(&patient_id).copied();
        
        results.push(ScdResult {
            patient_id,
            is_scd,
            first_scd_date,
            disease_categories,
        });
    }
    
    log::debug!("Found {} patients with SCD", results.len());
    
    // Add patients with no SCD
    let patients_without_scd: Vec<_> = unique_patients
        .into_iter()
        .filter(|id| !scd_patient_ids.contains(id))
        .collect();
    
    log::debug!("Adding {} patients without SCD", patients_without_scd.len());
    
    for patient_id in patients_without_scd {
        let mut disease_categories = HashMap::with_capacity(all_categories.len());
        for category in &all_categories {
            disease_categories.insert(category.clone(), false);
        }
        
        results.push(ScdResult {
            patient_id,
            is_scd: false,
            first_scd_date: None,
            disease_categories,
        });
    }
    
    Ok(results)
}

/// Convert SCD results to a record batch
pub fn scd_results_to_record_batch(results: &[ScdResult]) -> Result<RecordBatch> {
    if results.is_empty() {
        return Err(IdsError::Data("No SCD results to convert to RecordBatch".to_string()));
    }
    
    // Extract all unique disease categories
    let mut all_categories = HashSet::new();
    for result in results {
        all_categories.extend(result.disease_categories.keys().cloned());
    }
    let categories: Vec<String> = all_categories.into_iter().collect();
    
    // Create arrays for each column
    let mut patient_ids = Vec::with_capacity(results.len());
    let mut is_scd_values = Vec::with_capacity(results.len());
    let mut first_scd_dates = Vec::with_capacity(results.len());
    
    // Create category arrays
    let mut category_arrays = Vec::with_capacity(categories.len());
    for _ in &categories {
        category_arrays.push(Vec::with_capacity(results.len()));
    }
    
    // Fill arrays
    for result in results {
        patient_ids.push(Some(result.patient_id.clone()));
        is_scd_values.push(Some(result.is_scd));
        
        // Convert NaiveDate to days since epoch for Date32Array
        if let Some(date) = result.first_scd_date {
            let days = (date.signed_duration_since(
                NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
            ).num_days()) as i32;
            first_scd_dates.push(Some(days));
        } else {
            first_scd_dates.push(None);
        }
        
        // Fill category values
        for (i, category) in categories.iter().enumerate() {
            let value = *result.disease_categories.get(category).unwrap_or(&false);
            category_arrays[i].push(Some(value));
        }
    }
    
    // Create Arrow arrays
    let patient_id_array = Arc::new(StringArray::from(patient_ids)) as ArrayRef;
    let is_scd_array = Arc::new(BooleanArray::from(is_scd_values)) as ArrayRef;
    let first_scd_date_array = Arc::new(Date32Array::from(first_scd_dates)) as ArrayRef;
    
    // Create Arrow arrays for categories
    let mut arrow_category_arrays = Vec::with_capacity(categories.len());
    for values in category_arrays {
        arrow_category_arrays.push(Arc::new(BooleanArray::from(values)) as ArrayRef);
    }
    
    // Build fields
    let mut fields = vec![
        Field::new("patient_id", DataType::Utf8, false),
        Field::new("is_scd", DataType::Boolean, true),
        Field::new("first_scd_date", DataType::Date32, true),
    ];
    
    // Add category fields
    for category in &categories {
        let field_name = format!("category_{category}");
        fields.push(Field::new(field_name, DataType::Boolean, true));
    }
    
    // Create schema
    let schema = Arc::new(Schema::new(fields));
    
    // Create column arrays
    let mut columns = Vec::with_capacity(3 + categories.len());
    columns.push(patient_id_array);
    columns.push(is_scd_array);
    columns.push(first_scd_date_array);
    columns.extend(arrow_category_arrays);
    
    // Create record batch
    let batch = RecordBatch::try_new(schema, columns)
        .map_err(|e| IdsError::Data(format!("Error creating SCD results batch: {e}")))?;
    
    Ok(batch)
}
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
}

impl ScdDiseaseCodes {
    /// Create a new `ScdDiseaseCodes` instance with predefined disease categories and codes
    #[must_use] pub fn new() -> Self {
        let mut codes = HashMap::new();
        
        // Blood Disorders
        codes.insert("blood_disorders".to_string(), [
            "D55", "D56", "D57", "D58", "D59", "D60", "D61",
            "D64", "D65", "D66", "D67", "D68", "D69", "D70", "D71", "D72", "D73",
            "D76"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Immune System Disorders
        codes.insert("immune_system".to_string(), [
            "D80", "D81", "D82", "D83", "D84", "D86", "D89"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Endocrine Disorders
        codes.insert("endocrine".to_string(), [
            "E22", "E23", "E24", "E25", "E26", "E27", "E31", "E34",
            "E70", "E71", "E72", "E73", "E74", "E75", "E76", "E77", 
            "E78", "E79", "E80", "E83", "E84", "E85", "E88"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Neurological Disorders
        codes.insert("neurological".to_string(), [
            "F84", "G11", "G12", "G13", "G23", "G24", "G25", "G31", 
            "G40", "G41", "G70", "G71", "G72", "G80", "G81", "G82"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Cardiovascular Disorders
        codes.insert("cardiovascular".to_string(), [
            "I27", "I42", "I43", "I50", "I81", "I82", "I83"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Respiratory Disorders
        codes.insert("respiratory".to_string(), [
            "J41", "J42", "J43", "J44", "J45", "J47", "J60", "J61", "J62", 
            "J63", "J64", "J65", "J66", "J67", "J68", "J69", "J70", "J84", "J96"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Gastrointestinal Disorders
        codes.insert("gastrointestinal".to_string(), [
            "K50", "K51", "K73", "K74", "K86", "K87", "K90"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Musculoskeletal Disorders
        codes.insert("musculoskeletal".to_string(), [
            "M05", "M06", "M07", "M08", "M09", "M30", "M31", "M32", "M33",
            "M34", "M35", "M40", "M41", "M42", "M43", "M45", "M46"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Renal Disorders
        codes.insert("renal".to_string(), [
            "N01", "N02", "N03", "N04", "N05", "N06", "N07", "N08", 
            "N11", "N12", "N13", "N14", "N15", "N16", "N18", "N19", 
            "N20", "N21", "N22", "N23", "N24", "N25", "N26", "N27", "N28", "N29"
        ].iter().map(|s| (*s).to_string()).collect());
        
        // Congenital Disorders
        codes.insert("congenital".to_string(), [
            "P27", "Q01", "Q02", "Q03", "Q04", "Q05", "Q06", "Q07", 
            "Q20", "Q21", "Q22", "Q23", "Q24", "Q25", "Q26", "Q27", "Q28",
            "Q30", "Q31", "Q32", "Q33", "Q34", "Q35", "Q36", "Q37", 
            "Q38", "Q39", "Q40", "Q41", "Q42", "Q43", "Q44", "Q45", 
            "Q60", "Q61", "Q62", "Q63", "Q64", "Q77", "Q78", "Q79", 
            "Q80", "Q81", "Q82", "Q83", "Q84", "Q85", "Q86", "Q87", "Q89"
        ].iter().map(|s| (*s).to_string()).collect());
        
        Self { codes }
    }
    
    /// Get all SCD codes as a flat set
    #[must_use] pub fn all_codes(&self) -> HashSet<String> {
        let mut all_codes = HashSet::new();
        for codes in self.codes.values() {
            all_codes.extend(codes.iter().cloned());
        }
        all_codes
    }
    
    /// Check if a diagnosis code is a SCD code
    #[must_use] pub fn is_scd_code(&self, diagnosis: &str) -> bool {
        let diagnosis = diagnosis.trim();
        if diagnosis.is_empty() {
            return false;
        }
        
        // Standardize and extract prefix (first 3 characters after any letter prefix)
        let diag_upper = diagnosis.to_uppercase();
        
        // ICD-10 codes typically have a letter prefix followed by numbers
        // Extract the letter and first two digits (e.g., "D55" from "D551A")
        if diag_upper.len() >= 3 {
            let prefix = &diag_upper[..3];
            return self.all_codes().contains(prefix);
        }
        
        false
    }
    
    /// Determine the disease category for a diagnosis code
    #[must_use] pub fn get_disease_category(&self, diagnosis: &str) -> Option<String> {
        let diagnosis = diagnosis.trim();
        if diagnosis.is_empty() {
            return None;
        }
        
        // Standardize and extract prefix
        let diag_upper = diagnosis.to_uppercase();
        
        if diag_upper.len() >= 3 {
            let prefix = &diag_upper[..3];
            
            for (category, codes) in &self.codes {
                if codes.contains(prefix) {
                    return Some(category.clone());
                }
            }
        }
        
        None
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
    let mut results = Vec::new();
    
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
    
    // Maps to store intermediate results
    let mut patient_scd = HashMap::new();
    let mut patient_first_date = HashMap::new();
    let mut patient_categories = HashMap::new();
    
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
    
    // Process each row
    for row_idx in 0..health_data.num_rows() {
        // Skip if patient ID is null
        if patient_id_array.is_null(row_idx) {
            continue;
        }
        
        let patient_id = patient_id_array.value(row_idx).to_string();
        let mut is_scd_row = false;
        let mut categories = HashSet::new();
        
        // Check each diagnosis column
        for &diag_idx in &diag_indices {
            let diag_array = health_data.column(diag_idx);
            let diag_array = if let Some(array) = diag_array.as_any().downcast_ref::<StringArray>() { array } else {
                log::warn!("Diagnosis column is not a string array");
                continue;
            };
            
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
    
    // Create disease category map for each patient
    let all_categories: HashSet<String> = scd_codes.get_all_categories().keys().cloned().collect();
    
    // Create a set of patients that have SCD
    let scd_patient_ids: HashSet<String> = patient_scd.keys().cloned().collect();
    
    // Convert to ScdResult objects
    for (patient_id, is_scd) in patient_scd {
        let mut disease_categories = HashMap::new();
        
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
    
    // Add patients with no SCD
    let mut unique_patients = HashSet::new();
    for i in 0..patient_id_array.len() {
        if !patient_id_array.is_null(i) {
            unique_patients.insert(patient_id_array.value(i).to_string());
        }
    }
    
    for patient_id in unique_patients {
        if !scd_patient_ids.contains(&patient_id) {
            let mut disease_categories = HashMap::new();
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
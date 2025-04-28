//! LPR (Landspatientregistret) data processing algorithms
//!
//! This module implements data processing operations for the Danish National Patient Registry (LPR)
//! including integration of LPR2 and LPR3 data, data harmonization, and preparation for SCD analysis.

use arrow::array::{Array, BooleanArray, Date32Array, StringArray};
use arrow::compute::concat_batches;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{IdsError, Result};
use crate::utils::date_utils;

/// Configuration for LPR data processing
pub struct LprConfig {
    /// Whether to include LPR2 data
    pub include_lpr2: bool,
    /// Whether to include LPR3 data
    pub include_lpr3: bool,
    /// Start date for filtering (inclusive)
    pub start_date: Option<NaiveDate>,
    /// End date for filtering (inclusive)
    pub end_date: Option<NaiveDate>,
}

impl Default for LprConfig {
    fn default() -> Self {
        Self {
            include_lpr2: true,
            include_lpr3: true,
            start_date: None,
            end_date: None,
        }
    }
}

/// Merges multiple record batches into one
fn merge_batches(batches: &[RecordBatch], data_name: &str) -> Result<RecordBatch> {
    if batches.is_empty() {
        return Err(IdsError::Validation(format!("No {data_name} data provided")));
    } else if batches.len() == 1 {
        return Ok(batches[0].clone());
    }

    let schema = batches[0].schema();
    concat_batches(&schema, batches)
        .map_err(|e| IdsError::Data(format!("Failed to merge {data_name} batches: {e}")))
}

/// Extracts a string value from a record batch column
fn get_string_column(batch: &RecordBatch, column_name: &str) -> Result<Arc<StringArray>> {
    let col_idx = batch.schema().index_of(column_name)
        .map_err(|e| IdsError::Data(format!("{column_name} column not found: {e}")))?;
    
    let col_array = batch.column(col_idx);
    col_array.as_any().downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data(format!("{column_name} column is not a string array")))
        .map(|a| Arc::new(a.clone()))
}

/// Builds a map of record number to row index
fn build_recnum_index(recnum_array: &StringArray) -> HashMap<String, usize> {
    let mut recnum_to_row = HashMap::new();
    for i in 0..recnum_array.len() {
        if recnum_array.is_null(i) {
            continue;
        }
        let recnum = recnum_array.value(i);
        recnum_to_row.insert(recnum.to_string(), i);
    }
    recnum_to_row
}

/// Maps diagnoses with their types by record number
fn map_diagnoses_by_recnum(
    recnum_array: &StringArray,
    diag_array: &StringArray,
    diag_type_array: &StringArray,
) -> HashMap<String, Vec<(String, String)>> {
    let mut recnum_to_diagnoses = HashMap::new();
    
    for i in 0..recnum_array.len() {
        if recnum_array.is_null(i) || diag_array.is_null(i) {
            continue;
        }
        
        let recnum = recnum_array.value(i);
        let diagnosis = diag_array.value(i);
        let diag_type = if diag_type_array.is_null(i) {
            "A" // Default to 'A' (action diagnosis) if not specified
        } else {
            diag_type_array.value(i)
        };

        let diagnoses = recnum_to_diagnoses
            .entry(recnum.to_string())
            .or_insert_with(Vec::new);

        diagnoses.push((diagnosis.to_string(), diag_type.to_string()));
    }
    
    recnum_to_diagnoses
}

/// Maps treatment dates by record number
fn map_treatment_dates(recnum_array: &StringArray, date_array: &Date32Array) -> HashMap<String, Vec<i32>> {
    let mut recnum_to_dates = HashMap::new();
    
    for i in 0..recnum_array.len() {
        if recnum_array.is_null(i) || date_array.is_null(i) {
            continue;
        }
        
        let recnum = recnum_array.value(i);
        let date_i32 = date_array.value(i);

        let dates = recnum_to_dates
            .entry(recnum.to_string())
            .or_insert_with(Vec::new);

        dates.push(date_i32);
    }
    
    recnum_to_dates
}

/// Processes secondary diagnoses for a given record
fn process_secondary_diagnoses(
    diagnoses: &[(String, String)],
) -> Option<String> {
    // Filter to only include secondary diagnoses (not 'A' type)
    let secondary: Vec<String> = diagnoses
        .iter()
        .filter(|(_, diag_type)| diag_type != "A")
        .map(|(diag, _)| diag.clone())
        .collect();

    if secondary.is_empty() {
        None
    } else {
        // Join all secondary diagnoses with semicolons
        Some(secondary.join(";"))
    }
}

/// Creates the integrated record batch schema (common for both LPR2 and LPR3)
fn create_integrated_schema() -> Schema {
    Schema::new(vec![
        Field::new("patient_id", DataType::Utf8, true),
        Field::new("primary_diagnosis", DataType::Utf8, true),
        Field::new("secondary_diagnosis", DataType::Utf8, true),
        Field::new("admission_date", DataType::Date32, true),
        Field::new("discharge_date", DataType::Date32, true),
        Field::new("hospital_code", DataType::Utf8, true),
        Field::new("department_code", DataType::Utf8, true),
        Field::new("admission_type", DataType::Utf8, true),
    ])
}

/// Integrate LPR2 components (`LPR_ADM`, `LPR_DIAG`, and optionally `LPR_BES`)
pub fn integrate_lpr2_components(
    lpr_adm: &[RecordBatch],
    lpr_diag: &[RecordBatch],
    lpr_bes: Option<&[RecordBatch]>,
) -> Result<RecordBatch> {
    // First merge all batches
    let lpr_adm = merge_batches(lpr_adm, "LPR_ADM")?;
    let lpr_diag = merge_batches(lpr_diag, "LPR_DIAG")?;
    let lpr_bes = lpr_bes.map(|batches| merge_batches(batches, "LPR_BES")).transpose()?;

    // Get required columns from LPR_ADM
    let recnum_array = get_string_column(&lpr_adm, "RECNUM")?;
    let pnr_array = get_string_column(&lpr_adm, "PNR")?;
    let primary_diag_array = get_string_column(&lpr_adm, "C_ADIAG")?;
    let hospital_array = get_string_column(&lpr_adm, "C_SGH")?;
    let dept_array = get_string_column(&lpr_adm, "C_AFD")?;
    let pat_type_array = get_string_column(&lpr_adm, "C_PATTYPE")?;

    // Extract date columns
    let adm_date_idx = lpr_adm.schema().index_of("D_INDDTO")
        .map_err(|e| IdsError::Data(format!("D_INDDTO column not found in LPR_ADM: {e}")))?;
    let adm_date_array = lpr_adm.column(adm_date_idx);
    let adm_date_array = adm_date_array.as_any().downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("D_INDDTO column is not a date array".to_string()))?;

    let disc_date_idx = lpr_adm.schema().index_of("D_UDDTO")
        .map_err(|e| IdsError::Data(format!("D_UDDTO column not found in LPR_ADM: {e}")))?;
    let disc_date_array = lpr_adm.column(disc_date_idx);
    let disc_date_array = disc_date_array.as_any().downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("D_UDDTO column is not a date array".to_string()))?;

    // Build index from RECNUM to row
    let _recnum_to_row = build_recnum_index(&recnum_array);

    // Create a map from RECNUM to diagnoses in LPR_DIAG
    let recnum_array_diag = get_string_column(&lpr_diag, "RECNUM")?;
    let diag_array = get_string_column(&lpr_diag, "C_DIAG")?;
    let diag_type_array = get_string_column(&lpr_diag, "C_DIAGTYPE")?;
    
    let recnum_to_diagnoses = map_diagnoses_by_recnum(&recnum_array_diag, &diag_array, &diag_type_array);

    // Process LPR_BES data if available to get treatment dates
    if let Some(lpr_bes) = lpr_bes.as_ref() {
        // Get columns from LPR_BES
        let recnum_array_bes = get_string_column(lpr_bes, "RECNUM")?;
        
        // Get treatment date (D_AMBDTO) from LPR_BES
        let date_idx_bes = lpr_bes.schema().index_of("D_AMBDTO")
            .map_err(|e| IdsError::Data(format!("D_AMBDTO column not found in LPR_BES: {e}")))?;
        let date_array_bes = lpr_bes.column(date_idx_bes);
        
        // Try to convert the date column to a Date32Array regardless of its original type
        log::debug!("D_AMBDTO column type: {:?}", date_array_bes.data_type());
        
        // Use our more flexible date conversion utility
        let date_array_bes = date_utils::convert_to_date32_array(date_array_bes.as_ref())?;
        
        // Map RECNUM to treatment dates - currently not using this, but keeping for future use
        let _treatment_dates = map_treatment_dates(&recnum_array_bes, &date_array_bes);
    }

    // Prepare arrays for integrated data
    let num_rows = lpr_adm.num_rows();
    let mut patient_ids = Vec::with_capacity(num_rows);
    let mut primary_diagnoses = Vec::with_capacity(num_rows);
    let mut secondary_diagnoses = Vec::with_capacity(num_rows);
    let mut admission_dates = Vec::with_capacity(num_rows);
    let mut discharge_dates = Vec::with_capacity(num_rows);
    let mut hospital_codes = Vec::with_capacity(num_rows);
    let mut department_codes = Vec::with_capacity(num_rows);
    let mut admission_types = Vec::with_capacity(num_rows);

    // Process each row in LPR_ADM and add combined data
    for i in 0..num_rows {
        let recnum = if recnum_array.is_null(i) {
            continue; // Skip rows without RECNUM
        } else {
            recnum_array.value(i)
        };

        // Add patient ID
        let pnr = if pnr_array.is_null(i) {
            None
        } else {
            Some(pnr_array.value(i).to_string())
        };
        patient_ids.push(pnr);

        // Add primary diagnosis
        let primary_diag = if primary_diag_array.is_null(i) {
            None
        } else {
            Some(primary_diag_array.value(i).to_string())
        };
        primary_diagnoses.push(primary_diag);

        // Get secondary diagnoses for this RECNUM
        let secondary_diag = recnum_to_diagnoses.get(recnum)
            .and_then(|diagnoses| process_secondary_diagnoses(diagnoses));
        secondary_diagnoses.push(secondary_diag);

        // Add dates
        admission_dates.push(if adm_date_array.is_null(i) {
            None
        } else {
            Some(adm_date_array.value(i))
        });

        discharge_dates.push(if disc_date_array.is_null(i) {
            None
        } else {
            Some(disc_date_array.value(i))
        });

        // Add hospital and department codes
        hospital_codes.push(if hospital_array.is_null(i) {
            None
        } else {
            Some(hospital_array.value(i).to_string())
        });

        department_codes.push(if dept_array.is_null(i) {
            None
        } else {
            Some(dept_array.value(i).to_string())
        });

        // Add admission type
        admission_types.push(if pat_type_array.is_null(i) {
            None
        } else {
            Some(pat_type_array.value(i).to_string())
        });
    }

    // Create batch with common schema
    create_record_batch(
        patient_ids,
        primary_diagnoses,
        secondary_diagnoses,
        admission_dates,
        discharge_dates,
        hospital_codes,
        department_codes,
        admission_types,
        "LPR2",
    )
}

/// Creates a record batch from the collected vectors of data
fn create_record_batch(
    patient_ids: Vec<Option<String>>,
    primary_diagnoses: Vec<Option<String>>,
    secondary_diagnoses: Vec<Option<String>>,
    admission_dates: Vec<Option<i32>>,
    discharge_dates: Vec<Option<i32>>,
    hospital_codes: Vec<Option<String>>,
    department_codes: Vec<Option<String>>,
    admission_types: Vec<Option<String>>,
    source_name: &str,
) -> Result<RecordBatch> {
    // Create schema
    let schema = Arc::new(create_integrated_schema());
    
    // Create arrays
    let patient_id_array = Arc::new(StringArray::from(patient_ids));
    let primary_diag_array = Arc::new(StringArray::from(primary_diagnoses));
    let secondary_diag_array = Arc::new(StringArray::from(secondary_diagnoses));
    let admission_date_array = Arc::new(Date32Array::from(admission_dates));
    let discharge_date_array = Arc::new(Date32Array::from(discharge_dates));
    let hospital_code_array = Arc::new(StringArray::from(hospital_codes));
    let department_code_array = Arc::new(StringArray::from(department_codes));
    let admission_type_array = Arc::new(StringArray::from(admission_types));

    // Create batch
    RecordBatch::try_new(
        schema,
        vec![
            patient_id_array,
            primary_diag_array,
            secondary_diag_array,
            admission_date_array,
            discharge_date_array,
            hospital_code_array,
            department_code_array,
            admission_type_array,
        ],
    )
    .map_err(|e| IdsError::Data(format!("Failed to create integrated {source_name} batch: {e}")))
}

/// Integrate LPR3 components (`LPR3_KONTAKTER` and `LPR3_DIAGNOSER`)
pub fn integrate_lpr3_components(
    lpr3_kontakter: &[RecordBatch],
    lpr3_diagnoser: &[RecordBatch],
) -> Result<RecordBatch> {
    // First merge all batches
    let lpr3_kontakter = merge_batches(lpr3_kontakter, "LPR3_KONTAKTER")?;
    let lpr3_diagnoser = merge_batches(lpr3_diagnoser, "LPR3_DIAGNOSER")?;

    // Get required columns from LPR3_DIAGNOSER
    let kontakt_id_diag = get_string_column(&lpr3_diagnoser, "DW_EK_KONTAKT")?;
    let diag_array = get_string_column(&lpr3_diagnoser, "diagnosekode")?;
    let diag_type_array = get_string_column(&lpr3_diagnoser, "diagnosetype")?;
    
    // Create a map from DW_EK_KONTAKT to diagnoses
    let kontakt_to_diagnoses = map_diagnoses_by_recnum(&kontakt_id_diag, &diag_array, &diag_type_array);

    // Get columns from LPR3_KONTAKTER
    let kontakt_id_array = get_string_column(&lpr3_kontakter, "DW_EK_KONTAKT")?;
    let patient_id_array = get_string_column(&lpr3_kontakter, "CPR")?;
    let action_diag_array = get_string_column(&lpr3_kontakter, "aktionsdiagnose")?;
    let org_unit_array = get_string_column(&lpr3_kontakter, "SORENHED_ANS")?;
    let contact_type_array = get_string_column(&lpr3_kontakter, "kontakttype")?;
    
    // Get date columns
    let start_date_idx = lpr3_kontakter.schema().index_of("dato_start")
        .map_err(|e| IdsError::Data(format!("dato_start column not found in LPR3_KONTAKTER: {e}")))?;
    let start_date_array = lpr3_kontakter.column(start_date_idx);
    let start_date_array = start_date_array.as_any().downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("dato_start column is not a date array".to_string()))?;

    let end_date_idx = lpr3_kontakter.schema().index_of("dato_slut")
        .map_err(|e| IdsError::Data(format!("dato_slut column not found in LPR3_KONTAKTER: {e}")))?;
    let end_date_array = lpr3_kontakter.column(end_date_idx);
    let end_date_array = end_date_array.as_any().downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("dato_slut column is not a date array".to_string()))?;

    // Prepare arrays for integrated data
    let num_rows = lpr3_kontakter.num_rows();
    let mut patient_ids = Vec::with_capacity(num_rows);
    let mut primary_diagnoses = Vec::with_capacity(num_rows);
    let mut secondary_diagnoses = Vec::with_capacity(num_rows);
    let mut admission_dates = Vec::with_capacity(num_rows);
    let mut discharge_dates = Vec::with_capacity(num_rows);
    let mut hospital_codes = Vec::with_capacity(num_rows);
    let mut department_codes = Vec::with_capacity(num_rows);
    let mut admission_types = Vec::with_capacity(num_rows);

    // Process each row in LPR3_KONTAKTER
    for i in 0..num_rows {
        let kontakt_id = if kontakt_id_array.is_null(i) {
            continue; // Skip rows without contact ID
        } else {
            kontakt_id_array.value(i)
        };

        // Add patient ID
        let patient_id = if patient_id_array.is_null(i) {
            None
        } else {
            Some(patient_id_array.value(i).to_string())
        };
        patient_ids.push(patient_id);

        // Add primary diagnosis
        let primary_diag = if action_diag_array.is_null(i) {
            None
        } else {
            Some(action_diag_array.value(i).to_string())
        };
        primary_diagnoses.push(primary_diag);

        // Get secondary diagnoses for this contact
        let secondary_diag = kontakt_to_diagnoses.get(kontakt_id)
            .and_then(|diagnoses| process_secondary_diagnoses(diagnoses));
        secondary_diagnoses.push(secondary_diag);

        // Add dates
        admission_dates.push(if start_date_array.is_null(i) {
            None
        } else {
            Some(start_date_array.value(i))
        });

        discharge_dates.push(if end_date_array.is_null(i) {
            None
        } else {
            Some(end_date_array.value(i))
        });

        // Extract hospital and department codes from SORENHED_ANS
        let org_unit = if org_unit_array.is_null(i) {
            (None, None)
        } else {
            let value = org_unit_array.value(i);
            if let Some(pos) = value.find('-') {
                let hospital = &value[..pos];
                let department = &value[pos + 1..];
                (Some(hospital.to_string()), Some(department.to_string()))
            } else {
                (Some(value.to_string()), None)
            }
        };

        hospital_codes.push(org_unit.0);
        department_codes.push(org_unit.1);

        // Add admission type
        admission_types.push(if contact_type_array.is_null(i) {
            None
        } else {
            Some(contact_type_array.value(i).to_string())
        });
    }

    // Create batch using the common schema
    create_record_batch(
        patient_ids,
        primary_diagnoses,
        secondary_diagnoses,
        admission_dates,
        discharge_dates,
        hospital_codes,
        department_codes,
        admission_types,
        "LPR3",
    )
}

/// Combine harmonized LPR2 and LPR3 data
pub fn combine_harmonized_data(
    lpr2_data: Option<RecordBatch>,
    lpr3_data: Option<RecordBatch>,
) -> Result<RecordBatch> {
    match (lpr2_data, lpr3_data) {
        (Some(lpr2), Some(lpr3)) => {
            // Combine both datasets
            concat_batches(&lpr2.schema(), &[lpr2, lpr3])
                .map_err(|e| IdsError::Data(format!("Failed to combine LPR2 and LPR3 data: {e}")))
        }
        (Some(lpr2), None) => Ok(lpr2),
        (None, Some(lpr3)) => Ok(lpr3),
        (None, None) => Err(IdsError::Validation("No LPR data provided".to_string())),
    }
}

/// Filter harmonized health data by date range
pub fn filter_by_date_range(
    data: &RecordBatch,
    start_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
) -> Result<RecordBatch> {
    // If no date filters, return the original data
    if start_date.is_none() && end_date.is_none() {
        return Ok(data.clone());
    }

    // Get admission date column
    let date_idx = data.schema().index_of("admission_date")
        .map_err(|e| IdsError::Data(format!("admission_date column not found: {e}")))?;

    let date_array = data.column(date_idx);
    let date_array = date_array.as_any().downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("admission_date column is not a date array".to_string()))?;

    // Create filter mask
    let mut mask = Vec::with_capacity(data.num_rows());

    for i in 0..date_array.len() {
        if date_array.is_null(i) {
            mask.push(false);
            continue;
        }

        let date_i32 = date_array.value(i);
        let date = NaiveDate::from_num_days_from_ce_opt(date_i32 + 719163).unwrap_or_default();

        let passes_start = start_date.is_none_or(|start| date >= start);
        let passes_end = end_date.is_none_or(|end| date <= end);

        mask.push(passes_start && passes_end);
    }

    let mask_array = Arc::new(BooleanArray::from(mask));

    // Apply filter to all columns using the utility function
    let filtered_columns = date_utils::filter_arrays(data.columns(), &mask_array)?;

    // Create filtered batch
    RecordBatch::try_new(data.schema(), filtered_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))
}

/// Process LPR data for SCD algorithm
pub fn process_lpr_data(
    lpr2_adm: Option<&[RecordBatch]>,
    lpr2_diag: Option<&[RecordBatch]>,
    lpr2_bes: Option<&[RecordBatch]>,
    lpr3_kontakter: Option<&[RecordBatch]>,
    lpr3_diagnoser: Option<&[RecordBatch]>,
    config: &LprConfig,
) -> Result<RecordBatch> {
    let mut lpr2_data = None;
    let mut lpr3_data = None;

    // Process LPR2 data if available and included in config
    if config.include_lpr2 {
        if let (Some(adm), Some(diag)) = (lpr2_adm, lpr2_diag) {
            lpr2_data = Some(integrate_lpr2_components(adm, diag, lpr2_bes)?);
        }
    }

    // Process LPR3 data if available and included in config
    if config.include_lpr3 {
        if let (Some(kontakter), Some(diagnoser)) = (lpr3_kontakter, lpr3_diagnoser) {
            lpr3_data = Some(integrate_lpr3_components(kontakter, diagnoser)?);
        }
    }

    // Combine harmonized data
    let combined_data = combine_harmonized_data(lpr2_data, lpr3_data)?;

    // Apply date filtering if specified
    filter_by_date_range(&combined_data, config.start_date, config.end_date)
}
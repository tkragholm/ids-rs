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

use crate::algorithm::health::diagnosis::secondary::{
    create_secondary_diagnoses_array, create_secondary_diagnoses_field,
    process_secondary_diagnoses, SecondaryDiagnosis,
};
use crate::error::{IdsError, Result};
use crate::model::icd10::{diagnosis_pattern::normalize_diagnosis_code, Icd10Chapter};
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
        return Err(IdsError::Validation(format!(
            "No {data_name} data provided"
        )));
    } else if batches.len() == 1 {
        return Ok(batches[0].clone());
    }

    let schema = batches[0].schema();
    concat_batches(&schema, batches)
        .map_err(|e| IdsError::Data(format!("Failed to merge {data_name} batches: {e}")))
}

/// Extracts a string value from a record batch column
fn get_string_column(batch: &RecordBatch, column_name: &str) -> Result<Arc<StringArray>> {
    let col_idx = batch
        .schema()
        .index_of(column_name)
        .map_err(|e| IdsError::Data(format!("{column_name} column not found: {e}")))?;

    let col_array = batch.column(col_idx);
    col_array
        .as_any()
        .downcast_ref::<StringArray>()
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
fn map_treatment_dates(
    recnum_array: &StringArray,
    date_array: &Date32Array,
) -> HashMap<String, Vec<i32>> {
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

/// Get the ICD-10 chapter for a diagnosis code
fn get_diagnosis_chapter(diagnosis: &str) -> Option<String> {
    normalize_diagnosis_code(diagnosis)
        .and_then(|normalized| Icd10Chapter::from_code(&normalized.full_code))
        .map(|chapter| chapter.description().to_string())
}

/// Creates the integrated record batch schema (common for both LPR2 and LPR3)
fn create_integrated_schema() -> Schema {
    // Get secondary diagnosis field definition from the secondary diagnosis module
    let secondary_diag_list = create_secondary_diagnoses_field();

    // Add field for diagnosis chapter (based on ICD-10)
    Schema::new(vec![
        Field::new("patient_id", DataType::Utf8, true),
        Field::new("primary_diagnosis", DataType::Utf8, true),
        secondary_diag_list,
        Field::new("diagnosis_chapter", DataType::Utf8, true),
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
    let lpr_bes = lpr_bes
        .map(|batches| merge_batches(batches, "LPR_BES"))
        .transpose()?;

    // Get required columns from LPR_ADM
    let recnum_array = get_string_column(&lpr_adm, "RECNUM")?;
    let pnr_array = get_string_column(&lpr_adm, "PNR")?;
    let primary_diag_array = get_string_column(&lpr_adm, "C_ADIAG")?;
    let hospital_array = get_string_column(&lpr_adm, "C_SGH")?;
    let dept_array = get_string_column(&lpr_adm, "C_AFD")?;
    let pat_type_array = get_string_column(&lpr_adm, "C_PATTYPE")?;

    // Extract date columns
    let adm_date_idx = lpr_adm
        .schema()
        .index_of("D_INDDTO")
        .map_err(|e| IdsError::Data(format!("D_INDDTO column not found in LPR_ADM: {e}")))?;
    let adm_date_array = lpr_adm.column(adm_date_idx);

    // Use our flexible date conversion regardless of the column's actual type
    let adm_date_date32 = date_utils::convert_to_date32_array(adm_date_array.as_ref())?;

    let discharge_date_idx = lpr_adm
        .schema()
        .index_of("D_UDDTO")
        .map_err(|e| IdsError::Data(format!("D_UDDTO column not found in LPR_ADM: {e}")))?;
    let discharge_date_array = lpr_adm.column(discharge_date_idx);

    // Use our flexible date conversion regardless of the column's actual type
    let discharge_date_date32 = date_utils::convert_to_date32_array(discharge_date_array.as_ref())?;

    // Get diagnosis data from LPR_DIAG
    let diag_recnum_array = get_string_column(&lpr_diag, "RECNUM")?;
    let diag_array = get_string_column(&lpr_diag, "C_DIAG")?;
    let diag_type_array = get_string_column(&lpr_diag, "C_DIAGTYPE")?;

    // Get treatment data from LPR_BES if available
    let _bes_data = if let Some(lpr_bes) = &lpr_bes {
        let bes_recnum_array = get_string_column(lpr_bes, "RECNUM")?;
        let bes_date_idx = lpr_bes
            .schema()
            .index_of("D_AMBDTO")
            .map_err(|e| IdsError::Data(format!("D_AMBDTO column not found in LPR_BES: {e}")))?;
        let bes_date_array = lpr_bes.column(bes_date_idx);

        // Convert to Date32Array
        let bes_date_date32 = date_utils::convert_to_date32_array(bes_date_array.as_ref())?;

        // Map treatment dates by record number
        Some(map_treatment_dates(&bes_recnum_array, &bes_date_date32))
    } else {
        None
    };

    // Map diagnoses by record number
    let diagnoses_by_recnum =
        map_diagnoses_by_recnum(&diag_recnum_array, &diag_array, &diag_type_array);

    // Create a row index by record number for ADM data
    let _recnum_to_row = build_recnum_index(&recnum_array);

    // Build integrated records
    let num_rows = lpr_adm.num_rows();
    log::info!("Building integrated LPR2 records for {num_rows} rows");

    // Process in chunks to reduce memory usage
    const CHUNK_SIZE: usize = 500000; // Process 500k rows at a time
    let num_chunks = num_rows.div_ceil(CHUNK_SIZE); // Ceiling division
    
    let mut all_batches = Vec::with_capacity(num_chunks);
    let integrated_schema = create_integrated_schema();
    
    for chunk_idx in 0..num_chunks {
        let start_idx = chunk_idx * CHUNK_SIZE;
        let end_idx = std::cmp::min((chunk_idx + 1) * CHUNK_SIZE, num_rows);
        let chunk_size = end_idx - start_idx;
        
        log::info!("Processing chunk {}/{} (rows {}-{})", 
                  chunk_idx + 1, num_chunks, start_idx, end_idx - 1);
        
        // Result arrays for this chunk
        let mut patient_ids = Vec::with_capacity(chunk_size);
        let mut primary_diagnoses = Vec::with_capacity(chunk_size);
        let mut secondary_diagnoses_list: Vec<Option<Vec<SecondaryDiagnosis>>> =
            Vec::with_capacity(chunk_size);
        let mut diagnosis_chapters = Vec::with_capacity(chunk_size);
        let mut admission_dates = Vec::with_capacity(chunk_size);
        let mut discharge_dates = Vec::with_capacity(chunk_size);
        let mut hospital_codes = Vec::with_capacity(chunk_size);
        let mut department_codes = Vec::with_capacity(chunk_size);
        let mut admission_types = Vec::with_capacity(chunk_size);

        for i in start_idx..end_idx {
            // Get record number for this row
            let recnum = if recnum_array.is_null(i) {
                // Skip records without a record number
                continue;
            } else {
                recnum_array.value(i).to_string()
            };

            // Add patient ID (PNR)
            patient_ids.push(if pnr_array.is_null(i) {
                None
            } else {
                Some(pnr_array.value(i).to_string())
            });

            // Add primary diagnosis (normalize if non-null)
            primary_diagnoses.push(if primary_diag_array.is_null(i) {
                None
            } else {
                let primary_diag = primary_diag_array.value(i);
                normalize_diagnosis_code(primary_diag)
                    .map(|norm| norm.full_code)
                    .or(Some(primary_diag.to_string()))
            });

            // Add diagnosis chapter (based on primary diagnosis)
            diagnosis_chapters.push(if primary_diag_array.is_null(i) {
                None
            } else {
                let primary_diag = primary_diag_array.value(i);
                get_diagnosis_chapter(primary_diag)
            });

            // Add secondary diagnoses from LPR_DIAG
            if let Some(diagnoses) = diagnoses_by_recnum.get(&recnum) {
                // Process and convert secondary diagnoses
                let sec_diagnoses = process_secondary_diagnoses(diagnoses);
                if sec_diagnoses.is_empty() {
                    secondary_diagnoses_list.push(None);
                } else {
                    secondary_diagnoses_list.push(Some(sec_diagnoses));
                }
            } else {
                secondary_diagnoses_list.push(None);
            }

            // Add date columns
            admission_dates.push(if adm_date_date32.is_null(i) {
                None
            } else {
                Some(adm_date_date32.value(i))
            });

            discharge_dates.push(if discharge_date_date32.is_null(i) {
                None
            } else {
                Some(discharge_date_date32.value(i))
            });

            // Add hospital code
            hospital_codes.push(if hospital_array.is_null(i) {
                None
            } else {
                Some(hospital_array.value(i).to_string())
            });

            // Add department code
            department_codes.push(if dept_array.is_null(i) {
                None
            } else {
                Some(dept_array.value(i).to_string())
            });

            // Add patient type (admission type)
            admission_types.push(if pat_type_array.is_null(i) {
                None
            } else {
                Some(pat_type_array.value(i).to_string())
            });
        }
        
        // Create Arrow arrays for this chunk
        let patient_id_array = StringArray::from(patient_ids);
        let primary_diag_array = StringArray::from(primary_diagnoses);
        let sec_diag_array = create_secondary_diagnoses_array(&secondary_diagnoses_list);
        let diag_chapter_array = StringArray::from(diagnosis_chapters);
        let adm_date_array = Date32Array::from(admission_dates);
        let disch_date_array = Date32Array::from(discharge_dates);
        let hospital_array = StringArray::from(hospital_codes);
        let dept_array = StringArray::from(department_codes);
        let adm_type_array = StringArray::from(admission_types);
        
        // Create batch for this chunk
        let chunk_batch = RecordBatch::try_new(
            Arc::new(integrated_schema.clone()),
            vec![
                Arc::new(patient_id_array),
                Arc::new(primary_diag_array),
                sec_diag_array,
                Arc::new(diag_chapter_array),
                Arc::new(adm_date_array),
                Arc::new(disch_date_array),
                Arc::new(hospital_array),
                Arc::new(dept_array),
                Arc::new(adm_type_array),
            ],
        )
        .map_err(|e| IdsError::Data(format!("Failed to create integrated LPR2 batch for chunk {}: {e}", chunk_idx + 1)))?;
        
        all_batches.push(chunk_batch);
        
        // Force memory cleanup after each chunk
        std::mem::drop(secondary_diagnoses_list);
    }

    // Combine all chunks into a single batch
    if all_batches.is_empty() {
        return Err(IdsError::Data("No valid chunks were created".to_string()));
    } else if all_batches.len() == 1 {
        log::info!("Only one chunk was created, returning it directly");
        return Ok(all_batches.remove(0));
    }
    
    log::info!("Combining {} chunks into a single batch", all_batches.len());
    let schema_arc = Arc::new(integrated_schema);
    let integrated_batch = arrow::compute::concat_batches(&schema_arc, &all_batches)
        .map_err(|e| IdsError::Data(format!("Failed to concatenate LPR2 chunks: {e}")))?;
    
    Ok(integrated_batch)
}

/// Integrate LPR3 components (`LPR3_KONTAKTER` and `LPR3_DIAGNOSER`)
pub fn integrate_lpr3_components(
    lpr3_kontakter: &[RecordBatch],
    lpr3_diagnoser: &[RecordBatch],
) -> Result<RecordBatch> {
    // First merge all batches
    let lpr3_kontakter = merge_batches(lpr3_kontakter, "LPR3_KONTAKTER")?;
    let lpr3_diagnoser = merge_batches(lpr3_diagnoser, "LPR3_DIAGNOSER")?;

    // Get required columns from LPR3_KONTAKTER
    let kontakt_id_array = get_string_column(&lpr3_kontakter, "kontakt_id")?;
    let pnr_array = get_string_column(&lpr3_kontakter, "cpr")?;
    let start_date_idx = lpr3_kontakter
        .schema()
        .index_of("starttidspunkt")
        .map_err(|e| {
            IdsError::Data(format!(
                "starttidspunkt column not found in LPR3_KONTAKTER: {e}"
            ))
        })?;
    let start_date_array = lpr3_kontakter.column(start_date_idx);

    // Convert to Date32Array
    let start_date_date32 = date_utils::convert_to_date32_array(start_date_array.as_ref())?;

    let end_date_idx = lpr3_kontakter
        .schema()
        .index_of("sluttidspunkt")
        .map_err(|e| {
            IdsError::Data(format!(
                "sluttidspunkt column not found in LPR3_KONTAKTER: {e}"
            ))
        })?;
    let end_date_array = lpr3_kontakter.column(end_date_idx);

    // Convert to Date32Array
    let end_date_date32 = date_utils::convert_to_date32_array(end_date_array.as_ref())?;

    let sygehus_array = get_string_column(&lpr3_kontakter, "sygehus")?;
    let afdeling_array = get_string_column(&lpr3_kontakter, "afdeling")?;
    let kontakttype_array = get_string_column(&lpr3_kontakter, "kontakttype")?;

    // Get diagnosis data from LPR3_DIAGNOSER
    let diag_kontakt_id_array = get_string_column(&lpr3_diagnoser, "kontakt_id")?;
    let diag_array = get_string_column(&lpr3_diagnoser, "diagnosekode")?;
    let diag_type_array = get_string_column(&lpr3_diagnoser, "diagnose_type")?;

    // Map diagnoses by contact ID (equivalent to record number in LPR2)
    let diagnoses_by_kontakt_id =
        map_diagnoses_by_recnum(&diag_kontakt_id_array, &diag_array, &diag_type_array);

    // Create a row index by contact ID for KONTAKTER data
    let _kontakt_id_to_row = build_recnum_index(&kontakt_id_array);

    // Build integrated records
    let num_rows = lpr3_kontakter.num_rows();
    log::info!("Building integrated LPR3 records for {num_rows} rows");

    // Process in chunks to reduce memory usage
    const CHUNK_SIZE: usize = 500000; // Process 500k rows at a time
    let num_chunks = num_rows.div_ceil(CHUNK_SIZE); // Ceiling division
    
    let mut all_batches = Vec::with_capacity(num_chunks);
    let integrated_schema = create_integrated_schema();
    
    for chunk_idx in 0..num_chunks {
        let start_idx = chunk_idx * CHUNK_SIZE;
        let end_idx = std::cmp::min((chunk_idx + 1) * CHUNK_SIZE, num_rows);
        let chunk_size = end_idx - start_idx;
        
        log::info!("Processing LPR3 chunk {}/{} (rows {}-{})", 
                  chunk_idx + 1, num_chunks, start_idx, end_idx - 1);
        
        // Result arrays for this chunk
        let mut patient_ids = Vec::with_capacity(chunk_size);
        let mut primary_diagnoses = Vec::with_capacity(chunk_size);
        let mut secondary_diagnoses_list: Vec<Option<Vec<SecondaryDiagnosis>>> =
            Vec::with_capacity(chunk_size);
        let mut diagnosis_chapters = Vec::with_capacity(chunk_size);
        let mut admission_dates = Vec::with_capacity(chunk_size);
        let mut discharge_dates = Vec::with_capacity(chunk_size);
        let mut hospital_codes = Vec::with_capacity(chunk_size);
        let mut department_codes = Vec::with_capacity(chunk_size);
        let mut admission_types = Vec::with_capacity(chunk_size);

        for i in start_idx..end_idx {
            // Get contact ID for this row
            let kontakt_id = if kontakt_id_array.is_null(i) {
                // Skip records without a contact ID
                continue;
            } else {
                kontakt_id_array.value(i).to_string()
            };

            // Add patient ID (PNR/CPR)
            patient_ids.push(if pnr_array.is_null(i) {
                None
            } else {
                Some(pnr_array.value(i).to_string())
            });

            // Get diagnoses for this contact
            if let Some(diagnoses) = diagnoses_by_kontakt_id.get(&kontakt_id) {
                // Find primary diagnosis (type A)
                let primary_diag = diagnoses.iter().find(|(_, diag_type)| diag_type == "A");

                if let Some((diag, _)) = primary_diag {
                    // Normalize and add primary diagnosis
                    primary_diagnoses.push(
                        normalize_diagnosis_code(diag)
                            .map(|norm| norm.full_code)
                            .or(Some(diag.clone())),
                    );

                    // Add diagnosis chapter based on primary diagnosis
                    diagnosis_chapters.push(get_diagnosis_chapter(diag));
                } else {
                    // No primary diagnosis found
                    primary_diagnoses.push(None);
                    diagnosis_chapters.push(None);
                }

                // Process and convert secondary diagnoses
                let sec_diagnoses = process_secondary_diagnoses(diagnoses);
                if sec_diagnoses.is_empty() {
                    secondary_diagnoses_list.push(None);
                } else {
                    secondary_diagnoses_list.push(Some(sec_diagnoses));
                }
            } else {
                // No diagnoses for this contact
                primary_diagnoses.push(None);
                diagnosis_chapters.push(None);
                secondary_diagnoses_list.push(None);
            }

            // Add date columns
            admission_dates.push(if start_date_date32.is_null(i) {
                None
            } else {
                Some(start_date_date32.value(i))
            });

            discharge_dates.push(if end_date_date32.is_null(i) {
                None
            } else {
                Some(end_date_date32.value(i))
            });

            // Add hospital code
            hospital_codes.push(if sygehus_array.is_null(i) {
                None
            } else {
                Some(sygehus_array.value(i).to_string())
            });

            // Add department code
            department_codes.push(if afdeling_array.is_null(i) {
                None
            } else {
                Some(afdeling_array.value(i).to_string())
            });

            // Add contact type (admission type)
            admission_types.push(if kontakttype_array.is_null(i) {
                None
            } else {
                Some(kontakttype_array.value(i).to_string())
            });
        }

        // Create Arrow arrays for this chunk
        let patient_id_array = StringArray::from(patient_ids);
        let primary_diag_array = StringArray::from(primary_diagnoses);
        let sec_diag_array = create_secondary_diagnoses_array(&secondary_diagnoses_list);
        let diag_chapter_array = StringArray::from(diagnosis_chapters);
        let adm_date_array = Date32Array::from(admission_dates);
        let disch_date_array = Date32Array::from(discharge_dates);
        let hospital_array = StringArray::from(hospital_codes);
        let dept_array = StringArray::from(department_codes);
        let adm_type_array = StringArray::from(admission_types);
        
        // Create batch for this chunk
        let chunk_batch = RecordBatch::try_new(
            Arc::new(integrated_schema.clone()),
            vec![
                Arc::new(patient_id_array),
                Arc::new(primary_diag_array),
                sec_diag_array,
                Arc::new(diag_chapter_array),
                Arc::new(adm_date_array),
                Arc::new(disch_date_array),
                Arc::new(hospital_array),
                Arc::new(dept_array),
                Arc::new(adm_type_array),
            ],
        )
        .map_err(|e| IdsError::Data(format!("Failed to create integrated LPR3 batch for chunk {}: {e}", chunk_idx + 1)))?;
        
        all_batches.push(chunk_batch);
        
        // Force memory cleanup after each chunk
        std::mem::drop(secondary_diagnoses_list);
    }
    
    // Combine all chunks into a single batch
    if all_batches.is_empty() {
        return Err(IdsError::Data("No valid LPR3 chunks were created".to_string()));
    } else if all_batches.len() == 1 {
        log::info!("Only one LPR3 chunk was created, returning it directly");
        return Ok(all_batches.remove(0));
    }
    
    log::info!("Combining {} LPR3 chunks into a single batch", all_batches.len());
    let schema_arc = Arc::new(integrated_schema);
    let integrated_batch = arrow::compute::concat_batches(&schema_arc, &all_batches)
        .map_err(|e| IdsError::Data(format!("Failed to concatenate LPR3 chunks: {e}")))?;
    
    Ok(integrated_batch)
}

/// Combine harmonized LPR2 and LPR3 data
pub fn combine_harmonized_data(
    lpr2_data: Option<RecordBatch>,
    lpr3_data: Option<RecordBatch>,
) -> Result<RecordBatch> {
    match (lpr2_data, lpr3_data) {
        (Some(lpr2), Some(lpr3)) => {
            // Both LPR2 and LPR3 data available, combine them
            let schema = lpr2.schema();
            let batches = vec![lpr2, lpr3];
            concat_batches(&schema, &batches)
                .map_err(|e| IdsError::Data(format!("Failed to combine LPR2 and LPR3 data: {e}")))
        }
        (Some(lpr2), None) => {
            // Only LPR2 data available
            Ok(lpr2)
        }
        (None, Some(lpr3)) => {
            // Only LPR3 data available
            Ok(lpr3)
        }
        (None, None) => {
            // No data available
            Err(IdsError::Validation("No LPR data provided".to_string()))
        }
    }
}

/// Apply date filtering to health data
fn apply_date_filtering(batch: &RecordBatch, config: &LprConfig) -> Result<RecordBatch> {
    if config.start_date.is_none() && config.end_date.is_none() {
        // No date filtering needed
        return Ok(batch.clone());
    }

    // Get admission date column
    let adm_date_idx = batch
        .schema()
        .index_of("admission_date")
        .map_err(|e| IdsError::Data(format!("admission_date column not found: {e}")))?;
    let adm_date_array = batch.column(adm_date_idx);

    // Create mask for date filtering
    let mut mask = vec![true; batch.num_rows()];

    if let Some(start_date) = config.start_date {
        let start_days = start_date
            .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .num_days() as i32;

        if let Some(date_array) = adm_date_array.as_any().downcast_ref::<Date32Array>() {
            for i in 0..batch.num_rows() {
                if date_array.is_null(i) || date_array.value(i) < start_days {
                    mask[i] = false;
                }
            }
        }
    }

    if let Some(end_date) = config.end_date {
        let end_days = end_date
            .signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .num_days() as i32;

        if let Some(date_array) = adm_date_array.as_any().downcast_ref::<Date32Array>() {
            for i in 0..batch.num_rows() {
                if date_array.is_null(i) || date_array.value(i) > end_days {
                    mask[i] = false;
                }
            }
        }
    }

    // Convert mask to BooleanArray
    let mask_array = BooleanArray::from(mask);

    // Apply filter to each column
    let mut filtered_columns = Vec::with_capacity(batch.num_columns());
    for i in 0..batch.num_columns() {
        let column = batch.column(i);
        let filtered_column = arrow::compute::filter(column, &mask_array)
            .map_err(|e| IdsError::Data(format!("Failed to filter column {i}: {e}")))?;
        filtered_columns.push(filtered_column);
    }

    // Create filtered batch
    let filtered_batch = RecordBatch::try_new(batch.schema(), filtered_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))?;

    Ok(filtered_batch)
}

/// Process LPR data from LPR2 and/or LPR3 sources
pub fn process_lpr_data(
    lpr2_adm: Option<&[RecordBatch]>,
    lpr2_diag: Option<&[RecordBatch]>,
    lpr2_bes: Option<&[RecordBatch]>,
    lpr3_kontakter: Option<&[RecordBatch]>,
    lpr3_diagnoser: Option<&[RecordBatch]>,
    config: &LprConfig,
) -> Result<RecordBatch> {
    // Process LPR2 data if enabled and available
    let lpr2_data = if config.include_lpr2 && lpr2_adm.is_some() && lpr2_diag.is_some() {
        Some(integrate_lpr2_components(
            lpr2_adm.unwrap(),
            lpr2_diag.unwrap(),
            lpr2_bes,
        )?)
    } else {
        None
    };

    // Process LPR3 data if enabled and available
    let lpr3_data = if config.include_lpr3 && lpr3_kontakter.is_some() && lpr3_diagnoser.is_some() {
        Some(integrate_lpr3_components(
            lpr3_kontakter.unwrap(),
            lpr3_diagnoser.unwrap(),
        )?)
    } else {
        None
    };

    // Combine harmonized data
    let combined_data = combine_harmonized_data(lpr2_data, lpr3_data)?;

    // Apply date filtering if needed
    let filtered_data = apply_date_filtering(&combined_data, config)?;

    Ok(filtered_data)
}

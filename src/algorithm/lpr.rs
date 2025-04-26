//! LPR (Landspatientregistret) data processing algorithms
//!
//! This module implements data processing operations for the Danish National Patient Registry (LPR)
//! including integration of LPR2 and LPR3 data, data harmonization, and preparation for SCD analysis.

use arrow::array::{Array, BooleanArray, Date32Array, StringArray};
use arrow::compute::{concat_batches, filter};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use std::collections::HashMap;
use std::sync::Arc;

use crate::error::{IdsError, Result};

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

/// Integrate LPR2 components (`LPR_ADM`, `LPR_DIAG`, and optionally `LPR_BES`)
pub fn integrate_lpr2_components(
    lpr_adm: &[RecordBatch],
    lpr_diag: &[RecordBatch],
    lpr_bes: Option<&[RecordBatch]>,
) -> Result<RecordBatch> {
    // First merge all LPR_ADM batches
    let lpr_adm = if lpr_adm.len() > 1 {
        let schema = lpr_adm[0].schema();
        concat_batches(&schema, lpr_adm)
            .map_err(|e| IdsError::Data(format!("Failed to merge LPR_ADM batches: {e}")))?
    } else if !lpr_adm.is_empty() {
        lpr_adm[0].clone()
    } else {
        return Err(IdsError::Validation("No LPR_ADM data provided".to_string()));
    };

    // Merge all LPR_DIAG batches
    let lpr_diag = if lpr_diag.len() > 1 {
        let schema = lpr_diag[0].schema();
        concat_batches(&schema, lpr_diag)
            .map_err(|e| IdsError::Data(format!("Failed to merge LPR_DIAG batches: {e}")))?
    } else if !lpr_diag.is_empty() {
        lpr_diag[0].clone()
    } else {
        return Err(IdsError::Validation(
            "No LPR_DIAG data provided".to_string(),
        ));
    };

    // Optionally merge LPR_BES batches
    let lpr_bes = if let Some(bes_batches) = lpr_bes {
        if bes_batches.is_empty() {
            None
        } else if bes_batches.len() > 1 {
            let schema = bes_batches[0].schema();
            Some(
                concat_batches(&schema, bes_batches)
                    .map_err(|e| IdsError::Data(format!("Failed to merge LPR_BES batches: {e}")))?,
            )
        } else {
            Some(bes_batches[0].clone())
        }
    } else {
        None
    };

    // Now join data based on RECNUM

    // First, create a map from RECNUM to row index in LPR_ADM
    let recnum_idx = lpr_adm
        .schema()
        .index_of("RECNUM")
        .map_err(|e| IdsError::Data(format!("RECNUM column not found in LPR_ADM: {e}")))?;

    let recnum_array = lpr_adm.column(recnum_idx);
    let recnum_array = recnum_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("RECNUM column is not a string array".to_string()))?;

    let mut recnum_to_row = HashMap::new();
    for i in 0..recnum_array.len() {
        if recnum_array.is_null(i) {
            continue;
        }
        let recnum = recnum_array.value(i);
        recnum_to_row.insert(recnum.to_string(), i);
    }

    // Next, create a map from RECNUM to diagnoses in LPR_DIAG
    let recnum_idx_diag = lpr_diag
        .schema()
        .index_of("RECNUM")
        .map_err(|e| IdsError::Data(format!("RECNUM column not found in LPR_DIAG: {e}")))?;

    let recnum_array_diag = lpr_diag.column(recnum_idx_diag);
    let recnum_array_diag = recnum_array_diag
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("RECNUM column is not a string array".to_string()))?;

    let diag_idx = lpr_diag
        .schema()
        .index_of("C_DIAG")
        .map_err(|e| IdsError::Data(format!("C_DIAG column not found in LPR_DIAG: {e}")))?;

    let diag_array = lpr_diag.column(diag_idx);
    let diag_array = diag_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("C_DIAG column is not a string array".to_string()))?;

    let diag_type_idx = lpr_diag
        .schema()
        .index_of("C_DIAGTYPE")
        .map_err(|e| IdsError::Data(format!("C_DIAGTYPE column not found in LPR_DIAG: {e}")))?;

    let diag_type_array = lpr_diag.column(diag_type_idx);
    let diag_type_array = diag_type_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("C_DIAGTYPE column is not a string array".to_string()))?;

    let mut recnum_to_diagnoses = HashMap::new();
    for i in 0..recnum_array_diag.len() {
        if recnum_array_diag.is_null(i) || diag_array.is_null(i) {
            continue;
        }
        let recnum = recnum_array_diag.value(i);
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

    // Process LPR_BES data if available to get treatment dates
    let mut recnum_to_treatment_dates = HashMap::new();
    if let Some(lpr_bes) = lpr_bes.as_ref() {
        // Get RECNUM from LPR_BES
        let recnum_idx_bes = lpr_bes
            .schema()
            .index_of("RECNUM")
            .map_err(|e| IdsError::Data(format!("RECNUM column not found in LPR_BES: {e}")))?;

        let recnum_array_bes = lpr_bes.column(recnum_idx_bes);
        let recnum_array_bes = recnum_array_bes
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data("RECNUM column is not a string array".to_string()))?;

        // Get treatment date (D_AMBDTO) from LPR_BES
        let date_idx_bes = lpr_bes
            .schema()
            .index_of("D_AMBDTO")
            .map_err(|e| IdsError::Data(format!("D_AMBDTO column not found in LPR_BES: {e}")))?;

        let date_array_bes = lpr_bes.column(date_idx_bes);
        let date_array_bes = date_array_bes
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::Data("D_AMBDTO column is not a date array".to_string()))?;

        // Map RECNUM to treatment dates
        for i in 0..recnum_array_bes.len() {
            if recnum_array_bes.is_null(i) || date_array_bes.is_null(i) {
                continue;
            }
            let recnum = recnum_array_bes.value(i);
            let date_i32 = date_array_bes.value(i);

            let dates = recnum_to_treatment_dates
                .entry(recnum.to_string())
                .or_insert_with(Vec::new);

            dates.push(date_i32);
        }
    }

    // Create integrated data

    // Count resulting records first
    let num_rows = lpr_adm.num_rows();

    // We'll create arrays for each column in the integrated data
    let mut patient_ids = Vec::with_capacity(num_rows);
    let mut primary_diagnoses = Vec::with_capacity(num_rows);
    let mut secondary_diagnoses = Vec::with_capacity(num_rows);
    let mut admission_dates = Vec::with_capacity(num_rows);
    let mut discharge_dates = Vec::with_capacity(num_rows);
    let mut hospital_codes = Vec::with_capacity(num_rows);
    let mut department_codes = Vec::with_capacity(num_rows);
    let mut admission_types = Vec::with_capacity(num_rows);

    // Extract PNR column from LPR_ADM
    let pnr_idx = lpr_adm
        .schema()
        .index_of("PNR")
        .map_err(|e| IdsError::Data(format!("PNR column not found in LPR_ADM: {e}")))?;

    let pnr_array = lpr_adm.column(pnr_idx);
    let pnr_array = pnr_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;

    // Extract other needed columns from LPR_ADM
    let primary_diag_idx = lpr_adm
        .schema()
        .index_of("C_ADIAG")
        .map_err(|e| IdsError::Data(format!("C_ADIAG column not found in LPR_ADM: {e}")))?;

    let primary_diag_array = lpr_adm.column(primary_diag_idx);
    let primary_diag_array = primary_diag_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("C_ADIAG column is not a string array".to_string()))?;

    let adm_date_idx = lpr_adm
        .schema()
        .index_of("D_INDDTO")
        .map_err(|e| IdsError::Data(format!("D_INDDTO column not found in LPR_ADM: {e}")))?;

    let adm_date_array = lpr_adm.column(adm_date_idx);
    let adm_date_array = adm_date_array
        .as_any()
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("D_INDDTO column is not a date array".to_string()))?;

    let disc_date_idx = lpr_adm
        .schema()
        .index_of("D_UDDTO")
        .map_err(|e| IdsError::Data(format!("D_UDDTO column not found in LPR_ADM: {e}")))?;

    let disc_date_array = lpr_adm.column(disc_date_idx);
    let disc_date_array = disc_date_array
        .as_any()
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("D_UDDTO column is not a date array".to_string()))?;

    // Department and hospital codes
    let hospital_idx = lpr_adm
        .schema()
        .index_of("C_SGH")
        .map_err(|e| IdsError::Data(format!("C_SGH column not found in LPR_ADM: {e}")))?;

    let hospital_array = lpr_adm.column(hospital_idx);
    let hospital_array = hospital_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("C_SGH column is not a string array".to_string()))?;

    let dept_idx = lpr_adm
        .schema()
        .index_of("C_AFD")
        .map_err(|e| IdsError::Data(format!("C_AFD column not found in LPR_ADM: {e}")))?;

    let dept_array = lpr_adm.column(dept_idx);
    let dept_array = dept_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("C_AFD column is not a string array".to_string()))?;

    // Patient type (inpatient/outpatient)
    let pat_type_idx = lpr_adm
        .schema()
        .index_of("C_PATTYPE")
        .map_err(|e| IdsError::Data(format!("C_PATTYPE column not found in LPR_ADM: {e}")))?;

    let pat_type_array = lpr_adm.column(pat_type_idx);
    let pat_type_array = pat_type_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("C_PATTYPE column is not a string array".to_string()))?;

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
        let secondary_diag = if let Some(diagnoses) = recnum_to_diagnoses.get(recnum) {
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
        } else {
            None
        };
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

    // Create schema for integrated data
    let schema = Schema::new(vec![
        Field::new("patient_id", DataType::Utf8, true),
        Field::new("primary_diagnosis", DataType::Utf8, true),
        Field::new("secondary_diagnosis", DataType::Utf8, true),
        Field::new("admission_date", DataType::Date32, true),
        Field::new("discharge_date", DataType::Date32, true),
        Field::new("hospital_code", DataType::Utf8, true),
        Field::new("department_code", DataType::Utf8, true),
        Field::new("admission_type", DataType::Utf8, true),
    ]);

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
    let integrated_batch = RecordBatch::try_new(
        Arc::new(schema),
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
    .map_err(|e| IdsError::Data(format!("Failed to create integrated LPR2 batch: {e}")))?;

    Ok(integrated_batch)
}

/// Integrate LPR3 components (`LPR3_KONTAKTER` and `LPR3_DIAGNOSER`)
pub fn integrate_lpr3_components(
    lpr3_kontakter: &[RecordBatch],
    lpr3_diagnoser: &[RecordBatch],
) -> Result<RecordBatch> {
    // First merge all LPR3_KONTAKTER batches
    let lpr3_kontakter = if lpr3_kontakter.len() > 1 {
        let schema = lpr3_kontakter[0].schema();
        concat_batches(&schema, lpr3_kontakter)
            .map_err(|e| IdsError::Data(format!("Failed to merge LPR3_KONTAKTER batches: {e}")))?
    } else if !lpr3_kontakter.is_empty() {
        lpr3_kontakter[0].clone()
    } else {
        return Err(IdsError::Validation(
            "No LPR3_KONTAKTER data provided".to_string(),
        ));
    };

    // Merge all LPR3_DIAGNOSER batches
    let lpr3_diagnoser = if lpr3_diagnoser.len() > 1 {
        let schema = lpr3_diagnoser[0].schema();
        concat_batches(&schema, lpr3_diagnoser)
            .map_err(|e| IdsError::Data(format!("Failed to merge LPR3_DIAGNOSER batches: {e}")))?
    } else if !lpr3_diagnoser.is_empty() {
        lpr3_diagnoser[0].clone()
    } else {
        return Err(IdsError::Validation(
            "No LPR3_DIAGNOSER data provided".to_string(),
        ));
    };

    // Create a map from DW_EK_KONTAKT to diagnoses
    let kontakt_idx = lpr3_diagnoser
        .schema()
        .index_of("DW_EK_KONTAKT")
        .map_err(|e| {
            IdsError::Data(format!(
                "DW_EK_KONTAKT column not found in LPR3_DIAGNOSER: {e}"
            ))
        })?;

    let kontakt_array = lpr3_diagnoser.column(kontakt_idx);
    let kontakt_array = kontakt_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("DW_EK_KONTAKT column is not a string array".to_string()))?;

    let diag_idx = lpr3_diagnoser
        .schema()
        .index_of("diagnosekode")
        .map_err(|e| {
            IdsError::Data(format!(
                "diagnosekode column not found in LPR3_DIAGNOSER: {e}"
            ))
        })?;

    let diag_array = lpr3_diagnoser.column(diag_idx);
    let diag_array = diag_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("diagnosekode column is not a string array".to_string()))?;

    let diag_type_idx = lpr3_diagnoser
        .schema()
        .index_of("diagnosetype")
        .map_err(|e| {
            IdsError::Data(format!(
                "diagnosetype column not found in LPR3_DIAGNOSER: {e}"
            ))
        })?;

    let diag_type_array = lpr3_diagnoser.column(diag_type_idx);
    let diag_type_array = diag_type_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("diagnosetype column is not a string array".to_string()))?;

    let mut kontakt_to_diagnoses = HashMap::new();
    for i in 0..kontakt_array.len() {
        if kontakt_array.is_null(i) || diag_array.is_null(i) {
            continue;
        }
        let kontakt_id = kontakt_array.value(i);
        let diagnosis = diag_array.value(i);
        let diag_type = if diag_type_array.is_null(i) {
            "A" // Default to 'A' (action diagnosis) if not specified
        } else {
            diag_type_array.value(i)
        };

        let diagnoses = kontakt_to_diagnoses
            .entry(kontakt_id.to_string())
            .or_insert_with(Vec::new);

        diagnoses.push((diagnosis.to_string(), diag_type.to_string()));
    }

    // Now process LPR3_KONTAKTER and join with diagnoses

    // Get column indices for LPR3_KONTAKTER
    let kontakt_id_idx = lpr3_kontakter
        .schema()
        .index_of("DW_EK_KONTAKT")
        .map_err(|e| {
            IdsError::Data(format!(
                "DW_EK_KONTAKT column not found in LPR3_KONTAKTER: {e}"
            ))
        })?;

    let kontakt_id_array = lpr3_kontakter.column(kontakt_id_idx);
    let kontakt_id_array = kontakt_id_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("DW_EK_KONTAKT column is not a string array".to_string()))?;

    let patient_id_idx = lpr3_kontakter
        .schema()
        .index_of("CPR")
        .map_err(|e| IdsError::Data(format!("CPR column not found in LPR3_KONTAKTER: {e}")))?;

    let patient_id_array = lpr3_kontakter.column(patient_id_idx);
    let patient_id_array = patient_id_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("CPR column is not a string array".to_string()))?;

    let action_diag_idx = lpr3_kontakter
        .schema()
        .index_of("aktionsdiagnose")
        .map_err(|e| {
            IdsError::Data(format!(
                "aktionsdiagnose column not found in LPR3_KONTAKTER: {e}"
            ))
        })?;

    let action_diag_array = lpr3_kontakter.column(action_diag_idx);
    let action_diag_array = action_diag_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| {
            IdsError::Data("aktionsdiagnose column is not a string array".to_string())
        })?;

    let start_date_idx = lpr3_kontakter
        .schema()
        .index_of("dato_start")
        .map_err(|e| {
            IdsError::Data(format!(
                "dato_start column not found in LPR3_KONTAKTER: {e}"
            ))
        })?;

    let start_date_array = lpr3_kontakter.column(start_date_idx);
    let start_date_array = start_date_array
        .as_any()
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("dato_start column is not a date array".to_string()))?;

    let end_date_idx = lpr3_kontakter.schema().index_of("dato_slut").map_err(|e| {
        IdsError::Data(format!("dato_slut column not found in LPR3_KONTAKTER: {e}"))
    })?;

    let end_date_array = lpr3_kontakter.column(end_date_idx);
    let end_date_array = end_date_array
        .as_any()
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("dato_slut column is not a date array".to_string()))?;

    // Get organizational unit
    let org_unit_idx = lpr3_kontakter
        .schema()
        .index_of("SORENHED_ANS")
        .map_err(|e| {
            IdsError::Data(format!(
                "SORENHED_ANS column not found in LPR3_KONTAKTER: {e}"
            ))
        })?;

    let org_unit_array = lpr3_kontakter.column(org_unit_idx);
    let org_unit_array = org_unit_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("SORENHED_ANS column is not a string array".to_string()))?;

    // Get contact type
    let contact_type_idx = lpr3_kontakter
        .schema()
        .index_of("kontakttype")
        .map_err(|e| {
            IdsError::Data(format!(
                "kontakttype column not found in LPR3_KONTAKTER: {e}"
            ))
        })?;

    let contact_type_array = lpr3_kontakter.column(contact_type_idx);
    let contact_type_array = contact_type_array
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("kontakttype column is not a string array".to_string()))?;

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
            continue; // Skip rows without kontakt_id
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
        let secondary_diag = if let Some(diagnoses) = kontakt_to_diagnoses.get(kontakt_id) {
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
        } else {
            None
        };
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
        // In LPR3, this is stored as a single string like "1234-5678"
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

    // Create schema for integrated data
    let schema = Schema::new(vec![
        Field::new("patient_id", DataType::Utf8, true),
        Field::new("primary_diagnosis", DataType::Utf8, true),
        Field::new("secondary_diagnosis", DataType::Utf8, true),
        Field::new("admission_date", DataType::Date32, true),
        Field::new("discharge_date", DataType::Date32, true),
        Field::new("hospital_code", DataType::Utf8, true),
        Field::new("department_code", DataType::Utf8, true),
        Field::new("admission_type", DataType::Utf8, true),
    ]);

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
    let integrated_batch = RecordBatch::try_new(
        Arc::new(schema),
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
    .map_err(|e| IdsError::Data(format!("Failed to create integrated LPR3 batch: {e}")))?;

    Ok(integrated_batch)
}

/// Combine harmonized LPR2 and LPR3 data
pub fn combine_harmonized_data(
    lpr2_data: Option<RecordBatch>,
    lpr3_data: Option<RecordBatch>,
) -> Result<RecordBatch> {
    match (lpr2_data, lpr3_data) {
        (Some(lpr2), Some(lpr3)) => {
            // Combine both datasets
            let combined = concat_batches(&lpr2.schema(), &[lpr2, lpr3]).map_err(|e| {
                IdsError::Data(format!("Failed to combine LPR2 and LPR3 data: {e}"))
            })?;
            Ok(combined)
        }
        (Some(lpr2), None) => {
            // Only LPR2 data available
            Ok(lpr2)
        }
        (None, Some(lpr3)) => {
            // Only LPR3 data available
            Ok(lpr3)
        }
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
    let date_idx = data
        .schema()
        .index_of("admission_date")
        .map_err(|e| IdsError::Data(format!("admission_date column not found: {e}")))?;

    let date_array = data.column(date_idx);
    let date_array = date_array
        .as_any()
        .downcast_ref::<Date32Array>()
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

    // Apply filter to all columns
    let mut filtered_columns = Vec::with_capacity(data.num_columns());

    for col in data.columns() {
        let filtered_col = filter(col, &mask_array)
            .map_err(|e| IdsError::Data(format!("Failed to filter column: {e}")))?;
        filtered_columns.push(filtered_col);
    }

    // Create filtered batch
    let filtered_batch = RecordBatch::try_new(data.schema(), filtered_columns)
        .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))?;

    Ok(filtered_batch)
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
    let filtered_data = filter_by_date_range(&combined_data, config.start_date, config.end_date)?;

    Ok(filtered_data)
}

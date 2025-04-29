//! Integration of population data with additional registers
//!
//! This module implements the integration of population data with death,
//! death cause, and migration registers.

use arrow::array::{Array, ArrayRef, Date32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use std::collections::HashMap;
use std::sync::Arc;

use crate::algorithm::population::core::PopulationConfig;
use crate::error::{IdsError, Result};

/// Integrates population data with death, death cause, and migration data
pub fn integrate_population_data(
    population_data: &RecordBatch,
    death_data: Option<&[RecordBatch]>,
    death_cause_data: Option<&[RecordBatch]>,
    migration_data: Option<&[RecordBatch]>,
    config: &PopulationConfig,
) -> Result<RecordBatch> {
    // Create hashmap of PNR to row index for population data
    let pnr_col = population_data
        .column_by_name("PNR")
        .ok_or_else(|| IdsError::Data("PNR column not found in population data".to_string()))?;

    let pnr_array = pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column is not a StringArray".to_string()))?;

    // Create PNR to index mapping
    let mut pnr_to_index = HashMap::with_capacity(pnr_array.len());
    for i in 0..pnr_array.len() {
        if let Some(pnr) = pnr_array.value(i).to_string().into() {
            pnr_to_index.insert(pnr, i);
        }
    }

    // Start with existing population columns
    let mut result_columns = Vec::with_capacity(
        population_data.num_columns()
            + (if config.include_death_data { 1 } else { 0 })
            + (if config.include_death_cause_data {
                2
            } else {
                0
            })
            + (if config.include_migration_data { 2 } else { 0 }),
    );

    for i in 0..population_data.num_columns() {
        result_columns.push(population_data.column(i).clone());
    }

    // Add death data if included
    let mut death_date_array = None;
    if config.include_death_data {
        if let Some(data) = death_data {
            death_date_array = Some(integrate_death_data(data, &pnr_to_index, pnr_array.len())?);
            result_columns.push(death_date_array.clone().unwrap());
        }
    }

    // Add death cause data if included
    let mut death_cause_array = None;
    let mut death_cause_chapter_array = None;
    if config.include_death_cause_data {
        if let Some(data) = death_cause_data {
            let (cause, chapter) =
                integrate_death_cause_data(data, &pnr_to_index, pnr_array.len())?;
            death_cause_array = Some(cause.clone());
            death_cause_chapter_array = Some(chapter.clone());
            result_columns.push(cause);
            result_columns.push(chapter);
        }
    }

    // Add migration data if included
    let mut migration_type_array = None;
    let mut migration_date_array = None;
    if config.include_migration_data {
        if let Some(data) = migration_data {
            let (mtype, mdate) = integrate_migration_data(data, &pnr_to_index, pnr_array.len())?;
            migration_type_array = Some(mtype.clone());
            migration_date_array = Some(mdate.clone());
            result_columns.push(mtype);
            result_columns.push(mdate);
        }
    }

    // Create field definitions for the new schema
    let mut fields: Vec<Field> = population_data
        .schema()
        .fields()
        .iter()
        .map(|f| Field::new(f.name(), f.data_type().clone(), f.is_nullable()))
        .collect();

    if death_date_array.is_some() {
        fields.push(Field::new("DEATH_DATE", DataType::Date32, true));
    }

    if death_cause_array.is_some() {
        fields.push(Field::new("DEATH_CAUSE", DataType::Utf8, true));
    }

    if death_cause_chapter_array.is_some() {
        fields.push(Field::new("DEATH_CAUSE_CHAPTER", DataType::Utf8, true));
    }

    if migration_type_array.is_some() {
        fields.push(Field::new("MIGRATION_TYPE", DataType::Utf8, true));
    }

    if migration_date_array.is_some() {
        fields.push(Field::new("MIGRATION_DATE", DataType::Date32, true));
    }

    // Create the final batch with all integrated data
    let integrated_schema = Schema::new(fields);
    let result_batch = RecordBatch::try_new(Arc::new(integrated_schema), result_columns)?;

    Ok(result_batch)
}

/// Integrates death register data with population data
fn integrate_death_data(
    death_data: &[RecordBatch],
    pnr_to_index: &HashMap<String, usize>,
    population_size: usize,
) -> Result<ArrayRef> {
    // Create a null Date32Array of the population size
    let mut death_dates = vec![None; population_size];

    // Process each death batch
    for batch in death_data {
        let pnr_col = batch
            .column_by_name("PNR")
            .ok_or_else(|| IdsError::Data("PNR column not found in death data".to_string()))?;

        let date_col = batch.column_by_name("DEATH_DATE").ok_or_else(|| {
            IdsError::Data("DEATH_DATE column not found in death data".to_string())
        })?;

        let pnr_array = pnr_col
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data("PNR column is not a StringArray".to_string()))?;

        let date_array = date_col
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::Data("DEATH_DATE column is not a Date32Array".to_string()))?;

        // Copy death dates to the correct population index
        for i in 0..pnr_array.len() {
            if date_array.is_null(i) {
                continue;
            }

            if let Some(pnr) = pnr_array.value(i).to_string().into() {
                if let Some(&idx) = pnr_to_index.get(&pnr) {
                    death_dates[idx] = Some(date_array.value(i));
                }
            }
        }
    }

    // Create the final Date32Array
    let array = Date32Array::from(death_dates);
    Ok(Arc::new(array) as ArrayRef)
}

/// Integrates death cause register data with population data
fn integrate_death_cause_data(
    death_cause_data: &[RecordBatch],
    pnr_to_index: &HashMap<String, usize>,
    population_size: usize,
) -> Result<(ArrayRef, ArrayRef)> {
    // Create null arrays of the population size
    let mut death_causes = vec![None; population_size];
    let mut death_chapters = vec![None; population_size];

    // Process each death cause batch
    for batch in death_cause_data {
        let pnr_col = batch.column_by_name("PNR").ok_or_else(|| {
            IdsError::Data("PNR column not found in death cause data".to_string())
        })?;

        let cause_col = batch.column_by_name("DEATH_CAUSE").ok_or_else(|| {
            IdsError::Data("DEATH_CAUSE column not found in death cause data".to_string())
        })?;

        let chapter_col = batch
            .column_by_name("DEATH_CAUSE_CHAPTER")
            .ok_or_else(|| IdsError::Data("DEATH_CAUSE_CHAPTER column not found".to_string()))?;

        let pnr_array = pnr_col
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data("PNR column is not a StringArray".to_string()))?;

        let cause_array = cause_col
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data("DEATH_CAUSE column is not a StringArray".to_string()))?;

        let chapter_array = chapter_col
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                IdsError::Data("DEATH_CAUSE_CHAPTER column is not a StringArray".to_string())
            })?;

        // Copy data to the correct population index
        for i in 0..pnr_array.len() {
            if let Some(pnr) = pnr_array.value(i).to_string().into() {
                if let Some(&idx) = pnr_to_index.get(&pnr) {
                    if !cause_array.is_null(i) {
                        death_causes[idx] = Some(cause_array.value(i).to_string());
                    }

                    if !chapter_array.is_null(i) {
                        death_chapters[idx] = Some(chapter_array.value(i).to_string());
                    }
                }
            }
        }
    }

    // Create the final arrays
    let cause_array = Arc::new(StringArray::from(death_causes)) as ArrayRef;
    let chapter_array = Arc::new(StringArray::from(death_chapters)) as ArrayRef;

    Ok((cause_array, chapter_array))
}

/// Integrates migration register data with population data
fn integrate_migration_data(
    migration_data: &[RecordBatch],
    pnr_to_index: &HashMap<String, usize>,
    population_size: usize,
) -> Result<(ArrayRef, ArrayRef)> {
    // Create null arrays of the population size
    let mut migration_types = vec![None; population_size];
    let mut migration_dates = vec![None; population_size];

    // Process each migration batch
    for batch in migration_data {
        let pnr_col = batch
            .column_by_name("PNR")
            .ok_or_else(|| IdsError::Data("PNR column not found in migration data".to_string()))?;

        let type_col = batch.column_by_name("MIGRATION_TYPE").ok_or_else(|| {
            IdsError::Data("MIGRATION_TYPE column not found in migration data".to_string())
        })?;

        let date_col = batch.column_by_name("MIGRATION_DATE").ok_or_else(|| {
            IdsError::Data("MIGRATION_DATE column not found in migration data".to_string())
        })?;

        let pnr_array = pnr_col
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data("PNR column is not a StringArray".to_string()))?;

        let type_array = type_col
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| {
                IdsError::Data("MIGRATION_TYPE column is not a StringArray".to_string())
            })?;

        let date_array = date_col
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(|| {
                IdsError::Data("MIGRATION_DATE column is not a Date32Array".to_string())
            })?;

        // Copy migration data to the correct population index (use the most recent event)
        for i in 0..pnr_array.len() {
            if let Some(pnr) = pnr_array.value(i).to_string().into() {
                if let Some(&idx) = pnr_to_index.get(&pnr) {
                    // Check if we should update (take newer records)
                    let should_update = if date_array.is_null(i) {
                        false
                    } else if let Some(existing_date) = migration_dates[idx] {
                        date_array.value(i) > existing_date
                    } else {
                        true
                    };

                    if should_update && !type_array.is_null(i) && !date_array.is_null(i) {
                        migration_types[idx] = Some(type_array.value(i).to_string());
                        migration_dates[idx] = Some(date_array.value(i));
                    }
                }
            }
        }
    }

    // Create the final arrays
    let type_array = Arc::new(StringArray::from(migration_types)) as ArrayRef;
    let date_array = Arc::new(Date32Array::from(migration_dates)) as ArrayRef;

    Ok((type_array, date_array))
}

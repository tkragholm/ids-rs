//! Population data models for working with population and family data.

use std::collections::HashMap;
use chrono::NaiveDate;
use arrow::record_batch::RecordBatch;
use arrow::array::{StringArray, Date32Array, Array};
use arrow::datatypes::{DataType, Field, Schema};

use crate::error::{IdsError, Result};
use crate::model::pnr::Pnr;

/// Represents a person in the population with basic demographic information
#[derive(Debug, Clone)]
pub struct Population {
    /// Personal identification number
    pub pnr: Pnr,
    /// Birth date
    pub birth_date: NaiveDate,
    /// Father's personal identification number, if available
    pub father_id: Option<Pnr>,
    /// Mother's personal identification number, if available
    pub mother_id: Option<Pnr>,
    /// Family identification number, if available
    pub family_id: Option<String>,
}

/// Represents a person with linked family information including parent birth dates
#[derive(Debug, Clone)]
pub struct FamilyData {
    /// Personal identification number
    pub pnr: Pnr,
    /// Birth date
    pub birth_date: NaiveDate,
    /// Father's personal identification number, if available
    pub father_id: Option<Pnr>,
    /// Father's birth date, if available
    pub father_birth_date: Option<NaiveDate>,
    /// Mother's personal identification number, if available
    pub mother_id: Option<Pnr>,
    /// Mother's birth date, if available
    pub mother_birth_date: Option<NaiveDate>,
    /// Family identification number, if available
    pub family_id: Option<String>,
}

/// Utility functions for working with population data
impl Population {
    /// Create a new Population instance
    #[must_use] pub const fn new(
        pnr: Pnr,
        birth_date: NaiveDate,
        father_id: Option<Pnr>,
        mother_id: Option<Pnr>,
        family_id: Option<String>,
    ) -> Self {
        Self {
            pnr,
            birth_date,
            father_id,
            mother_id,
            family_id,
        }
    }

    /// Create a Population from a row in a `RecordBatch`
    pub fn from_record_batch(batch: &RecordBatch, row_index: usize) -> Result<Self> {
        let pnr_col = batch.column_by_name("PNR")
            .ok_or_else(|| IdsError::Data("Missing PNR column".to_string()))?;
        let birth_date_col = batch.column_by_name("FOED_DAG")
            .ok_or_else(|| IdsError::Data("Missing FOED_DAG column".to_string()))?;
        let father_id_col = batch.column_by_name("FAR_ID");
        let mother_id_col = batch.column_by_name("MOR_ID");
        let family_id_col = batch.column_by_name("FAMILIE_ID");

        // Extract the PNR as string and parse it
        let pnr_array = pnr_col.as_any().downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;
        let pnr_str = pnr_array.value(row_index);
        let pnr = Pnr::from(pnr_str);

        // Extract birth date
        let birth_date_array = birth_date_col.as_any().downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::Data("FOED_DAG column is not a date array".to_string()))?;
        let birth_date = birth_date_array.value_as_date(row_index)
            .ok_or_else(|| IdsError::Data("Invalid birth date".to_string()))?;

        // Extract optional father ID
        let father_id = if let Some(col) = father_id_col {
            let array = col.as_any().downcast_ref::<StringArray>()
                .ok_or_else(|| IdsError::Data("FAR_ID column is not a string array".to_string()))?;
            if array.is_null(row_index) {
                None
            } else {
                let id_str = array.value(row_index);
                if id_str.is_empty() {
                    None
                } else {
                    Some(Pnr::from(id_str))
                }
            }
        } else {
            None
        };

        // Extract optional mother ID
        let mother_id = if let Some(col) = mother_id_col {
            let array = col.as_any().downcast_ref::<StringArray>()
                .ok_or_else(|| IdsError::Data("MOR_ID column is not a string array".to_string()))?;
            if array.is_null(row_index) {
                None
            } else {
                let id_str = array.value(row_index);
                if id_str.is_empty() {
                    None
                } else {
                    Some(Pnr::from(id_str))
                }
            }
        } else {
            None
        };

        // Extract optional family ID
        let family_id = if let Some(col) = family_id_col {
            let array = col.as_any().downcast_ref::<StringArray>()
                .ok_or_else(|| IdsError::Data("FAMILIE_ID column is not a string array".to_string()))?;
            if array.is_null(row_index) {
                None
            } else {
                let id_str = array.value(row_index);
                if id_str.is_empty() {
                    None
                } else {
                    Some(id_str.to_string())
                }
            }
        } else {
            None
        };

        Ok(Self {
            pnr,
            birth_date,
            father_id,
            mother_id,
            family_id,
        })
    }

    /// Convert a vector of Population instances to a `RecordBatch`
    pub fn to_record_batch(population: &[Self]) -> Result<RecordBatch> {
        if population.is_empty() {
            return Err(IdsError::Data("Empty population data".to_string()));
        }

        let mut pnr_values = Vec::with_capacity(population.len());
        let mut birth_date_values = Vec::with_capacity(population.len());
        let mut father_id_values = Vec::with_capacity(population.len());
        let mut mother_id_values = Vec::with_capacity(population.len());
        let mut family_id_values = Vec::with_capacity(population.len());

        for person in population {
            pnr_values.push(Some(person.pnr.value().to_string()));
            birth_date_values.push(Some(person.birth_date));
            
            father_id_values.push(person.father_id.as_ref().map(|id| id.value().to_string()));
            mother_id_values.push(person.mother_id.as_ref().map(|id| id.value().to_string()));
            family_id_values.push(person.family_id.clone());
        }

        // Create Arrow arrays
        let pnr_array = StringArray::from(pnr_values);
        
        // Convert NaiveDate to i32 (days since epoch) for Date32Array
        let birth_date_i32: Vec<Option<i32>> = birth_date_values
            .iter()
            .map(|date| date.map(|d| {
                // Convert to days since epoch
                
                d.signed_duration_since(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days() as i32
            }))
            .collect();
        let birth_date_array = Date32Array::from(birth_date_i32);
        
        let father_id_array = StringArray::from(father_id_values);
        let mother_id_array = StringArray::from(mother_id_values);
        let family_id_array = StringArray::from(family_id_values);

        // Create schema
        let schema = Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("FOED_DAG", DataType::Date32, false),
            Field::new("FAR_ID", DataType::Utf8, true),
            Field::new("MOR_ID", DataType::Utf8, true),
            Field::new("FAMILIE_ID", DataType::Utf8, true),
        ]);

        // Create RecordBatch
        let batch = RecordBatch::try_new(
            std::sync::Arc::new(schema),
            vec![
                std::sync::Arc::new(pnr_array),
                std::sync::Arc::new(birth_date_array),
                std::sync::Arc::new(father_id_array),
                std::sync::Arc::new(mother_id_array),
                std::sync::Arc::new(family_id_array),
            ],
        ).map_err(|e| IdsError::Data(format!("Failed to create record batch: {e}")))?;

        Ok(batch)
    }
}

impl FamilyData {
    /// Create a new `FamilyData` instance
    #[must_use] pub const fn new(
        pnr: Pnr,
        birth_date: NaiveDate,
        father_id: Option<Pnr>,
        father_birth_date: Option<NaiveDate>,
        mother_id: Option<Pnr>,
        mother_birth_date: Option<NaiveDate>,
        family_id: Option<String>,
    ) -> Self {
        Self {
            pnr,
            birth_date,
            father_id,
            father_birth_date,
            mother_id,
            mother_birth_date,
            family_id,
        }
    }

    /// Creates family data by linking parent information to population records
    #[must_use] pub fn create_family_data(
        children: &[Population],
        parent_data: &HashMap<Pnr, NaiveDate>,
    ) -> Vec<Self> {
        children.iter().map(|child| {
            let father_birth_date = child.father_id.as_ref()
                .and_then(|id| parent_data.get(id).copied());
            
            let mother_birth_date = child.mother_id.as_ref()
                .and_then(|id| parent_data.get(id).copied());

            Self {
                pnr: child.pnr.clone(),
                birth_date: child.birth_date,
                father_id: child.father_id.clone(),
                father_birth_date,
                mother_id: child.mother_id.clone(),
                mother_birth_date,
                family_id: child.family_id.clone(),
            }
        }).collect()
    }

    /// Convert a vector of `FamilyData` instances to a `RecordBatch`
    pub fn to_record_batch(family_data: &[Self]) -> Result<RecordBatch> {
        if family_data.is_empty() {
            return Err(IdsError::Data("Empty family data".to_string()));
        }

        let mut pnr_values = Vec::with_capacity(family_data.len());
        let mut birth_date_values = Vec::with_capacity(family_data.len());
        let mut father_id_values = Vec::with_capacity(family_data.len());
        let mut father_birth_date_values = Vec::with_capacity(family_data.len());
        let mut mother_id_values = Vec::with_capacity(family_data.len());
        let mut mother_birth_date_values = Vec::with_capacity(family_data.len());
        let mut family_id_values = Vec::with_capacity(family_data.len());

        for person in family_data {
            pnr_values.push(Some(person.pnr.value().to_string()));
            birth_date_values.push(Some(person.birth_date));
            father_id_values.push(person.father_id.as_ref().map(|id| id.value().to_string()));
            father_birth_date_values.push(person.father_birth_date);
            mother_id_values.push(person.mother_id.as_ref().map(|id| id.value().to_string()));
            mother_birth_date_values.push(person.mother_birth_date);
            family_id_values.push(person.family_id.clone());
        }

        // Create Arrow arrays
        let pnr_array = StringArray::from(pnr_values);
        
        // Convert NaiveDate to i32 (days since epoch) for Date32Array
        let birth_date_i32: Vec<Option<i32>> = birth_date_values
            .iter()
            .map(|date| date.map(|d| {
                
                d.signed_duration_since(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days() as i32
            }))
            .collect();
        let birth_date_array = Date32Array::from(birth_date_i32);
        
        let father_id_array = StringArray::from(father_id_values);
        
        // Convert father birth dates
        let father_birth_date_i32: Vec<Option<i32>> = father_birth_date_values
            .iter()
            .map(|date| date.map(|d| {
                
                d.signed_duration_since(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days() as i32
            }))
            .collect();
        let father_birth_date_array = Date32Array::from(father_birth_date_i32);
        
        let mother_id_array = StringArray::from(mother_id_values);
        
        // Convert mother birth dates
        let mother_birth_date_i32: Vec<Option<i32>> = mother_birth_date_values
            .iter()
            .map(|date| date.map(|d| {
                
                d.signed_duration_since(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days() as i32
            }))
            .collect();
        let mother_birth_date_array = Date32Array::from(mother_birth_date_i32);
        
        let family_id_array = StringArray::from(family_id_values);

        // Create schema
        let schema = Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("FOED_DAG", DataType::Date32, false),
            Field::new("FAR_ID", DataType::Utf8, true),
            Field::new("FAR_FDAG", DataType::Date32, true),
            Field::new("MOR_ID", DataType::Utf8, true),
            Field::new("MOR_FDAG", DataType::Date32, true),
            Field::new("FAMILIE_ID", DataType::Utf8, true),
        ]);

        // Create RecordBatch
        let batch = RecordBatch::try_new(
            std::sync::Arc::new(schema),
            vec![
                std::sync::Arc::new(pnr_array),
                std::sync::Arc::new(birth_date_array),
                std::sync::Arc::new(father_id_array),
                std::sync::Arc::new(father_birth_date_array),
                std::sync::Arc::new(mother_id_array),
                std::sync::Arc::new(mother_birth_date_array),
                std::sync::Arc::new(family_id_array),
            ],
        ).map_err(|e| IdsError::Data(format!("Failed to create family data record batch: {e}")))?;

        Ok(batch)
    }
}
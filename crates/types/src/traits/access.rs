use crate::error::{IdsError, Result};
use arrow::{array::{Date32Array, StringArray}, record_batch::RecordBatch};
use chrono::{NaiveDate, Days};

/// Trait for accessing Arrow data
pub trait ArrowAccess {
    /// Get a string array from a record batch
    fn get_string_array<'a>(&self, batch: &'a RecordBatch, column: &str) -> Result<&'a StringArray>;
    
    /// Get a date array from a record batch
    fn get_date_array<'a>(&self, batch: &'a RecordBatch, column: &str) -> Result<&'a Date32Array>;
    
    /// Convert a Date32 value to a NaiveDate
    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate>;
}

// Default implementation for a dummy type
impl ArrowAccess for () {
    fn get_string_array<'a>(&self, batch: &'a RecordBatch, column: &str) -> Result<&'a StringArray> {
        batch
            .column_by_name(column)
            .ok_or_else(move || IdsError::MissingData(format!("Column {column} not found in record batch")))?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(move || IdsError::InvalidFormat(format!("Column {column} is not a StringArray")))
    }
    
    fn get_date_array<'a>(&self, batch: &'a RecordBatch, column: &str) -> Result<&'a Date32Array> {
        batch
            .column_by_name(column)
            .ok_or_else(move || IdsError::MissingData(format!("Column {column} not found in record batch")))?
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(move || IdsError::InvalidFormat(format!("Column {column} is not a Date32Array")))
    }
    
    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate> {
        let unix_epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .ok_or_else(|| IdsError::InvalidDate("Failed to create Unix epoch date".to_string()))?;
        
        unix_epoch
            .checked_add_days(Days::new(days_since_epoch as u64))
            .ok_or_else(|| IdsError::InvalidDate(
                format!("Invalid date: failed to add {days_since_epoch} days to Unix epoch")
            ))
    }
}

// Simple implementation for a temp access type for internal use
pub struct TempAccess;

impl ArrowAccess for TempAccess {
    fn get_string_array<'a>(&self, batch: &'a RecordBatch, column: &str) -> Result<&'a StringArray> {
        batch
            .column_by_name(column)
            .ok_or_else(move || IdsError::MissingData(format!("Column {column} not found in record batch")))?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(move || IdsError::InvalidFormat(format!("Column {column} is not a StringArray")))
    }
    
    fn get_date_array<'a>(&self, batch: &'a RecordBatch, column: &str) -> Result<&'a Date32Array> {
        batch
            .column_by_name(column)
            .ok_or_else(move || IdsError::MissingData(format!("Column {column} not found in record batch")))?
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(move || IdsError::InvalidFormat(format!("Column {column} is not a Date32Array")))
    }
    
    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate> {
        let unix_epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .ok_or_else(|| IdsError::InvalidDate("Failed to create Unix epoch date".to_string()))?;
        
        unix_epoch
            .checked_add_days(Days::new(days_since_epoch as u64))
            .ok_or_else(|| IdsError::InvalidDate(
                format!("Invalid date: failed to add {days_since_epoch} days to Unix epoch")
            ))
    }
}
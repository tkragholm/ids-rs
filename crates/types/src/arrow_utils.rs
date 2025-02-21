use crate::error::IdsError;
use arrow::array::Date32Array;
use arrow::array::{Array, Float64Array, Int32Array, StringArray};
use arrow::record_batch::RecordBatch;
use chrono::{Days, NaiveDate};

/// Combined trait for Arrow data access operations
pub trait ArrowAccess {
    /// Find the index of a PNR in a batch
    fn find_pnr_index(&self, batch: &RecordBatch, pnr: &str) -> Result<Option<usize>, IdsError>;

    /// Get a value from a specific column and index
    fn get_value<T: ArrowValue>(
        &self,
        batch: &RecordBatch,
        column: &str,
        index: usize,
    ) -> Result<Option<T>, IdsError>;

    /// Get a string array from a batch
    fn get_string_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a StringArray, IdsError>;

    /// Get a date array from a batch
    fn get_date_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a Date32Array, IdsError>;

    /// Convert a date32 value to NaiveDate
    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate, IdsError>;
}

/// Trait for types that can be extracted from Arrow arrays
pub trait ArrowValue: Sized {
    /// The Arrow array type this value comes from
    type ArrayType: Array;

    /// Convert from array at index to this type
    fn from_array(array: &Self::ArrayType, index: usize) -> Option<Self>;

    /// Get the array from a column
    fn get_array<'a>(batch: &'a RecordBatch, column: &str)
        -> Result<&'a Self::ArrayType, IdsError>;
}

// Default implementation of ArrowAccess
impl<T> ArrowAccess for T {
    fn find_pnr_index(&self, batch: &RecordBatch, pnr: &str) -> Result<Option<usize>, IdsError> {
        let pnr_array = self.get_string_array(batch, "PNR")?;
        Ok((0..batch.num_rows()).find(|&i| pnr_array.value(i) == pnr))
    }

    fn get_value<V: ArrowValue>(
        &self,
        batch: &RecordBatch,
        column: &str,
        index: usize,
    ) -> Result<Option<V>, IdsError> {
        let array = V::get_array(batch, column)?;
        Ok(V::from_array(array, index))
    }

    fn get_string_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a StringArray, IdsError> {
        batch
            .column_by_name(column_name)
            .ok_or_else(|| IdsError::MissingData(format!("{column_name} column not found")))?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::InvalidFormat(format!("Invalid {column_name} array type")))
    }

    fn get_date_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a Date32Array, IdsError> {
        batch
            .column_by_name(column_name)
            .ok_or_else(|| IdsError::MissingData(format!("{column_name} column not found")))?
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::InvalidFormat(format!("Invalid {column_name} array type")))
    }

    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate, IdsError> {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .ok_or_else(|| IdsError::InvalidDate("Invalid epoch date".to_string()))?;

        if days_since_epoch < -25567 || days_since_epoch > 25567 {
            return Err(IdsError::InvalidDate(format!(
                "Date value {} is outside reasonable range",
                days_since_epoch
            )));
        }

        if days_since_epoch >= 0 {
            epoch
                .checked_add_days(Days::new(days_since_epoch as u64))
                .ok_or_else(|| {
                    IdsError::InvalidDate(format!("Invalid date value: {}", days_since_epoch))
                })
        } else {
            epoch
                .checked_sub_days(Days::new(days_since_epoch.unsigned_abs() as u64))
                .ok_or_else(|| {
                    IdsError::InvalidDate(format!("Invalid date value: {}", days_since_epoch))
                })
        }
    }
}

// Implement ArrowValue for common types
impl ArrowValue for String {
    type ArrayType = StringArray;

    fn from_array(array: &Self::ArrayType, index: usize) -> Option<Self> {
        Some(array.value(index).to_string())
    }

    fn get_array<'a>(
        batch: &'a RecordBatch,
        column: &str,
    ) -> Result<&'a Self::ArrayType, IdsError> {
        batch
            .column_by_name(column)
            .ok_or_else(|| IdsError::MissingData(format!("{column} column not found")))?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::InvalidFormat(format!("Invalid {column} array type")))
    }
}

impl ArrowValue for i32 {
    type ArrayType = Int32Array;

    fn from_array(array: &Self::ArrayType, index: usize) -> Option<Self> {
        Some(array.value(index))
    }

    fn get_array<'a>(
        batch: &'a RecordBatch,
        column: &str,
    ) -> Result<&'a Self::ArrayType, IdsError> {
        batch
            .column_by_name(column)
            .ok_or_else(|| IdsError::MissingData(format!("{column} column not found")))?
            .as_any()
            .downcast_ref::<Int32Array>()
            .ok_or_else(|| IdsError::InvalidFormat(format!("Invalid {column} array type")))
    }
}

impl ArrowValue for f64 {
    type ArrayType = Float64Array;

    fn from_array(array: &Self::ArrayType, index: usize) -> Option<Self> {
        Some(array.value(index))
    }

    fn get_array<'a>(
        batch: &'a RecordBatch,
        column: &str,
    ) -> Result<&'a Self::ArrayType, IdsError> {
        batch
            .column_by_name(column)
            .ok_or_else(|| IdsError::MissingData(format!("{column} column not found")))?
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| IdsError::InvalidFormat(format!("Invalid {column} array type")))
    }
}

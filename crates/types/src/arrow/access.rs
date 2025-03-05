use crate::error::IdsError;
use arrow::array::{
    Array, BooleanArray, Date32Array, Float64Array, Int32Array, StringArray,
};
use arrow::compute::filter_record_batch;
use arrow::compute::kernels::boolean::and;
use arrow::record_batch::RecordBatch;
use chrono::{Days, NaiveDate};
use log;

/// Trait for accessing Arrow data
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

    /// Get array data from a batch column
    fn get_array_data(
        &self,
        batch: &RecordBatch,
        column_name: &str,
    ) -> Result<arrow::array::ArrayData, IdsError>;

    /// Convert a date32 value to NaiveDate
    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate, IdsError>;

    /// Filter a batch by a condition on a column
    fn filter_batch_by_column(
        &self,
        batch: &RecordBatch,
        column: &str,
        value: &str,
    ) -> Result<Option<RecordBatch>, IdsError>;

    /// Filter a batch by date range
    fn filter_batch_by_date_range(
        &self,
        batch: &RecordBatch,
        date_column: &str,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> Result<Option<RecordBatch>, IdsError>;

    /// Sort a batch by a column
    fn sort_batch_by_column(
        &self,
        batch: &RecordBatch,
        column: &str,
        ascending: bool,
    ) -> Result<RecordBatch, IdsError>;

    /// Validate a batch's data fully
    fn validate_batch(&self, batch: &RecordBatch) -> Result<(), IdsError>;
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
        // First try to get column with exact name "PNR"
        if let Ok(pnr_array) = self.get_string_array(batch, "PNR") {
            // First check for exact match
            for i in 0..pnr_array.len() {
                if pnr_array.is_valid(i) && pnr_array.value(i) == pnr {
                    return Ok(Some(i));
                }
            }
            
            // If exact match not found, try case-insensitive match
            let pnr_lower = pnr.to_lowercase();
            for i in 0..pnr_array.len() {
                if pnr_array.is_valid(i) && pnr_array.value(i).to_lowercase() == pnr_lower {
                    log::debug!("Found case-insensitive PNR match: {} (original: {})", pnr_array.value(i), pnr);
                    return Ok(Some(i));
                }
            }
        }
        
        // If PNR column not found or no match, try looking for "pnr" (lowercase)
        if let Ok(pnr_array) = self.get_string_array(batch, "pnr") {
            // Try exact match first
            for i in 0..pnr_array.len() {
                if pnr_array.is_valid(i) && pnr_array.value(i) == pnr {
                    log::debug!("Found PNR match in lowercase 'pnr' column");
                    return Ok(Some(i));
                }
            }
            
            // If no exact match, try case-insensitive
            let pnr_lower = pnr.to_lowercase();
            for i in 0..pnr_array.len() {
                if pnr_array.is_valid(i) && pnr_array.value(i).to_lowercase() == pnr_lower {
                    log::debug!("Found case-insensitive match in lowercase 'pnr' column: {} (original: {})", 
                        pnr_array.value(i), pnr);
                    return Ok(Some(i));
                }
            }
        }
        
        // No match found in either column
        Ok(None)
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
            .ok_or_else(|| IdsError::missing_data(format!("{column_name} column not found")))?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::invalid_format(format!("Invalid {column_name} array type")))
    }

    fn get_date_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a Date32Array, IdsError> {
        batch
            .column_by_name(column_name)
            .ok_or_else(|| IdsError::missing_data(format!("{column_name} column not found")))?
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::invalid_format(format!("Invalid {column_name} array type")))
    }

    fn get_array_data(
        &self,
        batch: &RecordBatch,
        column_name: &str,
    ) -> Result<arrow::array::ArrayData, IdsError> {
        Ok(batch
            .column_by_name(column_name)
            .ok_or_else(|| IdsError::missing_data(format!("{column_name} column not found")))?
            .to_data())
    }

    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate, IdsError> {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .ok_or_else(|| IdsError::invalid_date("Invalid epoch date".to_string()))?;

        if !(-25567..=25567).contains(&days_since_epoch) {
            return Err(IdsError::invalid_date(format!(
                "Date value {days_since_epoch} is outside reasonable range"
            )));
        }

        if days_since_epoch >= 0 {
            epoch
                .checked_add_days(Days::new(days_since_epoch as u64))
                .ok_or_else(|| {
                    IdsError::invalid_date(format!("Invalid date value: {days_since_epoch}"))
                })
        } else {
            epoch
                .checked_sub_days(Days::new(u64::from(days_since_epoch.unsigned_abs())))
                .ok_or_else(|| {
                    IdsError::invalid_date(format!("Invalid date value: {days_since_epoch}"))
                })
        }
    }

    fn filter_batch_by_column(
        &self,
        batch: &RecordBatch,
        column: &str,
        value: &str,
    ) -> Result<Option<RecordBatch>, IdsError> {
        let array = self.get_string_array(batch, column)?;

        #[allow(clippy::needless_range_loop)]
        let mut mask = vec![false; array.len()];
        #[allow(clippy::needless_range_loop)]
        for i in 0..array.len() {
            if array.is_valid(i) && array.value(i) == value {
                mask[i] = true;
            }
        }

        let bool_array = BooleanArray::from(mask);

        let filtered_batch = filter_record_batch(batch, &bool_array)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to filter batch: {e}")))?;

        if filtered_batch.num_rows() == 0 {
            Ok(None)
        } else {
            Ok(Some(filtered_batch))
        }
    }

    fn filter_batch_by_date_range(
        &self,
        batch: &RecordBatch,
        date_column: &str,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> Result<Option<RecordBatch>, IdsError> {
        let date_array = self.get_date_array(batch, date_column)?;
        let array_data = date_array.to_data();

        let start_days =
            (start_date - NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days() as i32;

        // Directly access array data for better performance
        #[allow(clippy::needless_range_loop)]
        let mut mask = vec![false; array_data.len()];
        #[allow(clippy::needless_range_loop)]
        for i in 0..array_data.len() {
            if array_data.is_valid(i) && date_array.value(i) >= start_days {
                mask[i] = true;
            }
        }

        let mut bool_array = BooleanArray::from(mask);

        if let Some(end) = end_date {
            let end_days = (end - NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()).num_days() as i32;

            // Create end date filter
            #[allow(clippy::needless_range_loop)]
            let mut end_mask = vec![false; array_data.len()];
            #[allow(clippy::needless_range_loop)]
            for i in 0..array_data.len() {
                if array_data.is_valid(i) && date_array.value(i) <= end_days {
                    end_mask[i] = true;
                }
            }

            let end_bool_array = BooleanArray::from(end_mask);

            bool_array = and(&bool_array, &end_bool_array).map_err(|e| {
                IdsError::invalid_operation(format!("Failed to combine date filters: {e}"))
            })?;
        }

        let filtered_batch = filter_record_batch(batch, &bool_array)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to filter batch: {e}")))?;

        if filtered_batch.num_rows() == 0 {
            Ok(None)
        } else {
            Ok(Some(filtered_batch))
        }
    }

    fn sort_batch_by_column(
        &self,
        _batch: &RecordBatch,
        _column: &str,
        _ascending: bool,
    ) -> Result<RecordBatch, IdsError> {
        Err(IdsError::invalid_operation(
            "Sorting is not fully implemented in this version",
        ))
    }

    fn validate_batch(&self, batch: &RecordBatch) -> Result<(), IdsError> {
        // Simply validate basic batch properties since schema.validate() doesn't exist anymore
        if batch.num_rows() > 0 && batch.column(0).len() != batch.num_rows() {
            return Err(IdsError::invalid_format("Invalid batch data: column length mismatch"));
        }
        Ok(())
    }
}

// Implement ArrowValue for common types
impl ArrowValue for String {
    type ArrayType = StringArray;

    fn from_array(array: &Self::ArrayType, index: usize) -> Option<Self> {
        if array.is_valid(index) {
            Some(array.value(index).to_string())
        } else {
            None
        }
    }

    fn get_array<'a>(
        batch: &'a RecordBatch,
        column: &str,
    ) -> Result<&'a Self::ArrayType, IdsError> {
        batch
            .column_by_name(column)
            .ok_or_else(|| IdsError::missing_data(format!("{column} column not found")))?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::invalid_format(format!("Invalid {column} array type")))
    }
}

impl ArrowValue for i32 {
    type ArrayType = Int32Array;

    fn from_array(array: &Self::ArrayType, index: usize) -> Option<Self> {
        if array.is_valid(index) {
            Some(array.value(index))
        } else {
            None
        }
    }

    fn get_array<'a>(
        batch: &'a RecordBatch,
        column: &str,
    ) -> Result<&'a Self::ArrayType, IdsError> {
        batch
            .column_by_name(column)
            .ok_or_else(|| IdsError::missing_data(format!("{column} column not found")))?
            .as_any()
            .downcast_ref::<Int32Array>()
            .ok_or_else(|| IdsError::invalid_format(format!("Invalid {column} array type")))
    }
}

impl ArrowValue for f64 {
    type ArrayType = Float64Array;

    fn from_array(array: &Self::ArrayType, index: usize) -> Option<Self> {
        if array.is_valid(index) {
            Some(array.value(index))
        } else {
            None
        }
    }

    fn get_array<'a>(
        batch: &'a RecordBatch,
        column: &str,
    ) -> Result<&'a Self::ArrayType, IdsError> {
        batch
            .column_by_name(column)
            .ok_or_else(|| IdsError::missing_data(format!("{column} column not found")))?
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| IdsError::invalid_format(format!("Invalid {column} array type")))
    }
}
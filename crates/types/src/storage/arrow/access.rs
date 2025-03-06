use crate::error::{IdsError, Result};
use arrow::array::{
    Array, BooleanArray, Date32Array, Float64Array, Int32Array, StringArray,
};
use arrow::compute::filter_record_batch;
use arrow::compute::kernels::boolean::and;
use arrow::record_batch::RecordBatch;
use chrono::{Days, NaiveDate};
use log;

/// Trait for accessing Arrow data
///
/// This trait provides a standardized interface for working with Arrow data,
/// including type-safe accessors, filtering, and validation operations.
pub trait ArrowAccess {
    /// Find the index of a PNR in a batch
    ///
    /// # Arguments
    /// * `batch` - The record batch to search
    /// * `pnr` - The PNR value to find
    ///
    /// # Returns
    /// An option containing the index if found
    fn find_pnr_index(&self, batch: &RecordBatch, pnr: &str) -> Result<Option<usize>>;

    /// Get a value from a specific column and index
    ///
    /// # Arguments
    /// * `batch` - The record batch to access
    /// * `column` - The column name
    /// * `index` - The row index
    ///
    /// # Returns
    /// An option containing the value if it exists and is valid
    fn get_value<T: ArrowValue>(
        &self,
        batch: &RecordBatch,
        column: &str,
        index: usize,
    ) -> Result<Option<T>>;

    /// Get a string array from a batch
    ///
    /// # Arguments
    /// * `batch` - The record batch to access
    /// * `column_name` - The column name
    ///
    /// # Returns
    /// A reference to the string array
    fn get_string_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a StringArray>;

    /// Get a date array from a batch
    ///
    /// # Arguments
    /// * `batch` - The record batch to access
    /// * `column_name` - The column name
    ///
    /// # Returns
    /// A reference to the date array
    fn get_date_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a Date32Array>;

    /// Get array data from a batch column
    ///
    /// # Arguments
    /// * `batch` - The record batch to access
    /// * `column_name` - The column name
    ///
    /// # Returns
    /// The array data
    fn get_array_data(
        &self,
        batch: &RecordBatch,
        column_name: &str,
    ) -> Result<arrow::array::ArrayData>;
    
    /// Get Unix epoch (1970-01-01) date safely
    ///
    /// # Returns
    /// The Unix epoch date
    ///
    /// # Errors
    /// Returns an error if the date 1970-01-01 cannot be created
    fn get_unix_epoch(&self) -> Result<NaiveDate>;
    
    /// Convert a NaiveDate to days since Unix epoch safely
    ///
    /// # Arguments
    /// * `date` - The date to convert
    ///
    /// # Returns
    /// Number of days since Unix epoch as i32
    ///
    /// # Errors
    /// Returns an error if the Unix epoch date can't be created or if the result would
    /// overflow an i32
    fn date_to_days_since_epoch(&self, date: NaiveDate) -> Result<i32>;

    /// Convert a date32 value to NaiveDate
    ///
    /// # Arguments
    /// * `days_since_epoch` - The number of days since Unix epoch
    ///
    /// # Returns
    /// A NaiveDate
    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate>;

    /// Filter a batch by a condition on a column
    ///
    /// # Arguments
    /// * `batch` - The record batch to filter
    /// * `column` - The column name to filter on
    /// * `value` - The value to match
    ///
    /// # Returns
    /// An option containing the filtered batch if any rows match
    fn filter_batch_by_column(
        &self,
        batch: &RecordBatch,
        column: &str,
        value: &str,
    ) -> Result<Option<RecordBatch>>;

    /// Filter a batch by date range
    ///
    /// # Arguments
    /// * `batch` - The record batch to filter
    /// * `date_column` - The column name containing dates
    /// * `start_date` - The minimum date (inclusive)
    /// * `end_date` - The maximum date (inclusive), if specified
    ///
    /// # Returns
    /// An option containing the filtered batch if any rows match
    fn filter_batch_by_date_range(
        &self,
        batch: &RecordBatch,
        date_column: &str,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> Result<Option<RecordBatch>>;

    /// Sort a batch by a column
    ///
    /// # Arguments
    /// * `batch` - The record batch to sort
    /// * `column` - The column name to sort by
    /// * `ascending` - Whether to sort in ascending order
    ///
    /// # Returns
    /// The sorted batch
    fn sort_batch_by_column(
        &self,
        batch: &RecordBatch,
        column: &str,
        ascending: bool,
    ) -> Result<RecordBatch>;

    /// Validate a batch's data fully
    ///
    /// # Arguments
    /// * `batch` - The record batch to validate
    ///
    /// # Returns
    /// Ok if the batch is valid
    fn validate_batch(&self, batch: &RecordBatch) -> Result<()>;
}

/// Trait for types that can be extracted from Arrow arrays
///
/// This trait allows for type-safe access to Arrow array data,
/// with conversions to Rust native types.
pub trait ArrowValue: Sized {
    /// The Arrow array type this value comes from
    type ArrayType: Array;

    /// Convert from array at index to this type
    ///
    /// # Arguments
    /// * `array` - The source array
    /// * `index` - The index to access
    ///
    /// # Returns
    /// An option containing the value if it exists and is valid
    fn from_array(array: &Self::ArrayType, index: usize) -> Option<Self>;

    /// Get the array from a column
    ///
    /// # Arguments
    /// * `batch` - The record batch to access
    /// * `column` - The column name
    ///
    /// # Returns
    /// A reference to the typed array
    fn get_array<'a>(batch: &'a RecordBatch, column: &str)
        -> Result<&'a Self::ArrayType>;
}

// Default implementation of ArrowAccess
impl<T> ArrowAccess for T {
    fn get_unix_epoch(&self) -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(1970, 1, 1)
            .ok_or_else(|| IdsError::invalid_date("Failed to create Unix epoch date (1970-01-01)"))
    }
    
    fn date_to_days_since_epoch(&self, date: NaiveDate) -> Result<i32> {
        let unix_epoch = self.get_unix_epoch()?;
        let days = date.signed_duration_since(unix_epoch).num_days();
        
        // Safely convert to i32, checking for overflow
        i32::try_from(days).map_err(|_| {
            IdsError::invalid_date(
                format!("Date conversion overflow: days since epoch ({days}) exceeds i32 range")
            )
        })
    }
    
    fn find_pnr_index(&self, batch: &RecordBatch, pnr: &str) -> Result<Option<usize>> {
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
    ) -> Result<Option<V>> {
        let array = V::get_array(batch, column)?;
        Ok(V::from_array(array, index))
    }

    fn get_string_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a StringArray> {
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
    ) -> Result<&'a Date32Array> {
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
    ) -> Result<arrow::array::ArrayData> {
        Ok(batch
            .column_by_name(column_name)
            .ok_or_else(|| IdsError::missing_data(format!("{column_name} column not found")))?
            .to_data())
    }

    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate> {
        let epoch = self.get_unix_epoch()?;

        // Validate range for reasonable dates (roughly 70 years before/after epoch)
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
    ) -> Result<Option<RecordBatch>> {
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
    ) -> Result<Option<RecordBatch>> {
        let date_array = self.get_date_array(batch, date_column)?;
        let array_data = date_array.to_data();

        // Convert start date to days since epoch
        let start_days = self.date_to_days_since_epoch(start_date)?;

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

        // If an end date is specified, apply that filter too
        if let Some(end) = end_date {
            // Convert end date to days since epoch
            let end_days = self.date_to_days_since_epoch(end)?;

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

            // Combine start and end date filters with AND operation
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
    ) -> Result<RecordBatch> {
        Err(IdsError::invalid_operation(
            "Sorting is not fully implemented in this version",
        ))
    }

    fn validate_batch(&self, batch: &RecordBatch) -> Result<()> {
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
    ) -> Result<&'a Self::ArrayType> {
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
    ) -> Result<&'a Self::ArrayType> {
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
    ) -> Result<&'a Self::ArrayType> {
        batch
            .column_by_name(column)
            .ok_or_else(|| IdsError::missing_data(format!("{column} column not found")))?
            .as_any()
            .downcast_ref::<Float64Array>()
            .ok_or_else(|| IdsError::invalid_format(format!("Invalid {column} array type")))
    }
}
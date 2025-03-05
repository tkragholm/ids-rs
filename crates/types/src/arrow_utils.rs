use crate::error::IdsError;
use arrow::array::{
    Array, ArrayData, BooleanArray, Date32Array, Float64Array, Int32Array, StringArray,
    make_array,
};
use arrow::buffer::Buffer;
use arrow::compute::filter_record_batch;
use arrow::compute::kernels::boolean::and;
use arrow::datatypes::{DataType, Field, Schema as ArrowSchema};
use arrow::record_batch::RecordBatch;
use chrono::{Days, NaiveDate};
use std::sync::Arc;

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

    /// Get array data from a batch column
    fn get_array_data(
        &self,
        batch: &RecordBatch,
        column_name: &str,
    ) -> Result<ArrayData, IdsError>;

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
    ) -> Result<ArrayData, IdsError> {
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

// Helper function for creating schemas
#[must_use] pub fn create_schema(fields: Vec<(&str, DataType)>) -> ArrowSchema {
    let fields = fields
        .into_iter()
        .map(|(name, data_type)| Field::new(name, data_type, true))
        .collect::<Vec<_>>();

    ArrowSchema::new(fields)
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

/// Utility functions for working with Arrow batches
pub struct ArrowUtils;

impl ArrowUtils {
    /// Create a new empty batch with the given schema
    #[must_use] pub fn create_empty_batch(schema: ArrowSchema) -> RecordBatch {
        let fields = schema.fields();
        let columns = fields
            .iter()
            .map(|field| match field.data_type() {
                DataType::Int32 => Arc::new(Int32Array::from(Vec::<i32>::new())) as Arc<dyn Array>,
                DataType::Float64 => {
                    Arc::new(Float64Array::from(Vec::<f64>::new())) as Arc<dyn Array>
                }
                DataType::Utf8 => {
                    Arc::new(StringArray::from(Vec::<String>::new())) as Arc<dyn Array>
                }
                DataType::Date32 => {
                    Arc::new(Date32Array::from(Vec::<i32>::new())) as Arc<dyn Array>
                }
                _ => Arc::new(StringArray::from(Vec::<String>::new())) as Arc<dyn Array>,
            })
            .collect();

        RecordBatch::try_new(Arc::new(schema), columns).unwrap()
    }

    /// Convert NaiveDate to days since epoch (for Date32 arrays)
    #[must_use] pub fn date_to_days_since_epoch(date: NaiveDate) -> i32 {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
        (date - epoch).num_days() as i32
    }

    /// Concatenate multiple batches with the same schema
    pub fn concat_batches(batches: &[RecordBatch]) -> Result<RecordBatch, IdsError> {
        if batches.is_empty() {
            return Err(IdsError::missing_data("No batches to concatenate"));
        }

        if batches.len() == 1 {
            return Ok(batches[0].clone());
        }

        let schema = Arc::new((*batches[0].schema()).clone());

        // Check that all batches have the same schema using ptr_eq for faster comparison
        // when the schemas are the same instance
        for batch in batches.iter().skip(1) {
            if !Arc::ptr_eq(&batch.schema(), &schema) {
                // Create references that live long enough
                let batch_schema = batch.schema();
                let batch_fields = batch_schema.fields();
                let schema_fields = schema.fields();

                if batch_fields.len() != schema_fields.len() {
                    return Err(IdsError::invalid_operation(
                        "Cannot concatenate batches with different schemas (field count mismatch)",
                    ));
                }

                for (i, field) in schema_fields.iter().enumerate() {
                    let batch_field = &batch_fields[i];
                    if field.name() != batch_field.name()
                        || field.data_type() != batch_field.data_type()
                    {
                        return Err(IdsError::invalid_operation(
                            "Cannot concatenate batches with different schemas (field mismatch)",
                        ));
                    }
                }
            }
        }

        // Concatenate each column
        let mut columns = Vec::with_capacity(schema.fields().len());

        for i in 0..schema.fields().len() {
            let arrays: Vec<&dyn Array> = batches
                .iter()
                .map(|batch| batch.column(i).as_ref())
                .collect();

            let concat = arrow::compute::concat(&arrays).map_err(|e| {
                IdsError::invalid_operation(format!("Failed to concatenate column: {e}"))
            })?;

            columns.push(concat);
        }

        RecordBatch::try_new(schema, columns).map_err(|e| {
            IdsError::invalid_operation(format!("Failed to create concatenated batch: {e}"))
        })
    }

    /// Create an array from builder for efficient memory usage
    pub fn create_optimized_string_array(
        strings: &[String],
        _capacity: usize,
    ) -> Result<StringArray, IdsError> {
        // Estimate total size of all strings
        let total_string_size: usize = strings.iter().map(std::string::String::len).sum();

        // Create buffers
        let mut values = String::with_capacity(total_string_size);
        let mut offsets = Vec::with_capacity(strings.len() + 1);
        let mut nulls = Vec::with_capacity(strings.len());

        // Start with offset 0
        offsets.push(0);

        // Fill the values and offsets
        for s in strings {
            values.push_str(s);
            offsets.push(values.len());
            nulls.push(true); // All values are valid
        }

        // Convert to Arrow buffers
        let values_buffer = Buffer::from(values.into_bytes());
        let offsets_buffer = Buffer::from(offsets.iter().map(|&o| o as i32).collect::<Vec<i32>>());

        // Create array data
        let builder = ArrayData::builder(DataType::Utf8)
            .len(strings.len())
            .add_buffer(offsets_buffer)
            .add_buffer(values_buffer);

        // Build array
        let array_data = unsafe { builder.build_unchecked() };
        Ok(StringArray::from(array_data))
    }

    /// Align a batch's buffers for better memory performance
    #[must_use] pub fn align_batch_buffers(batch: &RecordBatch) -> RecordBatch {
        let columns: Vec<Arc<dyn Array>> = batch
            .columns()
            .iter()
            .map(|col| {
                let mut array_data = col.to_data();
                array_data.align_buffers();
                make_array(array_data)
            })
            .collect();

        RecordBatch::try_new(batch.schema(), columns).expect("Failed to create aligned batch")
    }

    /// Create a sliced array for zero-copy operations
    pub fn slice_array(array: &dyn Array, offset: usize, length: usize) -> Arc<dyn Array> {
        array.slice(offset, length)
    }

    /// Check if two arrays have the same data by comparison
    pub fn arrays_equal_by_ptr(array1: &dyn Array, array2: &dyn Array) -> bool {
        // Since we can't use ptr_eq directly on ArrayData, we compare memory addresses
        std::ptr::eq(
            array1.to_data().buffers()[0].as_ptr(), 
            array2.to_data().buffers()[0].as_ptr()
        )
    }
}

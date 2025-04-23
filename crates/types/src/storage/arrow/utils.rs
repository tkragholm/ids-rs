use crate::error::{IdsError, Result};
use arrow::array::{
    make_array, Array, ArrayData, BooleanArray, Date32Array, Float64Array, Int32Array, StringArray,
};
use arrow::buffer::Buffer;
use arrow::compute::filter_record_batch;
use arrow::datatypes::{DataType, Schema as ArrowSchema};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use std::sync::Arc;

/// Utility functions for working with Arrow batches
///
/// This struct provides static methods for common Arrow operations,
/// including batch creation, manipulation, and optimization.
pub struct ArrowUtils;

impl ArrowUtils {
    /// Find PNR column index in a batch
    ///
    /// Searches for common PNR column names in different casing variations.
    ///
    /// # Arguments
    /// * `batch` - The record batch to search
    ///
    /// # Returns
    /// The index of the PNR column if found
    ///
    /// # Errors
    /// Returns an error if there's an issue accessing columns
    pub fn find_pnr_index(batch: &RecordBatch) -> Result<Option<usize>> {
        // Check common PNR column names
        for name in &["PNR", "pnr", "Pnr", "CPR", "cpr", "Cpr", "id", "ID", "Id"] {
            // The index_of function returns a Result, not an Option
            match batch.schema().index_of(name) {
                Ok(idx) => {
                    // Verify it's a string column
                    if matches!(batch.schema().field(idx).data_type(), DataType::Utf8) {
                        return Ok(Some(idx));
                    }
                }
                Err(_) => continue, // Column name not found, try the next one
            }
        }

        // No PNR column found
        Ok(None)
    }

    /// Filter a batch by a boolean mask
    ///
    /// # Arguments
    /// * `batch` - The record batch to filter
    /// * `mask` - Boolean array where true values keep the corresponding rows
    ///
    /// # Returns
    /// The filtered batch, or None if all rows were filtered out
    ///
    /// # Errors
    /// Returns an error if filtering fails
    pub fn filter_batch_by_mask(batch: &RecordBatch, mask: &[bool]) -> Result<Option<RecordBatch>> {
        // Create a BooleanArray from the mask
        let mask_array = BooleanArray::from(mask.to_vec());

        // Apply the filter
        match filter_record_batch(batch, &mask_array) {
            Ok(filtered) if filtered.num_rows() > 0 => Ok(Some(filtered)),
            Ok(_) => Ok(None), // Empty result
            Err(e) => Err(IdsError::invalid_operation(format!(
                "Failed to filter batch: {}",
                e
            ))),
        }
    }

    /// Create a new empty batch with the given schema
    ///
    /// # Arguments
    /// * `schema` - The schema for the empty batch
    ///
    /// # Returns
    /// A new empty RecordBatch with the provided schema
    ///
    /// # Errors
    /// Returns an error if the empty batch cannot be created
    pub fn create_empty_batch(schema: ArrowSchema) -> Result<RecordBatch> {
        let schema_arc = Arc::new(schema);
        let fields = schema_arc.fields();
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

        let fields_len = fields.len();
        RecordBatch::try_new(schema_arc, columns).map_err(|err| {
            IdsError::invalid_operation(format!(
                "Failed to create empty batch with {} fields: {}",
                fields_len, err
            ))
        })
    }

    /// Get Unix epoch (1970-01-01) date safely
    ///
    /// # Returns
    /// A Result containing the Unix epoch date
    ///
    /// # Errors
    /// Returns an error if the date 1970-01-01 cannot be created
    pub fn get_unix_epoch() -> Result<NaiveDate> {
        NaiveDate::from_ymd_opt(1970, 1, 1)
            .ok_or_else(|| IdsError::invalid_date("Failed to create Unix epoch date (1970-01-01)"))
    }

    /// Convert NaiveDate to days since epoch (for Date32 arrays) safely
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
    pub fn date_to_days_since_epoch(date: NaiveDate) -> Result<i32> {
        let epoch = Self::get_unix_epoch()?;
        let days = date.signed_duration_since(epoch).num_days();

        // Safely convert to i32, checking for overflow
        i32::try_from(days).map_err(|_| {
            IdsError::invalid_date(format!(
                "Date conversion overflow: days since epoch ({days}) exceeds i32 range"
            ))
        })
    }

    /// Concatenate multiple batches with the same schema
    ///
    /// # Arguments
    /// * `batches` - Array of record batches to concatenate
    ///
    /// # Returns
    /// A single concatenated batch
    ///
    /// # Errors
    /// Returns an error if batches have different schemas or concatenation fails
    pub fn concat_batches(batches: &[RecordBatch]) -> Result<RecordBatch> {
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
    ///
    /// # Arguments
    /// * `strings` - The string values to include in the array
    /// * `capacity` - Capacity hint for buffer allocation
    ///
    /// # Returns
    /// An optimized StringArray
    ///
    /// # Errors
    /// Returns an error if array creation fails
    pub fn create_optimized_string_array(
        strings: &[String],
        _capacity: usize,
    ) -> Result<StringArray> {
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
    ///
    /// # Arguments
    /// * `batch` - The record batch to align
    ///
    /// # Returns
    /// A new record batch with aligned buffers
    ///
    /// # Errors
    /// Returns an error if the aligned batch cannot be created
    pub fn align_batch_buffers(batch: &RecordBatch) -> Result<RecordBatch> {
        let columns: Vec<Arc<dyn Array>> = batch
            .columns()
            .iter()
            .map(|col| {
                let mut array_data = col.to_data();
                array_data.align_buffers();
                make_array(array_data)
            })
            .collect();

        RecordBatch::try_new(batch.schema(), columns).map_err(|err| {
            IdsError::invalid_operation(format!("Failed to create aligned batch: {err}"))
        })
    }

    /// Create a sliced array for zero-copy operations
    ///
    /// # Arguments
    /// * `array` - The source array to slice
    /// * `offset` - The starting index
    /// * `length` - The number of elements to include
    ///
    /// # Returns
    /// A new array view representing the slice
    pub fn slice_array(array: &dyn Array, offset: usize, length: usize) -> Arc<dyn Array> {
        array.slice(offset, length)
    }

    /// Check if two arrays have the same data by pointer comparison
    ///
    /// # Arguments
    /// * `array1` - First array to compare
    /// * `array2` - Second array to compare
    ///
    /// # Returns
    /// True if arrays share the same memory
    pub fn arrays_equal_by_ptr(array1: &dyn Array, array2: &dyn Array) -> bool {
        // Since we can't use ptr_eq directly on ArrayData, we compare memory addresses
        std::ptr::eq(
            array1.to_data().buffers()[0].as_ptr(),
            array2.to_data().buffers()[0].as_ptr(),
        )
    }
}

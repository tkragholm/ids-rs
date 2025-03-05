use crate::error::IdsError;
use arrow::array::{
    Array, ArrayData, Date32Array, Float64Array, Int32Array, StringArray,
    make_array, BooleanArray,
};
use arrow::buffer::Buffer;
use arrow::compute::filter_record_batch;
use arrow::datatypes::{DataType, Schema as ArrowSchema};
use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use std::sync::Arc;

/// Utility functions for working with Arrow batches
pub struct ArrowUtils;

impl ArrowUtils {
    /// Find PNR column index in a batch
    pub fn find_pnr_index(&self, batch: &RecordBatch) -> Result<Option<usize>, IdsError> {
        // Check common PNR column names
        for name in &["PNR", "pnr", "Pnr", "CPR", "cpr", "Cpr", "id", "ID", "Id"] {
            // The index_of function returns a Result, not an Option
            match batch.schema().index_of(name) {
                Ok(idx) => {
                    // Verify it's a string column
                    if matches!(batch.schema().field(idx).data_type(), DataType::Utf8) {
                        return Ok(Some(idx));
                    }
                },
                Err(_) => continue, // Column name not found, try the next one
            }
        }
        
        // No PNR column found
        Ok(None)
    }
    
    /// Filter a batch by a boolean mask
    pub fn filter_batch_by_mask(&self, batch: &RecordBatch, mask: &[bool]) -> Result<Option<RecordBatch>, IdsError> {
        // Create a BooleanArray from the mask
        let mask_array = BooleanArray::from(mask.to_vec());
        
        // Apply the filter
        match filter_record_batch(batch, &mask_array) {
            Ok(filtered) if filtered.num_rows() > 0 => Ok(Some(filtered)),
            Ok(_) => Ok(None), // Empty result
            Err(e) => Err(IdsError::invalid_operation(format!("Failed to filter batch: {}", e))),
        }
    }
    
    /// Create a new empty batch with the given schema
    #[must_use] 
    pub fn create_empty_batch(schema: ArrowSchema) -> RecordBatch {
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
    #[must_use] 
    pub fn date_to_days_since_epoch(date: NaiveDate) -> i32 {
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
    #[must_use] 
    pub fn align_batch_buffers(batch: &RecordBatch) -> RecordBatch {
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
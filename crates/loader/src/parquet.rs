use crate::LoaderProgress;
use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;

use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ProjectionMask;
use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use types::arrow_utils::{ArrowAccess, ArrowUtils};
use types::error::IdsError;

/// Reads a Parquet file and returns its contents as a vector of `RecordBatches`.
///
/// # Arguments
///
/// * `path` - A file path to the Parquet file to be read
/// * `schema` - An optional Arrow Schema for projecting specific columns
/// * `progress` - An optional progress tracker for user feedback
///
/// # Returns
///
/// A Result containing a vector of `RecordBatches` or an `IdsError`
///
/// # Errors
///
/// Returns an `IdsError` if:
/// - The file cannot be opened
/// - The file is not a valid Parquet file
/// - There are issues reading the record batches
pub fn read_parquet(
    path: &Path,
    schema: Option<&Schema>,
    progress: Option<&LoaderProgress>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Attempting to read parquet file: {}", path.display());

    // Check if the file exists
    if !path.exists() {
        log::error!("File not found: {}", path.display());
        return Err(IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("File not found: {}", path.display()),
        )));
    }

    // Try to get the canonical path to better understand where we're looking
    let canonical_path = match path.canonicalize() {
        Ok(p) => {
            log::debug!(
                "Canonical path resolved: {} -> {}",
                path.display(),
                p.display()
            );
            p
        }
        Err(e) => {
            log::warn!(
                "Unable to resolve canonical path for {}: {}",
                path.display(),
                e
            );
            path.to_path_buf()
        }
    };

    // Double-check if it's a file
    if !path.is_file() {
        log::error!("Path exists but is not a file: {}", path.display());
        return Err(IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Path is not a file: {}", path.display()),
        )));
    }

    // Open the file
    let file = match File::open(path) {
        Ok(f) => {
            log::debug!("Successfully opened file: {}", canonical_path.display());
            f
        }
        Err(e) => {
            log::error!("Error opening file {}: {}", canonical_path.display(), e);
            return Err(IdsError::Io(e));
        }
    };

    // Get file size for progress reporting
    let file_size = match file.metadata() {
        Ok(meta) => {
            let size = meta.len();
            log::debug!("File size: {} bytes", size);
            size
        }
        Err(e) => {
            log::warn!("Unable to get file size for {}: {}", path.display(), e);
            0
        }
    };

    let pb = progress.map(|p| {
        p.create_file_progress(
            file_size,
            path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown"),
        )
    });

    let builder = match ParquetRecordBatchReaderBuilder::try_new(file) {
        Ok(b) => b,
        Err(e) => {
            log::error!(
                "Failed to create ParquetRecordBatchReaderBuilder for {}: {}",
                path.display(),
                e
            );
            return Err(IdsError::invalid_format(format!(
                "Invalid Parquet file format at {}: {}",
                path.display(),
                e
            )));
        }
    };

    // Increase batch size for better performance
    let batch_size = 16384; // Doubled from original 8192

    let reader = match schema {
        Some(s) => {
            // Safety check to prevent index out of bounds error
            let schema_len = s.fields().len();
            // Only include indices that are within the range of the Parquet schema
            let parquet_schema = builder.parquet_schema();

            // Get the root field count in a schema-descriptor safe way
            // Get count of root fields in the schema
            let root_schema = parquet_schema.root_schema();
            let parquet_schema_len = root_schema.get_fields().len();

            // Verify and log schema lengths
            log::debug!(
                "Arrow schema field count: {}, Parquet schema field count: {}",
                schema_len,
                parquet_schema_len
            );

            // Create a safe projection that only includes fields that exist in both schemas
            let safe_indices: Vec<usize> = (0..schema_len)
                .filter(|i| *i < parquet_schema_len)
                .collect();

            // Create a thread-local counter for schema mismatch warnings
            thread_local! {
                static SCHEMA_MISMATCH_WARNING_COUNT: std::cell::RefCell<usize> = std::cell::RefCell::new(0);
            }

            // Check for schema mismatch
            if safe_indices.len() < schema_len {
                // Only log the first schema mismatch warning per thread
                SCHEMA_MISMATCH_WARNING_COUNT.with(|count| {
                    let mut count = count.borrow_mut();
                    if *count == 0 {
                        // First occurrence - log as warning but with minimal details
                        log::warn!("Schema mismatch detected: Arrow schema has more fields than Parquet schema. Using only common fields.");
                        // More detailed message at debug level
                        log::debug!("Schema details: Arrow has {} fields, Parquet has {} fields",
                                  schema_len, parquet_schema_len);
                    } else if *count == 1 {
                        // Second occurrence - notify at info level that further warnings will be suppressed
                        log::info!("Additional schema mismatches will not be logged (total: {})", *count + 1);
                    }
                    *count += 1;
                });
            }

            let mask = ProjectionMask::roots(parquet_schema, safe_indices);
            builder
                .with_batch_size(batch_size)
                .with_projection(mask)
                .build()
                .map_err(|e| {
                    IdsError::invalid_format(format!("Failed to create Parquet reader: {}", e))
                })?
        }
        None => builder.with_batch_size(batch_size).build().map_err(|e| {
            IdsError::invalid_format(format!("Failed to create Parquet reader: {}", e))
        })?,
    };

    let mut batches = Vec::new();
    let utils = ArrowUtils; // For using the validation functionality

    for batch_result in reader {
        // Get the batch
        let mut batch = batch_result
            .map_err(|e| IdsError::invalid_format(format!("Failed to read batch: {}", e)))?;

        // Validate batch
        if let Err(e) = utils.validate_batch(&batch) {
            log::warn!("Parquet batch validation warning: {}", e);
        }

        // Optimize memory layout
        #[allow(clippy::unnecessary_mut_passed)]
        ArrowUtils::align_batch_buffers(&mut batch);

        if let Some(pb) = &pb {
            pb.inc(batch.get_array_memory_size() as u64);
        }

        batches.push(batch);
    }

    if let Some(pb) = pb {
        pb.finish_with_message("Complete");
    }
    Ok(batches)
}

/// Filter a list of batches by a date range
///
/// # Arguments
///
/// * `batches` - The list of batches to filter
/// * `date_column` - The name of the date column
/// * `start_date` - The start date (inclusive)
/// * `end_date` - The end date (inclusive, optional)
///
/// # Returns
///
/// A Result containing filtered batches or an `IdsError`
#[allow(dead_code)]
pub fn filter_batches_by_date_range(
    batches: &[RecordBatch],
    date_column: &str,
    start_date: chrono::NaiveDate,
    end_date: Option<chrono::NaiveDate>,
) -> Result<Vec<RecordBatch>, IdsError> {
    let utils = ArrowUtils; // Just for trait implementation

    let mut filtered_batches = Vec::with_capacity(batches.len());

    for batch in batches {
        // Validate the batch first
        if let Err(e) = utils.validate_batch(batch) {
            log::warn!("Batch validation warning before filtering: {}", e);
        }

        if let Some(filtered) =
            utils.filter_batch_by_date_range(batch, date_column, start_date, end_date)?
        {
            // Optimize memory layout of the filtered batch
            let mut filtered_mut = filtered;
            #[allow(clippy::unnecessary_mut_passed)]
            ArrowUtils::align_batch_buffers(&mut filtered_mut);
            filtered_batches.push(filtered_mut);
        }
    }

    // If there are multiple small batches, consider combining them for better performance
    if filtered_batches.len() > 1 {
        let mut small_batches = Vec::new();
        let mut large_batches = Vec::new();

        // Separate small and large batches (arbitrary threshold of 1000 rows)
        for batch in filtered_batches {
            if batch.num_rows() < 1000 {
                small_batches.push(batch);
            } else {
                large_batches.push(batch);
            }
        }

        // Combine small batches if any
        if !small_batches.is_empty() {
            if let Some(combined) = combine_batches(&small_batches)? {
                large_batches.push(combined);
            }
        }

        return Ok(large_batches);
    }

    Ok(filtered_batches)
}

/// Combine multiple batches into one
///
/// # Arguments
///
/// * `batches` - The list of batches to combine
///
/// # Returns
///
/// A Result containing a single combined batch or an `IdsError`
#[allow(dead_code)]
pub fn combine_batches(batches: &[RecordBatch]) -> Result<Option<RecordBatch>, IdsError> {
    if batches.is_empty() {
        return Ok(None);
    }

    if batches.len() == 1 {
        return Ok(Some(batches[0].clone()));
    }

    // Use schema pointer equality for faster checks when possible
    if batches.len() > 1 {
        let first_schema = batches[0].schema();
        let all_same_schema = batches
            .iter()
            .skip(1)
            .all(|b| Arc::ptr_eq(&b.schema(), &first_schema));

        if all_same_schema {
            // If schemas are identical by pointer, we can use a faster path
            let combined = ArrowUtils::concat_batches(batches)?;

            // Optimize the combined batch's memory layout
            let mut combined_mut = combined;
            #[allow(clippy::unnecessary_mut_passed)]
            ArrowUtils::align_batch_buffers(&mut combined_mut);

            return Ok(Some(combined_mut));
        }
    }

    // Normal path for different schemas
    let combined = ArrowUtils::concat_batches(batches)?;

    // Optimize the combined batch's memory layout
    let mut combined_mut = combined;
    #[allow(clippy::unnecessary_mut_passed)]
    ArrowUtils::align_batch_buffers(&mut combined_mut);

    Ok(Some(combined_mut))
}

/// Split a batch into smaller chunks for parallel processing
///
/// # Arguments
///
/// * `batch` - The batch to split
/// * `chunk_size` - The approximate size of each chunk
///
/// # Returns
///
/// A vector of smaller batches
#[allow(dead_code)]
pub fn split_batch_for_parallel(batch: &RecordBatch, chunk_size: usize) -> Vec<RecordBatch> {
    if batch.num_rows() <= chunk_size {
        return vec![batch.clone()];
    }

    let num_chunks = batch.num_rows().div_ceil(chunk_size);
    let mut result = Vec::with_capacity(num_chunks);

    for i in 0..num_chunks {
        let start = i * chunk_size;
        let length = std::cmp::min(chunk_size, batch.num_rows() - start);

        let mut columns = Vec::with_capacity(batch.num_columns());
        for j in 0..batch.num_columns() {
            columns.push(ArrowUtils::slice_array(
                batch.column(j).as_ref(),
                start,
                length,
            ));
        }

        if let Ok(sliced_batch) = RecordBatch::try_new(batch.schema().clone(), columns) {
            result.push(sliced_batch);
        } else {
            log::warn!("Failed to create sliced batch at index {}", i);
        }
    }

    result
}

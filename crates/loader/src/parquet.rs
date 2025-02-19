use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use parquet::arrow::ProjectionMask;
use std::fs::File;
use std::path::Path;
use types::error::IdsError;

/// Reads a Parquet file and returns its contents as a vector of RecordBatches.
///
/// # Arguments
///
/// * `path` - A file path to the Parquet file to be read
/// * `schema` - An optional Arrow Schema for projecting specific columns
///
/// # Returns
///
/// A Result containing a vector of RecordBatches or an IdsError
///
/// # Errors
///
/// Returns an `IdsError` if:
/// - The file cannot be opened
/// - The file is not a valid Parquet file
/// - There are issues reading the record batches
pub fn read_parquet(path: &Path, schema: Option<&Schema>) -> Result<Vec<RecordBatch>, IdsError> {
    let file = File::open(path).map_err(IdsError::Io)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)
        .map_err(|e| IdsError::InvalidFormat(e.to_string()))?;

    let reader = match schema {
        Some(s) => {
            let indices: Vec<usize> = (0..s.fields().len()).collect();
            let mask = ProjectionMask::roots(builder.parquet_schema(), indices);
            builder
                .with_batch_size(8192)
                .with_projection(mask)
                .build()
                .map_err(|e| IdsError::InvalidFormat(e.to_string()))?
        }
        None => builder
            .build()
            .map_err(|e| IdsError::InvalidFormat(e.to_string()))?,
    };

    let mut batches = Vec::new();
    for batch_result in reader {
        batches.push(batch_result.map_err(|e| IdsError::InvalidFormat(e.to_string()))?);
    }

    Ok(batches)
}

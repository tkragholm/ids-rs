use arrow::datatypes::{DataType, Field, Schema as ArrowSchema};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;
use crate::error::IdsError;

/// Create a schema from field definitions
#[must_use]
pub fn create_schema(fields: Vec<(&str, DataType)>) -> ArrowSchema {
    let fields = fields
        .into_iter()
        .map(|(name, data_type)| Field::new(name, data_type, true))
        .collect::<Vec<_>>();

    ArrowSchema::new(fields)
}

/// Trait for converting between RecordBatch and DataFrame (for Polars integration)
pub trait RecordBatchConversion {
    /// Convert RecordBatch to DataFrame
    #[cfg(feature = "polars_backend")]
    fn to_df(&self) -> Result<polars::prelude::DataFrame, IdsError>;
    
    /// Create RecordBatch from DataFrame
    #[cfg(feature = "polars_backend")]
    fn from_df(df: &polars::prelude::DataFrame) -> Result<RecordBatch, IdsError>;
    
    /// Convert RecordBatch to CSV string
    fn to_csv(&self) -> Result<String, IdsError>;
    
    /// Create RecordBatch from CSV string
    fn from_csv(csv: &str, schema: &ArrowSchema) -> Result<RecordBatch, IdsError>;
}

impl RecordBatchConversion for RecordBatch {
    #[cfg(feature = "polars_backend")]
    fn to_df(&self) -> Result<polars::prelude::DataFrame, IdsError> {
        polars::prelude::DataFrame::try_from(self)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to convert RecordBatch to DataFrame: {e}")))
    }
    
    #[cfg(feature = "polars_backend")]
    fn from_df(df: &polars::prelude::DataFrame) -> Result<RecordBatch, IdsError> {
        RecordBatch::try_from(df)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to convert DataFrame to RecordBatch: {e}")))
    }
    
    fn to_csv(&self) -> Result<String, IdsError> {
        let mut writer = arrow::csv::WriterBuilder::new()
            .has_headers(true)
            .build(Vec::new());
            
        writer.write(self)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to convert RecordBatch to CSV: {e}")))?;
            
        let csv_bytes = writer.into_inner()
            .map_err(|e| IdsError::invalid_operation(format!("Failed to finalize CSV writer: {e}")))?;
            
        String::from_utf8(csv_bytes)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to convert CSV bytes to string: {e}")))
    }
    
    fn from_csv(csv: &str, schema: &ArrowSchema) -> Result<RecordBatch, IdsError> {
        let schema_ref = Arc::new(schema.clone());
        
        let mut reader = arrow::csv::ReaderBuilder::new()
            .has_header(true)
            .with_schema(schema_ref)
            .build(csv.as_bytes())
            .map_err(|e| IdsError::invalid_operation(format!("Failed to create CSV reader: {e}")))?;
            
        reader.next()
            .transpose()
            .map_err(|e| IdsError::invalid_operation(format!("Failed to read CSV data: {e}")))
            .and_then(|maybe_batch| {
                maybe_batch.ok_or_else(|| IdsError::missing_data("No data in CSV"))
            })
    }
}
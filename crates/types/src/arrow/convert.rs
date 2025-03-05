use arrow::datatypes::{DataType, Field, Schema as ArrowSchema};
use arrow::record_batch::RecordBatch;
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
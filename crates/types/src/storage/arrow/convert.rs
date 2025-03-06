use crate::error::Result;
use arrow::datatypes::{DataType, Field, Schema as ArrowSchema};
use arrow::record_batch::RecordBatch;

/// Create a schema from field definitions
///
/// This function simplifies the creation of Arrow schemas by taking
/// a simple vector of name/type pairs.
///
/// # Arguments
/// * `fields` - Vector of (name, datatype) tuples
///
/// # Returns
/// An Arrow Schema with the specified fields
#[must_use]
pub fn create_schema(fields: Vec<(&str, DataType)>) -> ArrowSchema {
    let fields = fields
        .into_iter()
        .map(|(name, data_type)| Field::new(name, data_type, true))
        .collect::<Vec<_>>();

    ArrowSchema::new(fields)
}

/// Trait for converting between RecordBatch and other formats
///
/// This trait provides methods for converting between Arrow RecordBatch
/// and other data formats like CSV.
pub trait RecordBatchConversion {
    /// Convert RecordBatch to CSV string
    ///
    /// # Returns
    /// A CSV string representation of the batch
    ///
    /// # Errors
    /// Returns an error if the conversion fails
    fn to_csv(&self) -> Result<String>;

    /// Create RecordBatch from CSV string
    ///
    /// # Arguments
    /// * `csv` - CSV data as string
    /// * `schema` - Arrow schema for parsing
    ///
    /// # Returns
    /// A RecordBatch containing the parsed data
    ///
    /// # Errors
    /// Returns an error if parsing fails
    fn from_csv(csv: &str, schema: &ArrowSchema) -> Result<RecordBatch>;
}
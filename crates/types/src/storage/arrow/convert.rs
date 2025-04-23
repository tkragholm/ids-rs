use crate::error::Result;
use arrow::array::{Array, ArrayRef};
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

/// Trait for converting between `RecordBatch` and other formats
///
/// This trait provides methods for converting between Arrow `RecordBatch`
/// and other data formats like CSV.
pub trait RecordBatchConversion {
    /// Convert `RecordBatch` to CSV string
    ///
    /// # Returns
    /// A CSV string representation of the batch
    ///
    /// # Errors
    /// Returns an error if the conversion fails
    fn to_csv(&self) -> Result<String>;

    /// Create `RecordBatch` from CSV string
    ///
    /// # Arguments
    /// * `csv` - CSV data as string
    /// * `schema` - Arrow schema for parsing
    ///
    /// # Returns
    /// A `RecordBatch` containing the parsed data
    ///
    /// # Errors
    /// Returns an error if parsing fails
    fn from_csv(csv: &str, schema: &ArrowSchema) -> Result<RecordBatch>;
}

/// Trait for types that can be converted from Arrow arrays
///
/// This trait enables consistent type-safe access to Arrow data by providing
/// a standardized way to extract values from Arrow arrays with proper type
/// checking and conversion.
pub trait ArrowType: Sized {
    /// Convert a value from an Arrow array at the specified index
    ///
    /// # Arguments
    /// * `array` - The Arrow array to extract from
    /// * `index` - The row index to extract
    ///
    /// # Returns
    /// * `Option<Self>` - The converted value or None if conversion failed
    fn from_array(array: &ArrayRef, index: usize) -> Option<Self>;
}

// Implement for basic types used in the codebase

impl ArrowType for String {
    fn from_array(array: &ArrayRef, index: usize) -> Option<Self> {
        if index >= array.len() || array.is_null(index) {
            return None;
        }

        match array.data_type() {
            DataType::Utf8 => {
                let array = array.as_any().downcast_ref::<arrow::array::StringArray>()?;
                Some(array.value(index).to_string())
            }
            DataType::LargeUtf8 => {
                let array = array
                    .as_any()
                    .downcast_ref::<arrow::array::LargeStringArray>()?;
                Some(array.value(index).to_string())
            }
            _ => None,
        }
    }
}

impl ArrowType for i32 {
    fn from_array(array: &ArrayRef, index: usize) -> Option<Self> {
        if index >= array.len() || array.is_null(index) {
            return None;
        }

        match array.data_type() {
            DataType::Int32 => {
                let array = array.as_any().downcast_ref::<arrow::array::Int32Array>()?;
                Some(array.value(index))
            }
            DataType::Date32 => {
                let array = array.as_any().downcast_ref::<arrow::array::Date32Array>()?;
                Some(array.value(index))
            }
            _ => None,
        }
    }
}

impl ArrowType for i64 {
    fn from_array(array: &ArrayRef, index: usize) -> Option<Self> {
        if index >= array.len() || array.is_null(index) {
            return None;
        }

        match array.data_type() {
            DataType::Int64 => {
                let array = array.as_any().downcast_ref::<arrow::array::Int64Array>()?;
                Some(array.value(index))
            }
            _ => None,
        }
    }
}

impl ArrowType for f64 {
    fn from_array(array: &ArrayRef, index: usize) -> Option<Self> {
        if index >= array.len() || array.is_null(index) {
            return None;
        }

        match array.data_type() {
            DataType::Float64 => {
                let array = array
                    .as_any()
                    .downcast_ref::<arrow::array::Float64Array>()?;
                Some(array.value(index))
            }
            _ => None,
        }
    }
}

impl ArrowType for bool {
    fn from_array(array: &ArrayRef, index: usize) -> Option<Self> {
        if index >= array.len() || array.is_null(index) {
            return None;
        }

        match array.data_type() {
            DataType::Boolean => {
                let array = array
                    .as_any()
                    .downcast_ref::<arrow::array::BooleanArray>()?;
                Some(array.value(index))
            }
            _ => None,
        }
    }
}

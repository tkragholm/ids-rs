use arrow::array::{Array, ArrayRef};
use arrow::datatypes::{DataType, SchemaRef};
use arrow::record_batch::RecordBatch;
use std::sync::Arc;

use crate::error::{IdsError, Result};
use crate::storage::arrow::convert::ArrowType;

/// Trait for accessing Arrow data with type safety
///
/// This trait provides a standardized interface for retrieving data from
/// Arrow-based storage with proper type conversion. It handles the details
/// of working with Arrow's data structures and provides a more ergonomic API.
pub trait ArrowAccess {
    /// Get a value from an Arrow column with proper type conversion
    ///
    /// # Arguments
    /// * `column` - The column name to retrieve data from
    /// * `row` - The row index to retrieve
    ///
    /// # Returns
    /// * `Result<T>` - The converted value or an error
    ///
    /// # Errors
    /// Returns an error if:
    /// - The column does not exist
    /// - The row index is out of bounds
    /// - The value cannot be converted to the requested type
    fn get_value<T: ArrowType>(&self, column: &str, row: usize) -> Result<T>;
    
    /// Get a value from an Arrow column with optional type conversion
    ///
    /// Similar to `get_value` but returns None for null values instead of an error.
    ///
    /// # Arguments
    /// * `column` - The column name to retrieve data from
    /// * `row` - The row index to retrieve
    ///
    /// # Returns
    /// * `Result<Option<T>>` - The converted value (or None if null) or an error
    ///
    /// # Errors
    /// Returns an error if:
    /// - The column does not exist
    /// - The row index is out of bounds
    /// - The value cannot be converted to the requested type
    fn get_optional_value<T: ArrowType>(&self, column: &str, row: usize) -> Result<Option<T>>;
    
    /// Check if a column exists
    ///
    /// # Arguments
    /// * `column` - The column name to check
    ///
    /// # Returns
    /// * `bool` - True if the column exists, false otherwise
    fn has_column(&self, column: &str) -> bool;
    
    /// Get the number of rows
    ///
    /// # Returns
    /// * `usize` - The number of rows in the record batch
    fn row_count(&self) -> usize;
    
    /// Get column names
    ///
    /// # Returns
    /// * `Vec<String>` - List of column names in the record batch
    fn column_names(&self) -> Vec<String>;
    
    /// Get the Arrow schema
    ///
    /// # Returns
    /// * `SchemaRef` - Reference to the Arrow schema
    fn schema(&self) -> SchemaRef;
    
    /// Get a column by name
    ///
    /// # Arguments
    /// * `column` - The column name to retrieve
    ///
    /// # Returns
    /// * `Result<ArrayRef>` - The column as an Arrow array or an error
    ///
    /// # Errors
    /// Returns an error if the column does not exist
    fn get_column(&self, column: &str) -> Result<ArrayRef>;
}

/// Extension trait for Arrow access with convenience methods
///
/// This trait provides additional utility methods that build on top of
/// the core ArrowAccess trait.
pub trait ArrowAccessExt: ArrowAccess {
    /// Get a slice of values from a column
    ///
    /// # Arguments
    /// * `column` - The column name to retrieve data from
    /// * `start` - The starting row index
    /// * `end` - The ending row index (exclusive)
    ///
    /// # Returns
    /// * `Result<Vec<T>>` - The converted values or an error
    ///
    /// # Errors
    /// Returns an error if:
    /// - The column does not exist
    /// - The row indices are out of bounds
    /// - The values cannot be converted to the requested type
    fn get_values<T: ArrowType>(&self, column: &str, start: usize, end: usize) -> Result<Vec<T>>;
    
    /// Check if a column is of a specific data type
    ///
    /// # Arguments
    /// * `column` - The column name to check
    /// * `data_type` - The data type to check against
    ///
    /// # Returns
    /// * `bool` - True if the column exists and is of the specified type, false otherwise
    fn is_column_type(&self, column: &str, data_type: &DataType) -> bool;
    
    /// Get all values from a column
    ///
    /// # Arguments
    /// * `column` - The column name to retrieve data from
    ///
    /// # Returns
    /// * `Result<Vec<T>>` - All values in the column or an error
    ///
    /// # Errors
    /// Returns an error if:
    /// - The column does not exist
    /// - The values cannot be converted to the requested type
    fn get_all_values<T: ArrowType>(&self, column: &str) -> Result<Vec<T>>;
    
    /// Get all optional values from a column
    ///
    /// # Arguments
    /// * `column` - The column name to retrieve data from
    ///
    /// # Returns
    /// * `Result<Vec<Option<T>>>` - All values in the column (with nulls as None) or an error
    ///
    /// # Errors
    /// Returns an error if:
    /// - The column does not exist
    /// - The values cannot be converted to the requested type
    fn get_all_optional_values<T: ArrowType>(&self, column: &str) -> Result<Vec<Option<T>>>;
}

// Implement the extension trait for any type that implements ArrowAccess
impl<T: ArrowAccess> ArrowAccessExt for T {
    fn get_values<U: ArrowType>(&self, column: &str, start: usize, end: usize) -> Result<Vec<U>> {
        let end = end.min(self.row_count());
        if start >= end {
            return Ok(Vec::new());
        }
        
        let mut values = Vec::with_capacity(end - start);
        for idx in start..end {
            values.push(self.get_value::<U>(column, idx)?);
        }
        Ok(values)
    }
    
    fn is_column_type(&self, column: &str, data_type: &DataType) -> bool {
        if let Ok(field) = self.schema().field_with_name(column) {
            return field.data_type() == data_type;
        }
        false
    }
    
    fn get_all_values<U: ArrowType>(&self, column: &str) -> Result<Vec<U>> {
        self.get_values::<U>(column, 0, self.row_count())
    }
    
    fn get_all_optional_values<U: ArrowType>(&self, column: &str) -> Result<Vec<Option<U>>> {
        let row_count = self.row_count();
        let mut values = Vec::with_capacity(row_count);
        
        for idx in 0..row_count {
            values.push(self.get_optional_value::<U>(column, idx)?);
        }
        
        Ok(values)
    }
}

/// Implementation of ArrowAccess for RecordBatch
impl ArrowAccess for RecordBatch {
    fn get_value<T: ArrowType>(&self, column: &str, row: usize) -> Result<T> {
        let column = self.get_column(column)?;
        
        if row >= column.len() {
            return Err(IdsError::index_out_of_bounds(format!(
                "Row index {} out of bounds (len: {})",
                row,
                column.len()
            )));
        }
        
        T::from_array(&column, row).ok_or_else(|| {
            IdsError::type_conversion(format!(
                "Failed to convert value at row {} to requested type",
                row
            ))
        })
    }
    
    fn get_optional_value<T: ArrowType>(&self, column: &str, row: usize) -> Result<Option<T>> {
        let column = self.get_column(column)?;
        
        if row >= column.len() {
            return Err(IdsError::index_out_of_bounds(format!(
                "Row index {} out of bounds (len: {})",
                row,
                column.len()
            )));
        }
        
        if column.is_null(row) {
            return Ok(None);
        }
        
        Ok(Some(T::from_array(&column, row).ok_or_else(|| {
            IdsError::type_conversion(format!(
                "Failed to convert value at row {} in column to requested type",
                row
            ))
        })?))
    }
    
    fn has_column(&self, column: &str) -> bool {
        self.schema().field_with_name(column).is_ok()
    }
    
    fn row_count(&self) -> usize {
        self.num_rows()
    }
    
    fn column_names(&self) -> Vec<String> {
        self.schema()
            .fields()
            .iter()
            .map(|f| f.name().clone())
            .collect()
    }
    
    fn schema(&self) -> SchemaRef {
        self.schema().clone()
    }
    
    fn get_column(&self, column: &str) -> Result<ArrayRef> {
        let idx = self.schema().index_of(column).map_err(|_| {
            IdsError::column_not_found(format!("Column '{}' not found", column))
        })?;
        
        Ok(Arc::clone(self.column(idx)))
    }
}

/// Implementation of ArrowAccess for RecordBatch reference
impl ArrowAccess for &RecordBatch {
    fn get_value<T: ArrowType>(&self, column: &str, row: usize) -> Result<T> {
        (*self).get_value(column, row)
    }
    
    fn get_optional_value<T: ArrowType>(&self, column: &str, row: usize) -> Result<Option<T>> {
        (*self).get_optional_value(column, row)
    }
    
    fn has_column(&self, column: &str) -> bool {
        (*self).has_column(column)
    }
    
    fn row_count(&self) -> usize {
        (*self).row_count()
    }
    
    fn column_names(&self) -> Vec<String> {
        (*self).column_names()
    }
    
    fn schema(&self) -> SchemaRef {
        (*self).schema()
    }
    
    fn get_column(&self, column: &str) -> Result<ArrayRef> {
        (*self).get_column(column)
    }
}

// Re-exports for backward compatibility
pub type ArrowAccessor<'a> = dyn ArrowAccess + 'a;
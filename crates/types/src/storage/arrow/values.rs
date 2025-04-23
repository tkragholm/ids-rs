use arrow::record_batch::RecordBatch;
use std::sync::Arc;

use crate::error::Result;

/// A wrapper around Arrow data structures that provides standardized access
///
/// This type ensures that data can be accessed consistently regardless of its
/// source or storage format. It provides a unified interface for working with
/// Arrow data throughout the codebase.
#[derive(Clone)]
pub struct ArrowValue {
    /// The underlying Arrow RecordBatch containing the data
    pub batch: Arc<RecordBatch>,
}

impl ArrowValue {
    /// Create a new ArrowValue from a RecordBatch
    ///
    /// # Arguments
    /// * `batch` - The RecordBatch to wrap
    ///
    /// # Returns
    /// * `ArrowValue` - The wrapped value
    pub fn new(batch: RecordBatch) -> Self {
        Self {
            batch: Arc::new(batch),
        }
    }

    /// Create a new ArrowValue from an Arc-wrapped RecordBatch
    ///
    /// # Arguments
    /// * `batch` - The RecordBatch to wrap, already in an Arc
    ///
    /// # Returns
    /// * `ArrowValue` - The wrapped value
    pub fn from_arc(batch: Arc<RecordBatch>) -> Self {
        Self { batch }
    }

    /// Get the underlying RecordBatch
    ///
    /// # Returns
    /// * `&RecordBatch` - Reference to the wrapped RecordBatch
    pub fn batch(&self) -> &RecordBatch {
        &self.batch
    }

    /// Get the number of rows in the batch
    ///
    /// # Returns
    /// * `usize` - The number of rows
    pub fn row_count(&self) -> usize {
        self.batch.num_rows()
    }

    /// Check if the batch is empty
    ///
    /// # Returns
    /// * `bool` - True if the batch has no rows, false otherwise
    pub fn is_empty(&self) -> bool {
        self.batch.num_rows() == 0
    }

    /// Create an empty ArrowValue with the same schema
    ///
    /// # Returns
    /// * `Result<ArrowValue>` - Empty batch with the same schema
    ///
    /// # Errors
    /// Returns an error if creating the empty batch fails
    pub fn empty_like(&self) -> Result<Self> {
        let schema = self.batch.schema();
        let empty_arrays = schema
            .fields()
            .iter()
            .map(|field| arrow::array::new_empty_array(field.data_type()))
            .collect::<Vec<_>>();

        let empty_batch = RecordBatch::try_new(schema.clone(), empty_arrays)?;
        Ok(Self::new(empty_batch))
    }
}

// Implementation of From<RecordBatch> for easier conversion
impl From<RecordBatch> for ArrowValue {
    fn from(batch: RecordBatch) -> Self {
        Self::new(batch)
    }
}

// Implementation of From<Arc<RecordBatch>> for easier conversion
impl From<Arc<RecordBatch>> for ArrowValue {
    fn from(batch: Arc<RecordBatch>) -> Self {
        Self::from_arc(batch)
    }
}

mod custom_path;
mod file;

pub use custom_path::CustomPathReader;
pub use file::FileReader;

use crate::schema;
use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;
use std::path::Path;
use types::error::IdsError;

/// Trait defining methods for reading different types of data records
///
/// This trait provides an abstraction for reading various data types from different sources,
/// supporting different file formats and data categories.
pub trait DataReader {
    /// Read record batches from a given file path with a specified schema
    ///
    /// # Arguments
    /// * `path` - Path to the file to be read
    /// * `schema` - Schema defining the structure of the data
    ///
    /// # Returns
    /// A vector of `RecordBatches` or an error
    fn read_batches(&self, path: &Path, schema: &Schema) -> Result<Vec<RecordBatch>, IdsError>;

    /// Read Annual Register (AKM) data for a specific year
    ///
    /// # Arguments
    /// * `year` - The year of data to read
    fn read_akm(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError>;

    /// Read Population Register (BEF) data for a specific year, optionally with quarterly granularity
    ///
    /// # Arguments
    /// * `year` - The year of data to read
    /// * `quarter` - Optional quarter of the year
    fn read_bef(&self, year: i32, quarter: Option<i32>) -> Result<Vec<RecordBatch>, IdsError>;

    /// Read Individual (IND) data for a specific year
    ///
    /// # Arguments
    /// * `year` - The year of data to read
    fn read_ind(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError>;

    /// Read Education Data (UDDF) for a specific period
    ///
    /// # Arguments
    /// * `period` - The period of data to read (e.g., "202209")
    fn read_uddf(&self, period: &str) -> Result<Vec<RecordBatch>, IdsError>;

    /// Read Family Relations data
    fn read_family(&self) -> Result<Vec<RecordBatch>, IdsError>;
}
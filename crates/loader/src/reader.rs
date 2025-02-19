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

/// File-based data reader implementation
///
/// Provides concrete methods for reading different types of data from file system
pub struct FileReader {
    base_path: String,
}

impl FileReader {
    /// Create a new `FileReader` with a specified base path
    ///
    /// # Arguments
    /// * `base_path` - Root directory containing data files
    #[must_use] pub const fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

impl DataReader for FileReader {
    fn read_batches(&self, path: &Path, schema: &Schema) -> Result<Vec<RecordBatch>, IdsError> {
        crate::parquet::read_parquet(path, Some(schema))
    }

    fn read_akm(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError> {
        let path = Path::new(&self.base_path)
            .join("akm")
            .join(format!("{year}.parquet"));
        self.read_batches(&path, &schema::akm_schema())
    }

    fn read_bef(&self, year: i32, quarter: Option<i32>) -> Result<Vec<RecordBatch>, IdsError> {
        let filename = match quarter {
            Some(q) => format!("{}{:02}.parquet", year, q * 3),
            None => format!("{year}.parquet"),
        };
        let path = Path::new(&self.base_path).join("bef").join(filename);
        self.read_batches(&path, &schema::bef_schema())
    }

    fn read_ind(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError> {
        let path = Path::new(&self.base_path)
            .join("ind")
            .join(format!("{year}.parquet"));
        self.read_batches(&path, &schema::ind_schema())
    }

    fn read_uddf(&self, period: &str) -> Result<Vec<RecordBatch>, IdsError> {
        let path = Path::new(&self.base_path)
            .join("uddf")
            .join(format!("{period}.parquet"));
        self.read_batches(&path, &schema::uddf_schema())
    }

    fn read_family(&self) -> Result<Vec<RecordBatch>, IdsError> {
        let path = Path::new(&self.base_path).join("family.parquet");
        self.read_batches(&path, &schema::family_schema())
    }
}

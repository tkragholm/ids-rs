use crate::readers::DataReader;
use crate::schema;
use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;
use std::collections::HashSet;
use std::path::Path;
use types::error::IdsError;

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
    #[must_use]
    pub const fn new(base_path: String) -> Self {
        Self { base_path }
    }
}

impl DataReader for FileReader {
    fn read_batches(&self, path: &Path, schema: &Schema) -> Result<Vec<RecordBatch>, IdsError> {
        // Get the absolute path for better diagnostics
        let absolute_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        log::debug!("FileReader attempting to access file: {}", absolute_path.display());
        log::debug!("Checking if exists: {}", path.exists());

        if !path.exists() {
            log::warn!("File does not exist: {}", path.display());
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )));
        }

        log::debug!("Reading batches from {}", path.display());
        let batches = match crate::formats::parquet::read_parquet(path, Some(schema), None, None) {
            Ok(b) => {
                log::debug!("Successfully read {} batches from {}", b.len(), path.display());
                b
            }
            Err(e) => {
                log::debug!("Error reading parquet file {}: {}", path.display(), e);
                return Err(e);
            }
        };
        Ok(batches)
    }
    
    fn read_batches_with_filter(
        &self, 
        path: &Path, 
        schema: &Schema,
        pnr_filter: &HashSet<String>
    ) -> Result<Vec<RecordBatch>, IdsError> {
        // Get the absolute path for better diagnostics
        let absolute_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        log::debug!("FileReader attempting to access file with filter: {}", absolute_path.display());
        log::debug!("Checking if exists: {}", path.exists());

        if !path.exists() {
            log::warn!("File does not exist: {}", path.display());
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )));
        }

        log::debug!("Reading batches with PNR filter from {}", path.display());
        let batches = match crate::formats::parquet::read_parquet_with_filter(path, Some(schema), pnr_filter, None) {
            Ok(b) => {
                log::debug!("Successfully read {} filtered batches from {}", b.len(), path.display());
                b
            }
            Err(e) => {
                log::debug!("Error reading parquet file with filter {}: {}", path.display(), e);
                return Err(e);
            }
        };
        Ok(batches)
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
            None => format!("{year}12.parquet"),
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
        log::debug!("FileReader attempting to read family.parquet from: {}", path.display());
        self.read_batches(&path, &schema::family_schema())
    }
}
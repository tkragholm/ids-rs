use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;
use std::collections::HashSet;
use std::fs::File;
use std::path::{Path, PathBuf};
use types::error::IdsError;

use crate::readers::DataReader;
use crate::schema;

/// Reader for custom specified data paths
///
/// This reader is designed to work with data in custom locations.
/// Instead of using conventional locations, it allows specifying exact file paths.
pub struct CustomPathReader {
    // The base path is used for building relative paths if needed
    #[allow(dead_code)]
    base_path: PathBuf,
    akm_path: Option<PathBuf>,
    bef_path: Option<PathBuf>,
    ind_path: Option<PathBuf>,
    uddf_path: Option<PathBuf>,
    family_path: Option<PathBuf>,
}

impl CustomPathReader {
    /// Create a new CustomPathReader with the given base path and custom paths
    ///
    /// # Arguments
    /// * `base_path` - The base directory for relative paths
    /// * `paths` - A map of register names to paths
    ///
    /// # Returns
    /// A new CustomPathReader instance
    pub fn new(base_path: &Path, paths: std::collections::HashMap<String, PathBuf>) -> Self {
        log::debug!(
            "Creating CustomPathReader with base path: {}",
            base_path.display()
        );

        let akm_path = paths.get("akm").map(|p| base_path.join(p));
        let bef_path = paths.get("bef").map(|p| base_path.join(p));
        let ind_path = paths.get("ind").map(|p| base_path.join(p));
        let uddf_path = paths.get("uddf").map(|p| base_path.join(p));
        let family_path = paths.get("family").map(|p| base_path.join(p));

        log::debug!("AKM path: {:?}", akm_path);
        log::debug!("BEF path: {:?}", bef_path);
        log::debug!("IND path: {:?}", ind_path);
        log::debug!("UDDF path: {:?}", uddf_path);
        log::debug!("Family path: {:?}", family_path);

        Self {
            base_path: base_path.to_path_buf(),
            akm_path,
            bef_path,
            ind_path,
            uddf_path,
            family_path,
        }
    }

    /// Check if file exists and is accessible
    fn check_file(&self, path: &Path) -> Result<(), IdsError> {
        if !path.exists() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )));
        }

        if !path.is_file() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Path is not a file: {}", path.display()),
            )));
        }

        // Check if file can be opened
        match File::open(path) {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Failed to open file {}: {}", path.display(), e);
                Err(IdsError::Io(e))
            }
        }
    }
}

impl DataReader for CustomPathReader {
    fn read_batches(&self, path: &Path, schema: &Schema) -> Result<Vec<RecordBatch>, IdsError> {
        log::info!("CustomPathReader attempting to read {}", path.display());

        if !path.exists() {
            // Be more explicit about missing files
            log::warn!("File does not exist: {}", path.display());
            log::debug!(" File does not exist: {}", path.display());
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )));
        }

        if !path.is_file() {
            log::warn!("Path exists but is not a file: {}", path.display());
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Path is not a file: {}", path.display()),
            )));
        }

        // Check if file can be opened
        match File::open(path) {
            Ok(_) => {
                log::debug!("File {} can be opened", path.display());
            }
            Err(e) => {
                log::error!("Failed to open file {}: {}", path.display(), e);
                log::debug!(" Failed to open file: {}", e);
                return Err(IdsError::Io(e));
            }
        }

        log::info!(
            "File exists and is readable, loading parquet from {}",
            path.display()
        );

        match crate::formats::parquet::read_parquet(path, Some(schema), None, None) {
            Ok(batches) => {
                log::info!(
                    "Successfully read {} batches from {}",
                    batches.len(),
                    path.display()
                );
                Ok(batches)
            }
            Err(e) => {
                log::error!("Error reading parquet file {}: {}", path.display(), e);
                Err(e)
            }
        }
    }

    fn read_batches_with_filter(
        &self,
        path: &Path,
        schema: &Schema,
        pnr_filter: &HashSet<String>,
    ) -> Result<Vec<RecordBatch>, IdsError> {
        log::info!(
            "CustomPathReader attempting to read {} with PNR filter",
            path.display()
        );

        if !path.exists() {
            log::warn!("File does not exist: {}", path.display());
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("File not found: {}", path.display()),
            )));
        }

        if !path.is_file() {
            log::warn!("Path exists but is not a file: {}", path.display());
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Path is not a file: {}", path.display()),
            )));
        }

        // Check if file can be opened
        match File::open(path) {
            Ok(_) => {
                log::debug!("File {} can be opened", path.display());
            }
            Err(e) => {
                log::error!("Failed to open file {}: {}", path.display(), e);
                return Err(IdsError::Io(e));
            }
        }

        log::info!("Loading parquet with PNR filter from {}", path.display());

        match crate::formats::parquet::read_parquet_with_filter(
            path,
            Some(schema),
            pnr_filter,
            None,
        ) {
            Ok(batches) => {
                log::info!(
                    "Successfully read {} batches from {} with PNR filter",
                    batches.len(),
                    path.display()
                );
                Ok(batches)
            }
            Err(e) => {
                log::error!(
                    "Error reading parquet file with filter {}: {}",
                    path.display(),
                    e
                );
                Err(e)
            }
        }
    }

    fn read_akm(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError> {
        log::info!("Reading AKM data for year {}", year);

        let path = match &self.akm_path {
            Some(p) => p.clone(),
            None => {
                return Err(IdsError::config("AKM path not configured for this reader"));
            }
        };

        self.check_file(&path)?;
        self.read_batches(&path, &schema::akm_schema())
    }

    fn read_bef(&self, year: i32, quarter: Option<i32>) -> Result<Vec<RecordBatch>, IdsError> {
        match quarter {
            Some(q) => log::info!("Reading BEF data for year {}, quarter {}", year, q),
            None => log::info!("Reading BEF data for year {}", year),
        }

        let path = match &self.bef_path {
            Some(p) => p.clone(),
            None => {
                return Err(IdsError::config("BEF path not configured for this reader"));
            }
        };

        self.check_file(&path)?;
        self.read_batches(&path, &schema::bef_schema())
    }

    fn read_ind(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError> {
        log::info!("Reading IND data for year {}", year);

        let path = match &self.ind_path {
            Some(p) => p.clone(),
            None => {
                return Err(IdsError::config("IND path not configured for this reader"));
            }
        };

        self.check_file(&path)?;
        self.read_batches(&path, &schema::ind_schema())
    }

    fn read_uddf(&self, period: &str) -> Result<Vec<RecordBatch>, IdsError> {
        log::info!("Reading UDDF data for period {}", period);

        let path = match &self.uddf_path {
            Some(p) => p.clone(),
            None => {
                return Err(IdsError::config("UDDF path not configured for this reader"));
            }
        };

        self.check_file(&path)?;
        self.read_batches(&path, &schema::uddf_schema())
    }

    fn read_family(&self) -> Result<Vec<RecordBatch>, IdsError> {
        log::info!("Reading family relation data");

        let path = match &self.family_path {
            Some(p) => p.clone(),
            None => {
                return Err(IdsError::config(
                    "Family path not configured for this reader",
                ));
            }
        };

        self.check_file(&path)?;
        self.read_batches(&path, &schema::family_schema())
    }
}

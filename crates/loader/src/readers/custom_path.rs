use crate::readers::DataReader;
use crate::schema;
use arrow::record_batch::RecordBatch;
use arrow_schema::Schema;
use std::io::Read;
use std::path::{Path, PathBuf};
use types::error::IdsError;

/// Reader for accessing data with custom paths
///
/// This reader allows overriding default file locations
pub struct CustomPathReader {
    base_path: String,
    custom_paths: hashbrown::HashMap<String, String>,
}

impl CustomPathReader {
    /// Create a new CustomPathReader
    ///
    /// # Arguments
    /// * `base_path` - Default base directory path
    /// * `custom_paths` - Map of register types to custom paths
    #[must_use]
    pub fn new(base_path: String, custom_paths: hashbrown::HashMap<String, String>) -> Self {
        Self {
            base_path,
            custom_paths,
        }
    }

    /// Get a custom path if available, or default
    fn get_path(&self, register_type: &str, default_subdir: Option<&str>) -> String {
        if let Some(custom_path) = self.custom_paths.get(register_type) {
            // Custom path provided - check if it's absolute, contains base_path, or is relative
            let custom_path_obj = Path::new(custom_path);
            if custom_path_obj.is_absolute() {
                log::debug!(
                    "Using absolute custom path for {}: {}",
                    register_type,
                    custom_path
                );
                custom_path.clone()
            } else if custom_path.contains(&self.base_path) {
                // Path already contains base_path, use as-is to avoid duplication
                log::debug!(
                    "Using custom path with base already included for {}: {}",
                    register_type,
                    custom_path
                );
                return custom_path.clone();
            } else {
                // Truly relative to base path
                let full_path = Path::new(&self.base_path).join(custom_path);
                log::debug!(
                    "Using relative custom path for {}: {}",
                    register_type,
                    full_path.display()
                );
                return full_path.to_string_lossy().to_string();
            }
        } else if let Some(subdir) = default_subdir {
            // Use default path construction
            let default_path = format!("{}/{}", self.base_path, subdir);
            log::debug!("Using default path for {}: {}", register_type, default_path);
            default_path
        } else {
            log::debug!("Using base path for {}: {}", register_type, self.base_path);
            self.base_path.clone()
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

        // Check file permissions and basic read access
        match std::fs::File::open(path) {
            Ok(mut file) => {
                // Try to read beginning of file to verify it's readable
                let mut header = [0; 4];
                match file.read_exact(&mut header) {
                    Ok(_) => {
                        if &header != b"PAR1" {
                            log::warn!(
                                "File doesn't have Parquet header signature: {}",
                                path.display()
                            );
                            log::debug!("Not a Parquet file (wrong header): {:?}", header);
                            // Continue anyway but log the warning
                        } else {
                            log::info!("Found Parquet header signature");
                            log::debug!(" Valid Parquet header found");
                        }
                    }
                    Err(e) => {
                        log::error!("Failed to read file header: {}", e);
                        log::debug!(" Failed to read file header: {}", e);
                        return Err(IdsError::Io(e));
                    }
                }
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

        match crate::utils::parquet::read_parquet(path, Some(schema), None, None) {
            Ok(batches) => {
                log::info!(
                    "Successfully read {} batches from {}",
                    batches.len(),
                    path.display()
                );
                Ok(batches)
            }
            Err(e) => {
                log::error!("Error reading batches from {}: {}", path.display(), e);
                log::debug!(" Error reading parquet batches: {}", e);
                Err(e)
            }
        }
    }

    fn read_akm(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError> {
        let akm_path = self.get_path("akm", Some("akm"));
        log::debug!(" AKM path for year {} is: {}", year, akm_path);
        
        let path = Path::new(&akm_path).join(format!("{year}.parquet"));
        log::debug!(" Full AKM file path for year {}: {}", year, path.display());
        log::debug!(
            "Attempting to read AKM file for year {}: {}",
            year,
            path.display()
        );

        // Try to list available years in the directory to help diagnose
        let dir_path = Path::new(&akm_path);
        if dir_path.exists() && dir_path.is_dir() {
            match std::fs::read_dir(dir_path) {
                Ok(entries) => {
                    let parquet_files: Vec<_> = entries
                        .filter_map(Result::ok)
                        .filter(|e| {
                            let path = e.path();
                            path.extension().is_some_and(|ext| ext == "parquet")
                        })
                        .collect();

                    if !parquet_files.is_empty() {
                        log::info!(
                            "Available AKM years in {}: {}",
                            akm_path,
                            parquet_files
                                .iter()
                                .filter_map(|entry| {
                                    entry
                                        .file_name()
                                        .to_str()
                                        .map(|s| s.replace(".parquet", ""))
                                })
                                .collect::<Vec<_>>()
                                .join(", ")
                        );
                    }
                }
                Err(e) => {
                    log::error!("Failed to read AKM directory: {}", e);
                }
            }
        }

        self.read_batches(&path, &schema::akm_schema())
    }

    fn read_bef(&self, year: i32, quarter: Option<i32>) -> Result<Vec<RecordBatch>, IdsError> {
        let bef_path = self.get_path("bef", Some("bef"));
        let filename = match quarter {
            Some(q) => format!("{}{:02}.parquet", year, q * 3),
            None => format!("{year}12.parquet"),
        };
        let path = Path::new(&bef_path).join(&filename);
        log::debug!(
            "Attempting to read BEF file for year {}{}: {}",
            year,
            quarter.map_or("".to_string(), |q| format!(" Q{}", q)),
            path.display()
        );

        self.read_batches(&path, &schema::bef_schema())
    }

    fn read_ind(&self, year: i32) -> Result<Vec<RecordBatch>, IdsError> {
        let ind_path = self.get_path("ind", Some("ind"));
        let path = Path::new(&ind_path).join(format!("{year}.parquet"));
        log::debug!(
            "Attempting to read IND file for year {}: {}",
            year,
            path.display()
        );

        self.read_batches(&path, &schema::ind_schema())
    }

    fn read_uddf(&self, period: &str) -> Result<Vec<RecordBatch>, IdsError> {
        let uddf_path = self.get_path("uddf", Some("uddf"));
        let path = Path::new(&uddf_path).join(format!("{period}.parquet"));
        log::debug!(
            "Attempting to read UDDF file for period {}: {}",
            period,
            path.display()
        );

        self.read_batches(&path, &schema::uddf_schema())
    }

    fn read_family(&self) -> Result<Vec<RecordBatch>, IdsError> {
        // Improved path resolution for family file
        let family_path = self.get_path("family", None);
        log::info!("Resolving family relations file path from: {}", family_path);
        log::debug!("Base path: {}", self.base_path);

        // Try multiple path resolution strategies in sequence
        let mut paths_to_try = Vec::new();

        // Strategy 1: Direct path as provided
        paths_to_try.push(Path::new(&family_path).to_path_buf());

        // Strategy 2: If "registers" exists in the path but not at the end, add family.parquet
        if family_path.contains("registers") && !family_path.ends_with("registers") {
            paths_to_try.push(Path::new(&family_path).join("family.parquet"));
        }

        // Strategy 3: Add .parquet if it doesn't have an extension
        if !family_path.ends_with(".parquet") {
            paths_to_try.push(Path::new(&format!("{}.parquet", family_path)).to_path_buf());
        }

        // Strategy 4: Check if path points to directories that need to be completed
        let path_obj = Path::new(&family_path);
        if path_obj.exists() && path_obj.is_dir() {
            // Try file directly in directory
            paths_to_try.push(path_obj.join("family.parquet"));

            // Try registers subdirectory
            paths_to_try.push(path_obj.join("registers").join("family.parquet"));
        }

        // Strategy 5: Try original base path + family.parquet
        paths_to_try.push(Path::new(&self.base_path).join("family.parquet"));

        // Strategy 6: Try registers subdirectory in base path
        paths_to_try.push(
            Path::new(&self.base_path)
                .join("registers")
                .join("family.parquet"),
        );

        // Try each path in sequence
        for path in &paths_to_try {
            log::info!("Attempting to find family relations at: {}", path.display());
            if path.exists() && path.is_file() {
                log::info!("Found family relations file at: {}", path.display());
                
                // Look for schemas in the IDS_SCHEMAS_DIR environment variable
                if let Ok(schemas_dir) = std::env::var("IDS_SCHEMAS_DIR") {
                    let schema_path = Path::new(&schemas_dir).join("family.json");
                    log::info!("Checking for family schema at: {}", schema_path.display());
                    
                    if schema_path.exists() {
                        log::info!("Found family schema at: {}", schema_path.display());
                        // TODO: Use the schema from the file
                        // For now, just proceed with the default schema
                    }
                }
                
                return self.read_batches(path, &schema::family_schema());
            }
        }

        // If we get here, we couldn't find the file
        let error_msg = format!(
            "Could not find family relations file. Tried paths: {}",
            paths_to_try
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
        log::error!("{}", error_msg);
        Err(IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            error_msg,
        )))
    }
}
use crate::{
    config::RegisterPathConfig,
    progress::LoaderProgress,
    readers::{CustomPathReader, DataReader, FileReader},
    utils,
    IdsError, 
    ArrowStore, 
    StoreLoader, 
    UnifiedStore
};
use std::path::Path;

/// Loads data from parquet files into an ArrowStore
pub struct ParquetLoader;

impl Default for ParquetLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParquetLoader {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }

    pub fn load_from_path(&self, base_path: String) -> Result<ArrowStore, IdsError> {
        <Self as StoreLoader>::load_from_path(base_path)
    }

    /// Load data with custom paths for different register types
    ///
    /// # Arguments
    ///
    /// * `base_path` - Base directory for relative paths
    /// * `custom_paths` - Map of register type to custom path (family, akm, bef, ind, uddf)
    ///   Custom paths can be absolute or relative. If relative and containing 
    ///   the base_path prefix, the path is used as-is. Otherwise, base_path is prepended.
    pub fn load_with_custom_paths_map(
        &self,
        base_path: String,
        custom_paths: hashbrown::HashMap<String, String>,
    ) -> Result<ArrowStore, IdsError> {
        let config = RegisterPathConfig {
            base_path,
            custom_paths,
        };
        <Self as StoreLoader>::load_with_custom_paths(config)
    }

    /// Helper method to load data using a specific DataReader implementation
    ///
    /// This centralizes the loading logic to avoid duplicating code
    fn load_with_reader<R: DataReader>(
        reader: &R,
        store: &mut UnifiedStore,
        progress: &mut LoaderProgress,
    ) -> Result<(), IdsError> {
        // Track missing or problematic data sources
        let mut loading_issues = Vec::new();

        // Load family relations - critical data
        log::info!("Starting to load family relations file");
        
        let family_result = reader.read_family();
        
        match family_result {
            Ok(family_batches) if !family_batches.is_empty() => {
                progress.increment_main();
                log::info!(
                    "Loaded {} family relation batches with {} total rows",
                    family_batches.len(),
                    family_batches.iter().map(|b| b.num_rows()).sum::<usize>()
                );
                
                match store.load_family_relations(family_batches) {
                    Ok(_) => {
                        log::debug!("Successfully loaded family relations into store");
                    }
                    Err(e) => {
                        log::error!("Error loading family relations into store: {}", e);
                        return Err(e);
                    }
                }
            }
            Ok(_) => {
                log::warn!("Family relations loaded successfully but contained no data");
                loading_issues.push("family relations (empty data)".to_string());
            }
            Err(e) => {
                log::error!("Failed to load family relations: {}", e);
                log::error!("This is the critical failure point - error loading family.parquet");
                
                // Return early with a specific error about the family file
                return Err(IdsError::invalid_format(format!(
                    "Failed to load critical family relations file: {}",
                    e
                )));
            }
        }

        // Load AKM data
        progress.main_progress_bar().set_message("Loading AKM data...");
        progress.start_sub_progress(23, "AKM Years".to_string());
        let mut akm_loaded = false;

        log::info!("Starting to load AKM data for years 2000-2022");

        // Load all years 2000-2022
        for year in 2000..=2022 {
            let akm_result = reader.read_akm(year);
            
            match akm_result {
                Ok(batches) if !batches.is_empty() => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    log::info!("Loaded {} AKM batches for year {}", batches.len(), year);
                    
                    match store.add_akm_data(year, batches) {
                        Ok(_) => {
                            log::debug!("Successfully added AKM data for year {}", year);
                            akm_loaded = true;
                        }
                        Err(e) => {
                            log::warn!("Failed to add AKM data for year {}: {}", year, e);
                        }
                    }
                }
                Ok(_) => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    log::warn!("AKM data for year {} was empty", year);
                }
                Err(e) => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    // Only log as error if it's not a simple "file not found" error
                    if e.to_string().contains("File not found") {
                        log::warn!("AKM data for year {} not found: {}", year, e);
                    } else {
                        log::error!("Failed to load AKM data for year {}: {}", year, e);
                    }
                }
            }
        }

        if !akm_loaded {
            loading_issues.push("AKM data (no valid years found)".to_string());
        }
        progress.increment_main();

        // Load IND data
        progress.main_progress_bar().set_message("Loading IND data...");
        progress.start_sub_progress(23, "IND Years".to_string());
        let mut ind_loaded = false;

        for year in 2000..=2022 {
            match reader.read_ind(year) {
                Ok(batches) if !batches.is_empty() => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    log::info!("Loaded {} IND batches for year {}", batches.len(), year);
                    if let Err(e) = store.add_ind_data(year, batches) {
                        log::warn!("Failed to add IND data for year {}: {}", year, e);
                    } else {
                        ind_loaded = true;
                    }
                }
                Ok(_) => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    log::warn!("IND data for year {} was empty", year);
                }
                Err(e) => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    // Only log as error if it's not a simple "file not found" error
                    if e.to_string().contains("File not found") {
                        log::warn!("IND data for year {} not found: {}", year, e);
                    } else {
                        log::error!("Failed to load IND data for year {}: {}", year, e);
                    }
                }
            }
        }

        if !ind_loaded {
            loading_issues.push("IND data (no valid years found)".to_string());
        }
        progress.increment_main();

        // Load BEF data
        progress.main_progress_bar().set_message("Loading BEF data...");
        progress.start_sub_progress(24, "BEF Years".to_string());
        let mut bef_loaded = false;

        for year in 2000..=2018 {
            match reader.read_bef(year, None) {
                Ok(batches) if !batches.is_empty() => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    log::info!("Loaded {} BEF batches for year {}", batches.len(), year);
                    if let Err(e) = store.add_bef_data(format!("{year}"), batches) {
                        log::warn!("Failed to add BEF data for year {}: {}", year, e);
                    } else {
                        bef_loaded = true;
                    }
                }
                Ok(_) => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    log::warn!("BEF data for year {} was empty", year);
                }
                Err(e) => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    // Only log as error if it's not a simple "file not found" error
                    if e.to_string().contains("File not found") {
                        log::warn!("BEF data for year {} not found: {}", year, e);
                    } else {
                        log::error!("Failed to load BEF data for year {}: {}", year, e);
                    }
                }
            }
        }

        for year in 2019..=2023 {
            let mut year_loaded = false;
            for quarter in 1..=4 {
                match reader.read_bef(year, Some(quarter)) {
                    Ok(batches) if !batches.is_empty() => {
                        log::info!(
                            "Loaded {} BEF batches for year {} Q{}",
                            batches.len(),
                            year,
                            quarter
                        );
                        if let Err(e) =
                            store.add_bef_data(format!("{}{:02}", year, quarter * 3), batches)
                        {
                            log::warn!(
                                "Failed to add BEF data for year {} Q{}: {}",
                                year,
                                quarter,
                                e
                            );
                        } else {
                            bef_loaded = true;
                            year_loaded = true;
                        }
                    }
                    Ok(_) => {
                        log::warn!("BEF data for year {} Q{} was empty", year, quarter);
                    }
                    Err(e) => {
                        // Only log as error if it's not a simple "file not found" error
                        if e.to_string().contains("File not found") {
                            log::warn!("BEF data for year {} Q{} not found: {}", year, quarter, e);
                        } else {
                            log::error!(
                                "Failed to load BEF data for year {} Q{}: {}",
                                year,
                                quarter,
                                e
                            );
                        }
                    }
                }
            }
            if !year_loaded {
                log::warn!("No BEF data loaded for year {}", year);
            }
            if let Some(pb) = progress.sub_progress_bar() {
                pb.inc(1);
            }
        }

        if !bef_loaded {
            loading_issues.push("BEF data (no valid years found)".to_string());
        }
        progress.increment_main();

        // Load UDDF data
        progress.main_progress_bar().set_message("Loading UDDF data...");
        progress.start_sub_progress(2, "UDDF Periods".to_string());
        let mut uddf_loaded = false;

        for period in ["202009", "202209"] {
            match reader.read_uddf(period) {
                Ok(batches) if !batches.is_empty() => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    log::info!(
                        "Loaded {} UDDF batches for period {}",
                        batches.len(),
                        period
                    );
                    if let Err(e) = store.add_uddf_data(period.to_string(), batches) {
                        log::warn!("Failed to add UDDF data for period {}: {}", period, e);
                    } else {
                        uddf_loaded = true;
                    }
                }
                Ok(_) => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    log::warn!("UDDF data for period {} was empty", period);
                }
                Err(e) => {
                    if let Some(pb) = progress.sub_progress_bar() {
                        pb.inc(1);
                    }
                    // Only log as error if it's not a simple "file not found" error
                    if e.to_string().contains("File not found") {
                        log::warn!("UDDF data for period {} not found: {}", period, e);
                    } else {
                        log::error!("Failed to load UDDF data for period {}: {}", period, e);
                    }
                }
            }
        }

        if !uddf_loaded {
            loading_issues.push("UDDF data (no valid periods found)".to_string());
        }
        progress.increment_main();

        // Summarize loading issues
        if !loading_issues.is_empty() {
            let issue_msg = format!(
                "Loading incomplete: missing data for {} registers",
                loading_issues.join(", ")
            );
            log::warn!("{}", &issue_msg);
            progress.main_progress_bar().finish_with_message(issue_msg);
        } else {
            progress.main_progress_bar().finish_with_message("Loading complete");
        }

        Ok(())
    }
}

impl StoreLoader for ParquetLoader {
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError> {
        let base_path_obj = Path::new(&base_path);
        if !base_path_obj.exists() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Base directory not found: {}", base_path),
            )));
        }
        
        log::info!("Checking base path structure: {}", base_path);
        
        // Use the utils function to detect directory structure
        let paths = utils::detect_data_structure(base_path_obj)?;
        
        if !paths.is_empty() {
            log::info!("Using detected paths for registers:");
            for (k, v) in &paths {
                log::info!("  - {}: {}", k, v.display());
            }
            
            // Convert to string paths for the custom paths map
            let custom_paths = paths.into_iter()
                .map(|(k, v)| (k, v.to_string_lossy().to_string()))
                .collect();
            
            let config = RegisterPathConfig {
                base_path,
                custom_paths,
            };
            
            Self::load_with_custom_paths(config)
        } else {
            // Use standard approach if no paths could be determined
            log::info!("No custom paths determined - using standard path structure");
            let config = RegisterPathConfig {
                base_path,
                custom_paths: hashbrown::HashMap::new(),
            };
            
            Self::load_with_custom_paths(config)
        }
    }

    fn load_with_custom_paths(config: RegisterPathConfig) -> Result<ArrowStore, IdsError> {
        let base_path = config.base_path.clone();
        log::info!("Creating UnifiedStore for data loading");
        
        let store_result = UnifiedStore::new_arrow();
        if let Err(e) = &store_result {
            log::error!("Failed to create UnifiedStore: {}", e);
        }
        
        let mut store = store_result?;
        let mut progress = LoaderProgress::new();

        log::info!("Loading data from path: {}", base_path);
        progress.main_progress_bar().set_message("Loading family relations...");

        // Verify that the base path exists
        let base_path_obj = Path::new(&base_path);
        if !base_path_obj.exists() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Base path does not exist: {}", base_path),
            )));
        }

        // Choose appropriate reader based on whether custom paths are provided
        if !config.custom_paths.is_empty() {
            log::info!("Using custom paths for register data");
            for (register, path) in &config.custom_paths {
                log::info!("Custom path for '{}': {}", register, path);
            }

            let custom_reader = CustomPathReader::new(base_path, config.custom_paths);
            
            // Use the custom reader for all operations
            if let Err(e) = Self::load_with_reader(&custom_reader, &mut store, &mut progress) {
                log::error!("Failed to load data with custom reader: {}", e);
                return Err(e);
            }
        } else {
            log::info!("Using default reader with standard paths");
            // Use the default reader
            let reader = FileReader::new(base_path);

            // Use the standard reader for all operations
            if let Err(e) = Self::load_with_reader(&reader, &mut store, &mut progress) {
                log::error!("Failed to load data with default reader: {}", e);
                return Err(e);
            }
        }

        // Get the ArrowBackend
        match store.as_arrow_backend() {
            Some(backend) => {
                log::info!("Successfully created Arrow backend");
                Ok(backend.clone())
            }
            None => {
                let error_msg = "Failed to access arrow backend - store may not have loaded properly";
                log::error!("{}", error_msg);
                Err(IdsError::invalid_operation(error_msg))
            }
        }
    }
}
mod parquet;
mod reader;
mod schema;

pub use reader::{DataReader, FileReader};
pub use types::{
    error::IdsError,
    family::FamilyRelations,
    models::*,
    storage::{ArrowBackend as ArrowStore, DataStore as UnifiedStore, Storage as Store},
};

use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::path::Path;

pub struct LoaderProgress {
    multi_progress: MultiProgress,
    main_pb: ProgressBar,
    sub_pb: Option<ProgressBar>,
}

impl Default for LoaderProgress {
    fn default() -> Self {
        Self::new()
    }
}

impl LoaderProgress {
    pub fn new() -> Self {
        let multi_progress = MultiProgress::new();
        let main_style = ProgressStyle::default_bar()
            .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap();

        let main_pb = multi_progress.add(ProgressBar::new(5));
        main_pb.set_style(main_style);
        main_pb.set_prefix("Overall Progress");

        Self {
            multi_progress,
            main_pb,
            sub_pb: None,
        }
    }

    pub fn create_file_progress(&self, size: u64, filename: &str) -> ProgressBar {
        let style = ProgressStyle::default_bar()
                .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.yellow/red} {bytes}/{total_bytes} ({percent}%) {msg}")
                .unwrap()
                .progress_chars("█▇▆▅▄▃▂▁  ");

        let pb = self.multi_progress.add(ProgressBar::new(size));
        pb.set_style(style);
        pb.set_prefix(filename.to_string());
        pb
    }

    pub fn start_sub_progress(&mut self, total: u64, prefix: String) {
        let style = ProgressStyle::default_bar()
                .template("{prefix:.bold.dim} [{elapsed_precise}] {bar:40.yellow/red} {pos}/{len} ({percent}%) {msg}")
                .unwrap();

        let pb = self.multi_progress.add(ProgressBar::new(total));
        pb.set_style(style);
        pb.set_prefix(prefix);
        self.sub_pb = Some(pb);
    }

    pub fn increment_main(&self) {
        self.main_pb.inc(1);
    }

    pub fn increment_sub(&self) {
        if let Some(pb) = &self.sub_pb {
            pb.inc(1);
        }
    }

    pub fn finish_main(&self, msg: &str) {
        self.main_pb.finish_with_message(msg.to_string());
    }
}

pub trait StoreLoader {
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError>;

    /// Load data with custom paths for different register types
    fn load_with_custom_paths(config: RegisterPathConfig) -> Result<ArrowStore, IdsError>;
}

pub struct ParquetLoader;

impl Default for ParquetLoader {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration for customizing register file paths
pub struct RegisterPathConfig {
    pub base_path: String,
    pub custom_paths: hashbrown::HashMap<String, String>,
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
        // Create a new map with properly resolved paths
        let mut resolved_paths = hashbrown::HashMap::new();
        
        // Normalize base_path for easier comparison
        let base_path_obj = std::path::Path::new(&base_path);
        let normalized_base_path = if let Ok(canonical) = base_path_obj.canonicalize() {
            canonical.to_string_lossy().to_string()
        } else {
            base_path.clone()
        };
        
        log::info!("Normalized base path: {}", normalized_base_path);
        
        for (key, path) in custom_paths {
            let path_obj = std::path::Path::new(&path);
            
            // If path is already absolute, use it as-is
            if path_obj.is_absolute() {
                log::debug!("Using absolute custom path for {}: {}", key, path);
                resolved_paths.insert(key, path);
                continue;
            }
            
            // Check if path already includes the base_path
            if path.contains(&base_path) || path.contains(&normalized_base_path) {
                log::debug!("Using custom path (with base prefix) for {}: {}", key, path);
                resolved_paths.insert(key, path);
                continue;
            }
            
            // Special handling for register directories that might be individual files
            if key == "family" && path_obj.extension().is_some_and(|ext| ext == "parquet") {
                // If a specific parquet file is specified, use it directly
                let full_path = if path_obj.is_absolute() {
                    path
                } else {
                    std::path::Path::new(&base_path)
                        .join(path_obj)
                        .to_string_lossy()
                        .to_string()
                };
                log::debug!("Using specific parquet file for {}: {}", key, full_path);
                resolved_paths.insert(key, full_path);
                continue;
            }
            
            // Prepend base_path for relative paths
            let full_path = std::path::Path::new(&base_path)
                .join(path_obj)
                .to_string_lossy()
                .to_string();
            log::debug!("Resolved relative path for {}: {} -> {}", key, path, full_path);
            resolved_paths.insert(key, full_path);
        }
        
        // Set a debug message for all resolved paths
        for (key, path) in &resolved_paths {
            log::info!("Final resolved path for {}: {}", key, path);
        }
        
        let config = RegisterPathConfig {
            base_path,
            custom_paths: resolved_paths,
        };
        <Self as StoreLoader>::load_with_custom_paths(config)
    }

    /// Helper method to load data using a specific DataReader implementation
    ///
    /// This centralizes the loading logic to avoid duplicating code
    fn load_with_reader<R: reader::DataReader>(
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
                
                // Wrap this in a try block to see if the error happens here
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
        progress.main_pb.set_message("Loading AKM data...");
        progress.start_sub_progress(23, "AKM Years".to_string());
        let mut akm_loaded = false;

        log::info!("Starting to load AKM data for years 2000-2022");

        // Load all years 2000-2022
        for year in 2000..=2022 {
            let akm_result = reader.read_akm(year);
            
            match akm_result {
                Ok(batches) if !batches.is_empty() => {
                    if let Some(ref pb) = progress.sub_pb {
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
                    if let Some(ref pb) = progress.sub_pb {
                        pb.inc(1);
                    }
                    log::warn!("AKM data for year {} was empty", year);
                }
                Err(e) => {
                    if let Some(ref pb) = progress.sub_pb {
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
        progress.main_pb.set_message("Loading IND data...");
        progress.start_sub_progress(23, "IND Years".to_string());
        let mut ind_loaded = false;

        for year in 2000..=2022 {
            match reader.read_ind(year) {
                Ok(batches) if !batches.is_empty() => {
                    if let Some(ref pb) = progress.sub_pb {
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
                    if let Some(ref pb) = progress.sub_pb {
                        pb.inc(1);
                    }
                    log::warn!("IND data for year {} was empty", year);
                }
                Err(e) => {
                    if let Some(ref pb) = progress.sub_pb {
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
        progress.main_pb.set_message("Loading BEF data...");
        progress.start_sub_progress(24, "BEF Years".to_string());
        let mut bef_loaded = false;

        for year in 2000..=2018 {
            match reader.read_bef(year, None) {
                Ok(batches) if !batches.is_empty() => {
                    if let Some(ref pb) = progress.sub_pb {
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
                    if let Some(ref pb) = progress.sub_pb {
                        pb.inc(1);
                    }
                    log::warn!("BEF data for year {} was empty", year);
                }
                Err(e) => {
                    if let Some(ref pb) = progress.sub_pb {
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
            if let Some(ref pb) = progress.sub_pb {
                pb.inc(1);
            }
        }

        if !bef_loaded {
            loading_issues.push("BEF data (no valid years found)".to_string());
        }
        progress.increment_main();

        // Load UDDF data
        progress.main_pb.set_message("Loading UDDF data...");
        progress.start_sub_progress(2, "UDDF Periods".to_string());
        let mut uddf_loaded = false;

        for period in ["202009", "202209"] {
            match reader.read_uddf(period) {
                Ok(batches) if !batches.is_empty() => {
                    if let Some(ref pb) = progress.sub_pb {
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
                    if let Some(ref pb) = progress.sub_pb {
                        pb.inc(1);
                    }
                    log::warn!("UDDF data for period {} was empty", period);
                }
                Err(e) => {
                    if let Some(ref pb) = progress.sub_pb {
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
            progress.main_pb.finish_with_message(issue_msg);
        } else {
            progress.main_pb.finish_with_message("Loading complete");
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
        
        // Check for direct vs nested structure (with /registers subdirectory)
        let registers_path = base_path_obj.join("registers");
        let has_registers_subdir = registers_path.exists() && registers_path.is_dir();
        
        let mut custom_paths = hashbrown::HashMap::new();
        
        // Logic to auto-detect structure and create appropriate custom_paths
        if has_registers_subdir {
            log::info!("Detected 'registers' subdirectory - checking for data inside");
            
            // Check for family.parquet
            let family_in_registers = registers_path.join("family.parquet");
            let family_direct = base_path_obj.join("family.parquet");
            
            if family_in_registers.exists() {
                log::info!("Found family.parquet in registers subdirectory");
                custom_paths.insert("family".to_string(), family_in_registers.to_string_lossy().to_string());
            } else if family_direct.exists() {
                log::info!("Found family.parquet in base directory");
                custom_paths.insert("family".to_string(), family_direct.to_string_lossy().to_string());
            } else {
                log::warn!("Family file not found in either location - will try default paths later");
            }
            
            // Check for register subdirectories
            let register_dirs = ["akm", "bef", "ind", "uddf"];
            for dir in &register_dirs {
                let dir_in_registers = registers_path.join(dir);
                let dir_direct = base_path_obj.join(dir);
                
                if dir_in_registers.exists() && dir_in_registers.is_dir() {
                    // Check if it has parquet files
                    match std::fs::read_dir(&dir_in_registers) {
                        Ok(entries) => {
                            let parquet_files: Vec<_> = entries
                                .filter_map(Result::ok)
                                .filter(|e| e.path().extension().is_some_and(|ext| ext == "parquet"))
                                .collect();
                                
                            if !parquet_files.is_empty() {
                                log::info!("Found {} directory in registers with {} parquet files", 
                                          dir, parquet_files.len());
                                custom_paths.insert(dir.to_string(), dir_in_registers.to_string_lossy().to_string());
                            } else {
                                log::warn!("{} directory exists in registers but has no parquet files", dir);
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to read {} directory in registers: {}", dir, e);
                        }
                    }
                } else if dir_direct.exists() && dir_direct.is_dir() {
                    // Check if it has parquet files
                    match std::fs::read_dir(&dir_direct) {
                        Ok(entries) => {
                            let parquet_files: Vec<_> = entries
                                .filter_map(Result::ok)
                                .filter(|e| e.path().extension().is_some_and(|ext| ext == "parquet"))
                                .collect();
                                
                            if !parquet_files.is_empty() {
                                log::info!("Found {} directory in base path with {} parquet files", 
                                          dir, parquet_files.len());
                                custom_paths.insert(dir.to_string(), dir_direct.to_string_lossy().to_string());
                            } else {
                                log::warn!("{} directory exists in base path but has no parquet files", dir);
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to read {} directory in base path: {}", dir, e);
                        }
                    }
                } else {
                    log::warn!("{} directory not found in either location", dir);
                }
            }
        }
        
        // If we've found custom paths, use them
        if !custom_paths.is_empty() {
            log::info!("Using auto-detected paths for registers:");
            for (k, v) in &custom_paths {
                log::info!("  - {}: {}", k, v);
            }
            
            let config = RegisterPathConfig {
                base_path,
                custom_paths,
            };
            
            Self::load_with_custom_paths(config)
        } else {
            // Use standard approach if no custom paths could be determined
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
        match &store_result {
            Ok(_) => {
                log::info!("Successfully created UnifiedStore");
            },
            Err(e) => {
                log::error!("Failed to create UnifiedStore: {}", e);
                
                // Additional diagnostics for specific error types
                if e.to_string().contains("No such file or directory") {
                    log::error!("The error appears to be related to missing files in the mappings directory");
                    
                    // Check the mappings directory
                    match std::env::current_dir() {
                        Ok(current_dir) => {
                            let mappings_dir = current_dir.join("mappings");
                            log::info!("Checking mappings directory at: {}", mappings_dir.display());
                            
                            if mappings_dir.exists() {
                                log::info!("Mappings directory exists, listing contents:");
                                
                                match std::fs::read_dir(&mappings_dir) {
                                    Ok(entries) => {
                                        for entry in entries.flatten() {
                                            log::debug!("Found mapping file: {}", entry.path().display());
                                        }
                                    },
                                    Err(e) => {
                                        log::error!("Failed to read mappings directory: {}", e);
                                    }
                                }
                            } else {
                                log::error!("Mappings directory does not exist at: {}", mappings_dir.display());
                            }
                        },
                        Err(e) => {
                            log::error!("Failed to get current directory: {}", e);
                        }
                    }
                }
            }
        }
        
        let mut store = store_result?;
        let mut progress = LoaderProgress::new();

        // Track missing data types to provide better error messages
        let mut _missing_data_types: Vec<String> = Vec::new();

        log::info!("Loading data from path: {}", base_path);
        progress.main_pb.set_message("Loading family relations...");

        log::debug!("Base path is {}", base_path);

        // Verify that the base path exists
        let base_path_obj = Path::new(&base_path);
        if !base_path_obj.exists() {
            let error_msg = format!("Base path does not exist: {}", base_path);
            log::error!("{}", error_msg);
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                error_msg,
            )));
        }

        // Verify custom paths
        log::debug!("Custom paths count: {}", config.custom_paths.len());
        let mut invalid_paths = Vec::new();

        for (k, v) in &config.custom_paths {
            let path = Path::new(v);
            let exists = path.exists();
            log::debug!("Custom path '{}' -> '{}', exists: {}", k, v, exists);

            if !exists {
                invalid_paths.push(format!("{} ({})", k, v));
            }
        }

        if !invalid_paths.is_empty() {
            log::warn!(
                "The following custom paths do not exist: {}",
                invalid_paths.join(", ")
            );
        }

        // Choose appropriate reader based on whether custom paths are provided
        if !config.custom_paths.is_empty() {
            log::info!("Using custom paths for register data");
            for (register, path) in &config.custom_paths {
                log::info!("Custom path for '{}': {}", register, path);
            }

            let custom_reader =
                reader::CustomPathReader::new(base_path.clone(), config.custom_paths.clone());

            // Try loading family relations directly first to see if that works
            log::info!("Trying to load family relations directly");
            match custom_reader.read_family() {
                Ok(batches) => {
                    log::info!(
                        "Successfully loaded family relations: {} batches",
                        batches.len()
                    );
                    if batches.is_empty() {
                        log::warn!("Family relations file exists but contains no data");
                    }
                }
                Err(e) => {
                    log::error!("Failed to load family relations: {}", e);
                    // Try to find any parquet files in the family path to help diagnose
                    if let Some(family_path) = config.custom_paths.get("family") {
                        let path_obj = Path::new(family_path);
                        if path_obj.exists() && path_obj.is_dir() {
                            match std::fs::read_dir(path_obj) {
                                Ok(entries) => {
                                    let files: Vec<_> = entries
                                        .filter_map(Result::ok)
                                        .filter(|e| e.path().extension().is_some_and(|ext| ext == "parquet"))
                                        .collect();
                                    
                                    if files.is_empty() {
                                        log::error!("No parquet files found in family directory: {}", family_path);
                                    } else {
                                        log::info!("Found these parquet files in family directory that could be used:");
                                        for file in files {
                                            log::info!("  - {}", file.path().display());
                                        }
                                    }
                                }
                                Err(e) => {
                                    log::error!("Failed to read family directory: {}", e);
                                }
                            }
                        }
                    }
                    // Continue - we'll handle the overall result below
                }
            }

            // Use the custom reader for all operations - but with more detailed error handling
            match Self::load_with_reader(&custom_reader, &mut store, &mut progress) {
                Ok(_) => {
                    log::info!("Successfully loaded data with custom reader");
                },
                Err(e) => {
                    log::error!("Failed to load data with custom reader: {}", e);
                    
                    // Add more detailed debugging for each register type
                    log::error!("--- Detailed diagnostic information ---");
                    
                    // Family file check
                    if let Some(path) = config.custom_paths.get("family") {
                        let file_path = Path::new(path);
                        if file_path.exists() {
                            if file_path.is_file() {
                                log::info!("Family file exists at {} (size: {} bytes)", 
                                          file_path.display(),
                                          std::fs::metadata(file_path)
                                             .map(|m| m.len())
                                             .unwrap_or(0));
                                
                                // Try to open and read a few bytes to confirm access
                                match std::fs::File::open(file_path) {
                                    Ok(mut f) => {
                                        let mut buffer = [0; 16];
                                        match std::io::Read::read(&mut f, &mut buffer) {
                                            Ok(n) => log::info!("Successfully read {} bytes from family file", n),
                                            Err(e) => log::error!("Failed to read from family file: {}", e),
                                        }
                                    },
                                    Err(e) => log::error!("Failed to open family file: {}", e),
                                }
                            } else {
                                log::error!("Family path exists but is not a file: {}", file_path.display());
                            }
                        } else {
                            log::error!("Family file does not exist at: {}", file_path.display());
                        }
                    }
                    
                    // Check read permissions on each directory
                    for register_type in ["akm", "bef", "ind", "uddf"] {
                        if let Some(path) = config.custom_paths.get(register_type) {
                            let dir_path = Path::new(path);
                            if dir_path.exists() {
                                if dir_path.is_dir() {
                                    log::info!("{} directory exists at {}", register_type, dir_path.display());
                                    
                                    // List a few files in the directory
                                    match std::fs::read_dir(dir_path) {
                                        Ok(entries) => {
                                            let files: Vec<_> = entries
                                                .filter_map(Result::ok)
                                                .filter(|e| e.path().extension().is_some_and(|ext| ext == "parquet"))
                                                .take(3)
                                                .collect();
                                                
                                            if !files.is_empty() {
                                                log::info!("Found {} parquet files in {} directory, including:", 
                                                         files.len(), register_type);
                                                
                                                for file in files {
                                                    let file_path = file.path();
                                                    let file_size = std::fs::metadata(&file_path)
                                                        .map(|m| m.len())
                                                        .unwrap_or(0);
                                                    
                                                    log::info!("  - {} (size: {} bytes)", 
                                                             file_path.display(), file_size);
                                                    
                                                    // Try to open and read a few bytes to confirm access
                                                    match std::fs::File::open(&file_path) {
                                                        Ok(mut f) => {
                                                            let mut buffer = [0; 16];
                                                            match std::io::Read::read(&mut f, &mut buffer) {
                                                                Ok(n) => log::info!("    Successfully read {} bytes", n),
                                                                Err(e) => log::error!("    Failed to read from file: {}", e),
                                                            }
                                                        },
                                                        Err(e) => log::error!("    Failed to open file: {}", e),
                                                    }
                                                }
                                            } else {
                                                log::error!("No parquet files found in {} directory", register_type);
                                            }
                                        },
                                        Err(e) => log::error!("Failed to read {} directory: {}", register_type, e),
                                    }
                                } else {
                                    log::error!("{} path exists but is not a directory: {}", 
                                              register_type, dir_path.display());
                                }
                            } else {
                                log::error!("{} directory does not exist at: {}", 
                                          register_type, dir_path.display());
                            }
                        }
                    }
                    
                    log::error!("---- End of diagnostic information ----");
                    return Err(e);
                }
            }
        } else {
            log::info!("Using default reader with standard paths");
            // Use the default reader
            let reader = reader::FileReader::new(base_path.clone());

            // Check for the family file explicitly as it's critical
            let family_path = base_path_obj.join("family.parquet");
            if !family_path.exists() {
                log::warn!("Critical file not found: {}", family_path.display());
            }

            // Use the standard reader for all operations
            if let Err(e) = Self::load_with_reader(&reader, &mut store, &mut progress) {
                log::error!("Failed to load data with default reader: {}", e);
                return Err(e);
            }
        }

        // // Get the data summary to verify what was actually loaded
        // log::info!(
        //     "Data loading complete - family relations: {} entries",
        //     match &store.as_arrow_backend() {
        //         Some(backend) => backend.family_relations.len().to_string(),
        //         None => "0".to_string(),
        //     }
        // );

        // In the updated version, we just take the ArrowBackend via as_arrow_backend
        match store.as_arrow_backend() {
            Some(backend) => {
                log::info!("Successfully created Arrow backend");
                Ok(backend.clone())
            }
            None => {
                let error_msg =
                    "Failed to access arrow backend - store may not have loaded properly";
                log::error!("{}", error_msg);
                Err(IdsError::invalid_operation(error_msg))
            }
        }
    }
}

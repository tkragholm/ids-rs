use std::path::{Path, PathBuf};
use types::{
    error::IdsError,
    models::CovariateType,
    storage::ArrowBackend as ArrowStore,
    store::polars_backend::PolarsBackend,
};
use polars::prelude::*;
use log::{info, warn};
use arrow::record_batch::RecordBatch;
use std::fs;
use crate::reader::LoaderProgress;
use crate::StoreLoader;
use crate::RegisterPathConfig;

/// PolarsLoader can load data from various sources directly into PolarsBackend
pub struct PolarsLoader;

impl PolarsLoader {
    pub fn new() -> Self {
        Self
    }
    
    /// Directly load AKM data from parquet files into Polars LazyFrames
    pub fn load_akm(&self, base_path: &str, years: &[i32]) -> Result<HashMap<i32, LazyFrame>, IdsError> {
        let mut result = HashMap::new();
        let mut progress = LoaderProgress::new();
        
        for &year in years {
            let path = format!("{}/akm{}.parquet", base_path, year);
            
            // Skip years where file doesn't exist
            if !Path::new(&path).exists() {
                continue;
            }
            
            info!("Loading AKM data for year {}", year);
            progress.start_with_spinner(format!("Loading AKM data for year {}", year));
            
            // Use Polars to read parquet directly
            let scan_args = ScanArgsParquet {
                parallel: true,
                low_memory: false,
                ..Default::default()
            };
            
            match LazyFrame::scan_parquet(path, scan_args) {
                Ok(lf) => {
                    result.insert(year, lf);
                    progress.finish_with_message(format!("Loaded AKM data for year {}", year));
                },
                Err(e) => {
                    progress.finish_with_error(format!("Failed to load AKM data for year {}: {}", year, e));
                    return Err(IdsError::invalid_operation(format!("Failed to load AKM data for year {}: {}", year, e)));
                }
            }
        }
        
        Ok(result)
    }
    
    /// Directly load BEF data from parquet files into Polars LazyFrames
    pub fn load_bef(&self, base_path: &str, periods: &[String]) -> Result<HashMap<String, LazyFrame>, IdsError> {
        let mut result = HashMap::new();
        let mut progress = LoaderProgress::new();
        
        for period in periods {
            let path = format!("{}/bef{}.parquet", base_path, period);
            
            // Skip periods where file doesn't exist
            if !Path::new(&path).exists() {
                continue;
            }
            
            info!("Loading BEF data for period {}", period);
            progress.start_with_spinner(format!("Loading BEF data for period {}", period));
            
            // Use Polars to read parquet directly with optimizations
            let scan_args = ScanArgsParquet {
                parallel: true,
                low_memory: false,
                ..Default::default()
            };
            
            match LazyFrame::scan_parquet(path, scan_args) {
                Ok(lf) => {
                    result.insert(period.clone(), lf);
                    progress.finish_with_message(format!("Loaded BEF data for period {}", period));
                },
                Err(e) => {
                    progress.finish_with_error(format!("Failed to load BEF data for period {}: {}", period, e));
                    return Err(IdsError::invalid_operation(format!("Failed to load BEF data for period {}: {}", period, e)));
                }
            }
        }
        
        Ok(result)
    }
    
    /// Directly load IND data from parquet files into Polars LazyFrames
    pub fn load_ind(&self, base_path: &str, years: &[i32]) -> Result<HashMap<i32, LazyFrame>, IdsError> {
        let mut result = HashMap::new();
        let mut progress = LoaderProgress::new();
        
        for &year in years {
            let path = format!("{}/ind{}.parquet", base_path, year);
            
            // Skip years where file doesn't exist
            if !Path::new(&path).exists() {
                continue;
            }
            
            info!("Loading IND data for year {}", year);
            progress.start_with_spinner(format!("Loading IND data for year {}", year));
            
            // Use Polars to read parquet directly
            let scan_args = ScanArgsParquet {
                parallel: true,
                low_memory: false,
                ..Default::default()
            };
            
            match LazyFrame::scan_parquet(path, scan_args) {
                Ok(lf) => {
                    result.insert(year, lf);
                    progress.finish_with_message(format!("Loaded IND data for year {}", year));
                },
                Err(e) => {
                    progress.finish_with_error(format!("Failed to load IND data for year {}: {}", year, e));
                    return Err(IdsError::invalid_operation(format!("Failed to load IND data for year {}: {}", year, e)));
                }
            }
        }
        
        Ok(result)
    }
    
    /// Directly load UDDF data from parquet files into Polars LazyFrames
    pub fn load_uddf(&self, base_path: &str, periods: &[String]) -> Result<HashMap<String, LazyFrame>, IdsError> {
        let mut result = HashMap::new();
        let mut progress = LoaderProgress::new();
        
        for period in periods {
            let path = format!("{}/uddf{}.parquet", base_path, period);
            
            // Skip periods where file doesn't exist
            if !Path::new(&path).exists() {
                continue;
            }
            
            info!("Loading UDDF data for period {}", period);
            progress.start_with_spinner(format!("Loading UDDF data for period {}", period));
            
            // Use Polars to read parquet directly
            let scan_args = ScanArgsParquet {
                parallel: true,
                low_memory: false,
                ..Default::default()
            };
            
            match LazyFrame::scan_parquet(path, scan_args) {
                Ok(lf) => {
                    result.insert(period.clone(), lf);
                    progress.finish_with_message(format!("Loaded UDDF data for period {}", period));
                },
                Err(e) => {
                    progress.finish_with_error(format!("Failed to load UDDF data for period {}: {}", period, e));
                    return Err(IdsError::invalid_operation(format!("Failed to load UDDF data for period {}: {}", period, e)));
                }
            }
        }
        
        Ok(result)
    }
    
    /// Load family relations from parquet file directly into Polars
    pub fn load_family_relations(&self, base_path: &str) -> Result<DataFrame, IdsError> {
        let path = format!("{}/families.parquet", base_path);
        let mut progress = LoaderProgress::new();
        
        if !Path::new(&path).exists() {
            return Err(IdsError::invalid_operation(format!("Family relations file not found at {}", path)));
        }
        
        info!("Loading family relations data");
        progress.start_with_spinner("Loading family relations data");
        
        // Use Polars to read parquet directly and materialize immediately
        let scan_args = ScanArgsParquet {
            parallel: true,
            low_memory: false,
            ..Default::default()
        };
        
        match LazyFrame::scan_parquet(path, scan_args) {
            Ok(lf) => {
                match lf.collect() {
                    Ok(df) => {
                        progress.finish_with_message("Loaded family relations data");
                        Ok(df)
                    },
                    Err(e) => {
                        progress.finish_with_error(format!("Failed to collect family relations data: {}", e));
                        Err(IdsError::invalid_operation(format!("Failed to collect family relations data: {}", e)))
                    }
                }
            },
            Err(e) => {
                progress.finish_with_error(format!("Failed to load family relations data: {}", e));
                Err(IdsError::invalid_operation(format!("Failed to load family relations data: {}", e)))
            }
        }
    }
    
    /// Find available data files in the base_path
    pub fn discover_available_data(&self, base_path: &str) -> Result<(Vec<i32>, Vec<String>, Vec<i32>, Vec<String>), IdsError> {
        let base_path = PathBuf::from(base_path);
        let mut akm_years = Vec::new();
        let mut bef_periods = Vec::new();
        let mut ind_years = Vec::new();
        let mut uddf_periods = Vec::new();
        
        info!("Scanning directory for parquet files: {}", base_path.display());
        
        // List all files in the directory
        let entries = fs::read_dir(&base_path).map_err(|e| {
            IdsError::invalid_operation(format!("Could not read directory {}: {}", base_path.display(), e))
        })?;
        
        for entry in entries {
            let entry = entry.map_err(|e| {
                IdsError::invalid_operation(format!("Could not read directory entry: {}", e))
            })?;
            
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext != "parquet" {
                    continue;
                }
                
                if let Some(file_name) = path.file_name() {
                    let file_name = file_name.to_string_lossy();
                    
                    // Check for AKM data
                    if file_name.starts_with("akm") && file_name.ends_with(".parquet") {
                        if let Some(year_str) = file_name.strip_prefix("akm").and_then(|s| s.strip_suffix(".parquet")) {
                            if let Ok(year) = year_str.parse::<i32>() {
                                akm_years.push(year);
                            }
                        }
                    }
                    
                    // Check for BEF data
                    if file_name.starts_with("bef") && file_name.ends_with(".parquet") {
                        if let Some(period) = file_name.strip_prefix("bef").and_then(|s| s.strip_suffix(".parquet")) {
                            bef_periods.push(period.to_string());
                        }
                    }
                    
                    // Check for IND data
                    if file_name.starts_with("ind") && file_name.ends_with(".parquet") {
                        if let Some(year_str) = file_name.strip_prefix("ind").and_then(|s| s.strip_suffix(".parquet")) {
                            if let Ok(year) = year_str.parse::<i32>() {
                                ind_years.push(year);
                            }
                        }
                    }
                    
                    // Check for UDDF data
                    if file_name.starts_with("uddf") && file_name.ends_with(".parquet") {
                        if let Some(period) = file_name.strip_prefix("uddf").and_then(|s| s.strip_suffix(".parquet")) {
                            uddf_periods.push(period.to_string());
                        }
                    }
                }
            }
        }
        
        // Sort the discovered data
        akm_years.sort();
        bef_periods.sort();
        ind_years.sort();
        uddf_periods.sort();
        
        info!("Found {} AKM years, {} BEF periods, {} IND years, {} UDDF periods", 
              akm_years.len(), bef_periods.len(), ind_years.len(), uddf_periods.len());
        
        Ok((akm_years, bef_periods, ind_years, uddf_periods))
    }
    
    /// Load data directly from a directory containing parquet files
    pub fn load_data_dir(&self, base_path: &str) -> Result<PolarsBackend, IdsError> {
        let mut polars_backend = PolarsBackend::new()?;
        let mut progress = LoaderProgress::new();
        
        info!("Scanning directory for data files: {}", base_path);
        
        // Discover available data files
        let (akm_years, bef_periods, ind_years, uddf_periods) = 
            self.discover_available_data(base_path)?;
        
        // Load family relations
        let family_path = format!("{}/family.parquet", base_path);
        if Path::new(&family_path).exists() {
            info!("Loading family relations from {}", family_path);
            progress.start_sub_progress(1, "Family relations".to_string());
            
            match polars_backend.load_family_file(&family_path) {
                Ok(_) => {
                    info!("Successfully loaded family relations");
                    progress.increment_main();
                },
                Err(e) => {
                    warn!("Failed to load family relations: {}", e);
                    progress.increment_main();
                }
            }
        } else {
            warn!("Family relations file not found at {}", family_path);
        }
        
        // Load AKM data
        if !akm_years.is_empty() {
            info!("Loading AKM data for {} years", akm_years.len());
            progress.start_sub_progress(akm_years.len() as u64, "AKM years".to_string());
            
            for &year in &akm_years {
                let file_path = format!("{}/akm{}.parquet", base_path, year);
                match polars_backend.load_akm_file(year, &file_path) {
                    Ok(_) => {
                        info!("Loaded AKM data for year {}", year);
                        progress.increment_sub();
                    },
                    Err(e) => {
                        warn!("Failed to load AKM data for year {}: {}", year, e);
                        progress.increment_sub();
                    }
                }
            }
            
            progress.increment_main();
        }
        
        // Load BEF data
        if !bef_periods.is_empty() {
            info!("Loading BEF data for {} periods", bef_periods.len());
            progress.start_sub_progress(bef_periods.len() as u64, "BEF periods".to_string());
            
            for period in &bef_periods {
                let file_path = format!("{}/bef{}.parquet", base_path, period);
                match polars_backend.load_bef_file(period, &file_path) {
                    Ok(_) => {
                        info!("Loaded BEF data for period {}", period);
                        progress.increment_sub();
                    },
                    Err(e) => {
                        warn!("Failed to load BEF data for period {}: {}", period, e);
                        progress.increment_sub();
                    }
                }
            }
            
            progress.increment_main();
        }
        
        // Load IND data
        if !ind_years.is_empty() {
            info!("Loading IND data for {} years", ind_years.len());
            progress.start_sub_progress(ind_years.len() as u64, "IND years".to_string());
            
            for &year in &ind_years {
                let file_path = format!("{}/ind{}.parquet", base_path, year);
                match polars_backend.load_ind_file(year, &file_path) {
                    Ok(_) => {
                        info!("Loaded IND data for year {}", year);
                        progress.increment_sub();
                    },
                    Err(e) => {
                        warn!("Failed to load IND data for year {}: {}", year, e);
                        progress.increment_sub();
                    }
                }
            }
            
            progress.increment_main();
        }
        
        // Load UDDF data
        if !uddf_periods.is_empty() {
            info!("Loading UDDF data for {} periods", uddf_periods.len());
            progress.start_sub_progress(uddf_periods.len() as u64, "UDDF periods".to_string());
            
            for period in &uddf_periods {
                let file_path = format!("{}/uddf{}.parquet", base_path, period);
                match polars_backend.load_uddf_file(period, &file_path) {
                    Ok(_) => {
                        info!("Loaded UDDF data for period {}", period);
                        progress.increment_sub();
                    },
                    Err(e) => {
                        warn!("Failed to load UDDF data for period {}: {}", period, e);
                        progress.increment_sub();
                    }
                }
            }
            
            progress.increment_main();
        }
        
        progress.finish_main("Completed loading data with Polars");
        Ok(polars_backend)
    }
}

/// Implement StoreLoader for PolarsLoader to maintain compatibility with existing code
impl StoreLoader for PolarsLoader {
    fn load_from_path(base_path: String) -> Result<ArrowStore, IdsError> {
        // For now, we still need to return ArrowStore for compatibility
        // In the future, we would update this to return PolarsBackend directly
        let loader = PolarsLoader::new();
        info!("Using Polars loader with direct parquet scanning");
        
        // Until we update the interfaces, we'll create a minimal ArrowStore
        let arrow_store = ArrowStore::new()?;
        
        Ok(arrow_store)
    }

    fn load_with_custom_paths(config: RegisterPathConfig) -> Result<ArrowStore, IdsError> {
        // For custom paths, we would need to update the interfaces to use PolarsBackend
        let arrow_store = ArrowStore::new()?;
        
        Ok(arrow_store)
    }
}
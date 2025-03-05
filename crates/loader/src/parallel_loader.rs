/// This file implements the Phase 1 optimizations for parallel loading of register files
/// It focuses on:
/// 1. Removing thread constraints
/// 2. Increasing batch sizes
/// 3. Implementing early PNR filtering
/// 4. Loading files in parallel

use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::fs;

use crate::{
    ArrowStore,
    IdsError,
    LoaderProgress,
    parquet::{read_parquet, load_parquet_files_parallel}
};

/// Parallel Register Loader implementation
pub struct ParallelLoader;

impl ParallelLoader {
    pub fn new() -> Self {
        Self
    }
    
    /// Load register parquet files in parallel with optional PNR filtering
    /// 
    /// This implements the Phase 1 optimization strategy:
    /// 1. Uses all available CPU cores for loading (controlled by IDS_MAX_THREADS)
    /// 2. Uses larger batch sizes (controlled by IDS_BATCH_SIZE)
    /// 3. Filters by PNR at load time to reduce memory usage
    /// 4. Loads all register files in parallel
    pub fn load_registers_parallel(
        base_path: &str,
        pnr_filter: Option<&HashSet<String>>,
    ) -> Result<ArrowStore, IdsError> {
        log::info!("Starting parallel register loading with Phase 1 optimizations");
        let base_dir = PathBuf::from(base_path);
        
        // Discover parquet files in base_path
        let mut akm_files = Vec::new();
        let mut bef_files = Vec::new();
        let mut ind_files = Vec::new();
        let mut uddf_files = Vec::new();
        let mut family_file = None;
        
        // Scan for parquet files
        if let Ok(entries) = fs::read_dir(&base_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.extension().map_or(false, |ext| ext == "parquet") {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    
                    if filename.starts_with("akm") {
                        akm_files.push(path.clone());
                    } else if filename.starts_with("bef") {
                        bef_files.push(path.clone());
                    } else if filename.starts_with("ind") {
                        ind_files.push(path.clone());
                    } else if filename.starts_with("uddf") {
                        uddf_files.push(path.clone());
                    } else if filename == "family.parquet" || filename == "families.parquet" {
                        family_file = Some(path.clone());
                    }
                }
            }
        }
        
        log::info!("Found {} AKM files, {} BEF files, {} IND files, {} UDDF files, family file: {}",
                  akm_files.len(), bef_files.len(), ind_files.len(), uddf_files.len(),
                  family_file.is_some());
        
        // Create a progress tracker
        let progress = LoaderProgress::new();
        
        // Create a new store
        let mut store = ArrowStore::new()?;
        
        // Load family file if it exists
        if let Some(family_path) = family_file {
            log::info!("Loading family file...");
            let family_batches = read_parquet(&family_path, None, Some(&progress), pnr_filter)?;
            store.load_family_relations(&family_batches)?;
        }
        
        // Load all the register files in parallel by type
        let process_register_files = |files: &[PathBuf], register_type: &str| -> Result<HashMap<String, Vec<arrow::record_batch::RecordBatch>>, IdsError> {
            if files.is_empty() {
                return Ok(HashMap::new());
            }
            
            log::info!("Loading {} {} register files in parallel", files.len(), register_type);
            load_parquet_files_parallel(files, None, Some(&progress), pnr_filter)
        };
        
        // Process each register type in sequence (but with internal parallelism)
        if !akm_files.is_empty() {
            let akm_batches = process_register_files(&akm_files, "AKM")?;
            for (filename, batches) in akm_batches {
                if let Some(year_str) = filename.strip_prefix("akm").and_then(|s| s.strip_suffix(".parquet")) {
                    if let Ok(year) = year_str.parse::<i32>() {
                        log::info!("Adding AKM data for year {}", year);
                        store.load_akm_data(year, &batches)?;
                    }
                }
            }
        }
        
        if !bef_files.is_empty() {
            let bef_batches = process_register_files(&bef_files, "BEF")?;
            for (filename, batches) in bef_batches {
                if let Some(period) = filename.strip_prefix("bef").and_then(|s| s.strip_suffix(".parquet")) {
                    log::info!("Adding BEF data for period {}", period);
                    store.load_bef_data(period, &batches)?;
                }
            }
        }
        
        if !ind_files.is_empty() {
            let ind_batches = process_register_files(&ind_files, "IND")?;
            for (filename, batches) in ind_batches {
                if let Some(year_str) = filename.strip_prefix("ind").and_then(|s| s.strip_suffix(".parquet")) {
                    if let Ok(year) = year_str.parse::<i32>() {
                        log::info!("Adding IND data for year {}", year);
                        store.load_ind_data(year, &batches)?;
                    }
                }
            }
        }
        
        if !uddf_files.is_empty() {
            let uddf_batches = process_register_files(&uddf_files, "UDDF")?;
            for (filename, batches) in uddf_batches {
                if let Some(period) = filename.strip_prefix("uddf").and_then(|s| s.strip_suffix(".parquet")) {
                    log::info!("Adding UDDF data for period {}", period);
                    store.load_uddf_data(period, &batches)?;
                }
            }
        }
        
        log::info!("Parallel register loading complete!");
        Ok(store)
    }
    
    /// Load from a PNR list file
    pub fn load_with_pnr_filter_file(
        base_path: &str,
        pnr_filter_file: &str
    ) -> Result<ArrowStore, IdsError> {
        // Read PNR filter file
        log::info!("Reading PNR filter file: {}", pnr_filter_file);
        let pnr_content = fs::read_to_string(pnr_filter_file)
            .map_err(|e| IdsError::invalid_operation(format!("Failed to read PNR filter file: {}", e)))?;
        
        // Parse PNR list (one per line)
        let pnr_set: HashSet<String> = pnr_content
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect();
        
        log::info!("Loaded {} PNRs from filter file", pnr_set.len());
        
        // Load with the PNR filter
        Self::load_registers_parallel(base_path, Some(&pnr_set))
    }
    
    /// Extract PNRs from family data and create a filter set
    pub fn extract_pnrs_from_family_batches(
        family_batches: &[arrow::record_batch::RecordBatch]
    ) -> Result<HashSet<String>, IdsError> {
        use arrow::array::StringArray;
        let utils = ArrowUtils;
        let mut pnr_set = HashSet::new();
        
        log::info!("Extracting PNRs from family data ({} batches)", family_batches.len());
        
        // Process each batch
        for batch in family_batches {
            // Try different field names for child PNR
            let child_idx = batch.schema().index_of("PNR")
                .or_else(|_| batch.schema().index_of("pnr"))
                .or_else(|_| batch.schema().index_of("child_pnr"))
                .or_else(|_| batch.schema().index_of("child_id"))
                .map_err(|_| IdsError::invalid_operation("Could not find child PNR column in family data".to_string()))?;
            
            // Try to find parent columns
            let mother_idx = batch.schema().index_of("mother_pnr")
                .or_else(|_| batch.schema().index_of("mother_id"))
                .or_else(|_| batch.schema().index_of("mor_pnr"))
                .ok();
                
            let father_idx = batch.schema().index_of("father_pnr")
                .or_else(|_| batch.schema().index_of("father_id"))
                .or_else(|_| batch.schema().index_of("far_pnr"))
                .ok();
            
            // Extract child PNRs
            if let Some(child_array) = batch.column(child_idx).as_any().downcast_ref::<StringArray>() {
                for i in 0..child_array.len() {
                    if let Some(pnr) = child_array.value(i).to_string().into() {
                        pnr_set.insert(pnr);
                    }
                }
            }
            
            // Extract mother PNRs if available
            if let Some(idx) = mother_idx {
                if let Some(mother_array) = batch.column(idx).as_any().downcast_ref::<StringArray>() {
                    for i in 0..mother_array.len() {
                        if !mother_array.is_null(i) {
                            if let Some(pnr) = mother_array.value(i).to_string().into() {
                                if !pnr.is_empty() {
                                    pnr_set.insert(pnr);
                                }
                            }
                        }
                    }
                }
            }
            
            // Extract father PNRs if available
            if let Some(idx) = father_idx {
                if let Some(father_array) = batch.column(idx).as_any().downcast_ref::<StringArray>() {
                    for i in 0..father_array.len() {
                        if !father_array.is_null(i) {
                            if let Some(pnr) = father_array.value(i).to_string().into() {
                                if !pnr.is_empty() {
                                    pnr_set.insert(pnr);
                                }
                            }
                        }
                    }
                }
            }
        }
        
        log::info!("Extracted {} unique PNRs from family data", pnr_set.len());
        Ok(pnr_set)
    }
    
    /// Load registers using family relations to filter data
    /// This loads only the family data first, extracts all relevant PNRs,
    /// and then loads other registers with PNR filtering
    pub fn load_with_family_based_filtering(base_path: &str) -> Result<ArrowStore, IdsError> {
        use std::path::PathBuf;
        
        log::info!("Loading with family-based PNR filtering optimization");
        
        // First locate the family file
        let base_dir = PathBuf::from(base_path);
        let mut family_path = None;
        
        // Look for common family file names
        for name in &["family.parquet", "families.parquet", "family_relations.parquet"] {
            let test_path = base_dir.join(name);
            if test_path.exists() && test_path.is_file() {
                family_path = Some(test_path);
                break;
            }
        }
        
        let family_path = family_path.ok_or_else(|| 
            IdsError::invalid_operation(format!("Could not find family file in {}", base_path)))?;
        
        log::info!("Found family file at {}", family_path.display());
        
        // Load family file first
        let progress = LoaderProgress::new();
        let family_batches = read_parquet(&family_path, None, Some(&progress), None)?;
        
        // Extract PNRs from family data
        let pnr_set = Self::extract_pnrs_from_family_batches(&family_batches)?;
        
        // Create store and load family relations
        let mut store = ArrowStore::new()?;
        store.load_family_relations(&family_batches)?;
        
        // Now load the rest of the register files with PNR filtering
        let mut akm_files = Vec::new();
        let mut bef_files = Vec::new();
        let mut ind_files = Vec::new();
        let mut uddf_files = Vec::new();
        
        // Scan for the rest of parquet files
        if let Ok(entries) = fs::read_dir(&base_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path == family_path {
                    continue; // Skip the family file we already loaded
                }
                
                if path.extension().map_or(false, |ext| ext == "parquet") {
                    let filename = path.file_name().unwrap().to_string_lossy().to_string();
                    
                    if filename.starts_with("akm") {
                        akm_files.push(path.clone());
                    } else if filename.starts_with("bef") {
                        bef_files.push(path.clone());
                    } else if filename.starts_with("ind") {
                        ind_files.push(path.clone());
                    } else if filename.starts_with("uddf") {
                        uddf_files.push(path.clone());
                    }
                }
            }
        }
        
        log::info!("Using family-based filtering with {} PNRs for {} AKM files, {} BEF files, {} IND files, {} UDDF files",
                  pnr_set.len(), akm_files.len(), bef_files.len(), ind_files.len(), uddf_files.len());
        
        // Process register files with PNR filtering
        let process_register_files = |files: &[PathBuf], register_type: &str| -> Result<HashMap<String, Vec<arrow::record_batch::RecordBatch>>, IdsError> {
            if files.is_empty() {
                return Ok(HashMap::new());
            }
            
            log::info!("Loading {} {} register files with family-based PNR filtering", files.len(), register_type);
            load_parquet_files_parallel(files, None, Some(&progress), Some(&pnr_set))
        };
        
        // Process each register type with PNR filtering
        if !akm_files.is_empty() {
            let akm_batches = process_register_files(&akm_files, "AKM")?;
            for (filename, batches) in akm_batches {
                if let Some(year_str) = filename.strip_prefix("akm").and_then(|s| s.strip_suffix(".parquet")) {
                    if let Ok(year) = year_str.parse::<i32>() {
                        log::info!("Adding AKM data for year {}", year);
                        store.load_akm_data(year, &batches)?;
                    }
                }
            }
        }
        
        // Apply same pattern for other register types
        if !bef_files.is_empty() {
            let bef_batches = process_register_files(&bef_files, "BEF")?;
            for (filename, batches) in bef_batches {
                if let Some(period) = filename.strip_prefix("bef").and_then(|s| s.strip_suffix(".parquet")) {
                    log::info!("Adding BEF data for period {}", period);
                    store.load_bef_data(period, &batches)?;
                }
            }
        }
        
        if !ind_files.is_empty() {
            let ind_batches = process_register_files(&ind_files, "IND")?;
            for (filename, batches) in ind_batches {
                if let Some(year_str) = filename.strip_prefix("ind").and_then(|s| s.strip_suffix(".parquet")) {
                    if let Ok(year) = year_str.parse::<i32>() {
                        log::info!("Adding IND data for year {}", year);
                        store.load_ind_data(year, &batches)?;
                    }
                }
            }
        }
        
        if !uddf_files.is_empty() {
            let uddf_batches = process_register_files(&uddf_files, "UDDF")?;
            for (filename, batches) in uddf_batches {
                if let Some(period) = filename.strip_prefix("uddf").and_then(|s| s.strip_suffix(".parquet")) {
                    log::info!("Adding UDDF data for period {}", period);
                    store.load_uddf_data(period, &batches)?;
                }
            }
        }
        
        log::info!("Family-based PNR filtering loading complete!");
        Ok(store)
    }
}
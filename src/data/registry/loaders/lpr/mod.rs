//! LPR (Landspatientregistret) registry loaders
//!
//! This module contains registry loaders for the LPR (Landspatientregistret) registry.

use crate::data::registry::traits::RegisterLoader;
use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::{Path, PathBuf};
use std::fs;
use std::sync::Arc;
use uuid::Uuid;
use datafusion::datasource::MemTable;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::Mutex;

pub mod lpr2_loader;
pub mod lpr3_loader;
pub mod lpr_provider;

pub use lpr2_loader::Lpr2Register;
pub use lpr3_loader::Lpr3Register;
pub use lpr_provider::LprTableProvider;

use crate::utils::logging::Component;
use crate::{debug_log, info_log, warn_log, error_log, log_function_entry, log_function_exit, log_step, traced_operation};

/// Container for all LPR data components
#[derive(Debug, Default)]
pub struct LprComponents {
    /// LPR2 admin data
    pub lpr2_adm: Option<Vec<RecordBatch>>,
    /// LPR2 diagnosis data
    pub lpr2_diag: Option<Vec<RecordBatch>>,
    /// LPR2 procedure data
    pub lpr2_bes: Option<Vec<RecordBatch>>,
    /// LPR3 contacts data
    pub lpr3_kontakter: Option<Vec<RecordBatch>>,
    /// LPR3 diagnoses data
    pub lpr3_diagnoser: Option<Vec<RecordBatch>>,
    /// LPR3 procedures data
    pub lpr3_procedurer: Option<Vec<RecordBatch>>,
}

impl LprComponents {
    /// Create a new empty LPR components container
    pub fn new() -> Self {
        Self::default()
    }

    /// Add LPR2 admin data
    pub fn with_lpr2_adm(mut self, data: Vec<RecordBatch>) -> Self {
        self.lpr2_adm = Some(data);
        self
    }

    /// Add LPR2 diagnosis data
    pub fn with_lpr2_diag(mut self, data: Vec<RecordBatch>) -> Self {
        self.lpr2_diag = Some(data);
        self
    }

    /// Add LPR2 procedure data
    pub fn with_lpr2_bes(mut self, data: Vec<RecordBatch>) -> Self {
        self.lpr2_bes = Some(data);
        self
    }

    /// Add LPR3 contacts data
    pub fn with_lpr3_kontakter(mut self, data: Vec<RecordBatch>) -> Self {
        self.lpr3_kontakter = Some(data);
        self
    }

    /// Add LPR3 diagnoses data
    pub fn with_lpr3_diagnoser(mut self, data: Vec<RecordBatch>) -> Self {
        self.lpr3_diagnoser = Some(data);
        self
    }

    /// Add LPR3 procedures data
    pub fn with_lpr3_procedurer(mut self, data: Vec<RecordBatch>) -> Self {
        self.lpr3_procedurer = Some(data);
        self
    }
}

/// Global cache for LPR data to avoid multiple loadings of the same data
static LPR_CACHE: Lazy<Mutex<LprDataCache>> = Lazy::new(|| Mutex::new(LprDataCache::new()));

/// Cache for LPR data
#[derive(Debug, Default)]
pub struct LprDataCache {
    /// Cache for LPR2 admin data
    lpr2_admin_cache: HashMap<String, Vec<RecordBatch>>,
    /// Cache for LPR2 diag data
    lpr2_diag_cache: HashMap<String, Vec<RecordBatch>>,
    /// Cache for LPR2 bes/proc data
    lpr2_bes_cache: HashMap<String, Vec<RecordBatch>>,
    /// Cache for LPR3 kontakter data
    lpr3_kontakter_cache: HashMap<String, Vec<RecordBatch>>,
    /// Cache for LPR3 diagnoser data
    lpr3_diagnoser_cache: HashMap<String, Vec<RecordBatch>>,
    /// Cache for LPR3 procedurer data
    lpr3_procedurer_cache: HashMap<String, Vec<RecordBatch>>,
}

impl LprDataCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self::default()
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.lpr2_admin_cache.clear();
        self.lpr2_diag_cache.clear();
        self.lpr2_bes_cache.clear();
        self.lpr3_kontakter_cache.clear();
        self.lpr3_diagnoser_cache.clear();
        self.lpr3_procedurer_cache.clear();
    }

    /// Get normalized path string (for use as cache key)
    fn normalize_path(path: &Path) -> String {
        match path.canonicalize() {
            Ok(p) => p.to_string_lossy().to_string(),
            Err(_) => path.to_string_lossy().to_string(),
        }
    }

    /// Store LPR2 admin data in the cache
    pub fn store_lpr2_admin(&mut self, path: &Path, data: Vec<RecordBatch>) {
        let key = Self::normalize_path(path);
        info_log!(Component::Lpr, "cache", "Storing LPR2 admin data in cache for path: {}", key);
        self.lpr2_admin_cache.insert(key, data);
    }

    /// Get LPR2 admin data from the cache
    pub fn get_lpr2_admin(&self, path: &Path) -> Option<Vec<RecordBatch>> {
        let key = Self::normalize_path(path);
        let result = self.lpr2_admin_cache.get(&key).cloned();
        if result.is_some() {
            info_log!(Component::Lpr, "cache", "Retrieved LPR2 admin data from cache for path: {}", key);
        }
        result
    }

    /// Store LPR2 diag data in the cache
    pub fn store_lpr2_diag(&mut self, path: &Path, data: Vec<RecordBatch>) {
        let key = Self::normalize_path(path);
        info_log!(Component::Lpr, "cache", "Storing LPR2 diag data in cache for path: {}", key);
        self.lpr2_diag_cache.insert(key, data);
    }

    /// Get LPR2 diag data from the cache
    pub fn get_lpr2_diag(&self, path: &Path) -> Option<Vec<RecordBatch>> {
        let key = Self::normalize_path(path);
        let result = self.lpr2_diag_cache.get(&key).cloned();
        if result.is_some() {
            info_log!(Component::Lpr, "cache", "Retrieved LPR2 diag data from cache for path: {}", key);
        }
        result
    }

    /// Store LPR2 bes data in the cache
    pub fn store_lpr2_bes(&mut self, path: &Path, data: Vec<RecordBatch>) {
        let key = Self::normalize_path(path);
        info_log!(Component::Lpr, "cache", "Storing LPR2 bes data in cache for path: {}", key);
        self.lpr2_bes_cache.insert(key, data);
    }

    /// Get LPR2 bes data from the cache
    pub fn get_lpr2_bes(&self, path: &Path) -> Option<Vec<RecordBatch>> {
        let key = Self::normalize_path(path);
        let result = self.lpr2_bes_cache.get(&key).cloned();
        if result.is_some() {
            info_log!(Component::Lpr, "cache", "Retrieved LPR2 bes data from cache for path: {}", key);
        }
        result
    }

    /// Store LPR3 kontakter data in the cache
    pub fn store_lpr3_kontakter(&mut self, path: &Path, data: Vec<RecordBatch>) {
        let key = Self::normalize_path(path);
        info_log!(Component::Lpr, "cache", "Storing LPR3 kontakter data in cache for path: {}", key);
        self.lpr3_kontakter_cache.insert(key, data);
    }

    /// Get LPR3 kontakter data from the cache
    pub fn get_lpr3_kontakter(&self, path: &Path) -> Option<Vec<RecordBatch>> {
        let key = Self::normalize_path(path);
        let result = self.lpr3_kontakter_cache.get(&key).cloned();
        if result.is_some() {
            info_log!(Component::Lpr, "cache", "Retrieved LPR3 kontakter data from cache for path: {}", key);
        }
        result
    }

    /// Store LPR3 diagnoser data in the cache
    pub fn store_lpr3_diagnoser(&mut self, path: &Path, data: Vec<RecordBatch>) {
        let key = Self::normalize_path(path);
        info_log!(Component::Lpr, "cache", "Storing LPR3 diagnoser data in cache for path: {}", key);
        self.lpr3_diagnoser_cache.insert(key, data);
    }

    /// Get LPR3 diagnoser data from the cache
    pub fn get_lpr3_diagnoser(&self, path: &Path) -> Option<Vec<RecordBatch>> {
        let key = Self::normalize_path(path);
        let result = self.lpr3_diagnoser_cache.get(&key).cloned();
        if result.is_some() {
            info_log!(Component::Lpr, "cache", "Retrieved LPR3 diagnoser data from cache for path: {}", key);
        }
        result
    }

    /// Store LPR3 procedurer data in the cache
    pub fn store_lpr3_procedurer(&mut self, path: &Path, data: Vec<RecordBatch>) {
        let key = Self::normalize_path(path);
        info_log!(Component::Lpr, "cache", "Storing LPR3 procedurer data in cache for path: {}", key);
        self.lpr3_procedurer_cache.insert(key, data);
    }

    /// Get LPR3 procedurer data from the cache
    pub fn get_lpr3_procedurer(&self, path: &Path) -> Option<Vec<RecordBatch>> {
        let key = Self::normalize_path(path);
        let result = self.lpr3_procedurer_cache.get(&key).cloned();
        if result.is_some() {
            info_log!(Component::Lpr, "cache", "Retrieved LPR3 procedurer data from cache for path: {}", key);
        }
        result
    }
}

/// Helper function to get the LPR cache
pub fn get_lpr_cache() -> &'static Mutex<LprDataCache> {
    &LPR_CACHE
}

/// Helper function to find all parquet files in a directory and subdirectories
/// using structured logging
pub fn find_parquet_files_in_dir(dir_path: &Path) -> Result<Vec<PathBuf>> {
    traced_operation!(Component::Lpr, "find_files", {
        log_function_entry!(Component::Lpr, "find_parquet_files_in_dir", dir_path);
        
        let mut files = Vec::new();
        
        // Check if directory exists and is a directory
        if !dir_path.exists() {
            warn_log!(Component::Lpr, "find_files", "Directory does not exist: {}", dir_path.display());
            return Ok(files);
        }
        
        if !dir_path.is_dir() {
            warn_log!(Component::Lpr, "find_files", "Path is not a directory: {}", dir_path.display());
            return Ok(files);
        }
        
        // Get the absolute path
        let abs_dir_path = match std::fs::canonicalize(dir_path) {
            Ok(p) => p,
            Err(e) => {
                warn_log!(
                    Component::Lpr, 
                    "find_files", 
                    "Failed to canonicalize path {}: {}", 
                    dir_path.display(), e
                );
                dir_path.to_path_buf()
            }
        };
        
        log_step!(Component::Lpr, "find_files", 1, "Scanning directory: {} (canonical: {})", 
            dir_path.display(), abs_dir_path.display());
        
        // Try to read directory contents
        let entries = match fs::read_dir(&abs_dir_path) {
            Ok(entries) => entries,
            Err(e) => {
                error_log!(
                    Component::Lpr, 
                    "find_files", 
                    "Failed to read directory {}: {}", 
                    abs_dir_path.display(), e
                );
                return Err(IdsError::Io(e));
            }
        };
        
        // Process each entry
        log_step!(Component::Lpr, "find_files", 2, "Processing entries");
        for entry in entries {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    error_log!(Component::Lpr, "find_files", "Failed to read directory entry: {}", e);
                    continue;
                }
            };
            
            let path = entry.path();
            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    error_log!(
                        Component::Lpr, 
                        "find_files", 
                        "Failed to get file type for {}: {}", 
                        path.display(), e
                    );
                    continue;
                }
            };
            
            debug_log!(Component::Lpr, "find_files", "Processing entry: {} (is_dir: {})", 
                path.display(), file_type.is_dir());
            
            if file_type.is_file() {
                // Check if this is a parquet file
                if let Some(ext) = path.extension() {
                    if ext == "parquet" {
                        debug_log!(Component::Lpr, "find_files", "Found parquet file: {}", path.display());
                        files.push(path.clone());
                    }
                }
            } else if file_type.is_dir() {
                // Recursively search subdirectories
                debug_log!(Component::Lpr, "find_files", "Recursively searching subdirectory: {}", path.display());
                
                match find_parquet_files_in_dir(&path) {
                    Ok(subdir_files) => {
                        if !subdir_files.is_empty() {
                            info_log!(
                                Component::Lpr, 
                                "find_files", 
                                "Found {} parquet files in subdirectory {}", 
                                subdir_files.len(), path.display()
                            );
                        }
                        files.extend(subdir_files);
                    }
                    Err(e) => {
                        warn_log!(
                            Component::Lpr, 
                            "find_files", 
                            "Error searching subdirectory {}: {}", 
                            path.display(), e
                        );
                    }
                }
            }
        }
        
        // Summarize results
        log_step!(Component::Lpr, "find_files", 3, "Summarizing results");
        if files.is_empty() {
            warn_log!(
                Component::Lpr, 
                "find_files", 
                "No parquet files found in directory or subdirectories: {}", 
                abs_dir_path.display()
            );
        } else {
            info_log!(
                Component::Lpr, 
                "find_files", 
                "Found {} parquet files in {} and its subdirectories", 
                files.len(), abs_dir_path.display()
            );
        }
        
        log_function_exit!(Component::Lpr, "find_parquet_files_in_dir", files.len());
        Ok(files)
    })
}

/// Helper method to convert RecordBatches to a DataFrame
pub async fn batches_to_dataframe(
    ctx: &SessionContext,
    batches: Vec<RecordBatch>,
) -> Result<DataFrame> {
    log_function_entry!(Component::Data, "batches_to_dataframe", batches.len());
    
    if batches.is_empty() {
        warn_log!(Component::Data, "dataframe", "Cannot create DataFrame from empty batches");
        return Err(IdsError::Validation(
            "Cannot create DataFrame from empty batches".to_string(),
        ));
    }

    // Get schema from the first batch
    let schema = batches[0].schema();
    debug_log!(Component::Data, "dataframe", "Creating DataFrame with schema: {:?}", schema);

    // Create memory table with unique name to avoid conflicts when joining
    let table_name = format!(
        "temp_table_{}",
        Uuid::new_v4().to_string().replace("-", "")
    );
    
    log_step!(Component::Data, "dataframe", 1, "Creating memory table");
    let mem_table = MemTable::try_new(schema, vec![batches])?;

    // Register table in context
    log_step!(Component::Data, "dataframe", 2, "Registering table in context as {}", table_name);
    ctx.register_table(&table_name, Arc::new(mem_table))?;

    // Get table as DataFrame (this is an async operation)
    log_step!(Component::Data, "dataframe", 3, "Converting table to DataFrame");
    match ctx.table(&table_name).await {
        Ok(df) => {
            info_log!(Component::Data, "dataframe", "Successfully created DataFrame");
            log_function_exit!(Component::Data, "batches_to_dataframe", "Success");
            Ok(df)
        }
        Err(e) => {
            error_log!(Component::Data, "dataframe", "Failed to read table: {}", e);
            log_function_exit!(Component::Data, "batches_to_dataframe", "Failed");
            Err(IdsError::DataFusion(e))
        }
    }
}

/// LPR version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LprVersion {
    /// LPR version 2
    V2,
    /// LPR version 3
    V3,
}

impl std::fmt::Display for LprVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::V2 => write!(f, "LPR v2"),
            Self::V3 => write!(f, "LPR v3"),
        }
    }
}

/// Paths to LPR data files
#[derive(Debug)]
pub struct LprPaths {
    /// Base path
    pub base_path: PathBuf,
    /// Path to admin files (LPR v2)
    pub admin_path: Option<PathBuf>,
    /// Path to diagnosis files (LPR v2)
    pub diag_path: Option<PathBuf>,
    /// Path to procedure files (LPR v2)
    pub proc_path: Option<PathBuf>,
    /// Path to contacts files (LPR v3)
    pub kontakter_path: Option<PathBuf>,
    /// Path to diagnoses files (LPR v3)
    pub diagnoser_path: Option<PathBuf>,
    /// Path to procedures files (LPR v3)
    pub procedurer_path: Option<PathBuf>,
}

impl LprPaths {
    /// Create a new `LprPaths` instance
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            admin_path: None,
            diag_path: None,
            proc_path: None,
            kontakter_path: None,
            diagnoser_path: None,
            procedurer_path: None,
        }
    }

    /// Set admin path
    pub fn with_admin_path(mut self, path: impl AsRef<Path>) -> Self {
        self.admin_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set diagnosis path
    pub fn with_diag_path(mut self, path: impl AsRef<Path>) -> Self {
        self.diag_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set procedure path
    pub fn with_proc_path(mut self, path: impl AsRef<Path>) -> Self {
        self.proc_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set kontakter path
    pub fn with_kontakter_path(mut self, path: impl AsRef<Path>) -> Self {
        self.kontakter_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set diagnoser path
    pub fn with_diagnoser_path(mut self, path: impl AsRef<Path>) -> Self {
        self.diagnoser_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set procedurer path
    pub fn with_procedurer_path(mut self, path: impl AsRef<Path>) -> Self {
        self.procedurer_path = Some(path.as_ref().to_path_buf());
        self
    }
}

/// Find LPR files in a directory
pub fn find_lpr_files(base_path: impl AsRef<Path>) -> Result<LprPaths> {
    traced_operation!(Component::Lpr, "find_lpr_paths", {
        // Make sure we have an absolute path for the base directory
        let base_path = crate::utils::path_utils::resolve_path(base_path)?;
        log_function_entry!(Component::Lpr, "find_lpr_files", base_path.display().to_string());
        
        // Verify the base path exists and is a directory
        if !base_path.exists() {
            error_log!(Component::Lpr, "find_lpr_paths", "Base path for LPR files does not exist: {}", base_path.display());
            return Err(crate::error::IdsError::Validation(format!(
                "Base path for LPR files does not exist: {}",
                base_path.display()
            )));
        }
        
        if !base_path.is_dir() {
            error_log!(Component::Lpr, "find_lpr_paths", "Base path for LPR files is not a directory: {}", base_path.display());
            return Err(crate::error::IdsError::Validation(format!(
                "Base path for LPR files is not a directory: {}",
                base_path.display()
            )));
        }
        
        let mut paths = LprPaths::new(&base_path);

        // Helper function to check if directory contains parquet files
        fn directory_has_parquet(dir_path: &Path) -> bool {
            if let Ok(entries) = std::fs::read_dir(dir_path) {
                for entry_result in entries {
                    if let Ok(entry) = entry_result {
                        let path = entry.path();
                        if path.is_file() && path.extension().map_or(false, |ext| ext == "parquet") {
                            return true;
                        }
                    }
                }
            }
            false
        }
        
        log_step!(Component::Lpr, "find_lpr_paths", 1, "Checking if current path is a specific LPR directory");
        
        // If we're loading from a specific LPR directory (e.g., lpr_diag), we need to look
        // for admin data in the parent directory or sibling directories
        let base_path_str = base_path.to_string_lossy().to_lowercase();
        let mut need_to_search_parent = false;
        
        // Check if this is one of the LPR directories directly
        if base_path_str.contains("lpr_adm") || base_path_str.contains("adm") {
            // Check if there are parquet files directly in this directory
            if directory_has_parquet(&base_path) {
                debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v2 admin path (direct): {}", base_path.display());
                paths = paths.with_admin_path(&base_path);
            }
        } else if base_path_str.contains("lpr_diag") || base_path_str.contains("diag") {
            // Check if there are parquet files directly in this directory
            if directory_has_parquet(&base_path) {
                debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v2 diagnosis path (direct): {}", base_path.display());
                paths = paths.with_diag_path(&base_path);
                need_to_search_parent = true;
            }
        } else if base_path_str.contains("lpr_bes") || base_path_str.contains("proc") {
            // Check if there are parquet files directly in this directory
            if directory_has_parquet(&base_path) {
                debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v2 procedure path (direct): {}", base_path.display());
                paths = paths.with_proc_path(&base_path);
                need_to_search_parent = true;
            }
        } else if base_path_str.contains("lpr3_kontakter") || base_path_str.contains("kontakter") {
            // Check if there are parquet files directly in this directory
            if directory_has_parquet(&base_path) {
                debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v3 kontakter path (direct): {}", base_path.display());
                paths = paths.with_kontakter_path(&base_path);
            }
        } else if base_path_str.contains("lpr3_diagnoser") || base_path_str.contains("diagnoser") {
            // Check if there are parquet files directly in this directory
            if directory_has_parquet(&base_path) {
                debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v3 diagnoser path (direct): {}", base_path.display());
                paths = paths.with_diagnoser_path(&base_path);
                need_to_search_parent = true;
            }
        } else if base_path_str.contains("lpr3_procedurer") || base_path_str.contains("procedurer") {
            // Check if there are parquet files directly in this directory
            if directory_has_parquet(&base_path) {
                debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v3 procedurer path (direct): {}", base_path.display());
                paths = paths.with_procedurer_path(&base_path);
                need_to_search_parent = true;
            }
        }
        
        log_step!(Component::Lpr, "find_lpr_paths", 2, "Checking parent directory if needed");
        
        // If we need to search the parent directory (i.e., we found diag/proc but not admin)
        if need_to_search_parent && paths.admin_path.is_none() {
            if let Some(parent_path) = base_path.parent() {
                debug_log!(Component::Lpr, "find_lpr_paths", "Searching parent directory for admin data: {}", parent_path.display());
                
                // Look for admin data in the parent directory
                if let Ok(entries) = std::fs::read_dir(parent_path) {
                    for entry_result in entries {
                        if let Ok(entry) = entry_result {
                            let path = entry.path();
                            if path.is_dir() {
                                let path_str = path.to_string_lossy().to_lowercase();
                                
                                // Check for LPR v2 admin file
                                if path_str.contains("lpr_adm") || path_str.contains("adm") {
                                    if directory_has_parquet(&path) {
                                        paths = paths.with_admin_path(&path);
                                        debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v2 admin path in parent directory: {}", path.display());
                                    }
                                }
                            }
                        }
                    }
                }
                
                // Also check if the parent directory itself has admin data files
                // by naming convention (e.g., /path/to/lpr_adm/2010.parquet)
                let parent_str = parent_path.to_string_lossy().to_lowercase();
                if (parent_str.contains("lpr_adm") || parent_str.contains("adm")) && directory_has_parquet(parent_path) {
                    paths = paths.with_admin_path(parent_path);
                    debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v2 admin path directly in parent directory: {}", parent_path.display());
                }
            }
        }
        
        log_step!(Component::Lpr, "find_lpr_paths", 3, "Checking subdirectories");
        
        // If we're still missing paths, try to find directories in the current path
        // but skip this if we already found a direct match in the current directory
        if paths.admin_path.is_none() && paths.kontakter_path.is_none() {
            if let Ok(entries) = std::fs::read_dir(&base_path) {
                for entry_result in entries {
                    if let Ok(entry) = entry_result {
                        let path = entry.path();
                        if path.is_dir() {
                            let path_str = path.to_string_lossy().to_lowercase();

                            debug_log!(Component::Lpr, "find_lpr_paths", "Checking LPR subdirectory: {}", path.display());

                            // Check for LPR v2 files
                            if path_str.contains("lpr_adm") || path_str.contains("adm") {
                                if directory_has_parquet(&path) {
                                    paths = paths.with_admin_path(&path);
                                    debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v2 admin path: {}", path.display());
                                }
                            } else if path_str.contains("lpr_diag") || path_str.contains("diag") {
                                if directory_has_parquet(&path) {
                                    paths = paths.with_diag_path(&path);
                                    debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v2 diagnosis path: {}", path.display());
                                }
                            } else if path_str.contains("lpr_bes") || path_str.contains("proc") {
                                if directory_has_parquet(&path) {
                                    paths = paths.with_proc_path(&path);
                                    debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v2 procedure path: {}", path.display());
                                }
                            }
                            // Check for LPR v3 files
                            else if path_str.contains("lpr3_kontakter") || path_str.contains("kontakter")
                            {
                                if directory_has_parquet(&path) {
                                    paths = paths.with_kontakter_path(&path);
                                    debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v3 kontakter path: {}", path.display());
                                }
                            } else if path_str.contains("lpr3_diagnoser") || path_str.contains("diagnoser")
                            {
                                if directory_has_parquet(&path) {
                                    paths = paths.with_diagnoser_path(&path);
                                    debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v3 diagnoser path: {}", path.display());
                                }
                            } else if path_str.contains("lpr3_procedurer")
                                || path_str.contains("procedurer")
                            {
                                if directory_has_parquet(&path) {
                                    paths = paths.with_procedurer_path(&path);
                                    debug_log!(Component::Lpr, "find_lpr_paths", "Found LPR v3 procedurer path: {}", path.display());
                                }
                            }
                        }
                    }
                }
            }
        }
        
        log_step!(Component::Lpr, "find_lpr_paths", 4, "Summarizing findings");
        
        // Log the discovered paths for easier troubleshooting
        info_log!(
            Component::Lpr, 
            "find_lpr_paths", 
            "LPR paths found: admin={}, diag={}, proc={}, kontakter={}, diagnoser={}, procedurer={}",
            paths.admin_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "None".to_string()),
            paths.diag_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "None".to_string()),
            paths.proc_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "None".to_string()),
            paths.kontakter_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "None".to_string()),
            paths.diagnoser_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "None".to_string()),
            paths.procedurer_path.as_ref().map(|p| p.display().to_string()).unwrap_or_else(|| "None".to_string())
        );
        
        // Check if any paths were found
        if paths.admin_path.is_none() && paths.kontakter_path.is_none() {
            warn_log!(
                Component::Lpr,
                "find_lpr_paths",
                "No LPR data directories found in {}",
                base_path.display()
            );
        }

        log_function_exit!(Component::Lpr, "find_lpr_files", "Complete");
        Ok(paths)
    })
}

/// Trait for LPR registry loaders
pub trait LprRegistry: RegisterLoader {
    /// Get the LPR version
    fn version(&self) -> LprVersion;

    /// Find LPR files
    fn find_files(&self, path: &Path) -> Result<LprPaths> {
        let abs_path = crate::utils::path_utils::resolve_path(path)?;
        debug_log!(Component::Lpr, "registry", "LprRegistry::find_files with absolute path: {}", abs_path.display());
        find_lpr_files(abs_path)
    }
    
    /// Load all LPR data components without joining them
    /// 
    /// This method loads the individual LPR components (adm, diag, bes for LPR2 or
    /// kontakter, diagnoser, procedurer for LPR3) without joining them.
    async fn load_components(
        &self,
        base_path: &str,
        pnr_filter: Option<&crate::data::registry::traits::PnrFilter>,
    ) -> Result<LprComponents>;
}

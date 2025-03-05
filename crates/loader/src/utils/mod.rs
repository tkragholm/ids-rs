pub mod parquet;
pub mod path;

use arrow::array::Array;
use arrow::record_batch::RecordBatch;
use std::path::{Path, PathBuf};
use std::collections::HashSet;
use crate::IdsError;

/// Detect the data directory structure
pub fn detect_data_structure(base_path: &Path) -> Result<hashbrown::HashMap<String, PathBuf>, crate::IdsError> {
    if !base_path.exists() {
        return Err(IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Base directory not found: {}", base_path.display()),
        )));
    }
    
    let mut paths = hashbrown::HashMap::new();
    
    // Check for direct vs nested structure (with /registers subdirectory)
    let registers_path = base_path.join("registers");
    let _has_registers_subdir = registers_path.exists() && registers_path.is_dir();
    
    // Check for family.parquet
    let family_paths = [
        base_path.join("family.parquet"),
        registers_path.join("family.parquet"),
    ];
    
    for path in &family_paths {
        if path.exists() && path.is_file() {
            log::info!("Found family relations file at: {}", path.display());
            paths.insert("family".to_string(), path.clone());
            break;
        }
    }
    
    // Check for register subdirectories
    let register_dirs = ["akm", "bef", "ind", "uddf"];
    for dir in &register_dirs {
        let paths_to_check = [
            base_path.join(dir),
            registers_path.join(dir),
        ];
        
        for path in &paths_to_check {
            if path.exists() && path.is_dir() {
                // Check if it has parquet files
                if let Ok(entries) = std::fs::read_dir(path) {
                    let parquet_files: Vec<_> = entries
                        .filter_map(Result::ok)
                        .filter(|e| e.path().extension().is_some_and(|ext| ext == "parquet"))
                        .collect();
                        
                    if !parquet_files.is_empty() {
                        log::info!("Found directory {} with {} parquet files", 
                                  path.display(), parquet_files.len());
                        paths.insert(dir.to_string(), path.clone());
                        break;
                    }
                }
            }
        }
    }
    
    Ok(paths)
}

/// Validate a parquet file exists and is readable
pub fn validate_parquet_file(path: &Path) -> Result<(), crate::IdsError> {
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
    
    // Try to open the file and read initial bytes
    let mut file = std::fs::File::open(path)?;
    let mut buffer = [0u8; 4];
    std::io::Read::read_exact(&mut file, &mut buffer)?;
    
    // Verify it's a parquet file (starting with PAR1)
    if &buffer != b"PAR1" {
        return Err(IdsError::invalid_format(format!(
            "File does not appear to be a valid Parquet file: {}",
            path.display()
        )));
    }
    
    Ok(())
}

/// Extract unique PNRs from a record batch
pub fn extract_pnrs_from_batch(batch: &RecordBatch) -> Result<HashSet<String>, crate::IdsError> {
    use arrow::array::StringArray;
    let mut pnr_set = HashSet::new();
    
    // Try different possible PNR column names
    let pnr_column_names = ["PNR", "pnr", "child_pnr", "child_id"];
    
    for name in &pnr_column_names {
        if let Ok(idx) = batch.schema().index_of(name) {
            if let Some(array) = batch.column(idx).as_any().downcast_ref::<StringArray>() {
                for i in 0..array.len() {
                    if !array.is_null(i) {
                        let pnr = array.value(i);
                        if !pnr.is_empty() {
                            pnr_set.insert(pnr.to_string());
                        }
                    }
                }
                break;
            }
        }
    }
    
    Ok(pnr_set)
}

/// Resolve a path, handling relative and absolute paths correctly
pub fn resolve_path(base_path: &Path, relative_path: &str) -> PathBuf {
    let path = Path::new(relative_path);
    if path.is_absolute() {
        path.to_path_buf()
    } else {
        base_path.join(path)
    }
}
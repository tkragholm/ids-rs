pub mod parquet;
pub mod path;

use std::path::{Path, PathBuf};
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

// Utility re-exports


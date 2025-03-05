use std::path::{Path, PathBuf};
use std::collections::HashMap;
use crate::IdsError;

/// Find a parquet file with flexible path resolution
///
/// This function tries multiple strategies to find a parquet file:
/// 1. Direct path as provided
/// 2. Adding .parquet extension if missing
/// 3. Checking subdirectories
///
/// # Arguments
///
/// * `base_paths` - Array of base paths to check
/// * `filename` - Name of the file to find
///
/// # Returns
///
/// PathBuf to the found file or error if not found
pub fn find_parquet_file(base_paths: &[&Path], filename: &str) -> Result<PathBuf, IdsError> {
    let mut paths_to_try = Vec::new();
    
    // Strategy 1: Direct combination of each base path with filename
    for base in base_paths {
        paths_to_try.push(base.join(filename));
    }
    
    // Strategy 2: Add .parquet extension if not present
    if !filename.ends_with(".parquet") {
        for base in base_paths {
            paths_to_try.push(base.join(format!("{}.parquet", filename)));
        }
    }
    
    // Strategy 3: Check in 'registers' subdirectory
    for base in base_paths {
        let registers_path = base.join("registers");
        paths_to_try.push(registers_path.join(filename));
        
        // Also try with extension in subdirectory
        if !filename.ends_with(".parquet") {
            paths_to_try.push(registers_path.join(format!("{}.parquet", filename)));
        }
    }
    
    // Try each path in sequence
    for path in &paths_to_try {
        if path.exists() && path.is_file() {
            log::info!("Found file at: {}", path.display());
            return Ok(path.clone());
        }
    }
    
    // If we get here, we couldn't find the file
    Err(IdsError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        format!(
            "Could not find file '{}'. Tried paths: {}",
            filename,
            paths_to_try
                .iter()
                .map(|p| p.display().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ),
    )))
}

/// Find all parquet files in a directory and its subdirectories
///
/// # Arguments
///
/// * `dir` - Directory to search
/// * `pattern` - Optional filename pattern to match
///
/// # Returns
///
/// Vector of paths to parquet files
pub fn find_all_parquet_files(dir: &Path, pattern: Option<&str>) -> Result<Vec<PathBuf>, IdsError> {
    if !dir.exists() || !dir.is_dir() {
        return Err(IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Directory not found: {}", dir.display()),
        )));
    }
    
    let mut result = Vec::new();
    find_parquet_files_recursive(dir, pattern, &mut result)?;
    
    Ok(result)
}

/// Recursive helper for find_all_parquet_files
fn find_parquet_files_recursive(
    dir: &Path, 
    pattern: Option<&str>,
    result: &mut Vec<PathBuf>
) -> Result<(), IdsError> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            find_parquet_files_recursive(&path, pattern, result)?;
        } else if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
            // If pattern is specified, check if filename matches
            if let Some(pat) = pattern {
                if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
                    if filename.contains(pat) {
                        result.push(path);
                    }
                }
            } else {
                // No pattern, include all parquet files
                result.push(path);
            }
        }
    }
    
    Ok(())
}

/// Group parquet files by register type based on filename patterns
///
/// # Arguments
///
/// * `files` - Vector of parquet file paths
///
/// # Returns
///
/// HashMap with keys for each register type (akm, bef, ind, uddf, family)
pub fn group_files_by_type(files: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    let mut grouped = HashMap::new();
    
    for path in files {
        if let Some(filename) = path.file_name().and_then(|n| n.to_str()) {
            let group = if filename.starts_with("akm") || filename.contains("/akm/") {
                "akm"
            } else if filename.starts_with("bef") || filename.contains("/bef/") {
                "bef"
            } else if filename.starts_with("ind") || filename.contains("/ind/") {
                "ind"
            } else if filename.starts_with("uddf") || filename.contains("/uddf/") {
                "uddf"
            } else if filename.starts_with("family") || filename.contains("family") {
                "family"
            } else {
                "other"
            };
            
            grouped.entry(group.to_string()).or_insert_with(Vec::new).push(path);
        }
    }
    
    grouped
}

/// Extracts numeric components from a filename, useful for sorting by year
///
/// # Arguments
///
/// * `path` - Path to extract numeric component from
///
/// # Returns
///
/// Optional numeric value (i32)
pub fn extract_year_from_filename(path: &Path) -> Option<i32> {
    path.file_stem()
        .and_then(|s| s.to_str())
        .and_then(|s| {
            // Try to extract a 4-digit year
            let year_regex = regex::Regex::new(r"(19|20)\d{2}").ok()?;
            year_regex.find(s).map(|m| m.as_str().parse::<i32>().ok())?
        })
}
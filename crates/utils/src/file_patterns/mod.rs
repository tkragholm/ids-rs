//! File utility functions for working with register data files
//!
//! This module provides functions for finding, organizing, and analyzing register data files.
//! It includes utilities for discovering files, determining file types, and extracting metadata
//! from filenames.

use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::fs;
use std::io::{self, Error, ErrorKind};

/// Find a parquet file across multiple base paths, trying various strategies
///
/// This function searches for a parquet file with the given filename in multiple base paths,
/// trying various strategies:
/// 1. Direct path
/// 2. Adding .parquet extension if needed
/// 3. Looking in subdirectories
///
/// # Arguments
/// * `base_paths` - Slice of base paths to search in
/// * `filename` - Filename to search for
///
/// # Returns
/// * `Result<PathBuf, io::Error>` - Path to the found file or error
pub fn find_parquet_file(base_paths: &[&Path], filename: &str) -> Result<PathBuf, io::Error> {
    let filename_with_ext = if !filename.ends_with(".parquet") {
        format!("{}.parquet", filename)
    } else {
        filename.to_string()
    };
    
    // Try direct path
    for base_path in base_paths {
        let path = base_path.join(&filename_with_ext);
        if path.exists() && path.is_file() {
            return Ok(path);
        }
    }
    
    // Look in subdirectories
    for base_path in base_paths {
        if let Ok(entries) = fs::read_dir(base_path) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    let subpath = path.join(&filename_with_ext);
                    if subpath.exists() && subpath.is_file() {
                        return Ok(subpath);
                    }
                }
            }
        }
    }
    
    Err(Error::new(
        ErrorKind::NotFound,
        format!("Could not find file {} in any of the provided paths", filename),
    ))
}

/// Find all parquet files in a directory and its subdirectories
///
/// # Arguments
/// * `dir` - Directory to search in
/// * `pattern` - Optional pattern to filter filenames
///
/// # Returns
/// * `Result<Vec<PathBuf>, io::Error>` - List of found files or error
pub fn find_all_parquet_files(dir: &Path, pattern: Option<&str>) -> Result<Vec<PathBuf>, io::Error> {
    let mut files = Vec::new();
    find_parquet_files_recursive(dir, &mut files, pattern)?;
    Ok(files)
}

/// Recursive helper for find_all_parquet_files
fn find_parquet_files_recursive(
    dir: &Path,
    files: &mut Vec<PathBuf>,
    pattern: Option<&str>,
) -> Result<(), io::Error> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                find_parquet_files_recursive(&path, files, pattern)?;
            } else if path.extension().is_some_and(|ext| ext == "parquet") {
                let file_name = path.file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("");
                
                let include = match pattern {
                    Some(p) => file_name.contains(p),
                    None => true,
                };
                
                if include {
                    files.push(path);
                }
            }
        }
    }
    Ok(())
}

/// Group files by register type based on filename
///
/// This function organizes parquet files by their register type (akm, bef, ind, etc.)
/// based on filename patterns.
///
/// # Arguments
/// * `files` - List of file paths to organize
///
/// # Returns
/// * `HashMap<String, Vec<PathBuf>>` - Map of register type to file paths
pub fn group_files_by_type(files: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    let mut groups = HashMap::new();
    
    for path in files {
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            let register_type = if file_name.starts_with("akm") {
                "akm"
            } else if file_name.starts_with("bef") {
                "bef"
            } else if file_name.starts_with("ind") {
                "ind"
            } else if file_name.starts_with("uddf") {
                "uddf"
            } else if file_name == "family.parquet" || file_name == "families.parquet" {
                "family"
            } else {
                continue; // Skip unknown files
            };
            
            groups.entry(register_type.to_string())
                .or_insert_with(Vec::new)
                .push(path);
        }
    }
    
    // Sort files within each group by year/period
    for files in groups.values_mut() {
        files.sort_by(|a, b| {
            let a_name = a.file_name().and_then(|n| n.to_str()).unwrap_or("");
            let b_name = b.file_name().and_then(|n| n.to_str()).unwrap_or("");
            
            let a_year = extract_year_from_filename(a_name).unwrap_or(0);
            let b_year = extract_year_from_filename(b_name).unwrap_or(0);
            
            a_year.cmp(&b_year)
        });
    }
    
    groups
}

/// Extract year from a filename using a regex pattern
///
/// This function extracts a year (4 digits starting with 19 or 20) from a filename.
///
/// # Arguments
/// * `path` - Path to extract year from
///
/// # Returns
/// * `Option<i32>` - Extracted year or None
pub fn extract_year_from_filename(filename: &str) -> Option<i32> {
    // Look for a pattern of 4 digits that starts with 19 or 20
    for i in 0..filename.len().saturating_sub(3) {
        let slice = &filename[i..i+4];
        if (slice.starts_with("19") || slice.starts_with("20")) && 
           slice.chars().all(|c| c.is_ascii_digit()) {
            return slice.parse::<i32>().ok();
        }
    }
    None
}

/// Extract period (YYYYMM or YYYY format) from filename
///
/// This function tries to extract a period in either YYYYMM or YYYY format from a filename.
///
/// # Arguments
/// * `filename` - Filename to extract period from
///
/// # Returns
/// * `Option<String>` - Extracted period or None
pub fn extract_period_from_filename(filename: &str) -> Option<String> {
    // First try to find a YYYYMM format
    for i in 0..filename.len().saturating_sub(5) {
        let slice = &filename[i..i+6];
        if (slice.starts_with("19") || slice.starts_with("20")) && 
           slice.chars().all(|c| c.is_ascii_digit()) {
            let _year = &slice[0..4];
            let month = &slice[4..6];
            // Validate month is between 01-12
            if let Ok(m) = month.parse::<u8>() {
                if (1..=12).contains(&m) {
                    return Some(slice.to_string());
                }
            }
        }
    }
    
    // If no YYYYMM found, fall back to just YYYY
    extract_year_from_filename(filename).map(|y| y.to_string())
}

/// Detect register type from filename
///
/// # Arguments
/// * `filename` - Filename to detect type from
///
/// # Returns
/// * `Option<&'static str>` - Register type or None
pub fn detect_register_type(filename: &str) -> Option<&'static str> {
    if filename.starts_with("akm") {
        Some("akm")
    } else if filename.starts_with("bef") {
        Some("bef")
    } else if filename.starts_with("ind") {
        Some("ind")
    } else if filename.starts_with("uddf") {
        Some("uddf")
    } else if filename == "family.parquet" || filename == "families.parquet" {
        Some("family")
    } else {
        None
    }
}

/// Detect data structure in a directory
///
/// This function analyzes a directory to find register files and categorize them.
///
/// # Arguments
/// * `base_dir` - Base directory to analyze
///
/// # Returns
/// * `Result<HashMap<String, PathBuf>, io::Error>` - Map of register type to path
pub fn detect_data_structure(base_dir: &Path) -> Result<HashMap<String, PathBuf>, io::Error> {
    let mut paths = HashMap::new();
    
    // First look for common structures
    if base_dir.join("family.parquet").exists() {
        paths.insert("family".to_string(), base_dir.join("family.parquet"));
    } else if base_dir.join("families.parquet").exists() {
        paths.insert("family".to_string(), base_dir.join("families.parquet"));
    }
    
    // Look for register directories
    for register in &["akm", "bef", "ind", "uddf"] {
        let register_dir = base_dir.join(register);
        if register_dir.exists() && register_dir.is_dir() {
            paths.insert(register.to_string(), register_dir);
        }
    }
    
    // Look for register files directly in base dir
    if let Ok(entries) = fs::read_dir(base_dir) {
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.extension().is_some_and(|ext| ext == "parquet") {
                if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                    if let Some(register_type) = detect_register_type(file_name) {
                        if !paths.contains_key(register_type) {
                            paths.insert(register_type.to_string(), path);
                        }
                    }
                }
            }
        }
    }
    
    // Check for "registers" subdirectory
    let registers_dir = base_dir.join("registers");
    if registers_dir.exists() && registers_dir.is_dir() {
        if let Ok(entries) = fs::read_dir(&registers_dir) {
            for entry in entries.filter_map(Result::ok) {
                let path = entry.path();
                if path.is_dir() {
                    if let Some(dirname) = path.file_name().and_then(|n| n.to_str()) {
                        if ["akm", "bef", "ind", "uddf"].contains(&dirname) {
                            paths.insert(dirname.to_string(), path);
                        }
                    }
                } else if path.extension().is_some_and(|ext| ext == "parquet") {
                    if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
                        if let Some(register_type) = detect_register_type(file_name) {
                            if !paths.contains_key(register_type) {
                                paths.insert(register_type.to_string(), path);
                            }
                        }
                    }
                }
            }
        }
    }
    
    Ok(paths)
}
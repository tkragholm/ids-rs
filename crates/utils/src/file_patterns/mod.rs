//! File pattern utilities for finding and categorizing files.
//!
//! This module provides utilities for working with file patterns, including
//! finding files that match specific patterns, detecting register types, and
//! organizing files into groups.

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::path::{Path, PathBuf};

/// Find a parquet file by name in a list of base directories
///
/// # Arguments
/// * `base_paths` - The base directories to search in
/// * `filename` - The filename to search for
///
/// # Returns
/// A Result containing the full path to the found file, or an error if not found
pub fn find_parquet_file(base_paths: &[&Path], filename: &str) -> Result<PathBuf, io::Error> {
    // Ensure filename ends with .parquet
    let file_to_find = if filename.ends_with(".parquet") {
        filename.to_string()
    } else {
        format!("{filename}.parquet")
    };

    for base_path in base_paths {
        let full_path = base_path.join(&file_to_find);
        if full_path.exists() {
            return Ok(full_path);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("File not found: {filename}"),
    ))
}

/// Find all parquet files in a directory that match a pattern
///
/// # Arguments
/// * `dir` - The directory to search in
/// * `pattern` - An optional regex pattern to match against filenames
///
/// # Returns
/// A Result containing a vector of paths to matching files, or an error
pub fn find_all_parquet_files(
    dir: &Path,
    pattern: Option<&str>,
) -> Result<Vec<PathBuf>, io::Error> {
    if !dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Directory not found: {}", dir.display()),
        ));
    }

    let regex = if let Some(pat) = pattern {
        Some(Regex::new(pat).map_err(|e| {
            io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid regex: {e}"))
        })?)
    } else {
        None
    };

    let mut result = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
            let filename = path.file_name().unwrap().to_string_lossy();

            if let Some(ref re) = regex {
                if re.is_match(&filename) {
                    result.push(path);
                }
            } else {
                result.push(path);
            }
        }
    }

    // Sort by modification time (newest first)
    result.sort_by(|a, b| {
        let a_metadata = std::fs::metadata(a).unwrap();
        let b_metadata = std::fs::metadata(b).unwrap();
        b_metadata
            .modified()
            .unwrap()
            .cmp(&a_metadata.modified().unwrap())
    });

    Ok(result)
}

/// Group files by their register type
///
/// # Arguments
/// * `files` - A vector of file paths
///
/// # Returns
/// A `HashMap` mapping register types to vectors of file paths
#[must_use] pub fn group_files_by_type(files: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
    let mut groups: HashMap<String, Vec<PathBuf>> = HashMap::new();

    for file in files {
        if let Some(filename) = file.file_name() {
            let filename_str = filename.to_string_lossy();
            if let Some(register_type) = detect_register_type(&filename_str) {
                groups
                    .entry(register_type.to_string())
                    .or_default()
                    .push(file);
            } else {
                groups.entry("unknown".to_string()).or_default().push(file);
            }
        }
    }

    groups
}

/// Detect the register type from a filename
///
/// # Arguments
/// * `filename` - The filename to analyze
///
/// # Returns
/// An Option containing the detected register type, or None if not detected
#[must_use] pub fn detect_register_type(filename: &str) -> Option<&'static str> {
    let patterns = [
        ("akm", r"(?i)akm|arbejdsklassifikationsmodul"),
        ("bef", r"(?i)bef|befolkning"),
        ("ind", r"(?i)ind|indkomst"),
        ("uddf", r"(?i)uddf|uddannelse"),
        ("idan", r"(?i)idan|idanmark"),
        ("lpr", r"(?i)lpr|landspatientregister"),
    ];

    for (register_type, pattern) in &patterns {
        if Regex::new(pattern).ok()?.is_match(filename) {
            return Some(register_type);
        }
    }

    None
}

/// Extract a period (YYYYMM or YYYY) from a filename
///
/// # Arguments
/// * `filename` - The filename to extract the period from
///
/// # Returns
/// An Option containing the period string, or None if not found
#[must_use] pub fn extract_period_from_filename(filename: &str) -> Option<String> {
    // Match YYYYMM pattern (e.g., 202301)
    let re_period = Regex::new(r"(?:^|[^\d])(\d{6})(?:[^\d]|$)").ok()?;
    if let Some(cap) = re_period.captures(filename) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }

    // Match YYYY pattern (e.g., 2023)
    let re_year = Regex::new(r"(?:^|[^\d])(\d{4})(?:[^\d]|$)").ok()?;
    if let Some(cap) = re_year.captures(filename) {
        return cap.get(1).map(|m| m.as_str().to_string());
    }

    None
}

/// Extract a year from a filename
///
/// # Arguments
/// * `filename` - The filename to extract the year from
///
/// # Returns
/// An Option containing the year as an i32, or None if not found
#[must_use] pub fn extract_year_from_filename(filename: &str) -> Option<i32> {
    let period = extract_period_from_filename(filename)?;

    // If we have a 6-digit period (YYYYMM), extract the year part
    if period.len() == 6 {
        return period[0..4].parse::<i32>().ok();
    }

    // Otherwise, assume it's already a year
    period.parse::<i32>().ok()
}

/// Detect the data structure in a directory by analyzing file patterns
///
/// # Arguments
/// * `base_dir` - The base directory to analyze
///
/// # Returns
/// A Result containing a `HashMap` mapping register types to file paths, or an error
pub fn detect_data_structure(base_dir: &Path) -> Result<HashMap<String, PathBuf>, io::Error> {
    let mut result = HashMap::new();

    // Find all parquet files
    let files = find_all_parquet_files(base_dir, None)?;

    // Group files by type
    let grouped = group_files_by_type(files);

    // Pick the newest file for each type
    for (register_type, files) in grouped {
        if !files.is_empty() {
            result.insert(register_type, files[0].clone());
        }
    }

    Ok(result)
}

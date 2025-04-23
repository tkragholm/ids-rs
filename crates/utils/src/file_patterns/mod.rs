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
    let file_to_find = if !filename.ends_with(".parquet") {
        format!("{}.parquet", filename)
    } else {
        filename.to_string()
    };

    for base_path in base_paths {
        let full_path = base_path.join(&file_to_find);
        if full_path.exists() {
            return Ok(full_path);
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        format!("File not found: {}", filename),
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
            io::Error::new(io::ErrorKind::InvalidInput, format!("Invalid regex: {}", e))
        })?)
    } else {
        None
    };

    let mut result = Vec::new();

    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() && path.extension().map_or(false, |ext| ext == "parquet") {
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
/// A HashMap mapping register types to vectors of file paths
pub fn group_files_by_type(files: Vec<PathBuf>) -> HashMap<String, Vec<PathBuf>> {
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
pub fn detect_register_type(filename: &str) -> Option<&'static str> {
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
pub fn extract_period_from_filename(filename: &str) -> Option<String> {
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
pub fn extract_year_from_filename(filename: &str) -> Option<i32> {
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
/// A Result containing a HashMap mapping register types to file paths, or an error
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{create_dir_all, File};
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_extract_period_from_filename() {
        assert_eq!(
            extract_period_from_filename("data_202301.csv"),
            Some("202301".to_string())
        );
        assert_eq!(
            extract_period_from_filename("data_2023.csv"),
            Some("2023".to_string())
        );
        assert_eq!(extract_period_from_filename("data.csv"), None);
    }

    #[test]
    fn test_extract_year_from_filename() {
        assert_eq!(extract_year_from_filename("data_202301.csv"), Some(2023));
        assert_eq!(extract_year_from_filename("data_2023.csv"), Some(2023));
        assert_eq!(extract_year_from_filename("data.csv"), None);
    }

    #[test]
    fn test_detect_register_type() {
        assert_eq!(detect_register_type("akm_2023.parquet"), Some("akm"));
        assert_eq!(detect_register_type("befolkning_data.parquet"), Some("bef"));
        assert_eq!(detect_register_type("indkomst_202301.parquet"), Some("ind"));
        assert_eq!(detect_register_type("unknown_data.parquet"), None);
    }

    #[test]
    fn test_find_parquet_file() -> Result<(), io::Error> {
        let temp_dir = tempdir()?;
        let file_path = temp_dir.path().join("test.parquet");

        // Create a test file
        let mut file = File::create(&file_path)?;
        file.write_all(b"test data")?;

        let base_paths = vec![temp_dir.path()];

        // Test finding existing file with .parquet extension
        let found = find_parquet_file(&base_paths.iter().collect::<Vec<&Path>>(), "test.parquet")?;
        assert_eq!(found, file_path);

        // Test finding existing file without .parquet extension
        let found = find_parquet_file(&base_paths.iter().collect::<Vec<&Path>>(), "test")?;
        assert_eq!(found, file_path);

        // Test not finding non-existent file
        let result = find_parquet_file(&base_paths.iter().collect::<Vec<&Path>>(), "nonexistent");
        assert!(result.is_err());

        Ok(())
    }

    #[test]
    fn test_find_all_parquet_files() -> Result<(), io::Error> {
        let temp_dir = tempdir()?;

        // Create test files
        let file1 = temp_dir.path().join("test1.parquet");
        let file2 = temp_dir.path().join("test2.parquet");
        let file3 = temp_dir.path().join("other.txt");

        File::create(&file1)?.write_all(b"test data 1")?;
        File::create(&file2)?.write_all(b"test data 2")?;
        File::create(&file3)?.write_all(b"not a parquet file")?;

        // Test finding all parquet files
        let files = find_all_parquet_files(temp_dir.path(), None)?;
        assert_eq!(files.len(), 2);

        // Test finding files matching pattern
        let files = find_all_parquet_files(temp_dir.path(), Some(r"test1"))?;
        assert_eq!(files.len(), 1);
        assert_eq!(files[0], file1);

        Ok(())
    }

    #[test]
    fn test_group_files_by_type() -> Result<(), io::Error> {
        let temp_dir = tempdir()?;

        // Create test files
        let akm_file = temp_dir.path().join("akm_2023.parquet");
        let bef_file = temp_dir.path().join("befolkning_2023.parquet");
        let unknown_file = temp_dir.path().join("unknown_2023.parquet");

        File::create(&akm_file)?;
        File::create(&bef_file)?;
        File::create(&unknown_file)?;

        let files = vec![akm_file.clone(), bef_file.clone(), unknown_file.clone()];

        // Test grouping files by type
        let grouped = group_files_by_type(files);

        assert_eq!(grouped.len(), 3);
        assert_eq!(grouped["akm"].len(), 1);
        assert_eq!(grouped["bef"].len(), 1);
        assert_eq!(grouped["unknown"].len(), 1);

        assert_eq!(grouped["akm"][0], akm_file);
        assert_eq!(grouped["bef"][0], bef_file);
        assert_eq!(grouped["unknown"][0], unknown_file);

        Ok(())
    }
}

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use types::error::IdsError;

/// Detect the data directory structure
///
/// # Arguments
/// * `base_path` - The base path to check
///
/// # Returns
/// A HashMap of register names to their paths
///
/// # Errors
/// Returns an error if the base path doesn't exist
#[allow(dead_code)]
pub fn detect_data_structure(base_path: &Path) -> Result<HashMap<String, PathBuf>, IdsError> {
    if !base_path.exists() {
        return Err(IdsError::io_error(format!(
            "Base directory not found: {}",
            base_path.display()
        )));
    }

    let mut paths = HashMap::new();

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
        let paths_to_check = [base_path.join(dir), registers_path.join(dir)];

        for path in &paths_to_check {
            if path.exists() && path.is_dir() {
                // Check if it has parquet files
                if let Ok(entries) = std::fs::read_dir(path) {
                    let parquet_files: Vec<_> = entries
                        .filter_map(Result::ok)
                        .filter(|e| e.path().extension().is_some_and(|ext| ext == "parquet"))
                        .collect();

                    if !parquet_files.is_empty() {
                        log::info!(
                            "Found directory {} with {} parquet files",
                            path.display(),
                            parquet_files.len()
                        );
                        paths.insert(dir.to_string(), path.clone());
                        break;
                    }
                }
            }
        }
    }

    Ok(paths)
}

/// Resolve paths, handling relative and absolute paths correctly
///
/// # Arguments
/// * `base_path` - The base path for resolving relative paths
/// * `paths` - HashMap of register names to their paths
///
/// # Returns
/// A HashMap of register names to their resolved paths
///
/// # Errors
/// Returns an error if the base path doesn't exist
#[allow(dead_code)]
pub fn resolve_paths(
    base_path: &str,
    paths: &HashMap<String, String>,
) -> Result<HashMap<String, PathBuf>, IdsError> {
    let mut resolved = HashMap::new();
    let base_path_obj = Path::new(base_path);

    if !base_path_obj.exists() {
        return Err(IdsError::invalid_operation(format!(
            "Base path does not exist: {}",
            base_path
        )));
    }

    // Normalize base_path for easier comparison
    let normalized_base_path = if let Ok(canonical) = base_path_obj.canonicalize() {
        canonical.to_string_lossy().to_string()
    } else {
        base_path.to_string()
    };

    for (key, path) in paths {
        let path_obj = Path::new(path);

        // If path is already absolute, use it as-is
        if path_obj.is_absolute() {
            resolved.insert(key.clone(), path_obj.to_path_buf());
            continue;
        }

        // Check if path already includes the base_path
        if path.contains(base_path) || path.contains(&normalized_base_path) {
            resolved.insert(key.clone(), path_obj.to_path_buf());
            continue;
        }

        // Prepend base_path for relative paths
        let full_path = base_path_obj.join(path_obj);
        resolved.insert(key.clone(), full_path);
    }

    Ok(resolved)
}

/// Validate that all paths exist
///
/// # Arguments
/// * `paths` - HashMap of register names to their paths
///
/// # Returns
/// Ok if all paths exist, otherwise an error with the invalid paths
///
/// # Errors
/// Returns an error if any of the paths don't exist
#[allow(dead_code)]
pub fn validate_paths(paths: &HashMap<String, PathBuf>) -> Result<(), IdsError> {
    let mut invalid_paths = Vec::new();

    for (key, path) in paths {
        if !path.exists() {
            invalid_paths.push(format!("{} ({})", key, path.display()));
        }
    }

    if invalid_paths.is_empty() {
        Ok(())
    } else {
        Err(IdsError::invalid_operation(format!(
            "The following paths do not exist: {}",
            invalid_paths.join(", ")
        )))
    }
}

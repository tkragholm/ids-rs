use log;
use std::path::Path;

use super::resolver::check_path_exists;

/// Normalize path with extensive validation
///
/// Handles special cases like family files and register directories,
/// with optional base directory resolution.
///
/// # Arguments
/// * `path` - The path to normalize
/// * `register_type` - The type of register (family, akm, etc.)
/// * `base_dir` - Optional base directory for relative paths
///
/// # Returns
/// * String - The normalized path
#[must_use]
pub fn normalize_path(path: &str, register_type: &str, base_dir: Option<&str>) -> String {
    let path_obj = Path::new(path);
    let is_family = register_type == "family";

    // Log the input path
    log::debug!("Normalizing {register_type} path: {path}");

    // Special handling for family files to make them more robust
    if is_family {
        // For family files, we prioritize finding the actual file without
        // trying to append it to the covariate directory

        // Check for .parquet extension and add if missing
        let mut family_path = path.to_string();
        if !family_path.ends_with(".parquet") {
            family_path = format!("{family_path}.parquet");
            log::debug!("Added .parquet extension to family path: {family_path}");
        }

        let family_obj = Path::new(&family_path);

        // Try direct absolute path first
        if family_obj.is_absolute() {
            let exists = check_path_exists(&family_path, "family file (absolute)");
            if exists {
                log::info!("Found family file at absolute path: {family_path}");
                return family_path;
            }
        }

        // Try relative to current directory
        let current_dir = std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        let relative_path = current_dir.join(&family_path);
        let relative_str = relative_path.to_string_lossy().to_string();

        let exists = check_path_exists(&relative_str, "family file (relative to current dir)");
        if exists {
            log::info!(
                "Found family file relative to current dir: {relative_str}"
            );
            return relative_str;
        }

        // As a last resort, if base_dir is specified, try there
        // but ONLY if the path doesn't already have directory components
        if let Some(base) = base_dir {
            // Don't do this if the family path already has directory components
            if family_obj.parent().is_none_or(|p| p == Path::new("")) {
                let cov_path = Path::new(base).join(family_obj.file_name().unwrap_or_default());
                let cov_str = cov_path.to_string_lossy().to_string();

                let exists = check_path_exists(&cov_str, "family file (in base dir)");
                if exists {
                    log::info!("Found family file in base dir: {cov_str}");
                    return cov_str;
                }
            }
        }

        // If we've tried everything and still don't have a file,
        // return the most reasonable path (prioritizing the original input)
        if family_obj.is_absolute() {
            return family_path;
        }
        return relative_str;
    }

    // Standard handling for non-family register directories
    if path_obj.is_absolute() {
        // If the path is absolute, use it as-is
        log::debug!("Using absolute path for {register_type}: {path}");
        let _ = check_path_exists(path, &format!("{register_type} (absolute)"));
        path.to_string()
    } else if let Some(base) = base_dir {
        // Check if the path already starts with the base_dir to avoid duplication
        if path.contains(base) {
            log::debug!("Path already contains base_dir ({base}): {path}");
            let _ = check_path_exists(path, &format!("{register_type} (with base_dir)"));
            path.to_string()
        } else {
            let full_path = Path::new(base).join(path).to_string_lossy().to_string();
            log::debug!(
                "Combining base_dir and path: {base} + {path} -> {full_path}"
            );
            let _ = check_path_exists(&full_path, &format!("{register_type} (combined)"));
            full_path
        }
    } else {
        // No base directory provided, use the path as-is
        log::debug!("Using path as-is (no base directory): {path}");
        let _ = check_path_exists(path, register_type);
        path.to_string()
    }
}

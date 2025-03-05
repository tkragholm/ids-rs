use std::path::Path;
use log;

/// Resolve a path - absolute or relative to base
///
/// If the path is absolute, it is returned as-is.
/// If it is relative, it is resolved relative to the base path.
///
/// # Arguments
/// * `base_path` - The base directory to resolve relative paths from
/// * `path` - The path to resolve
///
/// # Returns
/// * String - The resolved path
pub fn resolve_path(base_path: &str, path: &str) -> String {
    let path_obj = Path::new(path);
    
    if path_obj.is_absolute() {
        path.to_string()
    } else {
        Path::new(base_path)
            .join(path)
            .to_string_lossy()
            .to_string()
    }
}

/// Check if a path exists and log its status
///
/// # Arguments
/// * `path` - The path to check
/// * `path_type` - A description of what the path represents (for logging)
///
/// # Returns
/// * bool - True if the path exists, false otherwise
pub fn check_path_exists(path: &str, path_type: &str) -> bool {
    let path_obj = Path::new(path);
    let exists = path_obj.exists();
    let path_type_str = if path_obj.is_dir() {
        "directory"
    } else if path_obj.is_file() {
        "file"
    } else {
        "path"
    };

    if exists {
        log::info!("Found {} at {} ({})", path_type, path, path_type_str);
    } else {
        log::warn!("{} not found at {}", path_type, path);
    }
    exists
}

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
pub fn normalize_path(path: &str, register_type: &str, base_dir: Option<&str>) -> String {
    let path_obj = Path::new(path);
    let is_family = register_type == "family";

    // Log the input path
    log::debug!("Normalizing {} path: {}", register_type, path);

    // Special handling for family files to make them more robust
    if is_family {
        // For family files, we prioritize finding the actual file without
        // trying to append it to the covariate directory

        // Check for .parquet extension and add if missing
        let mut family_path = path.to_string();
        if !family_path.ends_with(".parquet") {
            family_path = format!("{family_path}.parquet");
            log::debug!("Added .parquet extension to family path: {}", family_path);
        }

        let family_obj = Path::new(&family_path);

        // Try direct absolute path first
        if family_obj.is_absolute() {
            check_path_exists(&family_path, "family file (absolute)");
            if Path::new(&family_path).exists() {
                log::info!("Found family file at absolute path: {}", family_path);
                return family_path;
            }
        }

        // Try relative to current directory
        let current_dir =
            std::env::current_dir().unwrap_or_else(|_| Path::new(".").to_path_buf());
        let relative_path = current_dir.join(&family_path);
        let relative_str = relative_path.to_string_lossy().to_string();

        check_path_exists(&relative_str, "family file (relative to current dir)");
        if relative_path.exists() {
            log::info!(
                "Found family file relative to current dir: {}",
                relative_str
            );
            return relative_str;
        }

        // As a last resort, if base_dir is specified, try there
        // but ONLY if the path doesn't already have directory components
        if let Some(base) = base_dir {
            // Don't do this if the family path already has directory components
            if family_obj.parent().map_or(true, |p| p == Path::new("")) {
                let cov_path = Path::new(base).join(family_obj.file_name().unwrap_or_default());
                let cov_str = cov_path.to_string_lossy().to_string();

                check_path_exists(&cov_str, "family file (in base dir)");
                if cov_path.exists() {
                    log::info!("Found family file in base dir: {}", cov_str);
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
        log::debug!("Using absolute path for {}: {}", register_type, path);
        check_path_exists(path, &format!("{register_type} (absolute)"));
        path.to_string()
    } else if let Some(base) = base_dir {
        // Check if the path already starts with the base_dir to avoid duplication
        if path.contains(base) {
            log::debug!("Path already contains base_dir ({}): {}", base, path);
            check_path_exists(path, &format!("{register_type} (with base_dir)"));
            path.to_string()
        } else {
            let full_path = Path::new(base).join(path).to_string_lossy().to_string();
            log::debug!(
                "Combining base_dir and path: {} + {} -> {}",
                base,
                path,
                full_path
            );
            check_path_exists(&full_path, &format!("{register_type} (combined)"));
            full_path
        }
    } else {
        // No base directory provided, use the path as-is
        log::debug!("Using path as-is (no base directory): {}", path);
        check_path_exists(path, register_type);
        path.to_string()
    }
}
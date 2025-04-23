use log;
use std::path::Path;

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
#[must_use]
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
#[must_use]
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
        log::info!("Found {path_type} at {path} ({path_type_str})");
    } else {
        log::warn!("{path_type} not found at {path}");
    }
    exists
}

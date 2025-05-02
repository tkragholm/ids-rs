//! Utilities for working with paths and file operations
//!
//! This module provides helper functions for path operations, including
//! resolving relative paths to absolute paths, path normalization, etc.

use std::path::{Path, PathBuf};

use crate::error::{IdsError, Result};

/// Resolves a path to an absolute path.
///
/// If the given path is already absolute, it is returned unchanged.
/// If the path is relative, it is resolved against the current working directory
/// and normalized to handle ".." components properly.
///
/// This is particularly important when passing paths to DataFusion, which requires
/// absolute paths when accessing files.
///
/// # Arguments
///
/// * `path` - The path to resolve, can be relative or absolute
///
/// # Returns
///
/// * `Result<PathBuf>` - The resolved absolute path or an error
///
/// # Examples
///
/// ```
/// use ids_rs::utils::path_utils::resolve_path;
///
/// let path = std::path::Path::new("./data/file.txt");
/// let abs_path = resolve_path(path).unwrap();
/// assert!(abs_path.is_absolute());
/// ```
pub fn resolve_path(path: impl AsRef<Path>) -> Result<PathBuf> {
    let path = path.as_ref();
    
    // Try to use the more robust fs::canonicalize first, which handles .. properly,
    // but it requires the path to exist
    if path.exists() {
        let canonical = std::fs::canonicalize(path)
            .map_err(|e| IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to canonicalize path {}: {}", path.display(), e),
            )))?;
            
        log::debug!(
            "Path resolution (canonicalized): original='{}' resolved='{}'", 
            path.display(), 
            canonical.display()
        );
        
        return Ok(canonical);
    }
    
    // Fall back to manual normalization if the path doesn't exist yet
    let result = if path.is_absolute() {
        // For absolute paths, we still need to normalize them (e.g., handle .. components)
        normalize_path(path)
    } else {
        // For relative paths, join with current dir and then normalize
        let cwd = std::env::current_dir().map_err(IdsError::Io)?;
        
        // Handle cases where we have a relative path with .. components like "../../path"
        // First, combine with current directory
        let mut absolute_path = cwd.clone();
        
        // Manually process the path components to handle "../" correctly
        for component in path.components() {
            match component {
                std::path::Component::ParentDir => {
                    // Go up one directory level if possible
                    if absolute_path.parent().is_some() {
                        absolute_path = absolute_path.parent().unwrap().to_path_buf();
                    }
                },
                std::path::Component::CurDir => {
                    // Current directory - no change needed
                },
                component => {
                    // Regular component - add it
                    absolute_path.push(component);
                }
            }
        }
        
        absolute_path
    };
    
    log::debug!(
        "Path resolution (normalized): original='{}' resolved='{}'", 
        path.display(), 
        result.display()
    );
    
    Ok(result)
}

/// Normalizes a path by removing redundant components like ".." and ".".
///
/// This is a workaround for Rust's lack of a built-in canonicalize that doesn't
/// require the path to exist. This function performs path normalization without
/// hitting the file system.
///
/// # Arguments
///
/// * `path` - The path to normalize
///
/// # Returns
///
/// * `PathBuf` - The normalized path
fn normalize_path(path: impl AsRef<Path>) -> PathBuf {
    let path = path.as_ref();
    
    // Split the path into components and process them
    let mut components = Vec::new();
    let is_absolute = path.is_absolute();
    
    // Process each component
    for component in path.components() {
        match component {
            std::path::Component::ParentDir => {
                // ".." - pop the last component if there is one to pop
                // (but don't pop past the root)
                if !components.is_empty() && !is_root_component(&components[components.len() - 1]) {
                    components.pop();
                } else if !is_absolute {
                    // If it's a relative path, keep the .. component
                    components.push(component);
                }
                // If absolute path and we're at root, ignore the ".."
            },
            std::path::Component::CurDir => {
                // "." - ignore it unless it's the only component
                if components.is_empty() && !is_absolute {
                    components.push(component);
                }
            },
            component => {
                // Normal component or root - add it
                components.push(component);
            }
        }
    }
    
    // Rebuild the path from components
    let mut result = PathBuf::new();
    for component in components {
        result.push(component.as_os_str());
    }
    
    // Handle empty path
    if result.as_os_str().is_empty() && !is_absolute {
        result.push(".");
    }
    
    result
}

/// Checks if a path component is a root component
fn is_root_component(component: &std::path::Component) -> bool {
    matches!(component, std::path::Component::RootDir | std::path::Component::Prefix(_))
}

/// Ensures a directory exists, creating it if necessary.
///
/// This is a convenience wrapper around `std::fs::create_dir_all` that returns
/// our custom `Result` type.
///
/// # Arguments
///
/// * `dir_path` - The directory path to ensure exists
///
/// # Returns
///
/// * `Result<()>` - Success or an error
///
/// # Examples
///
/// ```
/// use ids_rs::utils::path_utils::ensure_dir_exists;
///
/// let dir = std::path::Path::new("./output/reports");
/// ensure_dir_exists(dir).unwrap();
/// ```
pub fn ensure_dir_exists(dir_path: impl AsRef<Path>) -> Result<()> {
    let dir_path = dir_path.as_ref();
    
    if !dir_path.exists() {
        std::fs::create_dir_all(dir_path).map_err(IdsError::Io)?;
    }
    
    Ok(())
}
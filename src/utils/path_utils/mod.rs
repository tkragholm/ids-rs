//! Utilities for working with paths and file operations
//!
//! This module provides helper functions for path operations, including
//! resolving relative paths to absolute paths, path normalization, etc.

use std::path::{Path, PathBuf};

use crate::error::{IdsError, Result};

/// Resolves a path to an absolute path.
///
/// If the given path is already absolute, it is returned unchanged.
/// If the path is relative, it is joined with the current working directory.
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
    
    if path.is_absolute() {
        Ok(path.to_path_buf())
    } else {
        std::env::current_dir()
            .map_err(|e| IdsError::Io(e))
            .map(|cwd| cwd.join(path))
    }
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
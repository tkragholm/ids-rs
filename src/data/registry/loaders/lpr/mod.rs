//! LPR (Landspatientregistret) registry loaders
//!
//! This module contains registry loaders for the LPR (Landspatientregistret) registry.

use crate::data::registry::traits::RegisterLoader;
use crate::error::Result;
use std::path::{Path, PathBuf};

/// LPR version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LprVersion {
    /// LPR version 2
    V2,
    /// LPR version 3
    V3,
}

/// Paths to LPR data files
pub struct LprPaths {
    /// Base path
    pub base_path: PathBuf,
    /// Path to admin files (LPR v2)
    pub admin_path: Option<PathBuf>,
    /// Path to diagnosis files (LPR v2)
    pub diag_path: Option<PathBuf>,
    /// Path to procedure files (LPR v2)
    pub proc_path: Option<PathBuf>,
    /// Path to contacts files (LPR v3)
    pub kontakter_path: Option<PathBuf>,
    /// Path to diagnoses files (LPR v3)
    pub diagnoser_path: Option<PathBuf>,
    /// Path to procedures files (LPR v3)
    pub procedurer_path: Option<PathBuf>,
}

impl LprPaths {
    /// Create a new LprPaths instance
    pub fn new(base_path: impl AsRef<Path>) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
            admin_path: None,
            diag_path: None,
            proc_path: None,
            kontakter_path: None,
            diagnoser_path: None,
            procedurer_path: None,
        }
    }

    /// Set admin path
    pub fn with_admin_path(mut self, path: impl AsRef<Path>) -> Self {
        self.admin_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set diagnosis path
    pub fn with_diag_path(mut self, path: impl AsRef<Path>) -> Self {
        self.diag_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set procedure path
    pub fn with_proc_path(mut self, path: impl AsRef<Path>) -> Self {
        self.proc_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set kontakter path
    pub fn with_kontakter_path(mut self, path: impl AsRef<Path>) -> Self {
        self.kontakter_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set diagnoser path
    pub fn with_diagnoser_path(mut self, path: impl AsRef<Path>) -> Self {
        self.diagnoser_path = Some(path.as_ref().to_path_buf());
        self
    }

    /// Set procedurer path
    pub fn with_procedurer_path(mut self, path: impl AsRef<Path>) -> Self {
        self.procedurer_path = Some(path.as_ref().to_path_buf());
        self
    }
}

/// Find LPR files in a directory
pub fn find_lpr_files(base_path: impl AsRef<Path>) -> Result<LprPaths> {
    let base_path = base_path.as_ref();
    let mut paths = LprPaths::new(base_path);

    // Try to find subdirectories for LPR files
    if let Ok(entries) = std::fs::read_dir(base_path) {
        for entry_result in entries {
            if let Ok(entry) = entry_result {
                let path = entry.path();
                if path.is_dir() {
                    let path_str = path.to_string_lossy().to_lowercase();

                    // Check for LPR v2 files
                    if path_str.contains("lpr_adm") || path_str.contains("adm") {
                        paths = paths.with_admin_path(path);
                    } else if path_str.contains("lpr_diag") || path_str.contains("diag") {
                        paths = paths.with_diag_path(path);
                    } else if path_str.contains("lpr_bes") || path_str.contains("proc") {
                        paths = paths.with_proc_path(path);
                    }
                    // Check for LPR v3 files
                    else if path_str.contains("lpr3_kontakter") || path_str.contains("kontakter")
                    {
                        paths = paths.with_kontakter_path(path);
                    } else if path_str.contains("lpr3_diagnoser") || path_str.contains("diagnoser")
                    {
                        paths = paths.with_diagnoser_path(path);
                    } else if path_str.contains("lpr3_procedurer")
                        || path_str.contains("procedurer")
                    {
                        paths = paths.with_procedurer_path(path);
                    }
                }
            }
        }
    }

    Ok(paths)
}

/// Trait for LPR registry loaders
pub trait LprRegistry: RegisterLoader {
    /// Get the LPR version
    fn version(&self) -> LprVersion;

    /// Find LPR files
    fn find_files(&self, path: &Path) -> Result<LprPaths> {
        find_lpr_files(path)
    }
}

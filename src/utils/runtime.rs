//! Runtime utilities for async operations
//!
//! This module provides utilities for managing Tokio runtime instances.

use crate::error::{IdsError, Result};
use once_cell::sync::OnceCell;
use tokio::runtime::Runtime;

// Global runtime instance using OnceCell for safe initialization
static RUNTIME_INSTANCE: OnceCell<Runtime> = OnceCell::new();

/// Get a reference to the global tokio runtime
///
/// This function returns a reference to the global tokio runtime instance.
/// If the runtime has not yet been created, it will create a multi-threaded runtime.
pub fn get_runtime() -> Result<&'static Runtime> {
    RUNTIME_INSTANCE.get_or_try_init(|| {
        Runtime::new()
            .map_err(|e| IdsError::Data(format!("Failed to create Tokio runtime: {}", e)))
    })
}

/// Create a new Tokio runtime
///
/// This function creates a new Tokio runtime with the default configuration.
/// This should be used when you need a dedicated runtime for a specific purpose.
pub fn create_runtime() -> Result<Runtime> {
    Runtime::new()
        .map_err(|e| IdsError::Data(format!("Failed to create async runtime: {e}")))
}
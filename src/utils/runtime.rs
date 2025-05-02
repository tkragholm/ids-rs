//! Runtime utilities for async operations
//!
//! This module provides utilities for managing Tokio runtime instances.

use crate::error::{IdsError, Result};
use std::sync::Once;
use tokio::runtime::Runtime;

// Global runtime instance 
static mut RUNTIME_INSTANCE: Option<Runtime> = None;
static INIT_RUNTIME: Once = Once::new();

/// Get a reference to the global tokio runtime
///
/// This function returns a reference to the global tokio runtime instance.
/// If the runtime has not yet been created, it will create a multi-threaded runtime.
pub fn get_runtime() -> Result<&'static Runtime> {
    unsafe {
        INIT_RUNTIME.call_once(|| {
            RUNTIME_INSTANCE = Some(
                Runtime::new()
                    .expect("Failed to create Tokio runtime"),
            );
        });
        
        RUNTIME_INSTANCE
            .as_ref()
            .ok_or_else(|| IdsError::Data("Tokio runtime not initialized".to_string()))
    }
}

/// Create a new Tokio runtime
///
/// This function creates a new Tokio runtime with the default configuration.
/// This should be used when you need a dedicated runtime for a specific purpose.
pub fn create_runtime() -> Result<Runtime> {
    Runtime::new()
        .map_err(|e| IdsError::Data(format!("Failed to create async runtime: {e}")))
}
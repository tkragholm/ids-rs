//! Schema definitions for registry data
//!
//! This module contains schema definitions for various Danish registry data sources.

pub mod registry;
pub mod traits;

// Re-export schema traits
pub use traits::RegistrySchema;

// Re-export registry schemas
pub use registry::*;

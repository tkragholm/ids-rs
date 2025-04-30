//! Registry schema definitions
//!
//! This module contains schema definitions for various Danish registry data sources.

pub mod akm;

// Re-export registry schemas
pub use akm::AkmSchema;
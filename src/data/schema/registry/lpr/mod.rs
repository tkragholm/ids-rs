//! LPR (Landspatientregistret) schema definitions
//!
//! This module contains schema definitions for the LPR registry in both version 2 and 3 formats.

use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::Schema;
use std::collections::HashMap;

pub mod lpr2;
pub mod lpr3;

/// LPR version
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LprVersion {
    /// LPR version 2
    V2,
    /// LPR version 3
    V3,
}

/// Trait for LPR schemas
pub trait LprSchema: RegistrySchema {
    /// Get the LPR version for this schema
    fn version() -> LprVersion;

    /// Get the schema for a specific component (admin, diag, etc.)
    fn component_schema(component: &str) -> Option<Schema>;

    /// Get the component names for this LPR version
    fn component_names() -> Vec<&'static str>;
}

/// Get default metadata for LPR schemas
#[must_use] pub fn default_lpr_metadata(version: LprVersion) -> HashMap<String, String> {
    let mut metadata = HashMap::new();
    metadata.insert("source".to_string(), "ids-rs".to_string());
    metadata.insert("registry".to_string(), "LPR".to_string());

    match version {
        LprVersion::V2 => {
            metadata.insert("version".to_string(), "2".to_string());
            metadata.insert(
                "description".to_string(),
                "Hospital patient registry v2".to_string(),
            );
        }
        LprVersion::V3 => {
            metadata.insert("version".to_string(), "3".to_string());
            metadata.insert(
                "description".to_string(),
                "Hospital patient registry v3".to_string(),
            );
        }
    }

    metadata
}

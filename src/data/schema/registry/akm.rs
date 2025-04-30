use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// Schema implementation for AKM (Arbejdsklassifikationsmodulet) data
pub struct AkmSchema;

impl RegistrySchema for AkmSchema {
    /// Get the Arrow schema for AKM data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("SOCIO", DataType::Int8, true),
            Field::new("SOCIO02", DataType::Int8, true),
            Field::new("SOCIO13", DataType::Int8, true),
            Field::new("CPRTJEK", DataType::Utf8, true),
            Field::new("CPRTYPE", DataType::Utf8, true),
            Field::new("VERSION", DataType::Utf8, true),
            Field::new("SENR", DataType::Utf8, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec!["PNR", "SOCIO", "SOCIO02", "SOCIO13", "CPRTJEK", "CPRTYPE", "VERSION", "SENR"]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "AKM".to_string());
        metadata.insert("description".to_string(), "Employment information".to_string());
        metadata
    }
}
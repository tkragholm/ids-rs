use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// Schema implementation for UDDF (Uddannelse) data
pub struct UddfSchema;

impl RegistrySchema for UddfSchema {
    /// Get the Arrow schema for UDDF data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("CPRTJEK", DataType::Utf8, true),
            Field::new("CPRTYPE", DataType::Utf8, true),
            Field::new("HFAUDD", DataType::Utf8, true),
            Field::new("HF_KILDE", DataType::Utf8, true),
            Field::new("HF_VFRA", DataType::Utf8, true),
            Field::new("HF_VTIL", DataType::Utf8, true),
            Field::new("INSTNR", DataType::Int8, true),
            Field::new("VERSION", DataType::Utf8, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec![
            "PNR", "CPRTJEK", "CPRTYPE", "HFAUDD", "HF_KILDE", 
            "HF_VFRA", "HF_VTIL", "INSTNR", "VERSION"
        ]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "UDDF".to_string());
        metadata.insert("description".to_string(), "Educational information".to_string());
        metadata
    }
}
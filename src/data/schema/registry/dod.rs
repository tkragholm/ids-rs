use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// Schema implementation for the Danish Death Register (DOD)
pub struct DodSchema;

impl RegistrySchema for DodSchema {
    /// Get the Arrow schema for DOD data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("DODDATO", DataType::Utf8, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec!["PNR", "DODDATO"]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "DOD".to_string());
        metadata.insert("description".to_string(), "Danish Death Register".to_string());
        metadata
    }
}

/// Schema implementation for standardized DOD data
pub struct DodStandardizedSchema;

impl RegistrySchema for DodStandardizedSchema {
    /// Get the Arrow schema for standardized DOD data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("DEATH_DATE", DataType::Date32, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec!["PNR", "DEATH_DATE"]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "DOD".to_string());
        metadata.insert("description".to_string(), "Standardized Danish Death Register".to_string());
        metadata
    }
}
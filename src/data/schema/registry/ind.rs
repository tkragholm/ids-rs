use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// Schema implementation for IND (Indkomst) data
pub struct IndSchema;

impl RegistrySchema for IndSchema {
    /// Get the Arrow schema for IND data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("BESKST13", DataType::Int8, true),
            Field::new("CPRTJEK", DataType::Utf8, true),
            Field::new("CPRTYPE", DataType::Utf8, true),
            Field::new("LOENMV_13", DataType::Float64, true),
            Field::new("PERINDKIALT_13", DataType::Float64, true),
            Field::new("PNR", DataType::Utf8, false),
            Field::new("PRE_SOCIO", DataType::Int8, true),
            Field::new("VERSION", DataType::Utf8, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec!["BESKST13", "CPRTJEK", "CPRTYPE", "LOENMV_13", "PERINDKIALT_13", "PNR", "PRE_SOCIO", "VERSION"]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "IND".to_string());
        metadata.insert("description".to_string(), "Income and tax information".to_string());
        metadata
    }
}
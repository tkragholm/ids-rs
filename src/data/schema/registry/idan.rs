use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// Schema implementation for IDAN (Danish employment statistics) data
pub struct IdanSchema;

impl RegistrySchema for IdanSchema {
    /// Get the Arrow schema for IDAN data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("ARBGNR", DataType::Utf8, true),
            Field::new("ARBNR", DataType::Int8, true),
            Field::new("CPRTJEK", DataType::Int8, true),
            Field::new("CPRTYPE", DataType::Int8, true),
            Field::new("CVRNR", DataType::Utf8, true),
            Field::new("JOBKAT", DataType::Int8, true),
            Field::new("JOBLON", DataType::Float64, true),
            Field::new("LBNR", DataType::Utf8, true),
            Field::new("PNR", DataType::Utf8, false),
            Field::new("STILL", DataType::Utf8, true),
            Field::new("TILKNYT", DataType::Int8, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec![
            "ARBGNR", "ARBNR", "CPRTJEK", "CPRTYPE", "CVRNR", 
            "JOBKAT", "JOBLON", "LBNR", "PNR", "STILL", "TILKNYT"
        ]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "IDAN".to_string());
        metadata.insert("description".to_string(), "Danish employment statistics".to_string());
        metadata
    }
}
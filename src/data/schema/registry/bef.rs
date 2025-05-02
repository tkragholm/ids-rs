use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// Schema implementation for BEF (Befolkning) data
pub struct BefSchema;

impl RegistrySchema for BefSchema {
    /// Get the Arrow schema for BEF data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("FOED_DAG", DataType::Date32, true),
            Field::new("FAR_ID", DataType::Utf8, true),
            Field::new("MOR_ID", DataType::Utf8, true),
            Field::new("FAMILIE_ID", DataType::Utf8, true),
            // Optional fields that may be included for completeness
            Field::new("AEGTE_ID", DataType::Utf8, true),
            Field::new("ALDER", DataType::Int8, true),
            Field::new("ANTBOERNF", DataType::Int8, true),
            Field::new("ANTBOERNH", DataType::Int8, true),
            Field::new("ANTPERSF", DataType::Int8, true),
            Field::new("ANTPERSH", DataType::Int8, true),
            Field::new("BOP_VFRA", DataType::Date32, true),
            Field::new("CIVST", DataType::Utf8, true),
            Field::new("CPRTJEK", DataType::Int8, true),
            Field::new("CPRTYPE", DataType::Int8, true),
            Field::new("E_FAELLE_ID", DataType::Utf8, true),
            Field::new("FAMILIE_TYPE", DataType::Int8, true),
            Field::new("FM_MARK", DataType::Int8, true),
            Field::new("HUSTYPE", DataType::Int8, true),
            Field::new("IE_TYPE", DataType::Utf8, true),
            Field::new("KOEN", DataType::Utf8, true),
            Field::new("KOM", DataType::Int8, true),
            Field::new("OPR_LAND", DataType::Utf8, true),
            Field::new("PLADS", DataType::Int8, true),
            Field::new("REG", DataType::Int8, true),
            Field::new("STATSB", DataType::Int8, true),
            Field::new("VERSION", DataType::Utf8, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec![
            "PNR", "FOED_DAG", "FAR_ID", "MOR_ID", "FAMILIE_ID",
            "AEGTE_ID", "ALDER", "ANTBOERNF", "ANTBOERNH", "ANTPERSF",
            "ANTPERSH", "BOP_VFRA", "CIVST", "CPRTJEK", "CPRTYPE",
            "E_FAELLE_ID", "FAMILIE_TYPE", "FM_MARK", "HUSTYPE", "IE_TYPE",
            "KOEN", "KOM", "OPR_LAND", "PLADS", "REG", "STATSB", "VERSION",
        ]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "BEF".to_string());
        metadata.insert("description".to_string(), "Population demographic information".to_string());
        metadata
    }
}
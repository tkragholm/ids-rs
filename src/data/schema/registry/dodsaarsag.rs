use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// Schema implementation for the Danish Death Cause Register (DODSAARSAG)
pub struct DodsaarsagSchema;

impl RegistrySchema for DodsaarsagSchema {
    /// Get the Arrow schema for DODSAARSAG data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("C_AARSAG", DataType::Utf8, true),  // Cause of death code (ICD-10)
            Field::new("C_TILSTAND", DataType::Utf8, true),  // Condition code
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec!["PNR", "C_AARSAG", "C_TILSTAND"]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "DODSAARSAG".to_string());
        metadata.insert("description".to_string(), "Danish Death Cause Register".to_string());
        metadata
    }
}

/// Schema implementation for standardized DODSAARSAG data
pub struct DodsaarsagStandardizedSchema;

impl RegistrySchema for DodsaarsagStandardizedSchema {
    /// Get the Arrow schema for standardized DODSAARSAG data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("DEATH_CAUSE", DataType::Utf8, true),  // Normalized cause code
            Field::new("DEATH_CONDITION", DataType::Utf8, true),  // Normalized condition code
            Field::new("DEATH_CAUSE_CHAPTER", DataType::Utf8, true),  // ICD-10 chapter of death cause
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec!["PNR", "DEATH_CAUSE", "DEATH_CONDITION", "DEATH_CAUSE_CHAPTER"]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "DODSAARSAG".to_string());
        metadata.insert("description".to_string(), "Standardized Danish Death Cause Register".to_string());
        metadata
    }
}
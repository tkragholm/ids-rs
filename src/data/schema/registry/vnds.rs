use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// Schema implementation for the Danish Migration Register (VNDS)
pub struct VndsSchema;

impl RegistrySchema for VndsSchema {
    /// Get the Arrow schema for VNDS data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("INDUD_KODE", DataType::Utf8, true),  // Migration code (in/out)
            Field::new("HAEND_DATO", DataType::Utf8, true),  // Event date
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec!["PNR", "INDUD_KODE", "HAEND_DATO"]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "VNDS".to_string());
        metadata.insert("description".to_string(), "Danish Migration Register".to_string());
        metadata
    }
}

/// Schema implementation for standardized VNDS data
pub struct VndsStandardizedSchema;

impl RegistrySchema for VndsStandardizedSchema {
    /// Get the Arrow schema for standardized VNDS data
    fn schema() -> Schema {
        Schema::new(vec![
            Field::new("PNR", DataType::Utf8, false),
            Field::new("MIGRATION_TYPE", DataType::Utf8, true),  // "IN" or "OUT"
            Field::new("MIGRATION_DATE", DataType::Date32, true),  // Standardized date
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec!["PNR", "MIGRATION_TYPE", "MIGRATION_DATE"]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        let mut metadata = HashMap::new();
        metadata.insert("source".to_string(), "ids-rs".to_string());
        metadata.insert("registry".to_string(), "VNDS".to_string());
        metadata.insert("description".to_string(), "Standardized Danish Migration Register".to_string());
        metadata
    }
}

/// Migration type values
pub enum MigrationType {
    /// Immigration into Denmark
    Immigration,
    /// Emigration out of Denmark
    Emigration,
}

impl MigrationType {
    /// Convert a migration code to a migration type
    #[must_use] pub fn from_code(code: &str) -> Option<Self> {
        match code {
            "I" | "i" | "1" => Some(Self::Immigration),
            "U" | "u" | "0" => Some(Self::Emigration),
            _ => None,
        }
    }
    
    /// Get the string representation of the migration type
    #[must_use] pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Immigration => "IN",
            Self::Emigration => "OUT",
        }
    }
}
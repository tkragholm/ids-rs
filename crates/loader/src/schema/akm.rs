use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Annual Register (AKM) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various employment and occupation fields (nullable)
///
/// This schema matches the format in the actual parquet files
/// Converted from the Polars datatypes in schemas.py to Arrow datatypes
#[must_use]
pub fn akm_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("SOCIO", DataType::Int32, true),
        Field::new("SOCIO02", DataType::Int32, true),
        Field::new("SOCIO13", DataType::Int32, false),
        Field::new("CPRTJEK", DataType::Int32, true),
        Field::new("CPRTYPE", DataType::Int32, true),
        Field::new("VERSION", DataType::Utf8, true),
        Field::new("SENR", DataType::Utf8, true),
    ])
}

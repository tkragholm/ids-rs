use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Individual Register (IND) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various individual income and socioeconomic fields (nullable)
///
/// This schema matches the format in the actual parquet files
/// Converted from the Polars datatypes in schemas.py to Arrow datatypes
#[must_use]
pub fn ind_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("BESKST13", DataType::Int32, true),
        Field::new("LOENMV_13", DataType::Float64, true),
        Field::new("PERINDKIALT_13", DataType::Float64, true),
        Field::new("PRE_SOCIO", DataType::Int32, true),
    ])
}

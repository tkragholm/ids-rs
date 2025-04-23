use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Education Register (UDDF) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various education fields (nullable)
///
/// This schema matches the format in the actual parquet files
/// Converted from the Polars datatypes in schemas.py to Arrow datatypes
#[must_use]
pub fn uddf_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("HFAUDD", DataType::Utf8, true),
        Field::new("HF_VFRA", DataType::Date32, true),
        Field::new("HF_VTIL", DataType::Date32, true),
        Field::new("INSTNR", DataType::Int32, true),
    ])
}

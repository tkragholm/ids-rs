use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Population Register (BEF) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various demographic fields (nullable)
///
/// This schema matches the format in the actual parquet files
/// Converted from the Polars datatypes in schemas.py to Arrow datatypes
#[must_use]
pub fn bef_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("AEGTE_ID", DataType::Utf8, true),
        Field::new("ALDER", DataType::Utf8, false),
        Field::new("ANTBOERNF", DataType::Int32, true),
        Field::new("ANTBOERNH", DataType::Int32, true),
        Field::new("ANTPERSF", DataType::Int32, true),
        Field::new("ANTPERSH", DataType::Int32, true),
        Field::new("BOP_VFRA", DataType::Date32, true),
        Field::new("CIVST", DataType::Utf8, true),
        Field::new("FAMILIE_ID", DataType::Utf8, true),
        Field::new("FAMILIE_TYPE", DataType::Int32, true),
        Field::new("FAR_ID", DataType::Utf8, true),
        Field::new("FOED_DAG", DataType::Date32, false),
        Field::new("KOEN", DataType::Utf8, false),
        Field::new("KOM", DataType::Int32, true),
        Field::new("MOR_ID", DataType::Utf8, true),
        Field::new("STATSB", DataType::Utf8, true),
    ])
}
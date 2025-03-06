use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Population Register (BEF) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various demographic fields (nullable)
#[must_use]
pub fn bef_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("FOED_DAG", DataType::Date32, true),
        Field::new("KOEN", DataType::Int32, true),
        Field::new("STATSB", DataType::Int32, true),
        Field::new("IE_TYPE", DataType::Int32, true),
        Field::new("KOM", DataType::Int32, true),
        Field::new("BOPIKOM", DataType::Int32, true),
        Field::new("CIVST", DataType::Int32, true),
        Field::new("FAMILIE_ID", DataType::Utf8, true),
        Field::new("FAMILIE_TYPE", DataType::Int32, true),
        Field::new("HUSTYPE", DataType::Int32, true),
        Field::new("HUSSTAND_ID", DataType::Utf8, true),
        // Include additional fields as needed
    ])
}
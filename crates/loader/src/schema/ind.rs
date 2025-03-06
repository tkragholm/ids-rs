use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Individual Register (IND) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various individual income and socioeconomic fields (nullable)
#[must_use]
pub fn ind_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("SOCIO13", DataType::Int32, true),
        Field::new("PERINDKIALT", DataType::Float64, true),
        Field::new("LOENMV", DataType::Float64, true),
        Field::new("ERHVERVSINDK", DataType::Float64, true),
        Field::new("KORSTOETT", DataType::Float64, true),
        Field::new("NETFORMUE", DataType::Float64, true),
        Field::new("AFIGUALT", DataType::Int32, true),
        // Include additional fields as needed
    ])
}
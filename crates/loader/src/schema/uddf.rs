use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Education Register (UDDF) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various education fields (nullable)
#[must_use]
pub fn uddf_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("HFAUDD", DataType::Int32, true),
        Field::new("HFPRIA", DataType::Int32, true),
        Field::new("IGUDD", DataType::Int32, true),
        Field::new("KILDE", DataType::Int32, true),
        Field::new("AUDD", DataType::Int32, true),
        Field::new("AUDD_NAVN", DataType::Utf8, true),
        Field::new("UDD_START", DataType::Date32, true),
        Field::new("UDD_SLUT", DataType::Date32, true),
        // Include additional fields as needed
    ])
}
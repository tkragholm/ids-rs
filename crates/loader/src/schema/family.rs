use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Family Relations data
///
/// # Fields
/// - `PERSON_PNR`: Personal identifier of the person (non-nullable)
/// - `RELATION_PNR`: Personal identifier of the related person (non-nullable)
/// - `RELATION_TYPE`: Type of relation (non-nullable)
#[must_use]
pub fn family_schema() -> Schema {
    Schema::new(vec![
        Field::new("PERSON_PNR", DataType::Utf8, false),
        Field::new("RELATION_PNR", DataType::Utf8, false),
        Field::new("RELATION_TYPE", DataType::Int32, false),
    ])
}
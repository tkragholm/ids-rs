use arrow_schema::{Field, Schema};

/// Convert a Schema to a vector of Fields
///
/// This is a utility function for when you need to work with the fields
/// directly, rather than the schema as a whole.
///
/// # Arguments
/// * `schema` - The schema to convert
///
/// # Returns
/// A vector of Field objects from the schema
#[must_use]
#[allow(dead_code)]
pub fn schema_to_fields(schema: &Schema) -> Vec<Field> {
    schema.fields().iter().map(|f| f.as_ref().clone()).collect()
}

/// Check if a schema contains a specific field
///
/// # Arguments
/// * `schema` - The schema to check
/// * `field_name` - The name of the field to look for
///
/// # Returns
/// True if the field exists, false otherwise
#[must_use]
#[allow(dead_code)]
pub fn schema_has_field(schema: &Schema, field_name: &str) -> bool {
    schema.field_with_name(field_name).is_ok()
}

/// Get the position of a field in a schema
///
/// # Arguments
/// * `schema` - The schema to check
/// * `field_name` - The name of the field to look for
///
/// # Returns
/// The position of the field, or None if it doesn't exist
#[must_use]
#[allow(dead_code)]
pub fn schema_field_position(schema: &Schema, field_name: &str) -> Option<usize> {
    schema.fields().iter().position(|f| f.name() == field_name)
}

//! AKM schema definitions

use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

/// Get the Arrow schema for AKM data
#[must_use] pub fn akm_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("DATE", DataType::Date32, false),
        Field::new("STILLING", DataType::Utf8, true),
        Field::new("BRANCHE", DataType::Utf8, true),
        Field::new("SOCIO", DataType::Utf8, true),
        Field::new("TIMETAL", DataType::Float64, true),
        Field::new("ERHVERVSINDKOMST", DataType::Float64, true),
    ])
}

/// Get the Arrow schema for AKM data as an Arc
#[must_use] pub fn akm_schema_arc() -> Arc<Schema> {
    Arc::new(akm_schema())
}
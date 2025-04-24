//! UDDF schema definitions

use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

/// Get the Arrow schema for UDDF data
#[must_use] pub fn uddf_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("DATE", DataType::Date32, false),
        Field::new("INSTITUTION", DataType::Utf8, true),
        Field::new("UDDANNELSESKODE", DataType::Utf8, true),
        Field::new("STATUS", DataType::Utf8, true),
        Field::new("NIVEAU", DataType::Utf8, true),
    ])
}

/// Get the Arrow schema for UDDF data as an Arc
#[must_use] pub fn uddf_schema_arc() -> Arc<Schema> {
    Arc::new(uddf_schema())
}
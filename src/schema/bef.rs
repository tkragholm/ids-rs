//! BEF schema definitions

use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

/// Get the Arrow schema for BEF data
#[must_use] pub fn bef_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("DATE", DataType::Date32, false),
        Field::new("STATSB", DataType::Utf8, true),
        Field::new("KOMMUNE", DataType::Utf8, true),
        Field::new("CIVILSTAND", DataType::Utf8, true),
        Field::new("HUSTYPE", DataType::Utf8, true),
        Field::new("FAMTYPE", DataType::Utf8, true),
    ])
}

/// Get the Arrow schema for BEF data as an Arc
#[must_use] pub fn bef_schema_arc() -> Arc<Schema> {
    Arc::new(bef_schema())
}
//! IND schema definitions

use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

/// Get the Arrow schema for IND data
#[must_use] pub fn ind_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("DATE", DataType::Date32, false),
        Field::new("INDKOMST_PERSONLIG", DataType::Float64, true),
        Field::new("INDKOMST_FAMILIE", DataType::Float64, true),
        Field::new("FORMUE", DataType::Float64, true),
        Field::new("SKAT", DataType::Float64, true),
    ])
}

/// Get the Arrow schema for IND data as an Arc
#[must_use] pub fn ind_schema_arc() -> Arc<Schema> {
    Arc::new(ind_schema())
}
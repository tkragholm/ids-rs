//! IDAN schema definitions

use arrow::datatypes::{DataType, Field, Schema};
use std::sync::Arc;

/// Get the Arrow schema for IDAN data
#[must_use] pub fn idan_schema() -> Schema {
    Schema::new(vec![
        Field::new("ARBGNR", DataType::Utf8, true),
        Field::new("ARBNR", DataType::Int8, true),
        Field::new("CPRTJEK", DataType::Int8, true),
        Field::new("CPRTYPE", DataType::Int8, true),
        Field::new("CVRNR", DataType::Utf8, true),
        Field::new("JOBKAT", DataType::Int8, true),
        Field::new("JOBLON", DataType::Float64, true),
        Field::new("LBNR", DataType::Utf8, true),
        Field::new("PNR", DataType::Utf8, false),
        Field::new("STILL", DataType::Utf8, true),
        Field::new("TILKNYT", DataType::Int8, true),
    ])
}

/// Get the Arrow schema for IDAN data as an Arc
#[must_use] pub fn idan_schema_arc() -> Arc<Schema> {
    Arc::new(idan_schema())
}
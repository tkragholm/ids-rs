use arrow_schema::{DataType, Field, Schema};

/// Defines the schema for Annual Register (AKM) data
///
/// # Fields
/// - `PNR`: Unique personal identifier (non-nullable)
/// - Various employment and occupation fields (nullable)
#[must_use]
pub fn akm_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("BESKST13", DataType::Int32, true),
        Field::new("SOCIO13", DataType::Int32, true),
        Field::new("DISCO", DataType::Int32, true),
        Field::new("JOBTYP3", DataType::Int32, true),
        Field::new("STARTAAR", DataType::Int32, true),
        Field::new("STARTDATO", DataType::Date32, true),
        Field::new("SLUTAAR", DataType::Int32, true),
        Field::new("SLUTDATO", DataType::Date32, true),
        Field::new("ARB_TIMER", DataType::Float64, true),
        Field::new("BRUTTO_INDKM", DataType::Float64, true),
        Field::new("GGS_INDKM", DataType::Float64, true),
        Field::new("ARB_HHK", DataType::Int32, true),
        Field::new("BRANCHE3", DataType::Int32, true),
        Field::new("BRANCHENR_DB07", DataType::Int32, true),
        Field::new("DIST_KOEA", DataType::Float64, true),
        Field::new("JOBTYP1", DataType::Int32, true),
        Field::new("STILLING", DataType::Utf8, true),
        Field::new("TILKNYTKOEA", DataType::Int32, true),
        Field::new("LØN", DataType::Float64, true),
        // Legacy fields
        Field::new("SOCIO", DataType::Int32, true),
        Field::new("SOCIO02", DataType::Int32, true),
    ])
}
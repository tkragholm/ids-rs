//! LPR version 2 schema definitions
//!
//! This module contains schema definitions for the LPR version 2 registry.

use super::{default_lpr_metadata, LprSchema, LprVersion};
use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema};
use std::collections::HashMap;

/// LPR version 2 schema
pub struct Lpr2Schema;

impl RegistrySchema for Lpr2Schema {
    /// Get the main Arrow schema for LPR version 2
    /// This is a consolidated schema that includes all components
    fn schema() -> Schema {
        Schema::new(vec![
            // Admin fields
            Field::new("PNR", DataType::Utf8, false),
            Field::new("C_ADIAG", DataType::Utf8, true),
            Field::new("C_AFD", DataType::Utf8, true),
            Field::new("C_HAFD", DataType::Utf8, true),
            Field::new("C_HENM", DataType::Utf8, true),
            Field::new("C_HSGH", DataType::Utf8, true),
            Field::new("C_INDM", DataType::Utf8, true),
            Field::new("C_KOM", DataType::Utf8, true),
            Field::new("C_KONTAARS", DataType::Utf8, true),
            Field::new("C_PATTYPE", DataType::Utf8, true),
            Field::new("C_SGH", DataType::Utf8, true),
            Field::new("C_SPEC", DataType::Utf8, true),
            Field::new("C_UDM", DataType::Utf8, true),
            Field::new("CPRTJEK", DataType::Utf8, true),
            Field::new("CPRTYPE", DataType::Utf8, true),
            Field::new("D_HENDTO", DataType::Date32, true),
            Field::new("D_INDDTO", DataType::Date32, true),
            Field::new("D_UDDTO", DataType::Date32, true),
            Field::new("K_AFD", DataType::Utf8, true),
            Field::new("RECNUM", DataType::Utf8, true),
            Field::new("V_ALDDG", DataType::Int32, true),
            Field::new("V_ALDER", DataType::Int32, true),
            Field::new("V_INDMINUT", DataType::Int32, true),
            Field::new("V_INDTIME", DataType::Int32, true),
            Field::new("V_SENGDAGE", DataType::Int32, true),
            Field::new("V_UDTIME", DataType::Int32, true),
            Field::new("VERSION", DataType::Utf8, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec![
            "PNR",
            "C_ADIAG",
            "C_AFD",
            "C_HAFD",
            "C_HENM",
            "C_HSGH",
            "C_INDM",
            "C_KOM",
            "C_KONTAARS",
            "C_PATTYPE",
            "C_SGH",
            "C_SPEC",
            "C_UDM",
            "CPRTJEK",
            "CPRTYPE",
            "D_HENDTO",
            "D_INDDTO",
            "D_UDDTO",
            "K_AFD",
            "RECNUM",
            "V_ALDDG",
            "V_ALDER",
            "V_INDMINUT",
            "V_INDTIME",
            "V_SENGDAGE",
            "V_UDTIME",
            "VERSION",
        ]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        default_lpr_metadata(LprVersion::V2)
    }
}

impl LprSchema for Lpr2Schema {
    /// Get the LPR version for this schema
    fn version() -> LprVersion {
        LprVersion::V2
    }

    /// Get the schema for a specific component
    fn component_schema(component: &str) -> Option<Schema> {
        match component.to_lowercase().as_str() {
            "adm" => Some(lpr2_adm_schema()),
            "diag" => Some(lpr2_diag_schema()),
            "proc" => Some(lpr2_proc_schema()),
            _ => None,
        }
    }

    /// Get the component names for this LPR version
    fn component_names() -> Vec<&'static str> {
        vec!["adm", "diag", "proc"]
    }
}

/// Get the Arrow schema for LPR2 Admin (`LPR_ADM`) data
#[must_use] pub fn lpr2_adm_schema() -> Schema {
    Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("C_ADIAG", DataType::Utf8, true),
        Field::new("C_AFD", DataType::Utf8, true),
        Field::new("C_HAFD", DataType::Utf8, true),
        Field::new("C_HENM", DataType::Utf8, true),
        Field::new("C_HSGH", DataType::Utf8, true),
        Field::new("C_INDM", DataType::Utf8, true),
        Field::new("C_KOM", DataType::Utf8, true),
        Field::new("C_KONTAARS", DataType::Utf8, true),
        Field::new("C_PATTYPE", DataType::Utf8, true),
        Field::new("C_SGH", DataType::Utf8, true),
        Field::new("C_SPEC", DataType::Utf8, true),
        Field::new("C_UDM", DataType::Utf8, true),
        Field::new("CPRTJEK", DataType::Utf8, true),
        Field::new("CPRTYPE", DataType::Utf8, true),
        Field::new("D_HENDTO", DataType::Date32, true),
        Field::new("D_INDDTO", DataType::Date32, true),
        Field::new("D_UDDTO", DataType::Date32, true),
        Field::new("K_AFD", DataType::Utf8, true),
        Field::new("RECNUM", DataType::Utf8, true),
        Field::new("V_ALDDG", DataType::Int32, true),
        Field::new("V_ALDER", DataType::Int32, true),
        Field::new("V_INDMINUT", DataType::Int32, true),
        Field::new("V_INDTIME", DataType::Int32, true),
        Field::new("V_SENGDAGE", DataType::Int32, true),
        Field::new("V_UDTIME", DataType::Int32, true),
        Field::new("VERSION", DataType::Utf8, true),
    ])
}

/// Get the Arrow schema for LPR2 Diagnosis (`LPR_DIAG`) data
#[must_use] pub fn lpr2_diag_schema() -> Schema {
    Schema::new(vec![
        Field::new("C_DIAG", DataType::Utf8, true),
        Field::new("C_DIAGTYPE", DataType::Utf8, true),
        Field::new("C_TILDIAG", DataType::Utf8, true),
        Field::new("LEVERANCEDATO", DataType::Date32, true),
        Field::new("RECNUM", DataType::Utf8, true),
        Field::new("VERSION", DataType::Utf8, true),
    ])
}

/// Get the Arrow schema for LPR2 Procedure (`LPR_BES`) data
#[must_use] pub fn lpr2_proc_schema() -> Schema {
    Schema::new(vec![
        Field::new("C_KODE", DataType::Utf8, true),
        Field::new("C_ODIAG", DataType::Utf8, true),
        Field::new("C_PSGH", DataType::Utf8, true),
        Field::new("C_TILKODE", DataType::Utf8, true),
        Field::new("CPRTJEK", DataType::Utf8, true),
        Field::new("CPRTYPE", DataType::Utf8, true),
        Field::new("D_ODTO", DataType::Date32, true),
        Field::new("LEVERANCEDATO", DataType::Date32, true),
        Field::new("PNR", DataType::Utf8, false),
        Field::new("RECNUM", DataType::Utf8, true),
        Field::new("V_OMINUT", DataType::Int32, true),
        Field::new("V_OTIME", DataType::Int32, true),
        Field::new("VERSION", DataType::Utf8, true),
    ])
}

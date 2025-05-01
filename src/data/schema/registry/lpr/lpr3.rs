//! LPR version 3 schema definitions
//!
//! This module contains schema definitions for the LPR version 3 registry.

use super::{default_lpr_metadata, LprSchema, LprVersion};
use crate::data::schema::traits::RegistrySchema;
use arrow::datatypes::{DataType, Field, Schema, TimeUnit};
use std::collections::HashMap;

/// LPR version 3 schema
pub struct Lpr3Schema;

impl RegistrySchema for Lpr3Schema {
    /// Get the main Arrow schema for LPR version 3
    /// This is a consolidated schema that includes all components
    fn schema() -> Schema {
        Schema::new(vec![
            // Kontakter fields
            Field::new("CPR", DataType::Utf8, false),
            Field::new("SORENHED_IND", DataType::Utf8, true),
            Field::new("SORENHED_HEN", DataType::Utf8, true),
            Field::new("SORENHED_ANS", DataType::Utf8, true),
            Field::new("DW_EK_KONTAKT", DataType::Utf8, true),
            Field::new("DW_EK_FORLOEB", DataType::Utf8, true),
            Field::new("dato_start", DataType::Date32, true),
            Field::new("tidspunkt_start", DataType::Time32(TimeUnit::Second), true),
            Field::new("dato_slut", DataType::Date32, true),
            Field::new("tidspunkt_slut", DataType::Time32(TimeUnit::Second), true),
            Field::new("aktionsdiagnose", DataType::Utf8, true),
            Field::new("kontaktaarsag", DataType::Utf8, true),
            Field::new("prioritet", DataType::Utf8, true),
            Field::new("kontakttype", DataType::Utf8, true),
            Field::new("henvisningsaarsag", DataType::Utf8, true),
            Field::new("henvisningsmaade", DataType::Utf8, true),
            Field::new("dato_behandling_start", DataType::Date32, true),
            Field::new(
                "tidspunkt_behandling_start",
                DataType::Time32(TimeUnit::Second),
                true,
            ),
            Field::new("dato_indberetning", DataType::Date32, true),
            Field::new("lprindberetningssytem", DataType::Utf8, true),
        ])
    }

    /// Get column names for this schema
    fn column_names() -> Vec<&'static str> {
        vec![
            "CPR",
            "SORENHED_IND",
            "SORENHED_HEN",
            "SORENHED_ANS",
            "DW_EK_KONTAKT",
            "DW_EK_FORLOEB",
            "dato_start",
            "tidspunkt_start",
            "dato_slut",
            "tidspunkt_slut",
            "aktionsdiagnose",
            "kontaktaarsag",
            "prioritet",
            "kontakttype",
            "henvisningsaarsag",
            "henvisningsmaade",
            "dato_behandling_start",
            "tidspunkt_behandling_start",
            "dato_indberetning",
            "lprindberetningssytem",
        ]
    }

    /// Get default metadata for this schema
    fn default_metadata() -> HashMap<String, String> {
        default_lpr_metadata(LprVersion::V3)
    }
}

impl LprSchema for Lpr3Schema {
    /// Get the LPR version for this schema
    fn version() -> LprVersion {
        LprVersion::V3
    }

    /// Get the schema for a specific component
    fn component_schema(component: &str) -> Option<Schema> {
        match component.to_lowercase().as_str() {
            "kontakter" => Some(lpr3_kontakter_schema()),
            "diagnoser" => Some(lpr3_diagnoser_schema()),
            "procedurer" => Some(lpr3_procedurer_schema()),
            _ => None,
        }
    }

    /// Get the component names for this LPR version
    fn component_names() -> Vec<&'static str> {
        vec!["kontakter", "diagnoser", "procedurer"]
    }
}

/// Get the Arrow schema for LPR3 Kontakter data
#[must_use] pub fn lpr3_kontakter_schema() -> Schema {
    Schema::new(vec![
        Field::new("SORENHED_IND", DataType::Utf8, true),
        Field::new("SORENHED_HEN", DataType::Utf8, true),
        Field::new("SORENHED_ANS", DataType::Utf8, true),
        Field::new("DW_EK_KONTAKT", DataType::Utf8, true),
        Field::new("DW_EK_FORLOEB", DataType::Utf8, true),
        Field::new("CPR", DataType::Utf8, false),
        Field::new("dato_start", DataType::Date32, true),
        Field::new("tidspunkt_start", DataType::Time32(TimeUnit::Second), true),
        Field::new("dato_slut", DataType::Date32, true),
        Field::new("tidspunkt_slut", DataType::Time32(TimeUnit::Second), true),
        Field::new("aktionsdiagnose", DataType::Utf8, true),
        Field::new("kontaktaarsag", DataType::Utf8, true),
        Field::new("prioritet", DataType::Utf8, true),
        Field::new("kontakttype", DataType::Utf8, true),
        Field::new("henvisningsaarsag", DataType::Utf8, true),
        Field::new("henvisningsmaade", DataType::Utf8, true),
        Field::new("dato_behandling_start", DataType::Date32, true),
        Field::new(
            "tidspunkt_behandling_start",
            DataType::Time32(TimeUnit::Second),
            true,
        ),
        Field::new("dato_indberetning", DataType::Date32, true),
        Field::new("lprindberetningssytem", DataType::Utf8, true),
    ])
}

/// Get the Arrow schema for LPR3 Diagnoser data
#[must_use] pub fn lpr3_diagnoser_schema() -> Schema {
    Schema::new(vec![
        Field::new("DW_EK_KONTAKT", DataType::Utf8, true),
        Field::new("diagnosekode", DataType::Utf8, true),
        Field::new("diagnosetype", DataType::Utf8, true),
        Field::new("senere_afkraeftet", DataType::Utf8, true),
        Field::new("diagnosekode_parent", DataType::Utf8, true),
        Field::new("diagnosetype_parent", DataType::Utf8, true),
        Field::new("lprindberetningssystem", DataType::Utf8, true),
    ])
}

/// Get the Arrow schema for LPR3 Procedurer data
#[must_use] pub fn lpr3_procedurer_schema() -> Schema {
    Schema::new(vec![
        Field::new("DW_EK_KONTAKT", DataType::Utf8, true),
        Field::new("procedurekode", DataType::Utf8, true),
        Field::new("proceduretype", DataType::Utf8, true),
        Field::new("dato_procedure", DataType::Date32, true),
        Field::new(
            "tidspunkt_procedure",
            DataType::Time32(TimeUnit::Second),
            true,
        ),
        Field::new("producerende_enhed", DataType::Utf8, true),
        Field::new("procedureart", DataType::Utf8, true),
        Field::new("sideangivelse", DataType::Utf8, true),
        Field::new("handlingsspecifikation", DataType::Utf8, true),
        Field::new("kontakt_til_afslutning", DataType::Utf8, true),
        Field::new("supplerende_kode", DataType::Utf8, true),
    ])
}

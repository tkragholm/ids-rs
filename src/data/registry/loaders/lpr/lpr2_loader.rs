//! LPR version 2 registry loader
//!
//! This module contains registry loader for the LPR version 2 registry.

use crate::data::registry::loaders::lpr::{LprRegistry, LprVersion};
use crate::data::registry::traits::{PnrFilter, RegisterLoader};
use crate::data::schema::registry::lpr::lpr2::Lpr2Schema;
use crate::data::schema::registry::lpr::LprSchema;
use crate::data::schema::traits::RegistrySchema;
use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;
use std::path::Path;

/// LPR version 2 registry loader
pub struct Lpr2Register;

#[async_trait::async_trait]
impl RegisterLoader for Lpr2Register {
    type SchemaType = Lpr2Schema;

    /// Get the name of the register
    fn register_name(&self) -> &'static str {
        "LPR2"
    }

    /// Load records from the LPR v2 register
    ///
    /// # Arguments
    /// * `base_path` - Base directory containing the LPR v2 parquet files
    /// * `pnr_filter` - Optional filter to only load data for specific PNRs
    ///
    /// # Returns
    /// * `Result<Vec<RecordBatch>>` - Arrow record batches containing the loaded data
    async fn load(
        &self,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
    ) -> Result<Vec<RecordBatch>> {
        // Create a context
        let ctx = SessionContext::new();

        // Get schema
        let _schema = Self::SchemaType::schema_arc();

        // Find paths to LPR v2 files
        let paths = self.find_files(Path::new(base_path))?;

        // Verify the path exists
        let path = Path::new(base_path);
        if !path.exists() {
            return Err(IdsError::Validation(format!(
                "Path does not exist: {}",
                path.display()
            )));
        }

        // Register tables for each component
        if let Some(admin_path) = &paths.admin_path {
            let admin_schema = Self::SchemaType::component_schema("adm")
                .ok_or_else(|| IdsError::Validation("Failed to get admin schema".into()))?;

            ctx.register_parquet(
                "lpr2_adm",
                admin_path.to_string_lossy().as_ref(),
                ParquetReadOptions::default().schema(&admin_schema),
            )
            .await?;
        }

        if let Some(diag_path) = &paths.diag_path {
            let diag_schema = Self::SchemaType::component_schema("diag")
                .ok_or_else(|| IdsError::Validation("Failed to get diagnosis schema".into()))?;

            ctx.register_parquet(
                "lpr2_diag",
                diag_path.to_string_lossy().as_ref(),
                ParquetReadOptions::default().schema(&diag_schema),
            )
            .await?;
        }

        if let Some(proc_path) = &paths.proc_path {
            let proc_schema = Self::SchemaType::component_schema("proc")
                .ok_or_else(|| IdsError::Validation("Failed to get procedure schema".into()))?;

            ctx.register_parquet(
                "lpr2_proc",
                proc_path.to_string_lossy().as_ref(),
                ParquetReadOptions::default().schema(&proc_schema),
            )
            .await?;
        }

        // Join the tables if they exist
        let mut sql_parts = Vec::new();
        let table_name = self.register_name().to_lowercase();

        // Start with admin data
        sql_parts.push(format!(
            "CREATE OR REPLACE TABLE {table_name} AS SELECT * FROM lpr2_adm"
        ));

        // Apply PNR filter if provided
        if let Some(filter) = pnr_filter {
            if filter.is_direct_filter() {
                let pnrs_list = filter
                    .pnrs()
                    .iter()
                    .map(|p| format!("'{p}'"))
                    .collect::<Vec<_>>()
                    .join(",");

                if !pnrs_list.is_empty() {
                    sql_parts.push(format!(
                        "CREATE OR REPLACE TABLE {table_name} AS
                         SELECT * FROM {table_name}
                         WHERE PNR IN ({pnrs_list})"
                    ));
                }
            } else if let Some(relation_col) = filter.relation_column() {
                let pnrs_list = filter
                    .pnrs()
                    .iter()
                    .map(|p| format!("'{p}'"))
                    .collect::<Vec<_>>()
                    .join(",");

                if !pnrs_list.is_empty() {
                    sql_parts.push(format!(
                        "CREATE OR REPLACE TABLE {table_name} AS
                         SELECT * FROM {table_name}
                         WHERE {relation_col} IN ({pnrs_list})"
                    ));
                }
            }
        }

        // Execute all SQL statements
        for sql in sql_parts {
            ctx.sql(&sql).await?;
        }

        // Get the result and return
        let df = ctx.table(&table_name).await?;
        let result = df.collect().await?;
        Ok(result)
    }
}

impl LprRegistry for Lpr2Register {
    /// Get the LPR version
    fn version(&self) -> LprVersion {
        LprVersion::V2
    }
}

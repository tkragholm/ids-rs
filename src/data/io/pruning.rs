use crate::error::Result;
use arrow::array::ArrayRef;
use arrow::datatypes::SchemaRef;
use datafusion::catalog::memory::DataSourceExec;
use datafusion::common::DFSchema;
use datafusion::datasource::physical_plan::{FileScanConfig, ParquetExecBuilder};
use datafusion::datasource::TableProvider;
use datafusion::execution::context::SessionContext;
use datafusion::logical_expr::{col, lit, Expr};
use datafusion::physical_expr::create_physical_expr;
use datafusion::physical_expr::execution_props::ExecutionProps;
use datafusion::physical_optimizer::pruning::PruningPredicate;
use std::sync::Arc;

use datafusion::prelude::*;

use std::collections::{HashMap, HashSet};

/// Pruning statistics for efficient file filtering
pub struct RegistryPruningStatistics {
    schema: SchemaRef,
    min_values: HashMap<String, ArrayRef>,
    max_values: HashMap<String, ArrayRef>,
    row_counts: Option<ArrayRef>,
}

impl RegistryPruningStatistics {
    /// Create a new instance with schema
    pub fn new(schema: SchemaRef) -> Self {
        Self {
            schema,
            min_values: HashMap::new(),
            max_values: HashMap::new(),
            row_counts: None,
        }
    }

    /// Add min value statistics for a column
    pub fn with_min_value(mut self, column_name: &str, values: ArrayRef) -> Self {
        self.min_values.insert(column_name.to_string(), values);
        self
    }

    /// Add max value statistics for a column
    pub fn with_max_value(mut self, column_name: &str, values: ArrayRef) -> Self {
        self.max_values.insert(column_name.to_string(), values);
        self
    }

    /// Set row counts
    pub fn with_row_counts(mut self, row_counts: ArrayRef) -> Self {
        self.row_counts = Some(row_counts);
        self
    }
}

/// Create a pruning predicate for PNR filtering
pub fn create_pnr_pruning_predicate(
    pnrs: &HashSet<String>,
    schema: SchemaRef,
) -> Result<PruningPredicate> {
    if pnrs.is_empty() {
        // If no PNRs, return an expression that always evaluates to false
        let expr = lit(false);
        let df_schema = DFSchema::try_from(schema.as_ref().clone())?;
        let props = ExecutionProps::new();
        let physical_expr = create_physical_expr(&expr, &df_schema, &props)?;
        Ok(PruningPredicate::try_new(physical_expr, schema)?)
    } else {
        // Create IN expression for PNR
        let pnr_values: Vec<Expr> = pnrs.iter().map(|pnr| lit(pnr.clone())).collect();

        let expr = col("PNR").in_list(pnr_values, false);

        // Create pruning predicate
        let df_schema = DFSchema::try_from(schema.as_ref().clone())?;
        let props = ExecutionProps::new();
        let physical_expr = create_physical_expr(&expr, &df_schema, &props)?;

        Ok(PruningPredicate::try_new(physical_expr, schema)?)
    }
}

/// Create ParquetExec with pruning statistics
pub fn create_parquet_exec_with_pruning(
    config: FileScanConfig,
    statistics: RegistryPruningStatistics,
    predicate: Option<PruningPredicate>,
) -> Result<Arc<dyn TableProvider>> {
    let builder = ParquetExecBuilder::new(config);

    if let Some(predicate) = predicate {
        // Use the inner physical expression from the pruning predicate
        let physical_expr = predicate.inner().clone();
        Ok(Arc::new(builder.with_predicate(physical_expr).build()))
    } else {
        Ok(Arc::new(builder.build()))
    }
}

/// Create a SessionContext with a registered table using pruning
pub async fn create_context_with_pruning(
    path: &str,
    schema: SchemaRef,
    pnr_filter: Option<&HashSet<String>>,
    table_name: &str,
) -> Result<SessionContext> {
    let ctx = SessionContext::new();

    // Create read options with schema
    let read_options = ParquetReadOptions::default().schema(&schema);

    // Register the table
    ctx.register_parquet(table_name, path, read_options).await?;

    // Apply filter if provided
    if let Some(pnrs) = pnr_filter {
        if !pnrs.is_empty() {
            // Create PNR IN list for SQL
            let pnrs_list = pnrs
                .iter()
                .map(|p| format!("'{}'", p))
                .collect::<Vec<_>>()
                .join(",");

            let sql = format!(
                "
                CREATE OR REPLACE TABLE {table_name} AS
                SELECT * FROM {table_name}
                WHERE PNR IN ({pnrs_list})
            "
            );

            ctx.sql(&sql).await?;
        }
    }

    Ok(ctx)
}

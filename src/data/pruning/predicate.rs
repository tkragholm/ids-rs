use crate::error::{IdsError, Result};
use crate::data::filter::PnrFilter;
use arrow::datatypes::SchemaRef;
use datafusion::common::DFSchema;
use datafusion::logical_expr::{col, lit, Expr};
use datafusion::physical_expr::create_physical_expr;
use datafusion::physical_expr::execution_props::ExecutionProps;
use datafusion::physical_optimizer::pruning::PruningPredicate;
use std::collections::HashSet;

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

/// Create a pruning predicate from a filter
pub fn create_pruning_predicate(
    filter: &PnrFilter,
    schema: SchemaRef,
) -> Result<Option<PruningPredicate>> {
    // If filter is empty, return None
    if filter.pnrs().is_empty() {
        return Ok(None);
    }

    // Get the column name to filter on
    let column_name = if filter.is_direct_filter() {
        "PNR"
    } else if let Some(relation_col) = filter.relation_column() {
        relation_col
    } else {
        return Ok(None);
    };

    // Check if the column exists in the schema
    if !schema.fields().iter().any(|f| f.name() == column_name) {
        return Err(IdsError::Validation(format!(
            "Column {column_name} not found in schema"
        )));
    }

    // Create IN expression
    let pnr_values: Vec<Expr> = filter.pnrs().iter().map(|pnr| lit(pnr.clone())).collect();

    let expr = col(column_name).in_list(pnr_values, false);

    // Create pruning predicate
    let df_schema = DFSchema::try_from(schema.as_ref().clone())?;
    let props = ExecutionProps::new();
    let physical_expr = create_physical_expr(&expr, &df_schema, &props)?;

    Ok(Some(PruningPredicate::try_new(physical_expr, schema)?))
}

/// Create a SessionContext with a registered table using pruning
pub async fn create_context_with_pruning(
    path: &str,
    schema: SchemaRef,
    pnr_filter: Option<&PnrFilter>,
    table_name: &str,
) -> Result<datafusion::execution::context::SessionContext> {
    use datafusion::execution::context::{SessionConfig, SessionContext};
    use datafusion::prelude::*;
    
    let ctx = SessionContext::new_with_config(
        SessionConfig::new()
            .with_target_partitions(4)
            .with_batch_size(8192),
    );

    // Create read options with schema and page index for pruning
    let read_options = ParquetReadOptions::default().schema(schema.as_ref());

    // Register the table
    ctx.register_parquet(table_name, path, read_options).await?;

    // Apply filter if provided
    if let Some(filter) = pnr_filter {
        if let Some(expr) = filter.to_expr() {
            // Get the dataframe and apply filter
            let df = ctx.table(table_name).await?;
            let filtered_df = df.filter(expr)?;

            // Register the filtered dataframe
            ctx.register_table(table_name, filtered_df.into_view())?;
        }
    }

    Ok(ctx)
}
use crate::error::Result;
use datafusion::functions_aggregate::expr_fn::avg;
use datafusion::functions_aggregate::expr_fn::count;
use datafusion::functions_aggregate::expr_fn::max;
use datafusion::functions_aggregate::expr_fn::min;
use datafusion::functions_aggregate::expr_fn::sum;
use datafusion::prelude::*;

use super::TransformPipeline;

/// Create a group by and count transform
pub fn group_by_count(group_columns: &[&str]) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![count(lit(1)).alias("count")];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by and sum transform
pub fn group_by_sum(group_columns: &[&str], sum_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![sum(col(sum_column)).alias(&format!("sum_{}", sum_column))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by and average transform
pub fn group_by_avg(group_columns: &[&str], avg_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![avg(col(avg_column)).alias(&format!("avg_{}", avg_column))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by with multiple aggregations
pub fn group_by_multi_agg(
    group_columns: &[&str],
    agg_columns: &[(&str, &str)],
) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = agg_columns
        .iter()
        .map(|(op, col_name)| match *op {
            "sum" => sum(col(*col_name)).alias(&format!("sum_{}", col_name)),
            "avg" => avg(col(*col_name)).alias(&format!("avg_{}", col_name)),
            "min" => min(col(*col_name)).alias(&format!("min_{}", col_name)),
            "max" => max(col(*col_name)).alias(&format!("max_{}", col_name)),
            "count" => count(col(*col_name)).alias(&format!("count_{}", col_name)),
            _ => count(lit(1)).alias("count"),
        })
        .collect::<Vec<_>>();

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a window function transform
pub async fn window_function(
    ctx: &SessionContext,
    table_name: &str,
    window_function: &str,
    partition_by: &[&str],
    order_by: &[&str],
    alias: &str,
) -> Result<DataFrame> {
    let partition_clause = if partition_by.is_empty() {
        "".to_string()
    } else {
        format!("PARTITION BY {}", partition_by.join(", "))
    };

    let order_clause = if order_by.is_empty() {
        "".to_string()
    } else {
        format!("ORDER BY {}", order_by.join(", "))
    };

    let window_clause = format!(
        "{} OVER ({}{})",
        window_function,
        partition_clause,
        if !order_clause.is_empty() && !partition_clause.is_empty() {
            format!(" {}", order_clause)
        } else {
            order_clause
        }
    );

    let sql = format!(
        "SELECT *, {} AS {} FROM {}",
        window_clause, alias, table_name
    );

    Ok(ctx.sql(&sql).await?)
}

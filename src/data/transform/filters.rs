use super::TransformPipeline;
use crate::error::Result;
use datafusion::prelude::*;

/// Create a date range filter
pub fn date_range_filter(column: &str, start_date: &str, end_date: &str) -> TransformPipeline {
    TransformPipeline::new().add_filter(
        col(column)
            .gt_eq(lit(start_date))
            .and(col(column).lt_eq(lit(end_date))),
    )
}

/// Create a categorical value filter
pub fn categorical_filter(column: &str, values: &[&str]) -> TransformPipeline {
    let values = values
        .iter()
        .map(|v| lit(v.to_string()))
        .collect::<Vec<_>>();
    TransformPipeline::new().add_filter(col(column).in_list(values, false))
}

/// Create a numeric range filter
pub fn numeric_range_filter(column: &str, min: f64, max: f64) -> TransformPipeline {
    TransformPipeline::new()
        .add_filter(col(column).gt_eq(lit(min)).and(col(column).lt_eq(lit(max))))
}

/// Create a non-null filter for multiple columns
pub fn non_null_filter(columns: &[&str]) -> TransformPipeline {
    let mut pipeline = TransformPipeline::new();

    for column in columns {
        pipeline = pipeline.add_filter(col(*column).is_not_null());
    }

    pipeline
}

/// Create a custom SQL filter
pub async fn sql_filter(
    ctx: &SessionContext,
    table_name: &str,
    condition: &str,
) -> Result<DataFrame> {
    let sql = format!("SELECT * FROM {} WHERE {}", table_name, condition);
    Ok(ctx.sql(&sql).await?)
}

use super::TransformPipeline;
use datafusion::prelude::*;
use datafusion::functions::expr_fn::strpos;
use std::collections::HashSet;

/// Create a date range filter
#[must_use] pub fn date_range_filter(column: &str, start_date: &str, end_date: &str) -> TransformPipeline {
    TransformPipeline::new().add_filter(
        col(column)
            .gt_eq(lit(start_date))
            .and(col(column).lt_eq(lit(end_date))),
    )
}

/// Create a categorical value filter
#[must_use] pub fn categorical_filter(column: &str, values: &[&str]) -> TransformPipeline {
    let values = values
        .iter()
        .map(|v| lit((*v).to_string()))
        .collect::<Vec<_>>();
    TransformPipeline::new().add_filter(col(column).in_list(values, false))
}

/// Create a numeric range filter
#[must_use] pub fn numeric_range_filter(column: &str, min: f64, max: f64) -> TransformPipeline {
    TransformPipeline::new()
        .add_filter(col(column).gt_eq(lit(min)).and(col(column).lt_eq(lit(max))))
}

/// Create a non-null filter for multiple columns
#[must_use] pub fn non_null_filter(columns: &[&str]) -> TransformPipeline {
    let mut pipeline = TransformPipeline::new();

    for column in columns {
        pipeline = pipeline.add_filter(col(*column).is_not_null());
    }

    pipeline
}

/// Create a custom predicate filter using a function
pub fn predicate_filter<F>(f: F) -> TransformPipeline 
where 
    F: Fn() -> Expr + Send + Sync + 'static,
{
    TransformPipeline::new().add_filter(f())
}

/// Create a prefix filter (LIKE 'prefix%')
#[must_use] pub fn prefix_filter(column: &str, prefix: &str) -> TransformPipeline {
    let pattern = format!("{prefix}%");
    TransformPipeline::new().add_filter(
        col(column).like(lit(pattern))
    )
}

/// Create a contains filter (LIKE '%pattern%')
#[must_use] pub fn contains_filter(column: &str, pattern: &str) -> TransformPipeline {
    let pattern = format!("%{pattern}%");
    TransformPipeline::new().add_filter(
        col(column).like(lit(pattern))
    )
}

/// Create a substring filter (checking if the column contains a substring)
#[must_use] pub fn substring_filter(column: &str, substring: &str) -> TransformPipeline {
    // In DataFusion we can use strpos(column, substring) > 0
    TransformPipeline::new().add_filter(
        strpos(col(column), lit(substring.to_string())).gt(lit(0))
    )
}

/// Create a filter for values in a `HashSet`
#[must_use] pub fn hash_set_filter(column: &str, values: HashSet<String>) -> TransformPipeline {
    if values.is_empty() {
        return TransformPipeline::new();
    }
    
    let values_vec: Vec<Expr> = values
        .iter()
        .map(|v| lit(v.clone()))
        .collect();
    
    TransformPipeline::new().add_filter(col(column).in_list(values_vec, false))
}

/// Create a filter for multiple conditions with AND
#[must_use] pub fn and_filter(conditions: Vec<Expr>) -> TransformPipeline {
    if conditions.is_empty() {
        return TransformPipeline::new();
    }
    
    let combined = conditions.into_iter()
        .reduce(datafusion::prelude::Expr::and)
        .unwrap_or(lit(true));
    
    TransformPipeline::new().add_filter(combined)
}

/// Create a filter for multiple conditions with OR
#[must_use] pub fn or_filter(conditions: Vec<Expr>) -> TransformPipeline {
    if conditions.is_empty() {
        return TransformPipeline::new();
    }
    
    let combined = conditions.into_iter()
        .reduce(datafusion::prelude::Expr::or)
        .unwrap_or(lit(true));
    
    TransformPipeline::new().add_filter(combined)
}

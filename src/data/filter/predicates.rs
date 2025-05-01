use datafusion::prelude::*;
use datafusion::functions::expr_fn::strpos;
use std::collections::HashSet;

/// Create a date range filter
pub fn date_range_filter(column: &str, start_date: &str, end_date: &str) -> Expr {
    col(column)
        .gt_eq(lit(start_date))
        .and(col(column).lt_eq(lit(end_date)))
}

/// Create a categorical value filter
pub fn categorical_filter(column: &str, values: &[&str]) -> Expr {
    let values = values
        .iter()
        .map(|v| lit((*v).to_string()))
        .collect::<Vec<_>>();
    col(column).in_list(values, false)
}

/// Create a numeric range filter
pub fn numeric_range_filter(column: &str, min: f64, max: f64) -> Expr {
    col(column).gt_eq(lit(min)).and(col(column).lt_eq(lit(max)))
}

/// Create a non-null filter for multiple columns
pub fn non_null_filter(columns: &[&str]) -> Expr {
    let filters = columns
        .iter()
        .map(|column| col(*column).is_not_null())
        .collect::<Vec<_>>();
    
    and_filters(filters)
}

/// Create a prefix filter (LIKE 'prefix%')
pub fn prefix_filter(column: &str, prefix: &str) -> Expr {
    let pattern = format!("{prefix}%");
    col(column).like(lit(pattern))
}

/// Create a contains filter (LIKE '%pattern%')
pub fn contains_filter(column: &str, pattern: &str) -> Expr {
    let pattern = format!("%{pattern}%");
    col(column).like(lit(pattern))
}

/// Create a substring filter (checking if the column contains a substring)
pub fn substring_filter(column: &str, substring: &str) -> Expr {
    // In DataFusion we can use strpos(column, substring) > 0
    strpos(col(column), lit(substring.to_string())).gt(lit(0))
}

/// Create a filter for values in a `HashSet`
pub fn hash_set_filter(column: &str, values: HashSet<String>) -> Expr {
    if values.is_empty() {
        return lit(true); // Always true if no values
    }
    
    let values_vec: Vec<Expr> = values
        .iter()
        .map(|v| lit(v.clone()))
        .collect();
    
    col(column).in_list(values_vec, false)
}

/// Combine multiple filters with AND
pub fn and_filters(conditions: Vec<Expr>) -> Expr {
    if conditions.is_empty() {
        return lit(true);
    }
    
    conditions.into_iter()
        .reduce(datafusion::prelude::Expr::and)
        .unwrap_or(lit(true))
}

/// Combine multiple filters with OR
pub fn or_filters(conditions: Vec<Expr>) -> Expr {
    if conditions.is_empty() {
        return lit(true);
    }
    
    conditions.into_iter()
        .reduce(datafusion::prelude::Expr::or)
        .unwrap_or(lit(true))
}
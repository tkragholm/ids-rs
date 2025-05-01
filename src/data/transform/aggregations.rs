use datafusion::functions_aggregate::expr_fn::{
    // Basic aggregations
    avg, count, max, min, sum, count_distinct,
    // Statistical measures
    stddev, stddev_pop, 
    // Variance calculations
    var_pop, var_sample,
    // Percentiles and distribution
    approx_median, approx_percentile_cont,
    // Boolean operations
    bool_and, bool_or,
    // Array operations
    array_agg,
    // Correlation and covariance
    corr, covar_pop, covar_samp,
    // First/last values
    first_value, last_value
};
use datafusion::prelude::*;

use super::TransformPipeline;

/// Create a group by and count transform
#[must_use] pub fn group_by_count(group_columns: &[&str]) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![count(lit(1)).alias("count")];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by and sum transform
#[must_use] pub fn group_by_sum(group_columns: &[&str], sum_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![sum(col(sum_column)).alias(format!("sum_{sum_column}"))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by and average transform
#[must_use] pub fn group_by_avg(group_columns: &[&str], avg_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![avg(col(avg_column)).alias(format!("avg_{avg_column}"))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by with multiple aggregations
#[must_use] pub fn group_by_multi_agg(
    group_columns: &[&str],
    agg_columns: &[(&str, &str)],
) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = agg_columns
        .iter()
        .map(|(op, col_name)| match *op {
            "sum" => sum(col(*col_name)).alias(format!("sum_{col_name}")),
            "avg" => avg(col(*col_name)).alias(format!("avg_{col_name}")),
            "min" => min(col(*col_name)).alias(format!("min_{col_name}")),
            "max" => max(col(*col_name)).alias(format!("max_{col_name}")),
            "count" => count(col(*col_name)).alias(format!("count_{col_name}")),
            _ => count(lit(1)).alias("count"),
        })
        .collect::<Vec<_>>();

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by with count distinct aggregation
#[must_use] pub fn group_by_count_distinct(group_columns: &[&str], count_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![count_distinct(col(count_column)).alias(format!("count_distinct_{count_column}"))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by with standard deviation aggregation
#[must_use] pub fn group_by_stddev(group_columns: &[&str], stddev_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![stddev(col(stddev_column)).alias(format!("stddev_{stddev_column}"))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by with population standard deviation aggregation
#[must_use] pub fn group_by_stddev_pop(group_columns: &[&str], stddev_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![stddev_pop(col(stddev_column)).alias(format!("stddev_pop_{stddev_column}"))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by with variance aggregation (population)
#[must_use] pub fn group_by_var_pop(group_columns: &[&str], var_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![var_pop(col(var_column)).alias(format!("var_pop_{var_column}"))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a group by with variance aggregation (sample)
#[must_use] pub fn group_by_var_sample(group_columns: &[&str], var_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![var_sample(col(var_column)).alias(format!("var_sample_{var_column}"))];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a percentile aggregation
#[must_use] pub fn group_by_percentile(
    group_columns: &[&str], 
    value_column: &str,
    percentile: f64,
) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![
        approx_percentile_cont(col(value_column), lit(percentile), None)
            .alias(format!("percentile_{percentile}_of_{value_column}"))
    ];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a median aggregation (using approximation)
#[must_use] pub fn group_by_median(
    group_columns: &[&str], 
    value_column: &str,
) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![
        approx_median(col(value_column)).alias(format!("median_{value_column}"))
    ];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a custom aggregation function
pub fn group_by_custom<F>(
    group_columns: &[&str],
    agg_fn: F,
    alias: &str,
) -> TransformPipeline 
where 
    F: Fn() -> Expr + Send + Sync + 'static,
{
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![agg_fn().alias(alias)];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a transform that calculates statistics for a column
#[must_use] pub fn column_statistics(column: &str) -> TransformPipeline {
    let agg_exprs = vec![
        count(col(column)).alias(format!("count_{column}")),
        min(col(column)).alias(format!("min_{column}")),
        max(col(column)).alias(format!("max_{column}")),
        avg(col(column)).alias(format!("avg_{column}")),
        stddev(col(column)).alias(format!("stddev_{column}")),
        var_sample(col(column)).alias(format!("var_sample_{column}")),
        approx_median(col(column)).alias(format!("median_{column}")),
    ];

    TransformPipeline::new().add_aggregate(vec![], agg_exprs)
}

/// Create a transform that calculates correlation between two columns
#[must_use] pub fn correlation(x_column: &str, y_column: &str) -> TransformPipeline {
    let agg_exprs = vec![
        corr(col(x_column), col(y_column)).alias(format!("corr_{x_column}_{y_column}")),
    ];

    TransformPipeline::new().add_aggregate(vec![], agg_exprs)
}

/// Create a transform that calculates covariance between two columns
#[must_use] pub fn covariance(x_column: &str, y_column: &str, population: bool) -> TransformPipeline {
    let agg_exprs = if population {
        vec![
            covar_pop(col(x_column), col(y_column)).alias(format!("covar_pop_{x_column}_{y_column}")),
        ]
    } else {
        vec![
            covar_samp(col(x_column), col(y_column)).alias(format!("covar_samp_{x_column}_{y_column}")),
        ]
    };

    TransformPipeline::new().add_aggregate(vec![], agg_exprs)
}

/// Create a transform that aggregates values into an array
#[must_use] pub fn array_aggregation(group_columns: &[&str], array_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![
        array_agg(col(array_column)).alias(format!("array_{array_column}")),
    ];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a transform that calculates first and last values in a group
#[must_use] pub fn group_first_last(group_columns: &[&str], value_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![
        first_value(col(value_column), None).alias(format!("first_{value_column}")),
        last_value(col(value_column), None).alias(format!("last_{value_column}")),
    ];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

/// Create a transform that does boolean aggregation (ALL or ANY)
#[must_use] pub fn boolean_aggregation(group_columns: &[&str], bool_column: &str) -> TransformPipeline {
    let group_exprs = group_columns.iter().map(|c| col(*c)).collect::<Vec<_>>();
    let agg_exprs = vec![
        bool_and(col(bool_column)).alias(format!("all_{bool_column}")),
        bool_or(col(bool_column)).alias(format!("any_{bool_column}")),
    ];

    TransformPipeline::new().add_aggregate(group_exprs, agg_exprs)
}

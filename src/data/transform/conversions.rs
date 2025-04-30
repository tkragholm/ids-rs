use super::TransformPipeline;
use arrow::compute::DatePart;
use datafusion::prelude::*;

/// Create a transform that adds a year column from a date column
pub fn add_year_column(date_column: &str, year_column: &str) -> TransformPipeline {
    // Using date_part function instead of extract method
    let date_column_owned = date_column.to_string();
    let year_column_owned = year_column.to_string();
    TransformPipeline::new().add_operation(move |df| {
        Ok(df.with_column(&year_column_owned, date_part(lit("year"), col(&date_column_owned)))?)
    })
}

/// Create a transform that adds a month column from a date column
pub fn add_month_column(date_column: &str, month_column: &str) -> TransformPipeline {
    // Using date_part function instead of extract method
    let date_column_owned = date_column.to_string();
    let month_column_owned = month_column.to_string();
    TransformPipeline::new().add_operation(move |df| {
        Ok(df.with_column(&month_column_owned, date_part(lit("month"), col(&date_column_owned)))?)
    })
}

/// Create a transform that maps categorical values
pub fn map_categorical_values(
    column: &str,
    mapping: &[(&str, &str)],
    new_column: Option<&str>,
) -> TransformPipeline {
    let column_str = column.to_string();
    let output_column = new_column.unwrap_or(column).to_string();
    let case_expr = mapping.iter().fold(
        // Updated for DataFusion 47.0.0
        Expr::Case(
            datafusion::logical_expr::Case {
                expr: None,
                when_then_expr: vec![],
                else_expr: Some(Box::new(col(&column_str))),
            }
        ),
        |acc, (from, to)| {
            let mut when_then = match acc {
                Expr::Case(case) => case.when_then_expr,
                _ => vec![],
            };

            when_then.push((
                Box::new(col(&column_str).eq(lit(from.to_string()))),
                Box::new(lit(to.to_string())),
            ));

            // Updated for DataFusion 47.0.0
            Expr::Case(
                datafusion::logical_expr::Case {
                    expr: None,
                    when_then_expr: when_then,
                    else_expr: Some(Box::new(col(&column_str))),
                }
            )
        },
    );

    TransformPipeline::new()
        .add_operation(move |df| Ok(df.with_column(&output_column, case_expr.clone())?))
}

/// Create a transform that scales numeric values
pub fn scale_numeric_values(
    column: &str,
    scale_factor: f64,
    new_column: Option<&str>,
) -> TransformPipeline {
    let output_column = new_column.unwrap_or(column).to_string();

    // Clone the string to address lifetime issues
    let column_owned = column.to_string();
    TransformPipeline::new().add_operation(move |df| {
        Ok(df.with_column(&output_column, col(&column_owned) * lit(scale_factor))?)
    })
}

/// Create a transform that converts date format
pub fn convert_date_format(
    column: &str,
    from_format: &str,
    to_format: &str,
    new_column: Option<&str>,
) -> TransformPipeline {
    let output_column = new_column.unwrap_or(column).to_string();
    let from_format = from_format.to_string();
    let _to_format = to_format.to_string();

    // Clone the string to address lifetime issues
    let column_owned = column.to_string();
    TransformPipeline::new().add_operation(move |df| {
        // First parse the string to a date
        // Updated for DataFusion 47.0.0
        let parsed = date_trunc(
            lit("second"), // Use string literal for part name
            to_timestamp(vec![col(&column_owned), lit(from_format.clone())])
        );

        // TODO: The date_format function has changed or is no longer available in DataFusion 47.0.0
        // Need to find the correct function to format a timestamp as a string
        // For now, just return the parsed timestamp
        Ok(df.with_column(&output_column, parsed)?)
    })
}

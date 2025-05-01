use super::TransformPipeline;
use datafusion::prelude::*;
use datafusion::functions::math::expr_fn;

/// Create a transform that adds a year column from a date column
#[must_use] pub fn add_year_column(date_column: &str, year_column: &str) -> TransformPipeline {
    // Using date_part function instead of extract method
    let date_column_owned = date_column.to_string();
    let year_column_owned = year_column.to_string();
    TransformPipeline::new().add_operation(move |df| {
        Ok(df.with_column(
            &year_column_owned,
            date_part(lit("year"), col(&date_column_owned)),
        )?)
    })
}

/// Create a transform that adds a month column from a date column
#[must_use] pub fn add_month_column(date_column: &str, month_column: &str) -> TransformPipeline {
    // Using date_part function instead of extract method
    let date_column_owned = date_column.to_string();
    let month_column_owned = month_column.to_string();
    TransformPipeline::new().add_operation(move |df| {
        Ok(df.with_column(
            &month_column_owned,
            date_part(lit("month"), col(&date_column_owned)),
        )?)
    })
}

/// Create a transform that maps categorical values
#[must_use] pub fn map_categorical_values(
    column: &str,
    mapping: &[(&str, &str)],
    new_column: Option<&str>,
) -> TransformPipeline {
    let column_str = column.to_string();
    let output_column = new_column.unwrap_or(column).to_string();
    let case_expr = mapping.iter().fold(
        // Updated for DataFusion 47.0.0
        Expr::Case(datafusion::logical_expr::Case {
            expr: None,
            when_then_expr: vec![],
            else_expr: Some(Box::new(col(&column_str))),
        }),
        |acc, (from, to)| {
            let mut when_then = match acc {
                Expr::Case(case) => case.when_then_expr,
                _ => vec![],
            };

            when_then.push((
                Box::new(col(&column_str).eq(lit((*from).to_string()))),
                Box::new(lit((*to).to_string())),
            ));

            // Updated for DataFusion 47.0.0
            Expr::Case(datafusion::logical_expr::Case {
                expr: None,
                when_then_expr: when_then,
                else_expr: Some(Box::new(col(&column_str))),
            })
        },
    );

    TransformPipeline::new()
        .add_operation(move |df| Ok(df.with_column(&output_column, case_expr.clone())?))
}

/// Create a transform that scales numeric values
#[must_use] pub fn scale_numeric_values(
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
#[must_use] pub fn convert_date_format(
    column: &str,
    from_format: &str,
    new_column: Option<&str>,
) -> TransformPipeline {
    let output_column = new_column.unwrap_or(column).to_string();
    let from_format = from_format.to_string();

    // Clone the string to address lifetime issues
    let column_owned = column.to_string();
    TransformPipeline::new().add_operation(move |df| {
        // Parse the string to a date
        let parsed = to_timestamp(vec![col(&column_owned), lit(from_format.clone())]);
        Ok(df.with_column(&output_column, parsed)?)
    })
}

/// Create a transform that adds a day component from a date
#[must_use] pub fn add_day_column(date_column: &str, day_column: &str) -> TransformPipeline {
    let date_column_owned = date_column.to_string();
    let day_column_owned = day_column.to_string();
    TransformPipeline::new().add_operation(move |df| {
        Ok(df.with_column(
            &day_column_owned,
            date_part(lit("day"), col(&date_column_owned)),
        )?)
    })
}

/// Create a transform that adds a date components (year, month, day) from a date
#[must_use] pub fn add_date_components(date_column: &str) -> TransformPipeline {
    let date_column_owned = date_column.to_string();

    TransformPipeline::new().add_operation(move |df| {
        let with_year = df.with_column(
            &format!("{date_column_owned}_year"),
            date_part(lit("year"), col(&date_column_owned)),
        )?;

        let with_month = with_year.with_column(
            &format!("{date_column_owned}_month"),
            date_part(lit("month"), col(&date_column_owned)),
        )?;

        let with_day = with_month.with_column(
            &format!("{date_column_owned}_day"),
            date_part(lit("day"), col(&date_column_owned)),
        )?;

        Ok(with_day)
    })
}

/// Extract weekday (1-7) from a date
#[must_use] pub fn extract_weekday(date_column: &str, weekday_column: &str) -> TransformPipeline {
    let date_column_owned = date_column.to_string();
    let weekday_column_owned = weekday_column.to_string();

    TransformPipeline::new().add_operation(move |df| {
        Ok(df.with_column(
            &weekday_column_owned,
            date_part(lit("dow"), col(&date_column_owned)),
        )?)
    })
}

/// Convert between units (e.g. hours to minutes, etc.)
#[must_use] pub fn convert_units(
    column: &str,
    conversion_factor: f64,
    new_column: Option<&str>,
) -> TransformPipeline {
    scale_numeric_values(column, conversion_factor, new_column)
}

/// Round numeric values to a specified number of decimal places
#[must_use] pub fn round_numeric_values(
    column: &str,
    decimal_places: i64,
    new_column: Option<&str>,
) -> TransformPipeline {
    let output_column = new_column.unwrap_or(column).to_string();
    let column_owned = column.to_string();

    TransformPipeline::new().add_operation(move |df| {
        // Use the round function directly
        let round_expr = expr_fn::round(vec![col(&column_owned)]);
        
        // If decimal places is specified, we need to do additional transformation
        let expr = if decimal_places != 0 {
            let multiplier = 10_f64.powi(decimal_places as i32);
            // (round(column * 10^n)) / 10^n
            expr_fn::round(vec![col(&column_owned) * lit(multiplier)]) / lit(multiplier)
        } else {
            round_expr
        };

        Ok(df.with_column(&output_column, expr)?)
    })
}

/// Truncate numeric values toward zero
#[must_use] pub fn truncate_numeric_values(
    column: &str,
    decimal_places: i64,
    new_column: Option<&str>,
) -> TransformPipeline {
    let output_column = new_column.unwrap_or(column).to_string();
    let column_owned = column.to_string();

    TransformPipeline::new().add_operation(move |df| {
        // If decimal places is 0, just use trunc function directly
        let expr = if decimal_places == 0 {
            // In DataFusion 47.0, trunc takes a Vec<Expr> not individual arguments
            expr_fn::trunc(vec![col(&column_owned)])
        } else {
            // We need to account for the decimal places, but must use a vector
            // Since trunc only takes a single argument (Vec<Expr>) in DataFusion 47.0,
            // we need an alternative approach for decimal places
            let multiplier = 10_f64.powi(decimal_places as i32);
            // Use alternative approach: trunc(column * 10^n) / 10^n
            expr_fn::trunc(vec![col(&column_owned) * lit(multiplier)]) / lit(multiplier)
        };

        Ok(df.with_column(&output_column, expr)?)
    })
}

/// Calculate age from a birth date column and reference date
#[must_use] pub fn calculate_age(
    birth_date_column: &str,
    reference_date: &str,
    age_column: &str,
) -> TransformPipeline {
    let birth_date_column_owned = birth_date_column.to_string();
    let reference_date_owned = reference_date.to_string();
    let age_column_owned = age_column.to_string();

    TransformPipeline::new().add_operation(move |df| {
        // Convert reference date to a literal timestamp if it's not already a column
        let ref_date = if df
            .schema()
            .fields()
            .iter()
            .any(|f| f.name() == &reference_date_owned)
        {
            col(&reference_date_owned)
        } else {
            // Try to parse the reference date as a timestamp
            lit(reference_date_owned.clone())
        };

        // Calculate the age using date_part extractions
        // We'll use a simple approximation: extract year from ref_date - extract year from birth_date
        let birth_year = date_part(lit("year"), col(&birth_date_column_owned));
        let ref_year = date_part(lit("year"), ref_date);

        Ok(df.with_column(&age_column_owned, ref_year - birth_year)?)
    })
}

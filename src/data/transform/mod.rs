use crate::error::Result;
use arrow::record_batch::RecordBatch;
use datafusion::prelude::*;

pub mod aggregations;
pub mod conversions;
pub mod filters;
pub mod joins;

pub use aggregations::*;
pub use conversions::*;
pub use filters::*;
pub use joins::*;

/// Transform pipeline for composable data transformations
pub struct TransformPipeline {
    operations: Vec<Box<dyn Fn(DataFrame) -> Result<DataFrame> + Send + Sync>>,
}

impl TransformPipeline {
    /// Create a new transform pipeline
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add a filter to the pipeline
    pub fn add_filter(mut self, expr: Expr) -> Self {
        self.operations
            .push(Box::new(move |df| Ok(df.filter(expr.clone())?)));
        self
    }

    /// Add a select operation to the pipeline
    pub fn add_select(mut self, columns: Vec<&str>) -> Self {
        let columns = columns.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        self.operations.push(Box::new(move |df| {
            Ok(df.select(columns.iter().map(|c| col(c)).collect::<Vec<_>>())?)
        }));
        self
    }

    /// Add an aggregate operation to the pipeline
    pub fn add_aggregate(mut self, group_by: Vec<Expr>, aggregates: Vec<Expr>) -> Self {
        self.operations.push(Box::new(move |df| {
            Ok(df.aggregate(group_by.clone(), aggregates.clone())?)
        }));
        self
    }

    /// Add a custom operation to the pipeline
    pub fn add_operation<F>(mut self, op: F) -> Self
    where
        F: Fn(DataFrame) -> Result<DataFrame> + Send + Sync + 'static,
    {
        self.operations.push(Box::new(op));
        self
    }

    /// Apply the pipeline to a DataFrame
    pub fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        let mut current_df = df;

        for op in &self.operations {
            current_df = op(current_df)?;
        }

        Ok(current_df)
    }

    /// Apply the pipeline to a SessionContext and table
    pub async fn apply_to_context(
        &self,
        ctx: &SessionContext,
        table_name: &str,
    ) -> Result<DataFrame> {
        let df = ctx.table(table_name).await?;
        self.apply(df)
    }

    /// Execute the pipeline and collect the results
    pub async fn execute(
        &self,
        ctx: &SessionContext,
        table_name: &str,
    ) -> Result<Vec<RecordBatch>> {
        let df = self.apply_to_context(ctx, table_name).await?;
        Ok(df.collect().await?)
    }
}

/// Create a transform that filters by date range and adds a year column
pub fn date_range_transform(start_date: &str, end_date: &str) -> TransformPipeline {
    TransformPipeline::new()
        .add_filter(
            col("DATE")
                .gt_eq(lit(start_date))
                .and(col("DATE").lt_eq(lit(end_date))),
        )
        .add_operation(|df| {
            Ok(df.with_column(
                "YEAR",
                // Updated for DataFusion 47.0.0
                date_part(lit("YEAR"), col("DATE")).alias("YEAR"),
            )?)
        })
}

/// Create a transform that filters out rows with missing values in specified columns
pub fn filter_missing_values(columns: &[&str]) -> TransformPipeline {
    let mut pipeline = TransformPipeline::new();

    for column in columns {
        pipeline = pipeline.add_filter(col(*column).is_not_null());
    }

    pipeline
}

/// Create a transform that joins two tables
pub async fn join_transform(
    ctx: &SessionContext,
    left_table: &str,
    right_table: &str,
    join_type: JoinType,
    join_keys: Vec<&str>,
) -> Result<DataFrame> {
    let left_df = ctx.table(left_table).await?;
    let right_df = ctx.table(right_table).await?;

    let join_expr = join_keys
        .iter()
        .map(|k| col(&format!("{}.{}", left_table, k)).eq(col(&format!("{}.{}", right_table, k))))
        .reduce(|acc, expr| acc.and(expr))
        .unwrap_or(lit(true));

    Ok(left_df.join(right_df, join_type, &[], &[], Some(join_expr))?)
}

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

impl Default for TransformPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl TransformPipeline {
    /// Create a new transform pipeline
    #[must_use] pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add a filter to the pipeline
    #[must_use] pub fn add_filter(mut self, expr: Expr) -> Self {
        self.operations
            .push(Box::new(move |df| Ok(df.filter(expr.clone())?)));
        self
    }

    /// Add a select operation to the pipeline
    #[must_use] pub fn add_select(mut self, columns: Vec<&str>) -> Self {
        let columns = columns.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        self.operations.push(Box::new(move |df| {
            Ok(df.select(columns.iter().map(col))?)
        }));
        self
    }

    /// Add an aggregate operation to the pipeline
    #[must_use] pub fn add_aggregate(mut self, group_by: Vec<Expr>, aggregates: Vec<Expr>) -> Self {
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
    
    /// Add a sort operation to the pipeline
    #[must_use] pub fn add_sort(mut self, exprs: Vec<Expr>, ascending: Option<Vec<bool>>) -> Self {
        self.operations.push(Box::new(move |df| {
            let sort_exprs = match &ascending {
                Some(asc_values) => {
                    if exprs.len() != asc_values.len() {
                        return Err(crate::error::IdsError::Validation(
                            "Number of sort expressions must match number of ascending flags".to_string()
                        ));
                    }
                    
                    exprs.iter()
                        .zip(asc_values.iter())
                        .map(|(expr, &asc)| expr.clone().sort(asc, false)) // false = NULLS LAST
                        .collect()
                },
                None => {
                    // Default to ascending (true) and NULLS LAST (false)
                    exprs.iter()
                        .map(|expr| expr.clone().sort(true, false))
                        .collect()
                }
            };
            
            Ok(df.sort(sort_exprs)?)
        }));
        self
    }
    
    /// Add a limit operation to the pipeline
    #[must_use] pub fn add_limit(mut self, limit: usize) -> Self {
        self.operations.push(Box::new(move |df| {
            Ok(df.limit(0, Some(limit))?)
        }));
        self
    }
    
    /// Add a join with another table to the pipeline
    #[must_use] pub fn add_join(
        mut self, 
        right_table: &str, 
        join_type: JoinType, 
        left_cols: &[&str], 
        right_cols: &[&str]
    ) -> Self {
        let _right_table = right_table.to_string();
        let left_cols = left_cols.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        let right_cols = right_cols.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        let _join_type = join_type;
        
        self.operations.push(Box::new(move |_df| {
            // Since we can't directly access the session context from a DataFrame in DataFusion 47.0.0,
            // we'll use a simpler approach for handling joins that doesn't rely on session context.
            
            // First create join conditions using column references
            let mut join_exprs = vec![];
            for (l, r) in left_cols.iter().zip(right_cols.iter()) {
                join_exprs.push(col(l).eq(col(r)));
            }
            
            // Combine join conditions with AND
            let _join_expr = join_exprs.into_iter().reduce(datafusion::prelude::Expr::and)
                .unwrap_or(lit(true));
                
            // For this implementation, we'll assume the right DataFrame is passed as part of the operation
            // in a follow-up step, since we cannot directly get the session context to fetch the table.
            // 
            // The user will need to create a separate function to perform the actual join after getting 
            // both DataFrames:
            //
            // fn perform_join(left_df: DataFrame, right_df: DataFrame, join_type: JoinType, join_expr: Expr) -> Result<DataFrame> {
            //     Ok(left_df.join(right_df, join_type, &[], &[], Some(join_expr))?)
            // }
            
            // For now, we'll return a message explaining this limitation
            Err(crate::error::IdsError::Validation("Direct table joining in TransformPipeline requires modifying the code to handle async context. \
                Please implement a custom operation that fetches both DataFrames and joins them.".to_string()))
        }));
        self
    }

    /// Apply the pipeline to a `DataFrame`
    pub fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        let mut current_df = df;

        for op in &self.operations {
            current_df = op(current_df)?;
        }

        Ok(current_df)
    }

    /// Apply the pipeline to a `SessionContext` and table
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
    
    /// Save the pipeline results to a parquet file
    pub async fn save_to_parquet(
        &self,
        ctx: &SessionContext,
        table_name: &str,
        output_path: &str,
    ) -> Result<()> {
        let df = self.apply_to_context(ctx, table_name).await?;
        // Use DataFusion 47.0.0 API for write_parquet
        let write_options = datafusion::dataframe::DataFrameWriteOptions::default();
        df.write_parquet(output_path, write_options, None).await?;
        Ok(())
    }
}

/// Helper function to build a join condition from a list of column pairs
/// Type parameter Expr is used to make the fold operation type-safe
fn build_join_condition(conditions: Vec<(Expr, Expr)>) -> Expr {
    conditions
        .into_iter()
        .map(|(left, right)| left.eq(right))
        .fold(None::<Expr>, |acc, expr| match acc {
            Some(accumulated) => Some(accumulated.and(expr)),
            None => Some(expr),
        })
        .unwrap_or(lit(true))
}

/// Create a transform that filters by date range and adds a year column
#[must_use] pub fn date_range_transform(start_date: &str, end_date: &str) -> TransformPipeline {
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
#[must_use] pub fn filter_missing_values(columns: &[&str]) -> TransformPipeline {
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
        .map(|k| col(format!("{left_table}.{k}")).eq(col(format!("{right_table}.{k}"))))
        .reduce(datafusion::prelude::Expr::and)
        .unwrap_or(lit(true));

    Ok(left_df.join(right_df, join_type, &[], &[], Some(join_expr))?)
}

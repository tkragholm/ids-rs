use crate::data::filter::FilterBuilder;
use crate::error::{IdsError, Result};

use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;

use datafusion::execution::context::SessionContext;
use datafusion::functions_aggregate::expr_fn::count;
use datafusion::prelude::*;

use std::sync::Arc;

pub mod aggregations;
pub mod conversions;
pub mod joins;

pub use aggregations::*;
pub use conversions::*;
pub use joins::*;

/// Async operation that can be applied to a DataFrame
#[async_trait::async_trait]
pub trait AsyncDataFrameOperation: Send + Sync {
    /// Apply the operation to a DataFrame
    async fn apply(&self, df: DataFrame) -> Result<DataFrame>;

    /// Get the name of this operation
    fn name(&self) -> &str;
}

/// Async transform pipeline for composable data transformations
pub struct AsyncTransformPipeline {
    operations: Vec<Box<dyn AsyncDataFrameOperation>>,
}

impl Default for AsyncTransformPipeline {
    fn default() -> Self {
        Self::new()
    }
}

impl AsyncTransformPipeline {
    /// Create a new async transform pipeline
    #[must_use]
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add an operation to the pipeline
    pub fn add_operation<T: AsyncDataFrameOperation + 'static>(mut self, op: T) -> Self {
        self.operations.push(Box::new(op));
        self
    }

    /// Add a filter operation to the pipeline
    #[must_use]
    pub fn add_filter(self, expr: Expr) -> Self {
        self.add_operation(FilterOperation::new(expr))
    }

    /// Add a filter builder to the pipeline
    #[must_use]
    pub fn add_filter_builder(self, builder: FilterBuilder) -> Self {
        self.add_filter(builder.build_and())
    }

    /// Add a select operation to the pipeline
    #[must_use]
    pub fn add_select(self, columns: Vec<String>) -> Self {
        self.add_operation(SelectOperation::new(columns))
    }

    /// Add a select with str list to the pipeline
    #[must_use]
    pub fn add_select_str(self, columns: &[&str]) -> Self {
        let columns = columns.iter().map(|s| (*s).to_string()).collect();
        self.add_select(columns)
    }

    /// Add an aggregate operation to the pipeline
    #[must_use]
    pub fn add_aggregate(self, group_by: Vec<Expr>, aggregates: Vec<Expr>) -> Self {
        self.add_operation(AggregateOperation::new(group_by, aggregates))
    }

    /// Add an aggregation with column names as strings
    #[must_use]
    pub fn add_aggregate_str(self, group_by: &[&str], aggregates: Vec<Expr>) -> Self {
        let group_by_exprs = group_by.iter().map(|s| col(*s)).collect();
        self.add_aggregate(group_by_exprs, aggregates)
    }

    /// Add a count aggregation
    #[must_use]
    pub fn add_count(self, group_by: &[&str]) -> Self {
        let aggregates = vec![count(lit(1)).alias("count")];
        self.add_aggregate_str(group_by, aggregates)
    }

    /// Add a sort operation to the pipeline
    #[must_use]
    pub fn add_sort(self, exprs: Vec<Expr>, ascending: Option<Vec<bool>>) -> Self {
        self.add_operation(SortOperation::new(exprs, ascending))
    }

    /// Add a sort operation using column names
    #[must_use]
    pub fn add_sort_str(self, columns: &[&str], ascending: Option<Vec<bool>>) -> Self {
        let exprs = columns.iter().map(|s| col(*s)).collect();
        self.add_sort(exprs, ascending)
    }

    /// Add a simple ascending sort on one column
    #[must_use]
    pub fn add_sort_asc(self, column: &str) -> Self {
        self.add_sort_str(&[column], Some(vec![true]))
    }

    /// Add a simple descending sort on one column
    #[must_use]
    pub fn add_sort_desc(self, column: &str) -> Self {
        self.add_sort_str(&[column], Some(vec![false]))
    }

    /// Add a limit operation to the pipeline
    #[must_use]
    pub fn add_limit(self, limit: usize) -> Self {
        self.add_operation(LimitOperation::new(limit))
    }

    /// Add a join operation to the pipeline
    #[must_use]
    pub fn add_join(
        self,
        right_table: String,
        join_type: JoinType,
        left_cols: Vec<String>,
        right_cols: Vec<String>,
    ) -> Self {
        self.add_operation(JoinOperation::new(
            right_table,
            join_type,
            left_cols,
            right_cols,
        ))
    }

    /// Add a join operation with str lists to the pipeline
    #[must_use]
    pub fn add_join_str(
        self,
        right_table: &str,
        join_type: JoinType,
        left_cols: &[&str],
        right_cols: &[&str],
    ) -> Self {
        let right_table = right_table.to_string();
        let left_cols = left_cols.iter().map(|s| (*s).to_string()).collect();
        let right_cols = right_cols.iter().map(|s| (*s).to_string()).collect();
        self.add_join(right_table, join_type, left_cols, right_cols)
    }

    /// Apply the pipeline to a `DataFrame` asynchronously
    pub async fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        let mut current_df = df;

        for op in &self.operations {
            current_df = op.apply(current_df).await?;
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
        self.apply(df).await
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
        df.write_parquet(
            output_path,
            datafusion::dataframe::DataFrameWriteOptions::default(),
            None,
        )
        .await?;
        Ok(())
    }
}

/// Transform pipeline for synchronous data transformations
///
/// This is provided for backward compatibility but it's recommended
/// to use the `AsyncTransformPipeline` for most operations.
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
    #[must_use]
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    /// Add a filter to the pipeline
    #[must_use]
    pub fn add_filter(mut self, expr: Expr) -> Self {
        self.operations
            .push(Box::new(move |df| Ok(df.filter(expr.clone())?)));
        self
    }

    /// Add a filter builder to the pipeline
    #[must_use]
    pub fn add_filter_builder(self, builder: FilterBuilder) -> Self {
        self.add_filter(builder.build_and())
    }

    /// Add a select operation to the pipeline
    #[must_use]
    pub fn add_select(mut self, columns: Vec<&str>) -> Self {
        let columns = columns.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
        self.operations
            .push(Box::new(move |df| Ok(df.select(columns.iter().map(col))?)));
        self
    }

    /// Add an aggregate operation to the pipeline
    #[must_use]
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

    /// Add a sort operation to the pipeline
    #[must_use]
    pub fn add_sort(mut self, exprs: Vec<Expr>, ascending: Option<Vec<bool>>) -> Self {
        self.operations.push(Box::new(move |df| {
            let sort_exprs = match &ascending {
                Some(asc_values) => {
                    if exprs.len() != asc_values.len() {
                        return Err(crate::error::IdsError::Validation(
                            "Number of sort expressions must match number of ascending flags"
                                .to_string(),
                        ));
                    }

                    exprs
                        .iter()
                        .zip(asc_values.iter())
                        .map(|(expr, &asc)| expr.clone().sort(asc, false)) // false = NULLS LAST
                        .collect()
                }
                None => {
                    // Default to ascending (true) and NULLS LAST (false)
                    exprs
                        .iter()
                        .map(|expr| expr.clone().sort(true, false))
                        .collect()
                }
            };

            Ok(df.sort(sort_exprs)?)
        }));
        self
    }

    /// Add a limit operation to the pipeline
    #[must_use]
    pub fn add_limit(mut self, limit: usize) -> Self {
        self.operations
            .push(Box::new(move |df| Ok(df.limit(0, Some(limit))?)));
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
        df.write_parquet(
            output_path,
            datafusion::dataframe::DataFrameWriteOptions::default(),
            None,
        )
        .await?;
        Ok(())
    }

    /// Convert to an async transform pipeline
    #[must_use]
    pub fn to_async(self) -> AsyncTransformPipeline {
        let mut pipeline = AsyncTransformPipeline::new();

        for (i, op) in self.operations.into_iter().enumerate() {
            pipeline = pipeline.add_operation(SyncOperationWrapper {
                op,
                name: format!("sync_op_{i}"),
            });
        }

        pipeline
    }
}

/// Adapter to use sync operation in async pipeline
struct SyncOperationWrapper<F>
where
    F: Fn(DataFrame) -> Result<DataFrame> + Send + Sync,
{
    op: F,
    name: String,
}

#[async_trait::async_trait]
impl<F> AsyncDataFrameOperation for SyncOperationWrapper<F>
where
    F: Fn(DataFrame) -> Result<DataFrame> + Send + Sync + 'static,
{
    async fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        (self.op)(df)
    }

    fn name(&self) -> &str {
        &self.name
    }
}

/// Helper function to build a join condition from a list of column pairs
/// Type parameter Expr is used to make the fold operation type-safe
#[allow(dead_code)]
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
#[must_use]
pub fn date_range_transform(start_date: &str, end_date: &str) -> AsyncTransformPipeline {
    // Create a filter builder for the date range
    let filter_builder = FilterBuilder::new().with_date_range("DATE", start_date, end_date);

    // Create year extraction operation
    let year_column =
        AddColumnOperation::new("YEAR", date_part(lit("YEAR"), col("DATE")).alias("YEAR"));

    AsyncTransformPipeline::new()
        .add_filter_builder(filter_builder)
        .add_operation(year_column)
}

/// Create a transform that filters out rows with missing values in specified columns
#[must_use]
pub fn filter_missing_values(columns: &[&str]) -> AsyncTransformPipeline {
    // Create a filter builder for non-null columns
    let filter_builder = FilterBuilder::new().with_non_null(columns);

    // Apply the filter builder to the pipeline
    AsyncTransformPipeline::new().add_filter_builder(filter_builder)
}

/// Create an empty dataframe with the given schema
pub async fn empty_dataframe(ctx: &SessionContext, schema: SchemaRef) -> Result<DataFrame> {
    // Create an empty record batch
    let empty_batch = RecordBatch::new_empty(schema.clone());

    // Create a memory table directly with the empty batch
    let provider = datafusion::datasource::MemTable::try_new(schema, vec![vec![empty_batch]])?;

    // Create a dataframe directly without registering a table
    let df = ctx.read_table(Arc::new(provider))?;

    Ok(df)
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

    let joined_df = left_df.join(right_df, join_type, &[], &[], Some(join_expr))?;
    Ok(joined_df)
}

/// Operation that filters a `DataFrame`
pub struct FilterOperation {
    expr: Expr,
}

impl FilterOperation {
    /// Create a new filter operation
    #[must_use]
    pub fn new(expr: Expr) -> Self {
        Self { expr }
    }
}

#[async_trait::async_trait]
impl AsyncDataFrameOperation for FilterOperation {
    async fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        Ok(df.filter(self.expr.clone())?)
    }

    fn name(&self) -> &'static str {
        "filter"
    }
}

/// Operation that selects columns from a `DataFrame`
pub struct SelectOperation {
    columns: Vec<String>,
}

impl SelectOperation {
    /// Create a new select operation
    #[must_use]
    pub fn new(columns: Vec<String>) -> Self {
        Self { columns }
    }
}

#[async_trait::async_trait]
impl AsyncDataFrameOperation for SelectOperation {
    async fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        Ok(df.select(self.columns.iter().map(col))?)
    }

    fn name(&self) -> &'static str {
        "select"
    }
}

/// Operation that aggregates a `DataFrame`
pub struct AggregateOperation {
    group_by: Vec<Expr>,
    aggregates: Vec<Expr>,
}

impl AggregateOperation {
    /// Create a new aggregate operation
    #[must_use]
    pub fn new(group_by: Vec<Expr>, aggregates: Vec<Expr>) -> Self {
        Self {
            group_by,
            aggregates,
        }
    }
}

#[async_trait::async_trait]
impl AsyncDataFrameOperation for AggregateOperation {
    async fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        Ok(df.aggregate(self.group_by.clone(), self.aggregates.clone())?)
    }

    fn name(&self) -> &'static str {
        "aggregate"
    }
}

/// Operation that sorts a `DataFrame`
pub struct SortOperation {
    exprs: Vec<Expr>,
    ascending: Option<Vec<bool>>,
}

impl SortOperation {
    /// Create a new sort operation
    #[must_use]
    pub fn new(exprs: Vec<Expr>, ascending: Option<Vec<bool>>) -> Self {
        Self { exprs, ascending }
    }
}

#[async_trait::async_trait]
impl AsyncDataFrameOperation for SortOperation {
    async fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        let sort_exprs = match &self.ascending {
            Some(asc_values) => {
                if self.exprs.len() != asc_values.len() {
                    return Err(IdsError::Validation(
                        "Number of sort expressions must match number of ascending flags"
                            .to_string(),
                    ));
                }

                self.exprs
                    .iter()
                    .zip(asc_values.iter())
                    .map(|(expr, &asc)| expr.clone().sort(asc, false)) // false = NULLS LAST
                    .collect()
            }
            None => {
                // Default to ascending (true) and NULLS LAST (false)
                self.exprs
                    .iter()
                    .map(|expr| expr.clone().sort(true, false))
                    .collect()
            }
        };

        Ok(df.sort(sort_exprs)?)
    }

    fn name(&self) -> &'static str {
        "sort"
    }
}

/// Operation that limits a `DataFrame`
pub struct LimitOperation {
    limit: usize,
}

impl LimitOperation {
    /// Create a new limit operation
    #[must_use]
    pub fn new(limit: usize) -> Self {
        Self { limit }
    }
}

#[async_trait::async_trait]
impl AsyncDataFrameOperation for LimitOperation {
    async fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        Ok(df.limit(0, Some(self.limit))?)
    }

    fn name(&self) -> &'static str {
        "limit"
    }
}

/// Operation that joins a `DataFrame` with another table
#[allow(dead_code)]
pub struct JoinOperation {
    right_table: String,
    join_type: JoinType,
    left_cols: Vec<String>,
    right_cols: Vec<String>,
}

impl JoinOperation {
    /// Create a new join operation
    #[must_use]
    pub fn new(
        right_table: String,
        join_type: JoinType,
        left_cols: Vec<String>,
        right_cols: Vec<String>,
    ) -> Self {
        Self {
            right_table,
            join_type,
            left_cols,
            right_cols,
        }
    }
}

#[async_trait::async_trait]
impl AsyncDataFrameOperation for JoinOperation {
    async fn apply(&self, _df: DataFrame) -> Result<DataFrame> {
        // Get the session context from the dataframe
        // DataFusion 47.0.0 compatibility - we can't recover context from DataFrame
        // This is just a stub - the JoinOperation requires modification for DataFusion 47.0.0
        // We'll need to update this to accept a context reference in the constructor
        Err(IdsError::Validation("JoinOperation requires a context reference for DataFusion 47.0.0. Operation needs to be modified.".to_string()))

        // Notes: Implementation would look like:
        // 1. Get the right dataframe: ctx.table(&self.right_table).await?
        // 2. Create join conditions for each pair of columns
        // 3. Combine conditions with AND
        // 4. Perform join: df.join(right_df, self.join_type, &[], &[], Some(join_expr))?
    }

    fn name(&self) -> &'static str {
        "join"
    }
}

/// Operation that adds a column to a `DataFrame`
pub struct AddColumnOperation {
    name: String,
    expr: Expr,
}

impl AddColumnOperation {
    /// Create a new add column operation
    pub fn new(name: impl Into<String>, expr: Expr) -> Self {
        Self {
            name: name.into(),
            expr,
        }
    }
}

#[async_trait::async_trait]
impl AsyncDataFrameOperation for AddColumnOperation {
    async fn apply(&self, df: DataFrame) -> Result<DataFrame> {
        Ok(df.with_column(&self.name, self.expr.clone())?)
    }

    fn name(&self) -> &'static str {
        "add_column"
    }
}

/// Operation that adds SQL execution to the pipeline

#[allow(dead_code)]
pub struct SqlOperation {
    sql: String,
    temp_table_name: String,
}

impl SqlOperation {
    /// Create a new SQL operation
    pub fn new(sql: impl Into<String>, temp_table_name: impl Into<String>) -> Self {
        Self {
            sql: sql.into(),
            temp_table_name: temp_table_name.into(),
        }
    }
}

#[async_trait::async_trait]
impl AsyncDataFrameOperation for SqlOperation {
    async fn apply(&self, _df: DataFrame) -> Result<DataFrame> {
        // DataFusion 47.0.0 compatibility - we can't recover context from DataFrame
        // This is just a stub - the SqlOperation requires modification for DataFusion 47.0.0
        // We'll need to update this to accept a context reference in the constructor
        Err(IdsError::Validation("SqlOperation requires a context reference for DataFusion 47.0.0. Operation needs to be modified.".to_string()))

        // Notes: Implementation would look like:
        // 1. Register the input dataframe: ctx.register_table(&self.temp_table_name, df.into_view())?
        // 2. Execute SQL: ctx.sql(&self.sql).await?
        // 3. Return result DataFrame
    }

    fn name(&self) -> &'static str {
        "sql"
    }
}

use crate::data::registry::traits::PnrFilter;
use crate::data::registry::traits::RegisterLoader;
use crate::error::{IdsError, Result};
use arrow::record_batch::RecordBatch;
use datafusion::execution::context::SessionContext;
use datafusion::logical_expr::LogicalPlanBuilder;
use datafusion::prelude::*;
use std::collections::HashMap;

/// SQL query engine for registry data
pub struct RegistrySqlEngine {
    ctx: SessionContext,
    registered_tables: HashMap<String, String>,
}

impl Default for RegistrySqlEngine {
    fn default() -> Self {
        Self::new()
    }
}

impl RegistrySqlEngine {
    /// Create a new SQL engine
    #[must_use] pub fn new() -> Self {
        Self {
            ctx: SessionContext::new(),
            registered_tables: HashMap::new(),
        }
    }

    /// Register a registry as a table
    pub async fn register_registry<R: RegisterLoader>(
        &mut self,
        loader: &R,
        base_path: &str,
        pnr_filter: Option<&PnrFilter>,
        table_alias: Option<&str>,
    ) -> Result<()> {
        // Create a context for this registry
        let _ctx = loader.create_context(base_path, pnr_filter).await?;

        // Get the default table name
        let table_name = loader.register_name().to_lowercase();

        // Register with alias if provided
        let final_name = table_alias.unwrap_or(&table_name);

        // Register the table in our context
        // In DataFusion, we need to convert the DataFrame to a TableProvider
        // The current approach is to use the SQL API to create a view
        let create_view_sql = format!(
            "CREATE OR REPLACE VIEW {final_name} AS SELECT * FROM {table_name}"
        );

        // Execute the SQL to create the view
        self.ctx.sql(&create_view_sql).await?;

        // Store the mapping
        self.registered_tables
            .insert(final_name.to_string(), base_path.to_string());

        Ok(())
    }

    /// Execute a SQL query
    pub async fn execute_sql(&self, query: &str) -> Result<Vec<RecordBatch>> {
        let df = self.ctx.sql(query).await?;
        Ok(df.collect().await?)
    }

    /// Execute a SQL query and return a `DataFrame`
    pub async fn query_sql(&self, query: &str) -> Result<DataFrame> {
        Ok(self.ctx.sql(query).await?)
    }

    /// Get list of registered tables
    #[must_use] pub fn registered_tables(&self) -> Vec<String> {
        self.registered_tables.keys().cloned().collect()
    }

    /// Get the `DataFusion` context
    #[must_use] pub const fn context(&self) -> &SessionContext {
        &self.ctx
    }

    /// Create a new query builder
    #[must_use] pub fn query_builder(&self) -> QueryBuilder {
        QueryBuilder::new(self.ctx.clone())
    }
}

/// Query builder for fluent API
pub struct QueryBuilder {
    ctx: SessionContext,
    plan_builder: Option<LogicalPlanBuilder>,
}

impl QueryBuilder {
    /// Create a new query builder
    #[must_use] pub const fn new(ctx: SessionContext) -> Self {
        Self {
            ctx,
            plan_builder: None,
        }
    }

    /// Start building a query from a table
    pub async fn from(&mut self, table_name: &str) -> Result<&mut Self> {
        let df = self.ctx.table(table_name).await?;
        self.plan_builder = Some(LogicalPlanBuilder::from(df.into_optimized_plan()?));
        Ok(self)
    }

    /// Add a filter to the query
    pub fn filter(&mut self, expr: Expr) -> Result<&mut Self> {
        if let Some(builder) = &mut self.plan_builder {
            self.plan_builder = Some(builder.clone().filter(expr)?);
            Ok(self)
        } else {
            Err(IdsError::Validation(
                "No query plan started. Call from() first".to_string(),
            ))
        }
    }

    /// Add a projection to the query
    pub fn select(&mut self, exprs: Vec<Expr>) -> Result<&mut Self> {
        if let Some(builder) = &mut self.plan_builder {
            self.plan_builder = Some(builder.clone().project(exprs)?);
            Ok(self)
        } else {
            Err(IdsError::Validation(
                "No query plan started. Call from() first".to_string(),
            ))
        }
    }

    /// Add grouping to the query
    pub fn group_by(&mut self, group_by: Vec<Expr>, aggregates: Vec<Expr>) -> Result<&mut Self> {
        if let Some(builder) = &mut self.plan_builder {
            self.plan_builder = Some(builder.clone().aggregate(group_by, aggregates)?);
            Ok(self)
        } else {
            Err(IdsError::Validation(
                "No query plan started. Call from() first".to_string(),
            ))
        }
    }

    /// Add a limit to the query
    pub fn limit(&mut self, limit: usize) -> Result<&mut Self> {
        if let Some(builder) = &mut self.plan_builder {
            self.plan_builder = Some(builder.clone().limit(0, Some(limit))?);
            Ok(self)
        } else {
            Err(IdsError::Validation(
                "No query plan started. Call from() first".to_string(),
            ))
        }
    }

    /// Add sorting to the query
    pub fn sort(&mut self, exprs: Vec<Expr>) -> Result<&mut Self> {
        if let Some(builder) = &mut self.plan_builder {
            // Create sorted expressions with proper order information
            // Default to ascending (true) and NULLS LAST (false)
            let sort_exprs: Vec<_> = exprs
                .into_iter()
                .map(|expr| expr.sort(true, false))
                .collect();

            self.plan_builder = Some(builder.clone().sort(sort_exprs)?);
            Ok(self)
        } else {
            Err(IdsError::Validation(
                "No query plan started. Call from() first".to_string(),
            ))
        }
    }

    /// Build the query into a `DataFrame`
    pub fn build(&self) -> Result<DataFrame> {
        if let Some(builder) = &self.plan_builder {
            let plan = builder.clone().build()?;
            // Use state() to get the SessionState from SessionContext
            Ok(DataFrame::new(self.ctx.state(), plan))
        } else {
            Err(IdsError::Validation(
                "No query plan started. Call from() first".to_string(),
            ))
        }
    }

    /// Execute the query and return results
    pub async fn execute(&self) -> Result<Vec<RecordBatch>> {
        let df = self.build()?;
        Ok(df.collect().await?)
    }
}

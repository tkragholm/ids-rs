use crate::error::{IdsError, Result};
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::datasource::listing::PartitionedFile;
use datafusion::datasource::object_store::ObjectStoreUrl;
use datafusion::datasource::physical_plan::{FileScanConfigBuilder, ParquetSource};
use datafusion::datasource::source::DataSourceExec;
use datafusion::datasource::{MemTable, TableProvider};
use datafusion::execution::context::{SessionConfig, SessionContext};
use datafusion::physical_optimizer::pruning::PruningPredicate;
use datafusion::physical_plan::ExecutionPlan;
use datafusion::prelude::*;

use rand::random;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::datafusion::create_optimized_context;
use super::filtering::PnrFilter;
use super::pruning::create_pnr_pruning_predicate;

/// Options for controlling parquet reading behavior
#[derive(Debug, Clone)]
pub struct ParquetReadConfig {
    /// Path to read from (file or directory)
    pub path: PathBuf,
    /// Arrow schema for reading the file
    pub schema: Option<SchemaRef>,
    /// Batch size for reading
    pub batch_size: usize,
    /// Enable or disable parallel loading
    pub parallel: bool,
    /// Enable or disable asynchronous loading
    pub async_loading: bool,
    /// Enable statistics-based pruning
    pub enable_pruning: bool,
    /// Column to use for pruning (defaults to "PNR")
    pub pruning_column: String,
    /// Number of partitions to use for distributed processing
    pub partitions: usize,
    /// Optional filter expression to push down
    pub filter: Option<Expr>,
    /// Optional projection (list of column indices)
    pub projection: Option<Vec<usize>>,
    /// Optional limit on number of records
    pub limit: Option<usize>,
}

impl Default for ParquetReadConfig {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
            schema: None,
            batch_size: 8192,
            parallel: true,
            async_loading: true,
            enable_pruning: true,
            pruning_column: "PNR".to_string(),
            partitions: 4,
            filter: None,
            projection: None,
            limit: None,
        }
    }
}

/// Unified `ParquetReader` with `DataFusion` integration
pub struct ParquetReader {
    config: ParquetReadConfig,
    file_list: Option<Vec<PathBuf>>,
    session_context: Option<SessionContext>,
}

impl ParquetReader {
    /// Create a new `ParquetReader`
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self {
            config: ParquetReadConfig {
                path: path.as_ref().to_path_buf(),
                ..Default::default()
            },
            file_list: None,
            session_context: None,
        }
    }

    /// Set the schema for this reader
    #[must_use]
    pub fn with_schema(mut self, schema: SchemaRef) -> Self {
        self.config.schema = Some(schema);
        self
    }

    /// Set the batch size for this reader
    #[must_use]
    pub const fn with_batch_size(mut self, batch_size: usize) -> Self {
        self.config.batch_size = batch_size;
        self
    }

    /// Enable or disable parallel loading
    #[must_use]
    pub const fn parallel(mut self, parallel: bool) -> Self {
        self.config.parallel = parallel;
        self
    }

    /// Enable or disable async loading
    #[must_use]
    pub const fn async_loading(mut self, async_loading: bool) -> Self {
        self.config.async_loading = async_loading;
        self
    }

    /// Enable or disable statistics-based pruning
    #[must_use]
    pub const fn enable_pruning(mut self, enable_pruning: bool) -> Self {
        self.config.enable_pruning = enable_pruning;
        self
    }

    /// Set the column to use for pruning
    #[must_use]
    pub fn pruning_column(mut self, column: impl Into<String>) -> Self {
        self.config.pruning_column = column.into();
        self
    }

    /// Set the number of partitions
    #[must_use]
    pub const fn partitions(mut self, partitions: usize) -> Self {
        self.config.partitions = partitions;
        self
    }

    /// Add a filter expression
    #[must_use]
    pub fn filter(mut self, filter: Expr) -> Self {
        self.config.filter = Some(filter);
        self
    }

    /// Set projection (list of column indices)
    #[must_use]
    pub fn projection(mut self, projection: Vec<usize>) -> Self {
        self.config.projection = Some(projection);
        self
    }

    /// Set a limit on the number of records
    #[must_use]
    pub const fn limit(mut self, limit: usize) -> Self {
        self.config.limit = Some(limit);
        self
    }

    /// Initialize the session context with optimized settings
    fn init_session_context(&mut self) -> Result<&SessionContext> {
        if self.session_context.is_none() {
            let mut config = SessionConfig::new().with_batch_size(self.config.batch_size);

            if self.config.parallel {
                config = config.with_target_partitions(self.config.partitions);
            } else {
                config = config.with_target_partitions(1);
            }

            let ctx = SessionContext::new_with_config(config);
            self.session_context = Some(ctx);
        }

        Ok(self.session_context.as_ref().unwrap())
    }

    /// Discover parquet files in the path
    fn discover_files(&mut self) -> Result<&Vec<PathBuf>> {
        if self.file_list.is_none() {
            let path = &self.config.path;
            if !path.exists() {
                return Err(IdsError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Path not found: {}", path.display()),
                )));
            }

            let files = if path.is_dir() {
                Self::list_parquet_files(path)?
            } else if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
                vec![path.clone()]
            } else {
                return Err(IdsError::Validation(format!(
                    "Path is not a parquet file or directory: {}",
                    path.display()
                )));
            };

            if files.is_empty() {
                return Err(IdsError::Validation(format!(
                    "No parquet files found in: {}",
                    path.display()
                )));
            }

            self.file_list = Some(files);
        }

        Ok(self.file_list.as_ref().unwrap())
    }

    /// Create execution plan for parquet reading with DataFusion 47.0.0
    async fn create_execution_plan(&mut self) -> Result<Arc<dyn ExecutionPlan>> {
        // Ensure session context is initialized
        self.init_session_context()?;

        // Ensure files are discovered
        let files = self.discover_files()?.clone();

        // Create the format (ParquetSource as the FileSource implementation)
        let format = ParquetSource::default()
            .with_enable_page_index(self.config.enable_pruning) // Enable page-level pruning
            .with_pushdown_filters(true); // Enable predicate pushdown
        let format_arc = Arc::new(format);

        // Create the object store URL - we need to use file:// URI
        let url = ObjectStoreUrl::parse("file://".to_string())?;

        // Start building the config with schema
        let schema = if let Some(schema) = &self.config.schema {
            schema.clone()
        } else {
            // If no schema provided, try to infer from first file
            if let Some(first_file) = files.first() {
                let read_options = ParquetReadOptions::default();
                let ctx = self.session_context.as_ref().unwrap();
                let df = ctx
                    .read_parquet(first_file.to_string_lossy().to_string(), read_options)
                    .await?;
                df.schema()
            } else {
                return Err(IdsError::Validation("No files to read".to_string()));
            }
        };

        // Build the config with all files
        let mut config_builder = FileScanConfigBuilder::new(url, schema, format_arc);

        // Add each file individually with size info for better statistics
        for file_path in &files {
            let file_size = fs::metadata(file_path).map(|m| m.len()).unwrap_or(0);

            let file_path_str = file_path.to_string_lossy().to_string();
            config_builder =
                config_builder.with_file(PartitionedFile::new(file_path_str, file_size));
        }

        // Add projection if provided
        if let Some(proj) = &self.config.projection {
            config_builder = config_builder.with_projection(Some(proj.clone()));
        }

        // Add limit if provided
        if let Some(lim) = self.config.limit {
            config_builder = config_builder.with_limit(Some(lim));
        }

        // Build the config
        let config = config_builder.build();

        // Create DataSourceExec with the config and return it
        // DataSourceExec implements ExecutionPlan trait
        Ok(DataSourceExec::from_data_source(config) as Arc<dyn ExecutionPlan>)
    }

    /// Read parquet data synchronously
    pub fn read(&mut self) -> Result<Vec<RecordBatch>> {
        // Check if path exists
        if !self.config.path.exists() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Path not found: {}", self.config.path.display()),
            )));
        }

        // Use tokio runtime for async operation
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|e| IdsError::Data(format!("Failed to create runtime: {e}")))?;

        runtime.block_on(self.read_async())
    }

    /// Read parquet data asynchronously
    pub async fn read_async(&mut self) -> Result<Vec<RecordBatch>> {
        let ctx = self.init_session_context()?.clone();

        // If filter is set, we'll use the execution plan approach
        if self.config.filter.is_some()
            || self.config.enable_pruning
            || self.config.projection.is_some()
            || self.config.limit.is_some()
        {
            // Create execution plan
            let plan = self.create_execution_plan().await?;

            // Create dataframe from the execution plan
            let df = ctx.execute_physical_plan(plan).await?;

            // Apply filter if provided
            let df = if let Some(filter) = &self.config.filter {
                df.filter(filter.clone())?
            } else {
                df
            };

            // Collect and return the results
            Ok(df.collect().await?)
        } else {
            // Simpler path when no advanced features are needed
            // Create read options with schema if provided
            let mut read_options = ParquetReadOptions::default();

            if let Some(schema) = &self.config.schema {
                read_options = read_options.schema(schema.as_ref());
            }

            // Read the files
            let df = ctx
                .read_parquet(self.config.path.to_string_lossy().to_string(), read_options)
                .await?;

            // Collect and return the results
            Ok(df.collect().await?)
        }
    }

    /// Read parquet data with PNR filter
    pub async fn read_with_pnr_filter(
        &mut self,
        pnr_filter: &PnrFilter,
    ) -> Result<Vec<RecordBatch>> {
        // Clone the PnrFilter to an Expr and set it in config
        if let Some(expr) = pnr_filter.to_expr() {
            self.config.filter = Some(expr);
        }

        // Use read_async with the filter
        self.read_async().await
    }

    /// Create an execution plan that can be used in custom DataFusion operations
    pub async fn create_plan(&mut self) -> Result<Arc<dyn ExecutionPlan>> {
        self.create_execution_plan().await
    }

    /// Create a DataFrame from this reader
    pub async fn to_dataframe(&mut self) -> Result<DataFrame> {
        let ctx = self.init_session_context()?.clone();
        let plan = self.create_execution_plan().await?;
        let df = ctx.execute_physical_plan(plan).await?;

        // Apply filter if provided
        if let Some(filter) = &self.config.filter {
            Ok(df.filter(filter.clone())?)
        } else {
            Ok(df)
        }
    }

    /// List all parquet files in a directory
    pub fn list_parquet_files(dir_path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        let dir_path = dir_path.as_ref();
        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory not found: {}", dir_path.display()),
            )));
        }

        let mut result = Vec::new();
        for entry in std::fs::read_dir(dir_path).map_err(|e| {
            IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                format!("Failed to read directory {}: {}", dir_path.display(), e),
            ))
        })? {
            let entry = entry.map_err(|e| {
                IdsError::Io(std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!("Failed to read directory entry: {e}"),
                ))
            })?;

            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
                result.push(path);
            } else if path.is_dir() {
                // Recursively scan subdirectories
                let sub_files = Self::list_parquet_files(&path)?;
                result.extend(sub_files);
            }
        }

        Ok(result)
    }

    /// Create a custom table provider for more control over the execution
    pub async fn create_table_provider(&mut self, name: &str) -> Result<()> {
        // Ensure session context is initialized
        let ctx = self.init_session_context()?.clone();

        // Create execution plan
        let plan = self.create_execution_plan().await?;

        // Create a dataframe from the plan
        let df = ctx.execute_physical_plan(plan).await?;

        // Register the dataframe as a table
        ctx.register_table(name, df.into_view())?;

        Ok(())
    }
}

/// Load parquet files from a directory using `DataFusion` with pruning optimization
pub async fn load_parquet_directory(
    dir_path: impl AsRef<Path>,
    schema: Option<SchemaRef>,
    pnr_filter: Option<&PnrFilter>,
) -> Result<Vec<RecordBatch>> {
    let dir_path = dir_path.as_ref();

    // Create a ParquetReader with appropriate configuration
    let mut reader = ParquetReader::new(dir_path);

    // Set schema if provided
    if let Some(s) = schema {
        reader = reader.with_schema(s);
    }

    // Set filter if provided
    if let Some(filter) = pnr_filter {
        if let Some(expr) = filter.to_expr() {
            reader = reader.filter(expr);
        }
    }

    // Enable pruning for better performance
    reader = reader.enable_pruning(true);

    // Read the data
    reader.read_async().await
}

/// Load parquet files and register them as a table in a `SessionContext`
pub async fn register_parquet_as_table(
    ctx: &SessionContext,
    table_name: &str,
    path: impl AsRef<Path>,
    schema: Option<SchemaRef>,
    pnr_filter: Option<&PnrFilter>,
) -> Result<DataFrame> {
    let path = path.as_ref();

    // Create read options
    let mut read_options = ParquetReadOptions::default();
    if let Some(s) = &schema {
        read_options = read_options.schema(s.as_ref());
    }

    // Register the parquet file/directory
    ctx.register_parquet(table_name, path.to_string_lossy().to_string(), read_options)
        .await?;

    // Get the table
    let mut df = ctx.table(table_name).await?;

    // Apply filter if provided
    if let Some(filter) = pnr_filter {
        if let Some(expr) = filter.to_expr() {
            df = df.filter(expr)?;

            // Update the table with the filtered data
            ctx.register_table(table_name, df.clone().into_view())?;
        }
    }

    Ok(df)
}

/// Convert record batches to a DataFusion table
pub fn batches_to_table(batches: &[RecordBatch]) -> Result<Arc<dyn TableProvider>> {
    if batches.is_empty() {
        return Err(IdsError::Validation(
            "Cannot create table from empty batches".to_string(),
        ));
    }

    let provider = MemTable::try_new(batches[0].schema(), vec![batches.to_vec()])?;
    Ok(Arc::new(provider))
}

/// Register record batches as a table in a session context
pub fn register_batches(
    ctx: &SessionContext,
    table_name: &str,
    batches: &[RecordBatch],
) -> Result<()> {
    if batches.is_empty() {
        return Err(IdsError::Validation(
            "Cannot register empty batches".to_string(),
        ));
    }

    let provider = batches_to_table(batches)?;
    ctx.register_table(table_name, provider)?;
    Ok(())
}

/// Save record batches to parquet file
pub async fn save_batches_to_parquet(
    batches: &[RecordBatch],
    output_path: impl AsRef<Path>,
) -> Result<()> {
    if batches.is_empty() {
        return Err(IdsError::Validation(
            "Cannot save empty batches".to_string(),
        ));
    }

    // Create a session context
    let ctx = create_optimized_context();

    // Create a random table name
    let table_name = format!("temp_table_{}", random::<u64>());

    // Register the batches
    register_batches(&ctx, &table_name, batches)?;

    // Get the table as a DataFrame
    let df = ctx.table(&table_name).await?;

    // Write to parquet
    df.write_parquet(
        output_path.as_ref().to_str().unwrap(),
        datafusion::dataframe::DataFrameWriteOptions::default(),
        None,
    )
    .await?;

    Ok(())
}

use crate::error::{IdsError, Result};
use arrow::datatypes::SchemaRef;
use arrow::record_batch::RecordBatch;
use datafusion::datasource::listing::PartitionedFile;
use datafusion::datasource::object_store::ObjectStoreUrl;
use datafusion::datasource::physical_plan::{FileScanConfigBuilder, ParquetSource};
use datafusion::datasource::source::DataSourceExec;
use datafusion::datasource::{MemTable, TableProvider};
use datafusion::execution::context::{SessionConfig, SessionContext};
use datafusion::physical_plan::ExecutionPlan;
use datafusion::prelude::*;

use rand::random;

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::datafusion::create_optimized_context;
use crate::data::PnrFilter;
use crate::utils::path_utils::resolve_path;

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

    /// Discover parquet files in the path with enhanced error handling
    fn discover_files(&mut self) -> Result<&Vec<PathBuf>> {
        if self.file_list.is_none() {
            // Convert to absolute path if needed
            let abs_path = resolve_path(&self.config.path)?;

            log::debug!(
                "Discovering files using absolute path: {}",
                abs_path.display()
            );

            if !abs_path.exists() {
                log::error!("Path does not exist: {}", abs_path.display());
                return Err(IdsError::Io(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    format!("Path not found: {}", abs_path.display()),
                )));
            }

            let files = if abs_path.is_dir() {
                // Use our improved list_parquet_files to recursively find all parquet files
                log::debug!("Path is a directory, searching recursively for parquet files: {}", abs_path.display());
                match Self::list_parquet_files(&abs_path) {
                    Ok(files) => files,
                    Err(e) => {
                        log::error!("Failed to list parquet files in directory {}: {}", abs_path.display(), e);
                        return Err(e);
                    }
                }
            } else if abs_path.is_file() {
                if abs_path.extension().is_some_and(|ext| ext == "parquet") {
                    log::debug!("Path is a single parquet file: {}", abs_path.display());
                    vec![abs_path.clone()]
                } else {
                    log::error!("Path is a file but not a parquet file: {}", abs_path.display());
                    return Err(IdsError::Validation(format!(
                        "Path is not a parquet file: {}",
                        abs_path.display()
                    )));
                }
            } else {
                log::error!("Path is neither a file nor a directory: {}", abs_path.display());
                return Err(IdsError::Validation(format!(
                    "Path is not a parquet file or directory: {}",
                    abs_path.display()
                )));
            };

            if files.is_empty() {
                log::warn!("No parquet files found in: {}", abs_path.display());
                
                // Now handle empty directories with a more specific error message that includes 
                // suggestions to check file permissions or if the files are actually in subdirectories
                return Err(IdsError::Validation(format!(
                    "No parquet files found in '{}' or its subdirectories. Please check that:\n\
                    1. The path is correct\n\
                    2. The directory contains .parquet files (directly or in subdirectories)\n\
                    3. You have read permissions for the files\n\
                    4. Files have the correct '.parquet' extension",
                    abs_path.display()
                )));
            }

            log::info!("Successfully discovered {} parquet files", files.len());
            self.file_list = Some(files);
        }

        Ok(self.file_list.as_ref().unwrap())
    }

    /// Create physical execution plan for parquet reading with `DataFusion` 47.0.0
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
        let url = ObjectStoreUrl::parse("file://")?;

        // Start building the config with schema
        let schema = if let Some(schema) = &self.config.schema {
            schema.clone()
        } else {
            // If no schema provided, try to infer from first file
            if let Some(first_file) = files.first() {
                // Convert to absolute path if needed
                let abs_file_path = resolve_path(first_file)?;

                let read_options = ParquetReadOptions::default();
                let ctx = self.session_context.as_ref().unwrap();

                // Simply use the absolute path string - DataFusion will handle it
                // DataFusion seems to be adding its own file:// prefix, so we shouldn't add one
                let path_str = abs_file_path.to_string_lossy().to_string();
                log::debug!("Using absolute path for schema inference: {}", path_str);

                let df = ctx.read_parquet(path_str, read_options).await?;
                // Convert DFSchema to Schema using Arc::new
                Arc::new(df.schema().clone().into())
            } else {
                return Err(IdsError::Validation("No files to read".to_string()));
            }
        };

        // Build the config with all files
        let mut config_builder = FileScanConfigBuilder::new(url, schema, format_arc);

        // Add each file individually with size info for better statistics
        for file_path in &files {
            // Convert to absolute path if needed
            let abs_file_path = resolve_path(file_path)?;

            let file_size = fs::metadata(&abs_file_path).map(|m| m.len()).unwrap_or(0);

            // Simply use the absolute path string - DataFusion will handle it
            // DataFusion seems to be adding its own file:// prefix, so we shouldn't add one
            let path_str = abs_file_path.to_string_lossy().to_string();
            log::debug!("Adding file to execution plan: {}", path_str);

            config_builder = config_builder.with_file(PartitionedFile::new(path_str, file_size));
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
        // Check if path exists using the resolved path
        let abs_path = resolve_path(&self.config.path)?;
        if !abs_path.exists() {
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Path not found: {}", abs_path.display()),
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

        // Ensure path is absolute
        let abs_path = resolve_path(&self.config.path)?;

        log::debug!("Reading parquet from absolute path: {}", abs_path.display());

        // If filter is set, we'll use the execution plan approach
        if self.config.filter.is_some()
            || self.config.enable_pruning
            || self.config.projection.is_some()
            || self.config.limit.is_some()
        {
            // Create physical execution plan
            let physical_plan = self.create_execution_plan().await?;

            // Create a DataFrame from the physical plan using DataFusion's task context
            let task_ctx = ctx.task_ctx();
            let results =
                datafusion::physical_plan::collect(physical_plan.clone(), task_ctx).await?;

            // Apply filter if provided after collecting results - this is inefficient
            // but a proper implementation would incorporate the filter into the logical plan
            if let Some(filter) = &self.config.filter {
                // This is a simplified approach - in a real implementation,
                // you would incorporate the filter into the logical plan before creating the physical plan
                let df = ctx.read_batches(results.clone())?;
                let filtered_df = df.filter(filter.clone())?;
                Ok(filtered_df.collect().await?)
            } else {
                Ok(results)
            }
        } else {
            // Simpler path when no advanced features are needed
            // Create read options with schema if provided
            let mut read_options = ParquetReadOptions::default();

            if let Some(schema) = &self.config.schema {
                read_options = read_options.schema(schema.as_ref());
            }

            // Read the files using absolute path
            // Simply use the absolute path string - DataFusion will handle it
            // DataFusion seems to be adding its own file:// prefix, so we shouldn't add one
            let path_str = abs_path.to_string_lossy().to_string();
            log::debug!("Using absolute path for DataFusion: {}", path_str);

            let df = ctx.read_parquet(path_str, read_options).await?;

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

    /// Create an execution plan that can be used in custom `DataFusion` operations
    pub async fn create_plan(&mut self) -> Result<Arc<dyn ExecutionPlan>> {
        self.create_execution_plan().await
    }

    /// Create a `DataFrame` from this reader
    pub async fn to_dataframe(&mut self) -> Result<DataFrame> {
        let ctx = self.init_session_context()?.clone();
        let physical_plan = self.create_execution_plan().await?;

        // Create a DataFrame by collecting the results and then creating a DataFrame from them
        let task_ctx = ctx.task_ctx();
        let results = datafusion::physical_plan::collect(physical_plan.clone(), task_ctx).await?;
        let df = ctx.read_batches(results)?;

        // Apply filter if provided
        if let Some(filter) = &self.config.filter {
            Ok(df.filter(filter.clone())?)
        } else {
            Ok(df)
        }
    }

    /// List all parquet files in a directory with enhanced logging
    pub fn list_parquet_files(dir_path: impl AsRef<Path>) -> Result<Vec<PathBuf>> {
        let dir_path = dir_path.as_ref();
        if !dir_path.exists() || !dir_path.is_dir() {
            log::warn!("Directory does not exist or is not a directory: {}", dir_path.display());
            return Err(IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                format!("Directory not found: {}", dir_path.display()),
            )));
        }

        log::debug!("Scanning directory for parquet files: {}", dir_path.display());
        let mut result = Vec::new();
        
        let entries = match std::fs::read_dir(dir_path) {
            Ok(entries) => entries,
            Err(e) => {
                log::error!("Failed to read directory {}: {}", dir_path.display(), e);
                return Err(IdsError::Io(std::io::Error::new(
                    std::io::ErrorKind::PermissionDenied,
                    format!("Failed to read directory {}: {}", dir_path.display(), e),
                )));
            }
        };

        for entry_result in entries {
            let entry = match entry_result {
                Ok(e) => e,
                Err(e) => {
                    log::error!("Failed to read directory entry: {}", e);
                    continue;
                }
            };

            let path = entry.path();
            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(e) => {
                    log::error!("Failed to get file type for {}: {}", path.display(), e);
                    continue;
                }
            };
            
            log::debug!("Found entry: {} (is_dir: {})", path.display(), file_type.is_dir());
            
            if file_type.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == "parquet" {
                        log::debug!("Found parquet file: {}", path.display());
                        result.push(path);
                    } else {
                        log::debug!("Skipping non-parquet file: {} (extension: {:?})", path.display(), ext);
                    }
                } else {
                    log::debug!("Skipping file without extension: {}", path.display());
                }
            } else if file_type.is_dir() {
                // Recursively scan subdirectories
                log::debug!("Recursively searching subdirectory: {}", path.display());
                match Self::list_parquet_files(&path) {
                    Ok(sub_files) => {
                        log::debug!("Found {} parquet files in subdirectory {}", sub_files.len(), path.display());
                        result.extend(sub_files);
                    },
                    Err(e) => {
                        log::warn!("Error searching subdirectory {}: {}", path.display(), e);
                        // Continue with other entries even if there's an error in a subdirectory
                    }
                }
            }
        }

        if result.is_empty() {
            log::warn!("No parquet files found in directory or subdirectories: {}", dir_path.display());
        } else {
            log::info!("Found {} parquet files in {} and its subdirectories", result.len(), dir_path.display());
        }

        Ok(result)
    }

    /// Create a custom table provider for more control over the execution
    pub async fn create_table_provider(&mut self, name: &str) -> Result<()> {
        // Ensure session context is initialized
        let ctx = self.init_session_context()?.clone();

        // Create physical execution plan
        let physical_plan = self.create_execution_plan().await?;

        // Create a DataFrame by collecting the results and then creating a DataFrame from them
        let task_ctx = ctx.task_ctx();
        let results = datafusion::physical_plan::collect(physical_plan.clone(), task_ctx).await?;
        let df = ctx.read_batches(results)?;

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
    // Convert to absolute path for DataFusion - this handles relative paths like ../../
    let abs_path = resolve_path(dir_path)?;

    log::debug!("Loading parquet from absolute path: {}", abs_path.display());

    // Create a ParquetReader with appropriate configuration
    let mut reader = ParquetReader::new(abs_path);

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
    // Convert to absolute path for DataFusion - this handles relative paths like ../../
    let abs_path = resolve_path(path)?;

    log::debug!(
        "Registering parquet from absolute path: {}",
        abs_path.display()
    );

    // Create read options
    let mut read_options = ParquetReadOptions::default();
    if let Some(s) = &schema {
        read_options = read_options.schema(s.as_ref());
    }

    // Register the parquet file/directory using absolute path
    // Simply use the absolute path string - DataFusion will handle it
    // DataFusion seems to be adding its own file:// prefix, so we shouldn't add one
    let path_str = abs_path.to_string_lossy().to_string();
    log::debug!(
        "Using absolute path for DataFusion table registration: {}",
        path_str
    );

    ctx.register_parquet(table_name, path_str, read_options)
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

/// Convert record batches to a `DataFusion` table
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

/// Save record batches to parquet file with optimized settings
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

    // Get absolute path for the output
    let abs_path = crate::utils::path_utils::resolve_path(&output_path)?;
    let path_str = abs_path.to_string_lossy().to_string();
    log::debug!("Writing parquet batches to absolute path: {}", path_str);

    // Write to parquet with optimized settings
    df.write_parquet(
        &path_str,
        datafusion::dataframe::DataFrameWriteOptions::default(),
        Some(datafusion::common::config::TableParquetOptions::new()),
    )
    .await?;

    Ok(())
}

/// Save a single record batch to parquet file with optimized settings
pub async fn save_batch_to_parquet(
    batch: &RecordBatch,
    output_path: impl AsRef<Path>,
) -> Result<()> {
    // Create a session context
    let ctx = create_optimized_context();

    // Create a random table name
    let table_name = format!("temp_table_{}", random::<u64>());

    // Register the batch
    ctx.register_batch(&table_name, batch.clone())?;

    // Get the table as a DataFrame
    let df = ctx.table(&table_name).await?;

    // Get absolute path for the output
    let abs_path = crate::utils::path_utils::resolve_path(&output_path)?;
    let path_str = abs_path.to_string_lossy().to_string();
    log::debug!("Writing parquet to absolute path: {}", path_str);

    // Write to parquet with optimized settings
    df.write_parquet(
        &path_str,
        datafusion::dataframe::DataFrameWriteOptions::default(),
        Some(datafusion::common::config::TableParquetOptions::new()),
    )
    .await?;

    Ok(())
}

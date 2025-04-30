use std::any::Any;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use std::collections::HashSet;
use arrow::datatypes::SchemaRef;
use datafusion::datasource::{TableProvider, TableType};
use datafusion::error::Result as DFResult;
use datafusion::execution::context::{Session, SessionState};
use datafusion::logical_expr::Expr;
use datafusion::physical_plan::{ExecutionPlan, Partitioning};
use datafusion::physical_plan::file_format::{FileScanConfig, FileScanConfigBuilder, ParquetExec};
use datafusion::datasource::listing::{PartitionedFile, ListingTableUrl};
use datafusion::datasource::physical_plan::ParquetExecOptions;
use crate::error::{IdsError, Result};
use super::{LprVersion, LprPaths};

/// Custom table provider for LPR data
pub struct LprTableProvider {
    /// LPR version
    version: LprVersion,
    /// Paths to LPR files
    paths: LprPaths,
    /// Schema
    schema: SchemaRef,
    /// PNR filter
    pnr_filter: Option<HashSet<String>>,
}

impl LprTableProvider {
    /// Create a new LPR table provider
    pub fn new(
        version: LprVersion,
        paths: LprPaths,
        schema: SchemaRef,
        pnr_filter: Option<HashSet<String>>,
    ) -> Self {
        Self {
            version,
            paths,
            schema,
            pnr_filter,
        }
    }
    
    /// Find LPR files based on the version and paths
    fn find_lpr_files(&self) -> Result<Vec<PathBuf>> {
        let mut files = Vec::new();
        
        match self.version {
            LprVersion::V2 => {
                // Find admin files
                if let Some(admin_path) = &self.paths.admin_path {
                    files.extend(find_parquet_files(admin_path)?);
                }
                
                // Find diagnosis files
                if let Some(diag_path) = &self.paths.diag_path {
                    files.extend(find_parquet_files(diag_path)?);
                }
                
                // Find procedure files
                if let Some(proc_path) = &self.paths.proc_path {
                    files.extend(find_parquet_files(proc_path)?);
                }
            },
            LprVersion::V3 => {
                // Find kontakter files
                if let Some(kontakter_path) = &self.paths.kontakter_path {
                    files.extend(find_parquet_files(kontakter_path)?);
                }
                
                // Find diagnoser files
                if let Some(diagnoser_path) = &self.paths.diagnoser_path {
                    files.extend(find_parquet_files(diagnoser_path)?);
                }
                
                // Find procedurer files
                if let Some(procedurer_path) = &self.paths.procedurer_path {
                    files.extend(find_parquet_files(procedurer_path)?);
                }
            },
        }
        
        Ok(files)
    }
}

/// Helper function to find parquet files in a directory
fn find_parquet_files(dir: &PathBuf) -> Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    
    if !dir.exists() || !dir.is_dir() {
        return Ok(files);
    }
    
    for entry_result in fs::read_dir(dir).map_err(|e| {
        IdsError::Io(std::io::Error::new(
            std::io::ErrorKind::PermissionDenied,
            format!("Failed to read directory {}: {}", dir.display(), e),
        ))
    })? {
        let entry = entry_result.map_err(|e| {
            IdsError::Io(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Failed to read directory entry: {e}"),
            ))
        })?;
        
        let path = entry.path();
        if path.is_file() && path.extension().is_some_and(|ext| ext == "parquet") {
            files.push(path);
        }
    }
    
    Ok(files)
}

#[async_trait::async_trait]
impl TableProvider for LprTableProvider {
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn schema(&self) -> SchemaRef {
        self.schema.clone()
    }
    
    fn table_type(&self) -> TableType {
        TableType::Base
    }
    
    fn supports_filters_pushdown(
        &self,
        filters: &[Expr],
    ) -> DFResult<Vec<Expr>> {
        // We can push down PNR filter but not complex filters
        let mut pushdowns = Vec::new();
        
        for filter in filters {
            if let Expr::BinaryExpr { left, op, right } = filter {
                if let Expr::Column(col) = left.as_ref() {
                    if col.name == "PNR" {
                        pushdowns.push(filter.clone());
                    }
                }
            }
        }
        
        Ok(pushdowns)
    }
    
    async fn scan(
        &self,
        state: &SessionState,
        projection: Option<&Vec<usize>>,
        filters: &[Expr],
        limit: Option<usize>,
    ) -> DFResult<Arc<dyn ExecutionPlan>> {
        // Custom logic to handle LPR's multiple files and directories
        // using the version and paths information
        
        // Find all the parquet files
        let parquet_files = self.find_lpr_files()
            .map_err(|e| datafusion::error::DataFusionError::Execution(format!("Error finding LPR files: {}", e)))?;
        
        if parquet_files.is_empty() {
            return Err(datafusion::error::DataFusionError::Execution(
                "No LPR files found in the specified paths".to_string()
            ));
        }
        
        // Create the listing table URL
        let url = ListingTableUrl::from_path(&self.paths.base_path.to_string_lossy());
        
        // Create the file scan config builder
        let mut file_scan_config_builder = FileScanConfig::builder(url, self.schema.clone());
        
        // Set projection if provided
        if let Some(projection) = projection {
            file_scan_config_builder = file_scan_config_builder.with_projection(projection.clone());
        }
        
        // Set limit if provided
        if let Some(limit) = limit {
            file_scan_config_builder = file_scan_config_builder.with_limit(limit);
        }
        
        // Add all the files from the paths
        for file_path in parquet_files {
            // Get the file size for statistics
            let file_size = fs::metadata(&file_path)
                .map(|m| m.len() as usize)
                .unwrap_or(0);
            
            // Convert to string path
            let file_path_str = file_path.to_string_lossy().to_string();
            
            // Add the file
            file_scan_config_builder = file_scan_config_builder.with_file(
                PartitionedFile::new(file_path_str, file_size)
            );
        }
        
        // Build the file scan config
        let file_scan_config = file_scan_config_builder.build()?;
        
        // Create the parquet exec options
        let options = ParquetExecOptions::default();
        
        // Create the parquet exec
        let parquet_exec = ParquetExec::new(file_scan_config, filters.to_vec(), options);
        
        Ok(Arc::new(parquet_exec))
    }
}
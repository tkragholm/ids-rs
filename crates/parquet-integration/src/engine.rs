use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::time::Instant;

use arrow_array::RecordBatch;
use arrow_schema::Schema;
use futures::TryStreamExt;
use log::{debug, info};
use parquet::arrow::arrow_reader::{ArrowReaderOptions, ParquetRecordBatchReaderBuilder};
use parquet::arrow::ParquetRecordBatchStreamBuilder;
use parquet::file::metadata::ParquetMetaData;
use parquet::file::reader::SerializedFileReader;
use tokio::fs::File as TokioFile;
use tokio::runtime::Runtime;

use crate::config::{ColumnSelection, ParquetConfig, ReaderMode};
use crate::error::{ParquetIntegrationError, Result};
use crate::filter::post_process_batches;
use crate::optimize::Optimizer;
use crate::result::{ReadResult, ReadStatistics};
use crate::utils::{self, MemoryTracker};

/// Main Parquet reader engine
#[derive(Clone)]
pub struct ParquetReader {
    /// Reader configuration
    pub config: Arc<ParquetConfig>,
    
    /// Optimizer for reading strategy
    optimizer: Arc<Optimizer>,
}

impl Default for ParquetReader {
    fn default() -> Self {
        Self::new()
    }
}

impl ParquetReader {
    /// Create a new reader with default configuration
    pub fn new() -> Self {
        let config = Arc::new(ParquetConfig::default());
        let optimizer = Arc::new(Optimizer::new(config.clone()));
        
        Self { config, optimizer }
    }
    
    /// Create with custom configuration
    pub fn with_config(config: ParquetConfig) -> Self {
        let config = Arc::new(config);
        let optimizer = Arc::new(Optimizer::new(config.clone()));
        
        Self { config, optimizer }
    }
    
    /// Configure batch size
    pub fn with_batch_size(mut self, batch_size: usize) -> Self {
        let mut config = (*self.config).clone();
        config.batch_size = batch_size;
        self.config = Arc::new(config);
        self.optimizer = Arc::new(Optimizer::new(self.config.clone()));
        self
    }
    
    /// Configure reader mode
    pub fn with_mode(mut self, mode: ReaderMode) -> Self {
        let mut config = (*self.config).clone();
        config.mode = mode;
        self.config = Arc::new(config);
        self.optimizer = Arc::new(Optimizer::new(self.config.clone()));
        self
    }
    
    /// Configure column selection by index
    pub fn with_column_indices(self, indices: Vec<usize>) -> Self {
        let mut config = (*self.config).clone();
        config.columns = Some(ColumnSelection::ByIndex(indices));
        Self::with_config(config)
    }
    
    /// Configure column selection by name
    pub fn with_column_names(self, names: Vec<String>) -> Self {
        let mut config = (*self.config).clone();
        config.columns = Some(ColumnSelection::ByName(names));
        Self::with_config(config)
    }
    
    /// Read a Parquet file asynchronously
    pub async fn read_file(&self, path: &Path) -> Result<ReadResult> {
        // Get file stats
        let metadata = tokio::fs::metadata(path).await
            .map_err(|e| ParquetIntegrationError::IoError(e))?;
            
        let file_size = metadata.len() as usize;
        
        // Choose reading strategy based on file size and config
        let mode = self.optimizer.choose_read_mode(path, file_size).await?;
        
        debug!("Reading file {} with mode {:?}, batch size {}", 
            path.display(), mode, self.config.batch_size);
        
        // Initialize result and memory tracker
        let mut result = ReadResult::new()
            .with_file_size(file_size);
        let memory_tracker = MemoryTracker::new();
        
        // Read based on the selected mode
        match mode {
            ReaderMode::Async => {
                self.read_file_async(path, &mut result).await?;
            },
            ReaderMode::Sync => {
                self.read_file_sync(path, &mut result).await?;
            },
            ReaderMode::RowGroup => {
                self.read_file_by_row_groups(path, &mut result).await?;
            },
            ReaderMode::Metadata => {
                self.read_file_metadata_only(path, &mut result).await?;
            },
            ReaderMode::Adaptive => {
                // This should never happen as the optimizer should have chosen a specific mode
                self.read_file_async(path, &mut result).await?;
            }
        }
        
        // Update memory peak
        result.stats.memory_peak = memory_tracker.peak();
        
        // Finish result
        result.finish();
        
        // Apply PNR filter if provided
        if let Some(pnr_filter) = self.config.pnr_filter.as_ref() {
            result.batches = post_process_batches(result.batches, Some(pnr_filter))?;
        }
        
        Ok(result)
    }
    
    /// Read a Parquet file synchronously
    async fn read_file_sync(&self, path: &Path, result: &mut ReadResult) -> Result<()> {
        // Open the file
        let file = std::fs::File::open(path)
            .map_err(|e| ParquetIntegrationError::IoError(e))?;
            
        // Create reader
        let reader = SerializedFileReader::new(file)
            .map_err(|e| ParquetIntegrationError::ParquetError(e))?;
            
        // Get metadata
        let metadata = reader.metadata();
        result.stats.row_groups = metadata.row_groups().len();
        
        // Create Arrow reader
        let mut arrow_reader = ParquetRecordBatchReaderBuilder::try_new(reader)
            .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?
            .with_batch_size(self.config.batch_size);
            
        // Apply column projection if specified
        if let Some(columns) = &self.config.columns {
            match columns {
                ColumnSelection::ByName(names) => {
                    arrow_reader = arrow_reader.with_projection(names.clone());
                },
                ColumnSelection::ByIndex(indices) => {
                    let schema = arrow_reader.schema();
                    let field_names: Vec<String> = indices.iter()
                        .filter_map(|&i| schema.field(i).map(|f| f.name().clone()))
                        .collect();
                    arrow_reader = arrow_reader.with_projection(field_names);
                }
            }
        }
        
        // Build the reader
        let arrow_reader = arrow_reader.build()
            .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
            
        // Read all batches
        let mut batches = Vec::new();
        for batch_result in arrow_reader {
            let batch = batch_result.map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
            batches.push(batch);
        }
        
        result.add_batches(batches);
        
        Ok(())
    }
    
    /// Read a Parquet file asynchronously
    async fn read_file_async(&self, path: &Path, result: &mut ReadResult) -> Result<()> {
        // Open the file
        let file = TokioFile::open(path).await
            .map_err(|e| ParquetIntegrationError::IoError(e))?;
            
        // Create Arrow reader
        let mut builder = ParquetRecordBatchStreamBuilder::new(file)
            .await
            .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
        
        // Get metadata
        let metadata = builder.metadata().clone();
        result.stats.row_groups = metadata.row_groups().len();
        
        // Set batch size
        builder = builder.with_batch_size(self.config.batch_size);
        
        // Apply column projection if specified
        if let Some(columns) = &self.config.columns {
            match columns {
                ColumnSelection::ByName(names) => {
                    builder = builder.with_projection(names.clone());
                },
                ColumnSelection::ByIndex(indices) => {
                    let schema = builder.schema();
                    let field_names: Vec<String> = indices.iter()
                        .filter_map(|&i| schema.field(i).map(|f| f.name().clone()))
                        .collect();
                    builder = builder.with_projection(field_names);
                }
            }
        }
        
        // Build stream
        let stream = builder.build()
            .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
            
        // Read all batches
        let batches: Vec<RecordBatch> = stream
            .try_collect()
            .await
            .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
            
        result.add_batches(batches);
        
        Ok(())
    }
    
    /// Read a Parquet file by row groups
    async fn read_file_by_row_groups(&self, path: &Path, result: &mut ReadResult) -> Result<()> {
        // Open the file
        let file = std::fs::File::open(path)
            .map_err(|e| ParquetIntegrationError::IoError(e))?;
            
        // Create reader
        let reader = SerializedFileReader::new(file)
            .map_err(|e| ParquetIntegrationError::ParquetError(e))?;
            
        // Get metadata
        let metadata = reader.metadata();
        let row_groups = metadata.row_groups();
        result.stats.row_groups = row_groups.len();
        
        // Process each row group separately
        let mut batches = Vec::new();
        for (idx, _) in row_groups.iter().enumerate() {
            // Create options for row group selection
            let options = ArrowReaderOptions::new().with_page_index(true);
            
            // Create reader for this row group
            let mut arrow_reader = ParquetRecordBatchReaderBuilder::try_new_with_options(reader.clone(), options)
                .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?
                .with_batch_size(self.config.batch_size)
                .with_row_groups(vec![idx]);
                
            // Apply column projection if specified
            if let Some(columns) = &self.config.columns {
                match columns {
                    ColumnSelection::ByName(names) => {
                        arrow_reader = arrow_reader.with_projection(names.clone());
                    },
                    ColumnSelection::ByIndex(indices) => {
                        let schema = arrow_reader.schema();
                        let field_names: Vec<String> = indices.iter()
                            .filter_map(|&i| schema.field(i).map(|f| f.name().clone()))
                            .collect();
                        arrow_reader = arrow_reader.with_projection(field_names);
                    }
                }
            }
            
            // Build the reader
            let arrow_reader = arrow_reader.build()
                .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
                
            // Read all batches from this row group
            for batch_result in arrow_reader {
                let batch = batch_result.map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
                batches.push(batch);
            }
        }
        
        result.add_batches(batches);
        
        Ok(())
    }
    
    /// Read only metadata from a Parquet file
    async fn read_file_metadata_only(&self, path: &Path, result: &mut ReadResult) -> Result<()> {
        // Open the file
        let file = std::fs::File::open(path)
            .map_err(|e| ParquetIntegrationError::IoError(e))?;
            
        // Read metadata
        let metadata = ParquetMetaData::parse_metadata(&file)
            .map_err(|e| ParquetIntegrationError::ParquetError(e))?;
            
        result.stats.row_groups = metadata.row_groups().len();
        
        // We don't add any batches in metadata-only mode
        
        Ok(())
    }
    
    /// Load a Parquet file with schema projection
    pub fn load_with_schema(&self, path: &Path, schema: &Schema) -> Result<Vec<RecordBatch>> {
        // Create runtime for async execution
        let rt = Runtime::new().map_err(|e| {
            ParquetIntegrationError::OperationError(format!(
                "Failed to create runtime: {}",
                e
            ))
        })?;
        
        // Read the file asynchronously
        let result = rt.block_on(async {
            let file = TokioFile::open(path).await
                .map_err(|e| ParquetIntegrationError::IoError(e))?;
                
            // Create Arrow reader with schema projection
            let builder = ParquetRecordBatchStreamBuilder::new(file)
                .await
                .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
                
            // Get just the names from the provided schema
            let field_names: Vec<String> = schema.fields().iter()
                .map(|f| f.name().clone())
                .collect();
                
            // Create stream with the projection and batch size
            let stream = builder
                .with_projection(field_names)
                .with_batch_size(self.config.batch_size)
                .build()
                .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
                
            // Read all batches
            let batches: Vec<RecordBatch> = stream
                .try_collect()
                .await
                .map_err(|e| ParquetIntegrationError::ArrowError(format!("{:?}", e)))?;
                
            // Apply PNR filter if provided
            let filtered_batches = if let Some(pnr_filter) = self.config.pnr_filter.as_ref() {
                post_process_batches(batches, Some(pnr_filter))?
            } else {
                batches
            };
            
            Ok::<_, ParquetIntegrationError>(filtered_batches)
        })?;
        
        Ok(result)
    }
}
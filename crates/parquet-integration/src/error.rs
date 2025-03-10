use thiserror::Error;

/// Error type for parquet integration operations
#[derive(Debug, Error)]
pub enum ParquetIntegrationError {
    /// Error from the parquet library
    #[error("Parquet error: {0}")]
    ParquetError(#[from] parquet::errors::ParquetError),

    /// Error from the arrow library
    #[error("Arrow error: {0}")]
    ArrowError(String),

    /// I/O error
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),

    /// Configuration error
    #[error("Configuration error: {0}")]
    ConfigError(String),

    /// General operation error
    #[error("{0}")]
    OperationError(String),
}

impl From<arrow::error::ArrowError> for ParquetIntegrationError {
    fn from(err: arrow::error::ArrowError) -> Self {
        Self::ArrowError(err.to_string())
    }
}

#[cfg(feature = "types-integration")]
impl From<ParquetIntegrationError> for types::error::IdsError {
    fn from(err: ParquetIntegrationError) -> Self {
        match err {
            ParquetIntegrationError::IoError(e) => types::error::IdsError::io_error(e),
            ParquetIntegrationError::ConfigError(e) => types::error::IdsError::configuration_error(e),
            _ => types::error::IdsError::invalid_operation(err.to_string()),
        }
    }
}

/// Result type for parquet integration operations
pub type Result<T> = std::result::Result<T, ParquetIntegrationError>;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArrowError {
    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),

    #[error("Parquet error: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Schema error: {0}")]
    Schema(String),

    #[error("Missing data: {0}")]
    MissingData(String),
}

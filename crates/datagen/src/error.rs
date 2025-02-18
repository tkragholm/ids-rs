use thiserror::Error;

#[derive(Error, Debug)]
pub enum DataGenError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),

    #[error("Parquet error: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),

    #[error("Invalid configuration: {0}")]
    Config(String),

    #[error("Generation error: {0}")]
    Generation(String),
}

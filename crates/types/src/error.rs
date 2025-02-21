use thiserror::Error;

#[derive(Error, Debug)]
pub enum IdsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    #[error("Missing data: {0}")]
    MissingData(String),
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    #[error("Sampling error: {0}")]
    Sampling(String),
    #[error("Covariate error: {0}")]
    Covariate(String),
}

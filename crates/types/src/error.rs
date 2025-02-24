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

impl IdsError {
    pub fn invalid_operation<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::InvalidOperation(msg.to_string())
    }

    pub fn missing_data<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::MissingData(msg.to_string())
    }

    pub fn invalid_format<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::InvalidFormat(msg.to_string())
    }
}

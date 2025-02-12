use thiserror::Error;

#[derive(Error, Debug)]
pub enum CovariateError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("Invalid date: {0}")]
    InvalidDate(String),

    #[error("Missing data for period: {0}")]
    MissingData(String),

    #[error("Invalid format: {0}")]
    InvalidFormat(String),
}

use thiserror::Error;

/// The main error type for the IDS project
/// 
/// This error type has been designed to handle all errors from all crates in the project,
/// allowing for a unified error handling approach while maintaining domain-specific context.
#[derive(Error, Debug)]
pub enum IdsError {
    // Common external errors
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("Arrow error: {0}")]
    Arrow(#[from] arrow::error::ArrowError),
    
    #[error("Parquet error: {0}")]
    Parquet(#[from] parquet::errors::ParquetError),

    // Data validation errors
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    
    #[error("Missing data: {0}")]
    MissingData(String),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),

    // Domain-specific errors
    #[error("Sampling error: {0}")]
    Sampling(String),
    
    #[error("No eligible controls found for case")]
    NoEligibleControls,
    
    #[error("Invalid matching criteria: {0}")]
    InvalidCriteria(String),
    
    #[error("Covariate error: {0}")]
    Covariate(String),
    
    #[error("Plotting error: {0}")]
    Plotting(String),
    
    #[error("Data generation error: {0}")]
    Generation(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

impl IdsError {
    /// Create an invalid operation error
    pub fn invalid_operation<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::InvalidOperation(msg.to_string())
    }

    /// Create a missing data error
    pub fn missing_data<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::MissingData(msg.to_string())
    }

    /// Create an invalid format error
    pub fn invalid_format<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::InvalidFormat(msg.to_string())
    }
    
    /// Create an invalid date error
    pub fn invalid_date<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::InvalidDate(msg.to_string())
    }
    
    /// Create a sampling error
    pub fn sampling<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::Sampling(msg.to_string())
    }
    
    /// Create a plotting error
    pub fn plotting<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::Plotting(msg.to_string())
    }
    
    /// Create a covariate error
    pub fn covariate<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::Covariate(msg.to_string())
    }
    
    /// Create a generation error
    pub fn generation<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::Generation(msg.to_string())
    }
    
    /// Create a configuration error
    pub fn config<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::Config(msg.to_string())
    }
    
    /// Create an invalid criteria error
    pub fn invalid_criteria<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::InvalidCriteria(msg.to_string())
    }
}

// Type aliases for backwards compatibility
pub type SamplingError = IdsError;
pub type PlottingError = IdsError;
pub type DataGenError = IdsError;
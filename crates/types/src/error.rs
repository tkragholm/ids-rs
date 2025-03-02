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
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

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
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Logging error: {0}")]
    Logging(String),
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
    
    /// Create an IO error with a message
    pub fn io_error<T: std::fmt::Display>(msg: T) -> Self {
        IdsError::Io(std::io::Error::new(std::io::ErrorKind::Other, msg.to_string()))
    }
}

// Type aliases for backwards compatibility
pub type SamplingError = IdsError;
pub type PlottingError = IdsError;
pub type DataGenError = IdsError;
pub type Result<T> = std::result::Result<T, IdsError>;

/// Context trait for enhancing error handling with contextual information
/// 
/// This trait adds methods to Result types to simplify adding context to errors,
/// making error messages more informative.
pub trait Context<T, E> {
    /// Add context to an error, explaining what was happening when the error occurred
    /// 
    /// # Example
    /// ```
    /// use types::error::Context;
    /// 
    /// let result = std::fs::read_to_string("missing_file.txt")
    ///     .with_context(|| "Failed to read configuration file");
    /// ```
    fn with_context<C, F>(self, context: F) -> std::result::Result<T, IdsError>
    where
        F: FnOnce() -> C,
        C: std::fmt::Display;

    /// Add context with details to an error, allowing for formatted messages
    ///
    /// # Example
    /// ```
    /// use types::error::Context;
    /// 
    /// let file_path = "data/config.json";
    /// let result = std::fs::read_to_string(file_path)
    ///     .with_context_details(|| format!("Failed to read file at {}", file_path));
    /// ```
    fn with_context_details<C, F>(self, context: F) -> std::result::Result<T, IdsError>
    where
        F: FnOnce() -> C,
        C: std::fmt::Display;
}

// Implement Context for any Result where the error implements Display
impl<T, E: std::fmt::Display + 'static> Context<T, E> for std::result::Result<T, E> {
    fn with_context<C, F>(self, context: F) -> std::result::Result<T, IdsError>
    where
        F: FnOnce() -> C,
        C: std::fmt::Display,
    {
        self.map_err(|e| {
            // Special case for io::Error to preserve its type information
            if let Some(io_err) = (&e as &dyn std::any::Any).downcast_ref::<std::io::Error>() {
                IdsError::Io(std::io::Error::new(io_err.kind(), format!("{}: {}", context(), e)))
            } else {
                IdsError::Validation(format!("{}: {}", context(), e))
            }
        })
    }

    fn with_context_details<C, F>(self, context: F) -> std::result::Result<T, IdsError>
    where
        F: FnOnce() -> C,
        C: std::fmt::Display,
    {
        self.map_err(|e| {
            // Special case for io::Error to preserve its type information
            if let Some(io_err) = (&e as &dyn std::any::Any).downcast_ref::<std::io::Error>() {
                IdsError::Io(std::io::Error::new(io_err.kind(), format!("{}", context())))
            } else {
                IdsError::Validation(format!("{}", context()))
            }
        })
    }
}
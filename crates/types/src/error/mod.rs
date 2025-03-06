use std::error::Error as StdError;
use thiserror::Error;
pub use anyhow::{anyhow, Context, Result as AnyhowResultType};
pub use color_eyre::{eyre::Report, Result as EyreResult};

// New modules
mod context;
mod conversion;

// Re-export from submodules
pub use self::context::{ErrorContext, with_context, with_context_details};

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
    
    #[error("Logger error: failed to set logger")]
    Logger,

    // Domain-specific errors
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Data loading error: {0}")]
    DataLoading(String),
    
    #[error("Balance calculation error: {0}")]
    BalanceCalculation(String),
    
    #[error("Sampling error: {0}")]
    Sampling(String),
    
    #[error("Register generation error: {0}")]
    Generation(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Path resolution error: {0}")]
    PathResolution(String),
    
    #[error("CLI argument error: {0}")]
    CliArgument(String),
    
    // Data validation errors
    #[error("Invalid date: {0}")]
    InvalidDate(String),
    
    #[error("Missing data: {0}")]
    MissingData(String),
    
    #[error("Invalid format: {0}")]
    InvalidFormat(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
    
    #[error("No eligible controls found for case")]
    NoEligibleControls,
    
    #[error("Invalid matching criteria: {0}")]
    InvalidCriteria(String),
    
    #[error("Covariate error: {0}")]
    Covariate(String),
    
    #[error("Plotting error: {0}")]
    Plotting(String),
    
    // Catch-all for other errors
    #[error("Other error: {0}")]
    Other(String),
    
    // Anyhow/Eyre integration
    #[error(transparent)]
    Anyhow(#[from] anyhow::Error),
    
    #[error(transparent)]
    Eyre(#[from] Report),
    
    // Boxed dynamic error for external errors
    #[error("External error: {0}")]
    External(#[from] Box<dyn StdError + Send + Sync>),
}

impl IdsError {
    /// Create a configuration error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::Config` with the provided message
    #[must_use]
    pub fn config(msg: impl ToString) -> Self {
        Self::Config(msg.to_string())
    }

    /// Create a data loading error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::DataLoading` with the provided message
    #[must_use]
    pub fn data_loading(msg: impl ToString) -> Self {
        Self::DataLoading(msg.to_string())
    }

    /// Create a balance calculation error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::BalanceCalculation` with the provided message
    #[must_use]
    pub fn balance_calculation(msg: impl ToString) -> Self {
        Self::BalanceCalculation(msg.to_string())
    }

    /// Create a sampling error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::Sampling` with the provided message
    #[must_use]
    pub fn sampling(msg: impl ToString) -> Self {
        Self::Sampling(msg.to_string())
    }

    /// Create a generation error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::Generation` with the provided message
    #[must_use]
    pub fn generation(msg: impl ToString) -> Self {
        Self::Generation(msg.to_string())
    }

    /// Create a validation error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::Validation` with the provided message
    #[must_use]
    pub fn validation(msg: impl ToString) -> Self {
        Self::Validation(msg.to_string())
    }

    /// Create a path resolution error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::PathResolution` with the provided message
    #[must_use]
    pub fn path_resolution(msg: impl ToString) -> Self {
        Self::PathResolution(msg.to_string())
    }

    /// Create a CLI argument error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::CliArgument` with the provided message
    #[must_use]
    pub fn cli_argument(msg: impl ToString) -> Self {
        Self::CliArgument(msg.to_string())
    }

    /// Create an invalid operation error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::InvalidOperation` with the provided message
    #[must_use]
    pub fn invalid_operation(msg: impl ToString) -> Self {
        Self::InvalidOperation(msg.to_string())
    }

    /// Create a missing data error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::MissingData` with the provided message
    #[must_use]
    pub fn missing_data(msg: impl ToString) -> Self {
        Self::MissingData(msg.to_string())
    }

    /// Create an invalid format error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::InvalidFormat` with the provided message
    #[must_use]
    pub fn invalid_format(msg: impl ToString) -> Self {
        Self::InvalidFormat(msg.to_string())
    }
    
    /// Create an invalid date error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::InvalidDate` with the provided message
    #[must_use]
    pub fn invalid_date(msg: impl ToString) -> Self {
        Self::InvalidDate(msg.to_string())
    }
    
    /// Create a plotting error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::Plotting` with the provided message
    #[must_use]
    pub fn plotting(msg: impl ToString) -> Self {
        Self::Plotting(msg.to_string())
    }
    
    /// Create a covariate error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::Covariate` with the provided message
    #[must_use]
    pub fn covariate(msg: impl ToString) -> Self {
        Self::Covariate(msg.to_string())
    }
    
    /// Create an invalid criteria error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::InvalidCriteria` with the provided message
    #[must_use]
    pub fn invalid_criteria(msg: impl ToString) -> Self {
        Self::InvalidCriteria(msg.to_string())
    }
    
    /// Create an IO error with a message
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::Io` with the provided message
    #[must_use]
    pub fn io_error(msg: impl ToString) -> Self {
        Self::Io(std::io::Error::new(std::io::ErrorKind::Other, msg.to_string()))
    }

    /// Create a general error
    /// 
    /// # Arguments
    /// * `msg` - The error message
    /// 
    /// # Returns
    /// A new `IdsError::Other` with the provided message
    #[must_use]
    pub fn other(msg: impl ToString) -> Self {
        Self::Other(msg.to_string())
    }
    
    /// Create a logger error
    /// 
    /// # Returns
    /// A new `IdsError::Logger` error
    #[must_use]
    pub fn logger() -> Self {
        Self::Logger
    }
}

/// Type alias for Result with IdsError
pub type Result<T> = std::result::Result<T, IdsError>;

/// Enhanced error handling with better context support
/// 
/// This module provides easy integration with anyhow and color-eyre for richer error contexts.
/// 
/// # Examples
/// 
/// ## Using anyhow context
/// ```
/// use types::error::{Result, AnyhowResultType};
/// use anyhow::Context;
/// 
/// fn read_config(path: &str) -> AnyhowResultType<String> {
///     std::fs::read_to_string(path)
///         .context(format!("Failed to read configuration file at {path}"))
/// }
/// ```
/// 
/// ## Using color-eyre
/// ```
/// use types::error::{Result, EyreResult};
/// use color_eyre::eyre::WrapErr;
/// 
/// fn process_data(path: &str) -> EyreResult<()> {
///     let data = std::fs::read_to_string(path)
///         .wrap_err(format!("Failed to read data file at {path}"))?;
///     // Process data...
///     Ok(())
/// }
/// ```
/// 
/// ## Converting to IdsError
/// ```
/// use types::error::{Result, IdsError};
/// use anyhow::Context;
/// 
/// fn load_config(path: &str) -> Result<String> {
///     std::fs::read_to_string(path)
///         .context(format!("Failed to read configuration file at {path}"))
///         .map_err(IdsError::from)
/// }
/// ```

// Type aliases for backwards compatibility
pub type SamplingError = IdsError;
pub type PlottingError = IdsError;
pub type DataGenError = IdsError;

/// Add a prelude for convenient imports
pub mod prelude {
    pub use super::{
        IdsError, 
        Result, 
        ErrorContext,
        anyhow,
        AnyhowResultType,
        Context,
        Report,
        EyreResult
    };
    
    /// Re-export color_eyre's WrapErr trait for better error messages
    pub use color_eyre::eyre::WrapErr;
    
    /// Setup function to initialize color-eyre for pretty error reports
    ///
    /// Call this function at the beginning of your program to enable color-eyre's
    /// pretty error reporting with backtraces.
    ///
    /// # Example
    /// ```
    /// use types::error::prelude::*;
    ///
    /// fn main() -> EyreResult<()> {
    ///     install_color_eyre()?;
    ///
    ///     // Your program code...
    ///     Ok(())
    /// }
    /// ```
    pub fn install_color_eyre() -> color_eyre::Result<()> {
        color_eyre::install()
    }
}
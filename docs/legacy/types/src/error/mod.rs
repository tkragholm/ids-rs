pub use anyhow::{anyhow, Context, Result as AnyhowResultType};
pub use color_eyre::{eyre::Report, Result as EyreResult};
use std::error::Error as StdError;
use thiserror::Error;

// Submodules
mod context;
mod conversion;
#[cfg(doc)]
pub mod example; // Only included for documentation, not part of public API

mod macros;

// Re-export from submodules
pub use self::context::{with_context, with_context_details, ErrorContext, LegacyErrorContext};

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

    /// Arrow error with detailed context information
    #[error("Arrow error: {context}")]
    ArrowWithContext {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        context: String,
    },

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

    /// Data access error with source and context
    #[error("Data access error: {context}. Source: {source}")]
    DataAccess {
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
        context: String,
    },

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

    /// Schema-related errors (validation, mismatch, etc.)
    #[error("Schema error: {0}")]
    Schema(String),

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

    /// Create an invalid value error
    ///
    /// # Arguments
    /// * `msg` - The error message
    ///
    /// # Returns
    /// A new `IdsError::InvalidFormat` with the provided message about invalid values
    #[must_use]
    pub fn invalid_value(msg: impl ToString) -> Self {
        Self::InvalidFormat(format!("Invalid value: {}", msg.to_string()))
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
        Self::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            msg.to_string(),
        ))
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

    /// Create a new type conversion error
    ///
    /// # Arguments
    /// * `msg` - The error message
    ///
    /// # Returns
    /// A new error with type conversion context
    #[must_use]
    pub fn type_conversion(msg: impl ToString) -> Self {
        Self::DataLoading(format!("Type conversion: {}", msg.to_string()))
    }

    /// Create a new column not found error
    ///
    /// # Arguments
    /// * `msg` - The error message
    ///
    /// # Returns
    /// A new error with column not found context
    #[must_use]
    pub fn column_not_found(msg: impl ToString) -> Self {
        Self::DataLoading(format!("Column not found: {}", msg.to_string()))
    }

    /// Create a new index out of bounds error
    ///
    /// # Arguments
    /// * `msg` - The error message
    ///
    /// # Returns
    /// A new error with index out of bounds context
    #[must_use]
    pub fn index_out_of_bounds(msg: impl ToString) -> Self {
        Self::DataLoading(format!("Index out of bounds: {}", msg.to_string()))
    }

    /// Create a new date conversion error
    ///
    /// # Arguments
    /// * `msg` - The error message
    ///
    /// # Returns
    /// A new error with date conversion context
    #[must_use]
    pub fn date_conversion(msg: impl ToString) -> Self {
        Self::DataLoading(format!("Date conversion: {}", msg.to_string()))
    }

    /// Create a new missing value error
    ///
    /// # Arguments
    /// * `msg` - The error message
    ///
    /// # Returns
    /// A new error for missing values
    #[must_use]
    pub fn missing_value(msg: impl ToString) -> Self {
        Self::MissingData(format!("Missing value: {}", msg.to_string()))
    }

    /// Create a data access error with source and context
    ///
    /// # Arguments
    /// * `err` - The source error
    /// * `context` - The context describing what was happening when the error occurred
    ///
    /// # Returns
    /// A new `IdsError::DataAccess` with the provided source and context
    #[must_use]
    pub fn data_access<E, S>(err: E, context: S) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
        S: AsRef<str>,
    {
        Self::DataAccess {
            source: Box::new(err),
            context: context.as_ref().to_string(),
        }
    }

    /// Create an Arrow error with source and context
    ///
    /// # Arguments
    /// * `err` - The source error
    /// * `context` - The context describing what was happening when the error occurred
    ///
    /// # Returns
    /// A new `IdsError::ArrowWithContext` with the provided source and context
    #[must_use]
    pub fn arrow<E, S>(err: E, context: S) -> Self
    where
        E: std::error::Error + Send + Sync + 'static,
        S: AsRef<str>,
    {
        Self::ArrowWithContext {
            source: Box::new(err),
            context: context.as_ref().to_string(),
        }
    }

    /// Create an error for missing or invalid register data
    ///
    /// # Arguments
    /// * `register_type` - The type of register (e.g., "AKM", "BEF")
    /// * `details` - Additional details about what's missing
    ///
    /// # Returns
    /// A new `IdsError::DataLoading` with a formatted message
    #[must_use]
    pub fn register_data(register_type: impl ToString, details: impl ToString) -> Self {
        Self::DataLoading(format!(
            "Invalid {} register data: {}",
            register_type.to_string(),
            details.to_string()
        ))
    }

    /// Create an error for schema mismatches
    ///
    /// # Arguments
    /// * `expected` - The expected schema or field
    /// * `actual` - The actual schema or field found (or "not found")
    ///
    /// # Returns
    /// A new `IdsError::Schema` with a formatted message
    #[must_use]
    pub fn schema_mismatch(expected: impl ToString, actual: impl ToString) -> Self {
        Self::Schema(format!(
            "Schema mismatch: expected {}, found {}",
            expected.to_string(),
            actual.to_string()
        ))
    }

    /// Create an error for failed data lookup operations
    ///
    /// # Arguments
    /// * `entity_type` - The type of entity being looked up (PNR, column, etc.)
    /// * `identifier` - The identifier that wasn't found
    /// * `source` - The source being searched (register, dataset, etc.)
    ///
    /// # Returns
    /// A new `IdsError::MissingData` with a formatted message
    #[must_use]
    pub fn lookup_failed(
        entity_type: impl ToString,
        identifier: impl ToString,
        source: impl ToString,
    ) -> Self {
        Self::MissingData(format!(
            "Failed to find {} '{}' in {}",
            entity_type.to_string(),
            identifier.to_string(),
            source.to_string()
        ))
    }

    /// Create an error for invalid data types or conversions
    ///
    /// # Arguments
    /// * `source_type` - The source data type
    /// * `target_type` - The target data type
    /// * `details` - Optional additional details about the conversion failure
    ///
    /// # Returns
    /// A new `IdsError::TypeConversion` with a formatted message
    #[must_use]
    pub fn type_conversion_detailed(
        source_type: impl ToString,
        target_type: impl ToString,
        details: Option<impl ToString>,
    ) -> Self {
        let msg = if let Some(details) = details {
            format!(
                "Failed to convert {} to {}: {}",
                source_type.to_string(),
                target_type.to_string(),
                details.to_string()
            )
        } else {
            format!(
                "Failed to convert {} to {}",
                source_type.to_string(),
                target_type.to_string()
            )
        };

        Self::type_conversion(msg)
    }
}

/// Type alias for Result with `IdsError`
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
/// ## Converting to `IdsError`
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
        anyhow, AnyhowResultType, Context, ErrorContext, EyreResult, IdsError, LegacyErrorContext,
        Report, Result,
    };

    // Re-export macros defined at crate root
    pub use crate::{bail, ensure, try_with_context};

    /// Re-export `color_eyre`'s `WrapErr` trait for better error messages
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

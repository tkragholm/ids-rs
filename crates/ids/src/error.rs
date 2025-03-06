use datagen::DataGenError;
use std::fmt;

/// Custom error type for the ids crate
#[derive(Debug)]
pub enum IdsError {
    Io(std::io::Error),
    Config(String),
    DataLoading(String),
    BalanceCalculation(String),
    Sampling(String),
    Generation(String),
    Validation(String),
    PathResolution(String),
    CliArgument(String),
    LogInit(log::SetLoggerError),
    Covariate(Box<dyn std::error::Error>),
    Types(types::error::IdsError),
    Datagen(DataGenError),
    Serialization(serde_json::Error),
    Other(String),
}

// Implement Display trait for IdsError
impl fmt::Display for IdsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(e) => write!(f, "IO error: {e}"),
            Self::Config(msg) => write!(f, "Configuration error: {msg}"),
            Self::DataLoading(msg) => write!(f, "Data loading error: {msg}"),
            Self::BalanceCalculation(msg) => write!(f, "Balance calculation error: {msg}"),
            Self::Sampling(msg) => write!(f, "Data sampling error: {msg}"),
            Self::Generation(msg) => write!(f, "Register generation error: {msg}"),
            Self::Validation(msg) => write!(f, "Validation error: {msg}"),
            Self::PathResolution(msg) => write!(f, "Path resolution error: {msg}"),
            Self::CliArgument(msg) => write!(f, "CLI argument error: {msg}"),
            Self::LogInit(e) => write!(f, "Log initialization error: {e}"),
            Self::Covariate(e) => write!(f, "Covariate error: {e}"),
            Self::Types(e) => write!(f, "Types error: {e}"),
            Self::Datagen(e) => write!(f, "Datagen error: {e}"),
            Self::Serialization(e) => write!(f, "Serialization error: {e}"),
            Self::Other(msg) => write!(f, "Other error: {msg}"),
        }
    }
}

// Implement the std::error::Error trait for IdsError
impl std::error::Error for IdsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(e) => Some(e),
            Self::LogInit(e) => Some(e),
            Self::Covariate(e) => Some(e.as_ref()),
            Self::Types(e) | Self::Datagen(e) => Some(e),
            Self::Serialization(e) => Some(e),
            _ => None,
        }
    }
}

// Implement From traits for automatic conversions
impl From<std::io::Error> for IdsError {
    fn from(error: std::io::Error) -> Self {
        Self::Io(error)
    }
}

impl From<log::SetLoggerError> for IdsError {
    fn from(error: log::SetLoggerError) -> Self {
        Self::LogInit(error)
    }
}

// We need to handle types::error::IdsError explicitly
impl From<types::error::IdsError> for IdsError {
    fn from(error: types::error::IdsError) -> Self {
        Self::Types(error)
    }
}

// For DataGenError, we need to use a wrapper method instead of From implementation
// due to the potential conflict with types::error::IdsError
impl IdsError {
    /// Creates a new `IdsError` from a `DataGenError`
    /// 
    /// # Returns
    /// A new `IdsError` containing the `DataGenError`
    #[must_use]
    pub const fn from_datagen_error(error: DataGenError) -> Self {
        Self::Datagen(error)
    }
}

impl From<serde_json::Error> for IdsError {
    fn from(error: serde_json::Error) -> Self {
        Self::Serialization(error)
    }
}

// Convenience methods for creating errors
impl IdsError {
    /// Creates a new configuration error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::Config` with the provided message
    #[must_use]
    pub fn config<S: ToString>(msg: S) -> Self {
        Self::Config(msg.to_string())
    }

    /// Creates a new data loading error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::DataLoading` with the provided message
    #[must_use]
    pub fn data_loading<S: ToString>(msg: S) -> Self {
        Self::DataLoading(msg.to_string())
    }

    /// Creates a new balance calculation error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::BalanceCalculation` with the provided message
    #[must_use]
    pub fn balance_calculation<S: ToString>(msg: S) -> Self {
        Self::BalanceCalculation(msg.to_string())
    }

    /// Creates a new sampling error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::Sampling` with the provided message
    #[must_use]
    pub fn sampling<S: ToString>(msg: S) -> Self {
        Self::Sampling(msg.to_string())
    }

    /// Creates a new generation error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::Generation` with the provided message
    #[must_use]
    pub fn generation<S: ToString>(msg: S) -> Self {
        Self::Generation(msg.to_string())
    }

    /// Creates a new validation error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::Validation` with the provided message
    #[must_use]
    pub fn validation<S: ToString>(msg: S) -> Self {
        Self::Validation(msg.to_string())
    }

    /// Creates a new path resolution error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::PathResolution` with the provided message
    #[must_use]
    pub fn path_resolution<S: ToString>(msg: S) -> Self {
        Self::PathResolution(msg.to_string())
    }

    /// Creates a new CLI argument error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::CliArgument` with the provided message
    #[must_use]
    pub fn cli_argument<S: ToString>(msg: S) -> Self {
        Self::CliArgument(msg.to_string())
    }

    /// Creates a new generic error with the given message
    /// 
    /// # Returns
    /// A new `IdsError::Other` with the provided message
    #[must_use]
    pub fn other<S: ToString>(msg: S) -> Self {
        Self::Other(msg.to_string())
    }
    
    /// Creates a new covariate error with the given error
    /// 
    /// # Returns
    /// A new `IdsError::Covariate` containing the boxed error
    #[must_use]
    pub fn covariate<E: std::error::Error + 'static>(err: E) -> Self {
        Self::Covariate(Box::new(err))
    }
}

// Result type alias for convenience
pub type IdsResult<T> = Result<T, IdsError>;

/// Trait to convert any error to `IdsError`
///
/// # Errors
/// Returns an `IdsError` with a message that combines the provided message and the original error
pub trait IntoIdsError<T> {
    /// Converts an error into an `IdsError` with an additional message
    ///
    /// # Errors
    /// Returns an `IdsError` containing the original error message prefixed with the provided message
    fn into_ids_error<S: ToString>(self, msg: S) -> Result<T, IdsError>;
}

// Implement for all types that can be converted to a standard error
impl<T, E: std::error::Error + 'static> IntoIdsError<T> for Result<T, E> {
    fn into_ids_error<S: ToString>(self, msg: S) -> Result<T, IdsError> {
        self.map_err(|e| IdsError::Other(format!("{}: {e}", msg.to_string())))
    }
}
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
            IdsError::Io(e) => write!(f, "IO error: {}", e),
            IdsError::Config(msg) => write!(f, "Configuration error: {}", msg),
            IdsError::DataLoading(msg) => write!(f, "Data loading error: {}", msg),
            IdsError::BalanceCalculation(msg) => write!(f, "Balance calculation error: {}", msg),
            IdsError::Sampling(msg) => write!(f, "Data sampling error: {}", msg),
            IdsError::Generation(msg) => write!(f, "Register generation error: {}", msg),
            IdsError::Validation(msg) => write!(f, "Validation error: {}", msg),
            IdsError::PathResolution(msg) => write!(f, "Path resolution error: {}", msg),
            IdsError::CliArgument(msg) => write!(f, "CLI argument error: {}", msg),
            IdsError::LogInit(e) => write!(f, "Log initialization error: {}", e),
            IdsError::Covariate(e) => write!(f, "Covariate error: {}", e),
            IdsError::Types(e) => write!(f, "Types error: {}", e),
            IdsError::Datagen(e) => write!(f, "Datagen error: {}", e),
            IdsError::Serialization(e) => write!(f, "Serialization error: {}", e),
            IdsError::Other(msg) => write!(f, "Other error: {}", msg),
        }
    }
}

// Implement the std::error::Error trait for IdsError
impl std::error::Error for IdsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IdsError::Io(e) => Some(e),
            IdsError::LogInit(e) => Some(e),
            IdsError::Covariate(e) => Some(e.as_ref()),
            IdsError::Types(e) => Some(e),
            IdsError::Datagen(e) => Some(e),
            IdsError::Serialization(e) => Some(e),
            _ => None,
        }
    }
}

// Implement From traits for automatic conversions
impl From<std::io::Error> for IdsError {
    fn from(error: std::io::Error) -> Self {
        IdsError::Io(error)
    }
}

impl From<log::SetLoggerError> for IdsError {
    fn from(error: log::SetLoggerError) -> Self {
        IdsError::LogInit(error)
    }
}

// We need to handle types::error::IdsError explicitly
impl From<types::error::IdsError> for IdsError {
    fn from(error: types::error::IdsError) -> Self {
        IdsError::Types(error)
    }
}

// For DataGenError, we need to use a wrapper method instead of From implementation
// due to the potential conflict with types::error::IdsError
impl IdsError {
    pub fn from_datagen_error(error: DataGenError) -> Self {
        IdsError::Datagen(error)
    }
}

impl From<serde_json::Error> for IdsError {
    fn from(error: serde_json::Error) -> Self {
        IdsError::Serialization(error)
    }
}

// Convenience methods for creating errors
impl IdsError {
    pub fn config<S: ToString>(msg: S) -> Self {
        IdsError::Config(msg.to_string())
    }

    pub fn data_loading<S: ToString>(msg: S) -> Self {
        IdsError::DataLoading(msg.to_string())
    }

    pub fn balance_calculation<S: ToString>(msg: S) -> Self {
        IdsError::BalanceCalculation(msg.to_string())
    }

    pub fn sampling<S: ToString>(msg: S) -> Self {
        IdsError::Sampling(msg.to_string())
    }

    pub fn generation<S: ToString>(msg: S) -> Self {
        IdsError::Generation(msg.to_string())
    }

    pub fn validation<S: ToString>(msg: S) -> Self {
        IdsError::Validation(msg.to_string())
    }

    pub fn path_resolution<S: ToString>(msg: S) -> Self {
        IdsError::PathResolution(msg.to_string())
    }

    pub fn cli_argument<S: ToString>(msg: S) -> Self {
        IdsError::CliArgument(msg.to_string())
    }

    pub fn other<S: ToString>(msg: S) -> Self {
        IdsError::Other(msg.to_string())
    }
    
    pub fn covariate<E: std::error::Error + 'static>(err: E) -> Self {
        IdsError::Covariate(Box::new(err))
    }
}

// Result type alias for convenience
pub type IdsResult<T> = Result<T, IdsError>;

// Trait to convert any error to IdsError
pub trait IntoIdsError<T> {
    fn into_ids_error<S: ToString>(self, msg: S) -> Result<T, IdsError>;
}

// Implement for all types that can be converted to a standard error
impl<T, E: std::error::Error + 'static> IntoIdsError<T> for Result<T, E> {
    fn into_ids_error<S: ToString>(self, msg: S) -> Result<T, IdsError> {
        self.map_err(|e| IdsError::Other(format!("{}: {}", msg.to_string(), e)))
    }
}
use std::error::Error as StdError;
use std::fmt;
use std::io;

/// The main error type for the IDS-RS crate
#[derive(Debug)]
pub enum IdsError {
    /// IO-related errors
    Io(io::Error),

    /// Validation errors when data doesn't meet requirements
    Validation(String),

    /// Data processing errors
    Data(String),

    /// Errors that occur during computation
    Computation(String),

    /// Errors from external dependencies
    External(Box<dyn StdError + Send + Sync>),

    /// Arrow errors with context
    ArrowWithContext {
        source: Box<dyn StdError + Send + Sync>,
        context: String,
    },
}

impl fmt::Display for IdsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => write!(f, "IO error: {err}"),
            Self::Validation(msg) => write!(f, "Validation error: {msg}"),
            Self::Data(msg) => write!(f, "Data error: {msg}"),
            Self::Computation(msg) => write!(f, "Computation error: {msg}"),
            Self::External(err) => write!(f, "External error: {err}"),
            Self::ArrowWithContext { source, context } => {
                write!(f, "Arrow error: {source} (context: {context})")
            }
        }
    }
}

impl StdError for IdsError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::Io(err) => Some(err),
            Self::External(err) => Some(err.as_ref()),
            Self::ArrowWithContext { source, .. } => Some(source.as_ref()),
            _ => None,
        }
    }
}

impl From<io::Error> for IdsError {
    fn from(err: io::Error) -> Self {
        Self::Io(err)
    }
}

impl From<arrow::error::ArrowError> for IdsError {
    fn from(err: arrow::error::ArrowError) -> Self {
        Self::External(Box::new(err))
    }
}

impl From<datafusion::error::DataFusionError> for IdsError {
    fn from(err: datafusion::error::DataFusionError) -> Self {
        Self::External(Box::new(err))
    }
}

// Remove this impl since it's not allowed to implement traits for types not in current crate

// Helper functions
pub fn validation_error(message: impl ToString) -> IdsError {
    IdsError::Validation(message.to_string())
}

pub fn data_error(message: impl ToString) -> IdsError {
    IdsError::Data(message.to_string())
}

pub fn computation_error(message: impl ToString) -> IdsError {
    IdsError::Computation(message.to_string())
}

/// Result type alias for `IdsError`
pub type Result<T> = std::result::Result<T, IdsError>;

/// Extension trait for adding context to error results
pub trait ErrorContext<T, E> {
    /// Add context to an error result
    fn with_context<C, F>(self, context_fn: F) -> Result<T>
    where
        C: ToString,
        F: FnOnce() -> C;
}

impl<T, E: StdError + Send + Sync + 'static> ErrorContext<T, E> for std::result::Result<T, E> {
    fn with_context<C, F>(self, context_fn: F) -> Result<T>
    where
        C: ToString,
        F: FnOnce() -> C,
    {
        self.map_err(|e| {
            let _context = context_fn().to_string();
            IdsError::External(Box::new(e))
        })
    }
}

use thiserror::Error;

/// Main error type for utility functions
#[derive(Error, Debug)]
pub enum UtilsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("Date parsing error: {0}")]
    DateParse(String),
    
    #[error("Configuration error: {0}")]
    Config(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Logging error: {0}")]
    Logging(String),
}

/// Type alias for Result with UtilsError as error type
pub type Result<T> = std::result::Result<T, UtilsError>;

/// Convert any error type that implements Display into a UtilsError
/// with a custom context message
pub fn into_error<E>(error: E, context: &str) -> UtilsError
where
    E: std::fmt::Display,
{
    // Create a validation error with the message
    UtilsError::Validation(format!("{}: {}", context, error))
}

/// Trait to facilitate converting Result types to Result<T, UtilsError>
pub trait IntoResult<T> {
    /// Convert a result to Result<T, UtilsError> with a custom context message
    fn with_context(self, context: &str) -> Result<T>;
}

/// Implement IntoResult for any Result where the error implements Display
impl<T, E: std::fmt::Display + 'static> IntoResult<T> for std::result::Result<T, E> {
    fn with_context(self, context: &str) -> Result<T> {
        self.map_err(|e| into_error(e, context))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_into_error_with_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let utils_error = into_error(io_error, "Failed to read file");
        assert!(matches!(utils_error, UtilsError::Io(_)));
    }
    
    #[test]
    fn test_into_error_with_generic_error() {
        let utils_error = into_error("custom error", "Operation failed");
        assert!(matches!(utils_error, UtilsError::Validation(_)));
        if let UtilsError::Validation(msg) = utils_error {
            assert_eq!(msg, "Operation failed: custom error");
        }
    }
    
    #[test]
    fn test_with_context() {
        let result: std::result::Result<(), &str> = Err("Something went wrong");
        let utils_result = result.with_context("Operation failed");
        assert!(matches!(utils_result, Err(UtilsError::Validation(_))));
    }
}
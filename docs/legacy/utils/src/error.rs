// Re-export IdsError from the types crate
pub use types::error::{IdsError, LegacyErrorContext as Context, Result};

// Helper functions for specific error types
pub fn date_parse_error<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::InvalidDate(msg.to_string())
}

pub fn config_error<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::Config(msg.to_string())
}

pub fn validation_error<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::Validation(msg.to_string())
}

pub fn logging_error<T: std::fmt::Display>(msg: T) -> IdsError {
    // Since Logging variant was removed, use Other variant instead
    IdsError::other(format!("Logging error: {msg}"))
}

// Simplified helper for converting errors with context
pub fn with_context<T, E>(result: std::result::Result<T, E>, context: String) -> Result<T>
where
    E: std::fmt::Display + 'static,
{
    result.with_context(move || context)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_with_context_for_io_error() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let result: std::result::Result<(), _> = Err(io_error);
        let ids_result = result.with_context(|| "Failed to read file");
        assert!(matches!(ids_result, Err(IdsError::Io(_))));
    }

    #[test]
    fn test_with_context_for_generic_error() {
        let result: std::result::Result<(), &str> = Err("custom error");
        let ids_result = result.with_context(|| "Operation failed");
        assert!(matches!(ids_result, Err(IdsError::Validation(_))));
        if let Err(IdsError::Validation(msg)) = ids_result {
            assert!(msg.contains("Operation failed"));
            assert!(msg.contains("custom error"));
        }
    }

    #[test]
    fn test_helper_functions() {
        let error = validation_error("Invalid input");
        assert!(matches!(error, IdsError::Validation(_)));

        let error = config_error("Missing configuration");
        assert!(matches!(error, IdsError::Config(_)));

        let error = date_parse_error("Invalid date format");
        assert!(matches!(error, IdsError::InvalidDate(_)));
    }
}

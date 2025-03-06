use std::fmt::Display;
use super::{IdsError, Result};

/// Trait for adding context to errors
pub trait ErrorContext<T, E> {
    /// Add context to an error, explaining what was happening when the error occurred
    /// 
    /// # Example
    /// ```
    /// use types::error::ErrorContext;
    /// 
    /// let result = std::fs::read_to_string("missing_file.txt")
    ///     .with_context(|| "Failed to read configuration file");
    /// ```
    /// 
    /// # Errors
    /// Returns the original error wrapped with the provided context
    fn with_context<C, F>(self, context_fn: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: Display + Send + Sync + 'static;
    
    /// Add context with details to an error, allowing for formatted messages
    ///
    /// # Example
    /// ```
    /// use types::error::ErrorContext;
    /// 
    /// let file_path = "data/config.json";
    /// let result = std::fs::read_to_string(file_path)
    ///     .with_context_details(|| format!("Failed to read file at {file_path}"));
    /// ```
    ///
    /// # Errors
    /// Returns the original error replaced with the provided context
    fn with_context_details<C, F>(self, context_fn: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: Display + Send + Sync + 'static;
}

/// Helper function to add context to Result types
#[inline]
pub fn with_context<T, E, C, F>(
    result: std::result::Result<T, E>,
    context_fn: F,
) -> Result<T>
where
    E: Display + 'static,
    F: FnOnce() -> C,
    C: Display,
{
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            // Special case for io::Error to preserve its type information
            if let Some(io_err) = (&e as &dyn std::any::Any).downcast_ref::<std::io::Error>() {
                let ctx = context_fn();
                Err(IdsError::Io(std::io::Error::new(
                    io_err.kind(),
                    format!("{}: {}", ctx, e),
                )))
            } else {
                // General case for other errors
                let ctx = context_fn();
                Err(IdsError::Validation(format!("{}: {}", ctx, e)))
            }
        }
    }
}

/// Helper function to replace error with context for Result types
#[inline]
pub fn with_context_details<T, E, C, F>(
    result: std::result::Result<T, E>,
    context_fn: F,
) -> Result<T>
where
    E: Display + 'static,
    F: FnOnce() -> C,
    C: Display,
{
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            // Special case for io::Error to preserve its type information
            if let Some(io_err) = (&e as &dyn std::any::Any).downcast_ref::<std::io::Error>() {
                let ctx = context_fn();
                Err(IdsError::Io(std::io::Error::new(
                    io_err.kind(),
                    format!("{}", ctx),
                )))
            } else {
                // For other errors, just use the context message
                let ctx = context_fn();
                Err(IdsError::Validation(ctx.to_string()))
            }
        }
    }
}

// Implement ErrorContext for all Result types using the helper functions
impl<T, E: Display + 'static> ErrorContext<T, E> for std::result::Result<T, E> {
    fn with_context<C, F>(self, context_fn: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: Display + Send + Sync + 'static,
    {
        with_context(self, context_fn)
    }

    fn with_context_details<C, F>(self, context_fn: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: Display + Send + Sync + 'static,
    {
        with_context_details(self, context_fn)
    }
}
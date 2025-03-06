use std::fmt::Display;
use std::error::Error as StdError;
use super::{IdsError, Result};

/// Trait for adding context to errors
///
/// This trait provides methods for adding context to error results.
pub trait ErrorContext<T, E> {
    /// Add context to an error, explaining what was happening when the error occurred
    /// 
    /// This method preserves the original error as the source, allowing for inspection
    /// of the error chain while providing better context about the operation that failed.
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
    /// This method is particularly useful when you need to include specific details about the 
    /// error context, such as file paths, ids, or parameters that would help diagnose the issue.
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

/// Legacy version of ErrorContext that doesn't require thread-safety bounds
/// 
/// This trait is provided for backwards compatibility with existing code.
pub trait LegacyErrorContext<T, E: Display + 'static> {
    /// Add context to an error
    fn with_context<C, F>(self, context_fn: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: Display;
}

/// Maps an error type to the appropriate IdsError variant based on common patterns
fn map_error_type<E: StdError + Send + Sync + 'static>(error: E, context: &str) -> IdsError {
    // Get the error as a trait object for pattern matching
    let error_ref = &error as &dyn StdError;
    
    // Check for common error types and map to appropriate variants
    if let Some(io_err) = error_ref.downcast_ref::<std::io::Error>() {
        // Preserve io::Error kind
        IdsError::Io(std::io::Error::new(
            io_err.kind(),
            format!("{}: {}", context, error),
        ))
    } else if let Some(_arrow_err) = error_ref.downcast_ref::<arrow::error::ArrowError>() {
        // Arrow errors get special treatment with the ArrowWithContext variant
        IdsError::ArrowWithContext {
            source: Box::new(error),
            context: context.to_string(),
        }
    } else if let Some(_parquet_err) = error_ref.downcast_ref::<parquet::errors::ParquetError>() {
        // Parquet errors are also wrapped in ArrowWithContext
        IdsError::ArrowWithContext {
            source: Box::new(error),
            context: context.to_string(),
        }
    } else if let Some(_csv_err) = error_ref.downcast_ref::<csv::Error>() {
        // CSV errors get converted to CSV variant
        IdsError::Csv(csv::Error::from(std::io::Error::new(
            std::io::ErrorKind::Other, 
            format!("{}: {}", context, error)
        )))
    } else if let Some(_date_err) = error_ref.downcast_ref::<chrono::format::ParseError>() {
        // Date parsing errors
        IdsError::InvalidDate(format!("{}: {}", context, error))
    } else if let Some(_parse_int_err) = error_ref.downcast_ref::<std::num::ParseIntError>() {
        // Integer parsing errors
        IdsError::type_conversion(format!("{}: {}", context, error))
    } else if let Some(_parse_float_err) = error_ref.downcast_ref::<std::num::ParseFloatError>() {
        // Float parsing errors
        IdsError::type_conversion(format!("{}: {}", context, error))
    } else if let Some(_json_err) = error_ref.downcast_ref::<serde_json::Error>() {
        // JSON parsing errors - create a proper JSON error
        IdsError::invalid_format(format!("{}: {}", context, error))
    } else {
        // For any other error type, use DataAccess with rich context
        IdsError::DataAccess {
            source: Box::new(error),
            context: context.to_string(),
        }
    }
}

/// Helper function to add context to Result types
///
/// This function preserves type information for known error types while adding context.
/// It's designed to be more intelligent about error conversion than a simple map_err.
#[inline]
pub fn with_context<T, E, C, F>(
    result: std::result::Result<T, E>,
    context_fn: F,
) -> Result<T>
where
    E: StdError + Send + Sync + 'static,
    F: FnOnce() -> C,
    C: Display,
{
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            // Get context value
            let ctx = context_fn().to_string();
            
            // Create a descriptive error message that combines all previous context
            let error_description = format!("{}", e);
            
            // Create an appropriate IdsError that preserves context
            // Check if we're already dealing with an IdsError
            let new_error = if let Some(ids_err) = (&e as &dyn StdError).downcast_ref::<super::IdsError>() {
                match ids_err {
                    // For Validation errors, preserve the error type with chained context
                    super::IdsError::Validation(_) => {
                        super::IdsError::Validation(format!("{}: {}", ctx, error_description))
                    },
                    // For DataAccess errors, preserve their structure and source
                    super::IdsError::DataAccess { source: _, context: _ } => {
                        // Create a new DataAccess error with updated context
                        let new_context = format!("{}: {}", ctx, error_description);
                        let boxed_error = Box::new(std::io::Error::new(std::io::ErrorKind::Other, error_description));
                        super::IdsError::DataAccess {
                            source: boxed_error,
                            context: new_context,
                        }
                    },
                    // Use standard mapping for other IdsError types
                    _ => map_error_type(e, &ctx)
                }
            } else {
                // For non-IdsError types, use the standard mapping
                map_error_type(e, &ctx)
            };
            
            Err(new_error)
        }
    }
}

/// Helper function to replace error with context for Result types
///
/// Unlike with_context, this function is designed to replace the error message entirely
/// with the provided context, rather than adding to it. This is useful when the original
/// error message isn't helpful or relevant.
#[inline]
pub fn with_context_details<T, E, C, F>(
    result: std::result::Result<T, E>,
    context_fn: F,
) -> Result<T>
where
    E: StdError + Send + Sync + 'static,
    F: FnOnce() -> C,
    C: Display,
{
    match result {
        Ok(value) => Ok(value),
        Err(e) => {
            // Get context value
            let ctx = context_fn().to_string();
            
            // Check for specific error types
            let error_ref = &e as &dyn StdError;
            
            if let Some(io_err) = error_ref.downcast_ref::<std::io::Error>() {
                // Preserve io::Error kind but use only our context
                Err(IdsError::Io(std::io::Error::new(
                    io_err.kind(),
                    ctx,
                )))
            } else if let Some(_arrow_err) = error_ref.downcast_ref::<arrow::error::ArrowError>() {
                // For Arrow errors, use the DataAccess variant with context
                Err(IdsError::DataAccess {
                    source: Box::new(e),
                    context: ctx,
                })
            } else if let Some(_) = error_ref.downcast_ref::<parquet::errors::ParquetError>() {
                // For Parquet errors, create a generic DataAccess error with the source
                Err(IdsError::DataAccess {
                    source: Box::new(e),
                    context: ctx,
                })
            } else {
                // For most cases, use a validation error with just our context
                Err(IdsError::Validation(ctx))
            }
        }
    }
}

// Implement ErrorContext for all Result types using the helper functions
impl<T, E: StdError + Send + Sync + 'static> ErrorContext<T, E> for std::result::Result<T, E> {
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

// Add a simplified legacy context implementation
impl<T, E: Display + 'static> LegacyErrorContext<T, E> for std::result::Result<T, E> {
    fn with_context<C, F>(self, context_fn: F) -> Result<T>
    where
        F: FnOnce() -> C,
        C: Display,
    {
        match self {
            Ok(value) => Ok(value),
            Err(err) => {
                let context = context_fn().to_string();
                let message = format!("{}: {}", context, err);
                
                // Special case for io::Error to preserve behavior expected by the utils crate
                if let Some(io_err) = (&err as &dyn std::any::Any).downcast_ref::<std::io::Error>() {
                    return Err(IdsError::Io(std::io::Error::new(
                        io_err.kind(),
                        message,
                    )));
                }
                
                // For regular Display errors, create a Validation error to match expected behavior
                Err(IdsError::Validation(message))
            }
        }
    }
}
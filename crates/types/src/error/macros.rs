/// Ensure a condition is true, otherwise return with an error
///
/// This macro is inspired by the `assert!` macro but returns an error instead of panicking.
/// It has three forms:
///
/// 1. With a direct IdsError instance: `ensure!(val >= 0, IdsError::validation("Invalid value"))`
/// 2. With a simple string message: `ensure!(val >= 0, "Value must be non-negative")`
/// 3. With a formatted message: `ensure!(val < 100, "Value {} exceeds maximum allowed (100)", val)`
///
/// # Examples
///
/// ```
/// use types::error::{ensure, IdsError, Result};
///
/// fn process_value(val: i32) -> Result<()> {
///     // Return early if val < 0 with direct error
///     ensure!(val >= 0, IdsError::validation("Value must be non-negative"));
///
///     // Using simple message version (creates a Validation error)
///     ensure!(val < 1000, "Value exceeds maximum allowed");
///
///     // Using formatted message version (creates a Validation error)
///     ensure!(val < 100, "Value {} exceeds maximum allowed (100)", val);
///
///     Ok(())
/// }
/// ```
///
/// The macro is particularly useful for validating input parameters and preconditions
/// with clear and descriptive error messages. This improves debugging and makes
/// the code more self-documenting.
///
/// # Error Types
///
/// When using the string message forms, a `Validation` error is created.
/// For more specific error types, use the direct error form.
#[macro_export]
macro_rules! ensure {
    // Condition with direct error expression form
    ($condition:expr, $error:expr $(,)?) => {
        if !($condition) {
            return Err($error);
        }
    };
    // Condition with simple string message form
    ($condition:expr, $message:literal) => {
        if !($condition) {
            return Err($crate::error::IdsError::validation($message));
        }
    };
    // Condition with formatted message form
    ($condition:expr, $fmt:expr, $($arg:tt)*) => {
        if !($condition) {
            return Err($crate::error::IdsError::validation(format!($fmt, $($arg)*)));
        }
    };
}

/// Try to run an operation, with context if it fails
///
/// This macro wraps an operation that returns a Result, adding context
/// if the operation fails. It uses the `with_context` implementation to 
/// preserve error types where possible.
///
/// # Examples
///
/// ```
/// use types::error::{try_with_context, IdsError, Result};
///
/// fn read_config(path: &str) -> Result<String> {
///     let content = try_with_context!(
///         std::fs::read_to_string(path),
///         "Failed to read configuration file"
///     );
///
///     // Using formatted message version
///     let value = try_with_context!(
///         content.parse::<i32>(), 
///         "Failed to parse '{}' as integer from config", 
///         content.trim()
///     );
///
///     Ok(format!("Config value: {}", value))
/// }
/// ```
///
/// The macro is intelligent about error handling:
///
/// 1. For standard library errors (io, parse, etc.), it preserves their type info
/// 2. For Arrow errors, it uses the specialized ArrowWithContext variant
/// 3. For Parquet errors, it also uses ArrowWithContext for consistency
/// 4. For other errors, it creates a DataAccess error with the original as source
#[macro_export]
macro_rules! try_with_context {
    ($expr:expr, $context:expr) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return $crate::error::with_context(std::result::Result::Err(err), || $context);
            }
        }
    };
    ($expr:expr, $fmt:expr, $($arg:tt)*) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return $crate::error::with_context(
                    std::result::Result::Err(err), 
                    || format!($fmt, $($arg)*)
                );
            }
        }
    };
}

/// Bail out of a function early with an error
///
/// This macro returns early with an error, similar to `return Err(...);`.
/// It can be used in three ways:
///
/// 1. With a direct IdsError instance: `bail!(IdsError::validation("Invalid data"))`
/// 2. With a simple string message: `bail!("No data provided")` (creates a Validation error)
/// 3. With a formatted message: `bail!("Empty data: expected at least {} characters", 1)`
///
/// # Examples
///
/// ```
/// use types::error::{bail, IdsError, Result};
///
/// fn process_data(data: Option<&str>) -> Result<()> {
///     let data = match data {
///         Some(d) => d,
///         None => bail!("No data provided"),
///     };
///
///     // Using formatted message version
///     if data.is_empty() {
///         bail!("Empty data: expected at least {} characters", 1);
///     }
///
///     // Using direct error instance
///     if data.len() > 1000 {
///         bail!(IdsError::validation("Data exceeds maximum allowed size"));
///     }
///
///     Ok(())
/// }
/// ```
///
/// The macro's behavior can be customized by which variant you choose:
///
/// - For validation and precondition errors, prefer the string message form
/// - For specific error types, use the direct error instance form
/// - For complex error messages, use the formatted message form
#[macro_export]
macro_rules! bail {
    // Direct error expression form
    ($error:expr $(,)?) => {
        return Err($error);
    };
    // Simple string message form (creates a Validation error)
    ($message:literal) => {
        return Err($crate::error::IdsError::validation($message));
    };
    // Formatted message form (creates a Validation error)
    ($fmt:expr, $($arg:tt)*) => {
        return Err($crate::error::IdsError::validation(format!($fmt, $($arg)*)));
    };
}

// Re-export macros for the prelude - macros are automatically public since they're defined at crate level
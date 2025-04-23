// Re-export the types crate error related items
pub use types::error::{ErrorContext, IdsError, Result};

/// Alias for backward compatibility and consistency
pub type IdsResult<T> = Result<T>;

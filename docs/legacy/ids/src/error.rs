// Re-export the types crate error related items
pub use types::error::{IdsError, Result, ErrorContext};

// Alias for backward compatibility
pub type IdsResult<T> = Result<T>;
// Re-export from the types crate
pub use types::error::{LegacyErrorContext as Context, DataGenError, IdsError, Result};

// Simple alias for easier use within this crate
pub type Error = IdsError;

// Helper function for data generation errors
pub fn generation_error<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::Generation(msg.to_string())
}

// For backwards compatibility with existing code
pub trait IntoDataGenError<T> {
    fn into_datagen_error(self, msg: &str) -> Result<T>;
}

// Implement for any Result that can be converted to a String
impl<T, E: std::fmt::Display + 'static> IntoDataGenError<T> for std::result::Result<T, E> {
    fn into_datagen_error(self, msg: &str) -> Result<T> {
        self.with_context(|| format!("Data generation error: {}", msg))
    }
}
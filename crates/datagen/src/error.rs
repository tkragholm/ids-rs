pub use types::error::{DataGenError, IdsError};

// Re-export the main IdsError to make it easier to use in this crate
pub type Error = IdsError;

// For backwards compatibility with existing code
pub trait IntoDataGenError<T> {
    fn into_datagen_error(self, msg: &str) -> Result<T, DataGenError>;
}

// Implement for any Result that can be converted to a String
impl<T, E: std::fmt::Display> IntoDataGenError<T> for Result<T, E> {
    fn into_datagen_error(self, msg: &str) -> Result<T, DataGenError> {
        self.map_err(|e| DataGenError::generation(format!("{msg}: {e}")))
    }
}
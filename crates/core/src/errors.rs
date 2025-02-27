pub use types::error::IdsError;

// Type aliases to maintain compatibility
pub type PlottingError = IdsError;
pub type SamplingError = IdsError;

// Re-export the main IdsError to make it easier to use in this crate
pub type Error = IdsError;

// For backwards compatibility with existing code
pub trait IntoSamplingError<T> {
    fn into_sampling_error(self, msg: &str) -> Result<T, SamplingError>;
}

// Implement for any Result that can be converted to a String
impl<T, E: std::fmt::Display> IntoSamplingError<T> for Result<T, E> {
    fn into_sampling_error(self, msg: &str) -> Result<T, SamplingError> {
        self.map_err(|e| SamplingError::sampling(format!("{msg}: {e}")))
    }
}
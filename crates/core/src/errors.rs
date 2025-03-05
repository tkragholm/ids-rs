// Re-export from the types crate
pub use types::error::{ErrorContext as Context, IdsError, Result};

// Type aliases to maintain compatibility
pub type PlottingError = IdsError;
pub type SamplingError = IdsError;

// Simple alias for easier use within this crate
pub type Error = IdsError;

// Wrapper functions for common error types
pub fn invalid_date<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::InvalidDate(msg.to_string())
}

pub fn sampling_error<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::Sampling(msg.to_string())
}

pub fn invalid_criteria<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::InvalidCriteria(msg.to_string())
}

pub fn plotting_error<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::Plotting(msg.to_string())
}

pub const NO_ELIGIBLE_CONTROLS: IdsError = IdsError::NoEligibleControls;

// For backwards compatibility with existing code
pub trait IntoSamplingError<T> {
    fn into_sampling_error(self, msg: &str) -> Result<T>;
}

// Implement for any Result that can be converted to a String
impl<T, E: std::fmt::Display + 'static> IntoSamplingError<T> for std::result::Result<T, E> {
    fn into_sampling_error(self, msg: &str) -> Result<T> {
        self.with_context(|| format!("Sampling error: {msg}"))
    }
}

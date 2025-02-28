pub use types::error::IdsError;

// Type aliases to maintain compatibility
pub type PlottingError = IdsError;
pub type SamplingError = IdsError;

// Re-export the main IdsError to make it easier to use in this crate
pub type Error = IdsError;

// Wrapper functions for common error types
pub fn invalid_date<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::InvalidDate(msg.to_string())
}

pub fn sampling<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::Sampling(msg.to_string())
}

pub fn invalid_criteria<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::InvalidCriteria(msg.to_string())
}

pub fn plotting<T: std::fmt::Display>(msg: T) -> IdsError {
    IdsError::Plotting(msg.to_string())
}

pub const NO_ELIGIBLE_CONTROLS: IdsError = IdsError::NoEligibleControls;

// For backwards compatibility with existing code
pub trait IntoSamplingError<T> {
    fn into_sampling_error(self, msg: &str) -> Result<T, SamplingError>;
}

// Implement for any Result that can be converted to a String
impl<T, E: std::fmt::Display> IntoSamplingError<T> for Result<T, E> {
    fn into_sampling_error(self, msg: &str) -> Result<T, SamplingError> {
        self.map_err(|e| IdsError::Sampling(format!("{msg}: {e}")))
    }
}

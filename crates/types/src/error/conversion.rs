use std::convert::From;
use super::IdsError;

// Convert from log::SetLoggerError to IdsError
impl From<log::SetLoggerError> for IdsError {
    fn from(_: log::SetLoggerError) -> Self {
        IdsError::Logger
    }
}

// Add implementations for error types that don't have #[from] in the enum
// This file will be expanded as we identify more error types needing conversion
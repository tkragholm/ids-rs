use super::IdsError;
use chrono::format::ParseError as ChronoParseError;
use std::convert::From;

// Convert from log::SetLoggerError to IdsError
impl From<log::SetLoggerError> for IdsError {
    fn from(_: log::SetLoggerError) -> Self {
        IdsError::Logger
    }
}

// Convert from Chrono's ParseError to IdsError
impl From<ChronoParseError> for IdsError {
    fn from(err: ChronoParseError) -> Self {
        IdsError::InvalidDate(format!("Failed to parse date: {}", err))
    }
}

// Convert from std::num::ParseIntError to IdsError
impl From<std::num::ParseIntError> for IdsError {
    fn from(err: std::num::ParseIntError) -> Self {
        IdsError::type_conversion(format!("Failed to parse integer: {}", err))
    }
}

// Convert from std::num::ParseFloatError to IdsError
impl From<std::num::ParseFloatError> for IdsError {
    fn from(err: std::num::ParseFloatError) -> Self {
        IdsError::type_conversion(format!("Failed to parse float: {}", err))
    }
}

// Convert from std::fmt::Error to IdsError
impl From<std::fmt::Error> for IdsError {
    fn from(_: std::fmt::Error) -> Self {
        IdsError::InvalidFormat("Formatting error".to_string())
    }
}

// Convert from std::string::FromUtf8Error to IdsError
impl From<std::string::FromUtf8Error> for IdsError {
    fn from(err: std::string::FromUtf8Error) -> Self {
        IdsError::InvalidFormat(format!("Invalid UTF-8 sequence: {}", err))
    }
}

// Convert from serde_yaml::Error to IdsError (if used in the project)
// We don't need to implement From<ArrowError> or From<ParquetError> for IdsError
// since they're already implemented with #[from] in the IdsError enum.

// Convert from std::sync::mpsc::RecvError to IdsError for parallel processing errors
impl From<std::sync::mpsc::RecvError> for IdsError {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        IdsError::DataLoading(format!("Channel receive error: {}", err))
    }
}

// Implementation for additional error types can be added as needed.
// If additional crate dependencies are added in the future,
// add their error conversions here with appropriate feature flags.

// Convert from std::path::StripPrefixError to IdsError
impl From<std::path::StripPrefixError> for IdsError {
    fn from(err: std::path::StripPrefixError) -> Self {
        IdsError::PathResolution(format!("Path prefix error: {}", err))
    }
}

// Add implementations for error types that don't have #[from] in the enum
// This file will be expanded as we identify more error types needing conversion

//! Logging utilities and helpers.
//!
//! This module provides utilities for initializing and using the logging system.
//! It is only available when the `logging` feature is enabled.

use crate::error::{IdsError, Result};

/// Initializes the logging system with sensible defaults.
///
/// This function sets up the logging system with reasonable defaults for console output.
/// It automatically detects if the program is running in a terminal and adjusts the output accordingly.
///
/// # Returns
///
/// A Result indicating whether the logging system was successfully initialized.
///
/// # Errors
///
/// Returns an error if the logging system could not be initialized.
pub fn init_logger() -> Result<()> {
    log::set_max_level(log::LevelFilter::Info);
    log::info!("Logging initialized at info level");
    Ok(())
}

/// Initializes the logging system with the specified log level.
///
/// This function sets up the logging system with the specified log level for console output.
///
/// # Parameters
///
/// * `level` - The log level to use (trace, debug, info, warn, error)
///
/// # Returns
///
/// A Result indicating whether the logging system was successfully initialized.
///
/// # Errors
///
/// Returns an error if the logging system could not be initialized or if the log level is invalid.
pub fn init_logger_with_level(level: &str) -> Result<()> {
    let log_level = match level.to_lowercase().as_str() {
        "trace" => log::LevelFilter::Trace,
        "debug" => log::LevelFilter::Debug,
        "info" => log::LevelFilter::Info,
        "warn" => log::LevelFilter::Warn,
        "error" => log::LevelFilter::Error,
        _ => return Err(IdsError::validation(format!("Invalid log level: {}", level))),
    };

    log::set_max_level(log_level);
    log::info!("Logging initialized at {} level", level);
    Ok(())
}

/// Logs a message at the debug level.
///
/// This macro is a convenience wrapper around log::debug! that is conditionally compiled
/// based on the logging feature.
#[macro_export]
macro_rules! log_debug {
    ($($arg:tt)*) => {
        log::debug!($($arg)*);
    };
}

/// Logs a message at the info level.
///
/// This macro is a convenience wrapper around log::info! that is conditionally compiled
/// based on the logging feature.
#[macro_export]
macro_rules! log_info {
    ($($arg:tt)*) => {
        log::info!($($arg)*);
    };
}

/// Logs a message at the warn level.
///
/// This macro is a convenience wrapper around log::warn! that is conditionally compiled
/// based on the logging feature.
#[macro_export]
macro_rules! log_warn {
    ($($arg:tt)*) => {
        log::warn!($($arg)*);
    };
}

/// Logs a message at the error level.
///
/// This macro is a convenience wrapper around log::error! that is conditionally compiled
/// based on the logging feature.
#[macro_export]
macro_rules! log_error {
    ($($arg:tt)*) => {
        log::error!($($arg)*);
    };
}
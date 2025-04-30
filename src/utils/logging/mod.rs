//! Logging utilities for application logging setup.

use colored::Colorize;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use std::sync::Once;

static LOGGER_INIT: Once = Once::new();

/// Simple logger that outputs to console with colors
pub struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let level_str = match record.level() {
                Level::Error => "ERROR".red().bold(),
                Level::Warn => "WARN ".yellow().bold(),
                Level::Info => "INFO ".green(),
                Level::Debug => "DEBUG".blue(),
                Level::Trace => "TRACE".normal(),
            };

            println!(
                "[{}] {}: {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_str,
                record.args()
            );
        }
    }

    fn flush(&self) {}
}

/// Initialize logging with the specified level
pub fn init_logging(level: LevelFilter) -> Result<(), SetLoggerError> {
    let mut result = Ok(());

    LOGGER_INIT.call_once(|| {
        result = log::set_logger(&SimpleLogger).map(|()| log::set_max_level(level));
    });

    result
}

/// Initialize default logging (Info level)
pub fn init_default_logging() -> Result<(), SetLoggerError> {
    init_logging(LevelFilter::Info)
}

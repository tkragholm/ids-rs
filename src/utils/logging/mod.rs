//! Structured logging utilities for the IDS-RS application.
//!
//! This module provides a structured approach to logging with trace context
//! and component categorization.

pub mod macros;

use colored::Colorize;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};
use once_cell::sync::OnceCell;
use std::fmt;
use std::sync::RwLock;
use uuid::Uuid;

static LOGGER: OnceCell<StructuredLogger> = OnceCell::new();
static TRACE_ID: RwLock<Option<String>> = RwLock::new(None);

/// Logging component categories for better organization
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Component {
    /// Core application logic
    Core,
    /// Data loading and transformation
    Data,
    /// Algorithm components
    Algorithm,
    /// CLI interface
    Cli,
    /// I/O operations
    Io,
    /// Health-related operations
    Health,
    /// LPR registry operations
    Lpr,
    /// Population analysis
    Population,
    /// Utility functions
    Utility,
}

impl fmt::Display for Component {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Core => write!(f, "CORE"),
            Self::Data => write!(f, "DATA"),
            Self::Algorithm => write!(f, "ALGO"),
            Self::Cli => write!(f, "CLI "),
            Self::Io => write!(f, "IO  "),
            Self::Health => write!(f, "HLTH"),
            Self::Lpr => write!(f, "LPR "),
            Self::Population => write!(f, "POPL"),
            Self::Utility => write!(f, "UTIL"),
        }
    }
}

impl Default for Component {
    fn default() -> Self {
        Self::Core
    }
}

/// Represents structured log entry metadata
pub struct LogContext {
    /// Component that generated the log
    pub component: Component,
    /// Operation being performed
    pub operation: Option<String>,
    /// ID for tracing related log entries
    pub trace_id: Option<String>,
}

impl Default for LogContext {
    fn default() -> Self {
        Self {
            component: Component::default(),
            operation: None,
            trace_id: get_trace_id(),
        }
    }
}

/// Create a new trace ID and store it in the global context
pub fn start_trace() -> String {
    let new_id = Uuid::new_v4().to_string();
    // Only use the first segment of the UUID for brevity
    let short_id = new_id.split('-').next().unwrap_or("unknown").to_string();
    
    if let Ok(mut trace) = TRACE_ID.write() {
        *trace = Some(short_id.clone());
    }
    
    short_id
}

/// Clear the current trace ID
pub fn end_trace() {
    if let Ok(mut trace) = TRACE_ID.write() {
        *trace = None;
    }
}

/// Get the current trace ID if available
pub fn get_trace_id() -> Option<String> {
    if let Ok(trace) = TRACE_ID.read() {
        trace.clone()
    } else {
        None
    }
}

/// Structured logger that outputs with component and trace context
pub struct StructuredLogger {
    level: LevelFilter,
}

impl StructuredLogger {
    /// Create a new structured logger with the specified level
    pub fn new(level: LevelFilter) -> Self {
        Self { level }
    }
    
    /// Log a message with structured context
    pub fn log_with_context(&self, 
                           level: Level, 
                           context: &LogContext, 
                           message: &str) {
        if level > self.level {
            return;
        }
        
        let level_str = match level {
            Level::Error => "ERROR".red().bold(),
            Level::Warn => "WARN ".yellow().bold(),
            Level::Info => "INFO ".green(),
            Level::Debug => "DEBUG".blue(),
            Level::Trace => "TRACE".normal(),
        };
        
        let component_str = match context.component {
            Component::Core => "CORE".white(),
            Component::Data => "DATA".cyan(),
            Component::Algorithm => "ALGO".magenta(),
            Component::Cli => "CLI ".white(),
            Component::Io => "IO  ".cyan(),
            Component::Health => "HLTH".green(),
            Component::Lpr => "LPR ".yellow(),
            Component::Population => "POPL".blue(),
            Component::Utility => "UTIL".normal(),
        };
        
        let operation_str = if let Some(op) = &context.operation {
            format!("[{}]", op)
        } else {
            "".to_string()
        };
        
        let trace_str = if let Some(trace) = &context.trace_id {
            format!("[{}]", trace)
        } else {
            "".to_string()
        };
        
        println!(
            "[{}] {} {} {}{}: {}",
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
            level_str,
            component_str,
            trace_str,
            operation_str,
            message
        );
    }
}

impl log::Log for StructuredLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= self.level
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // Extract component and trace from target if possible
            // Format: component=LPR,trace=abc123,operation=load
            let mut context = LogContext::default();
            
            // Parse target for structured information
            let target = record.target();
            for part in target.split(',') {
                if let Some((key, value)) = part.split_once('=') {
                    match key.trim() {
                        "component" => {
                            context.component = match value.trim().to_uppercase().as_str() {
                                "CORE" => Component::Core,
                                "DATA" => Component::Data,
                                "ALGORITHM" | "ALGO" => Component::Algorithm,
                                "CLI" => Component::Cli,
                                "IO" => Component::Io,
                                "HEALTH" | "HLTH" => Component::Health,
                                "LPR" => Component::Lpr,
                                "POPULATION" | "POPL" => Component::Population,
                                "UTILITY" | "UTIL" => Component::Utility,
                                _ => Component::Core,
                            };
                        }
                        "operation" => {
                            context.operation = Some(value.trim().to_string());
                        }
                        "trace" => {
                            context.trace_id = Some(value.trim().to_string());
                        }
                        _ => {}
                    }
                }
            }
            
            // If no trace ID in the target but we have a global one, use it
            if context.trace_id.is_none() {
                context.trace_id = get_trace_id();
            }
            
            self.log_with_context(record.level(), &context, &record.args().to_string());
        }
    }

    fn flush(&self) {}
}

/// Initialize structured logging with the specified level
pub fn init_logging(level: LevelFilter) -> Result<(), SetLoggerError> {
    let logger = StructuredLogger::new(level);
    
    LOGGER.get_or_init(|| logger);
    
    log::set_max_level(level);
    log::set_boxed_logger(Box::new(LOGGER.get().unwrap().clone()))
}

/// Initialize default logging (Info level)
pub fn init_default_logging() -> Result<(), SetLoggerError> {
    init_logging(LevelFilter::Info)
}

// Enable cloning for the logger
impl Clone for StructuredLogger {
    fn clone(&self) -> Self {
        Self { level: self.level }
    }
}

/// Get the global logger instance
pub fn logger() -> &'static StructuredLogger {
    LOGGER.get().expect("Logger not initialized")
}

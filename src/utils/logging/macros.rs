//! Logging macros for structured logging.
//!
//! This module provides macros for easier structured logging.

/// Log at trace level with component context
#[macro_export]
macro_rules! trace_log {
    ($component:expr, $operation:expr, $($arg:tt)+) => {
        log::trace!(target: &format!("component={},operation={}", $component, $operation), $($arg)+)
    };
    ($component:expr, $($arg:tt)+) => {
        log::trace!(target: &format!("component={}", $component), $($arg)+)
    };
}

/// Log at debug level with component context
#[macro_export]
macro_rules! debug_log {
    ($component:expr, $operation:expr, $($arg:tt)+) => {
        log::debug!(target: &format!("component={},operation={}", $component, $operation), $($arg)+)
    };
    ($component:expr, $($arg:tt)+) => {
        log::debug!(target: &format!("component={}", $component), $($arg)+)
    };
}

/// Log at info level with component context
#[macro_export]
macro_rules! info_log {
    ($component:expr, $operation:expr, $($arg:tt)+) => {
        log::info!(target: &format!("component={},operation={}", $component, $operation), $($arg)+)
    };
    ($component:expr, $($arg:tt)+) => {
        log::info!(target: &format!("component={}", $component), $($arg)+)
    };
}

/// Log at warn level with component context
#[macro_export]
macro_rules! warn_log {
    ($component:expr, $operation:expr, $($arg:tt)+) => {
        log::warn!(target: &format!("component={},operation={}", $component, $operation), $($arg)+)
    };
    ($component:expr, $($arg:tt)+) => {
        log::warn!(target: &format!("component={}", $component), $($arg)+)
    };
}

/// Log at error level with component context
#[macro_export]
macro_rules! error_log {
    ($component:expr, $operation:expr, $($arg:tt)+) => {
        log::error!(target: &format!("component={},operation={}", $component, $operation), $($arg)+)
    };
    ($component:expr, $($arg:tt)+) => {
        log::error!(target: &format!("component={}", $component), $($arg)+)
    };
}

/// Start a traced operation block
///
/// This macro creates a new trace ID, logs the start of an operation,
/// executes the provided block, and logs the completion. It also captures
/// and logs any errors that occur.
#[macro_export]
macro_rules! traced_operation {
    ($component:expr, $operation:expr, $block:block) => {{
        use $crate::utils::logging::{start_trace, end_trace, Component};
        
        let _trace_id = start_trace(); // underscore prefix to prevent unused variable warning
        $crate::info_log!($component, $operation, "Operation started");
        
        let result = (|| -> $crate::error::Result<_> {
            $block
        })();
        
        match &result {
            Ok(_) => $crate::info_log!($component, $operation, "Operation completed successfully"),
            Err(e) => $crate::error_log!($component, $operation, "Operation failed: {}", e),
        }
        
        end_trace();
        result
    }};
}

/// Log the start of a function or method with its parameters
#[macro_export]
macro_rules! log_function_entry {
    ($component:expr, $function:expr, $($param:expr),*) => {{
        let mut params = Vec::new();
        $(
            params.push(format!("{:?}", $param));
        )*
        $crate::debug_log!($component, $function, "ENTER: ({})", params.join(", "));
    }};
    ($component:expr, $function:expr) => {
        $crate::debug_log!($component, $function, "ENTER");
    };
}

/// Log the exit of a function or method with its return value
#[macro_export]
macro_rules! log_function_exit {
    ($component:expr, $function:expr, $result:expr) => {
        $crate::debug_log!($component, $function, "EXIT: {:?}", $result);
    };
    ($component:expr, $function:expr) => {
        $crate::debug_log!($component, $function, "EXIT");
    };
}

/// Log a step within a function
#[macro_export]
macro_rules! log_step {
    ($component:expr, $operation:expr, $step:expr, $($arg:tt)+) => {
        $crate::debug_log!($component, $operation, "STEP {}: {}", $step, format!($($arg)+));
    };
}
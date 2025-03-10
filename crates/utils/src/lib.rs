pub mod error;
pub mod date;
pub mod string;
pub mod config;
pub mod file_patterns;
pub mod logging;
pub mod rich_console;

// Re-export common utilities to make them easier to use
pub use error::{Context, IdsError, Result, with_context};
pub use date::{DateUtils, quarter_from_date};
pub use string::StringUtils;
pub use logging::setup_logger;
pub use rich_console::RichConsole;
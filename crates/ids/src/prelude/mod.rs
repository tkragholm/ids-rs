// The prelude module re-exports commonly used types and functions
// to make it more convenient to import them

// Re-export error types
pub use crate::core::error::{ErrorContext, IdsError, IdsResult};

// Re-export CLI types
pub use crate::cli::types::{Cli, Commands, ConfigCommands};

// Re-export configuration types
pub use crate::commands::balance::config::BalanceCheckConfig;

// Re-export utility functions
pub use crate::utils::paths::normalize_path;

# Rust Crate Refactoring Plan

After analyzing the provided code, I've identified several areas for improvement. Here's my comprehensive refactoring plan to make the code more idiomatic, maintainable, and aligned with Rust best practices.

## Overall Structural Changes

1. **Module Organization**: Reorganize the code into a more logical module structure
2. **Error Handling**: Implement a custom error type with proper error propagation
3. **Configuration**: Create dedicated configuration modules for each command
4. **Separation of Concerns**: Better separate CLI parsing from business logic

## File-by-File Refactoring Plan

### `cli.rs`

The CLI module is well-structured but could benefit from:

1. **Add Derive Traits**: Add `Debug` and `Clone` to enums and structs
2. **Improve Documentation**: Enhance doc comments for better command-line help
3. **Use Consistent Naming**: Standardize argument naming conventions

```rust
// Keep the current structure but add:
#[derive(Parser, Debug, Clone)]
// Additional documentation

// Rename some arguments for consistency
// match_window instead of birth_window, etc.
```

### `lib.rs`

This file should be restructured to serve as a clear API entry point:

1. **Reorganize Modules**: Organize modules based on functionality
2. **Expose Public API**: Clearly define what's public vs internal
3. **Add Examples**: Include documentation examples for key functions

```rust
pub mod cli;
pub mod commands;
pub mod config;
pub mod error;
pub mod utils;

// Re-export key types for convenient access
pub use cli::{Cli, Commands, ConfigCommands};
pub use error::IdsError;

// Main entry function for the library
pub fn run() -> Result<(), IdsError> {
    // Call to refactored main_run implementation
}
```

### `main_run.rs` → Split into multiple files

This file is too large and should be split into separate modules by functionality:

1. **Create `commands` module**: Move command handlers to separate files
2. **Create `config` module**: Move configuration functionality
3. **Create `error` module**: Create a custom error type
4. **Create `utils` module**: Move helper functions

#### New files to create:

### `error.rs`

```rust
#[derive(Debug, thiserror::Error)]
pub enum IdsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Data loading error: {0}")]
    DataLoading(String),

    #[error("Balance calculation error: {0}")]
    BalanceCalculation(String),

    // More error variants
}

// Result type alias for convenience
pub type IdsResult<T> = Result<T, IdsError>;
```

### `commands/mod.rs`

```rust
mod balance;
mod config;
mod generate;
mod sample;

pub use balance::handle_balance_check;
pub use config::handle_config_command;
pub use generate::handle_generate_registers;
pub use sample::handle_sampling;
```

### `commands/balance.rs`

```rust
use crate::error::IdsResult;

pub struct BalanceCheckConfig<'a> {
    // Configuration fields
}

pub fn handle_balance_check(config: &BalanceCheckConfig) -> IdsResult<()> {
    // Implementation moved from main_run.rs
}
```

### `commands/sample.rs`

```rust
use crate::error::IdsResult;

pub fn handle_sampling(
    input: &str,
    controls: usize,
    birth_window: i64,
    parent_window: i64,
    output_dir: &str,
) -> IdsResult<()> {
    // Implementation moved from main_run.rs
}
```

### `commands/generate.rs`

```rust
use crate::error::IdsResult;

pub fn handle_generate_registers(
    output_dir: &str,
    num_records: usize,
    num_cases: usize,
    start_year: i32,
    end_year: i32,
    seed: Option<u64>,
) -> IdsResult<()> {
    // Implementation moved from main_run.rs
}
```

### `commands/config.rs`

```rust
use crate::cli::ConfigCommands;
use crate::error::IdsResult;

pub fn handle_config_command(cmd: &ConfigCommands) -> IdsResult<()> {
    // Implementation moved from main_run.rs
}
```

### `utils/mod.rs`

```rust
mod paths;
mod setup;
mod reports;

pub use paths::resolve_path;
pub use setup::{setup_directories, configure_logging_with_dir};
pub use reports::generate_structured_reports;
```

### `utils/setup.rs`

```rust
use crate::error::IdsResult;

/// Create necessary output directories for application
pub fn setup_directories(output_dir: &str) -> IdsResult<()> {
    // Implementation moved from main_run.rs
}

/// Configure logging with custom directory
pub fn configure_logging_with_dir(output_dir: &str) -> IdsResult<()> {
    // Implementation moved from main_run.rs
}
```

### `utils/reports.rs`

```rust
use crate::error::IdsResult;

/// Generate structured reports from balance results and matched pairs data
pub fn generate_structured_reports(
    balance_results: &covariates::balance::results::BalanceResults,
    matched_pairs: &[covariates::matched_pairs::record::MatchedPairRecord],
    output_dir: &str,
) -> IdsResult<()> {
    // Implementation moved from main_run.rs
}
```

### `utils/paths.rs`

```rust
use std::path::Path;

/// Resolve a path - absolute or relative to base
pub fn resolve_path(base_path: &str, path: &str) -> String {
    // Implementation moved from main_run.rs
}

/// Helper functions for path resolution and validation
pub fn check_path_exists(path: &str, path_type: &str) -> bool {
    // Extracted from main_run.rs
}

pub fn normalize_path(path: &str, register_type: &str, base_dir: Option<&str>) -> String {
    // Extracted and improved from main_run.rs
}
```

### `main.rs`

The main file becomes much simpler:

```rust
use ids::run;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    run()
}
```

## Specific Improvements

1. **Better Error Handling**:
   - Use the `thiserror` crate for defining error types
   - Create custom error types with meaningful messages
   - Provide context for errors using `.with_context()` from `anyhow`

2. **Configuration Management**:
   - Use strongly typed configuration structs instead of passing individual parameters
   - Implement validation for configuration values

3. **CLI Command Processing**:
   - Create a command dispatcher to reduce boilerplate in the run function
   - Move command implementations to separate modules

4. **Code Duplication**:
   - Extract common functionality into utility functions
   - Standardize console output and logging patterns

5. **Path Handling**:
   - Create helper functions for path resolution and validation
   - Use Path and PathBuf more consistently

6. **Testing**:
   - Add unit tests for critical functionality
   - Make functions more testable by separating I/O from pure logic

This refactoring approach will significantly improve the maintainability, readability, and adherence to Rust idioms in the codebase.

pub mod config;
pub mod data_loading;
pub mod handler;
pub mod metrics;
pub mod reporting;

// Re-export the main handler function for convenience
pub use handler::handle_balance_check;

// Re-export the configuration struct for convenience
pub use config::BalanceCheckConfig;

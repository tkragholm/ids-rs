// ids-rs/crates/ids-covariates/src/lib.rs
pub mod balance;
pub mod error;
pub mod loader;
pub mod matched_pairs;
pub mod models;
pub mod storage;

// Re-export commonly used types
pub use balance::BalanceChecker;
pub use error::CovariateError;
pub use loader::CovariateLoader;
pub use models::{CovariateSummary, Education, Income, Occupation};
pub use storage::CovariateStore;

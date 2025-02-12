// ids-rs/crates/ids-covariates/src/lib.rs
pub mod balance;
pub mod error;
pub mod loader;
pub mod matched_pairs;
pub mod models;
pub mod storage;

pub use balance::BalanceChecker;
pub use error::CovariateError;
pub use loader::CovariateLoader;
pub use matched_pairs::{is_case, load_matched_pairs};
pub use models::{CovariateSummary, Education, Income, Occupation};
pub use storage::CovariateStore;

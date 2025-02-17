pub mod balance;
pub mod matched_pairs;
pub mod models;
pub mod reporting;
pub mod storage;

// Re-export types that we want to expose
pub use types::{
    error::IdsError as CovariateError,
    models::{Education, Income, Occupation},
    CovariateSnapshot,
};

pub use balance::BalanceChecker;
pub use matched_pairs::{is_case, load_matched_pairs};
pub use models::CovariateSummary;
pub use storage::CovariateStore;

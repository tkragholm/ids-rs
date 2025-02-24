pub mod balance;
pub mod matched_pairs;
pub mod models;
pub mod reporting;

pub use balance::BalanceChecker;
pub use matched_pairs::loader::load_matched_pairs;
pub use models::CovariateSummary;
pub use reporting::{BalanceReport, ComprehensiveReport, CsvReport};

// Re-export types that we want to expose
pub use types::{
    error::IdsError as CovariateError,
    models::{Covariate, CovariateType, CovariateValue},
    store::Store,
};

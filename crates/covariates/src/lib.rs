pub mod balance;
pub mod config;
pub mod matched_pairs;
pub mod models;
pub mod processors;
pub mod reporting;
pub mod storage;

pub use balance::BalanceChecker;
pub use config::{CovariatesConfig, CovariateTypeConfig, CovariateVariableConfig, generate_default_config};
pub use matched_pairs::loader::load_matched_pairs;
pub use models::CovariateSummary;
pub use processors::{
    ConfigurableProcessor,
    ConfigurableVariableProcessor,
    CovariateProcessorRegistry,
    DemographicsProcessor,
    EducationProcessor,
    IncomeProcessor,
    OccupationProcessor,
    ProcessorFactory,
};
pub use reporting::{BalanceReport, ComprehensiveReport, CsvReport};

// Re-export types that we want to expose
pub use types::{
    error::IdsError as CovariateError,
    models::{Covariate, CovariateType, CovariateValue},
    storage::Storage as Store,
    traits::{CovariateProcessor, VariableType},
};

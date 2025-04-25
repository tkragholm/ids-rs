// Core modules
pub mod balance;
pub mod core;
pub mod data;
pub mod models;
pub mod prelude;
pub mod processing;
pub mod reporting;

// Re-exports for backward compatibility
pub use core::config::{
    generate_default_config, CovariateTypeConfig, CovariateVariableConfig, CovariatesConfig,
};
pub use core::Error as CovariateError;

// Balance checking functionality
pub use balance::{
    memory_manager, BalanceChecker, BalanceCheckerBuilder, BalanceResults, MemoryGuard, MemoryTier,
    OptimizationStrategy,
};

// Data access
pub use data::matched_pairs::loader::load_matched_pairs;
pub use models::CovariateSummary;

// Processors
pub use core::registry::CovariateProcessorRegistry;
pub use processing::{
    ConfigurableProcessor, DemographicsProcessor, EducationProcessor, IncomeProcessor,
    OccupationProcessor, ProcessorFactory,
};

// Reporting
pub use reporting::{BalanceReport, ComprehensiveReport, CsvReport};

// Re-export types we depend on for convenience
pub use types::models::{Covariate, CovariateType, CovariateValue};
pub use types::traits::{CovariateProcessor, Store, VariableType};

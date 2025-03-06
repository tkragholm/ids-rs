// Core modules
pub mod core;
pub mod data;
pub mod processing;
pub mod balance;
pub mod reporting;
pub mod models;
pub mod prelude;

// Re-exports for backward compatibility
pub use core::config::{CovariatesConfig, CovariateTypeConfig, CovariateVariableConfig, generate_default_config};
pub use core::Error as CovariateError;

// Balance checking functionality
pub use balance::{
    BalanceChecker, 
    BalanceCheckerBuilder,
    BalanceResults, 
    OptimizationStrategy,
    memory_manager, 
    MemoryGuard, 
    MemoryTier,
};

// Data access
pub use data::matched_pairs::loader::load_matched_pairs;
pub use models::CovariateSummary;

// Processors
pub use processing::{
    ConfigurableProcessor, 
    ProcessorFactory,
    DemographicsProcessor,
    EducationProcessor,
    IncomeProcessor,
    OccupationProcessor,
};
pub use core::registry::CovariateProcessorRegistry;

// Reporting
pub use reporting::{BalanceReport, ComprehensiveReport, CsvReport};

// Re-export types we depend on for convenience
pub use types::models::{Covariate, CovariateType, CovariateValue};
pub use types::traits::{CovariateProcessor, Store, VariableType};

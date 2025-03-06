// Re-export the most commonly used types
pub use crate::core::{
    Error, Result,
};

pub use crate::core::config::{
    CovariatesConfig, 
    CovariateTypeConfig, 
    CovariateVariableConfig
};

pub use crate::balance::{
    BalanceChecker,
    BalanceCheckerBuilder,
    BalanceResults,
    OptimizationStrategy,
    memory_manager,
    MemoryGuard,
    MemoryTier,
};

pub use crate::processing::{
    ConfigurableProcessor,
    DemographicsProcessor,
    EducationProcessor,
    IncomeProcessor,
    OccupationProcessor,
    ProcessorFactory,
};

pub use crate::data::matched_pairs::{
    MatchedPairRecord,
    CaseWithControls,
    Control,
    load_matched_pairs,
    load_matched_pair_records,
};

pub use crate::reporting::{
    BalanceReport,
    ComprehensiveReport,
    CsvReport,
};

// Re-export types from the types crate that we commonly use
pub use types::models::{
    Covariate,
    CovariateType,
    CovariateValue,
};

pub use types::traits::{
    CovariateProcessor,
    Store,
    VariableType,
};
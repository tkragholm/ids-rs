// Re-export the most commonly used types
pub use crate::core::{Error, Result};

pub use crate::core::config::{CovariateTypeConfig, CovariateVariableConfig, CovariatesConfig};

pub use crate::balance::{
    memory_manager, BalanceChecker, BalanceCheckerBuilder, BalanceResults, MemoryGuard, MemoryTier,
    OptimizationStrategy,
};

pub use crate::processing::{
    ConfigurableProcessor, DemographicsProcessor, EducationProcessor, IncomeProcessor,
    OccupationProcessor, ProcessorFactory,
};

pub use crate::data::matched_pairs::{
    load_matched_pair_records, load_matched_pairs, CaseWithControls, Control, MatchedPairRecord,
};

pub use crate::reporting::{BalanceReport, ComprehensiveReport, CsvReport};

// Re-export types from the types crate that we commonly use
pub use types::models::{Covariate, CovariateType, CovariateValue};

pub use types::traits::{CovariateProcessor, Store, VariableType};

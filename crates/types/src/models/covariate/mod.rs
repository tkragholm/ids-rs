// Modules
mod types;
mod values;
pub mod builders;

// Re-exports
pub use self::types::{Covariate, CovariateType, CovariateValue, DemographicExtras};

// Re-export builders for convenience
pub use self::builders::{
    EducationBuilder, 
    IncomeBuilder,
    OccupationBuilder,
    DemographicsBuilder,
};
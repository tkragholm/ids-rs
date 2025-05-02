// Modules
pub mod builders;
mod types;
mod values;

// Re-exports
pub use self::types::{Covariate, CovariateType, CovariateValue, DemographicExtras};

// Re-export builders for convenience
pub use self::builders::{DemographicsBuilder, EducationBuilder, IncomeBuilder, OccupationBuilder};

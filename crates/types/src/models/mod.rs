//! Domain models for epidemiological research data.
//!
//! This module contains the core data structures used to represent
//! various types of data used in epidemiological research, including:
//!
//! - **Covariates**: Variables that may influence outcomes (education, income, etc.)
//! - **Family Relations**: Relationships between individuals
//! - **PNR Data**: Personal identification number handling
//! - **Time-Varying Values**: Values that change over time
//!
//! ## Examples
//!
//! Creating covariates using builders:
//!
//! ```
//! use types::models::{
//!     CovariateType,
//!     EducationBuilder,
//!     DemographicsBuilder,
//! };
//!
//! // Create an education covariate
//! let education = EducationBuilder::new("higher")
//!     .with_years(16.0)
//!     .build();
//!
//! // Create a demographics covariate
//! let demographics = DemographicsBuilder::new(2, 101, "nuclear")
//!     .with_age(42)
//!     .with_gender("M")
//!     .build();
//! ```
//!
//! Working with time-varying values:
//!
//! ```
//! use types::models::{TimeVaryingValue, Covariate};
//! use chrono::NaiveDate;
//!
//! fn process_time_varying(value: TimeVaryingValue<Covariate>) {
//!     println!("PNR: {}", value.pnr);
//!     println!("Start Date: {:?}", value.start_date);
//!     println!("End Date: {:?}", value.end_date);
//!     println!("Value: {:?}", value.value);
//! }
//! ```

// Submodules
pub mod covariate;
pub mod family;
pub mod pnr;
pub mod time_varying;

// Re-exports

/// Covariate models
pub use covariate::{Covariate, CovariateType, CovariateValue, DemographicExtras};

/// Builder types for creating covariates
pub use covariate::builders::{
    DemographicsBuilder, EducationBuilder, IncomeBuilder, OccupationBuilder,
};

/// Time-varying value models
pub use time_varying::TimeVaryingValue;

/// Family relation models
pub use family::FamilyRelations;

/// PNR (personal identification number) models - defined here since they're moved into the models directory
pub struct Pnr(pub String);
pub type PnrPool = hashbrown::HashMap<String, usize>;
pub struct PersonInfo;
pub struct ParentPair;
pub struct FamilyInfo;

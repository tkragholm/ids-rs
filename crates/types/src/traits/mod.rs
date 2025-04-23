//! Traits for the ids-rs codebase.
//!
//! This module contains trait definitions that provide standardized interfaces
//! for various components of the system, including:
//!
//! - **Store**: Core trait for data storage and retrieval
//! - **FamilyAccess**: Trait for accessing family relations
//! - **CovariateProcessor**: Trait for processing covariates
//! - **DateHelpers**: Trait for date handling utilities
//! - **Cacheable**: Trait for cacheable objects
//! - **TimeVaryingAccess**: Trait for accessing time-varying data
//!
//! These traits allow for loose coupling between components and enable
//! alternative implementations of key functionality.

// Submodules
pub mod access;
pub mod cacheable;
pub mod processing;
pub mod utils;

// Imports
use crate::{
    error::Result,
    family::relations::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
};
use chrono::NaiveDate;

// Re-exports
pub use self::cacheable::Cacheable;
pub use self::processing::{CovariateProcessor, VariableType};
pub use self::utils::DateHelpers;
pub use crate::storage::arrow::access::ArrowAccess;

/// Store trait for data storage and retrieval.
///
/// This trait defines the core interface for all data storage backends.
/// It provides methods for accessing covariates, family relations, and
/// loading data into the store.
///
/// # Examples
///
/// ```
/// use types::prelude::*;
/// use chrono::NaiveDate;
///
/// fn process_person_data<S: Store>(
///     store: &mut S,
///     pnr: &str,
///     date: NaiveDate
/// ) -> Result<()> {
///     // Get education covariate
///     if let Some(education) = store.covariate(pnr, CovariateType::Education, date)? {
///         println!("Education: {:?}", education);
///     }
///
///     // Get all covariates (example only, this method would also need &mut self)
///     let all_covariates = vec![]; // Placeholder for store.covariates(pnr, date)?
///     println!("Found {} covariates", all_covariates.len());
///
///     Ok(())
/// }
/// ```
pub trait Store: Send + Sync {
    /// Returns a specific covariate for a person at a given date.
    ///
    /// # Parameters
    ///
    /// * `pnr` - The personal identification number (PNR)
    /// * `covariate_type` - The type of covariate to retrieve
    /// * `date` - The date at which to retrieve the covariate
    ///
    /// # Returns
    ///
    /// A Result containing an Option of the covariate, or an error if retrieval failed.
    /// The Option will be None if no covariate of the specified type exists for the
    /// person at the given date.
    fn covariate(
        &mut self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>>;

    /// Returns family relations for a person.
    ///
    /// # Parameters
    ///
    /// * `pnr` - The personal identification number (PNR)
    ///
    /// # Returns
    ///
    /// An Option containing a reference to the family relations if they exist,
    /// or None if no family relations exist for the person.
    fn family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;

    /// Loads data into the store.
    ///
    /// # Parameters
    ///
    /// * `data` - A vector of time-varying covariates to load
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure of the operation.
    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<()>;

    /// Returns all covariates for a person at a given date.
    ///
    /// # Parameters
    ///
    /// * `pnr` - The personal identification number (PNR)
    /// * `date` - The date at which to retrieve the covariates
    ///
    /// # Returns
    ///
    /// A Result containing a HashMap of covariates indexed by type,
    /// or an error if retrieval failed.
    fn covariates(
        &mut self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<hashbrown::HashMap<CovariateType, Covariate>> {
        let mut covariates = hashbrown::HashMap::new();

        for covariate_type in &[
            CovariateType::Demographics,
            CovariateType::Education,
            CovariateType::Income,
            CovariateType::Occupation,
        ] {
            if let Some(covariate) = self.covariate(pnr, *covariate_type, date)? {
                covariates.insert(*covariate_type, covariate);
            }
        }

        Ok(covariates)
    }

    /// Returns covariates for a person's family at a given date.
    ///
    /// # Parameters
    ///
    /// * `pnr` - The personal identification number (PNR)
    /// * `date` - The date at which to retrieve the covariates
    ///
    /// # Returns
    ///
    /// A Result containing an Option of a HashMap of covariates indexed by type,
    /// or an error if retrieval failed. The Option will be None if the person has
    /// no family relations or if no covariates exist for the family.
    fn family_covariates(
        &mut self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<hashbrown::HashMap<CovariateType, Covariate>>> {
        let family = self.family_relations(pnr);

        if let Some(_family) = family {
            let covariates = self.covariates(pnr, date)?;
            if !covariates.is_empty() {
                return Ok(Some(covariates));
            }
        }

        Ok(None)
    }

    /// Converts to Any for dynamic casting.
    ///
    /// This method is primarily used for internal type conversions
    /// and should not be used directly in most cases.
    fn as_any(&self) -> &dyn std::any::Any;

    /// Converts to Any for dynamic casting (mutable).
    ///
    /// This method is primarily used for internal type conversions
    /// and should not be used directly in most cases.
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Trait for accessing family relations.
///
/// This trait provides methods for accessing family relations,
/// including parents and birth dates.
///
/// # Examples
///
/// ```
/// use types::prelude::*;
/// use chrono::NaiveDate;
///
/// fn print_family_info<F: FamilyAccess>(family_access: &F, pnr: &str) {
///     // Get parents
///     if let Some((father, mother)) = family_access.parents(pnr) {
///         println!("Father: {:?}, Mother: {:?}", father, mother);
///     }
///
///     // Get birth date
///     if let Some(birth_date) = family_access.birth_date(pnr) {
///         println!("Birth date: {}", birth_date);
///     }
/// }
/// ```
pub trait FamilyAccess {
    /// Returns family relations for a person.
    ///
    /// # Parameters
    ///
    /// * `pnr` - The personal identification number (PNR)
    ///
    /// # Returns
    ///
    /// An Option containing a reference to the family relations if they exist,
    /// or None if no family relations exist for the person.
    fn family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;

    /// Returns the parents' PNRs for a person.
    ///
    /// # Parameters
    ///
    /// * `pnr` - The personal identification number (PNR)
    ///
    /// # Returns
    ///
    /// An Option containing a tuple of Options for father and mother PNRs,
    /// or None if no family relations exist for the person.
    fn parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)>;

    /// Returns the birth date for a person.
    ///
    /// # Parameters
    ///
    /// * `pnr` - The personal identification number (PNR)
    ///
    /// # Returns
    ///
    /// An Option containing the birth date if it exists,
    /// or None if no birth date exists for the person.
    fn birth_date(&self, pnr: &str) -> Option<NaiveDate>;
}

/// Trait for accessing time-varying data.
///
/// This trait provides methods for accessing data that varies over time,
/// such as covariates that change at different dates.
///
/// # Type Parameters
///
/// * `T` - The type of data being accessed
///
/// # Examples
///
/// ```
/// use types::prelude::*;
/// use chrono::NaiveDate;
///
/// fn print_data_at_date<A: TimeVaryingAccess<Covariate>>(
///     access: &A,
///     pnr: &str,
///     date: NaiveDate
/// ) {
///     if let Some(covariates) = access.at_date(pnr, date) {
///         println!("Found {} covariates at {}", covariates.len(), date);
///     }
/// }
/// ```
pub trait TimeVaryingAccess<T> {
    /// Returns data for a person at a given date.
    ///
    /// # Parameters
    ///
    /// * `pnr` - The personal identification number (PNR)
    /// * `date` - The date at which to retrieve the data
    ///
    /// # Returns
    ///
    /// An Option containing a vector of data if it exists,
    /// or None if no data exists for the person at the given date.
    fn at_date(&self, pnr: &str, date: NaiveDate) -> Option<Vec<T>>;

    /// Loads data into the store.
    ///
    /// # Parameters
    ///
    /// * `data` - A vector of time-varying data to load
    ///
    /// # Returns
    ///
    /// A Result indicating success or failure of the operation.
    fn load_data(&self, data: Vec<TimeVaryingValue<T>>) -> Result<()>;
}

// Implement FamilyAccess for any type that implements Store
impl<T: Store> FamilyAccess for T {
    fn family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        Store::family_relations(self, pnr)
    }

    fn parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)> {
        self.family_relations(pnr)
            .map(|rel| (rel.father_id.clone(), rel.mother_id.clone()))
    }

    fn birth_date(&self, pnr: &str) -> Option<NaiveDate> {
        self.family_relations(pnr).map(|rel| rel.birth_date)
    }
}

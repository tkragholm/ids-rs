use crate::{
    error::IdsError,
    family::relations::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
};
use chrono::{Datelike, NaiveDate};

// Add Store trait definition here directly
/// Combined Store trait - both data and family access
pub trait Store: Send + Sync {
    /// Get a covariate for a person at a specific date
    fn get_covariate(
        &self, 
        pnr: &str, 
        covariate_type: CovariateType, 
        date: NaiveDate
    ) -> Result<Option<Covariate>, IdsError>;

    /// Get family relations for a person
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;

    /// Load data into the store
    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError>;

    /// Get all covariates for a person at a specific date
    fn get_covariates(
        &self, 
        pnr: &str, 
        date: NaiveDate
    ) -> Result<hashbrown::HashMap<CovariateType, Covariate>, IdsError> {
        let mut covariates = hashbrown::HashMap::new();
        
        for covariate_type in &[
            CovariateType::Demographics,
            CovariateType::Education,
            CovariateType::Income,
            CovariateType::Occupation,
        ] {
            if let Some(covariate) = self.get_covariate(pnr, *covariate_type, date)? {
                covariates.insert(*covariate_type, covariate);
            }
        }
        
        Ok(covariates)
    }

    /// Get family covariates for a person at a specific date
    fn get_family_covariates(
        &self, 
        pnr: &str, 
        date: NaiveDate
    ) -> Result<Option<hashbrown::HashMap<CovariateType, Covariate>>, IdsError> {
        let family = self.get_family_relations(pnr);
        
        if let Some(family) = family {
            let covariates = self.get_covariates(pnr, date)?;
            if !covariates.is_empty() {
                return Ok(Some(covariates));
            }
        }
        
        Ok(None)
    }
    
    /// Convert to Any for dynamic casting
    fn as_any(&self) -> &dyn std::any::Any;
    
    /// Convert to Any for dynamic casting (mutable)
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

/// Trait for accessing family relations
pub trait FamilyAccess {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;
    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)>;
    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate>;
}

/// Trait for accessing time-varying data
pub trait TimeVaryingAccess<T> {
    fn get_at_date(&self, pnr: &str, date: NaiveDate) -> Option<Vec<T>>;
    fn load_data(&self, data: Vec<TimeVaryingValue<T>>) -> Result<(), IdsError>;
}

/// Helper for date operations
pub trait DateHelpers: Datelike {
    fn get_quarter(&self) -> u32 {
        ((self.month() - 1) / 3) + 1
    }
}

impl DateHelpers for NaiveDate {}

// Implement FamilyAccess for any type that implements Store
impl<T: Store> FamilyAccess for T {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        Store::get_family_relations(self, pnr)
    }

    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)> {
        self.get_family_relations(pnr)
            .map(|rel| (rel.father_id.clone(), rel.mother_id.clone()))
    }

    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate> {
        self.get_family_relations(pnr)
            .map(|rel| rel.birth_date)
    }
}

/// Variable type for covariate processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum VariableType {
    /// Numeric variable (f64)
    Numeric,
    /// Categorical variable (String)
    Categorical,
    /// Binary variable (0 or 1)
    Binary,
}

/// Trait that standardizes the processing of different covariate types
pub trait CovariateProcessor: Send + Sync {
    /// Get the name of this processor
    fn get_name(&self) -> &str;
    
    /// Get the covariate type this processor handles
    fn get_covariate_type(&self) -> CovariateType;
    
    /// Extract a numeric value from a covariate, returning None if not applicable
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64>;
    
    /// Extract a categorical value from a covariate, returning None if not applicable
    fn process_categorical(&self, covariate: &Covariate) -> Option<String>;
    
    /// Determine if this variable should be treated as categorical
    fn is_categorical(&self) -> bool;
    
    /// Get the variable type for this processor
    fn get_variable_type(&self) -> VariableType {
        if self.is_categorical() {
            VariableType::Categorical
        } else {
            VariableType::Numeric
        }
    }
    
    /// Convert a categorical value to a numeric representation if needed for calculations
    fn categorical_to_numeric(&self, value: &str) -> f64 {
        if let Ok(num) = value.parse::<f64>() { 
            num 
        } else {
            // Hash the string to create a stable numeric value
            let mut hash = 0.0;
            for (i, b) in value.bytes().enumerate() {
                hash += f64::from(b) * (i + 1) as f64;
            }
            hash
        }
    }
}
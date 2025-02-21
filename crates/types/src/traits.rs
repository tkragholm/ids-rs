use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
};
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;

// Core traits
pub trait DataAccess {
    fn get_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<HashMap<CovariateType, Covariate>, IdsError>;

    fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<HashMap<CovariateType, Covariate>>, IdsError>;
}

pub trait FamilyAccess {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;
    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)>;
    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate>;
}

// Data storage traits
pub trait Store: DataAccess + FamilyAccess {
    fn load_covariate(&self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError>;
}

pub trait TimeVaryingAccess<T> {
    fn get_at_date(&self, pnr: &str, date: NaiveDate) -> Option<Vec<T>>;
    fn load_data(&self, data: Vec<TimeVaryingValue<T>>) -> Result<(), IdsError>;
}

pub trait DateHelpers: Datelike {
    fn get_quarter(&self) -> u32 {
        ((self.month() - 1) / 3) + 1
    }
}

impl DateHelpers for NaiveDate {}

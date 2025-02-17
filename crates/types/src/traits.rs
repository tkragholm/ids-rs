use crate::{error::IdsError, family::FamilyRelations, models::*, snapshot::CovariateSnapshot};
use chrono::{Datelike, NaiveDate};

// Core traits
pub trait DataAccess {
    fn get_covariates_at_date(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<CovariateSnapshot, IdsError>;
    fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<CovariateSnapshot>, IdsError>;
}

pub trait FamilyAccess {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;
    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)>;
    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate>;
}

// Data storage traits
pub trait Store: DataAccess + FamilyAccess {
    fn load_education(&self, data: Vec<TimeVaryingValue<Education>>) -> Result<(), IdsError>;
    fn load_income(&self, data: Vec<TimeVaryingValue<Income>>) -> Result<(), IdsError>;
    fn load_occupation(&self, data: Vec<TimeVaryingValue<Occupation>>) -> Result<(), IdsError>;
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

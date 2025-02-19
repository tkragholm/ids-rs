use super::{BaseStore, Store};
use crate::{
    arrow_utils::ArrowStore,
    error::IdsError,
    family::FamilyRelations,
    models::{Education, Income, Occupation, TimeVaryingValue},
    snapshot::CovariateSnapshot,
    traits::{DataAccess, FamilyAccess},
};
use chrono::NaiveDate;

pub struct CombinedStore {
    base_store: BaseStore,
    arrow_store: ArrowStore,
}

impl CombinedStore {
    #[must_use] pub const fn new(base_store: BaseStore, arrow_store: ArrowStore) -> Self {
        Self {
            base_store,
            arrow_store,
        }
    }
}

impl Store for CombinedStore {
    fn load_education(&self, data: Vec<TimeVaryingValue<Education>>) -> Result<(), IdsError> {
        self.base_store.load_education(data)
    }

    fn load_income(&self, data: Vec<TimeVaryingValue<Income>>) -> Result<(), IdsError> {
        self.base_store.load_income(data)
    }

    fn load_occupation(&self, data: Vec<TimeVaryingValue<Occupation>>) -> Result<(), IdsError> {
        self.base_store.load_occupation(data)
    }
}

impl DataAccess for CombinedStore {
    fn get_covariates_at_date(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<CovariateSnapshot, IdsError> {
        self.base_store
            .get_covariates_at_date(pnr, date)
            .or_else(|_| self.arrow_store.get_covariates_at_date(pnr, date))
    }

    fn get_family_covariates(
        &self,
        pnr: &str,
        date: NaiveDate,
    ) -> Result<Option<CovariateSnapshot>, IdsError> {
        self.base_store
            .get_family_covariates(pnr, date)
            .or_else(|_| self.arrow_store.get_family_covariates(pnr, date))
    }
}

impl FamilyAccess for CombinedStore {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.base_store
            .get_family_relations(pnr)
            .or_else(|| self.arrow_store.get_family_relations(pnr))
    }

    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)> {
        self.base_store
            .get_parents(pnr)
            .or_else(|| self.arrow_store.get_parents(pnr))
    }

    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate> {
        self.base_store
            .get_birth_date(pnr)
            .or_else(|| self.arrow_store.get_birth_date(pnr))
    }
}

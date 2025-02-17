use chrono::NaiveDate;
use types::{BaseStore, CovariateSnapshot, Store};

pub struct CovariateStore {
    store: Box<dyn Store>,
}

impl CovariateStore {
    pub fn new() -> Self {
        Self {
            store: Box::new(BaseStore::new()),
        }
    }

    pub fn with_store(store: Box<dyn Store>) -> Self {
        Self { store }
    }

    pub fn get_covariates_at_date(&self, pnr: &str, date: NaiveDate) -> Option<CovariateSnapshot> {
        self.store.get_covariates_at_date(pnr, date).ok()
    }
}

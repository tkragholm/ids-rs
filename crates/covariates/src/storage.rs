use chrono::NaiveDate;
use types::{BaseStore, CovariateSnapshot, Store};

pub struct CovariateStore {
    store: Box<dyn Store>,
}

impl Default for CovariateStore {
    fn default() -> Self {
        Self::new()
    }
}

impl CovariateStore {
    #[must_use] pub fn new() -> Self {
        Self {
            store: Box::new(BaseStore::new()),
        }
    }

    #[must_use] pub fn with_store(store: Box<dyn Store>) -> Self {
        Self { store }
    }

    #[must_use] pub fn get_covariates_at_date(&self, pnr: &str, date: NaiveDate) -> Option<CovariateSnapshot> {
        self.store.get_covariates_at_date(pnr, date).ok()
    }
}

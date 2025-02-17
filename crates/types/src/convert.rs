use crate::CovariateSnapshot;
use chrono::NaiveDate;

pub trait IntoSnapshot {
    fn into_snapshot(self, date: NaiveDate) -> CovariateSnapshot;
}

// You can add default implementations or helper functions here

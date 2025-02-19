use crate::{error::IdsError, models::TimeVaryingValue, traits::TimeVaryingAccess};
use chrono::NaiveDate;
use dashmap::DashMap;
use std::collections::HashMap;

pub struct TimeVaryingStore<T> {
    data: DashMap<String, Vec<TimeVaryingValue<T>>>,
}

impl<T: Clone> Default for TimeVaryingStore<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone> TimeVaryingStore<T> {
    #[must_use] pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }

    fn load_internal(&self, values: Vec<TimeVaryingValue<T>>) -> Result<(), IdsError> {
        let grouped: HashMap<String, Vec<TimeVaryingValue<T>>> =
            values.into_iter().fold(HashMap::new(), |mut acc, value| {
                acc.entry(value.pnr.clone()).or_default().push(value);
                acc
            });

        for (pnr, mut values) in grouped {
            values.sort_by_key(|v| v.date);
            self.data.insert(pnr, values);
        }
        Ok(())
    }
}

impl<T: Clone> TimeVaryingAccess<T> for TimeVaryingStore<T> {
    fn get_at_date(&self, pnr: &str, date: NaiveDate) -> Option<Vec<T>> {
        self.data.get(pnr).map(|values| {
            values
                .iter()
                .filter(|v| v.date <= date)
                .map(|v| v.value.clone())
                .collect()
        })
    }

    fn load_data(&self, data: Vec<TimeVaryingValue<T>>) -> Result<(), IdsError> {
        self.load_internal(data)
    }
}

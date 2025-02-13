use crate::converter::IntoSnapshot;
use crate::error::CovariateError;
use crate::models::*;
use chrono::NaiveDate;
use dashmap::DashMap;
use std::collections::HashMap;

pub struct CovariateStore {
    education: DashMap<String, Vec<TimeVaryingValue<Education>>>,
    income: DashMap<String, Vec<TimeVaryingValue<Income>>>,
    occupation: DashMap<String, Vec<TimeVaryingValue<Occupation>>>,
}

impl CovariateStore {
    pub fn new() -> Self {
        Self {
            education: DashMap::new(),
            income: DashMap::new(),
            occupation: DashMap::new(),
        }
    }

    pub fn get_covariates_at_date(&self, pnr: &str, date: NaiveDate) -> Option<PersonCovariates> {
        let education = self.education.get(pnr).map(|values| {
            values
                .iter()
                .filter(|v| v.at_or_before_date(date))
                .cloned()
                .collect()
        });

        let income = self.income.get(pnr).map(|values| {
            values
                .iter()
                .filter(|v| v.at_or_before_date(date))
                .cloned()
                .collect()
        });

        let occupation = self.occupation.get(pnr).map(|values| {
            values
                .iter()
                .filter(|v| v.at_or_before_date(date))
                .cloned()
                .collect()
        });

        Some(PersonCovariates {
            pnr: pnr.to_string(),
            education: education?,
            income: income?,
            occupation: occupation?,
        })
    }

    pub fn load_education(
        &self,
        data: Vec<TimeVaryingValue<Education>>,
    ) -> Result<(), CovariateError> {
        let grouped: HashMap<String, Vec<_>> =
            data.into_iter().fold(HashMap::new(), |mut acc, value| {
                acc.entry(value.pnr.clone()).or_default().push(value);
                acc
            });

        for (pnr, values) in grouped {
            self.education.insert(pnr, values);
        }
        Ok(())
    }

    pub fn load_income(&self, data: Vec<TimeVaryingValue<Income>>) -> Result<(), CovariateError> {
        let grouped: HashMap<String, Vec<_>> =
            data.into_iter().fold(HashMap::new(), |mut acc, value| {
                acc.entry(value.pnr.clone()).or_default().push(value);
                acc
            });

        for (pnr, mut values) in grouped {
            // Sort by date
            values.sort_by_key(|v| v.date);
            self.income.insert(pnr, values);
        }
        Ok(())
    }

    pub fn load_occupation(
        &self,
        data: Vec<TimeVaryingValue<Occupation>>,
    ) -> Result<(), CovariateError> {
        let grouped: HashMap<String, Vec<_>> =
            data.into_iter().fold(HashMap::new(), |mut acc, value| {
                acc.entry(value.pnr.clone()).or_default().push(value);
                acc
            });

        for (pnr, mut values) in grouped {
            // Sort by date
            values.sort_by_key(|v| v.date);
            self.occupation.insert(pnr, values);
        }
        Ok(())
    }
}

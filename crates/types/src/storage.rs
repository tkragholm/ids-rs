use arrow::record_batch::RecordBatch;
use chrono::NaiveDate;
use dashmap::DashMap;
use std::collections::HashMap;

use crate::{
    error::IdsError,
    family::FamilyRelations,
    models::{Covariate, CovariateType, TimeVaryingValue},
};

#[derive(Debug)]
pub enum StorageBackend {
    Arrow(ArrowStorage),
    TimeVarying(TimeVaryingStorage),
}

pub trait Storage: Send + Sync {
    fn get_covariate(
        &self,
        pnr: &str,
        covariate_type: CovariateType,
        date: NaiveDate,
    ) -> Result<Option<Covariate>, IdsError>;

    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;

    fn load_data(&mut self, data: Vec<TimeVaryingValue<Covariate>>) -> Result<(), IdsError>;
}

#[derive(Debug, Clone)]
pub struct ArrowStorage {
    family_data: HashMap<String, FamilyRelations>,
    akm_data: HashMap<i32, Vec<RecordBatch>>,
    bef_data: HashMap<String, Vec<RecordBatch>>,
    ind_data: HashMap<i32, Vec<RecordBatch>>,
    uddf_data: HashMap<String, Vec<RecordBatch>>,
}

#[derive(Debug)]
pub struct TimeVaryingStorage {
    data: DashMap<String, Vec<TimeVaryingValue<Covariate>>>,
    family_data: HashMap<String, FamilyRelations>,
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct CacheKey {
    pub pnr: String,
    pub covariate_type: CovariateType,
    pub date: NaiveDate,
}

pub struct DataStore {
    storage: StorageBackend,
    cache: DashMap<CacheKey, Covariate>,
}

use crate::{
    arrow_utils::{ArrowDataHelper, ArrowStore},
    error::IdsError,
};
use arrow::{array::Array, record_batch::RecordBatch};
pub use chrono::NaiveDate;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct FamilyRelations {
    pub pnr: String,
    pub birth_date: NaiveDate,
    pub father_id: Option<String>,
    pub father_birth_date: Option<NaiveDate>,
    pub mother_id: Option<String>,
    pub mother_birth_date: Option<NaiveDate>,
    pub family_id: Option<String>,
}

#[derive(Clone)]
pub struct FamilyStore {
    #[allow(dead_code)]
    arrow_store: ArrowStore,
    relations: HashMap<String, FamilyRelations>,
}

impl FamilyStore {
    pub fn new(arrow_store: ArrowStore) -> Self {
        Self {
            arrow_store,
            relations: HashMap::new(),
        }
    }

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        for batch in batches {
            // Get all required column arrays
            let pnr_array = self.get_string_array(&batch, "pnr")?;
            let birth_date_array = self.get_date_array(&batch, "birth_date")?;
            let father_id_array = self.get_string_array(&batch, "father_id")?;
            let father_birth_date_array = self.get_date_array(&batch, "father_birth_date")?;
            let mother_id_array = self.get_string_array(&batch, "mother_id")?;
            let mother_birth_date_array = self.get_date_array(&batch, "mother_birth_date")?;
            let family_id_array = self.get_string_array(&batch, "family_id")?;

            for i in 0..batch.num_rows() {
                let pnr = pnr_array.value(i).to_string();

                // Convert date32 to NaiveDate
                let birth_date = self.convert_date32_to_naive_date(birth_date_array.value(i))?;

                // Handle optional fields
                let father_id = if father_id_array.is_null(i) {
                    None
                } else {
                    Some(father_id_array.value(i).to_string())
                };

                let father_birth_date = if father_birth_date_array.is_null(i) {
                    None
                } else {
                    Some(self.convert_date32_to_naive_date(father_birth_date_array.value(i))?)
                };

                let mother_id = if mother_id_array.is_null(i) {
                    None
                } else {
                    Some(mother_id_array.value(i).to_string())
                };

                let mother_birth_date = if mother_birth_date_array.is_null(i) {
                    None
                } else {
                    Some(self.convert_date32_to_naive_date(mother_birth_date_array.value(i))?)
                };

                let family_id = if family_id_array.is_null(i) {
                    None
                } else {
                    Some(family_id_array.value(i).to_string())
                };

                // Create and store FamilyRelations
                let relation = FamilyRelations {
                    pnr,
                    birth_date,
                    father_id,
                    father_birth_date,
                    mother_id,
                    mother_birth_date,
                    family_id,
                };

                self.relations.insert(relation.pnr.clone(), relation);
            }
        }
        Ok(())
    }

    pub fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.relations.get(pnr)
    }
}

pub trait FamilyAccess {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;
    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)>;
    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate>;
}

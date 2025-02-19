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
    #[must_use]
    pub fn new(arrow_store: ArrowStore) -> Self {
        Self {
            arrow_store,
            relations: HashMap::new(),
        }
    }

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        for batch in batches {
            // Get all required column arrays
            let pnr_array = self.get_string_array(&batch, "PNR")?;
            let birth_date_array = self.get_date_array(&batch, "BIRTH_DATE")?;
            let father_id_array = self.get_string_array(&batch, "FATHER_ID")?;
            let father_birth_date_array = self.get_date_array(&batch, "FATHER_BIRTH_DATE")?;
            let mother_id_array = self.get_string_array(&batch, "MOTHER_ID")?;
            let mother_birth_date_array = self.get_date_array(&batch, "MOTHER_BIRTH_DATE")?;
            let family_id_array = self.get_string_array(&batch, "FAMILY_ID")?;

            for i in 0..batch.num_rows() {
                let pnr = pnr_array.value(i).to_string();

                // Convert birth date with validation
                let birth_date = match self.convert_date32_to_naive_date(birth_date_array.value(i))
                {
                    Ok(date) => date,
                    Err(e) => {
                        log::warn!("Invalid birth date for PNR {}: {}", pnr, e);
                        continue;
                    }
                };

                // Handle optional father data
                let (father_id, father_birth_date) = if father_id_array.is_null(i) {
                    (None, None)
                } else {
                    let father_id = Some(father_id_array.value(i).to_string());
                    let father_birth_date = if father_birth_date_array.is_null(i) {
                        None
                    } else {
                        match self.convert_date32_to_naive_date(father_birth_date_array.value(i)) {
                            Ok(date) => Some(date),
                            Err(e) => {
                                log::warn!("Invalid father birth date for PNR {}: {}", pnr, e);
                                None
                            }
                        }
                    };
                    (father_id, father_birth_date)
                };

                // Handle optional mother data
                let (mother_id, mother_birth_date) = if mother_id_array.is_null(i) {
                    (None, None)
                } else {
                    let mother_id = Some(mother_id_array.value(i).to_string());
                    let mother_birth_date = if mother_birth_date_array.is_null(i) {
                        None
                    } else {
                        match self.convert_date32_to_naive_date(mother_birth_date_array.value(i)) {
                            Ok(date) => Some(date),
                            Err(e) => {
                                log::warn!("Invalid mother birth date for PNR {}: {}", pnr, e);
                                None
                            }
                        }
                    };
                    (mother_id, mother_birth_date)
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

    #[must_use]
    pub fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.relations.get(pnr)
    }
}

pub trait FamilyAccess {
    fn get_family_relations(&self, pnr: &str) -> Option<&FamilyRelations>;
    fn get_parents(&self, pnr: &str) -> Option<(Option<String>, Option<String>)>;
    fn get_birth_date(&self, pnr: &str) -> Option<NaiveDate>;
}

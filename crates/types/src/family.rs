use crate::{
    arrow_utils::ArrowAccess, // Add ArrowValue back
    error::IdsError,
    store::ArrowStore,
};
use arrow::{
    array::Array, // Add specific array types
    record_batch::RecordBatch,
};
use chrono::NaiveDate; // Add Days type from chrono
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

#[derive(Clone, Debug)]
pub struct FamilyStore {
    pub arrow_backend: ArrowStore,
    pub relations: HashMap<String, FamilyRelations>,
}

impl FamilyStore {
    #[must_use]
    pub fn new(arrow_backend: ArrowStore) -> Self {
        Self {
            arrow_backend,
            relations: HashMap::new(),
        }
    }

    pub fn get_arrow_backend(&self) -> &ArrowStore {
        &self.arrow_backend
    }

    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        for batch in batches {
            self.process_batch(&batch)?;
        }
        Ok(())
    }

    fn process_batch(&mut self, batch: &RecordBatch) -> Result<(), IdsError> {
        let pnr_array = self.get_string_array(batch, "PNR")?;
        let birth_date_array = self.get_date_array(batch, "BIRTH_DATE")?;
        let father_id_array = self.get_string_array(batch, "FATHER_ID")?;
        let father_birth_date_array = self.get_date_array(batch, "FATHER_BIRTH_DATE")?;
        let mother_id_array = self.get_string_array(batch, "MOTHER_ID")?;
        let mother_birth_date_array = self.get_date_array(batch, "MOTHER_BIRTH_DATE")?;
        let family_id_array = self.get_string_array(batch, "FAMILY_ID")?;

        for i in 0..batch.num_rows() {
            let pnr = pnr_array.value(i).to_string();
            let birth_date = self.convert_date32_to_naive_date(birth_date_array.value(i))?;

            let relation = FamilyRelations {
                pnr: pnr.clone(),
                birth_date,
                father_id: if father_id_array.is_null(i) {
                    None
                } else {
                    Some(father_id_array.value(i).to_string())
                },
                father_birth_date: if father_birth_date_array.is_null(i) {
                    None
                } else {
                    Some(self.convert_date32_to_naive_date(father_birth_date_array.value(i))?)
                },
                mother_id: if mother_id_array.is_null(i) {
                    None
                } else {
                    Some(mother_id_array.value(i).to_string())
                },
                mother_birth_date: if mother_birth_date_array.is_null(i) {
                    None
                } else {
                    Some(self.convert_date32_to_naive_date(mother_birth_date_array.value(i))?)
                },
                family_id: if family_id_array.is_null(i) {
                    None
                } else {
                    Some(family_id_array.value(i).to_string())
                },
            };

            self.relations.insert(pnr, relation);
        }
        Ok(())
    }
}

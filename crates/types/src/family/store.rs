use crate::{
    error::IdsError,
    family::FamilyRelations,
};
use arrow::{
    array::{Array, StringArray, Date32Array},
    record_batch::RecordBatch,
};
use chrono::NaiveDate;
use hashbrown::HashMap;

/// A store for family relationships
#[derive(Clone, Debug)]
pub struct FamilyStore {
    pub relations: HashMap<String, FamilyRelations>,
}

impl Default for FamilyStore {
    fn default() -> Self {
        Self::new()
    }
}

impl FamilyStore {
    /// Create a new, empty family store
    #[must_use]
    pub fn new() -> Self {
        Self {
            relations: HashMap::new(),
        }
    }

    /// Get all relations in this store
    #[must_use] 
    pub fn get_relations(&self) -> &HashMap<String, FamilyRelations> {
        &self.relations
    }

    /// Get a specific relation by PNR
    #[must_use]
    pub fn get_relation(&self, pnr: &str) -> Option<&FamilyRelations> {
        self.relations.get(pnr)
    }

    /// Add a relation to this store
    pub fn add_relation(&mut self, relation: FamilyRelations) {
        self.relations.insert(relation.pnr.clone(), relation);
    }

    /// Load family relations from a set of Arrow RecordBatches
    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<(), IdsError> {
        for batch in batches {
            self.process_batch(&batch)?;
        }
        Ok(())
    }

    fn process_batch(&mut self, batch: &RecordBatch) -> Result<(), IdsError> {
        // Get the string arrays from the batch
        let pnr_column = batch.column(batch.schema().index_of("PNR")?);
        let pnr_array = pnr_column.as_any().downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::data_loading("PNR column is not a string array".to_string()))?;
            
        let birth_date_column = batch.column(batch.schema().index_of("BIRTH_DATE")?);
        let birth_date_array = birth_date_column.as_any().downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::data_loading("BIRTH_DATE column is not a date array".to_string()))?;
            
        let father_id_column = batch.column(batch.schema().index_of("FATHER_ID")?);
        let father_id_array = father_id_column.as_any().downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::data_loading("FATHER_ID column is not a string array".to_string()))?;
            
        let father_birth_date_column = batch.column(batch.schema().index_of("FATHER_BIRTH_DATE")?);
        let father_birth_date_array = father_birth_date_column.as_any().downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::data_loading("FATHER_BIRTH_DATE column is not a date array".to_string()))?;
            
        let mother_id_column = batch.column(batch.schema().index_of("MOTHER_ID")?);
        let mother_id_array = mother_id_column.as_any().downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::data_loading("MOTHER_ID column is not a string array".to_string()))?;
            
        let mother_birth_date_column = batch.column(batch.schema().index_of("MOTHER_BIRTH_DATE")?);
        let mother_birth_date_array = mother_birth_date_column.as_any().downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::data_loading("MOTHER_BIRTH_DATE column is not a date array".to_string()))?;
            
        let family_id_column = batch.column(batch.schema().index_of("FAMILY_ID")?);
        let family_id_array = family_id_column.as_any().downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::data_loading("FAMILY_ID column is not a string array".to_string()))?;

        // Helper function to convert date32 to NaiveDate
        fn convert_date32_to_naive_date(days_since_epoch: i32) -> Result<NaiveDate, IdsError> {
            // Use the new safe method
            NaiveDate::from_num_days_from_ce_opt(days_since_epoch)
                .ok_or_else(|| IdsError::data_loading(format!(
                    "Could not convert {} days since epoch to date", days_since_epoch
                )))
        }
        
        for i in 0..batch.num_rows() {
            let pnr = pnr_array.value(i).to_string();
            let birth_date = convert_date32_to_naive_date(birth_date_array.value(i))?;

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
                    Some(convert_date32_to_naive_date(father_birth_date_array.value(i))?)
                },
                mother_id: if mother_id_array.is_null(i) {
                    None
                } else {
                    Some(mother_id_array.value(i).to_string())
                },
                mother_birth_date: if mother_birth_date_array.is_null(i) {
                    None
                } else {
                    Some(convert_date32_to_naive_date(mother_birth_date_array.value(i))?)
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
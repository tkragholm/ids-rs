use super::relations::FamilyRelations;
use crate::error::{IdsError, Result};
use arrow::{array::Array, record_batch::RecordBatch};
use chrono::NaiveDate;
use hashbrown::HashMap;

/// Convert an Arrow Date32 value (days since epoch) to a NaiveDate
fn convert_date32_to_naive_date(days_since_epoch: i32) -> Result<NaiveDate> {
    NaiveDate::from_num_days_from_ce_opt(days_since_epoch).ok_or_else(|| {
        IdsError::date_conversion(format!(
            "Could not convert {} days since epoch to date",
            days_since_epoch
        ))
    })
}

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
    pub fn load_family_relations(&mut self, batches: Vec<RecordBatch>) -> Result<()> {
        for batch in batches {
            self.process_batch(&batch)?;
        }
        Ok(())
    }

    fn process_batch(&mut self, batch: &RecordBatch) -> Result<()> {
        // Use the batch directly as it implements ArrowAccess

        // Get the arrays using the batch directly
        let pnr_array = batch.column(batch.schema().index_of("PNR")?);
        let birth_date_array = batch.column(batch.schema().index_of("BIRTH_DATE")?);
        let father_id_array = batch.column(batch.schema().index_of("FATHER_ID")?);
        let father_birth_date_array = batch.column(batch.schema().index_of("FATHER_BIRTH_DATE")?);
        let mother_id_array = batch.column(batch.schema().index_of("MOTHER_ID")?);
        let mother_birth_date_array = batch.column(batch.schema().index_of("MOTHER_BIRTH_DATE")?);
        let family_id_array = batch.column(batch.schema().index_of("FAMILY_ID")?);

        // Convert arrays to appropriate types
        let pnr_array = pnr_array
            .as_any()
            .downcast_ref::<arrow::array::StringArray>()
            .ok_or_else(|| {
                crate::error::IdsError::data_loading(
                    "Failed to convert PNR array to StringArray".to_string(),
                )
            })?;
        let birth_date_array = birth_date_array
            .as_any()
            .downcast_ref::<arrow::array::Date32Array>()
            .ok_or_else(|| {
                crate::error::IdsError::data_loading(
                    "Failed to convert BIRTH_DATE array to Date32Array".to_string(),
                )
            })?;
        let father_id_array = father_id_array
            .as_any()
            .downcast_ref::<arrow::array::StringArray>()
            .ok_or_else(|| {
                crate::error::IdsError::data_loading(
                    "Failed to convert FATHER_ID array to StringArray".to_string(),
                )
            })?;
        let father_birth_date_array = father_birth_date_array
            .as_any()
            .downcast_ref::<arrow::array::Date32Array>()
            .ok_or_else(|| {
                crate::error::IdsError::data_loading(
                    "Failed to convert FATHER_BIRTH_DATE array to Date32Array".to_string(),
                )
            })?;
        let mother_id_array = mother_id_array
            .as_any()
            .downcast_ref::<arrow::array::StringArray>()
            .ok_or_else(|| {
                crate::error::IdsError::data_loading(
                    "Failed to convert MOTHER_ID array to StringArray".to_string(),
                )
            })?;
        let mother_birth_date_array = mother_birth_date_array
            .as_any()
            .downcast_ref::<arrow::array::Date32Array>()
            .ok_or_else(|| {
                crate::error::IdsError::data_loading(
                    "Failed to convert MOTHER_BIRTH_DATE array to Date32Array".to_string(),
                )
            })?;
        let family_id_array = family_id_array
            .as_any()
            .downcast_ref::<arrow::array::StringArray>()
            .ok_or_else(|| {
                crate::error::IdsError::data_loading(
                    "Failed to convert FAMILY_ID array to StringArray".to_string(),
                )
            })?;

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
                    Some(convert_date32_to_naive_date(
                        father_birth_date_array.value(i),
                    )?)
                },
                mother_id: if mother_id_array.is_null(i) {
                    None
                } else {
                    Some(mother_id_array.value(i).to_string())
                },
                mother_birth_date: if mother_birth_date_array.is_null(i) {
                    None
                } else {
                    Some(convert_date32_to_naive_date(
                        mother_birth_date_array.value(i),
                    )?)
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

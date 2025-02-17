use super::types::ArrowPrimitive;
use crate::error::IdsError;
use arrow::array::{Array, Date32Array, StringArray};
use arrow::record_batch::RecordBatch;
use chrono::{Days, NaiveDate};

pub trait ArrowAccess {
    fn find_pnr_index(&self, batch: &RecordBatch, pnr: &str) -> Result<Option<usize>, IdsError>;
    fn get_value_at_index<T: ArrowPrimitive>(
        &self,
        batch: &RecordBatch,
        column: &str,
        index: usize,
    ) -> Result<Option<T>, IdsError>;
}

pub trait ArrowDataHelper {
    fn get_string_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a StringArray, IdsError>;

    fn get_date_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a Date32Array, IdsError>;

    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate, IdsError>;
}

// Default implementation that can be used by any type
impl<T> ArrowDataHelper for T {
    fn get_string_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a StringArray, IdsError> {
        batch
            .column_by_name(column_name)
            .ok_or_else(|| IdsError::MissingData(format!("{} column not found", column_name)))?
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::InvalidFormat(format!("Invalid {} array type", column_name)))
    }

    fn get_date_array<'a>(
        &self,
        batch: &'a RecordBatch,
        column_name: &str,
    ) -> Result<&'a Date32Array, IdsError> {
        batch
            .column_by_name(column_name)
            .ok_or_else(|| IdsError::MissingData(format!("{} column not found", column_name)))?
            .as_any()
            .downcast_ref::<Date32Array>()
            .ok_or_else(|| IdsError::InvalidFormat(format!("Invalid {} array type", column_name)))
    }

    fn convert_date32_to_naive_date(&self, days_since_epoch: i32) -> Result<NaiveDate, IdsError> {
        let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
            .ok_or_else(|| IdsError::InvalidDate("Invalid epoch date".to_string()))?;

        epoch
            .checked_add_days(Days::new(days_since_epoch as u64))
            .ok_or_else(|| {
                IdsError::InvalidDate(format!("Invalid date value: {}", days_since_epoch))
            })
    }
}

//! Date utilities for working with Arrow arrays.

use chrono::{Datelike, NaiveDate};
use crate::error::{IdsError, Result};
use crate::utils::date_utils::parsing::parse_flexible;
use arrow::array::{Array, ArrayRef, Date32Array, Date64Array, StringArray, Int32Array};
use arrow::compute::filter;
use arrow::array::BooleanArray;
use log::warn;

/// Extracts a `NaiveDate` from an Arrow array element
/// 
/// This function tries to handle multiple Arrow date types:
/// - `Date32Array` (days since epoch)
/// - `Date64Array` (milliseconds since epoch)
/// - `StringArray` (with date strings that can be parsed)
/// - `Int32Array` (assuming YYYYMMDD format)
///
/// Returns Some(NaiveDate) if a date could be extracted, None otherwise
#[must_use] 
pub fn extract_date_from_array(array: &dyn Array, row_index: usize) -> Option<NaiveDate> {
    if array.is_null(row_index) {
        return None;
    }
    
    if let Some(date32_array) = array.as_any().downcast_ref::<Date32Array>() {
        if !date32_array.is_null(row_index) {
            let date_i32 = date32_array.value(row_index);
            return arrow::temporal_conversions::date32_to_datetime(date_i32)
                .map(|dt| dt.date());
        }
    } else if let Some(date64_array) = array.as_any().downcast_ref::<Date64Array>() {
        if !date64_array.is_null(row_index) {
            let date_i64 = date64_array.value(row_index);
            return arrow::temporal_conversions::timestamp_ms_to_datetime(date_i64)
                .map(|dt| dt.date());
        }
    } else if let Some(string_array) = array.as_any().downcast_ref::<StringArray>() {
        let date_str = string_array.value(row_index);
        if let Ok(date) = parse_flexible(date_str) {
            return Some(date);
        }
    } else if let Some(int_array) = array.as_any().downcast_ref::<Int32Array>() {
        let date_int = int_array.value(row_index);
        let year = date_int / 10000;
        let month = ((date_int % 10000) / 100) as u32;
        let day = (date_int % 100) as u32;
        
        if (1..=12).contains(&month) && (1..=31).contains(&day) {
            return NaiveDate::from_ymd_opt(year, month, day);
        }
    }
    
    None
}

/// Extracts years from an Arrow array that contains date information
/// 
/// This function tries to handle multiple Arrow date types:
/// - `Date32Array` (days since epoch)
/// - `Date64Array` (milliseconds since epoch)
/// - `StringArray` (with date strings that can be parsed)
/// - `Int32Array` (assuming YYYYMMDD format)
///
/// Returns a boolean mask indicating which rows have years in the specified range
pub fn filter_by_year_range(array: &ArrayRef, start_year: i32, end_year: i32) -> Result<BooleanArray> {
    let rows = array.len();
    let mut mask = vec![false; rows];
    
    if let Some(date32_array) = array.as_any().downcast_ref::<Date32Array>() {
        // Handle Date32Array (days since epoch)
        for (i, maybe_date) in date32_array.iter().enumerate() {
            if let Some(date_i32) = maybe_date {
                if let Some(datetime) = arrow::temporal_conversions::date32_to_datetime(date_i32) {
                    let year = datetime.date().year();
                    mask[i] = year >= start_year && year <= end_year;
                }
            }
        }
    } else if let Some(date64_array) = array.as_any().downcast_ref::<Date64Array>() {
        // Handle Date64Array (milliseconds since epoch)
        for (i, maybe_date) in date64_array.iter().enumerate() {
            if let Some(date_i64) = maybe_date {
                if let Some(datetime) = arrow::temporal_conversions::timestamp_ms_to_datetime(date_i64) {
                    let year = datetime.date().year();
                    mask[i] = year >= start_year && year <= end_year;
                }
            }
        }
    } else if let Some(string_array) = array.as_any().downcast_ref::<StringArray>() {
        // Handle StringArray by trying to parse as dates
        for (i, mask_item) in mask.iter_mut().enumerate().take(string_array.len()) {
            if !string_array.is_null(i) {
                let date_str = string_array.value(i);
                if let Ok(date) = parse_flexible(date_str) {
                    let year = date.year();
                    *mask_item = year >= start_year && year <= end_year;
                }
            }
        }
    } else if let Some(int_array) = array.as_any().downcast_ref::<Int32Array>() {
        // Handle Int32Array assuming YYYYMMDD format
        for (i, mask_item) in mask.iter_mut().enumerate().take(int_array.len()) {
            if !int_array.is_null(i) {
                let date_int = int_array.value(i);
                let year = date_int / 10000; // Extract year from YYYYMMDD
                *mask_item = year >= start_year && year <= end_year;
            }
        }
    } else {
        // Unsupported array type
        warn!("Unsupported array type for date filtering. Allowing all rows.");
        // Default to keeping all rows
        for mask_item in mask.iter_mut().take(rows) {
            *mask_item = true;
        }
    }
    
    Ok(BooleanArray::from(mask))
}

/// Filters a vector of Arrow arrays using a boolean mask
/// 
/// This applies the same filter mask to all columns in the input array
pub fn filter_arrays(arrays: &[ArrayRef], mask: &BooleanArray) -> Result<Vec<ArrayRef>> {
    let mut filtered_arrays = Vec::with_capacity(arrays.len());
    
    for arr in arrays {
        let filtered = filter(arr, mask)
            .map_err(|e| IdsError::Data(format!("Error filtering array: {e}")))?;
        filtered_arrays.push(filtered);
    }
    
    Ok(filtered_arrays)
}

/// Converts an Arrow array with date information to a `Date32Array` (days since epoch)
///
/// This function handles various date formats:
/// - `Date32Array` (returned as is)
/// - `Date64Array` (milliseconds to days conversion)
/// - `StringArray` (parsed and converted to days)
/// - `Int32Array` (assuming YYYYMMDD format)
///
/// Returns a `Date32Array` representing days since Unix epoch (January 1, 1970)
pub fn convert_to_date32_array(array: &dyn Array) -> Result<Date32Array> {
    let rows = array.len();
    let mut date_values: Vec<Option<i32>> = Vec::with_capacity(rows);
    
    // Get the epoch reference date (1970-01-01)
    let epoch = NaiveDate::from_ymd_opt(1970, 1, 1)
        .expect("Epoch date should be valid");
    
    // If it's already a Date32Array, we can just clone it
    if let Some(date32_array) = array.as_any().downcast_ref::<Date32Array>() {
        return Ok(date32_array.clone());
    }
    
    // Otherwise, extract dates and convert to days since epoch
    for i in 0..rows {
        let date_opt = extract_date_from_array(array, i);
        
        // Convert to days since epoch (Date32 format)
        let epoch_day_opt = date_opt.map(|d| {
            d.signed_duration_since(epoch).num_days() as i32
        });
        
        date_values.push(epoch_day_opt);
    }
    
    // Create and return a new Date32Array
    Ok(Date32Array::from(date_values))
}

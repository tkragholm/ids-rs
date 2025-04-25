//! Population generation algorithms for combining BEF and MFR data

use arrow::array::BooleanArray;
use arrow::array::{Array, Date32Array, StringArray};
use arrow::compute::filter;
use arrow::datatypes::{DataType, Field, Schema};
use arrow::record_batch::RecordBatch;
use chrono::{Datelike, NaiveDate};
use std::collections::{HashMap, HashSet};

use crate::error::{IdsError, Result};
use crate::model::pnr::Pnr;
use crate::model::population::{FamilyData, Population};
use crate::schema::filter_expr::Expr;

/// Configuration for population generation
pub struct PopulationConfig {
    /// Start year for filtering births (inclusive)
    pub birth_inclusion_start_year: i32,
    /// End year for filtering births (inclusive)
    pub birth_inclusion_end_year: i32,
}

impl Default for PopulationConfig {
    fn default() -> Self {
        Self {
            birth_inclusion_start_year: 1995,
            birth_inclusion_end_year: 2020,
        }
    }
}

/// Summary statistics for population data
#[derive(Debug, Clone)]
pub struct PopulationSummary {
    /// Total records from BEF register
    pub total_bef_records: usize,
    /// Total records from MFR register
    pub total_mfr_records: usize,
    /// Number of BEF records with missing father
    pub bef_missing_father: usize,
    /// Number of BEF records with missing mother
    pub bef_missing_mother: usize,
    /// Number of MFR records with missing father
    pub mfr_missing_father: usize,
    /// Number of MFR records with missing mother
    pub mfr_missing_mother: usize,
    /// Total combined records
    pub total_combined_records: usize,
    /// Number of combined records with missing father
    pub combined_missing_father: usize,
    /// Number of combined records with missing mother
    pub combined_missing_mother: usize,
    /// Number of records only in BEF
    pub records_only_in_bef: usize,
    /// Number of records only in MFR
    pub records_only_in_mfr: usize,
}

/// Creates a filter expression for birth year range
#[must_use] pub fn create_birth_year_filter(column_name: &str, start_year: i32, end_year: i32) -> Expr {
    use crate::schema::filter_expr::col;

    // Create filter for birth year >= start_year
    let start_filter = col(column_name).gt(i64::from(start_year) - 1);

    // Create filter for birth year <= end_year
    let end_filter = col(column_name).lt(i64::from(end_year) + 1);

    // Combine with AND
    start_filter.and(end_filter)
}

/// Filter record batch by birth year
pub fn filter_by_birth_year(
    batch: &RecordBatch,
    date_column: &str,
    start_year: i32,
    end_year: i32,
) -> Result<RecordBatch> {
    // Get the date column
    let date_col = batch
        .column_by_name(date_column)
        .ok_or_else(|| IdsError::Data(format!("Missing {date_column} column")))?;

    // Extract year from date
    let date_array = date_col
        .as_any()
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data(format!("{date_column} column is not a date array")))?;

    // Create a filter for birth year range
    let mut mask = vec![false; batch.num_rows()];

    for (i, maybe_date) in date_array.iter().enumerate() {
        if let Some(date_i32) = maybe_date {
            // Convert the i32 date value to a NaiveDateTime using arrow's temporal conversions
            // date32_to_datetime returns a chrono::NaiveDateTime
            let datetime = arrow::temporal_conversions::date32_to_datetime(date_i32).unwrap();
            // Extract year using chrono::Datelike trait
            let year = datetime.date().year();
            mask[i] = year >= start_year && year <= end_year;
        }
    }

    let mask_array = BooleanArray::from(mask);

    // Apply the filter to each column and create a new batch
    let mut filtered_columns = Vec::with_capacity(batch.num_columns());
    for col in batch.columns() {
        let filtered_col = filter(col, &mask_array)
            .map_err(|e| IdsError::Data(format!("Error filtering column: {e}")))?;
        filtered_columns.push(filtered_col);
    }

    // Create a new RecordBatch with the filtered columns
    let filtered_batch = RecordBatch::try_new(batch.schema(), filtered_columns)
        .map_err(|e| IdsError::Data(format!("Error creating filtered batch: {e}")))?;

    Ok(filtered_batch)
}

/// Extract records from BEF data and standardize columns
pub fn extract_bef_children(
    bef_data: &RecordBatch,
    config: &PopulationConfig,
) -> Result<RecordBatch> {
    // Apply birth year filter
    let filtered_batch = filter_by_birth_year(
        bef_data,
        "FOED_DAG",
        config.birth_inclusion_start_year,
        config.birth_inclusion_end_year,
    )?;

    // Return the filtered batch (already has standard column names)
    Ok(filtered_batch)
}

/// Extract records from MFR data and standardize columns
pub fn extract_mfr_children(
    mfr_data: &RecordBatch,
    config: &PopulationConfig,
) -> Result<RecordBatch> {
    // Apply birth year filter
    let filtered_batch = filter_by_birth_year(
        mfr_data,
        "FOEDSELSDATO",
        config.birth_inclusion_start_year,
        config.birth_inclusion_end_year,
    )?;

    // Need to standardize column names
    let mut columns = Vec::with_capacity(5);
    let mut fields = Vec::with_capacity(5);

    // Map MFR columns to standard names
    let schema = filtered_batch.schema();
    let mfr_columns = filtered_batch.columns();

    // PNR (from CPR_BARN)
    let pnr_idx = schema
        .index_of("CPR_BARN")
        .map_err(|e| IdsError::Data(format!("Column CPR_BARN not found: {e}")))?;
    columns.push(mfr_columns[pnr_idx].clone());
    fields.push(Field::new("PNR", DataType::Utf8, true));

    // FOED_DAG (from FOEDSELSDATO)
    let birth_date_idx = schema
        .index_of("FOEDSELSDATO")
        .map_err(|e| IdsError::Data(format!("Column FOEDSELSDATO not found: {e}")))?;
    columns.push(mfr_columns[birth_date_idx].clone());
    fields.push(Field::new("FOED_DAG", DataType::Date32, true));

    // FAR_ID (from CPR_FADER)
    let father_idx = schema
        .index_of("CPR_FADER")
        .map_err(|e| IdsError::Data(format!("Column CPR_FADER not found: {e}")))?;
    columns.push(mfr_columns[father_idx].clone());
    fields.push(Field::new("FAR_ID", DataType::Utf8, true));

    // MOR_ID (from CPR_MODER)
    let mother_idx = schema
        .index_of("CPR_MODER")
        .map_err(|e| IdsError::Data(format!("Column CPR_MODER not found: {e}")))?;
    columns.push(mfr_columns[mother_idx].clone());
    fields.push(Field::new("MOR_ID", DataType::Utf8, true));

    // Add a null FAMILIE_ID column
    let null_values: Vec<Option<String>> = vec![None; filtered_batch.num_rows()];
    let family_id_array = StringArray::from(null_values);
    columns.push(std::sync::Arc::new(family_id_array));
    fields.push(Field::new("FAMILIE_ID", DataType::Utf8, true));

    // Create a new schema
    let new_schema = Schema::new(fields);

    // Create a new RecordBatch with standardized columns
    let std_batch = RecordBatch::try_new(std::sync::Arc::new(new_schema), columns)
        .map_err(|e| IdsError::Data(format!("Error creating standardized batch: {e}")))?;

    Ok(std_batch)
}

/// Combine children data from BEF and MFR
pub fn combine_children_data(
    bef_children: &RecordBatch,
    mfr_children: &RecordBatch,
) -> Result<(RecordBatch, PopulationSummary)> {
    // Calculate summary statistics before merge
    let summary_before = PopulationSummary {
        total_bef_records: bef_children.num_rows(),
        total_mfr_records: mfr_children.num_rows(),
        bef_missing_father: count_null_values(bef_children, "FAR_ID")?,
        bef_missing_mother: count_null_values(bef_children, "MOR_ID")?,
        mfr_missing_father: count_null_values(mfr_children, "FAR_ID")?,
        mfr_missing_mother: count_null_values(mfr_children, "MOR_ID")?,
        // These will be filled in after merge
        total_combined_records: 0,
        combined_missing_father: 0,
        combined_missing_mother: 0,
        records_only_in_bef: 0,
        records_only_in_mfr: 0,
    };

    // Extract PNRs from both datasets for matching
    let bef_pnr_col = bef_children
        .column_by_name("PNR")
        .ok_or_else(|| IdsError::Data("Missing PNR column in BEF data".to_string()))?;
    let bef_pnr_array = bef_pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column in BEF is not a string array".to_string()))?;

    let mfr_pnr_col = mfr_children
        .column_by_name("PNR")
        .ok_or_else(|| IdsError::Data("Missing PNR column in MFR data".to_string()))?;
    let mfr_pnr_array = mfr_pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column in MFR is not a string array".to_string()))?;

    // Build sets for checking overlaps
    let mut bef_pnrs = HashSet::new();
    for i in 0..bef_pnr_array.len() {
        if !bef_pnr_array.is_null(i) {
            bef_pnrs.insert(bef_pnr_array.value(i).to_string());
        }
    }

    let mut mfr_pnrs = HashSet::new();
    for i in 0..mfr_pnr_array.len() {
        if !mfr_pnr_array.is_null(i) {
            mfr_pnrs.insert(mfr_pnr_array.value(i).to_string());
        }
    }

    // Count records only in BEF and only in MFR
    let records_only_in_bef = bef_pnrs
        .iter()
        .filter(|pnr| !mfr_pnrs.contains(*pnr))
        .count();

    let records_only_in_mfr = mfr_pnrs
        .iter()
        .filter(|pnr| !bef_pnrs.contains(*pnr))
        .count();

    // Combine all unique PNRs
    let mut all_pnrs = bef_pnrs;
    all_pnrs.extend(mfr_pnrs);
    let total_combined_records = all_pnrs.len();

    // Process each unique PNR to combine data
    let mut combined_pnrs = Vec::with_capacity(total_combined_records);
    let mut combined_birth_dates = Vec::with_capacity(total_combined_records);
    let mut combined_father_ids = Vec::with_capacity(total_combined_records);
    let mut combined_mother_ids = Vec::with_capacity(total_combined_records);
    let mut combined_family_ids = Vec::with_capacity(total_combined_records);

    for pnr in all_pnrs {
        let mut birth_date = None;
        let mut father_id = None;
        let mut mother_id = None;
        let mut family_id = None;

        // Check if in BEF data
        for i in 0..bef_pnr_array.len() {
            if !bef_pnr_array.is_null(i) && bef_pnr_array.value(i) == pnr {
                // Get BEF values
                birth_date = get_date_value(bef_children, "FOED_DAG", i)?;
                father_id = get_string_value(bef_children, "FAR_ID", i)?;
                mother_id = get_string_value(bef_children, "MOR_ID", i)?;
                family_id = get_string_value(bef_children, "FAMILIE_ID", i)?;
                break;
            }
        }

        // Check if in MFR data and fill in missing values
        for i in 0..mfr_pnr_array.len() {
            if !mfr_pnr_array.is_null(i) && mfr_pnr_array.value(i) == pnr {
                // Fill in from MFR if missing in BEF
                if birth_date.is_none() {
                    birth_date = get_date_value(mfr_children, "FOED_DAG", i)?;
                }
                if father_id.is_none() {
                    father_id = get_string_value(mfr_children, "FAR_ID", i)?;
                }
                if mother_id.is_none() {
                    mother_id = get_string_value(mfr_children, "MOR_ID", i)?;
                }
                // Note: MFR doesn't have FAMILIE_ID, so no need to check
                break;
            }
        }

        // Add to combined arrays
        combined_pnrs.push(Some(pnr));
        combined_birth_dates.push(birth_date);
        combined_father_ids.push(father_id);
        combined_mother_ids.push(mother_id);
        combined_family_ids.push(family_id);
    }

    // Count missing values in combined data
    let combined_missing_father = combined_father_ids.iter().filter(|id| id.is_none()).count();
    let combined_missing_mother = combined_mother_ids.iter().filter(|id| id.is_none()).count();

    // Update summary with post-merge statistics
    let summary = PopulationSummary {
        total_combined_records,
        combined_missing_father,
        combined_missing_mother,
        records_only_in_bef,
        records_only_in_mfr,
        ..summary_before
    };

    // Create Arrow arrays
    let pnr_array = StringArray::from(combined_pnrs);

    // Convert NaiveDate to i32 (days since epoch) for Date32Array
    let birth_date_i32: Vec<Option<i32>> = combined_birth_dates
        .iter()
        .map(|date| {
            date.map(|d| {
                // Use arrow's temporal conversion to convert date to days since epoch
                
                d
                    .signed_duration_since(chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
                    .num_days() as i32
            })
        })
        .collect();
    let birth_date_array = Date32Array::from(birth_date_i32);

    let father_id_array = StringArray::from(combined_father_ids);
    let mother_id_array = StringArray::from(combined_mother_ids);
    let family_id_array = StringArray::from(combined_family_ids);

    // Create schema
    let schema = Schema::new(vec![
        Field::new("PNR", DataType::Utf8, false),
        Field::new("FOED_DAG", DataType::Date32, true),
        Field::new("FAR_ID", DataType::Utf8, true),
        Field::new("MOR_ID", DataType::Utf8, true),
        Field::new("FAMILIE_ID", DataType::Utf8, true),
    ]);

    // Create RecordBatch
    let combined_batch = RecordBatch::try_new(
        std::sync::Arc::new(schema),
        vec![
            std::sync::Arc::new(pnr_array),
            std::sync::Arc::new(birth_date_array),
            std::sync::Arc::new(father_id_array),
            std::sync::Arc::new(mother_id_array),
            std::sync::Arc::new(family_id_array),
        ],
    )
    .map_err(|e| IdsError::Data(format!("Error creating combined record batch: {e}")))?;

    Ok((combined_batch, summary))
}

/// Process parent data from BEF data
pub fn process_parents(bef_data: &RecordBatch) -> Result<HashMap<Pnr, NaiveDate>> {
    let pnr_col = bef_data
        .column_by_name("PNR")
        .ok_or_else(|| IdsError::Data("Missing PNR column in parent data".to_string()))?;
    let date_col = bef_data
        .column_by_name("FOED_DAG")
        .ok_or_else(|| IdsError::Data("Missing FOED_DAG column in parent data".to_string()))?;

    let pnr_array = pnr_col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;
    let date_array = date_col
        .as_any()
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("FOED_DAG column is not a date array".to_string()))?;

    let mut parent_map = HashMap::new();

    for i in 0..pnr_array.len() {
        if pnr_array.is_null(i) || date_array.is_null(i) {
            continue;
        }

        let pnr_str = pnr_array.value(i);
        if pnr_str.is_empty() {
            continue;
        }

        let pnr = Pnr::from(pnr_str);
        if let Some(date) = date_array.value_as_date(i) {
            // Only insert if PNR not already in map or if this is the first occurrence
            parent_map.entry(pnr).or_insert(date);
        }
    }

    Ok(parent_map)
}

/// Create family data by linking children to their parents' information
pub fn create_family_data(
    children: &RecordBatch,
    parent_data: &HashMap<Pnr, NaiveDate>,
) -> Result<RecordBatch> {
    // Extract data from children batch
    let mut child_population = Vec::new();
    for i in 0..children.num_rows() {
        match Population::from_record_batch(children, i) {
            Ok(person) => child_population.push(person),
            Err(_) => continue, // Skip invalid records
        }
    }

    // Create family data by linking parent information
    let family_data = FamilyData::create_family_data(&child_population, parent_data);

    // Convert to RecordBatch
    FamilyData::to_record_batch(&family_data)
}

/// Count null values in a column
fn count_null_values(batch: &RecordBatch, column_name: &str) -> Result<usize> {
    let col = batch
        .column_by_name(column_name)
        .ok_or_else(|| IdsError::Data(format!("Missing {column_name} column")))?;

    let mut null_count = 0;
    for i in 0..col.len() {
        if col.is_null(i) {
            null_count += 1;
        }
    }

    Ok(null_count)
}

/// Get a date value from a `RecordBatch`
fn get_date_value(
    batch: &RecordBatch,
    column_name: &str,
    row_index: usize,
) -> Result<Option<NaiveDate>> {
    let col = batch
        .column_by_name(column_name)
        .ok_or_else(|| IdsError::Data(format!("Missing {column_name} column")))?;

    let date_array = col
        .as_any()
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data(format!("{column_name} column is not a date array")))?;

    if date_array.is_null(row_index) {
        Ok(None)
    } else {
        Ok(date_array.value_as_date(row_index))
    }
}

/// Get a string value from a `RecordBatch`
fn get_string_value(
    batch: &RecordBatch,
    column_name: &str,
    row_index: usize,
) -> Result<Option<String>> {
    let col = batch
        .column_by_name(column_name)
        .ok_or_else(|| IdsError::Data(format!("Missing {column_name} column")))?;

    let string_array = col
        .as_any()
        .downcast_ref::<StringArray>()
        .ok_or_else(|| IdsError::Data(format!("{column_name} column is not a string array")))?;

    if string_array.is_null(row_index) {
        Ok(None)
    } else {
        let value = string_array.value(row_index);
        if value.is_empty() {
            Ok(None)
        } else {
            Ok(Some(value.to_string()))
        }
    }
}

/// Combine BEF and MFR data to create a population dataset
pub fn generate_population(
    bef_data: &RecordBatch,
    mfr_data: &RecordBatch,
    config: &PopulationConfig,
) -> Result<(RecordBatch, PopulationSummary)> {
    // Process children from both data sources
    let bef_children = extract_bef_children(bef_data, config)?;
    let mfr_children = extract_mfr_children(mfr_data, config)?;

    // Combine children data
    let (combined_children, summary) = combine_children_data(&bef_children, &mfr_children)?;

    // Process parent data
    let parent_data = process_parents(bef_data)?;

    // Create family data
    let family_data = create_family_data(&combined_children, &parent_data)?;

    Ok((family_data, summary))
}

//! Population report generation

use arrow::array::{Array, Date32Array};
use arrow::record_batch::RecordBatch;
use chrono::{Datelike, NaiveDate};
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

use crate::algorithm::population::PopulationSummary;
use crate::error::{IdsError, Result};

/// Save population summary statistics to CSV files
pub fn save_population_summary(
    family_data: &RecordBatch,
    output_dir: &Path,
    summary_before: &PopulationSummary,
    summary_after: &PopulationSummary,
) -> Result<()> {
    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Save summary before merge
    save_summary_before(
        summary_before,
        &output_dir.join("population_summary_before.csv"),
    )?;

    // Save summary after merge
    save_summary_after(
        summary_after,
        &output_dir.join("population_summary_after.csv"),
    )?;

    // Save basic statistics
    save_basic_stats(family_data, &output_dir.join("basic_stats.csv"))?;

    // Save missing data summary
    save_missing_data(family_data, &output_dir.join("missing_data.csv"))?;

    // Save child age distribution data
    save_child_age_distribution(family_data, &output_dir.join("child_age_distribution.csv"))?;

    // Save parent-child age difference data
    save_parent_child_age_diff(family_data, &output_dir.join("parent_child_age_diff.csv"))?;

    Ok(())
}

/// Save summary statistics before data merge
fn save_summary_before(summary: &PopulationSummary, output_file: &Path) -> Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Write headers
    writeln!(writer, "Metric,Value")?;

    // Write metrics
    writeln!(writer, "total_bef_records,{}", summary.total_bef_records)?;
    writeln!(writer, "total_mfr_records,{}", summary.total_mfr_records)?;
    writeln!(writer, "bef_missing_father,{}", summary.bef_missing_father)?;
    writeln!(writer, "bef_missing_mother,{}", summary.bef_missing_mother)?;
    writeln!(writer, "mfr_missing_father,{}", summary.mfr_missing_father)?;
    writeln!(writer, "mfr_missing_mother,{}", summary.mfr_missing_mother)?;

    writer.flush()?;
    Ok(())
}

/// Save summary statistics after data merge
fn save_summary_after(summary: &PopulationSummary, output_file: &Path) -> Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Write headers
    writeln!(writer, "Metric,Value")?;

    // Write metrics
    writeln!(
        writer,
        "total_combined_records,{}",
        summary.total_combined_records
    )?;
    writeln!(
        writer,
        "combined_missing_father,{}",
        summary.combined_missing_father
    )?;
    writeln!(
        writer,
        "combined_missing_mother,{}",
        summary.combined_missing_mother
    )?;
    writeln!(
        writer,
        "records_only_in_bef,{}",
        summary.records_only_in_bef
    )?;
    writeln!(
        writer,
        "records_only_in_mfr,{}",
        summary.records_only_in_mfr
    )?;

    writer.flush()?;
    Ok(())
}

/// Save basic statistics about the family data
fn save_basic_stats(family_data: &RecordBatch, output_file: &Path) -> Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Write headers
    writeln!(writer, "Column,Count,Min,Max,Mean,Standard Deviation")?;

    // Process each column
    for i in 0..family_data.num_columns() {
        let column = family_data.column(i);
        // Store schema in a variable to prevent temporary value dropping
        let schema = family_data.schema();
        let column_name = schema.field(i).name();

        // Skip non-numeric columns for statistics
        if column_name == "PNR"
            || column_name == "FAR_ID"
            || column_name == "MOR_ID"
            || column_name == "FAMILIE_ID"
        {
            writeln!(
                writer,
                "{},{},,,",
                column_name,
                column.len() - column.null_count()
            )?;
            continue;
        }

        // Handle date columns
        if column_name == "FOED_DAG" || column_name == "FAR_FDAG" || column_name == "MOR_FDAG" {
            if let Some(date_array) = column.as_any().downcast_ref::<Date32Array>() {
                let non_null_count = column.len() - column.null_count();

                // Calculate min and max dates
                let mut min_date: Option<NaiveDate> = None;
                let mut max_date: Option<NaiveDate> = None;

                for i in 0..date_array.len() {
                    if !date_array.is_null(i) {
                        let date = date_array.value_as_date(i);
                        if let Some(date) = date {
                            if min_date.is_none() || date < min_date.unwrap() {
                                min_date = Some(date);
                            }
                            if max_date.is_none() || date > max_date.unwrap() {
                                max_date = Some(date);
                            }
                        }
                    }
                }

                let min_str = min_date.map_or(String::new(), |d| d.format("%Y-%m-%d").to_string());
                let max_str = max_date.map_or(String::new(), |d| d.format("%Y-%m-%d").to_string());

                writeln!(
                    writer,
                    "{column_name},{non_null_count},{min_str},{max_str},,,"
                )?;
            }
        }
    }

    writer.flush()?;
    Ok(())
}

/// Save missing data summary
fn save_missing_data(family_data: &RecordBatch, output_file: &Path) -> Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Write headers
    writeln!(writer, "Column,Missing Values,Percentage")?;

    // Process each column
    for i in 0..family_data.num_columns() {
        let column = family_data.column(i);
        // Store schema in a variable to prevent temporary value dropping
        let schema = family_data.schema();
        let column_name = schema.field(i).name();
        let null_count = column.null_count();
        let percentage = (null_count as f64 / column.len() as f64) * 100.0;

        writeln!(writer, "{column_name},{null_count},{percentage:.2}")?;
    }

    writer.flush()?;
    Ok(())
}

/// Save child age distribution data
fn save_child_age_distribution(
    family_data: &RecordBatch,
    output_file: &Path,
) -> Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Get birth date column
    let birth_date_col = family_data
        .column_by_name("FOED_DAG")
        .ok_or_else(|| IdsError::Data("Missing FOED_DAG column".to_string()))?;

    // Store the as_any() result in a variable to avoid the temporary value issue
    let birth_date_any = birth_date_col.as_any();
    let birth_date_array = birth_date_any
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("FOED_DAG column is not a date array".to_string()))?;

    // Write headers
    writeln!(writer, "Birth Date")?;

    // Optimize the processing of large arrays by using a year-based summary instead of individual dates
    // This is much more efficient for large datasets and produces a more useful report
    let mut year_counts: HashMap<i32, usize> = HashMap::with_capacity(50); // Preallocate with reasonable capacity
    let mut null_count = 0;
    let total_rows = birth_date_array.len();
    
    // Process dates in chunks for better performance with large datasets
    const CHUNK_SIZE: usize = 10000;
    for chunk_start in (0..total_rows).step_by(CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + CHUNK_SIZE, total_rows);
        
        for i in chunk_start..chunk_end {
            if birth_date_array.is_null(i) {
                null_count += 1;
                continue;
            }
            
            if let Some(date) = birth_date_array.value_as_date(i) {
                let year = date.year();
                *year_counts.entry(year).or_insert(0) += 1;
            }
        }
        
        // Log progress for large datasets
        if total_rows > 100000 && chunk_start > 0 && chunk_start % (total_rows / 5) < CHUNK_SIZE {
            log::debug!("Processing birth date distribution: {}% complete", 
                       (chunk_start * 100) / total_rows);
        }
    }
    
    // Write year counts instead of individual dates (more efficient and more useful)
    let mut years: Vec<i32> = year_counts.keys().cloned().collect();
    years.sort_unstable();
    
    for year in years {
        let count = year_counts[&year];
        writeln!(writer, "{year},{count}")?;
    }
    
    // Include null count information
    if null_count > 0 {
        writeln!(writer, "NULL,{null_count}")?;
    }

    writer.flush()?;
    Ok(())
}

/// Save parent-child age difference data
fn save_parent_child_age_diff(
    family_data: &RecordBatch,
    output_file: &Path,
) -> Result<()> {
    let file = File::create(output_file)?;
    let mut writer = BufWriter::new(file);

    // Get birth date columns
    let child_date_col = family_data
        .column_by_name("FOED_DAG")
        .ok_or_else(|| IdsError::Data("Missing FOED_DAG column".to_string()))?;
    let father_date_col = family_data
        .column_by_name("FAR_FDAG")
        .ok_or_else(|| IdsError::Data("Missing FAR_FDAG column".to_string()))?;
    let mother_date_col = family_data
        .column_by_name("MOR_FDAG")
        .ok_or_else(|| IdsError::Data("Missing MOR_FDAG column".to_string()))?;

    // Store the as_any() results in variables to avoid temporary value issues
    let child_date_any = child_date_col.as_any();
    let father_date_any = father_date_col.as_any();
    let mother_date_any = mother_date_col.as_any();
    
    let child_date_array = child_date_any
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("FOED_DAG column is not a date array".to_string()))?;
    let father_date_array = father_date_any
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("FAR_FDAG column is not a date array".to_string()))?;
    let mother_date_array = mother_date_any
        .downcast_ref::<Date32Array>()
        .ok_or_else(|| IdsError::Data("MOR_FDAG column is not a date array".to_string()))?;

    // Create histograms for age differences (more efficient and more useful than raw data)
    // Use 5-year bins for better summary statistics - using integers as keys for HashMap
    let mut father_age_diffs: HashMap<i32, usize> = HashMap::new();
    let mut mother_age_diffs: HashMap<i32, usize> = HashMap::new();
    let mut total_father_diffs = 0;
    let mut total_mother_diffs = 0;
    let total_rows = child_date_array.len();
    
    // Process in chunks for better performance with large datasets
    const CHUNK_SIZE: usize = 10000;
    for chunk_start in (0..total_rows).step_by(CHUNK_SIZE) {
        let chunk_end = std::cmp::min(chunk_start + CHUNK_SIZE, total_rows);
        
        for i in chunk_start..chunk_end {
            if child_date_array.is_null(i) {
                continue;
            }
            
            let child_date = child_date_array.value_as_date(i);
            if child_date.is_none() {
                continue;
            }
            
            // Father age difference
            if !father_date_array.is_null(i) {
                let father_date = father_date_array.value_as_date(i);
                if let (Some(child), Some(father)) = (child_date, father_date) {
                    // Calculate age difference in years
                    let diff_days = (child.signed_duration_since(father)).num_days();
                    let diff_years = diff_days as f64 / 365.25;
                    
                    // Use 5-year bins (round to nearest 5) - convert to integer key
                    let bin = (diff_years / 5.0).round() as i32 * 5;
                    *father_age_diffs.entry(bin).or_insert(0) += 1;
                    total_father_diffs += 1;
                }
            }
            
            // Mother age difference
            if !mother_date_array.is_null(i) {
                let mother_date = mother_date_array.value_as_date(i);
                if let (Some(child), Some(mother)) = (child_date, mother_date) {
                    // Calculate age difference in years
                    let diff_days = (child.signed_duration_since(mother)).num_days();
                    let diff_years = diff_days as f64 / 365.25;
                    
                    // Use 5-year bins (round to nearest 5) - convert to integer key
                    let bin = (diff_years / 5.0).round() as i32 * 5;
                    *mother_age_diffs.entry(bin).or_insert(0) += 1;
                    total_mother_diffs += 1;
                }
            }
        }
    }
    
    // Write histogram data
    writeln!(writer, "Age Difference (years),Father Count,Father %,Mother Count,Mother %")?;
    
    // Collect all unique bins
    let mut all_bins: Vec<i32> = father_age_diffs.keys().cloned().collect();
    all_bins.extend(mother_age_diffs.keys().cloned());
    all_bins.sort();
    all_bins.dedup();
    
    for &bin in &all_bins {
        let f_count = father_age_diffs.get(&bin).cloned().unwrap_or(0);
        let m_count = mother_age_diffs.get(&bin).cloned().unwrap_or(0);
        
        let f_percent = if total_father_diffs > 0 {
            (f_count as f64 / total_father_diffs as f64) * 100.0
        } else {
            0.0
        };
        
        let m_percent = if total_mother_diffs > 0 {
            (m_count as f64 / total_mother_diffs as f64) * 100.0
        } else {
            0.0
        };
        
        writeln!(
            writer, 
            "{},{},{:.2},{},{:.2}", 
            bin, f_count, f_percent, m_count, m_percent
        )?;
    }
    
    // Add summary statistics
    writeln!(writer, "\nSummary Statistics")?;
    writeln!(writer, "Metric,Father,Mother")?;
    writeln!(writer, "Total records with data,{},{}",
             total_father_diffs, total_mother_diffs)?;
    writeln!(writer, "Missing values,{},{}",
             total_rows - total_father_diffs, total_rows - total_mother_diffs)?;

    writer.flush()?;
    Ok(())
}

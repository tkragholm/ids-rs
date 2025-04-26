//! CSV Report generation utilities
//!
//! This module provides functions for generating CSV reports from various data structures.

use crate::algorithm::balance::BalanceReport;
use crate::error::{IdsError, Result};
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;

/// Generate a balance report in CSV format
///
/// # Arguments
/// * `report_path` - The path to save the CSV report to
/// * `report` - The balance report to format
///
/// # Returns
/// * `Result<()>` - Success or error
///
/// # Errors
/// Returns an error if file writing fails
pub fn generate_balance_report(report_path: &str, report: &BalanceReport) -> Result<()> {
    // Create the report file
    let file = File::create(report_path).map_err(|e| {
        IdsError::Io(e)
    })?;
    
    let mut writer = BufWriter::new(file);
    
    // Write header
    writeln!(writer, "Variable,Type,Case Mean,Control Mean,Case SD,Control SD,Standardized Difference")
        .map_err(IdsError::Io)?;
    
    // Write metrics sorted by absolute standardized difference (descending)
    let mut sorted_metrics = report.metrics.clone();
    sorted_metrics.sort_by(|a, b| {
        b.standardized_difference.abs().partial_cmp(&a.standardized_difference.abs()).unwrap()
    });
    
    for metric in &sorted_metrics {
        let var_type = if metric.categorical { "Categorical" } else { "Numeric" };
        writeln!(
            writer,
            "{},{},{:.4},{:.4},{:.4},{:.4},{:.4}",
            metric.name,
            var_type,
            metric.case_mean,
            metric.control_mean,
            metric.case_std,
            metric.control_std,
            metric.standardized_difference
        ).map_err(IdsError::Io)?;
    }
    
    // Write summary
    writeln!(writer).map_err(IdsError::Io)?;
    writeln!(writer, "Summary Statistics").map_err(IdsError::Io)?;
    writeln!(writer, "Total Covariates,{}", report.summary.total_covariates).map_err(IdsError::Io)?;
    writeln!(writer, "Imbalanced Covariates (|SMD| > 0.1),{}", report.summary.imbalanced_covariates).map_err(IdsError::Io)?;
    writeln!(writer, "Maximum Absolute Standardized Difference,{:.4}", report.summary.max_standardized_difference).map_err(IdsError::Io)?;
    writeln!(writer, "Mean Absolute Standardized Difference,{:.4}", report.summary.mean_absolute_standardized_difference).map_err(IdsError::Io)?;
    
    Ok(())
}

/// Write a generic CSV report to a file
///
/// # Arguments
/// * `path` - The path to save the CSV report to
/// * `rows` - A slice of string vectors, each representing a row in the CSV
///
/// # Returns
/// * `Result<()>` - Success or error
pub fn write_csv_report(path: &Path, rows: &[Vec<String>]) -> Result<()> {
    let file = File::create(path)
        .map_err(IdsError::Io)?;
    
    let mut writer = BufWriter::new(file);
    
    for row in rows {
        let line = row.join(",");
        writeln!(writer, "{line}")
            .map_err(IdsError::Io)?;
    }
    
    Ok(())
}
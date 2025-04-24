use chrono::NaiveDate;
use crate::error::{IdsError, Result};

/// Extract birth date from a PNR string
/// 
/// Danish PNR format: DDMMYY-XXXX
pub fn extract_birth_date(pnr: &str) -> Result<NaiveDate> {
    if pnr.len() != 11 || (pnr.chars().nth(6) != Some('-')) {
        return Err(IdsError::Validation(format!("Invalid PNR format: {pnr}")));
    }

    let day = &pnr[0..2];
    let month = &pnr[2..4];
    let year_short = &pnr[4..6];
    
    // Determine century
    let control_digit = pnr.chars().nth(7).unwrap().to_digit(10).unwrap();
    let century = match control_digit {
        0..=3 => "19",
        4..=9 => "20",
        _ => "19", // Fallback, should never happen
    };
    
    let year = format!("{century}{year_short}");
    
    // Parse the date
    let date_str = format!("{year}-{month}-{day}");
    match NaiveDate::parse_from_str(&date_str, "%Y-%m-%d") {
        Ok(date) => Ok(date),
        Err(_) => Err(IdsError::Validation(format!("Invalid date in PNR: {pnr}"))),
    }
}

/// Validate a Danish PNR (Personal Identification Number)
#[must_use] pub fn validate_pnr(pnr: &str) -> bool {
    // Basic format check
    if pnr.len() != 11 || (pnr.chars().nth(6) != Some('-')) {
        return false;
    }
    
    // Digit check
    if !pnr.chars().enumerate().all(|(i, c)| {
        if i == 6 {
            c == '-'
        } else {
            c.is_ascii_digit()
        }
    }) {
        return false;
    }
    
    // Valid date check
    extract_birth_date(pnr).is_ok()
}

/// Generate check digit for a Danish PNR
pub fn generate_check_digit(pnr_without_check: &str) -> Result<u8> {
    if pnr_without_check.len() != 10 {
        return Err(IdsError::Validation("PNR without check digit must be 10 characters".to_string()));
    }
    
    // Implementation depends on specific PNR rules
    // This is a placeholder - actual implementation would follow Danish PNR rules
    Ok(0)
}
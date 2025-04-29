//! Diagnosis pattern matching for ICD-10 codes
//!
//! This module provides pattern matching utilities for ICD-10 diagnosis codes,
//! supporting both prefix matching and regex-based pattern matching.

use regex::Regex;
use std::fmt;

/// Represents a normalized diagnosis code with prefix and full code
#[derive(Debug, Clone)]
pub struct NormalizedDiagnosis {
    /// The first 3 characters of a normalized ICD-10 code
    pub prefix: String,
    /// The complete normalized ICD-10 code
    pub full_code: String,
}

/// Represents a pattern used to match diagnosis codes
#[derive(Clone)]
pub struct DiagnosisPattern {
    /// Simple prefix to match (more efficient)
    pub prefix: Option<String>,
    /// Regular expression pattern for more complex matching
    pub regex: Option<Regex>,
    /// Description of what this pattern represents
    pub description: String,
}

impl fmt::Debug for DiagnosisPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DiagnosisPattern")
            .field("prefix", &self.prefix)
            .field("regex", &self.regex.as_ref().map(regex::Regex::as_str))
            .field("description", &self.description)
            .finish()
    }
}

impl DiagnosisPattern {
    /// Create a new pattern with a simple prefix (e.g., "D61")
    #[must_use] pub fn new_prefix(prefix: &str, description: &str) -> Self {
        Self {
            prefix: Some(prefix.to_string()),
            regex: None,
            description: description.to_string(),
        }
    }

    /// Create a new pattern with a regular expression (e.g., "D61[0389]")
    pub fn new_regex(pattern: &str, description: &str) -> Result<Self, regex::Error> {
        Ok(Self {
            prefix: None,
            regex: Some(Regex::new(&format!("^{pattern}"))?),
            description: description.to_string(),
        })
    }

    /// Check if this pattern matches the given diagnosis code
    #[must_use] pub fn matches(&self, diagnosis: &NormalizedDiagnosis) -> bool {
        // Check prefix match first (faster)
        if let Some(prefix) = &self.prefix {
            if &diagnosis.prefix == prefix {
                return true;
            }
        }

        // Fall back to regex match if needed
        if let Some(regex) = &self.regex {
            return regex.is_match(&diagnosis.full_code);
        }

        false
    }
}

/// Normalize an ICD-10 diagnosis code for consistent matching
#[must_use] pub fn normalize_diagnosis_code(code: &str) -> Option<NormalizedDiagnosis> {
    let code = code.trim();
    if code.is_empty() || code.len() < 3 {
        return None;
    }

    // Remove dots if present (e.g., "I10.9" -> "I109")
    let mut clean_code = code.replace('.', "").to_ascii_uppercase();
    
    // Handle Danish-specific ICD-10 prefixes (D-prefixed codes)
    // In Danish healthcare, codes are often prefixed with a D, e.g., "DA10" for "A10"
    if clean_code.len() >= 4 && clean_code.starts_with('D') {
        let second_char = clean_code.chars().nth(1).unwrap_or('X');
        if second_char.is_ascii_alphabetic() {
            // This is likely a Danish D-prefixed code (e.g., DA10)
            // Remove the D prefix for standardization
            clean_code = clean_code[1..].to_string();
        }
    }
    
    // Extract the prefix (first 3 characters)
    let prefix = clean_code.chars().take(3).collect();
    
    Some(NormalizedDiagnosis {
        prefix,
        full_code: clean_code,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_diagnosis_code() {
        // Test standard code
        let result = normalize_diagnosis_code("I10").unwrap();
        assert_eq!(result.prefix, "I10");
        assert_eq!(result.full_code, "I10");
        
        // Test code with dot
        let result = normalize_diagnosis_code("I10.9").unwrap();
        assert_eq!(result.prefix, "I10");
        assert_eq!(result.full_code, "I109");
        
        // Test lowercase code
        let result = normalize_diagnosis_code("i109").unwrap();
        assert_eq!(result.prefix, "I10");
        assert_eq!(result.full_code, "I109");
        
        // Test with whitespace
        let result = normalize_diagnosis_code(" I10.9 ").unwrap();
        assert_eq!(result.prefix, "I10");
        assert_eq!(result.full_code, "I109");
        
        // Test too short code
        assert!(normalize_diagnosis_code("I1").is_none());
        
        // Test empty code
        assert!(normalize_diagnosis_code("").is_none());
    }
    
    #[test]
    fn test_normalize_danish_prefixed_codes() {
        // Test Danish D-prefixed code
        let result = normalize_diagnosis_code("DA10").unwrap();
        assert_eq!(result.prefix, "A10");
        assert_eq!(result.full_code, "A10");
        
        // Test Danish D-prefixed code with subcategory
        let result = normalize_diagnosis_code("DA10.9").unwrap();
        assert_eq!(result.prefix, "A10");
        assert_eq!(result.full_code, "A109");
        
        // Test D-prefixed code in lowercase
        let result = normalize_diagnosis_code("da109").unwrap();
        assert_eq!(result.prefix, "A10");
        assert_eq!(result.full_code, "A109");
        
        // Test with whitespace
        let result = normalize_diagnosis_code(" DA10.9 ").unwrap();
        assert_eq!(result.prefix, "A10");
        assert_eq!(result.full_code, "A109");
        
        // Test D code that is NOT a Danish prefix (e.g., regular D-codes)
        // D50 is a legitimate ICD-10 code for iron deficiency anemia, not a prefix
        let result = normalize_diagnosis_code("D50").unwrap();
        assert_eq!(result.prefix, "D50");
        assert_eq!(result.full_code, "D50");
        
        // Test with dots
        let result = normalize_diagnosis_code("D50.1").unwrap();
        assert_eq!(result.prefix, "D50");
        assert_eq!(result.full_code, "D501");
    }

    #[test]
    fn test_diagnosis_pattern_prefix() {
        let pattern = DiagnosisPattern::new_prefix("I10", "Hypertension");
        
        let code1 = normalize_diagnosis_code("I10").unwrap();
        let code2 = normalize_diagnosis_code("I10.9").unwrap();
        let code3 = normalize_diagnosis_code("I11").unwrap();
        
        assert!(pattern.matches(&code1));
        assert!(pattern.matches(&code2));
        assert!(!pattern.matches(&code3));
    }

    #[test]
    fn test_diagnosis_pattern_regex() {
        let pattern = DiagnosisPattern::new_regex("I1[01]", "Hypertension").unwrap();
        
        let code1 = normalize_diagnosis_code("I10").unwrap();
        let code2 = normalize_diagnosis_code("I11").unwrap();
        let code3 = normalize_diagnosis_code("I12").unwrap();
        
        assert!(pattern.matches(&code1));
        assert!(pattern.matches(&code2));
        assert!(!pattern.matches(&code3));
    }
}
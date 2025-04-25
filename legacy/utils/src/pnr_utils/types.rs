//! Core PNR types and definitions.
//!
//! This module defines the core types used for working with Personal
//! Identification Numbers (PNRs) throughout the codebase.

use chrono::NaiveDate;

/// Represents an individual's information (birth date and PNR)
pub type PersonInfo = (NaiveDate, String);

/// Represents a pair of parents (father, mother)
pub type ParentPair = (PersonInfo, PersonInfo);

/// Represents a family (child and parents)
pub type FamilyInfo = (PersonInfo, ParentPair);

/// The gender of a person, derived from their PNR
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Gender {
    /// Male
    Male,
    /// Female
    Female,
    /// Unknown or unspecified gender
    Unknown,
}

impl Gender {
    /// Create a Gender from a PNR's last digit
    ///
    /// # Arguments
    /// * `last_digit` - The last digit of the PNR
    ///
    /// # Returns
    /// The gender (Male for odd, Female for even)
    #[must_use] pub const fn from_pnr_digit(last_digit: u8) -> Self {
        match last_digit % 2 {
            0 => Self::Female,
            _ => Self::Male,
        }
    }

    /// Get a string representation of the gender
    ///
    /// # Returns
    /// "M" for Male, "F" for Female, "U" for Unknown
    #[must_use] pub const fn to_string(&self) -> &'static str {
        match self {
            Self::Male => "M",
            Self::Female => "F",
            Self::Unknown => "U",
        }
    }
}

impl From<&str> for Gender {
    fn from(s: &str) -> Self {
        match s.trim().to_uppercase().as_str() {
            "M" | "MALE" => Self::Male,
            "F" | "FEMALE" => Self::Female,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gender_from_pnr_digit() {
        assert_eq!(Gender::from_pnr_digit(1), Gender::Male);
        assert_eq!(Gender::from_pnr_digit(2), Gender::Female);
        assert_eq!(Gender::from_pnr_digit(3), Gender::Male);
        assert_eq!(Gender::from_pnr_digit(4), Gender::Female);
    }

    #[test]
    fn test_gender_to_string() {
        assert_eq!(Gender::Male.to_string(), "M");
        assert_eq!(Gender::Female.to_string(), "F");
        assert_eq!(Gender::Unknown.to_string(), "U");
    }

    #[test]
    fn test_gender_from_string() {
        assert_eq!(Gender::from("M"), Gender::Male);
        assert_eq!(Gender::from("F"), Gender::Female);
        assert_eq!(Gender::from("Male"), Gender::Male);
        assert_eq!(Gender::from("Female"), Gender::Female);
        assert_eq!(Gender::from("MALE"), Gender::Male);
        assert_eq!(Gender::from("FEMALE"), Gender::Female);
        assert_eq!(Gender::from("Unknown"), Gender::Unknown);
    }
}

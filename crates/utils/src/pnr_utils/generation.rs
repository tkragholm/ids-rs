//! PNR generation utilities.
//!
//! This module provides utilities for generating synthetic PNRs for testing
//! and development purposes.

use super::types::{FamilyInfo, ParentPair, PersonInfo};
use crate::error::{validation_error, Result};
use chrono::{Duration, NaiveDate};
use hashbrown::HashMap;
use rand::Rng;

/// A pool of generated PNRs for testing and development
#[derive(Debug)]
pub struct PnrPool {
    /// All PNRs in the pool
    pool: HashMap<usize, PersonInfo>,
    /// Only children PNRs
    children: HashMap<usize, PersonInfo>,
    /// Only parent PNRs
    parents: HashMap<usize, PersonInfo>,
}

impl PnrPool {
    /// Create a new PNR pool with the specified number of records
    ///
    /// # Arguments
    /// * `total_records` - The number of records to generate
    /// * `rng` - The random number generator to use
    ///
    /// # Returns
    /// A Result containing the PNR pool or an error
    pub fn new<R: Rng>(total_records: usize, rng: &mut R) -> Result<Self> {
        let mut pool = HashMap::new();
        let mut children = HashMap::new();
        let mut parents = HashMap::new();

        // Define study period constants
        let _study_start = NaiveDate::from_ymd_opt(2000, 1, 1)
            .ok_or_else(|| validation_error("Invalid study start date (2000-01-01)"))?;
        let study_end = NaiveDate::from_ymd_opt(2018, 12, 31)
            .ok_or_else(|| validation_error("Invalid study end date (2018-12-31)"))?;
        let earliest_birth = NaiveDate::from_ymd_opt(1995, 1, 1)
            .ok_or_else(|| validation_error("Invalid earliest birth date (1995-01-01)"))?;
        let latest_birth = study_end;

        let birth_range_days = (latest_birth - earliest_birth).num_days() as i32;

        // Generate children first
        for i in 0..total_records {
            // Generate child's birth date within study period
            let days_offset = rng.random_range(0..=birth_range_days);
            let birth_date = earliest_birth + Duration::days(i64::from(days_offset));

            let sequence = rng.random_range(0..10000);
            let pnr = format!("{}-{:04}", birth_date.format("%d%m%y"), sequence);

            children.insert(i, (birth_date, pnr.clone()));
            pool.insert(i, (birth_date, pnr));

            // Generate parents based on child's birth date
            let mother_age = rng.random_range(20..46); // mothers aged 20-45 at birth
            let father_age = rng.random_range(20..50); // fathers aged 20-49 at birth

            let mother_birth = birth_date - Duration::days(mother_age * 365);
            let father_birth = birth_date - Duration::days(father_age * 365);

            let mother_sequence = rng.random_range(0..10000);
            let father_sequence = rng.random_range(0..10000);

            let mother_pnr = format!("{}-{:04}", mother_birth.format("%d%m%y"), mother_sequence);
            let father_pnr = format!("{}-{:04}", father_birth.format("%d%m%y"), father_sequence);

            parents.insert(i + 1_000_000, (father_birth, father_pnr.clone())); // Father
            parents.insert(i + 2_000_000, (mother_birth, mother_pnr.clone())); // Mother

            pool.insert(i + 1_000_000, (father_birth, father_pnr));
            pool.insert(i + 2_000_000, (mother_birth, mother_pnr));
        }

        Ok(Self {
            pool,
            children,
            parents,
        })
    }

    /// Get a person's information by index
    ///
    /// # Arguments
    /// * `index` - The index of the person
    ///
    /// # Returns
    /// An Option containing the person's information
    #[must_use]
    pub fn get(&self, index: &usize) -> Option<PersonInfo> {
        self.pool.get(index).map(|(date, pnr)| (*date, pnr.clone()))
    }

    /// Get a child's information by index
    ///
    /// # Arguments
    /// * `index` - The index of the child
    ///
    /// # Returns
    /// An Option containing the child's information
    #[must_use]
    pub fn get_child(&self, index: &usize) -> Option<PersonInfo> {
        self.children
            .get(index)
            .map(|(date, pnr)| (*date, pnr.clone()))
    }

    /// Get a child's parents by index
    ///
    /// # Arguments
    /// * `index` - The index of the child
    ///
    /// # Returns
    /// An Option containing the child's parents' information
    #[must_use]
    pub fn get_parents(&self, index: &usize) -> Option<ParentPair> {
        let father = self.parents.get(&(index + 1_000_000))?;
        let mother = self.parents.get(&(index + 2_000_000))?;
        Some(((father.0, father.1.clone()), (mother.0, mother.1.clone())))
    }

    /// Get a child's family (child and parents) by index
    ///
    /// # Arguments
    /// * `index` - The index of the child
    ///
    /// # Returns
    /// An Option containing the child's family information
    #[must_use]
    pub fn get_family(&self, index: &usize) -> Option<FamilyInfo> {
        let child = self.get_child(index)?;
        let parents = self.get_parents(index)?;
        Some((child, parents))
    }

    /// Get the total number of records in the pool
    ///
    /// # Returns
    /// The number of records in the pool
    #[must_use]
    pub fn len(&self) -> usize {
        self.pool.len()
    }

    /// Check if the pool is empty
    ///
    /// # Returns
    /// True if the pool is empty, false otherwise
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.pool.is_empty()
    }

    /// Get the number of children in the pool
    ///
    /// # Returns
    /// The number of children in the pool
    #[must_use]
    pub fn num_children(&self) -> usize {
        self.children.len()
    }

    /// Get the number of parents in the pool
    ///
    /// # Returns
    /// The number of parents in the pool
    #[must_use]
    pub fn num_parents(&self) -> usize {
        self.parents.len()
    }
}

/// Generate a PNR from a date and sequence number
///
/// # Arguments
/// * `date` - The birth date
/// * `sequence` - The sequence number (0-9999)
///
/// # Returns
/// A PNR string in the format DD-MM-YY-XXXX
#[must_use] pub fn generate_pnr(date: &NaiveDate, sequence: u16) -> String {
    format!("{}-{:04}", date.format("%d%m%y"), sequence % 10000)
}

/// Generate a pool of PNRs for testing
///
/// # Arguments
/// * `size` - The number of PNRs to generate
/// * `start_year` - The start year for birth dates
/// * `end_year` - The end year for birth dates
///
/// # Returns
/// A vector of PNRs
#[must_use] pub fn generate_test_pnrs(size: usize, start_year: i32, end_year: i32) -> Vec<String> {
    let mut rng = rand::rng();
    let mut pnrs = Vec::with_capacity(size);

    let start_date = NaiveDate::from_ymd_opt(start_year, 1, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(2000, 1, 1).unwrap());

    let end_date = NaiveDate::from_ymd_opt(end_year, 12, 31)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(2020, 12, 31).unwrap());

    let days = (end_date - start_date).num_days() as i32;

    for _ in 0..size {
        let offset = rng.random_range(0..=days);
        let date = start_date + Duration::days(i64::from(offset));
        let sequence = rng.random_range(0..10000);

        pnrs.push(generate_pnr(&date, sequence));
    }

    pnrs
}

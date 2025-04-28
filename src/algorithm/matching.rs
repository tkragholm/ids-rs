use crate::error::{IdsError, Result};
use crate::model::pnr::Pnr;
use chrono::{Duration, NaiveDate};
use rand::seq::IndexedRandom;

/// Criteria for matching cases to controls
#[derive(Debug, Clone)]
pub struct MatchingCriteria {
    /// Maximum difference in days between birth dates
    pub birth_date_window_days: i32,

    /// Maximum difference in days between parent birth dates
    pub parent_birth_date_window_days: i32,

    /// Whether both parents are required
    pub require_both_parents: bool,

    /// Whether the same gender is required
    pub require_same_gender: bool,
}

impl Default for MatchingCriteria {
    fn default() -> Self {
        Self {
            birth_date_window_days: 30,
            parent_birth_date_window_days: 365,
            require_both_parents: false,
            require_same_gender: true,
        }
    }
}

/// Pair of matched case and control
#[derive(Debug, Clone)]
pub struct MatchedPair {
    /// Case PNR
    pub case_pnr: Pnr,

    /// Case birth date
    pub case_birth_date: NaiveDate,

    /// Control PNR
    pub control_pnr: Pnr,

    /// Control birth date
    pub control_birth_date: NaiveDate,

    /// Date when the match was made
    pub match_date: NaiveDate,
}

/// Matcher for pairing cases with controls
pub struct Matcher {
    /// Matching criteria
    criteria: MatchingCriteria,
}

impl Matcher {
    /// Create a new matcher with the given criteria
    #[must_use]
    pub fn new(criteria: MatchingCriteria) -> Self {
        Self { criteria }
    }

    /// Match cases to controls
    pub fn match_cases_to_controls(
        &self,
        cases: Vec<(Pnr, NaiveDate)>,
        controls: Vec<(Pnr, NaiveDate)>,
        match_date: NaiveDate,
    ) -> Result<Vec<MatchedPair>> {
        let mut matches = Vec::new();
        let mut available_controls = controls.clone();

        for (case_pnr, case_birth_date) in cases {
            // Find eligible controls
            let eligible_indices =
                self.find_eligible_controls(&case_pnr, case_birth_date, &available_controls)?;

            if eligible_indices.is_empty() {
                return Err(IdsError::Validation(format!(
                    "No eligible controls found for case {}",
                    case_pnr.value()
                )));
            }

            // Select a random control
            let mut rng = rand::rng();
            let selected_idx = *eligible_indices.choose(&mut rng).unwrap();
            let (control_pnr, control_birth_date) = available_controls.remove(selected_idx);

            // Create matched pair
            matches.push(MatchedPair {
                case_pnr,
                case_birth_date,
                control_pnr,
                control_birth_date,
                match_date,
            });
        }

        Ok(matches)
    }

    /// Find eligible controls for a case
    fn find_eligible_controls(
        &self,
        case_pnr: &Pnr,
        case_birth_date: NaiveDate,
        controls: &[(Pnr, NaiveDate)],
    ) -> Result<Vec<usize>> {
        let mut eligible_indices = Vec::new();
        let _window = Duration::days(self.criteria.birth_date_window_days);

        for (idx, (control_pnr, control_birth_date)) in controls.iter().enumerate() {
            // Skip if case and control are the same person
            if case_pnr.value() == control_pnr.value() {
                continue;
            }

            // Check birth date window
            let diff = (*control_birth_date - case_birth_date).num_days().abs();
            if diff > self.criteria.birth_date_window_days {
                continue;
            }

            // Additional criteria checks would go here
            // (gender, parents, etc. - simplified for this example)

            eligible_indices.push(idx);
        }

        Ok(eligible_indices)
    }
}

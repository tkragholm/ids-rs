use crate::error::{IdsError, Result};
use crate::model::pnr::Pnr;
use arrow::array::{Array, ArrayRef, BooleanArray, StringArray};
use arrow::compute;
use arrow::record_batch::RecordBatch;
use chrono::{Datelike, Duration, NaiveDate};
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use rand::rng;
use rand::seq::{IndexedRandom, SliceRandom};
use rustc_hash::{FxHashMap, FxHashSet};
use smallvec::SmallVec;
use std::time::Instant;

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
    // Constants for optimization
    const BATCH_SIZE: usize = 1024;

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
            let mut rng = rng();
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

    /// Build optimized birth date index for faster matching
    fn build_birth_date_index(
        &self,
        controls: &[(Pnr, NaiveDate)],
    ) -> FxHashMap<i32, SmallVec<[usize; 16]>> {
        let mut index = FxHashMap::default();
        let window_days = self.criteria.birth_date_window_days;

        for (idx, (_, birth_date)) in controls.iter().enumerate() {
            // Create buckets of birth dates based on days from CE
            let days_from_ce = birth_date.num_days_from_ce();
            let bucket = days_from_ce / window_days;

            // Store the control index in the appropriate bucket
            index
                .entry(bucket as i32)
                .or_insert_with(SmallVec::<[usize; 16]>::new)
                .push(idx);
        }

        index
    }

    /// Find eligible controls for a case
    fn find_eligible_controls(
        &self,
        case_pnr: &Pnr,
        case_birth_date: NaiveDate,
        controls: &[(Pnr, NaiveDate)],
    ) -> Result<Vec<usize>> {
        let mut eligible_indices = Vec::new();
        let _window = Duration::days(i64::from(self.criteria.birth_date_window_days));

        for (idx, (control_pnr, control_birth_date)) in controls.iter().enumerate() {
            // Skip if case and control are the same person
            if case_pnr.value() == control_pnr.value() {
                continue;
            }

            // Check birth date window
            let diff = (*control_birth_date - case_birth_date).num_days().abs() as i32;
            if diff > self.criteria.birth_date_window_days {
                continue;
            }

            // Additional criteria checks would go here
            // (gender, parents, etc. - simplified for this example)

            eligible_indices.push(idx);
        }

        Ok(eligible_indices)
    }

    /// Perform optimized matching between cases and controls
    pub fn perform_matching(
        &self,
        cases: &RecordBatch,
        controls: &RecordBatch,
        matching_ratio: usize,
    ) -> Result<(RecordBatch, RecordBatch)> {
        let start_time = Instant::now();

        // Extract PNR and birth date pairs
        let case_pairs = self.extract_pnr_and_birth_date(cases)?;
        let control_pairs = self.extract_pnr_and_birth_date(controls)?;

        // Create lookup maps for PNR to row index
        let case_pnr_to_idx = self.build_pnr_index(cases)?;
        let control_pnr_to_idx = self.build_pnr_index(controls)?;

        info!(
            "Matching {} cases with control pool of {} candidates",
            case_pairs.len(),
            control_pairs.len()
        );

        // Precompute birth date index for efficient lookup
        let birth_date_index = self.build_birth_date_index(&control_pairs);

        // Track which controls have been used
        let mut used_control_indices = FxHashSet::default();

        // Prepare result containers with appropriate capacity
        let mut matched_case_indices = Vec::with_capacity(case_pairs.len());
        let mut matched_control_indices = Vec::with_capacity(case_pairs.len() * matching_ratio);

        // Set up progress bar
        let total_cases = case_pairs.len();
        let pb = ProgressBar::new(total_cases as u64);
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len} cases ({per_sec}) {msg}")
                .unwrap()
                .progress_chars("#>-"),
        );

        // Process in reasonable-sized batches for better progress reporting
        for (case_idx, (case_pnr, case_birth_date)) in case_pairs.iter().enumerate() {
            // Find eligible controls using efficient birth date indexing
            let days_from_ce = case_birth_date.num_days_from_ce();
            let window_days = self.criteria.birth_date_window_days;
            let bucket = days_from_ce / window_days;

            // Use SmallVec to avoid heap allocations for small sets of eligible controls
            let mut eligible_control_indices = SmallVec::<[usize; 32]>::new();

            // Check current bucket and adjacent buckets for potential matches
            for b in [bucket - 1, bucket, bucket + 1].iter() {
                if let Some(indices) = birth_date_index.get(&(*b as i32)) {
                    for &ctrl_idx in indices {
                        // Skip already used controls
                        if used_control_indices.contains(&ctrl_idx) {
                            continue;
                        }

                        let (control_pnr, control_birth_date) = &control_pairs[ctrl_idx];

                        // Skip if case and control are the same person
                        if case_pnr.value() == control_pnr.value() {
                            continue;
                        }

                        // Check if birth dates are within the allowed window
                        let diff = (*control_birth_date - *case_birth_date).num_days().abs() as i32;
                        if diff <= self.criteria.birth_date_window_days {
                            eligible_control_indices.push(ctrl_idx);
                        }
                    }
                }
            }

            // Select up to matching_ratio controls randomly
            let num_to_select = std::cmp::min(matching_ratio, eligible_control_indices.len());
            if num_to_select > 0 {
                // Get the case index in the record batch
                if let Some(&case_batch_idx) = case_pnr_to_idx.get(case_pnr.value()) {
                    matched_case_indices.push(case_batch_idx);

                    // Select controls randomly using thread_rng for better randomness
                    let mut rng = rng();
                    let mut indices_vec: Vec<usize> =
                        eligible_control_indices.into_iter().collect();
                    indices_vec.partial_shuffle(&mut rng, num_to_select);

                    // Add selected controls to results
                    for i in 0..num_to_select {
                        let ctrl_idx = indices_vec[i];
                        let (control_pnr, _) = &control_pairs[ctrl_idx];

                        if let Some(&control_batch_idx) =
                            control_pnr_to_idx.get(control_pnr.value())
                        {
                            matched_control_indices.push(control_batch_idx);
                        }

                        // Mark control as used
                        used_control_indices.insert(ctrl_idx);
                    }
                }
            }

            // Update progress
            pb.inc(1);
            if case_idx % 100 == 0 {
                pb.set_message(format!("Found {} matches", matched_case_indices.len()));
            }
        }

        pb.finish_with_message("Matching complete");

        if matched_case_indices.is_empty() {
            return Err(IdsError::Validation(
                "No matches found for any cases".to_string(),
            ));
        }

        // Create filtered RecordBatches using batch filtering
        let case_batch = self.filter_batch_by_indices(cases, &matched_case_indices)?;
        let control_batch = self.filter_batch_by_indices(controls, &matched_control_indices)?;

        let elapsed = start_time.elapsed();
        info!(
            "Matching complete: {} cases matched with {} controls in {:.2?} ({:.2} cases/sec)",
            case_batch.num_rows(),
            control_batch.num_rows(),
            elapsed,
            case_pairs.len() as f64 / elapsed.as_secs_f64()
        );

        Ok((case_batch, control_batch))
    }

    /// Extract PNR and birth date pairs from a RecordBatch
    fn extract_pnr_and_birth_date(&self, batch: &RecordBatch) -> Result<Vec<(Pnr, NaiveDate)>> {
        let pnr_idx = batch
            .schema()
            .index_of("PNR")
            .map_err(|e| IdsError::Data(format!("PNR column not found: {e}")))?;

        let birth_date_idx = batch
            .schema()
            .index_of("FOED_DAG")
            .map_err(|e| IdsError::Data(format!("FOED_DAG column not found: {e}")))?;

        let pnr_col = batch.column(pnr_idx);
        let birth_date_col = batch.column(birth_date_idx);

        let pnr_array = pnr_col
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;

        let mut pairs = Vec::with_capacity(batch.num_rows());

        for i in 0..batch.num_rows() {
            if pnr_array.is_null(i) {
                continue;
            }

            let pnr_str = pnr_array.value(i);
            let pnr = Pnr::from(pnr_str);

            if let Some(date) =
                crate::utils::date_utils::extract_date_from_array(birth_date_col.as_ref(), i)
            {
                pairs.push((pnr, date));
            }
        }

        Ok(pairs)
    }

    /// Build a map from PNR to row index for fast lookups
    fn build_pnr_index(&self, batch: &RecordBatch) -> Result<FxHashMap<String, usize>> {
        let pnr_idx = batch
            .schema()
            .index_of("PNR")
            .map_err(|e| IdsError::Data(format!("PNR column not found: {e}")))?;

        let pnr_col = batch.column(pnr_idx);
        let pnr_array = pnr_col
            .as_any()
            .downcast_ref::<StringArray>()
            .ok_or_else(|| IdsError::Data("PNR column is not a string array".to_string()))?;

        let mut pnr_to_idx = FxHashMap::default();
        pnr_to_idx.reserve(batch.num_rows());

        for i in 0..pnr_array.len() {
            if !pnr_array.is_null(i) {
                pnr_to_idx.insert(pnr_array.value(i).to_string(), i);
            }
        }

        Ok(pnr_to_idx)
    }

    /// Filter a RecordBatch by row indices
    fn filter_batch_by_indices(
        &self,
        batch: &RecordBatch,
        indices: &[usize],
    ) -> Result<RecordBatch> {
        // Create a boolean mask for the selected rows
        let mut mask = vec![false; batch.num_rows()];
        for &idx in indices {
            mask[idx] = true;
        }

        let bool_array = BooleanArray::from(mask);

        // Apply the mask to all columns
        let filtered_columns: Result<Vec<ArrayRef>> = batch
            .columns()
            .iter()
            .map(|col| {
                compute::filter(col, &bool_array)
                    .map_err(|e| IdsError::Data(format!("Failed to filter column: {e}")))
            })
            .collect();

        // Create the filtered RecordBatch
        RecordBatch::try_new(batch.schema(), filtered_columns?)
            .map_err(|e| IdsError::Data(format!("Failed to create filtered batch: {e}")))
    }
}

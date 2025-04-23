use super::BalanceChecker;
use crate::balance::results::BalanceResults;
use crate::models::MatchedPairDetail;
use chrono::NaiveDate;
use hashbrown::HashMap;
use log::debug;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::sync::Arc;
use types::error::IdsError;
use types::models::{Covariate, CovariateType};

impl BalanceChecker {
    /// Add matched pair details to the results
    pub(crate) fn add_matched_pair_details(
        &self,
        results: &mut BalanceResults,
        cases: &[(String, NaiveDate)],
        controls: &[(String, NaiveDate)],
    ) -> Result<(), IdsError> {
        // Group cases by date for better batch processing
        let mut cases_by_date: HashMap<NaiveDate, Vec<&str>> = HashMap::new();
        for (case_pnr, case_date) in cases {
            cases_by_date.entry(*case_date).or_default().push(case_pnr);
        }

        // Do the same for controls
        let mut controls_by_date: HashMap<NaiveDate, Vec<&str>> = HashMap::new();
        for (control_pnr, control_date) in controls {
            controls_by_date
                .entry(*control_date)
                .or_default()
                .push(control_pnr);
        }

        // Determine optimal chunk size based on number of pairs
        let total_pairs: usize = cases_by_date
            .iter()
            .map(|(date, case_pnrs)| {
                let control_count = controls_by_date.get(date).map_or(0, |c| c.len());
                case_pnrs.len() * control_count
            })
            .sum();

        let num_threads = rayon::current_num_threads();
        let chunk_size = (total_pairs / num_threads).clamp(100, 5000);

        debug!(
            "Processing {} matched pairs for {} cases and {} controls using chunk size {}",
            total_pairs,
            cases.len(),
            controls.len(),
            chunk_size
        );

        // Use a thread-safe container for collecting results
        let pair_details = Arc::new(Mutex::new(Vec::with_capacity(total_pairs * 4)));

        // Define the variables we'll use for prefetching - include Occupation
        let covariate_types = [
            CovariateType::Demographics,
            CovariateType::Income,
            CovariateType::Education,
            CovariateType::Occupation,
        ];

        // Convert HashMaps to Vecs for parallel processing
        let date_groups: Vec<(NaiveDate, Vec<&str>)> = cases_by_date.into_iter().collect();

        // Process each date group in parallel using rayon
        date_groups.par_iter().for_each(|(date, case_pnrs)| {
            // Get matching controls for this date
            let control_pnrs = match controls_by_date.get(date) {
                Some(pnrs) => pnrs,
                None => return, // No controls for this date
            };

            // For large enough groups, prefetch all the data we'll need
            if case_pnrs.len() * control_pnrs.len() > 100 {
                // Collect all PNRs for prefetching (both cases and controls)
                let mut all_pnrs = Vec::with_capacity(case_pnrs.len() + control_pnrs.len());
                all_pnrs.extend(case_pnrs.iter().map(|p| p.to_string()));
                all_pnrs.extend(control_pnrs.iter().map(|p| p.to_string()));

                // Prefetch all data for this date group
                self.prefetch_data(&all_pnrs, &covariate_types, &[*date]);
            }

            // Process each case-control pair
            for &case_pnr in case_pnrs {
                for &control_pnr in control_pnrs {
                    let mut batch_details = Vec::new();

                    // --- DEMOGRAPHICS ---

                    // Original variables

                    // Family Size
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Demographics,
                        "Family Size",
                        |cov| cov.family_size().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // Municipality
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Demographics,
                        "Municipality",
                        |cov| cov.municipality().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // New demographics variables

                    // Age
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Demographics,
                        "Age",
                        |cov| cov.age().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // Children Count
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Demographics,
                        "Children Count",
                        |cov| cov.children_count().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // --- INCOME ---

                    // Original income variable
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Income,
                        "Income",
                        |cov| cov.income_amount(),
                    ) {
                        batch_details.push(detail);
                    }

                    // New income variables

                    // Wage Income
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Income,
                        "Wage Income",
                        |cov| cov.wage_income(),
                    ) {
                        batch_details.push(detail);
                    }

                    // Employment Status
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Income,
                        "Employment Status",
                        |cov| cov.employment_status().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // --- EDUCATION ---

                    // Education Level - treated as a numeric value
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Education,
                        "Education Level",
                        |cov| {
                            cov.education_level()
                                .and_then(|level| level.parse::<f64>().ok())
                        },
                    ) {
                        batch_details.push(detail);
                    }

                    // ISCED Level - convert from string code to numeric value for comparison
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Education,
                        "ISCED Level",
                        |cov| {
                            cov.isced_code().and_then(|code| {
                                // Extract the first character which should be the ISCED level
                                if !code.is_empty() {
                                    code[0..1].parse::<f64>().ok()
                                } else {
                                    None
                                }
                            })
                        },
                    ) {
                        batch_details.push(detail);
                    }

                    // Education Years - already a numeric value
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Education,
                        "Education Years",
                        |cov| cov.education_years().map(|y| y as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // --- OCCUPATION ---

                    // Original occupation variables

                    // SOCIO13 Occupation Code - convert directly to numeric
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "SOCIO13 Value",
                        |cov| {
                            cov.occupation_code()
                                .and_then(|code| code.parse::<f64>().ok())
                        },
                    ) {
                        batch_details.push(detail);
                    }

                    // Classification System - treat as categorical but convert to numeric
                    // This is retained for compatibility with any non-SOCIO13 classification systems
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "Classification System",
                        |cov| {
                            cov.classification().map(|class| {
                                // Simple hash to create a numeric value for comparison
                                let mut hash = 0.0;
                                for (i, c) in class.chars().enumerate() {
                                    hash += (c as u32 as f64) * (i + 1) as f64;
                                }
                                hash
                            })
                        },
                    ) {
                        batch_details.push(detail);
                    }

                    // New occupation variables

                    // SOCIO
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "SOCIO",
                        |cov| cov.socio().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // SOCIO02
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "SOCIO02",
                        |cov| cov.socio02().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // PRE_SOCIO
                    if let Ok(Some(detail)) = self.process_matched_pair(
                        case_pnr,
                        control_pnr,
                        *date,
                        CovariateType::Occupation,
                        "Previous Socioeconomic Status",
                        |cov| cov.pre_socio().map(|val| val as f64),
                    ) {
                        batch_details.push(detail);
                    }

                    // Add all details at once to minimize lock contention
                    if !batch_details.is_empty() {
                        let mut details = pair_details.lock();
                        details.extend(batch_details);
                    }
                }
            }
        });

        // Add all collected pair details to the results
        let collected_details = match Arc::try_unwrap(pair_details) {
            Ok(mutex) => mutex.into_inner(),
            Err(arc) => {
                let guard = arc.lock();
                guard.clone()
            }
        };

        log::debug!("Collected {} matched pair details", collected_details.len());

        for detail in collected_details {
            results.add_pair_detail(detail);
        }

        Ok(())
    }

    /// Process a single matched pair for a specific variable
    fn process_matched_pair(
        &self,
        case_pnr: &str,
        control_pnr: &str,
        date: NaiveDate,
        covariate_type: CovariateType,
        variable_name: &str,
        value_extractor: impl Fn(&Covariate) -> Option<f64>,
    ) -> Result<Option<MatchedPairDetail>, IdsError> {
        let case_value = self
            .covariate(case_pnr, covariate_type, date)?
            .as_ref()
            .and_then(&value_extractor);

        let control_value = self
            .covariate(control_pnr, covariate_type, date)?
            .as_ref()
            .and_then(&value_extractor);

        match (case_value, control_value) {
            (Some(case_val), Some(ctrl_val)) => Ok(Some(MatchedPairDetail::new(
                case_pnr.to_string(),
                vec![control_pnr.to_string()],
                date,
                variable_name.to_string(),
                case_val,
                ctrl_val,
                MatchedPairDetail::calculate_std_diff(case_val, ctrl_val),
            ))),
            _ => Ok(None),
        }
    }
}

use crate::models::{CovariateSummary, MatchedPairDetail};
use chrono::NaiveDate;
use hashbrown::HashMap;

#[derive(Clone)]
pub struct BalanceResults {
    pub summaries: Vec<CovariateSummary>,
    pub missing_data_rates: HashMap<String, (f64, f64)>,
    pub matched_pair_details: Vec<MatchedPairDetail>,
}

#[derive(Clone)]
pub struct MatchedPairSummary {
    pub case_pnr: String,
    pub control_pnrs: Vec<String>,
    pub treatment_date: NaiveDate,
    pub summaries: Vec<CovariateSummary>,
    pub missing_rates: HashMap<String, (f64, f64)>,
}

impl Default for BalanceResults {
    fn default() -> Self {
        Self::new()
    }
}

impl BalanceResults {
    #[must_use] pub fn new() -> Self {
        Self {
            summaries: Vec::new(),
            missing_data_rates: HashMap::new(),
            matched_pair_details: Vec::new(),
        }
    }

    pub fn add_summary(&mut self, summary: CovariateSummary) {
        self.summaries.push(summary);
    }

    pub fn add_missing_rate(&mut self, variable: String, case_rate: f64, control_rate: f64) {
        self.missing_data_rates
            .insert(variable, (case_rate, control_rate));
    }

    pub fn add_pair_detail(&mut self, detail: MatchedPairDetail) {
        self.matched_pair_details.push(detail);
    }

    /// Combine another `BalanceResults` into this one
    pub fn combine(&mut self, other: BalanceResults) {
        // Add all summaries
        self.summaries.extend(other.summaries);

        // Merge missing data rates
        for (variable, rates) in other.missing_data_rates {
            self.missing_data_rates.insert(variable, rates);
        }

        // Add all matched pair details
        self.matched_pair_details.extend(other.matched_pair_details);
    }
}

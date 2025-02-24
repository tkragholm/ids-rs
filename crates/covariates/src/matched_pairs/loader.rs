use super::{MatchedPairRecord, MatchedPairsResult};
use std::collections::HashMap;
use std::path::Path;
use types::error::IdsError;

pub fn load_matched_pairs(path: &Path) -> Result<MatchedPairsResult, IdsError> {
    let mut reader = csv::Reader::from_path(path).map_err(IdsError::from)?;
    let mut pairs: HashMap<(String, chrono::NaiveDate), Vec<String>> = HashMap::new();

    for result in reader.deserialize() {
        let record: MatchedPairRecord = result.map_err(IdsError::from)?;

        pairs
            .entry((record.case_pnr.clone(), record.case_treatment_date))
            .or_default()
            .push(record.control_pnr.clone());
    }

    Ok(pairs
        .into_iter()
        .map(|((case_pnr, date), controls)| (case_pnr, date, controls))
        .collect())
}

pub mod loader;
pub mod record;

pub use loader::load_matched_pairs;
pub use record::{CaseWithControls, Control, MatchedPairRecord};

pub type MatchedPairsResult = Vec<(String, chrono::NaiveDate, Vec<String>)>;

/// Load matched pair records from a CSV file
/// 
/// This is a wrapper around the csv reader to load matched pair records
/// 
/// # Arguments
/// * `path` - The path to the CSV file
/// 
/// # Returns
/// * `Result<Vec<MatchedPairRecord>, Box<dyn std::error::Error>>` - A list of matched pair records
pub fn load_matched_pair_records(path: &std::path::Path) -> Result<Vec<MatchedPairRecord>, Box<dyn std::error::Error>> {
    use std::fs::File;
    use csv::Reader;
    
    let file = File::open(path)?;
    let mut reader = Reader::from_reader(file);
    
    let records: Result<Vec<MatchedPairRecord>, _> = reader.deserialize().collect();
    Ok(records?)
}

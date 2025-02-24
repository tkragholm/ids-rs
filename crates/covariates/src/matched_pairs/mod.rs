pub mod loader;
pub mod record;

pub use loader::load_matched_pairs;
pub use record::MatchedPairRecord;

pub type MatchedPairsResult = Vec<(String, chrono::NaiveDate, Vec<String>)>;

use chrono::{Datelike, NaiveDate};

/// Helper for date operations
pub trait DateHelpers: Datelike {
    /// Get the quarter (1-4) for this date
    fn get_quarter(&self) -> u32 {
        ((self.month() - 1) / 3) + 1
    }
}

impl DateHelpers for NaiveDate {}
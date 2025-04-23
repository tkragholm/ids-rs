// Placeholder file - plotting functionality has been removed
use std::fmt::Debug;

// Empty trait and implementation for compatibility
#[derive(Debug)]
pub struct DefaultPlotter;

impl DefaultPlotter {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for DefaultPlotter {
    fn default() -> Self {
        Self::new()
    }
}

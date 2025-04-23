use covariates::balance::BalanceChecker;
use types::storage::arrow::backend::ArrowBackend;

/// Extension trait for BalanceChecker to provide diagnostic capabilities
pub trait BalanceCheckerDiagnostic {
    /// Create a new checker with an empty store for diagnostic mode
    fn new_diagnostic() -> BalanceChecker;
}

impl BalanceCheckerDiagnostic for BalanceChecker {
    /// Create a new checker with an empty store for diagnostic mode
    fn new_diagnostic() -> Self {
        // Create an empty store for diagnostic/fallback mode
        let empty_store = ArrowBackend::new_empty();

        // Create a checker with an empty store
        // This will have limited functionality, but allows the UI to continue
        BalanceChecker::new(empty_store)
    }
}

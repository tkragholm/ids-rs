use datafusion::prelude::*;
use std::collections::HashSet;
use super::predicates::*;

/// Builder for creating complex filter expressions
pub struct FilterBuilder {
    filters: Vec<Expr>,
}

impl FilterBuilder {
    /// Create a new filter builder
    #[must_use]
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// Add a date range filter
    #[must_use]
    pub fn with_date_range(mut self, column: &str, start_date: &str, end_date: &str) -> Self {
        self.filters.push(date_range_filter(column, start_date, end_date));
        self
    }

    /// Add a categorical filter
    #[must_use]
    pub fn with_categorical(mut self, column: &str, values: &[&str]) -> Self {
        self.filters.push(categorical_filter(column, values));
        self
    }

    /// Add a numeric range filter
    #[must_use]
    pub fn with_numeric_range(mut self, column: &str, min: f64, max: f64) -> Self {
        self.filters.push(numeric_range_filter(column, min, max));
        self
    }

    /// Add a non-null filter
    #[must_use]
    pub fn with_non_null(mut self, columns: &[&str]) -> Self {
        for column in columns {
            self.filters.push(col(*column).is_not_null());
        }
        self
    }

    /// Add a prefix filter
    #[must_use]
    pub fn with_prefix(mut self, column: &str, prefix: &str) -> Self {
        self.filters.push(prefix_filter(column, prefix));
        self
    }

    /// Add a contains filter
    #[must_use]
    pub fn with_contains(mut self, column: &str, pattern: &str) -> Self {
        self.filters.push(contains_filter(column, pattern));
        self
    }

    /// Add a custom filter expression
    #[must_use]
    pub fn with_expr(mut self, expr: Expr) -> Self {
        self.filters.push(expr);
        self
    }

    /// Add a hash set filter
    #[must_use]
    pub fn with_hash_set(mut self, column: &str, values: HashSet<String>) -> Self {
        if !values.is_empty() {
            self.filters.push(hash_set_filter(column, values));
        }
        self
    }

    /// Build the final filter expression combining all filters with AND
    #[must_use]
    pub fn build_and(self) -> Expr {
        and_filters(self.filters)
    }

    /// Build the final filter expression combining all filters with OR
    #[must_use]
    pub fn build_or(self) -> Expr {
        or_filters(self.filters)
    }

    /// Apply to a DataFrame (combines all filters with AND)
    pub fn apply_to_dataframe(self, df: DataFrame) -> Result<DataFrame, datafusion::error::DataFusionError> {
        if self.filters.is_empty() {
            return Ok(df);
        }
        df.filter(self.build_and())
    }
}
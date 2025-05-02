use crate::error::Result;
use datafusion::prelude::*;
use std::collections::HashSet;

/// A structure for PNR filtering
pub struct PnrFilter {
    pnrs: HashSet<String>,
    direct_filter: bool,
    relation_column: Option<String>,
    pnr_column: String,
}

impl PnrFilter {
    /// Create a new PNR filter
    #[must_use] pub fn new(pnrs: HashSet<String>) -> Self {
        Self {
            pnrs,
            direct_filter: true,
            relation_column: None,
            pnr_column: "PNR".to_string(),
        }
    }

    /// Create a new PNR filter with a specific column name
    #[must_use] pub fn with_column(pnrs: HashSet<String>, column_name: &str) -> Self {
        Self {
            pnrs,
            direct_filter: true,
            relation_column: None,
            pnr_column: column_name.to_string(),
        }
    }

    /// Create a PNR filter with relation
    #[must_use] pub fn with_relation(pnrs: HashSet<String>, relation_column: &str) -> Self {
        Self {
            pnrs,
            direct_filter: false,
            relation_column: Some(relation_column.to_string()),
            pnr_column: "PNR".to_string(),
        }
    }

    /// Get the PNRs in this filter
    #[must_use] pub const fn pnrs(&self) -> &HashSet<String> {
        &self.pnrs
    }

    /// Get the relation column if any
    #[must_use] pub fn relation_column(&self) -> Option<&str> {
        self.relation_column.as_deref()
    }

    /// Get the PNR column name
    #[must_use] pub fn pnr_column(&self) -> &str {
        &self.pnr_column
    }

    /// Check if this is a direct filter
    #[must_use] pub const fn is_direct_filter(&self) -> bool {
        self.direct_filter
    }

    /// Convert to a `DataFusion` expression
    #[must_use] pub fn to_expr(&self) -> Option<Expr> {
        if self.pnrs.is_empty() {
            return None;
        }

        // Convert HashSet to literals for the IN expression
        let pnr_values: Vec<Expr> = self.pnrs.iter().map(|pnr| lit(pnr.clone())).collect();

        // Create IN expression
        if self.direct_filter {
            Some(col(&self.pnr_column).in_list(pnr_values, false))
        } else { 
            self.relation_column.as_ref().map(|relation_col| col(relation_col).in_list(pnr_values, false)) 
        }
    }

    /// Apply filter to a `DataFrame`
    pub fn apply_to_dataframe(&self, df: DataFrame) -> Result<DataFrame> {
        if let Some(expr) = self.to_expr() {
            Ok(df.filter(expr)?)
        } else {
            Ok(df)
        }
    }

    /// Create a predicate for `DataFusion` execution
    #[must_use] pub fn to_predicate(&self) -> Option<Expr> {
        self.to_expr()
    }
    
    /// Check if this filter is empty (contains no PNRs)
    #[must_use] pub fn is_empty(&self) -> bool {
        self.pnrs.is_empty()
    }
    
    /// Get the number of PNRs in this filter
    #[must_use] pub fn len(&self) -> usize {
        self.pnrs.len()
    }
}
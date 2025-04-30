use crate::error::Result;
use datafusion::prelude::*;
use std::collections::HashSet;

/// A structure for PNR filtering
pub struct PnrFilter {
    pnrs: HashSet<String>,
    direct_filter: bool,
    relation_column: Option<String>,
}

impl PnrFilter {
    /// Create a new PNR filter
    pub fn new(pnrs: HashSet<String>) -> Self {
        Self {
            pnrs,
            direct_filter: true,
            relation_column: None,
        }
    }

    /// Create a PNR filter with relation
    pub fn with_relation(pnrs: HashSet<String>, relation_column: &str) -> Self {
        Self {
            pnrs,
            direct_filter: false,
            relation_column: Some(relation_column.to_string()),
        }
    }

    /// Get the PNRs in this filter
    pub fn pnrs(&self) -> &HashSet<String> {
        &self.pnrs
    }

    /// Get the relation column if any
    pub fn relation_column(&self) -> Option<&str> {
        self.relation_column.as_deref()
    }

    /// Check if this is a direct filter
    pub fn is_direct_filter(&self) -> bool {
        self.direct_filter
    }

    /// Convert to a DataFusion expression
    pub fn to_expr(&self) -> Option<Expr> {
        if self.pnrs.is_empty() {
            return None;
        }

        // Convert HashSet to literals for the IN expression
        let pnr_values: Vec<Expr> = self.pnrs.iter().map(|pnr| lit(pnr.clone())).collect();

        // Create IN expression
        if self.direct_filter {
            Some(col("PNR").in_list(pnr_values, false))
        } else if let Some(relation_col) = &self.relation_column {
            Some(col(relation_col).in_list(pnr_values, false))
        } else {
            None
        }
    }

    /// Apply filter to a DataFrame
    pub fn apply_to_dataframe(&self, df: DataFrame) -> Result<DataFrame> {
        if let Some(expr) = self.to_expr() {
            Ok(df.filter(expr)?)
        } else {
            Ok(df)
        }
    }
}

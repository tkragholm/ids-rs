//! Statistical utilities for the IDS-RS library
//!
//! This module contains statistical utility functions.

/// Calculate the mean of a vector of values
#[must_use] pub fn mean(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    values.iter().sum::<f64>() / values.len() as f64
}

/// Calculate the variance of a vector of values
#[must_use] pub fn variance(values: &[f64], mean_value: f64) -> f64 {
    if values.len() <= 1 {
        return 0.0;
    }
    values.iter()
        .map(|x| (x - mean_value).powi(2))
        .sum::<f64>() / (values.len() - 1) as f64
}

/// Calculate the standard deviation of a vector of values
#[must_use] pub fn std_dev(values: &[f64], mean_value: f64) -> f64 {
    variance(values, mean_value).sqrt()
}

/// Calculate the median of a vector of values
#[must_use] pub fn median(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    if sorted.len() % 2 == 0 {
        // Even number of elements
        let mid = sorted.len() / 2;
        (sorted[mid - 1] + sorted[mid]) / 2.0
    } else {
        // Odd number of elements
        sorted[sorted.len() / 2]
    }
}

/// Calculate the standardized difference between two groups
#[must_use] pub fn standardized_difference(group1: &[f64], group2: &[f64]) -> f64 {
    let mean1 = mean(group1);
    let mean2 = mean(group2);
    let var1 = variance(group1, mean1);
    let var2 = variance(group2, mean2);
    
    let pooled_std = ((var1 + var2) / 2.0).sqrt();
    
    if pooled_std == 0.0 {
        return 0.0;
    }
    
    (mean1 - mean2) / pooled_std
}

/// Calculate summary statistics for a vector of values
#[must_use] pub fn summary_statistics(values: &[f64]) -> SummaryStatistics {
    if values.is_empty() {
        return SummaryStatistics::default();
    }
    
    let m = mean(values);
    let v = variance(values, m);
    let sd = v.sqrt();
    let med = median(values);
    
    let mut sorted = values.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    let min = sorted[0];
    let max = sorted[sorted.len() - 1];
    
    let q1 = if sorted.len() > 1 {
        let q1_idx = sorted.len() / 4;
        sorted[q1_idx]
    } else {
        min
    };
    
    let q3 = if sorted.len() > 1 {
        let q3_idx = (3 * sorted.len()) / 4;
        sorted[q3_idx]
    } else {
        max
    };
    
    SummaryStatistics {
        count: values.len(),
        mean: m,
        median: med,
        min,
        max,
        q1,
        q3,
        variance: v,
        std_dev: sd,
    }
}

/// Summary statistics for a dataset
#[derive(Debug, Clone, Copy)]
pub struct SummaryStatistics {
    /// Number of values
    pub count: usize,
    
    /// Mean
    pub mean: f64,
    
    /// Median
    pub median: f64,
    
    /// Minimum value
    pub min: f64,
    
    /// Maximum value
    pub max: f64,
    
    /// First quartile (25th percentile)
    pub q1: f64,
    
    /// Third quartile (75th percentile)
    pub q3: f64,
    
    /// Variance
    pub variance: f64,
    
    /// Standard deviation
    pub std_dev: f64,
}

impl Default for SummaryStatistics {
    fn default() -> Self {
        Self {
            count: 0,
            mean: 0.0,
            median: 0.0,
            min: 0.0,
            max: 0.0,
            q1: 0.0,
            q3: 0.0,
            variance: 0.0,
            std_dev: 0.0,
        }
    }
}
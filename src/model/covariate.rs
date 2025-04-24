use std::collections::HashMap;
use chrono::NaiveDate;

/// Value of a covariate
#[derive(Debug, Clone, PartialEq)]
pub enum CovariateValue {
    /// Numeric value
    Numeric(f64),
    
    /// Categorical value
    Categorical(String),
    
    /// Boolean value
    Boolean(bool),
    
    /// Date value
    Date(NaiveDate),
    
    /// Missing value
    None,
}

impl CovariateValue {
    /// Get the value as a number if possible
    #[must_use] pub fn as_numeric(&self) -> Option<f64> {
        match self {
            Self::Numeric(value) => Some(*value),
            Self::Boolean(true) => Some(1.0),
            Self::Boolean(false) => Some(0.0),
            _ => None,
        }
    }
    
    /// Get the value as a string if possible
    #[must_use] pub fn as_categorical(&self) -> Option<&str> {
        match self {
            Self::Categorical(value) => Some(value),
            _ => None,
        }
    }
    
    /// Get the value as a boolean if possible
    #[must_use] pub fn as_boolean(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            Self::Numeric(1.0) => Some(true),
            Self::Numeric(0.0) => Some(false),
            _ => None,
        }
    }
    
    /// Get the value as a date if possible
    #[must_use] pub fn as_date(&self) -> Option<NaiveDate> {
        match self {
            Self::Date(value) => Some(*value),
            _ => None,
        }
    }
    
    /// Check if the value is missing
    #[must_use] pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

/// Represents a covariate with multiple values
#[derive(Debug, Clone)]
pub struct Covariate {
    /// Name of the covariate
    name: String,
    
    /// Values of the covariate
    values: HashMap<String, CovariateValue>,
}

impl Covariate {
    /// Create a new covariate
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: HashMap::new(),
        }
    }

    /// Add a value to the covariate
    pub fn with_value(mut self, key: impl Into<String>, value: CovariateValue) -> Self {
        self.values.insert(key.into(), value);
        self
    }

    /// Get the name of the covariate
    #[must_use] pub fn name(&self) -> &str {
        &self.name
    }

    /// Get all values
    #[must_use] pub fn values(&self) -> &HashMap<String, CovariateValue> {
        &self.values
    }

    /// Get a specific value
    #[must_use] pub fn get(&self, key: &str) -> Option<&CovariateValue> {
        self.values.get(key)
    }
    
    /// Add a value
    pub fn add_value(&mut self, key: impl Into<String>, value: CovariateValue) {
        self.values.insert(key.into(), value);
    }
}
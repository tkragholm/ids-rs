use std::collections::HashMap;
use types::models::{Covariate, CovariateType};
use types::traits::{CovariateProcessor, VariableType};

/// Processor for income covariates
pub struct IncomeProcessor {
    name: String,
    variables: HashMap<String, VariableType>,
}

impl Default for IncomeProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl IncomeProcessor {
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        
        // Register all income variables with their types
        variables.insert("Income".to_string(), VariableType::Numeric);
        variables.insert("Wage Income".to_string(), VariableType::Numeric);
        variables.insert("Employment Status".to_string(), VariableType::Numeric);
        variables.insert("Employment Status Category".to_string(), VariableType::Categorical);
        variables.insert("Currency".to_string(), VariableType::Categorical);
        variables.insert("Income Type".to_string(), VariableType::Categorical);
        
        Self {
            name: "Income".to_string(),
            variables,
        }
    }
    
    /// Get variable type for a specific income variable
    pub fn get_variable_type_for(&self, variable_name: &str) -> VariableType {
        self.variables
            .get(variable_name)
            .copied()
            .unwrap_or(VariableType::Numeric) // Most income variables are numeric
    }
}

impl CovariateProcessor for IncomeProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Income
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        match covariate.get_type() {
            CovariateType::Income => {
                // Income amount is the primary numeric value
                covariate.get_income_amount()
            },
            _ => None,
        }
    }
    
    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        match covariate.get_type() {
            CovariateType::Income => {
                // Income type is the primary categorical value
                covariate.get_income_type_code().map(|s| s.to_string())
            },
            _ => None,
        }
    }
    
    fn is_categorical(&self) -> bool {
        // Income is primarily numeric
        false
    }
}

/// Specific income variable processor - holds settings for a specific variable
pub struct IncomeVariableProcessor {
    name: String,
    variable_type: VariableType,
    accessor: fn(&Covariate) -> Option<f64>,
    categorical_accessor: Option<fn(&Covariate) -> Option<String>>,
}

impl IncomeVariableProcessor {
    /// Create a new processor for primary income
    pub fn income() -> Self {
        Self {
            name: "Income".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_income_amount(),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for wage income
    pub fn wage_income() -> Self {
        Self {
            name: "Wage Income".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_wage_income(),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for employment status
    pub fn employment_status() -> Self {
        Self {
            name: "Employment Status".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_employment_status().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for employment status as category
    pub fn employment_status_category() -> Self {
        Self {
            name: "Employment Status Category".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_employment_status().map(|v| v as f64),
            categorical_accessor: Some(|c| c.get_employment_status().map(|v| v.to_string())),
        }
    }
    
    /// Create a new processor for currency
    pub fn currency() -> Self {
        Self {
            name: "Currency".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| {
                c.get_currency().clone().map(|v| {
                    let mut hash = 0.0;
                    for (i, b) in v.bytes().enumerate() {
                        hash += (b as f64) * (i + 1) as f64;
                    }
                    hash
                })
            },
            categorical_accessor: Some(|c| c.get_currency().map(|s| s.to_string())),
        }
    }
    
    /// Create a new processor for income type
    pub fn income_type() -> Self {
        Self {
            name: "Income Type".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| {
                c.get_income_type_code().clone().map(|v| {
                    let mut hash = 0.0;
                    for (i, b) in v.bytes().enumerate() {
                        hash += (b as f64) * (i + 1) as f64;
                    }
                    hash
                })
            },
            categorical_accessor: Some(|c| c.get_income_type_code().map(|s| s.to_string())),
        }
    }
}

impl CovariateProcessor for IncomeVariableProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Income
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != CovariateType::Income {
            return None;
        }
        
        (self.accessor)(covariate)
    }
    
    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != CovariateType::Income {
            return None;
        }
        
        match self.categorical_accessor {
            Some(accessor) => accessor(covariate),
            None => None,
        }
    }
    
    fn is_categorical(&self) -> bool {
        matches!(self.variable_type, VariableType::Categorical)
    }
    
    fn get_variable_type(&self) -> VariableType {
        self.variable_type
    }
}
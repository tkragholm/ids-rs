use std::collections::HashMap;
use types::models::{Covariate, CovariateType};
use types::traits::{CovariateProcessor, VariableType};

/// Processor for education covariates
pub struct EducationProcessor {
    name: String,
    variables: HashMap<String, VariableType>,
}

impl Default for EducationProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl EducationProcessor {
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        
        // Register all education variables with their types
        variables.insert("Education Level".to_string(), VariableType::Categorical);
        variables.insert("ISCED Level".to_string(), VariableType::Categorical);
        variables.insert("Education Years".to_string(), VariableType::Numeric);
        
        Self {
            name: "Education".to_string(),
            variables,
        }
    }
    
    /// Get variable type for a specific education variable
    pub fn get_variable_type_for(&self, variable_name: &str) -> VariableType {
        self.variables
            .get(variable_name)
            .copied()
            .unwrap_or(VariableType::Categorical) // Most education variables are categorical
    }
}

impl CovariateProcessor for EducationProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Education
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        match covariate.get_type() {
            CovariateType::Education => {
                // Education years is the primary numeric value
                covariate.get_education_years().map(|val| val as f64)
            },
            _ => None,
        }
    }
    
    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        match covariate.get_type() {
            CovariateType::Education => {
                // Education level is the primary categorical value
                covariate.get_education_level().map(|s| s.to_string())
            },
            _ => None,
        }
    }
    
    fn is_categorical(&self) -> bool {
        // Education is primarily categorical
        true
    }
}

/// Specific education variable processor - holds settings for a specific variable
pub struct EducationVariableProcessor {
    name: String,
    variable_type: VariableType,
    accessor: fn(&Covariate) -> Option<f64>,
    categorical_accessor: Option<fn(&Covariate) -> Option<String>>,
}

impl EducationVariableProcessor {
    /// Create a new processor for education level
    pub fn education_level() -> Self {
        Self {
            name: "Education Level".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_education_level().clone().and_then(|v| v.parse::<f64>().ok()),
            categorical_accessor: Some(|c| c.get_education_level().map(|s| s.to_string())),
        }
    }
    
    /// Create a new processor for ISCED level
    pub fn isced_level() -> Self {
        Self {
            name: "ISCED Level".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_isced_code().and_then(|v| v.parse::<f64>().ok()),
            categorical_accessor: Some(|c| c.get_isced_code().map(|s| s.to_string())),
        }
    }
    
    /// Create a new processor for education years
    pub fn education_years() -> Self {
        Self {
            name: "Education Years".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_education_years().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
}

impl CovariateProcessor for EducationVariableProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Education
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != CovariateType::Education {
            return None;
        }
        
        (self.accessor)(covariate)
    }
    
    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != CovariateType::Education {
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
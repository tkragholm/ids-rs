use std::collections::HashMap;
use types::models::{Covariate, CovariateType};
use types::traits::{CovariateProcessor, VariableType};

/// Processor for occupation covariates
pub struct OccupationProcessor {
    name: String,
    variables: HashMap<String, VariableType>,
}

impl Default for OccupationProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl OccupationProcessor {
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        
        // Register all occupation variables with their types
        variables.insert("SOCIO13 Code".to_string(), VariableType::Categorical);
        variables.insert("SOCIO13 Value".to_string(), VariableType::Numeric);
        variables.insert("Classification System".to_string(), VariableType::Categorical);
        variables.insert("SOCIO".to_string(), VariableType::Numeric);
        variables.insert("SOCIO Category".to_string(), VariableType::Categorical);
        variables.insert("SOCIO02".to_string(), VariableType::Numeric);
        variables.insert("SOCIO02 Category".to_string(), VariableType::Categorical);
        variables.insert("Previous Socioeconomic Status".to_string(), VariableType::Numeric);
        variables.insert("Previous Socioeconomic Category".to_string(), VariableType::Categorical);
        
        Self {
            name: "Occupation".to_string(),
            variables,
        }
    }
    
    /// Get variable type for a specific occupation variable
    pub fn get_variable_type_for(&self, variable_name: &str) -> VariableType {
        self.variables
            .get(variable_name)
            .copied()
            .unwrap_or(VariableType::Categorical) // Most occupation variables are categorical
    }
}

impl CovariateProcessor for OccupationProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Occupation
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        match covariate.get_type() {
            CovariateType::Occupation => {
                // Convert occupation code to numeric if possible
                covariate
                    .get_occupation_code()
                    .and_then(|code| code.parse::<f64>().ok())
            },
            _ => None,
        }
    }
    
    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        match covariate.get_type() {
            CovariateType::Occupation => {
                // Occupation code is the primary categorical value
                covariate.get_occupation_code().map(|s| s.to_string())
            },
            _ => None,
        }
    }
    
    fn is_categorical(&self) -> bool {
        // Occupation is primarily categorical
        true
    }
}

/// Specific occupation variable processor - holds settings for a specific variable
pub struct OccupationVariableProcessor {
    name: String,
    variable_type: VariableType,
    accessor: fn(&Covariate) -> Option<f64>,
    categorical_accessor: Option<fn(&Covariate) -> Option<String>>,
}

impl OccupationVariableProcessor {
    /// Create a new processor for SOCIO13 code
    pub fn socio13_code() -> Self {
        Self {
            name: "SOCIO13 Code".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_occupation_code().and_then(|v| v.parse::<f64>().ok()),
            categorical_accessor: Some(|c| c.get_occupation_code().map(|s| s.to_string())),
        }
    }
    
    /// Create a new processor for SOCIO13 numeric value
    pub fn socio13_value() -> Self {
        Self {
            name: "SOCIO13 Value".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_occupation_code().and_then(|v| v.parse::<f64>().ok()),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for classification system
    pub fn classification_system() -> Self {
        Self {
            name: "Classification System".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| {
                c.get_classification().map(|v| {
                    // Simple hash for string
                    let mut hash = 0.0;
                    for (i, b) in v.bytes().enumerate() {
                        hash += (b as f64) * (i + 1) as f64;
                    }
                    hash
                })
            },
            categorical_accessor: Some(|c| c.get_classification().map(|s| s.to_string())),
        }
    }
    
    /// Create a new processor for SOCIO
    pub fn socio() -> Self {
        Self {
            name: "SOCIO".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_socio().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for SOCIO as a category
    pub fn socio_category() -> Self {
        Self {
            name: "SOCIO Category".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_socio().map(|v| v as f64),
            categorical_accessor: Some(|c| c.get_socio().map(|v| v.to_string())),
        }
    }
    
    /// Create a new processor for SOCIO02
    pub fn socio02() -> Self {
        Self {
            name: "SOCIO02".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_socio02().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for SOCIO02 as a category
    pub fn socio02_category() -> Self {
        Self {
            name: "SOCIO02 Category".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_socio02().map(|v| v as f64),
            categorical_accessor: Some(|c| c.get_socio02().map(|v| v.to_string())),
        }
    }
    
    /// Create a new processor for previous socioeconomic status
    pub fn pre_socio() -> Self {
        Self {
            name: "Previous Socioeconomic Status".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_pre_socio().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for previous socioeconomic status as a category
    pub fn pre_socio_category() -> Self {
        Self {
            name: "Previous Socioeconomic Category".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_pre_socio().map(|v| v as f64),
            categorical_accessor: Some(|c| c.get_pre_socio().map(|v| v.to_string())),
        }
    }
}

impl CovariateProcessor for OccupationVariableProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Occupation
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != CovariateType::Occupation {
            return None;
        }
        
        (self.accessor)(covariate)
    }
    
    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != CovariateType::Occupation {
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
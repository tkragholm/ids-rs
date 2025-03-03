use std::collections::HashMap;
use types::models::{Covariate, CovariateType};
use types::traits::{CovariateProcessor, VariableType};

/// Processor for demographic covariates
pub struct DemographicsProcessor {
    name: String,
    variables: HashMap<String, VariableType>,
}

impl Default for DemographicsProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl DemographicsProcessor {
    pub fn new() -> Self {
        let mut variables = HashMap::new();
        
        // Register all demographic variables with their types
        variables.insert("Family Size".to_string(), VariableType::Numeric);
        variables.insert("Municipality".to_string(), VariableType::Numeric);
        variables.insert("Family Type".to_string(), VariableType::Categorical);
        variables.insert("Civil Status".to_string(), VariableType::Categorical);
        variables.insert("Gender".to_string(), VariableType::Categorical);
        variables.insert("Citizenship".to_string(), VariableType::Categorical);
        variables.insert("Age".to_string(), VariableType::Numeric);
        variables.insert("Children Count".to_string(), VariableType::Numeric);
        
        Self {
            name: "Demographics".to_string(),
            variables,
        }
    }
    
    /// Get variable type for a specific demographic variable
    pub fn get_variable_type_for(&self, variable_name: &str) -> VariableType {
        self.variables
            .get(variable_name)
            .copied()
            .unwrap_or(VariableType::Numeric)
    }
}

impl CovariateProcessor for DemographicsProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Demographics
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        // Function to extract numeric values based on variable name
        // This could be extended with a match statement for different variables
        match covariate.get_type() {
            CovariateType::Demographics => {
                // Family size is always available in demographics
                covariate.get_family_size().map(|val| val as f64)
            },
            _ => None,
        }
    }
    
    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        // Function to extract categorical values based on variable name
        match covariate.get_type() {
            CovariateType::Demographics => {
                // Family type is always available in demographics
                covariate.get_family_type()
            },
            _ => None,
        }
    }
    
    fn is_categorical(&self) -> bool {
        // Demographics has a mix of categorical and numeric values
        // Default to numeric
        false
    }
}

/// Specific demographic variable processor - holds settings for a specific variable
pub struct DemographicVariableProcessor {
    name: String,
    variable_type: VariableType,
    accessor: fn(&Covariate) -> Option<f64>,
    categorical_accessor: Option<fn(&Covariate) -> Option<String>>,
}

impl DemographicVariableProcessor {
    /// Create a new processor for family size
    pub fn family_size() -> Self {
        Self {
            name: "Family Size".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_family_size().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for municipality
    pub fn municipality() -> Self {
        Self {
            name: "Municipality".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_municipality().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for family type
    pub fn family_type() -> Self {
        Self {
            name: "Family Type".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_family_type().and_then(|v| v.parse::<f64>().ok()),
            categorical_accessor: Some(|c| c.get_family_type()),
        }
    }
    
    /// Create a new processor for civil status
    pub fn civil_status() -> Self {
        Self {
            name: "Civil Status".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_civil_status().map(|v| v.bytes().next().unwrap_or(0) as f64),
            categorical_accessor: Some(|c| c.get_civil_status()),
        }
    }
    
    /// Create a new processor for gender
    pub fn gender() -> Self {
        Self {
            name: "Gender".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| c.get_gender().map(|v| v.bytes().next().unwrap_or(0) as f64),
            categorical_accessor: Some(|c| c.get_gender()),
        }
    }
    
    /// Create a new processor for citizenship
    pub fn citizenship() -> Self {
        Self {
            name: "Citizenship".to_string(),
            variable_type: VariableType::Categorical,
            accessor: |c| {
                c.get_citizenship().map(|v| {
                    let mut hash = 0.0;
                    for (i, b) in v.bytes().enumerate() {
                        hash += (b as f64) * (i + 1) as f64;
                    }
                    hash
                })
            },
            categorical_accessor: Some(|c| c.get_citizenship()),
        }
    }
    
    /// Create a new processor for age
    pub fn age() -> Self {
        Self {
            name: "Age".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_age().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
    
    /// Create a new processor for children count
    pub fn children_count() -> Self {
        Self {
            name: "Children Count".to_string(),
            variable_type: VariableType::Numeric,
            accessor: |c| c.get_children_count().map(|v| v as f64),
            categorical_accessor: None,
        }
    }
}

impl CovariateProcessor for DemographicVariableProcessor {
    fn get_name(&self) -> &str {
        &self.name
    }
    
    fn get_covariate_type(&self) -> CovariateType {
        CovariateType::Demographics
    }
    
    fn process_numeric(&self, covariate: &Covariate) -> Option<f64> {
        if covariate.get_type() != CovariateType::Demographics {
            return None;
        }
        
        (self.accessor)(covariate)
    }
    
    fn process_categorical(&self, covariate: &Covariate) -> Option<String> {
        if covariate.get_type() != CovariateType::Demographics {
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
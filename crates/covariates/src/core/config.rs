use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use types::models::CovariateType;
use types::traits::VariableType;

/// Configuration for a covariate variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovariateVariableConfig {
    /// Variable name (e.g., "Age", "Family Size")
    pub name: String,
    
    /// Variable type (Numeric, Categorical, Binary)
    pub variable_type: VariableType,
    
    /// Field accessor method in the Covariate struct (e.g., "get_age", "get_family_size")
    pub accessor: String,
    
    /// Translation map to use for categorical variables (e.g., "family_type", "civst")
    pub translation: Option<String>,
    
    /// Description of the variable
    pub description: Option<String>,
    
    /// Default value when variable is missing
    pub default_value: Option<String>,
    
    /// Additional configuration options as key-value pairs
    pub options: HashMap<String, String>,
}

/// Configuration for a covariate type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovariateTypeConfig {
    /// Name of the covariate type (e.g., "Demographics", "Income")
    pub name: String,
    
    /// Covariate type enum
    pub covariate_type: CovariateType,
    
    /// List of variables in this covariate type
    pub variables: Vec<CovariateVariableConfig>,
    
    /// Description of the covariate type
    pub description: Option<String>,
}

/// Complete configuration for all covariate processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CovariatesConfig {
    /// List of covariate types
    pub covariate_types: Vec<CovariateTypeConfig>,
    
    /// Global settings
    pub settings: HashMap<String, String>,
}

impl CovariatesConfig {
    /// Load a covariates configuration from a file
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let config: CovariatesConfig = serde_json::from_reader(reader)?;
        Ok(config)
    }
    
    /// Get a covariate type configuration by type
    pub fn get_covariate_type(&self, covariate_type: CovariateType) -> Option<&CovariateTypeConfig> {
        self.covariate_types
            .iter()
            .find(|ct| ct.covariate_type == covariate_type)
    }
    
    /// Get a default configuration with all standard covariate types and variables
    pub fn default_config() -> Self {
        // Demographics variables
        let demographics_variables = vec![
            CovariateVariableConfig {
                name: "Family Size".to_string(),
                variable_type: VariableType::Numeric,
                accessor: "get_family_size".to_string(),
                translation: None,
                description: Some("Size of the family unit".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Municipality".to_string(),
                variable_type: VariableType::Numeric,
                accessor: "get_municipality".to_string(),
                translation: None,
                description: Some("Municipality code".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Family Type".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_family_type".to_string(),
                translation: Some("family_type".to_string()),
                description: Some("Type of family structure".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Civil Status".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_civil_status".to_string(),
                translation: Some("civst".to_string()),
                description: Some("Civil status code".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Gender".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_gender".to_string(),
                translation: None,
                description: Some("Gender code".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Citizenship".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_citizenship".to_string(),
                translation: Some("statsb".to_string()),
                description: Some("Citizenship/country code".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Age".to_string(),
                variable_type: VariableType::Numeric,
                accessor: "get_age".to_string(),
                translation: None,
                description: Some("Age in years".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Children Count".to_string(),
                variable_type: VariableType::Numeric,
                accessor: "get_children_count".to_string(),
                translation: None,
                description: Some("Number of children".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
        ];

        // Income variables
        let income_variables = vec![
            CovariateVariableConfig {
                name: "Income Amount".to_string(),
                variable_type: VariableType::Numeric,
                accessor: "get_income_amount".to_string(),
                translation: None,
                description: Some("Total income amount".to_string()),
                default_value: Some("0".to_string()),
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Wage Income".to_string(),
                variable_type: VariableType::Numeric,
                accessor: "get_wage_income".to_string(),
                translation: None,
                description: Some("Income from wages (LOENMV_13)".to_string()),
                default_value: Some("0".to_string()),
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Employment Status".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_employment_status".to_string(),
                translation: Some("beskst13".to_string()),
                description: Some("Employment status code (BESKST13)".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
        ];

        // Education variables
        let education_variables = vec![
            CovariateVariableConfig {
                name: "Education Level".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_education_level".to_string(),
                translation: None,
                description: Some("Highest education level attained".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "ISCED Code".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_isced_code".to_string(),
                translation: None,
                description: Some("International Standard Classification of Education code".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Education Years".to_string(),
                variable_type: VariableType::Numeric,
                accessor: "get_education_years".to_string(),
                translation: None,
                description: Some("Years of education".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
        ];

        // Occupation variables
        let occupation_variables = vec![
            CovariateVariableConfig {
                name: "Occupation Code".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_occupation_code".to_string(),
                translation: None,
                description: Some("Occupation classification code".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "Classification".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_classification".to_string(),
                translation: None,
                description: Some("Classification system used for the occupation code".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "SOCIO".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_socio".to_string(),
                translation: Some("socio13".to_string()),
                description: Some("Socioeconomic classification (SOCIO)".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "SOCIO02".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_socio02".to_string(),
                translation: None,
                description: Some("Alternative socioeconomic classification (SOCIO02)".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
            CovariateVariableConfig {
                name: "PRE_SOCIO".to_string(),
                variable_type: VariableType::Categorical,
                accessor: "get_pre_socio".to_string(),
                translation: Some("pre_socio".to_string()),
                description: Some("Previous socioeconomic classification (PRE_SOCIO)".to_string()),
                default_value: None,
                options: HashMap::new(),
            },
        ];

        // Create the complete configuration
        Self {
            covariate_types: vec![
                CovariateTypeConfig {
                    name: "Demographics".to_string(),
                    covariate_type: CovariateType::Demographics,
                    variables: demographics_variables,
                    description: Some("Demographic variables like age, gender, and family status".to_string()),
                },
                CovariateTypeConfig {
                    name: "Income".to_string(),
                    covariate_type: CovariateType::Income,
                    variables: income_variables,
                    description: Some("Income-related variables".to_string()),
                },
                CovariateTypeConfig {
                    name: "Education".to_string(),
                    covariate_type: CovariateType::Education,
                    variables: education_variables,
                    description: Some("Education-related variables".to_string()),
                },
                CovariateTypeConfig {
                    name: "Occupation".to_string(),
                    covariate_type: CovariateType::Occupation,
                    variables: occupation_variables,
                    description: Some("Occupation and employment variables".to_string()),
                },
            ],
            settings: HashMap::new(),
        }
    }
    
    /// Save the configuration to a file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

/// Helper for generating a default configuration file
pub fn generate_default_config<P: AsRef<Path>>(path: P) -> Result<(), Box<dyn std::error::Error>> {
    let config = CovariatesConfig::default_config();
    config.save_to_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;
    
    #[test]
    fn test_config_serialization() {
        let config = CovariatesConfig::default_config();
        let serialized = serde_json::to_string_pretty(&config).unwrap();
        let deserialized: CovariatesConfig = serde_json::from_str(&serialized).unwrap();
        
        assert_eq!(config.covariate_types.len(), deserialized.covariate_types.len());
        
        // Check that all covariate types are present
        for ct in &config.covariate_types {
            let found = deserialized.covariate_types
                .iter()
                .find(|c| c.covariate_type == ct.covariate_type)
                .unwrap();
                
            assert_eq!(ct.name, found.name);
            assert_eq!(ct.variables.len(), found.variables.len());
        }
    }
    
    #[test]
    fn test_save_and_load_config() -> Result<(), Box<dyn std::error::Error>> {
        let config = CovariatesConfig::default_config();
        
        // Create a temporary file
        let mut temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_path_buf();
        
        // Write the config to the file
        let json = serde_json::to_string_pretty(&config)?;
        temp_file.write_all(json.as_bytes())?;
        temp_file.flush()?;
        
        // Load the config from the file
        let loaded_config = CovariatesConfig::from_file(&path)?;
        
        // Check that the loaded config matches the original
        assert_eq!(config.covariate_types.len(), loaded_config.covariate_types.len());
        
        Ok(())
    }
    
    #[test]
    fn test_generate_default_config() -> Result<(), Box<dyn std::error::Error>> {
        // Create a temporary file
        let temp_file = NamedTempFile::new()?;
        let path = temp_file.path().to_path_buf();
        
        // Generate the default config
        generate_default_config(&path)?;
        
        // Load the config from the file
        let config = CovariatesConfig::from_file(&path)?;
        
        // Check that the config has the expected types
        assert_eq!(config.covariate_types.len(), 4);
        
        let types: Vec<CovariateType> = config.covariate_types
            .iter()
            .map(|ct| ct.covariate_type)
            .collect();
            
        assert!(types.contains(&CovariateType::Demographics));
        assert!(types.contains(&CovariateType::Income));
        assert!(types.contains(&CovariateType::Education));
        assert!(types.contains(&CovariateType::Occupation));
        
        Ok(())
    }
}
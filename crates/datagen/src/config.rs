/// Configuration for synthetic register data generation
#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    /// Start year for data generation (1980-2023)
    pub start_year: i32,
    
    /// End year for data generation (1980-2023)
    pub end_year: i32,
    
    /// Total number of records to generate
    pub total_records: usize,
    
    /// Number of treatment cases to include in the data
    pub treatment_cases: usize,
    
    /// Output directory path for generated data
    pub output_dir: String,
    
    /// Optional random seed for reproducible generation
    pub seed: Option<u64>,
}

impl GeneratorConfig {
    /// Create a new configuration with default year range (2000-2023)
    ///
    /// # Arguments
    /// * `total_records` - Total number of records to generate
    /// * `treatment_cases` - Number of treatment cases to include
    /// * `output_dir` - Directory path where generated data will be stored
    #[must_use]
    pub const fn new(total_records: usize, treatment_cases: usize, output_dir: String) -> Self {
        Self {
            start_year: 2000,
            end_year: 2023,
            total_records,
            treatment_cases,
            output_dir,
            seed: None,
        }
    }

    /// Set a random seed for reproducible data generation
    ///
    /// # Arguments
    /// * `seed` - The random seed to use
    #[must_use]
    pub const fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    /// Set a custom year range for data generation
    ///
    /// # Arguments
    /// * `start` - Start year (1980-2023)
    /// * `end` - End year (1980-2023)
    #[must_use]
    pub const fn with_year_range(mut self, start: i32, end: i32) -> Self {
        self.start_year = start;
        self.end_year = end;
        self
    }

    /// Validate the configuration parameters
    ///
    /// # Returns
    /// * `Result<(), String>` - Ok if valid, or an error message
    pub fn validate(&self) -> Result<(), String> {
        if self.treatment_cases >= self.total_records {
            return Err(format!(
                "Number of treatment cases ({}) must be less than total records ({})",
                self.treatment_cases, self.total_records
            ));
        }
        
        if self.start_year >= self.end_year {
            return Err(format!(
                "Start year ({}) must be before end year ({})",
                self.start_year, self.end_year
            ));
        }
        
        if self.start_year < 1980 || self.end_year > 2023 {
            return Err(format!(
                "Year range ({}-{}) must be between 1980 and 2023",
                self.start_year, self.end_year
            ));
        }
        
        Ok(())
    }
}

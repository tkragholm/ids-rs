#[derive(Debug, Clone)]
pub struct GeneratorConfig {
    pub start_year: i32,
    pub end_year: i32,
    pub total_records: usize,
    pub treatment_cases: usize,
    pub output_dir: String,
    pub seed: Option<u64>,
}

impl GeneratorConfig {
    pub fn new(total_records: usize, treatment_cases: usize, output_dir: String) -> Self {
        Self {
            start_year: 2000,
            end_year: 2022,
            total_records,
            treatment_cases,
            output_dir,
            seed: None,
        }
    }

    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = Some(seed);
        self
    }

    pub fn with_year_range(mut self, start: i32, end: i32) -> Self {
        self.start_year = start;
        self.end_year = end;
        self
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.treatment_cases >= self.total_records {
            return Err("Number of treatment cases must be less than total records".into());
        }
        if self.start_year >= self.end_year {
            return Err("Start year must be before end year".into());
        }
        if self.start_year < 1980 || self.end_year > 2023 {
            return Err("Year range must be between 1980 and 2023".into());
        }
        Ok(())
    }
}

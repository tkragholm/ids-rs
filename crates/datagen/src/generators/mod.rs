mod akm;
mod bef;
mod family;
mod ind;
mod pediatric;
mod uddf;

use crate::{config::GeneratorConfig, error::DataGenError};
use chrono::NaiveDate;
use indicatif::MultiProgress;

use rand::rngs::StdRng;
use rand::SeedableRng;

use std::path::Path;
use types::models::pnr::PnrPool;

/// Generator for synthetic register data that simulates Nordic administrative registers
/// 
/// This generator creates multiple register files in Parquet format including:
/// - Family relations
/// - Population registers (BEF)
/// - Labor market registers (AKM)
/// - Individual registers (IND)
/// - Education registers (UDDF)
pub struct RegisterGenerator {
    config: GeneratorConfig,
    rng: StdRng,
    progress: MultiProgress,
    pnr_pool: PnrPool,
}

impl RegisterGenerator {
    /// Create a new RegisterGenerator with the provided configuration
    ///
    /// # Arguments
    /// * `config` - Configuration for the data generation
    ///
    /// # Returns
    /// * `Result<Self, DataGenError>` - The generator or an error if configuration is invalid
    pub fn new(config: GeneratorConfig) -> Result<Self, DataGenError> {
        // Validate configuration settings
        config.validate().map_err(DataGenError::Config)?;

        // Initialize random number generator with seed or from OS entropy
        let mut rng = match config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_os_rng(),
        };

        // Initialize the generator
        Ok(Self {
            pnr_pool: PnrPool::new(config.total_records, &mut rng)?,
            config,
            rng,
            progress: MultiProgress::new(),
        })
    }

    /// Get a person record by index
    fn get_person(&self, index: usize) -> Option<(NaiveDate, String)> {
        self.pnr_pool.get_child(&index)
    }

    /// Get a family (child, father, mother) by index
    #[allow(clippy::type_complexity)]
    fn get_family(
        &self,
        index: usize,
    ) -> Option<(
        (NaiveDate, String),
        ((NaiveDate, String), (NaiveDate, String)),
    )> {
        self.pnr_pool.get_family(&index)
    }

    /// Convert a date to days since epoch (1970-01-01)
    const fn date_to_days_since_epoch(date: NaiveDate) -> i32 {
        date.signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .num_days() as i32
    }

    /// Generate all register data based on configuration
    ///
    /// This generates data for all registers in the following order:
    /// 1. Family relationships
    /// 2. Employment (AKM)
    /// 3. Population (BEF)
    /// 4. Individual (IND)
    /// 5. Education (UDDF)
    pub fn generate_all(&mut self) -> Result<(), DataGenError> {
        // Create output directories
        self.create_directories()?;

        // Generate family relationships first
        self.generate_family()?;

        // Generate data for each register in parallel
        // Note: Currently sequential for deterministic output, but could be parallelized
        self.generate_akm()?;
        self.generate_bef()?;
        self.generate_ind()?;
        self.generate_uddf()?;

        Ok(())
    }

    /// Create necessary output directories
    fn create_directories(&self) -> Result<(), DataGenError> {
        // Create base output directory
        let base_dir = Path::new(&self.config.output_dir);
        std::fs::create_dir_all(base_dir)?;
        
        // Create register subdirectories
        let register_dirs = ["akm", "bef", "ind", "uddf"];
        for dir in &register_dirs {
            std::fs::create_dir_all(base_dir.join(dir))?;
        }

        Ok(())
    }
}

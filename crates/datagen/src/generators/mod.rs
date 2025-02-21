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
use types::pnr::PnrPool;

pub struct RegisterGenerator {
    config: GeneratorConfig,
    rng: StdRng,
    progress: MultiProgress,
    pnr_pool: PnrPool,
}

impl RegisterGenerator {
    pub fn new(config: GeneratorConfig) -> Result<Self, DataGenError> {
        config.validate().map_err(DataGenError::Config)?;

        let mut rng = match config.seed {
            Some(seed) => StdRng::seed_from_u64(seed),
            None => StdRng::from_entropy(),
        };

        Ok(Self {
            pnr_pool: PnrPool::new(config.total_records, &mut rng),
            config,
            rng,
            progress: MultiProgress::new(),
        })
    }

    fn get_person(&self, index: usize) -> Option<(NaiveDate, String)> {
        self.pnr_pool.get_child(&index)
    }

    fn get_family(
        &self,
        index: usize,
    ) -> Option<(
        (NaiveDate, String),
        ((NaiveDate, String), (NaiveDate, String)),
    )> {
        self.pnr_pool.get_family(&index)
    }

    const fn date_to_days_since_epoch(date: NaiveDate) -> i32 {
        date.signed_duration_since(NaiveDate::from_ymd_opt(1970, 1, 1).unwrap())
            .num_days() as i32
    }

    pub fn generate_all(&mut self) -> Result<(), DataGenError> {
        // Create output directories
        self.create_directories()?;

        // Generate family relationships first
        self.generate_family()?;

        // Generate data for each register
        self.generate_akm()?;
        self.generate_bef()?;
        self.generate_ind()?;
        self.generate_uddf()?;

        Ok(())
    }

    fn create_directories(&self) -> Result<(), DataGenError> {
        // Create register directories
        for dir in ["akm", "bef", "ind", "uddf"] {
            std::fs::create_dir_all(Path::new(&self.config.output_dir).join(dir))?;
        }

        // Create base output directory if it doesn't exist
        std::fs::create_dir_all(&self.config.output_dir)?;

        Ok(())
    }
}

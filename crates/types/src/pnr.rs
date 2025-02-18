use chrono::{Duration, NaiveDate};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct PnrPool {
    pool: HashMap<usize, (NaiveDate, String)>,
    children: HashMap<usize, (NaiveDate, String)>,
    parents: HashMap<usize, (NaiveDate, String)>,
}

impl PnrPool {
    pub fn new<R: Rng>(total_records: usize, rng: &mut R) -> Self {
        let mut pool = HashMap::new();
        let mut children = HashMap::new();
        let mut parents = HashMap::new();

        // Define study period constants
        let study_start = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
        let study_end = NaiveDate::from_ymd_opt(2018, 12, 31).unwrap();
        let earliest_birth = NaiveDate::from_ymd_opt(1995, 1, 1).unwrap();
        let latest_birth = study_end;

        let birth_range_days = (latest_birth - earliest_birth).num_days() as i32;

        // Generate children first
        for i in 0..total_records {
            // Generate child's birth date within study period
            let days_offset = rng.gen_range(0..=birth_range_days);
            let birth_date = earliest_birth + Duration::days(days_offset as i64);

            let sequence = rng.gen_range(0..10000);
            let pnr = format!("{}-{:04}", birth_date.format("%d%m%y"), sequence);

            children.insert(i, (birth_date, pnr.clone()));
            pool.insert(i, (birth_date, pnr));

            // Generate parents based on child's birth date
            let mother_age = rng.gen_range(20..46); // mothers aged 20-45 at birth
            let father_age = rng.gen_range(20..50); // fathers aged 20-49 at birth

            let mother_birth = birth_date - Duration::days(mother_age * 365);
            let father_birth = birth_date - Duration::days(father_age * 365);

            let mother_sequence = rng.gen_range(0..10000);
            let father_sequence = rng.gen_range(0..10000);

            let mother_pnr = format!("{}-{:04}", mother_birth.format("%d%m%y"), mother_sequence);
            let father_pnr = format!("{}-{:04}", father_birth.format("%d%m%y"), father_sequence);

            parents.insert(i + 1000000, (father_birth, father_pnr.clone())); // Father
            parents.insert(i + 2000000, (mother_birth, mother_pnr.clone())); // Mother

            pool.insert(i + 1000000, (father_birth, father_pnr));
            pool.insert(i + 2000000, (mother_birth, mother_pnr));
        }

        Self {
            pool,
            children,
            parents,
        }
    }

    pub fn get(&self, index: &usize) -> Option<(NaiveDate, String)> {
        self.pool.get(index).map(|(date, pnr)| (*date, pnr.clone()))
    }

    pub fn get_child(&self, index: &usize) -> Option<(NaiveDate, String)> {
        self.children
            .get(index)
            .map(|(date, pnr)| (*date, pnr.clone()))
    }

    pub fn get_parents(&self, index: &usize) -> Option<((NaiveDate, String), (NaiveDate, String))> {
        let father = self.parents.get(&(index + 1000000))?;
        let mother = self.parents.get(&(index + 2000000))?;
        Some(((father.0, father.1.clone()), (mother.0, mother.1.clone())))
    }

    pub fn get_family(
        &self,
        index: &usize,
    ) -> Option<(
        (NaiveDate, String),                        // Child
        ((NaiveDate, String), (NaiveDate, String)), // Parents (Father, Mother)
    )> {
        let child = self.get_child(index)?;
        let parents = self.get_parents(index)?;
        Some((child, parents))
    }
}

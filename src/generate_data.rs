use chrono::{Duration, NaiveDate};
use csv::Writer;
use rand::seq::SliceRandom;
use rand::Rng;
use std::collections::HashSet;
use std::error::Error;

pub fn generate_pediatric_data(
    filename: &str,
    total_records: usize,
    treatment_cases: usize,
) -> Result<(), Box<dyn Error>> {
    let mut rng = rand::rng();
    let mut writer = Writer::from_path(filename)?;

    // Write header
    writer.write_record(&[
        "pnr",
        "bday",
        "treatment_date",
        "mother_bday",
        "father_bday",
    ])?;

    // Define the study period (2000-2018)
    let study_start = NaiveDate::from_ymd_opt(2000, 1, 1).unwrap();
    let study_end = NaiveDate::from_ymd_opt(2018, 12, 31).unwrap();

    // Calculate the earliest possible birth date (6 years before study end)
    let earliest_birth = NaiveDate::from_ymd_opt(1995, 1, 1).unwrap(); // To allow for children up to 6 years in 2000
    let latest_birth = study_end;

    let birth_range_days = (latest_birth - earliest_birth).num_days() as i32;

    // Create a vec of indices for treatment cases
    let mut indices: Vec<usize> = (0..total_records).collect();
    indices.shuffle(&mut rng);
    let treatment_indices: HashSet<usize> = indices.into_iter().take(treatment_cases).collect();

    for i in 0..total_records {
        // Generate birth date between 1995 and 2018
        let birth_days = rng.random_range(0..birth_range_days);
        let birth_date = earliest_birth + Duration::days(birth_days as i64);

        // Generate parent ages at birth (typical reproductive age range: 20-45 years)
        let mother_age = rng.random_range(20..46); // 20-45 years
        let father_age = rng.random_range(20..50); // 20-49 years

        let mother_birth_date = birth_date - Duration::days((mother_age * 365) as i64);
        let father_birth_date = birth_date - Duration::days((father_age * 365) as i64);

        // Generate treatment date for cases
        let treatment_date = if treatment_indices.contains(&i) {
            // Treatment should be within the study period and after birth
            let treatment_start = birth_date.max(study_start);
            let treatment_end = (birth_date + Duration::days(6 * 365)).min(study_end);

            if treatment_start <= treatment_end {
                let treatment_days =
                    rng.random_range(0..=(treatment_end - treatment_start).num_days());
                let date = treatment_start + Duration::days(treatment_days);
                date.format("%Y-%m-%d").to_string()
            } else {
                "NA".to_string() // Skip treatment if no valid treatment window
            }
        } else {
            "NA".to_string()
        };

        // Generate unique PNR (format: YYYYMMDD-XXXX)
        let pnr = format!(
            "{}-{:04}",
            birth_date.format("%Y%m%d"),
            rng.random_range(0..10000)
        );

        writer.write_record(&[
            &pnr,
            &birth_date.format("%Y-%m-%d").to_string(),
            &treatment_date,
            &mother_birth_date.format("%Y-%m-%d").to_string(),
            &father_birth_date.format("%Y-%m-%d").to_string(),
        ])?;

        // Print progress every 100,000 records
        if i > 0 && i % 100_000 == 0 {
            println!(
                "Generated {} records ({:.1}%)...",
                i,
                (i as f64 / total_records as f64) * 100.0
            );
        }
    }

    writer.flush()?;
    Ok(())
}

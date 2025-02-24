use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CovariateData {
    pub pnr: String,
    pub date: NaiveDate,
    pub covariate_type: CovariateType,
    pub value: CovariateValue,
}

pub fn save_covariates(
    data: &[CovariateData],
    path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = std::fs::File::create(path)?;
    let writer = std::io::BufWriter::new(file);
    serde_json::to_writer(writer, data)?;
    Ok(())
}

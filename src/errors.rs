use thiserror::Error;

#[derive(Error, Debug)]
pub enum SamplingError {
    #[error("Invalid date format")]
    InvalidDate,
    #[error("No eligible controls found for case")]
    NoEligibleControls,
    #[error("Invalid matching criteria")]
    InvalidCriteria,
    #[error("CSV error: {0}")]
    CsvError(#[from] csv::Error),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum PlottingError {
    #[error("Plotting error: {0}")]
    PlotError(String),
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

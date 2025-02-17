// use ids_core::{sampler, utils};
// use pyo3::prelude::*;
// use std::path::Path;

// #[pyfunction]
// #[pyo3(
//     signature = (
//         input_file,
//         n_controls = 4,
//         birth_window = 30,
//         parent_window = 365,
//         output_dir = "output"
//     )
// )]
// fn sample_controls(
//     input_file: &str,
//     n_controls: usize,
//     birth_window: i64,
//     parent_window: i64,
//     output_dir: &str,
// ) -> PyResult<()> {
//     // Configure logging without a file (will log to stderr)
//     if let Err(e) = utils::configure_logging(None) {
//         eprintln!("Warning: Failed to initialize logger: {}", e);
//     }

//     // Input validation
//     if !Path::new(input_file).exists() {
//         return Err(PyErr::new::<pyo3::exceptions::PyFileNotFoundError, _>(
//             format!("Input file not found: {}", input_file),
//         ));
//     }

//     // Create output directory
//     std::fs::create_dir_all(output_dir).map_err(|e| {
//         PyErr::new::<pyo3::exceptions::PyOSError, _>(format!(
//             "Failed to create output directory: {}",
//             e
//         ))
//     })?;

//     // Load records
//     let records = utils::load_records(input_file).map_err(|e| {
//         PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!("Failed to load records: {}", e))
//     })?;

//     let criteria = utils::MatchingCriteria {
//         birth_date_window: birth_window,
//         parent_date_window: parent_window,
//     };

//     let sampler = sampler::IncidenceDensitySampler::new(records, criteria).map_err(|e| {
//         PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
//             "Failed to initialize sampler: {}",
//             e
//         ))
//     })?;

//     match sampler.sample_controls(n_controls) {
//         Ok(case_control_pairs) => {
//             let matches_path = Path::new(output_dir).join("matched_pairs.csv");
//             sampler
//                 .save_matches_to_csv(&case_control_pairs, &matches_path.to_string_lossy())
//                 .map_err(|e| {
//                     PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
//                         "Failed to save matches: {}",
//                         e
//                     ))
//                 })?;

//             let stats_path = Path::new(output_dir).join("matching_stats.csv");
//             sampler
//                 .save_matching_statistics(&case_control_pairs, &stats_path.to_string_lossy())
//                 .map_err(|e| {
//                     PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
//                         "Failed to save statistics: {}",
//                         e
//                     ))
//                 })?;

//             Ok(())
//         }
//         Err(e) => Err(PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(format!(
//             "Failed to sample controls: {}",
//             e
//         ))),
//     }
// }

// #[pymodule]
// fn ids_rs(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
//     m.add_function(wrap_pyfunction!(sample_controls, m)?)?;
//     Ok(())
// }

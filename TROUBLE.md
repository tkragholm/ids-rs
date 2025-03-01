cargo build
Compiling covariates v0.1.0 (/Users/tobiaskragholm/dev/ids-rs/crates/covariates)
error[E0603]: module `results` is private
--> crates/covariates/src/reporting/structured_output.rs:1:21
|
1 | use crate::balance::results::BalanceResults;
| ^^^^^^^ private module
|
note: the module `results` is defined here
--> crates/covariates/src/balance/mod.rs:5:1
|
5 | mod results;
| ^^^^^^^^^^^^

warning: unused import: `std::io::Error as IoError`
--> crates/covariates/src/reporting/structured_output.rs:6:5
|
6 | use std::io::Error as IoError;
| ^^^^^^^^^^^^^^^^^^^^^^^^^
|
= note: `#[warn(unused_imports)]` on by default

error[E0599]: no variant or associated item named `io_error` found for enum `IdsError` in the current scope
--> crates/covariates/src/reporting/structured_output.rs:56:27
|
56 | ... IdsError::io_error(format!("Failed to create directory {:?}: {}", dir, e))
| ^^^^^^^^ variant or associated item not found in `IdsError`
|
note: if you're trying to build a new `IdsError` consider using one of the following associated functions:
IdsError::invalid_operation
IdsError::missing_data
IdsError::invalid_format
IdsError::invalid_date
and 6 others
--> /Users/tobiaskragholm/dev/ids-rs/crates/types/src/error.rs:60:5
|
60 | pub fn invalid_operation<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
65 | pub fn missing_data<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
70 | pub fn invalid_format<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
75 | pub fn invalid_date<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:197:24
|
197 | record.controls.iter().map(move |control| {
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:222:41
|
222 | let n_controls = record.controls.len();
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:228:50
|
228 | let avg_birth_diff: f64 = record.controls.iter()
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:232:45
|
232 | let max_birth_diff = record.controls.iter()
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:237:51
|
237 | let avg_mother_diff: f64 = record.controls.iter()
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:242:51
|
242 | let avg_father_diff: f64 = record.controls.iter()
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0599]: no variant or associated item named `io_error` found for enum `IdsError` in the current scope
--> crates/covariates/src/reporting/structured_output.rs:426:23
|
426 | IdsError::io_error(format!("Failed to write index.html: {}", e))
| ^^^^^^^^ variant or associated item not found in `IdsError`
|
note: if you're trying to build a new `IdsError` consider using one of the following associated functions:
IdsError::invalid_operation
IdsError::missing_data
IdsError::invalid_format
IdsError::invalid_date
and 6 others
--> /Users/tobiaskragholm/dev/ids-rs/crates/types/src/error.rs:60:5
|
60 | pub fn invalid_operation<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
65 | pub fn missing_data<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
70 | pub fn invalid_format<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
75 | pub fn invalid_date<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no variant or associated item named `io_error` found for enum `IdsError` in the current scope
--> crates/covariates/src/reporting/structured_output.rs:441:23
|
441 | IdsError::io_error(format!("Failed to write CSV data: {}", e))
| ^^^^^^^^ variant or associated item not found in `IdsError`
|
note: if you're trying to build a new `IdsError` consider using one of the following associated functions:
IdsError::invalid_operation
IdsError::missing_data
IdsError::invalid_format
IdsError::invalid_date
and 6 others
--> /Users/tobiaskragholm/dev/ids-rs/crates/types/src/error.rs:60:5
|
60 | pub fn invalid_operation<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
65 | pub fn missing_data<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
70 | pub fn invalid_format<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
75 | pub fn invalid_date<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no variant or associated item named `io_error` found for enum `IdsError` in the current scope
--> crates/covariates/src/reporting/structured_output.rs:662:23
|
662 | IdsError::io_error(format!("Failed to write balance HTML report: {}", e))
| ^^^^^^^^ variant or associated item not found in `IdsError`
|
note: if you're trying to build a new `IdsError` consider using one of the following associated functions:
IdsError::invalid_operation
IdsError::missing_data
IdsError::invalid_format
IdsError::invalid_date
and 6 others
--> /Users/tobiaskragholm/dev/ids-rs/crates/types/src/error.rs:60:5
|
60 | pub fn invalid_operation<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
65 | pub fn missing_data<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
70 | pub fn invalid_format<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
75 | pub fn invalid_date<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0609]: no field `controls` on type `&&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:681:28
|
681 | .filter(|r| !r.controls.is_empty())
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:684:24
|
684 | .map(|r| r.controls.len())
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:695:36
|
695 | for control in &record.controls {
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:711:41
|
711 | let controls_count = record.controls.len();
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0609]: no field `controls` on type `&MatchedPairRecord`
--> crates/covariates/src/reporting/structured_output.rs:713:45
|
713 | let first_control = &record.controls[0];
| ^^^^^^^^ unknown field
|
= note: available fields are: `case_id`, `case_pnr`, `case_birth_date`, `case_treatment_date`, `control_id` ... and 5 others

error[E0599]: no variant or associated item named `io_error` found for enum `IdsError` in the current scope
--> crates/covariates/src/reporting/structured_output.rs:933:23
|
933 | ... IdsError::io_error(format!("Failed to write matching HTML report: {}", e))
| ^^^^^^^^ variant or associated item not found in `IdsError`
|
note: if you're trying to build a new `IdsError` consider using one of the following associated functions:
IdsError::invalid_operation
IdsError::missing_data
IdsError::invalid_format
IdsError::invalid_date
and 6 others
--> /Users/tobiaskragholm/dev/ids-rs/crates/types/src/error.rs:60:5
|
60 | pub fn invalid_operation<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
65 | pub fn missing_data<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
70 | pub fn invalid_format<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
75 | pub fn invalid_date<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error[E0599]: no variant or associated item named `io_error` found for enum `IdsError` in the current scope
--> crates/covariates/src/reporting/structured_output.rs:1041:23
|
1041 | ... IdsError::io_error(format!("Failed to write data quality HTML report: {}", e))
| ^^^^^^^^ variant or associated item not found in `IdsError`
|
note: if you're trying to build a new `IdsError` consider using one of the following associated functions:
IdsError::invalid_operation
IdsError::missing_data
IdsError::invalid_format
IdsError::invalid_date
and 6 others
--> /Users/tobiaskragholm/dev/ids-rs/crates/types/src/error.rs:60:5
|
60 | pub fn invalid_operation<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
65 | pub fn missing_data<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
70 | pub fn invalid_format<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
...
75 | pub fn invalid_date<T: std::fmt::Display>(msg: T) -> Self {
| ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Some errors have detailed explanations: E0599, E0603, E0609.
For more information about an error, try `rustc --explain E0599`.
warning: `covariates` (lib) generated 1 warning
error: could not compile `covariates` (lib) due to 18 previous errors; 1 warning emitted
error: Recipe `build` failed on line 23 with exit code 101

ids-rs on  main [!] is 📦 v0.1.0 via 🦀 v1.84.0 took 2s
❯ just complete-pipeline-small
cargo build --release
warning: variable does not need to be mutable
   --> crates/types/src/storage.rs:323:13
    |
323 |         let mut ind_data = HashMap::new();
    |             ----^^^^^^^^
    |             |
    |             help: remove this `mut`
    |
    = note: `#[warn(unused_mut)]` on by default

warning: variable does not need to be mutable
   --> crates/types/src/storage.rs:324:13
    |
324 |         let mut bef_data = HashMap::new();
    |             ----^^^^^^^^
    |             |
    |             help: remove this `mut`

warning: `types` (lib) generated 2 warnings (run `cargo fix --lib -p types` to apply 2 suggestions)
warning: unused import: `parquet::schema::types::SchemaDescriptor`
 --> crates/loader/src/parquet.rs:7:5
  |
7 | use parquet::schema::types::SchemaDescriptor;
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: `loader` (lib) generated 1 warning (run `cargo fix --lib -p loader` to apply 1 suggestion)
warning: unused import: `warn`
   --> crates/covariates/src/balance/checker.rs:179:32
    |
179 |         use log::{debug, info, warn};
    |                                ^^^^
    |
    = note: `#[warn(unused_imports)]` on by default

warning: unnecessary parentheses around `if` condition
   --> crates/covariates/src/balance/checker.rs:292:36
    |
292 | ...                   if (year != date.year() || month != date.month()) {
    |                          ^                                            ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
292 -                                 if (year != date.year() || month != date.month()) {
292 +                                 if year != date.year() || month != date.month() {
    |

warning: `covariates` (lib) generated 2 warnings (run `cargo fix --lib -p covariates` to apply 2 suggestions)
warning: unused import: `configure_logging`
 --> crates/py/../cli/src/main.rs:7:39
  |
7 |         configure_logging_with_level, configure_logging,
  |                                       ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `i`
  --> crates/py/../cli/src/main.rs:26:10
   |
26 |     for (i, arg) in std::env::args().enumerate() {
   |          ^ help: if this is intentional, prefix it with an underscore: `_i`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: unused import: `configure_logging`
 --> crates/cli/src/main.rs:7:39
  |
7 |         configure_logging_with_level, configure_logging,
  |                                       ^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

warning: unused variable: `i`
  --> crates/cli/src/main.rs:26:10
   |
26 |     for (i, arg) in std::env::args().enumerate() {
   |          ^ help: if this is intentional, prefix it with an underscore: `_i`
   |
   = note: `#[warn(unused_variables)]` on by default

warning: `py` (bin "ids-py") generated 2 warnings (run `cargo fix --bin "ids-py"` to apply 1 suggestion)
warning: `cli` (bin "ids") generated 2 warnings (run `cargo fix --bin "ids"` to apply 1 suggestion)
    Finished `release` profile [optimized] target(s) in 0.12s
RUST_LOG=info target/release/ids generate-registers -o data/registers_small -t 100000 -c 5000 -s 2000 -e 2023
[2025-02-28T14:34:23Z INFO  ids] Created output directories in output

Synthetic Register Data Generation
══════════════════════════════════

Configuration
─────────────
Output directory: data/registers_small
Total records: 100.00K
Case records: 5.00K
Year range: 2000 - 2023
Random seed: None (using system random)

Generating Data
───────────────
[00:00:00] ████████████████████████████████████████ 100000/100000 Family generation completed[00:00:01] ████████████████████████████████████████ 24/24 AKM generation completed[00:00:05] ████████████████████████████████████████ 39/39 BEF generation completed[00:00:00] ████████████████████████████████████████ 24/24 IND generation completed[00:00:00] ████████████████████████████████████████ 2/2 UDDF generation completedGeneration time: 8.222s

Generating Pediatric Data
─────────────────────────
[00:00:00] ████████████████████████████████████████ 100000/100000 Pediatric data generationcompletedPediatric data file: data/registers_small/pediatric.csv
Pediatric generation time: 105ms

Summary
═══════
Output directory: data/registers_small
Records generated: 100.00K
Total execution time: 8.328s
✓ Register data generation completed successfully
RUST_LOG=info target/release/ids sample -i data/registers_small/pediatric.csv -n 4 -b 30 -p365
[2025-02-28T14:34:32Z INFO  ids] Created output directories in output

Incidence Density Sampling
══════════════════════════

Data Validation
───────────────
✓ CSV format validation completed in 11.708518ms
Birth date window: 30 days
Parent date window: 365 days

Data Loading
────────────
Input file: data/registers_small/pediatric.csv
Records loaded: 100.00K
Loading time: 129ms

Sampler Initialization
──────────────────────

Dataset Statistics
══════════════════
│ Total Records: 100.00K
│ Cases: 5.00K (5.0%)
│ Controls: 95.00K (95.0%)
│ Case/Control Ratio: 19.00
└──────────────────────────────

Initialization time: 8ms

Sampling Controls
─────────────────
Requested controls per case: 4

Sampling Controls
=================
│ Total Cases: 5000
│ Batch Size: 1024
  [00:00:00] [############################################] 5/5 chunks (137.8798/s) Complete
Sampling Results:
│ Time Elapsed: 36ms 290us 157ns
│ Total Matches: 4788
│ Speed: 137778.41
└────────────────────────────
Sampling time: 36ms
Matches found: 4788

Saving Results
──────────────
✓ Matches saved to output/matched_pairs.csv
✓ Statistics saved to output/matching_stats.csv

Matching Quality Analysis
─────────────────────────

Matching Quality Report
══════════════════════
│ Matching Rate: 4788/5000 (95.76%)
│ Control Utilization: 9576/14233 (100.00%)
│ Average Controls per Case: 2.97

Birth Date Differences
─────────────────────
  25th percentile: 7 days
  50th percentile: 15 days
  75th percentile: 23 days

Mother Age Differences
─────────────────────
  25th percentile: 15 days
  50th percentile: 30 days
  75th percentile: 350 days

Father Age Differences
─────────────────────
  25th percentile: 15 days
  50th percentile: 30 days
  75th percentile: 350 days

Balance Metrics
──────────────
  Birth Date Balance: 1.663
  Parent Age Balance: 1.085

✓ Quality plots generated in output/plots

Summary
═══════
Input file: data/registers_small/pediatric.csv
Output directory: output
Total execution time: 229ms
✓ Sampling completed successfully
RUST_LOG=warn target/release/ids check-balance -m output/matched_pairs.csv --family-file data/registers_small/family.parquet --akm-dir data/registers_small/akm/ --bef-dir data/registers_small/bef/ --ind-dir data/registers_small/ind/ --uddf-dir data/registers_small/uddf/

Covariate Balance Analysis
══════════════════════════

Loading Matched Pairs
─────────────────────
Matched pairs loaded: 4788
Unique PNRs found: 17987
Loading time: 13ms
ℹ Found family (relative) at /home/tkragholm/Development/ids-rs/data/registers_small/family.parquet (file)
Using custom family file: /home/tkragholm/Development/ids-rs/data/registers_small/family.parquet
ℹ Found akm (relative) at /home/tkragholm/Development/ids-rs/data/registers_small/akm (directory)
Using custom AKM directory: /home/tkragholm/Development/ids-rs/data/registers_small/akm
ℹ Found bef (relative) at /home/tkragholm/Development/ids-rs/data/registers_small/bef (directory)
Using custom BEF directory: /home/tkragholm/Development/ids-rs/data/registers_small/bef
ℹ Found ind (relative) at /home/tkragholm/Development/ids-rs/data/registers_small/ind (directory)
Using custom IND directory: /home/tkragholm/Development/ids-rs/data/registers_small/ind
ℹ Found uddf (relative) at /home/tkragholm/Development/ids-rs/data/registers_small/uddf (directory)
Using custom UDDF directory: /home/tkragholm/Development/ids-rs/data/registers_small/uddf

Loading Register Data
─────────────────────
ℹ No base directory provided, using current directory: /home/tkragholm/Development/ids-rs
ℹ CRITICAL CHECK: Verifying family file at /home/tkragholm/Development/ids-rs/data/registers_small/family.parquet
ℹ Family path exists and is a file - checking access
ℹ Successfully read 16 bytes from family file
ℹ First bytes: 50 41 52 31 15 04 15 b8 a9 81 01 15 b8 a9 81 01
ℹ Testing file with pqrs (shell command)
ℹ pqrs output: {PNR: "030199-0604", BIRTH_DATE: 1999-01-03, FATHER_ID: "110165-0166", FATHER_BIRTH_DATE: 1965-01-11, MOTHER_ID: "110165-9528", MOTHER_BIRTH_DATE: 1965-01-11, FAMILY_ID: "F00000000"}
ℹ Using the following custom register paths:
  - family: /home/tkragholm/Development/ids-rs/data/registers_small/family.parquet
  - uddf: /home/tkragholm/Development/ids-rs/data/registers_small/uddf
ℹ Directory /home/tkragholm/Development/ids-rs/data/registers_small/uddf contains: 202209.parquet, 202009.parquet
  - ind: /home/tkragholm/Development/ids-rs/data/registers_small/ind
ℹ Directory /home/tkragholm/Development/ids-rs/data/registers_small/ind contains: 2013.parquet, 2010.parquet, 2001.parquet, 2012.parquet, 2021.parquet, ...
  - bef: /home/tkragholm/Development/ids-rs/data/registers_small/bef
ℹ Directory /home/tkragholm/Development/ids-rs/data/registers_small/bef contains: 201412.parquet, 202012.parquet, 202206.parquet, 201906.parquet, 202203.parquet, ...
  - akm: /home/tkragholm/Development/ids-rs/data/registers_small/akm
ℹ Directory /home/tkragholm/Development/ids-rs/data/registers_small/akm contains: 2013.parquet, 2010.parquet, 2001.parquet, 2012.parquet, 2021.parquet, ...
Overall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0/24 (0%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ███░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 2/24 (8%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 5/24 (21%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ███████████░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 7/24 (29%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░ 8/24 (33%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ████████████████░░░░░░░░░░░░░░░░░░░░░░░░ 10/24 (42%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ██████████████████░░░░░░░░░░░░░░░░░░░░░░ 11/24 (46%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] █████████████████████░░░░░░░░░░░░░░░░░░░ 13/24 (54%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ███████████████████████░░░░░░░░░░░░░░░░░ 14/24 (58%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] █████████████████████████░░░░░░░░░░░░░░░ 15/24 (62%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ██████████████████████████░░░░░░░░░░░░░░ 16/24 (67%)[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:33Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ██████████████████████████████░░░░░░░░░░ 18/24 (75%)[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ███████████████████████████████░░░░░░░░░ 19/24 (79%)[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] █████████████████████████████████░░░░░░░ 20/24 (83%)[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:00] ███████████████████████████████████░░░░░ 21/24 (88%)[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:01] ████████████████████████████████████░░░░ 22/24 (92%)[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:00] ████████████████████████░░░░░░░░░░░░░░░░ 3/5 Loading BEF data...
BEF Years [00:00:01] ██████████████████████████████████████░░ 23/24 (96%)[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T14:34:34Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields butOverall Progress [00:00:02] ████████████████████████████████████████ 5/5 Loading complete
✓ Successfully loaded register data

Preparing Analysis
──────────────────
Total cases: 4790
Total controls: 14231
│ ID     │ PNR         │ Date       │
├────────┼─────────────┼────────────┼─
│ Case 1 │ 230408-7731 │ 2012-06-23 │
│ Case 2 │ 240703-3375 │ 2007-09-30 │
│ Case 3 │ 070611-1276 │ 2012-10-18 │
│ Case 4 │ 030698-5992 │ 2002-05-30 │
│ Case 5 │ 111202-2386 │ 2008-05-24 │

Calculating Balance
───────────────────
[00:00:00] ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0/3 Processing demographics...[2025-02-28T14:34:34Z WARN  covariates::balance::metrics] ✗ Error accessing covariate for PNR 030698-5992: Invalid format: Invalid FAMILIE_TYPE array type
[2025-02-28T14:34:44Z WARN  covariates::balance::metrics] ✗ Error accessing covariate for PNR 030698-5992: Invalid format: Invalid FAMILIE_TYPE array type
[00:00:10] █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░ 1/3 Processing income...[00:00:35] ██████████████████████████░░░░░░░░░░░░░░ 2/3 Processing education...✗ Failed to calculate balance: Invalid format: Invalid FAMILIE_TYPE array type
Error: InvalidFormat("Invalid FAMILIE_TYPE array type")
error: Recipe `complete-pipeline-small` failed on line 143 with exit code 1

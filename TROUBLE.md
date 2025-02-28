[2025-02-28T13:47:07Z INFO  loader::parquet] Attempting to read parquet file: /home/tkragholm/Development/ids-rs/data/registers/bef/202312.parquet
[2025-02-28T13:47:07Z WARN  loader::parquet] Schema mismatch: Arrow schema has 27 fields but Parquet schema has only 17 fields. Using only common fields.
[2025-02-28T13:47:07Z INFO  loader::reader] Successfully read 62 batches from /home/tkragholm/Development/ids-rs/data/registers/bef/202312.parquet
CRITICAL DEBUG: Successfully read 62 batches
Overall Progress [00:00:10] ████████████████████████████████░░░░░░░░ 4/5 Loading UDDF data...
UDDF Periods [00:00:00] ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0/2 (0%)[2025-02-28T13:47:07Z INFO  loader::reader] CustomPathReader attempting to read /home/tkragholm/Development/ids-rs/data/registers/uddf/202009.parquet
CRITICAL DEBUG: CustomPathReader attempting to read /home/tkragholm/Development/ids-rs/data/registers/uddf/202009.parquet
[2025-02-28T13:47:07Z INFO  loader::reader] File size: 18048272 bytes
[2025-02-28T13:47:07Z INFO  loader::reader] Found Parquet header signature
[2025-02-28T13:47:07Z INFO  loader::reader] Successfully read 16 more bytes from middle of file
CRITICAL DEBUG: Read 16 bytes from middle: 15 04 15 f4 a9 81 01 15 f4 a9 81 01 4c 15 cc cf
[2025-02-28T13:47:07Z INFO  loader::reader] File exists and is readable, loading parquet from /home/tkragholm/Development/ids-rs/data/registers/uddf/202009.parquet
CRITICAL DEBUG: Attempting to read parquet data from /home/tkragholm/Development/ids-rs/data/registers/uddf/202009.parquet
[2025-02-28T13:47:07Z INFO  loader::parquet] Attempting to read parquet file: /home/tkragholm/Development/ids-rs/data/registers/uddf/202009.parquet
[2025-02-28T13:47:07Z INFO  loader::reader] Successfully read 62 batches from /home/tkragholOverall Progress [00:00:10] ████████████████████████████████░░░░░░░░ 4/5 Loading UDDF data...
UDDF Periods [00:00:00] ████████████████████░░░░░░░░░░░░░░░░░░░░ 1/2 (50%)[2025-02-28T13:47:07Z INFO  loader] Loaded 62 UDDF batches for period 202009
[2025-02-28T13:47:07Z INFO  loader::reader] CustomPathReader attempting to read /home/tkragholm/Development/ids-rs/data/registers/uddf/202209.parquet
CRITICAL DEBUG: CustomPathReader attempting to read /home/tkragholm/Development/ids-rs/data/registers/uddf/202209.parquet
[2025-02-28T13:47:07Z INFO  loader::reader] File size: 18507864 bytes
[2025-02-28T13:47:07Z INFO  loader::reader] Found Parquet header signature
[2025-02-28T13:47:07Z INFO  loader::reader] Successfully read 16 more bytes from middle of file
CRITICAL DEBUG: Read 16 bytes from middle: 15 04 15 f4 a9 81 01 15 f4 a9 81 01 4c 15 cc cf
[2025-02-28T13:47:07Z INFO  loader::reader] File exists and is readable, loading parquet from /home/tkragholm/Development/ids-rs/data/registers/uddf/202209.parquet
CRITICAL DEBUG: Attempting to read parquet data from /home/tkragholm/Development/ids-rs/data/registers/uddf/202209.parquet
[2025-02-28T13:47:07Z INFO  loader::parquet] Attempting to read parquet file: /home/tkragholm/Development/ids-rs/data/registers/uddf/202209.parquet
[2025-02-28T13:47:07Z INFO  loader::reader] Successfully read 62 batches from /home/tkragholOverall Progress [00:00:10] ████████████████████████████████░░░░░░░░ 4/5 Loading UDDF data...
Overall Progress [00:00:10] ████████████████████████████████████████ 5/5 Loading complete
UDDF Periods [00:00:00] ████████████████████████████████████████ 2/2 (100%)[2025-02-28T13:47:07Z INFO  loader] Successfully loaded data with custom reader
[2025-02-28T13:47:07Z INFO  loader] Successfully created Arrow backend
✓ Successfully loaded register data

Preparing Analysis
──────────────────
[2025-02-28T13:47:08Z INFO  ids] Collected 49988 unique case IDs
Total cases: 50108
Total controls: 199892
│ ID     │ PNR         │ Date       │
├────────┼─────────────┼────────────┼─
│ Case 1 │ 010214-0764 │ 2018-12-28 │
│ Case 2 │ 041002-2272 │ 2003-04-27 │
│ Case 3 │ 141100-3297 │ 2005-03-21 │
│ Case 4 │ 270403-2266 │ 2006-12-04 │
│ Case 5 │ 131118-6979 │ 2018-12-31 │

Calculating Balance
───────────────────
[00:00:00] ░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░░ 0/3 Processing demographics...[2025-02-28T13:47:08Z WARN  covariates::balance::metrics] ✗ Error accessing covariate for PNR 141100-3297: Invalid format: Invalid FAMILIE_TYPE array type
[2025-02-28T13:54:08Z WARN  covariates::balance::metrics] ✗ Error accessing covariate for PNR 141100-3297: Invalid format: Invalid FAMILIE_TYPE array type
[00:09:18] █████████████░░░░░░░░░░░░░░░░░░░░░░░░░░░ 1/3 Processing income...Numeric Values [00:00:00] ████████████████████████████████████████      13/13      (100%) Processing numeric covariates...
⏱️  ETA: 00:00:00 | 🚀 113.7233/s records/sec | 📊 Processing: Income

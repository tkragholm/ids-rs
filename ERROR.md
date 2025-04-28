❯ ./target/release/ids-rs population-scd --population ../../TEST_OUT/population.parquet --lpr ../../generated_data/parquet/ --output ../../TEST_OUT/ -vvv
[2025-04-28T11:16:17Z DEBUG ids_rs::cli::commands] Log level set to: DEBUG

Identifying Children with Severe Chronic Disease in Population
==============================================================
Population Data: ../../TEST_OUT/population.parquet
LPR Data: ../../generated_data/parquet/
Output Directory: ../../TEST_OUT/
Include LPR2: true
Include LPR3: true
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loading population data from: ../../TEST_OUT/population.parquet
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loaded 985793 population records
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Searching for LPR files in: ../../generated_data/parquet/
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Found LPR files:
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler]   LPR_ADM: ../../generated_data/parquet/lpr_adm
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler]   LPR_DIAG: ../../generated_data/parquet/lpr_diag
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler]   LPR_BES: ../../generated_data/parquet/lpr_bes
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loading LPR data...
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loading LPR_ADM data...
...
[2025-04-28T11:16:17Z WARN  ids_rs::schema::parquet_utils] Field VERSION not found in parquet file, skipping
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loaded 2947 LPR_ADM batches
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loading LPR_DIAG data...
...
[2025-04-28T11:16:17Z WARN  ids_rs::schema::parquet_utils] Field VERSION not found in parquet file, skipping
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loaded 6028 LPR_DIAG batches
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loading LPR_BES data...
[2025-04-28T11:16:17Z WARN  ids_rs::schema::parquet_utils] Field LEVERANCEDATO not found in parquet file, skipping
...
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Loaded 6027 LPR_BES batches
[2025-04-28T11:16:17Z INFO  ids_rs::commands::population_scd::handler] Processing LPR data and identifying SCD in population...
[2025-04-28T11:16:18Z DEBUG ids_rs::algorithm::lpr] D_INDDTO column type: Utf8
[2025-04-28T11:16:18Z DEBUG ids_rs::algorithm::lpr] D_UDDTO column type: Utf8
[2025-04-28T11:16:21Z DEBUG ids_rs::algorithm::lpr] D_AMBDTO column type: Utf8
[2025-04-28T11:16:39Z INFO  ids_rs::algorithm::population_scd] Processed LPR data: 3009472 rows
[2025-04-28T11:18:16Z INFO  ids_rs::algorithm::population_scd] SCD analysis complete: 1164170 patient records
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler] Population SCD analysis complete:
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   Total children in population: 985793
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   Children with SCD: 122220 (12.40%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler] SCD by disease category:
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   respiratory: 7573 (0.77%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   gastrointestinal: 3599 (0.37%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   immune_system: 7732 (0.78%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   neurological: 21399 (2.17%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   renal: 16522 (1.68%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   musculoskeletal: 8920 (0.90%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   congenital: 33920 (3.44%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   endocrine: 17677 (1.79%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   blood_disorders: 15439 (1.57%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler]   cardiovascular: 3459 (0.35%)
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler] Extracting children with SCD...
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler] Extracted 122220 children with SCD
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler] Saved population with SCD indicators to: ../../TEST_OUT/population_scd.parquet
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler] Saved SCD children to: ../../TEST_OUT/scd_children.parquet
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler] Saved population SCD summary to: ../../TEST_OUT/population_scd_summary.csv
[2025-04-28T11:18:19Z INFO  ids_rs::commands::population_scd::handler] Population SCD command completed successfully
✓ Population SCD analysis completed

[2025-05-02T11:05:49Z INFO  ids_rs::commands::population::handler] Saving summary reports to: "../../TEST_OUT/01_population/reports"
[2025-05-02T11:05:49Z DEBUG ids_rs::utils::reports::population] Processing birth date distribution: 20% complete
[2025-05-02T11:05:49Z DEBUG ids_rs::utils::reports::population] Processing birth date distribution: 40% complete
[2025-05-02T11:05:50Z DEBUG ids_rs::utils::reports::population] Processing birth date distribution: 60% complete
[2025-05-02T11:05:50Z DEBUG ids_rs::utils::reports::population] Processing birth date distribution: 80% complete
[2025-05-02T11:05:50Z INFO  ids_rs::commands::population::handler] Population generation completed successfully
[2025-05-02T11:05:50Z INFO  ids_rs::commands::study_design::handler] Step 2: Identifying SCD in Population
[2025-05-02T11:05:50Z INFO  ids_rs::commands::population_scd::handler] Loading population data from: ../../TEST_OUT/01_population/population.parquet
[2025-05-02T11:05:50Z DEBUG datafusion_functions::crypto] crypto functions disabled
[2025-05-02T11:05:50Z DEBUG sqlparser::parser] Parsing sql '?table?'...
Error: External error: Parquet error: External: Object at location /%2E%2E/%2E%2E/TEST_OUT/01_population/population.parquet not found: No such file or directory (os error 2)

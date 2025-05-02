[2025-05-02 17:20:21] INFO  CORE : Saving summary reports to: "../../TEST_OUT/01_population/reports"
[2025-05-02 17:20:22] DEBUG CORE : Processing birth date distribution: 20% complete
[2025-05-02 17:20:22] DEBUG CORE : Processing birth date distribution: 40% complete
[2025-05-02 17:20:22] DEBUG CORE : Processing birth date distribution: 60% complete
[2025-05-02 17:20:22] DEBUG CORE : Processing birth date distribution: 80% complete
[2025-05-02 17:20:23] INFO  CORE : Population generation completed successfully
[2025-05-02 17:20:23] INFO  CORE : Step 2: Identifying SCD in Population
[2025-05-02 17:20:23] INFO  CORE : Loading population data from: ../../TEST_OUT/01_population/population.parquet
[2025-05-02 17:20:23] DEBUG CORE : Path resolution (canonicalized): original='../../TEST_OUT/01_population/population.parquet' resolved='/home/tkragholm/TEST_OUT/01_population/population.parquet'
[2025-05-02 17:20:23] DEBUG CORE : Loading parquet from absolute path: /home/tkragholm/TEST_OUT/01_population/population.parquet
[2025-05-02 17:20:23] DEBUG CORE : crypto functions disabled
[2025-05-02 17:20:23] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/TEST_OUT/01_population/population.parquet' resolved='/home/tkragholm/TEST_OUT/01_population/population.parquet'
[2025-05-02 17:20:23] DEBUG CORE : Reading parquet from absolute path: /home/tkragholm/TEST_OUT/01_population/population.parquet
[2025-05-02 17:20:23] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/TEST_OUT/01_population/population.parquet' resolved='/home/tkragholm/TEST_OUT/01_population/population.parquet'
[2025-05-02 17:20:23] DEBUG CORE : Discovering files using absolute path: /home/tkragholm/TEST_OUT/01_population/population.parquet
[2025-05-02 17:20:23] DEBUG CORE : Path is a single parquet file: /home/tkragholm/TEST_OUT/01_population/population.parquet
[2025-05-02 17:20:23] INFO  CORE : Successfully discovered 1 parquet files
[2025-05-02 17:20:23] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/TEST_OUT/01_population/population.parquet' resolved='/home/tkragholm/TEST_OUT/01_population/population.parquet'
[2025-05-02 17:20:23] DEBUG CORE : Using absolute path for schema inference: /home/tkragholm/TEST_OUT/01_population/population.parquet
[2025-05-02 17:20:23] DEBUG CORE : Parsing sql '?table?'...
[2025-05-02 17:20:23] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/TEST_OUT/01_population/population.parquet' resolved='/home/tkragholm/TEST_OUT/01_population/population.parquet'
[2025-05-02 17:20:23] DEBUG CORE : Adding file to execution plan: /home/tkragholm/TEST_OUT/01_population/population.parquet
[2025-05-02 17:20:24] INFO  CORE : Loaded 985793 population records
[2025-05-02 17:20:24] INFO  CORE : Searching for LPR files in: ../../generated_data/parquet/
[2025-05-02 17:20:24] INFO  LPR  [99b7fd5d][find_lpr_paths]: Operation started
[2025-05-02 17:20:24] DEBUG CORE [99b7fd5d]: Path resolution (canonicalized): original='../../generated_data/parquet/' resolved='/home/tkragholm/generated_data/parquet'
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_files]: ENTER: ("/home/tkragholm/generated_data/parquet")
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: STEP 1: Checking if current path is a specific LPR directory
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: STEP 2: Checking parent directory if needed
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: STEP 3: Checking subdirectories
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/ind
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Found LPR v2 diagnosis path: /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/vnds
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Found LPR v2 procedure path: /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/uddf
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/mfr
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/bef
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Found LPR v2 admin path: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: Checking LPR subdirectory: /home/tkragholm/generated_data/parquet/akm
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_paths]: STEP 4: Summarizing findings
[2025-05-02 17:20:24] INFO  LPR  [99b7fd5d][find_lpr_paths]: LPR paths found: admin=/home/tkragholm/generated_data/parquet/lpr_adm, diag=/home/tkragholm/generated_data/parquet/lpr_diag, proc=/home/tkragholm/generated_data/parquet/lpr_bes, kontakter=None, diagnoser=None, procedurer=None
[2025-05-02 17:20:24] DEBUG LPR  [99b7fd5d][find_lpr_files]: EXIT: "Complete"
[2025-05-02 17:20:24] INFO  LPR  [99b7fd5d][find_lpr_paths]: Operation completed successfully
[2025-05-02 17:20:24] INFO  CORE : Found LPR files:
[2025-05-02 17:20:24] INFO  CORE :   LPR_ADM: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] INFO  CORE :   LPR_DIAG: /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:24] INFO  CORE :   LPR_BES: /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:24] INFO  CORE : Loading LPR data...
[2025-05-02 17:20:24] INFO  CORE : Loading LPR_ADM data...
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] ENTER Lpr2Register::load with base_path: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] DEBUG CORE : crypto functions disabled
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Got schema with 27 fields
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Finding LPR v2 file paths from base: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_adm' resolved='/home/tkragholm/generated_data/parquet/lpr_adm'
[2025-05-02 17:20:24] DEBUG LPR  [registry]: LprRegistry::find_files with absolute path: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] INFO  LPR  [4b87d72a][find_lpr_paths]: Operation started
[2025-05-02 17:20:24] DEBUG CORE [4b87d72a]: Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_adm' resolved='/home/tkragholm/generated_data/parquet/lpr_adm'
[2025-05-02 17:20:24] DEBUG LPR  [4b87d72a][find_lpr_files]: ENTER: ("/home/tkragholm/generated_data/parquet/lpr_adm")
[2025-05-02 17:20:24] DEBUG LPR  [4b87d72a][find_lpr_paths]: STEP 1: Checking if current path is a specific LPR directory
[2025-05-02 17:20:24] DEBUG LPR  [4b87d72a][find_lpr_paths]: Found LPR v2 admin path (direct): /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] DEBUG LPR  [4b87d72a][find_lpr_paths]: STEP 2: Checking parent directory if needed
[2025-05-02 17:20:24] DEBUG LPR  [4b87d72a][find_lpr_paths]: STEP 3: Checking subdirectories
[2025-05-02 17:20:24] DEBUG LPR  [4b87d72a][find_lpr_paths]: STEP 4: Summarizing findings
[2025-05-02 17:20:24] INFO  LPR  [4b87d72a][find_lpr_paths]: LPR paths found: admin=/home/tkragholm/generated_data/parquet/lpr_adm, diag=None, proc=None, kontakter=None, diagnoser=None, procedurer=None
[2025-05-02 17:20:24] DEBUG LPR  [4b87d72a][find_lpr_files]: EXIT: "Complete"
[2025-05-02 17:20:24] INFO  LPR  [4b87d72a][find_lpr_paths]: Operation completed successfully
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Found admin_path: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] LPR paths found: admin_path=Some("/home/tkragholm/generated_data/parquet/lpr_adm"), diag_path=None, proc_path=None
[2025-05-02 17:20:24] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_adm' resolved='/home/tkragholm/generated_data/parquet/lpr_adm'
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Loading LPR2 admin data from: /home/tkragholm/generated_data/parquet/lpr_adm (resolved from /home/tkragholm/generated_data/parquet/lpr_adm)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Listing contents of admin directory: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2013.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2010.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2012.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2021.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2019.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2016.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2011.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2022.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2020.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2015.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2014.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2017.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2018.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet (is_dir: false)
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Registering parquet directory as table 'lpr2_admin_abe03bbf' from: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:24] DEBUG CORE : Parsing sql 'lpr2_admin_abe03bbf'...
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Successfully registered parquet directory as table 'lpr2_admin_abe03bbf'
[2025-05-02 17:20:24] DEBUG CORE : Parsing sql 'lpr2_admin_abe03bbf'...
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Successfully got table 'lpr2_admin_abe03bbf' as DataFrame
[2025-05-02 17:20:24] INFO  CORE : [DIAG-abe03bbf] Collecting admin data records...
[2025-05-02 17:20:24] DEBUG CORE : resolve_grouping_function:
TableScan: lpr2_admin_abe03bbf

[2025-05-02 17:20:24] DEBUG CORE : type_coercion:
TableScan: lpr2_admin_abe03bbf

[2025-05-02 17:20:24] DEBUG CORE : Final analyzed plan:
TableScan: lpr2_admin_abe03bbf

[2025-05-02 17:20:24] DEBUG CORE : Analyzer took 0 ms
[2025-05-02 17:20:24] DEBUG CORE : Optimizer input (pass 0):
TableScan: lpr2_admin_abe03bbf

[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 0)
[2025-05-02 17:20:24] DEBUG CORE : optimize_projections:
TableScan: lpr2_admin_abe03bbf projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:24] DEBUG CORE : Optimized plan (pass 0):
TableScan: lpr2_admin_abe03bbf projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:24] DEBUG CORE : Optimizer input (pass 1):
TableScan: lpr2_admin_abe03bbf projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 1)
[2025-05-02 17:20:24] DEBUG CORE : optimize_projections:
TableScan: lpr2_admin_abe03bbf projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:24] DEBUG CORE : Optimized plan (pass 1):
TableScan: lpr2_admin_abe03bbf projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:24] DEBUG CORE : optimizer pass 1 did not make changes
[2025-05-02 17:20:24] DEBUG CORE : Final optimized plan:
TableScan: lpr2_admin_abe03bbf projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:24] DEBUG CORE : Optimizer took 2 ms
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:24] DEBUG CORE : Input physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet], ...]}, projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION], file_type=parquet


[2025-05-02 17:20:24] DEBUG CORE : Optimized physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet:0..3045996, home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet:0..3081988, home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet:0..188435], [home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet:188435..3115074, home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet:0..3156140, home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet:0..233640], [home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet:233640..3188669, home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet:0..3228738, home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet:0..132652], [home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet:132652..3263970, home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet:0..3185101], [home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet:3185101..3297655, home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet:0..3334471, home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet:0..2869394], ...]}, projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION], file_type=parquet


[2025-05-02 17:20:27] INFO  CORE : [DIAG-abe03bbf] Collected 381 admin data batches with 3009472 total records
[2025-05-02 17:20:27] INFO  CORE : [DIAG-abe03bbf] No diagnosis or procedure paths found, returning admin data only
[2025-05-02 17:20:27] INFO  CORE : Loaded 381 LPR_ADM batches
[2025-05-02 17:20:27] INFO  CORE : Loading LPR_DIAG data...
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] ENTER Lpr2Register::load with base_path: /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:27] DEBUG CORE : crypto functions disabled
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Got schema with 27 fields
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Finding LPR v2 file paths from base: /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:27] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_diag' resolved='/home/tkragholm/generated_data/parquet/lpr_diag'
[2025-05-02 17:20:27] DEBUG LPR  [registry]: LprRegistry::find_files with absolute path: /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:27] INFO  LPR  [0805ea4b][find_lpr_paths]: Operation started
[2025-05-02 17:20:27] DEBUG CORE [0805ea4b]: Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_diag' resolved='/home/tkragholm/generated_data/parquet/lpr_diag'
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_files]: ENTER: ("/home/tkragholm/generated_data/parquet/lpr_diag")
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_paths]: STEP 1: Checking if current path is a specific LPR directory
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_paths]: Found LPR v2 diagnosis path (direct): /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_paths]: STEP 2: Checking parent directory if needed
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_paths]: Searching parent directory for admin data: /home/tkragholm/generated_data/parquet
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_paths]: Found LPR v2 admin path in parent directory: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_paths]: STEP 3: Checking subdirectories
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_paths]: STEP 4: Summarizing findings
[2025-05-02 17:20:27] INFO  LPR  [0805ea4b][find_lpr_paths]: LPR paths found: admin=/home/tkragholm/generated_data/parquet/lpr_adm, diag=/home/tkragholm/generated_data/parquet/lpr_diag, proc=None, kontakter=None, diagnoser=None, procedurer=None
[2025-05-02 17:20:27] DEBUG LPR  [0805ea4b][find_lpr_files]: EXIT: "Complete"
[2025-05-02 17:20:27] INFO  LPR  [0805ea4b][find_lpr_paths]: Operation completed successfully
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Found admin_path: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Found diag_path: /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] LPR paths found: admin_path=Some("/home/tkragholm/generated_data/parquet/lpr_adm"), diag_path=Some("/home/tkragholm/generated_data/parquet/lpr_diag"), proc_path=None
[2025-05-02 17:20:27] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_adm' resolved='/home/tkragholm/generated_data/parquet/lpr_adm'
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Loading LPR2 admin data from: /home/tkragholm/generated_data/parquet/lpr_adm (resolved from /home/tkragholm/generated_data/parquet/lpr_adm)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Listing contents of admin directory: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2013.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2010.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2012.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2021.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2019.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2016.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2011.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2022.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2020.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2015.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2014.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2017.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2018.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet (is_dir: false)
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Registering parquet directory as table 'lpr2_admin_5b2b8be7' from: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:27] DEBUG CORE : Parsing sql 'lpr2_admin_5b2b8be7'...
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Successfully registered parquet directory as table 'lpr2_admin_5b2b8be7'
[2025-05-02 17:20:27] DEBUG CORE : Parsing sql 'lpr2_admin_5b2b8be7'...
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Successfully got table 'lpr2_admin_5b2b8be7' as DataFrame
[2025-05-02 17:20:27] INFO  CORE : [DIAG-5b2b8be7] Collecting admin data records...
[2025-05-02 17:20:27] DEBUG CORE : resolve_grouping_function:
TableScan: lpr2_admin_5b2b8be7

[2025-05-02 17:20:27] DEBUG CORE : type_coercion:
TableScan: lpr2_admin_5b2b8be7

[2025-05-02 17:20:27] DEBUG CORE : Final analyzed plan:
TableScan: lpr2_admin_5b2b8be7

[2025-05-02 17:20:27] DEBUG CORE : Analyzer took 0 ms
[2025-05-02 17:20:27] DEBUG CORE : Optimizer input (pass 0):
TableScan: lpr2_admin_5b2b8be7

[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 0)
[2025-05-02 17:20:27] DEBUG CORE : optimize_projections:
TableScan: lpr2_admin_5b2b8be7 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:27] DEBUG CORE : Optimized plan (pass 0):
TableScan: lpr2_admin_5b2b8be7 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:27] DEBUG CORE : Optimizer input (pass 1):
TableScan: lpr2_admin_5b2b8be7 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 1)
[2025-05-02 17:20:27] DEBUG CORE : optimize_projections:
TableScan: lpr2_admin_5b2b8be7 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:27] DEBUG CORE : Optimized plan (pass 1):
TableScan: lpr2_admin_5b2b8be7 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:27] DEBUG CORE : optimizer pass 1 did not make changes
[2025-05-02 17:20:27] DEBUG CORE : Final optimized plan:
TableScan: lpr2_admin_5b2b8be7 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:27] DEBUG CORE : Optimizer took 2 ms
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:27] DEBUG CORE : Input physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet], ...]}, projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION], file_type=parquet


[2025-05-02 17:20:27] DEBUG CORE : Optimized physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet:0..3045996, home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet:0..3081988, home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet:0..188435], [home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet:188435..3115074, home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet:0..3156140, home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet:0..233640], [home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet:233640..3188669, home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet:0..3228738, home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet:0..132652], [home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet:132652..3263970, home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet:0..3185101], [home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet:3185101..3297655, home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet:0..3334471, home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet:0..2869394], ...]}, projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION], file_type=parquet


[2025-05-02 17:20:30] INFO  CORE : [DIAG-5b2b8be7] Collected 381 admin data batches with 3009472 total records
[2025-05-02 17:20:30] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_diag' resolved='/home/tkragholm/generated_data/parquet/lpr_diag'
[2025-05-02 17:20:30] INFO  CORE : [DIAG-5b2b8be7] Loading diagnosis data from: /home/tkragholm/generated_data/parquet/lpr_diag
[2025-05-02 17:20:30] DEBUG CORE : Parsing sql 'lpr2_diag_5b2b8be7'...
[2025-05-02 17:20:30] INFO  CORE : [DIAG-5b2b8be7] Successfully registered diagnosis data as table 'lpr2_diag_5b2b8be7'
[2025-05-02 17:20:30] DEBUG CORE : Parsing sql 'lpr2_diag_5b2b8be7'...
[2025-05-02 17:20:30] DEBUG CORE : resolve_grouping_function:
TableScan: lpr2_diag_5b2b8be7

[2025-05-02 17:20:30] DEBUG CORE : type_coercion:
TableScan: lpr2_diag_5b2b8be7

[2025-05-02 17:20:30] DEBUG CORE : Final analyzed plan:
TableScan: lpr2_diag_5b2b8be7

[2025-05-02 17:20:30] DEBUG CORE : Analyzer took 0 ms
[2025-05-02 17:20:30] DEBUG CORE : Optimizer input (pass 0):
TableScan: lpr2_diag_5b2b8be7

[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 0)
[2025-05-02 17:20:30] DEBUG CORE : optimize_projections:
TableScan: lpr2_diag_5b2b8be7 projection=[C_DIAG, C_DIAGTYPE, C_TILDIAG, RECNUM]

[2025-05-02 17:20:30] DEBUG CORE : Optimized plan (pass 0):
TableScan: lpr2_diag_5b2b8be7 projection=[C_DIAG, C_DIAGTYPE, C_TILDIAG, RECNUM]

[2025-05-02 17:20:30] DEBUG CORE : Optimizer input (pass 1):
TableScan: lpr2_diag_5b2b8be7 projection=[C_DIAG, C_DIAGTYPE, C_TILDIAG, RECNUM]

[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 1)
[2025-05-02 17:20:30] DEBUG CORE : optimize_projections:
TableScan: lpr2_diag_5b2b8be7 projection=[C_DIAG, C_DIAGTYPE, C_TILDIAG, RECNUM]

[2025-05-02 17:20:30] DEBUG CORE : Optimized plan (pass 1):
TableScan: lpr2_diag_5b2b8be7 projection=[C_DIAG, C_DIAGTYPE, C_TILDIAG, RECNUM]

[2025-05-02 17:20:30] DEBUG CORE : optimizer pass 1 did not make changes
[2025-05-02 17:20:30] DEBUG CORE : Final optimized plan:
TableScan: lpr2_diag_5b2b8be7 projection=[C_DIAG, C_DIAGTYPE, C_TILDIAG, RECNUM]

[2025-05-02 17:20:30] DEBUG CORE : Optimizer took 1 ms
[2025-05-02 17:20:30] DEBUG CORE : Input physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_diag/2000.parquet, home/tkragholm/generated_data/parquet/lpr_diag/2001.parquet], [home/tkragholm/generated_data/parquet/lpr_diag/2002.parquet, home/tkragholm/generated_data/parquet/lpr_diag/2003.parquet], [home/tkragholm/generated_data/parquet/lpr_diag/2004.parquet, home/tkragholm/generated_data/parquet/lpr_diag/2005.parquet], [home/tkragholm/generated_data/parquet/lpr_diag/2006.parquet, home/tkragholm/generated_data/parquet/lpr_diag/2007.parquet], [home/tkragholm/generated_data/parquet/lpr_diag/2008.parquet, home/tkragholm/generated_data/parquet/lpr_diag/2009.parquet], ...]}, projection=[C_DIAG, C_DIAGTYPE, C_TILDIAG, RECNUM], file_type=parquet


[2025-05-02 17:20:30] DEBUG CORE : Optimized physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_diag/2000.parquet:0..1772923, home/tkragholm/generated_data/parquet/lpr_diag/2001.parquet:0..1629284], [home/tkragholm/generated_data/parquet/lpr_diag/2001.parquet:1629284..1776166, home/tkragholm/generated_data/parquet/lpr_diag/2002.parquet:0..1774466, home/tkragholm/generated_data/parquet/lpr_diag/2003.parquet:0..1480859], [home/tkragholm/generated_data/parquet/lpr_diag/2003.parquet:1480859..1773079, home/tkragholm/generated_data/parquet/lpr_diag/2004.parquet:0..1773207, home/tkragholm/generated_data/parquet/lpr_diag/2005.parquet:0..1336780], [home/tkragholm/generated_data/parquet/lpr_diag/2005.parquet:1336780..1773894, home/tkragholm/generated_data/parquet/lpr_diag/2006.parquet:0..1773333, home/tkragholm/generated_data/parquet/lpr_diag/2007.parquet:0..1191760], [home/tkragholm/generated_data/parquet/lpr_diag/2007.parquet:1191760..1774200, home/tkragholm/generated_data/parquet/lpr_diag/2008.parquet:0..1779358, home/tkragholm/generated_data/parquet/lpr_diag/2009.parquet:0..1040409], ...]}, projection=[C_DIAG, C_DIAGTYPE, C_TILDIAG, RECNUM], file_type=parquet


[2025-05-02 17:20:31] INFO  CORE : [DIAG-5b2b8be7] Collected 759 diagnosis data batches with 6164145 total records
[2025-05-02 17:20:31] INFO  CORE : [DIAG-5b2b8be7] Joining admin data with diagnosis and/or procedure data
[2025-05-02 17:20:31] DEBUG DATA [batches_to_dataframe]: ENTER: (381)
[2025-05-02 17:20:31] DEBUG DATA [dataframe]: Creating DataFrame with schema: Schema { fields: [Field { name: "PNR", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_ADIAG", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_AFD", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_HAFD", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_HENM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_HSGH", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_INDM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_KOM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_KONTAARS", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_PATTYPE", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_SGH", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_SPEC", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_UDM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "CPRTJEK", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "CPRTYPE", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false,metadata: {} }, Field { name: "D_HENDTO", data_type: Date32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "D_INDDTO", data_type: Date32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "D_UDDTO", data_type: Date32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "K_AFD", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "RECNUM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_ALDDG", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_ALDER", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_INDMINUT", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_INDTIME", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_SENGDAGE", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_UDTIME", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "VERSION", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }], metadata: {} }
[2025-05-02 17:20:31] DEBUG DATA [dataframe]: STEP 1: Creating memory table
[2025-05-02 17:20:31] DEBUG DATA [dataframe]: STEP 2: Registering table in context as temp_table_4e830cd9a7514c6eb232544654dfe96d
[2025-05-02 17:20:31] DEBUG CORE : Parsing sql 'temp_table_4e830cd9a7514c6eb232544654dfe96d'...
[2025-05-02 17:20:31] DEBUG DATA [dataframe]: STEP 3: Converting table to DataFrame
[2025-05-02 17:20:31] DEBUG CORE : Parsing sql 'temp_table_4e830cd9a7514c6eb232544654dfe96d'...
[2025-05-02 17:20:31] INFO  DATA [dataframe]: Successfully created DataFrame
[2025-05-02 17:20:31] DEBUG DATA [batches_to_dataframe]: EXIT: "Success"
[2025-05-02 17:20:31] DEBUG CORE : [DIAG-5b2b8be7] Created admin DataFrame
[2025-05-02 17:20:31] DEBUG DATA [batches_to_dataframe]: ENTER: (759)
[2025-05-02 17:20:31] DEBUG DATA [dataframe]: Creating DataFrame with schema: Schema { fields: [Field { name: "C_DIAG", data_type: Utf8View, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_DIAGTYPE", data_type: Utf8View, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_TILDIAG", data_type: Utf8View, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "RECNUM", data_type: Utf8View, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }], metadata: {} }
[2025-05-02 17:20:31] DEBUG DATA [dataframe]: STEP 1: Creating memory table
[2025-05-02 17:20:31] DEBUG DATA [dataframe]: STEP 2: Registering table in context as temp_table_287dd08181d94a11805fb19aa48634ad
[2025-05-02 17:20:31] DEBUG CORE : Parsing sql 'temp_table_287dd08181d94a11805fb19aa48634ad'...
[2025-05-02 17:20:31] DEBUG DATA [dataframe]: STEP 3: Converting table to DataFrame
[2025-05-02 17:20:31] DEBUG CORE : Parsing sql 'temp_table_287dd08181d94a11805fb19aa48634ad'...
[2025-05-02 17:20:31] INFO  DATA [dataframe]: Successfully created DataFrame
[2025-05-02 17:20:31] DEBUG DATA [batches_to_dataframe]: EXIT: "Success"
[2025-05-02 17:20:31] DEBUG CORE : [DIAG-5b2b8be7] Created diagnosis DataFrame
[2025-05-02 17:20:31] DEBUG CORE : Parsing sql 'RECNUM'...
[2025-05-02 17:20:31] DEBUG CORE : Parsing sql 'RECNUM'...
[2025-05-02 17:20:31] WARN  CORE : [DIAG-5b2b8be7] Failed to join admin and diagnosis data: Schema error: No field named recnum. Valid fields are temp_table_4e830cd9a7514c6eb232544654dfe96d."PNR", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_ADIAG", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_AFD", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_HAFD", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_HENM", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_HSGH", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_INDM", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_KOM", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_KONTAARS", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_PATTYPE", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_SGH", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_SPEC", temp_table_4e830cd9a7514c6eb232544654dfe96d."C_UDM", temp_table_4e830cd9a7514c6eb232544654dfe96d."CPRTJEK", temp_table_4e830cd9a7514c6eb232544654dfe96d."CPRTYPE", temp_table_4e830cd9a7514c6eb232544654dfe96d."D_HENDTO", temp_table_4e830cd9a7514c6eb232544654dfe96d."D_INDDTO", temp_table_4e830cd9a7514c6eb232544654dfe96d."D_UDDTO", temp_table_4e830cd9a7514c6eb232544654dfe96d."K_AFD", temp_table_4e830cd9a7514c6eb232544654dfe96d."RECNUM", temp_table_4e830cd9a7514c6eb232544654dfe96d."V_ALDDG", temp_table_4e830cd9a7514c6eb232544654dfe96d."V_ALDER", temp_table_4e830cd9a7514c6eb232544654dfe96d."V_INDMINUT", temp_table_4e830cd9a7514c6eb232544654dfe96d."V_INDTIME", temp_table_4e830cd9a7514c6eb232544654dfe96d."V_SENGDAGE", temp_table_4e830cd9a7514c6eb232544654dfe96d."V_UDTIME", temp_table_4e830cd9a7514c6eb232544654dfe96d."VERSION".. Continuing with admin data only.
[2025-05-02 17:20:31] DEBUG CORE : resolve_grouping_function:
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d

[2025-05-02 17:20:31] DEBUG CORE : type_coercion:
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d

[2025-05-02 17:20:31] DEBUG CORE : Final analyzed plan:
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d

[2025-05-02 17:20:31] DEBUG CORE : Analyzer took 0 ms
[2025-05-02 17:20:31] DEBUG CORE : Optimizer input (pass 0):
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d

[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : optimize_projections:
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Optimized plan (pass 0):
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Optimizer input (pass 1):
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : optimize_projections:
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Optimized plan (pass 1):
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : optimizer pass 1 did not make changes
[2025-05-02 17:20:31] DEBUG CORE : Final optimized plan:
TableScan: temp_table_4e830cd9a7514c6eb232544654dfe96d projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Optimizer took 2 ms
[2025-05-02 17:20:31] DEBUG CORE : Input physical plan:
DataSourceExec: partitions=1, partition_sizes=[381]


[2025-05-02 17:20:31] DEBUG CORE : Optimized physical plan:
DataSourceExec: partitions=1, partition_sizes=[381]


[2025-05-02 17:20:31] INFO  CORE : [DIAG-5b2b8be7] Successfully created joined result with 381 record batches and 3009472 total records
[2025-05-02 17:20:31] INFO  CORE : Loaded 381 LPR_DIAG batches
[2025-05-02 17:20:31] INFO  CORE : Loading LPR_BES data...
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] ENTER Lpr2Register::load with base_path: /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:31] DEBUG CORE : crypto functions disabled
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Got schema with 27 fields
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Finding LPR v2 file paths from base: /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:31] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_bes' resolved='/home/tkragholm/generated_data/parquet/lpr_bes'
[2025-05-02 17:20:31] DEBUG LPR  [registry]: LprRegistry::find_files with absolute path: /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:31] INFO  LPR  [16092c0d][find_lpr_paths]: Operation started
[2025-05-02 17:20:31] DEBUG CORE [16092c0d]: Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_bes' resolved='/home/tkragholm/generated_data/parquet/lpr_bes'
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_files]: ENTER: ("/home/tkragholm/generated_data/parquet/lpr_bes")
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_paths]: STEP 1: Checking if current path is a specific LPR directory
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_paths]: Found LPR v2 procedure path (direct): /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_paths]: STEP 2: Checking parent directory if needed
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_paths]: Searching parent directory for admin data: /home/tkragholm/generated_data/parquet
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_paths]: Found LPR v2 admin path in parent directory: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_paths]: STEP 3: Checking subdirectories
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_paths]: STEP 4: Summarizing findings
[2025-05-02 17:20:31] INFO  LPR  [16092c0d][find_lpr_paths]: LPR paths found: admin=/home/tkragholm/generated_data/parquet/lpr_adm, diag=None, proc=/home/tkragholm/generated_data/parquet/lpr_bes, kontakter=None, diagnoser=None, procedurer=None
[2025-05-02 17:20:31] DEBUG LPR  [16092c0d][find_lpr_files]: EXIT: "Complete"
[2025-05-02 17:20:31] INFO  LPR  [16092c0d][find_lpr_paths]: Operation completed successfully
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Found admin_path: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Found proc_path: /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] LPR paths found: admin_path=Some("/home/tkragholm/generated_data/parquet/lpr_adm"), diag_path=None, proc_path=Some("/home/tkragholm/generated_data/parquet/lpr_bes")
[2025-05-02 17:20:31] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_adm' resolved='/home/tkragholm/generated_data/parquet/lpr_adm'
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Loading LPR2 admin data from: /home/tkragholm/generated_data/parquet/lpr_adm (resolved from /home/tkragholm/generated_data/parquet/lpr_adm)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Listing contents of admin directory: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2013.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2010.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2012.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2021.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2019.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2016.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2011.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2022.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2020.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2015.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2014.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2017.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2018.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Directory entry: /home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet (is_dir: false)
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Registering parquet directory as table 'lpr2_admin_e42c9c21' from: /home/tkragholm/generated_data/parquet/lpr_adm
[2025-05-02 17:20:31] DEBUG CORE : Parsing sql 'lpr2_admin_e42c9c21'...
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Successfully registered parquet directory as table 'lpr2_admin_e42c9c21'
[2025-05-02 17:20:31] DEBUG CORE : Parsing sql 'lpr2_admin_e42c9c21'...
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Successfully got table 'lpr2_admin_e42c9c21' as DataFrame
[2025-05-02 17:20:31] INFO  CORE : [DIAG-e42c9c21] Collecting admin data records...
[2025-05-02 17:20:31] DEBUG CORE : resolve_grouping_function:
TableScan: lpr2_admin_e42c9c21

[2025-05-02 17:20:31] DEBUG CORE : type_coercion:
TableScan: lpr2_admin_e42c9c21

[2025-05-02 17:20:31] DEBUG CORE : Final analyzed plan:
TableScan: lpr2_admin_e42c9c21

[2025-05-02 17:20:31] DEBUG CORE : Analyzer took 0 ms
[2025-05-02 17:20:31] DEBUG CORE : Optimizer input (pass 0):
TableScan: lpr2_admin_e42c9c21

[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 0)
[2025-05-02 17:20:31] DEBUG CORE : optimize_projections:
TableScan: lpr2_admin_e42c9c21 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Optimized plan (pass 0):
TableScan: lpr2_admin_e42c9c21 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Optimizer input (pass 1):
TableScan: lpr2_admin_e42c9c21 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 1)
[2025-05-02 17:20:31] DEBUG CORE : optimize_projections:
TableScan: lpr2_admin_e42c9c21 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Optimized plan (pass 1):
TableScan: lpr2_admin_e42c9c21 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : optimizer pass 1 did not make changes
[2025-05-02 17:20:31] DEBUG CORE : Final optimized plan:
TableScan: lpr2_admin_e42c9c21 projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:31] DEBUG CORE : Optimizer took 2 ms
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HAFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HENM' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_HSGH' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_KONTAARS' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'C_SPEC' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'K_AFD' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDDG' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_ALDER' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDMINUT' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_INDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'V_UDTIME' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Failed to create statistics converter: Arrow: Column 'VERSION' not found in schema for statistics conversion
[2025-05-02 17:20:31] DEBUG CORE : Input physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet], [home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet, home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet], ...]}, projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION], file_type=parquet


[2025-05-02 17:20:31] DEBUG CORE : Optimized physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_adm/2000.parquet:0..3045996, home/tkragholm/generated_data/parquet/lpr_adm/2001.parquet:0..3081988, home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet:0..188435], [home/tkragholm/generated_data/parquet/lpr_adm/2002.parquet:188435..3115074, home/tkragholm/generated_data/parquet/lpr_adm/2003.parquet:0..3156140, home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet:0..233640], [home/tkragholm/generated_data/parquet/lpr_adm/2004.parquet:233640..3188669, home/tkragholm/generated_data/parquet/lpr_adm/2005.parquet:0..3228738, home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet:0..132652], [home/tkragholm/generated_data/parquet/lpr_adm/2006.parquet:132652..3263970, home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet:0..3185101], [home/tkragholm/generated_data/parquet/lpr_adm/2007.parquet:3185101..3297655, home/tkragholm/generated_data/parquet/lpr_adm/2008.parquet:0..3334471, home/tkragholm/generated_data/parquet/lpr_adm/2009.parquet:0..2869394], ...]}, projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION], file_type=parquet


[2025-05-02 17:20:33] INFO  CORE : [DIAG-e42c9c21] Collected 381 admin data batches with 3009472 total records
[2025-05-02 17:20:33] DEBUG CORE : Path resolution (canonicalized): original='/home/tkragholm/generated_data/parquet/lpr_bes' resolved='/home/tkragholm/generated_data/parquet/lpr_bes'
[2025-05-02 17:20:33] INFO  CORE : [DIAG-e42c9c21] Loading procedure data from: /home/tkragholm/generated_data/parquet/lpr_bes
[2025-05-02 17:20:33] DEBUG CORE : Parsing sql 'lpr2_proc_e42c9c21'...
[2025-05-02 17:20:33] INFO  CORE : [DIAG-e42c9c21] Successfully registered procedure data as table 'lpr2_proc_e42c9c21'
[2025-05-02 17:20:33] DEBUG CORE : Parsing sql 'lpr2_proc_e42c9c21'...
[2025-05-02 17:20:33] DEBUG CORE : resolve_grouping_function:
TableScan: lpr2_proc_e42c9c21

[2025-05-02 17:20:33] DEBUG CORE : type_coercion:
TableScan: lpr2_proc_e42c9c21

[2025-05-02 17:20:33] DEBUG CORE : Final analyzed plan:
TableScan: lpr2_proc_e42c9c21

[2025-05-02 17:20:33] DEBUG CORE : Analyzer took 0 ms
[2025-05-02 17:20:33] DEBUG CORE : Optimizer input (pass 0):
TableScan: lpr2_proc_e42c9c21

[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 0)
[2025-05-02 17:20:33] DEBUG CORE : optimize_projections:
TableScan: lpr2_proc_e42c9c21 projection=[D_AMBDTO, RECNUM]

[2025-05-02 17:20:33] DEBUG CORE : Optimized plan (pass 0):
TableScan: lpr2_proc_e42c9c21 projection=[D_AMBDTO, RECNUM]

[2025-05-02 17:20:33] DEBUG CORE : Optimizer input (pass 1):
TableScan: lpr2_proc_e42c9c21 projection=[D_AMBDTO, RECNUM]

[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 1)
[2025-05-02 17:20:33] DEBUG CORE : optimize_projections:
TableScan: lpr2_proc_e42c9c21 projection=[D_AMBDTO, RECNUM]

[2025-05-02 17:20:33] DEBUG CORE : Optimized plan (pass 1):
TableScan: lpr2_proc_e42c9c21 projection=[D_AMBDTO, RECNUM]

[2025-05-02 17:20:33] DEBUG CORE : optimizer pass 1 did not make changes
[2025-05-02 17:20:33] DEBUG CORE : Final optimized plan:
TableScan: lpr2_proc_e42c9c21 projection=[D_AMBDTO, RECNUM]

[2025-05-02 17:20:33] DEBUG CORE : Optimizer took 1 ms
[2025-05-02 17:20:33] DEBUG CORE : Input physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_bes/2000.parquet, home/tkragholm/generated_data/parquet/lpr_bes/2001.parquet], [home/tkragholm/generated_data/parquet/lpr_bes/2002.parquet, home/tkragholm/generated_data/parquet/lpr_bes/2003.parquet], [home/tkragholm/generated_data/parquet/lpr_bes/2004.parquet, home/tkragholm/generated_data/parquet/lpr_bes/2005.parquet], [home/tkragholm/generated_data/parquet/lpr_bes/2006.parquet, home/tkragholm/generated_data/parquet/lpr_bes/2007.parquet], [home/tkragholm/generated_data/parquet/lpr_bes/2008.parquet, home/tkragholm/generated_data/parquet/lpr_bes/2009.parquet], ...]}, projection=[D_AMBDTO, RECNUM], file_type=parquet


[2025-05-02 17:20:33] DEBUG CORE : Optimized physical plan:
DataSourceExec: file_groups={12 groups: [[home/tkragholm/generated_data/parquet/lpr_bes/2000.parquet:0..1784409, home/tkragholm/generated_data/parquet/lpr_bes/2001.parquet:0..1635997], [home/tkragholm/generated_data/parquet/lpr_bes/2001.parquet:1635997..1782361, home/tkragholm/generated_data/parquet/lpr_bes/2002.parquet:0..1786434, home/tkragholm/generated_data/parquet/lpr_bes/2003.parquet:0..1487608], [home/tkragholm/generated_data/parquet/lpr_bes/2003.parquet:1487608..1785494, home/tkragholm/generated_data/parquet/lpr_bes/2004.parquet:0..1784196, home/tkragholm/generated_data/parquet/lpr_bes/2005.parquet:0..1338324], [home/tkragholm/generated_data/parquet/lpr_bes/2005.parquet:1338324..1785128, home/tkragholm/generated_data/parquet/lpr_bes/2006.parquet:0..1783994, home/tkragholm/generated_data/parquet/lpr_bes/2007.parquet:0..1189608], [home/tkragholm/generated_data/parquet/lpr_bes/2007.parquet:1189608..1782330, home/tkragholm/generated_data/parquet/lpr_bes/2008.parquet:0..1784485, home/tkragholm/generated_data/parquet/lpr_bes/2009.parquet:0..1043199], ...]}, projection=[D_AMBDTO, RECNUM], file_type=parquet


[2025-05-02 17:20:34] INFO  CORE : [DIAG-e42c9c21] Collected 759 procedure data batches with 6164280 total records
[2025-05-02 17:20:34] INFO  CORE : [DIAG-e42c9c21] Joining admin data with diagnosis and/or procedure data
[2025-05-02 17:20:34] DEBUG DATA [batches_to_dataframe]: ENTER: (381)
[2025-05-02 17:20:34] DEBUG DATA [dataframe]: Creating DataFrame with schema: Schema { fields: [Field { name: "PNR", data_type: Utf8, nullable: false, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_ADIAG", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_AFD", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_HAFD", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_HENM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_HSGH", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_INDM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_KOM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_KONTAARS", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_PATTYPE", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_SGH", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_SPEC", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "C_UDM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "CPRTJEK", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "CPRTYPE", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false,metadata: {} }, Field { name: "D_HENDTO", data_type: Date32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "D_INDDTO", data_type: Date32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "D_UDDTO", data_type: Date32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "K_AFD", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "RECNUM", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_ALDDG", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_ALDER", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_INDMINUT", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_INDTIME", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_SENGDAGE", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "V_UDTIME", data_type: Int32, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "VERSION", data_type: Utf8, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }], metadata: {} }
[2025-05-02 17:20:34] DEBUG DATA [dataframe]: STEP 1: Creating memory table
[2025-05-02 17:20:34] DEBUG DATA [dataframe]: STEP 2: Registering table in context as temp_table_b56c06f7a100416785ddaaa0e0e7246b
[2025-05-02 17:20:34] DEBUG CORE : Parsing sql 'temp_table_b56c06f7a100416785ddaaa0e0e7246b'...
[2025-05-02 17:20:34] DEBUG DATA [dataframe]: STEP 3: Converting table to DataFrame
[2025-05-02 17:20:34] DEBUG CORE : Parsing sql 'temp_table_b56c06f7a100416785ddaaa0e0e7246b'...
[2025-05-02 17:20:34] INFO  DATA [dataframe]: Successfully created DataFrame
[2025-05-02 17:20:34] DEBUG DATA [batches_to_dataframe]: EXIT: "Success"
[2025-05-02 17:20:34] DEBUG CORE : [DIAG-e42c9c21] Created admin DataFrame
[2025-05-02 17:20:34] DEBUG DATA [batches_to_dataframe]: ENTER: (759)
[2025-05-02 17:20:34] DEBUG DATA [dataframe]: Creating DataFrame with schema: Schema { fields: [Field { name: "D_AMBDTO", data_type: Utf8View, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }, Field { name: "RECNUM", data_type: Utf8View, nullable: true, dict_id: 0, dict_is_ordered: false, metadata: {} }], metadata: {} }
[2025-05-02 17:20:34] DEBUG DATA [dataframe]: STEP 1: Creating memory table
[2025-05-02 17:20:34] DEBUG DATA [dataframe]: STEP 2: Registering table in context as temp_table_2df3850d29f243bfb8db91913bed35fb
[2025-05-02 17:20:34] DEBUG CORE : Parsing sql 'temp_table_2df3850d29f243bfb8db91913bed35fb'...
[2025-05-02 17:20:34] DEBUG DATA [dataframe]: STEP 3: Converting table to DataFrame
[2025-05-02 17:20:34] DEBUG CORE : Parsing sql 'temp_table_2df3850d29f243bfb8db91913bed35fb'...
[2025-05-02 17:20:34] INFO  DATA [dataframe]: Successfully created DataFrame
[2025-05-02 17:20:34] DEBUG DATA [batches_to_dataframe]: EXIT: "Success"
[2025-05-02 17:20:34] DEBUG CORE : [DIAG-e42c9c21] Created procedure DataFrame
[2025-05-02 17:20:34] DEBUG CORE : Parsing sql 'RECNUM'...
[2025-05-02 17:20:34] DEBUG CORE : Parsing sql 'RECNUM'...
[2025-05-02 17:20:34] WARN  CORE : [DIAG-e42c9c21] Failed to join data with procedure data: Schema error: No field named recnum. Valid fields are temp_table_b56c06f7a100416785ddaaa0e0e7246b."PNR", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_ADIAG", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_AFD", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_HAFD", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_HENM", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_HSGH", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_INDM", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_KOM", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_KONTAARS", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_PATTYPE", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_SGH", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_SPEC", temp_table_b56c06f7a100416785ddaaa0e0e7246b."C_UDM", temp_table_b56c06f7a100416785ddaaa0e0e7246b."CPRTJEK", temp_table_b56c06f7a100416785ddaaa0e0e7246b."CPRTYPE", temp_table_b56c06f7a100416785ddaaa0e0e7246b."D_HENDTO", temp_table_b56c06f7a100416785ddaaa0e0e7246b."D_INDDTO", temp_table_b56c06f7a100416785ddaaa0e0e7246b."D_UDDTO", temp_table_b56c06f7a100416785ddaaa0e0e7246b."K_AFD", temp_table_b56c06f7a100416785ddaaa0e0e7246b."RECNUM", temp_table_b56c06f7a100416785ddaaa0e0e7246b."V_ALDDG", temp_table_b56c06f7a100416785ddaaa0e0e7246b."V_ALDER", temp_table_b56c06f7a100416785ddaaa0e0e7246b."V_INDMINUT", temp_table_b56c06f7a100416785ddaaa0e0e7246b."V_INDTIME", temp_table_b56c06f7a100416785ddaaa0e0e7246b."V_SENGDAGE", temp_table_b56c06f7a100416785ddaaa0e0e7246b."V_UDTIME", temp_table_b56c06f7a100416785ddaaa0e0e7246b."VERSION".. Continuing without procedure join.
[2025-05-02 17:20:34] DEBUG CORE : resolve_grouping_function:
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b

[2025-05-02 17:20:34] DEBUG CORE : type_coercion:
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b

[2025-05-02 17:20:34] DEBUG CORE : Final analyzed plan:
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b

[2025-05-02 17:20:34] DEBUG CORE : Analyzer took 0 ms
[2025-05-02 17:20:34] DEBUG CORE : Optimizer input (pass 0):
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b

[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 0)
[2025-05-02 17:20:34] DEBUG CORE : optimize_projections:
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:34] DEBUG CORE : Optimized plan (pass 0):
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:34] DEBUG CORE : Optimizer input (pass 1):
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_nested_union' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'simplify_expressions' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'replace_distinct_aggregate' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_join' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'decorrelate_predicate_subquery' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'scalar_subquery_to_join' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'extract_equijoin_predicate' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_duplicated_expr' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_filter' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_cross_join' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_limit' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'propagate_empty_relation' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_one_union' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'filter_null_join_keys' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_outer_join' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_limit' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'push_down_filter' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'single_distinct_aggregation_to_group_by' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'eliminate_group_by_constant' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : Plan unchanged by optimizer rule 'common_sub_expression_eliminate' (pass 1)
[2025-05-02 17:20:34] DEBUG CORE : optimize_projections:
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:34] DEBUG CORE : Optimized plan (pass 1):
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:34] DEBUG CORE : optimizer pass 1 did not make changes
[2025-05-02 17:20:34] DEBUG CORE : Final optimized plan:
TableScan: temp_table_b56c06f7a100416785ddaaa0e0e7246b projection=[PNR, C_ADIAG, C_AFD, C_HAFD, C_HENM, C_HSGH, C_INDM, C_KOM, C_KONTAARS, C_PATTYPE, C_SGH, C_SPEC, C_UDM, CPRTJEK, CPRTYPE, D_HENDTO, D_INDDTO, D_UDDTO, K_AFD, RECNUM, V_ALDDG, V_ALDER, V_INDMINUT, V_INDTIME, V_SENGDAGE, V_UDTIME, VERSION]

[2025-05-02 17:20:34] DEBUG CORE : Optimizer took 2 ms
[2025-05-02 17:20:34] DEBUG CORE : Input physical plan:
DataSourceExec: partitions=1, partition_sizes=[381]


[2025-05-02 17:20:34] DEBUG CORE : Optimized physical plan:
DataSourceExec: partitions=1, partition_sizes=[381]


[2025-05-02 17:20:34] INFO  CORE : [DIAG-e42c9c21] Successfully created joined result with 381 record batches and 3009472 total records
[2025-05-02 17:20:34] INFO  CORE : Loaded 381 LPR_BES batches
[2025-05-02 17:20:34] INFO  CORE : Processing LPR data and identifying SCD in population...
Error: Data error: C_DIAG column not found: Schema error: Unable to get field named "C_DIAG". Valid fields: ["PNR", "C_ADIAG", "C_AFD", "C_HAFD", "C_HENM", "C_HSGH", "C_INDM", "C_KOM", "C_KONTAARS", "C_PATTYPE", "C_SGH", "C_SPEC", "C_UDM", "CPRTJEK", "CPRTYPE", "D_HENDTO", "D_INDDTO", "D_UDDTO", "K_AFD", "RECNUM", "V_ALDDG", "V_ALDER", "V_INDMINUT", "V_INDTIME", "V_SENGDAGE", "V_UDTIME", "VERSION"]

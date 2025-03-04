(venv) (base) PS E:\workdata\708245\CDEF\Projekter\Family\matching_analysis> ids.exe check-balance -m .\output\matched_pairs.csv -c ..\..\..\..\data\registers\. --family-file .\family.parquet --structured

Covariate Balance Analysis
══════════════════════════

Loading Matched Pairs
─────────────────────
Matched pairs loaded: 42454
Unique PNRs found: 197992
Loading time: 224ms
! family (combined) not found at ..\..\..\..\data\registers\.\.\family.parquet - will attempt to find alternative paths
[2025-03-04T10:01:48Z WARN  ids::main_run] family (combined) not found at ..\..\..\..\data\registers\.\.\family.parquet
Using custom family file: ..\..\..\..\data\registers\.\.\family.parquet

Loading Register Data
─────────────────────
ℹ Using base directory: ..\..\..\..\data\registers\.
ℹ CRITICAL CHECK: Verifying family file at ..\..\..\..\data\registers\.\.\family.parquet
! Family file not found at: ..\..\..\..\data\registers\.\.\family.parquet
ℹ Using the following custom register paths:
  - family: ..\..\..\..\data\registers\.\.\family.parquet
! Path doesn't exist: ..\..\..\..\data\registers\.\.\family.parquet
[2025-03-04T10:01:48Z WARN  ids::main_run] Path doesn't exist: ..\..\..\..\data\registers\.\.\family.parquet
[2025-03-04T10:01:48Z WARN  types::translation] Failed to open translation map at mappings/statsb.json: The system cannot find the path specified. (os error 3)
[2025-03-04T10:01:48Z ERROR loader] Failed to create UnifiedStore: Invalid format: The system cannot find the path specified. (os error 3)
✗ Loading failed with error: Invalid format: The system cannot find the path specified. (os error 3)
ℹ Trying to diagnose the issue:
! Failed to load register data: Invalid format: The system cannot find the path specified. (os error 3)
[2025-03-04T10:01:48Z ERROR ids::main_run] Detailed register loading error: InvalidFormat("The system cannot find the path specified. (os error 3)")
ℹ Continuing in diagnostic mode with simulated data
ℹ Note: Results will be based on simulated data, not actual register data    ids.exe check-balance -m .\output\matched_pairs.csv --akm-dir E:\workdata\708245\data\registers\akm\ --bef-dir E:\workdata\708245\data\registers\bef\ --ind-dir E:\workdata\708245\data\registers\ind\ --uddf-dir E:\workdata\708245\data\registers\uddf\ --family-file .\family.parquet --structured

Covariate Balance Analysis
══════════════════════════

Loading Matched Pairs
─────────────────────
Matched pairs loaded: 42454
Unique PNRs found: 197992
Loading time: 297ms
ℹ Found family (relative) at \\?\UNC\srvfsenas1.dstfse.local\data\workdata\708245\CDEF\Projekter\Family\matching_analysis\family.parquet (file)
Using custom family file: \\?\UNC\srvfsenas1.dstfse.local\data\workdata\708245\CDEF\Projekter\Family\matching_analysis\family.parquet
ℹ Found akm (absolute) at E:\workdata\708245\data\registers\akm\ (directory)
Using custom AKM directory: E:\workdata\708245\data\registers\akm\
ℹ Found bef (absolute) at E:\workdata\708245\data\registers\bef\ (directory)
Using custom BEF directory: E:\workdata\708245\data\registers\bef\
ℹ Found ind (absolute) at E:\workdata\708245\data\registers\ind\ (directory)
Using custom IND directory: E:\workdata\708245\data\registers\ind\
ℹ Found uddf (absolute) at E:\workdata\708245\data\registers\uddf\ (directory)
Using custom UDDF directory: E:\workdata\708245\data\registers\uddf\

Loading Register Data
─────────────────────
ℹ No base directory provided, using current directory: E:\workdata\708245\CDEF\Projekter\Family\matching_analysis
ℹ CRITICAL CHECK: Verifying family file at \\?\UNC\srvfsenas1.dstfse.local\data\workdata\708245\CDEF\Projekter\Family\matching_analysis\family.parquet
ℹ Family path exists and is a file - checking access
ℹ Successfully read 16 bytes from family file
ℹ First bytes: 50 41 52 31 15 04 15 80 80 80 01 15 96 91 4d 4c
ℹ Testing file with pqrs (shell command)
✗ Failed to run pqrs command
ℹ Using the following custom register paths:
  - family: \\?\UNC\srvfsenas1.dstfse.local\data\workdata\708245\CDEF\Projekter\Family\matching_analysis\family.parquet
  - bef: E:\workdata\708245\data\registers\bef\
ℹ Directory E:\workdata\708245\data\registers\bef\ contains: 200012.parquet, 200112.parquet, 200212.parquet, 200312.parquet, 200412.parquet, ...
  - akm: E:\workdata\708245\data\registers\akm\
ℹ Directory E:\workdata\708245\data\registers\akm\ contains: 2000.parquet, 2001.parquet, 2002.parquet, 2003.parquet, 2004.parquet, ...
  - uddf: E:\workdata\708245\data\registers\uddf\
ℹ Directory E:\workdata\708245\data\registers\uddf\ contains: 202009.parquet, 202209.parquet
  - ind: E:\workdata\708245\data\registers\ind\
ℹ Directory E:\workdata\708245\data\registers\ind\ contains: 2000.parquet, 2001.parquet, 2002.parquet, 2003.parquet, 2004.parquet, ...
[2025-03-04T10:06:48Z WARN  types::translation] Failed to open translation map at mappings/statsb.json: The system cannot find the path specified. (os error 3)
[2025-03-04T10:06:48Z ERROR loader] Failed to create UnifiedStore: Invalid format: The system cannot find the path specified. (os error 3)
✗ Loading failed with error: Invalid format: The system cannot find the path specified. (os error 3)
ℹ Trying to diagnose the issue:
ℹ Family file exists at \\?\UNC\srvfsenas1.dstfse.local\data\workdata\708245\CDEF\Projekter\Family\matching_analysis\family.parquet
ℹ Family file can be opened with Rust std::fs::File
ℹ Register directory bef exists
ℹ Checking sample bef file: E:\workdata\708245\data\registers\bef\200012.parquet
ℹ Sample file can be opened with Rust std::fs::File
ℹ Register directory akm exists
ℹ Checking sample akm file: E:\workdata\708245\data\registers\akm\2000.parquet
ℹ Sample file can be opened with Rust std::fs::File
ℹ Register directory uddf exists
ℹ Checking sample uddf file: E:\workdata\708245\data\registers\uddf\202009.parquet
ℹ Sample file can be opened with Rust std::fs::File
ℹ Register directory ind exists
ℹ Checking sample ind file: E:\workdata\708245\data\registers\ind\2000.parquet
ℹ Sample file can be opened with Rust std::fs::File
! Failed to load register data: Invalid format: The system cannot find the path specified. (os error 3)
[2025-03-04T10:06:48Z ERROR ids::main_run] Detailed register loading error: InvalidFormat("The system cannot find the path specified. (os error 3)")
ℹ Continuing in diagnostic mode with simulated data
ℹ Note: Results will be based on simulated data, not actual register data
(venv) (base) PS E:\workdata\708245\CDEF\Projekter\Family\matching_analysis>

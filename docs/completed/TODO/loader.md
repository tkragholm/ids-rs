<analysis>
<duplication_area>
1. Registry Loading Functions
The most significant area of code duplication is in the registry loading functions (load_akm, load_bef, load_ind, load_uddf, load_family). Each function follows almost identical patterns:
- Create a progress tracker
- Normalize path handling
- Determine search paths
- Load Parquet files
- Handle potential errors

Current pattern example (from src/registry/akm.rs):
```rust
pub fn load_akm(
    base_path: &str,
    pnr_filter: Option<&HashSet<String>>,
) -> Result<Vec<RecordBatch>, IdsError> {
    log::info!("Loading AKM data from {base_path}");

    let progress = LoaderProgress::new();
    progress.set_main_message("Loading AKM data");

    let path = Path::new(base_path);

    let akm_path = if path.is_dir() {
        // Complex path resolution logic
        // ...
    } else {
        // Fallback path handling
        // ...
    };

    let schema = akm_schema();

    let batches = if akm_path.is_dir() {
        load_parquet_files_parallel(&akm_path, Some(&schema), pnr_filter, Some(&progress))?
    } else if path.exists() && path.extension().is_some_and(|ext| ext == "parquet") {
        // Direct file handling
        crate::formats::read_parquet(path, Some(&schema), Some(&progress), pnr_filter)?
    } else {
        Vec::new()
    };

    log::info!("Loaded {} record batches of AKM data", batches.len());
    Ok(batches)
}
```
</duplication_area>

<refactoring_example name="registry_loading">
Proposed Refactoring:
1. Create a generic loader trait in the utils crate
2. Implement a generic loading function that can be customized

```rust
// In utils crate
pub trait RegisterLoader {
    fn get_schema() -> Schema;
    fn get_register_name() -> &'static str;
}

// Centralized loading function
pub fn load_register<T: RegisterLoader>(
    base_path: &str,
    pnr_filter: Option<&HashSet<String>>
) -> Result<Vec<RecordBatch>, IdsError> {
    let progress = LoaderProgress::new();
    progress.set_main_message(&format!("Loading {} data", T::get_register_name()));

    let path = Path::new(base_path);
    let register_path = resolve_register_path(path, T::get_register_name());
    let schema = T::get_schema();

    let batches = if register_path.is_dir() {
        load_parquet_files_parallel(&register_path, Some(&schema), pnr_filter, Some(&progress))?
    } else if path.exists() && path.extension().is_some_and(|ext| ext == "parquet") {
        crate::formats::read_parquet(path, Some(&schema), Some(&progress), pnr_filter)?
    } else {
        Vec::new()
    };

    log::info!("Loaded {} record batches of {} data", batches.len(), T::get_register_name());
    Ok(batches)
}

// Example implementation
impl RegisterLoader for AkmRegister {
    fn get_schema() -> Schema { akm_schema() }
    fn get_register_name() -> &'static str { "AKM" }
}

// Replace existing loading functions
pub fn load_akm(base_path: &str, pnr_filter: Option<&HashSet<String>>) -> Result<Vec<RecordBatch>, IdsError> {
    load_register::<AkmRegister>(base_path, pnr_filter)
}
```
</refactoring_example>

<duplication_area>
2. Path Resolution Logic
Similar path resolution logic is repeated across multiple modules:
- config/path.rs
- readers/custom_path.rs
- registry loading functions

The current implementation involves complex, repeated path detection and resolution logic.
</duplication_area>

<refactoring_example name="path_resolution">
Proposed Centralized Path Resolution:
```rust
// In utils crate
pub struct PathResolver;

impl PathResolver {
    pub fn resolve_register_path(base_path: &Path, register_type: &str) -> PathBuf {
        // Centralized path resolution logic
        let register_dir = base_path.join(register_type.to_lowercase());
        let registers_dir = base_path.join("registers").join(register_type.to_lowercase());

        if register_dir.exists() && register_dir.is_dir() {
            register_dir
        } else if registers_dir.exists() && registers_dir.is_dir() {
            registers_dir
        } else {
            base_path.to_path_buf()
        }
    }

    pub fn find_parquet_files(dir: &Path) -> Vec<PathBuf> {
        // Utility to find Parquet files
        // Implementation using walkdir or std::fs
    }
}
```
</refactoring_example>

<duplication_area>
3. Progress Tracking
Multiple modules implement similar progress tracking logic with minor variations.
</duplication_area>

<refactoring_example name="progress_tracking">
Proposed Progress Tracking Utility:
```rust
// Enhanced progress tracking utility
pub struct ProgressTracker {
    multi_progress: MultiProgress,
    main_bar: ProgressBar,
    sub_bars: Vec<ProgressBar>,
}

impl ProgressTracker {
    pub fn new(total_stages: u64) -> Self {
        let multi_progress = MultiProgress::new();
        let main_bar = create_main_progress_bar(total_stages);

        Self {
            multi_progress,
            main_bar,
            sub_bars: Vec::new(),
        }
    }

    pub fn track_stage(&mut self, stage_name: &str) {
        self.main_bar.set_message(stage_name.to_string());
        self.main_bar.inc(1);
    }

    pub fn create_file_progress(&self, file_path: &Path) -> ProgressBar {
        // Consistent file progress tracking
    }
}
```
</refactoring_example>

<recommendation>
Main Refactoring Recommendations:

1. Centralize Path Resolution
- Create a unified `PathResolver` in the utils crate
- Standardize path detection and resolution across modules
- Reduce duplicated path handling logic

2. Generic Register Loading
- Implement a trait-based generic loading mechanism
- Create a centralized loading function that can be customized
- Eliminate repeated loading patterns in registry modules

3. Enhanced Progress Tracking
- Develop a more flexible `ProgressTracker`
- Provide consistent progress reporting across different loading scenarios
- Reduce boilerplate progress tracking code

Potential Benefits:
- Reduced code complexity
- Improved maintainability
- More consistent error handling
- Easier testing and validation
- Better separation of concerns

Challenges:
- Refactoring requires careful migration
- Some performance overhead with generic approaches
- Requires coordinated changes across multiple modules

Recommended Implementation Strategy:
1. Start with utility crate modifications
2. Incrementally refactor existing modules
3. Comprehensive testing at each stage
4. Gradual rollout of changes
</recommendation>
</analysis>

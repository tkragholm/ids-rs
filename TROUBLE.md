diagnostics
ids-rs/crates/loader/tests/test_edge_cases.rs

```rust
use std::io::Write;
use tempfile::{TempDir};
// warning: Unnecessary braces in use statement

use loader::{ParallelLoader, SequentialLoader, StoreLoader, RegisterPathConfig};
```

ids-rs/crates/loader/tests/test_registry.rs

```rust
// Import test helpers
mod test_helpers;
use test_helpers::{setup, registry};
// error: unresolved import `test_helpers::registry`
//        consider importing this module instead:
//        loader::registry

// Helper to generate test data in a temporary directory
```

ids-rs/crates/types/benches/arrow_backend.rs

```rust
            b.iter(|| {
                for pnr in ["0123456789", "1234567890", "2345678901"] {
                    let _ = backend.find_pnr_index("demographics", black_box(pnr));
// error: function `find_pnr_index` is private
                }
            });
```

```rust
            b.iter(|| {
                for pnr in ["0123456789", "1234567890", "2345678901"] {
                    let _: Result<Option<Demographics>> = backend.get_demographics(black_box(pnr), 2020);
// error: function `get_demographics` is private
                }
            });
```

ids-rs/crates/loader/tests/test_formats.rs

```rust
use std::io::Write;
// warning: unused import: `std::io::Write`
//          `#[warn(unused_imports)]` on by default
use arrow::array::{Int32Array, StringArray};
use arrow::datatypes::{DataType, Field, Schema};
```

```rust
// Import test helpers
mod test_helpers;
use test_helpers::{setup, formats};
// error: unresolved import `test_helpers::formats`
//        consider importing this module instead:
//        loader::formats

// Helper to create a test parquet file
```

ids-rs/crates/loader/tests/test_helpers.rs

```rust
// Extension trait to add methods to check if data is present in the ArrowStore
pub trait ArrowBackendExt {
    fn has_family_data(&self) -> bool;
// warning: methods `has_family_data`, `has_akm_data`, `has_bef_data`, `has_ind_data`, and `has_uddf_data` are never used
//          `#[warn(dead_code)]` on by default
    fn has_akm_data(&self) -> bool;
    fn has_bef_data(&self) -> bool;
```

```rust
pub trait ArrowBackendExt {
    fn has_family_data(&self) -> bool;
    fn has_akm_data(&self) -> bool;
// warning: methods `has_family_data`, `has_akm_data`, `has_bef_data`, `has_ind_data`, and `has_uddf_data` are never used
//          `#[warn(dead_code)]` on by default
    fn has_bef_data(&self) -> bool;
    fn has_ind_data(&self) -> bool;
```

```rust
    fn has_family_data(&self) -> bool;
    fn has_akm_data(&self) -> bool;
    fn has_bef_data(&self) -> bool;
// warning: methods `has_family_data`, `has_akm_data`, `has_bef_data`, `has_ind_data`, and `has_uddf_data` are never used
//          `#[warn(dead_code)]` on by default
    fn has_ind_data(&self) -> bool;
    fn has_uddf_data(&self) -> bool;
```

```rust
    fn has_akm_data(&self) -> bool;
    fn has_bef_data(&self) -> bool;
    fn has_ind_data(&self) -> bool;
// warning: methods `has_family_data`, `has_akm_data`, `has_bef_data`, `has_ind_data`, and `has_uddf_data` are never used
//          `#[warn(dead_code)]` on by default
    fn has_uddf_data(&self) -> bool;
}
```

```rust
    fn has_bef_data(&self) -> bool;
    fn has_ind_data(&self) -> bool;
    fn has_uddf_data(&self) -> bool;
// warning: methods `has_family_data`, `has_akm_data`, `has_bef_data`, `has_ind_data`, and `has_uddf_data` are never used
//          `#[warn(dead_code)]` on by default
}
```

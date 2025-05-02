# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_catalog_listing`

## Modules

## Module `helpers`

Helper functions for the table implementation

```rust
pub mod helpers { /* ... */ }
```

### Types

#### Struct `Partition`

```rust
pub struct Partition {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ErasedDestructor**
- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `expr_applicable_for_cols`

Check whether the given expression can be resolved using only the columns `col_names`.
This means that if this function returns true:
- the table provider can filter the table partition values with this expression
- the expression can be marked as `TableProviderFilterPushDown::Exact` once this filtering
  was performed

```rust
pub fn expr_applicable_for_cols(col_names: &[&str], expr: &datafusion_expr::Expr) -> bool { /* ... */ }
```

#### Function `split_files`

**Attributes:**

- `#[deprecated(since = "47.0.0", note = "use `FileGroup::split_files` instead")]`

**⚠️ Deprecated since 47.0.0**: use `FileGroup::split_files` instead

Partition the list of files into `n` groups

```rust
pub fn split_files(partitioned_files: Vec<datafusion_datasource::PartitionedFile>, n: usize) -> Vec<Vec<datafusion_datasource::PartitionedFile>> { /* ... */ }
```

#### Function `list_partitions`

Returns a recursive list of the partitions in `table_path` up to `max_depth`

```rust
pub async fn list_partitions(store: &dyn ObjectStore, table_path: &datafusion_datasource::ListingTableUrl, max_depth: usize, partition_prefix: Option<object_store::path::Path>) -> datafusion_common::Result<Vec<Partition>> { /* ... */ }
```

#### Function `evaluate_partition_prefix`

```rust
pub fn evaluate_partition_prefix<''a>(partition_cols: &''a [(String, arrow::datatypes::DataType)], filters: &''a [datafusion_expr::Expr]) -> Option<object_store::path::Path> { /* ... */ }
```

#### Function `pruned_partition_list`

Discover the partitions on the given path and prune out files
that belong to irrelevant partitions using `filters` expressions.
`filters` should only contain expressions that can be evaluated
using only the partition columns.

```rust
pub async fn pruned_partition_list<''a>(ctx: &''a dyn Session, store: &''a dyn ObjectStore, table_path: &''a datafusion_datasource::ListingTableUrl, filters: &''a [datafusion_expr::Expr], file_extension: &''a str, partition_cols: &''a [(String, arrow::datatypes::DataType)]) -> datafusion_common::Result<futures::stream::BoxStream<''a, datafusion_common::Result<datafusion_datasource::PartitionedFile>>> { /* ... */ }
```

#### Function `parse_partitions_for_path`

Extract the partition values for the given `file_path` (in the given `table_path`)
associated to the partitions defined by `table_partition_cols`

```rust
pub fn parse_partitions_for_path<''a, I>(table_path: &datafusion_datasource::ListingTableUrl, file_path: &''a object_store::path::Path, table_partition_cols: I) -> Option<Vec<&''a str>>
where
    I: IntoIterator<Item = &''a str> { /* ... */ }
```

#### Function `describe_partition`

Describe a partition as a (path, depth, files) tuple for easier assertions

```rust
pub fn describe_partition(partition: &Partition) -> (&str, usize, Vec<&str>) { /* ... */ }
```


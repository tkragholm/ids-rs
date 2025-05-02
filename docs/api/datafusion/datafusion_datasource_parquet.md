# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_datasource_parquet`

[`ParquetExec`] FileSource for reading Parquet files

## Modules

## Module `access_plan`

```rust
pub mod access_plan { /* ... */ }
```

### Types

#### Struct `ParquetAccessPlan`

A selection of rows and row groups within a ParquetFile to decode.

A `ParquetAccessPlan` is used to limit the row groups and data pages a `DataSourceExec`
will read and decode to improve performance.

Note that page level pruning based on ArrowPredicate is applied after all of
these selections

# Example

For example, given a Parquet file with 4 row groups, a `ParquetAccessPlan`
can be used to specify skipping row group 0 and 2, scanning a range of rows
in row group 1, and scanning all rows in row group 3 as follows:

```rust
# use parquet::arrow::arrow_reader::{RowSelection, RowSelector};
# use datafusion_datasource_parquet::ParquetAccessPlan;
// Default to scan all row groups
let mut access_plan = ParquetAccessPlan::new_all(4);
access_plan.skip(0); // skip row group
// Use parquet reader RowSelector to specify scanning rows 100-200 and 350-400
// in a row group that has 1000 rows
let row_selection = RowSelection::from(vec![
   RowSelector::skip(100),
   RowSelector::select(100),
   RowSelector::skip(150),
   RowSelector::select(50),
   RowSelector::skip(600),  // skip last 600 rows
]);
access_plan.scan_selection(1, row_selection);
access_plan.skip(2); // skip row group 2
// row group 3 is scanned by default
```

The resulting plan would look like:

```text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐

│                   │  SKIP

└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
 Row Group 0
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
 ┌────────────────┐    SCAN ONLY ROWS
│└────────────────┘ │  100-200
 ┌────────────────┐    350-400
│└────────────────┘ │
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
 Row Group 1
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                       SKIP
│                   │

└ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
 Row Group 2
┌───────────────────┐
│                   │  SCAN ALL ROWS
│                   │
│                   │
└───────────────────┘
 Row Group 3
```

```rust
pub struct ParquetAccessPlan {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new_all(row_group_count: usize) -> Self { /* ... */ }
  ```
  Create a new `ParquetAccessPlan` that scans all row groups

- ```rust
  pub fn new_none(row_group_count: usize) -> Self { /* ... */ }
  ```
  Create a new `ParquetAccessPlan` that scans no row groups

- ```rust
  pub fn new(row_groups: Vec<RowGroupAccess>) -> Self { /* ... */ }
  ```
  Create a new `ParquetAccessPlan` from the specified [`RowGroupAccess`]es

- ```rust
  pub fn set(self: &mut Self, idx: usize, access: RowGroupAccess) { /* ... */ }
  ```
  Set the i-th row group to the specified [`RowGroupAccess`]

- ```rust
  pub fn skip(self: &mut Self, idx: usize) { /* ... */ }
  ```
  skips the i-th row group (should not be scanned)

- ```rust
  pub fn scan(self: &mut Self, idx: usize) { /* ... */ }
  ```
  scan the i-th row group

- ```rust
  pub fn should_scan(self: &Self, idx: usize) -> bool { /* ... */ }
  ```
  Return true if the i-th row group should be scanned

- ```rust
  pub fn scan_selection(self: &mut Self, idx: usize, selection: RowSelection) { /* ... */ }
  ```
  Set to scan only the [`RowSelection`] in the specified row group.

- ```rust
  pub fn into_overall_row_selection(self: Self, row_group_meta_data: &[RowGroupMetaData]) -> Result<Option<RowSelection>> { /* ... */ }
  ```
  Return an overall `RowSelection`, if needed

- ```rust
  pub fn row_group_index_iter(self: &Self) -> impl Iterator<Item = usize> + ''_ { /* ... */ }
  ```
  Return an iterator over the row group indexes that should be scanned

- ```rust
  pub fn row_group_indexes(self: &Self) -> Vec<usize> { /* ... */ }
  ```
  Return a vec of all row group indexes to scan

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Return the total number of row groups (not the total number or groups to

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Return true if there are no row groups

- ```rust
  pub fn inner(self: &Self) -> &[RowGroupAccess] { /* ... */ }
  ```
  Get a reference to the inner accesses

- ```rust
  pub fn into_inner(self: Self) -> Vec<RowGroupAccess> { /* ... */ }
  ```
  Covert into the inner row group accesses

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParquetAccessPlan { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Allocation**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParquetAccessPlan) -> bool { /* ... */ }
    ```

- **IntoEither**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
#### Enum `RowGroupAccess`

Describes how the parquet reader will access a row group

```rust
pub enum RowGroupAccess {
    Skip,
    Scan,
    Selection(parquet::arrow::arrow_reader::RowSelection),
}
```

##### Variants

###### `Skip`

Do not read the row group at all

###### `Scan`

Read all rows from the row group

###### `Selection`

Scan only the specified rows within the row group

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `parquet::arrow::arrow_reader::RowSelection` |  |

##### Implementations

###### Methods

- ```rust
  pub fn should_scan(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this row group should be scanned

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RowGroupAccess) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RowGroupAccess { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
## Module `file_format`

[`ParquetFormat`]: Parquet [`FileFormat`] abstractions

```rust
pub mod file_format { /* ... */ }
```

### Types

#### Struct `ParquetFormatFactory`

Factory struct used to create [ParquetFormat]

```rust
pub struct ParquetFormatFactory {
    pub options: Option<datafusion_common::config::TableParquetOptions>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `options` | `Option<datafusion_common::config::TableParquetOptions>` | inner options for parquet |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates an instance of [ParquetFormatFactory]

- ```rust
  pub fn new_with_options(options: TableParquetOptions) -> Self { /* ... */ }
  ```
  Creates an instance of [ParquetFormatFactory] with customized default options

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Send**
- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **FileFormatFactory**
  - ```rust
    fn create(self: &Self, state: &dyn Session, format_options: &std::collections::HashMap<String, String>) -> Result<Arc<dyn FileFormat>> { /* ... */ }
    ```

  - ```rust
    fn default(self: &Self) -> Arc<dyn FileFormat> { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

- **GetExt**
  - ```rust
    fn get_ext(self: &Self) -> String { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ParquetFormatFactory { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `ParquetFormat`

The Apache Parquet `FileFormat` implementation

```rust
pub struct ParquetFormat {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Construct a new Format with no local overrides

- ```rust
  pub fn with_enable_pruning(self: Self, enable: bool) -> Self { /* ... */ }
  ```
  Activate statistics based row group level pruning

- ```rust
  pub fn enable_pruning(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if pruning is enabled

- ```rust
  pub fn with_metadata_size_hint(self: Self, size_hint: Option<usize>) -> Self { /* ... */ }
  ```
  Provide a hint to the size of the file metadata. If a hint is provided

- ```rust
  pub fn metadata_size_hint(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Return the metadata size hint if set

- ```rust
  pub fn with_skip_metadata(self: Self, skip_metadata: bool) -> Self { /* ... */ }
  ```
  Tell the parquet reader to skip any metadata that may be in

- ```rust
  pub fn skip_metadata(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if schema metadata will be cleared prior to

- ```rust
  pub fn with_options(self: Self, options: TableParquetOptions) -> Self { /* ... */ }
  ```
  Set Parquet options for the ParquetFormat

- ```rust
  pub fn options(self: &Self) -> &TableParquetOptions { /* ... */ }
  ```
  Parquet options

- ```rust
  pub fn force_view_types(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if should use view types.

- ```rust
  pub fn with_force_view_types(self: Self, use_views: bool) -> Self { /* ... */ }
  ```
  If true, will use view types. See [`Self::force_view_types`] for details

- ```rust
  pub fn binary_as_string(self: &Self) -> bool { /* ... */ }
  ```
  Return `true` if binary types will be read as strings.

- ```rust
  pub fn with_binary_as_string(self: Self, binary_as_string: bool) -> Self { /* ... */ }
  ```
  If true, will read binary types as strings. See [`Self::binary_as_string`] for details

- ```rust
  pub fn coerce_int96(self: &Self) -> Option<String> { /* ... */ }
  ```

- ```rust
  pub fn with_coerce_int96(self: Self, time_unit: Option<String>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ParquetFormat { /* ... */ }
    ```

- **FileFormat**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn get_ext(self: &Self) -> String { /* ... */ }
    ```

  - ```rust
    fn get_ext_with_compression(self: &Self, file_compression_type: &FileCompressionType) -> Result<String> { /* ... */ }
    ```

  - ```rust
    fn infer_schema<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, state: &''life1 dyn Session, store: &''life2 Arc<dyn ObjectStore>, objects: &''life3 [ObjectMeta]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<SchemaRef>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn infer_stats<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, store: &''life2 Arc<dyn ObjectStore>, table_schema: SchemaRef, object: &''life3 ObjectMeta) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Statistics>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn create_physical_plan<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, conf: FileScanConfig, filters: Option<&''life2 Arc<dyn PhysicalExpr>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn create_writer_physical_plan<''life0, ''life1, ''async_trait>(self: &''life0 Self, input: Arc<dyn ExecutionPlan>, _state: &''life1 dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn supports_filters_pushdown(self: &Self, file_schema: &Schema, table_schema: &Schema, filters: &[&Expr]) -> Result<FilePushdownSupport> { /* ... */ }
    ```

  - ```rust
    fn file_source(self: &Self) -> Arc<dyn FileSource> { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
#### Struct `ParquetSink`

Implements [`DataSink`] for writing to a parquet file.

```rust
pub struct ParquetSink {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(config: FileSinkConfig, parquet_options: TableParquetOptions) -> Self { /* ... */ }
  ```
  Create from config.

- ```rust
  pub fn written(self: &Self) -> HashMap<Path, FileMetaData> { /* ... */ }
  ```
  Retrieve the file metadata for the written files, keyed to the path

- ```rust
  pub fn parquet_options(self: &Self) -> &TableParquetOptions { /* ... */ }
  ```
  Parquet options

###### Trait Implementations

- **UnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **FileSink**
  - ```rust
    fn config(self: &Self) -> &FileSinkConfig { /* ... */ }
    ```

  - ```rust
    fn spawn_writer_tasks_and_join<''life0, ''life1, ''async_trait>(self: &''life0 Self, context: &''life1 Arc<TaskContext>, demux_task: SpawnedTask<Result<()>>, file_stream_rx: DemuxedStreamReceiver, object_store: Arc<dyn ObjectStore>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<u64>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DataSink**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn schema(self: &Self) -> &SchemaRef { /* ... */ }
    ```

  - ```rust
    fn write_all<''life0, ''life1, ''async_trait>(self: &''life0 Self, data: SendableRecordBatchStream, context: &''life1 Arc<TaskContext>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<u64>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

### Functions

#### Function `apply_file_schema_type_coercions`

Apply necessary schema type coercions to make file schema match table schema.

This function performs two main types of transformations in a single pass:
1. Binary types to string types conversion - Converts binary data types to their
   corresponding string types when the table schema expects string data
2. Regular to view types conversion - Converts standard string/binary types to
   view types when the table schema uses view types

# Arguments
* `table_schema` - The table schema containing the desired types
* `file_schema` - The file schema to be transformed

# Returns
* `Some(Schema)` - If any transformations were applied, returns the transformed schema
* `None` - If no transformations were needed

```rust
pub fn apply_file_schema_type_coercions(table_schema: &arrow::datatypes::Schema, file_schema: &arrow::datatypes::Schema) -> Option<arrow::datatypes::Schema> { /* ... */ }
```

#### Function `coerce_int96_to_resolution`

Coerces the file schema's Timestamps to the provided TimeUnit if Parquet schema contains INT96.

```rust
pub fn coerce_int96_to_resolution(parquet_schema: &parquet::schema::types::SchemaDescriptor, file_schema: &arrow::datatypes::Schema, time_unit: &arrow::datatypes::TimeUnit) -> Option<arrow::datatypes::Schema> { /* ... */ }
```

#### Function `coerce_file_schema_to_view_type`

**Attributes:**

- `#[deprecated(since = "47.0.0", note =
"Use `apply_file_schema_type_coercions` instead")]`

**⚠️ Deprecated since 47.0.0**: Use `apply_file_schema_type_coercions` instead

Coerces the file schema if the table schema uses a view type.

```rust
pub fn coerce_file_schema_to_view_type(table_schema: &arrow::datatypes::Schema, file_schema: &arrow::datatypes::Schema) -> Option<arrow::datatypes::Schema> { /* ... */ }
```

#### Function `coerce_file_schema_to_string_type`

**Attributes:**

- `#[deprecated(since = "47.0.0", note =
"Use `apply_file_schema_type_coercions` instead")]`

**⚠️ Deprecated since 47.0.0**: Use `apply_file_schema_type_coercions` instead

If the table schema uses a string type, coerce the file schema to use a string type.

See [ParquetFormat::binary_as_string] for details

```rust
pub fn coerce_file_schema_to_string_type(table_schema: &arrow::datatypes::Schema, file_schema: &arrow::datatypes::Schema) -> Option<arrow::datatypes::Schema> { /* ... */ }
```

#### Function `transform_schema_to_view`

Transform a schema to use view types for Utf8 and Binary

See [ParquetFormat::force_view_types] for details

```rust
pub fn transform_schema_to_view(schema: &arrow::datatypes::Schema) -> arrow::datatypes::Schema { /* ... */ }
```

#### Function `transform_binary_to_string`

Transform a schema so that any binary types are strings

```rust
pub fn transform_binary_to_string(schema: &arrow::datatypes::Schema) -> arrow::datatypes::Schema { /* ... */ }
```

#### Function `fetch_parquet_metadata`

Fetches parquet metadata from ObjectStore for given object

This component is a subject to **change** in near future and is exposed for low level integrations
through [`ParquetFileReaderFactory`].

[`ParquetFileReaderFactory`]: crate::ParquetFileReaderFactory

```rust
pub async fn fetch_parquet_metadata(store: &dyn ObjectStore, meta: &object_store::ObjectMeta, size_hint: Option<usize>) -> datafusion_common::Result<parquet::file::metadata::ParquetMetaData> { /* ... */ }
```

#### Function `fetch_statistics`

Read and parse the statistics of the Parquet file at location `path`

See [`statistics_from_parquet_meta_calc`] for more details

```rust
pub async fn fetch_statistics(store: &dyn ObjectStore, table_schema: arrow::datatypes::SchemaRef, file: &object_store::ObjectMeta, metadata_size_hint: Option<usize>) -> datafusion_common::Result<datafusion_common::Statistics> { /* ... */ }
```

#### Function `statistics_from_parquet_meta_calc`

Convert statistics in [`ParquetMetaData`] into [`Statistics`] using [`StatisticsConverter`]

The statistics are calculated for each column in the table schema
using the row group statistics in the parquet metadata.

# Key behaviors:

1. Extracts row counts and byte sizes from all row groups
2. Applies schema type coercions to align file schema with table schema
3. Collects and aggregates statistics across row groups when available

# When there are no statistics:

If the Parquet file doesn't contain any statistics (has_statistics is false), the function returns a Statistics object with:
- Exact row count
- Exact byte size
- All column statistics marked as unknown via Statistics::unknown_column(&table_schema)
# When only some columns have statistics:

For columns with statistics:
- Min/max values are properly extracted and represented as Precision::Exact
- Null counts are calculated by summing across row groups

For columns without statistics,
- For min/max, there are two situations:
    1. The column isn't in arrow schema, then min/max values are set to Precision::Absent
    2. The column is in arrow schema, but not in parquet schema due to schema revolution, min/max values are set to Precision::Exact(null)
- Null counts are set to Precision::Exact(num_rows) (conservatively assuming all values could be null)

```rust
pub fn statistics_from_parquet_meta_calc(metadata: &parquet::file::metadata::ParquetMetaData, table_schema: arrow::datatypes::SchemaRef) -> datafusion_common::Result<datafusion_common::Statistics> { /* ... */ }
```

## Module `source`

ParquetSource implementation for reading parquet files

```rust
pub mod source { /* ... */ }
```

### Types

#### Struct `ParquetSource`

Execution plan for reading one or more Parquet files.

```text
            ▲
            │
            │  Produce a stream of
            │  RecordBatches
            │
┌───────────────────────┐
│                       │
│     DataSourceExec    │
│                       │
└───────────────────────┘
            ▲
            │  Asynchronously read from one
            │  or more parquet files via
            │  ObjectStore interface
            │
            │
  .───────────────────.
 │                     )
 │`───────────────────'│
 │    ObjectStore      │
 │.───────────────────.│
 │                     )
  `───────────────────'

```

# Example: Create a `DataSourceExec`
```
# use std::sync::Arc;
# use arrow::datatypes::Schema;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource_parquet::source::ParquetSource;
# use datafusion_datasource::PartitionedFile;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_physical_expr::expressions::lit;
# use datafusion_datasource::source::DataSourceExec;
# use datafusion_common::config::TableParquetOptions;

# let file_schema = Arc::new(Schema::empty());
# let object_store_url = ObjectStoreUrl::local_filesystem();
# let predicate = lit(true);
let source = Arc::new(
    ParquetSource::default()
    .with_predicate(Arc::clone(&file_schema), predicate)
);
// Create a DataSourceExec for reading `file1.parquet` with a file size of 100MB
let config = FileScanConfigBuilder::new(object_store_url, file_schema, source)
   .with_file(PartitionedFile::new("file1.parquet", 100*1024*1024)).build();
let exec = DataSourceExec::from_data_source(config);
```

# Features

Supports the following optimizations:

* Concurrent reads: reads from one or more files in parallel as multiple
  partitions, including concurrently reading multiple row groups from a single
  file.

* Predicate push down: skips row groups, pages, rows based on metadata
  and late materialization. See "Predicate Pushdown" below.

* Projection pushdown: reads and decodes only the columns required.

* Limit pushdown: stop execution early after some number of rows are read.

* Custom readers: customize reading  parquet files, e.g. to cache metadata,
  coalesce I/O operations, etc. See [`ParquetFileReaderFactory`] for more
  details.

* Schema evolution: read parquet files with different schemas into a unified
  table schema. See [`SchemaAdapterFactory`] for more details.

* metadata_size_hint: controls the number of bytes read from the end of the
  file in the initial I/O when the default [`ParquetFileReaderFactory`]. If a
  custom reader is used, it supplies the metadata directly and this parameter
  is ignored. [`ParquetSource::with_metadata_size_hint`] for more details.

* User provided  `ParquetAccessPlan`s to skip row groups and/or pages
  based on external information. See "Implementing External Indexes" below

# Predicate Pushdown

`DataSourceExec` uses the provided [`PhysicalExpr`] predicate as a filter to
skip reading unnecessary data and improve query performance using several techniques:

* Row group pruning: skips entire row groups based on min/max statistics
  found in [`ParquetMetaData`] and any Bloom filters that are present.

* Page pruning: skips individual pages within a ColumnChunk using the
  [Parquet PageIndex], if present.

* Row filtering: skips rows within a page using a form of late
  materialization. When possible, predicates are applied by the parquet
  decoder *during* decode (see [`ArrowPredicate`] and [`RowFilter`] for more
  details). This is only enabled if `ParquetScanOptions::pushdown_filters` is set to true.

Note: If the predicate can not be used to accelerate the scan, it is ignored
(no error is raised on predicate evaluation errors).

[`ArrowPredicate`]: parquet::arrow::arrow_reader::ArrowPredicate
[`RowFilter`]: parquet::arrow::arrow_reader::RowFilter
[Parquet PageIndex]: https://github.com/apache/parquet-format/blob/master/PageIndex.md

# Example: rewriting `DataSourceExec`

You can modify a `DataSourceExec` using [`ParquetSource`], for example
to change files or add a predicate.

```no_run
# use std::sync::Arc;
# use arrow::datatypes::Schema;
# use datafusion_datasource::file_scan_config::FileScanConfig;
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource::source::DataSourceExec;

# fn parquet_exec() -> DataSourceExec { unimplemented!() }
// Split a single DataSourceExec into multiple DataSourceExecs, one for each file
let exec = parquet_exec();
let data_source = exec.data_source();
let base_config = data_source.as_any().downcast_ref::<FileScanConfig>().unwrap();
let existing_file_groups = &base_config.file_groups;
let new_execs = existing_file_groups
  .iter()
  .map(|file_group| {
    // create a new exec by copying the existing exec's source config
    let new_config = base_config
        .clone()
       .with_file_groups(vec![file_group.clone()]);

    (DataSourceExec::from_data_source(new_config))
  })
  .collect::<Vec<_>>();
```

# Implementing External Indexes

It is possible to restrict the row groups and selections within those row
groups that the DataSourceExec will consider by providing an initial
`ParquetAccessPlan` as `extensions` on `PartitionedFile`. This can be
used to implement external indexes on top of parquet files and select only
portions of the files.

The `DataSourceExec` will try and reduce any provided `ParquetAccessPlan`
further based on the contents of `ParquetMetadata` and other settings.

## Example of providing a ParquetAccessPlan

```
# use std::sync::Arc;
# use arrow::datatypes::{Schema, SchemaRef};
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource_parquet::ParquetAccessPlan;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource_parquet::source::ParquetSource;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_datasource::source::DataSourceExec;

# fn schema() -> SchemaRef {
#   Arc::new(Schema::empty())
# }
// create an access plan to scan row group 0, 1 and 3 and skip row groups 2 and 4
let mut access_plan = ParquetAccessPlan::new_all(5);
access_plan.skip(2);
access_plan.skip(4);
// provide the plan as extension to the FileScanConfig
let partitioned_file = PartitionedFile::new("my_file.parquet", 1234)
  .with_extensions(Arc::new(access_plan));
// create a FileScanConfig to scan this file
let config = FileScanConfigBuilder::new(ObjectStoreUrl::local_filesystem(), schema(), Arc::new(ParquetSource::default()))
    .with_file(partitioned_file).build();
// this parquet DataSourceExec will not even try to read row groups 2 and 4. Additional
// pruning based on predicates may also happen
let exec = DataSourceExec::from_data_source(config);
```

For a complete example, see the [`advanced_parquet_index` example]).

[`parquet_index_advanced` example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/advanced_parquet_index.rs

# Execution Overview

* Step 1: `DataSourceExec::execute` is called, returning a `FileStream`
  configured to open parquet files with a `ParquetOpener`.

* Step 2: When the stream is polled, the `ParquetOpener` is called to open
  the file.

* Step 3: The `ParquetOpener` gets the [`ParquetMetaData`] (file metadata)
  via [`ParquetFileReaderFactory`], creating a `ParquetAccessPlan` by
  applying predicates to metadata. The plan and projections are used to
  determine what pages must be read.

* Step 4: The stream begins reading data, fetching the required parquet
  pages incrementally decoding them, and applying any row filters (see
  [`Self::with_pushdown_filters`]).

* Step 5: As each [`RecordBatch`] is read, it may be adapted by a
  [`SchemaAdapter`] to match the table schema. By default missing columns are
  filled with nulls, but this can be customized via [`SchemaAdapterFactory`].

[`RecordBatch`]: arrow::record_batch::RecordBatch
[`SchemaAdapter`]: datafusion_datasource::schema_adapter::SchemaAdapter
[`ParquetMetadata`]: parquet::file::metadata::ParquetMetaData

```rust
pub struct ParquetSource {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(table_parquet_options: TableParquetOptions) -> Self { /* ... */ }
  ```
  Create a new ParquetSource to read the data specified in the file scan

- ```rust
  pub fn with_metadata_size_hint(self: Self, metadata_size_hint: usize) -> Self { /* ... */ }
  ```
  Set the metadata size hint

- ```rust
  pub fn with_predicate(self: &Self, file_schema: Arc<Schema>, predicate: Arc<dyn PhysicalExpr>) -> Self { /* ... */ }
  ```
  Set predicate information, also sets pruning_predicate and page_pruning_predicate attributes

- ```rust
  pub fn table_parquet_options(self: &Self) -> &TableParquetOptions { /* ... */ }
  ```
  Options passed to the parquet reader for this scan

- ```rust
  pub fn predicate(self: &Self) -> Option<&Arc<dyn PhysicalExpr>> { /* ... */ }
  ```
  Optional predicate.

- ```rust
  pub fn parquet_file_reader_factory(self: &Self) -> Option<&Arc<dyn ParquetFileReaderFactory>> { /* ... */ }
  ```
  return the optional file reader factory

- ```rust
  pub fn with_parquet_file_reader_factory(self: Self, parquet_file_reader_factory: Arc<dyn ParquetFileReaderFactory>) -> Self { /* ... */ }
  ```
  Optional user defined parquet file reader factory.

- ```rust
  pub fn schema_adapter_factory(self: &Self) -> Option<&Arc<dyn SchemaAdapterFactory>> { /* ... */ }
  ```
  return the optional schema adapter factory

- ```rust
  pub fn with_schema_adapter_factory(self: Self, schema_adapter_factory: Arc<dyn SchemaAdapterFactory>) -> Self { /* ... */ }
  ```
  Set optional schema adapter factory.

- ```rust
  pub fn with_pushdown_filters(self: Self, pushdown_filters: bool) -> Self { /* ... */ }
  ```
  If true, the predicate will be used during the parquet scan.

- ```rust
  pub fn with_reorder_filters(self: Self, reorder_filters: bool) -> Self { /* ... */ }
  ```
  If true, the `RowFilter` made by `pushdown_filters` may try to

- ```rust
  pub fn with_enable_page_index(self: Self, enable_page_index: bool) -> Self { /* ... */ }
  ```
  If enabled, the reader will read the page index

- ```rust
  pub fn with_bloom_filter_on_read(self: Self, bloom_filter_on_read: bool) -> Self { /* ... */ }
  ```
  If enabled, the reader will read by the bloom filter

- ```rust
  pub fn with_bloom_filter_on_write(self: Self, enable_bloom_filter_on_write: bool) -> Self { /* ... */ }
  ```
  If enabled, the writer will write by the bloom filter

###### Trait Implementations

- **FileSource**
  - ```rust
    fn create_file_opener(self: &Self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, partition: usize) -> Arc<dyn FileOpener> { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn with_batch_size(self: &Self, batch_size: usize) -> Arc<dyn FileSource> { /* ... */ }
    ```

  - ```rust
    fn with_schema(self: &Self, _schema: SchemaRef) -> Arc<dyn FileSource> { /* ... */ }
    ```

  - ```rust
    fn with_statistics(self: &Self, statistics: Statistics) -> Arc<dyn FileSource> { /* ... */ }
    ```

  - ```rust
    fn with_projection(self: &Self, _config: &FileScanConfig) -> Arc<dyn FileSource> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> &ExecutionPlanMetricsSet { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> datafusion_common::Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn file_type(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn fmt_extra(self: &Self, t: DisplayFormatType, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
- **IntoEither**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParquetSource { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ParquetSource { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Types

### Struct `ParquetExec`

**Attributes:**

- `#[deprecated(since = "46.0.0", note = "use DataSourceExec instead")]`

**⚠️ Deprecated since 46.0.0**: use DataSourceExec instead

Deprecated Execution plan replaced with DataSourceExec

```rust
pub struct ParquetExec {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(base_config: FileScanConfig, predicate: Option<Arc<dyn PhysicalExpr>>, metadata_size_hint: Option<usize>, table_parquet_options: TableParquetOptions) -> Self { /* ... */ }
  ```
  Create a new Parquet reader execution plan provided file list and schema.

- ```rust
  pub fn builder(file_scan_config: FileScanConfig) -> ParquetExecBuilder { /* ... */ }
  ```
  Return a [`ParquetExecBuilder`].

- ```rust
  pub fn into_builder(self: Self) -> ParquetExecBuilder { /* ... */ }
  ```
  Convert this `ParquetExec` into a builder for modification

- ```rust
  pub fn base_config(self: &Self) -> &FileScanConfig { /* ... */ }
  ```
  [`FileScanConfig`] that controls this scan (such as which files to read)

- ```rust
  pub fn table_parquet_options(self: &Self) -> &TableParquetOptions { /* ... */ }
  ```
  Options passed to the parquet reader for this scan

- ```rust
  pub fn predicate(self: &Self) -> Option<&Arc<dyn PhysicalExpr>> { /* ... */ }
  ```
  Optional predicate.

- ```rust
  pub fn pruning_predicate(self: &Self) -> Option<&Arc<PruningPredicate>> { /* ... */ }
  ```
  Optional reference to this parquet scan's pruning predicate

- ```rust
  pub fn parquet_file_reader_factory(self: &Self) -> Option<&Arc<dyn ParquetFileReaderFactory>> { /* ... */ }
  ```
  return the optional file reader factory

- ```rust
  pub fn with_parquet_file_reader_factory(self: Self, parquet_file_reader_factory: Arc<dyn ParquetFileReaderFactory>) -> Self { /* ... */ }
  ```
  Optional user defined parquet file reader factory.

- ```rust
  pub fn schema_adapter_factory(self: &Self) -> Option<&Arc<dyn SchemaAdapterFactory>> { /* ... */ }
  ```
  return the optional schema adapter factory

- ```rust
  pub fn with_schema_adapter_factory(self: Self, schema_adapter_factory: Arc<dyn SchemaAdapterFactory>) -> Self { /* ... */ }
  ```
  Set optional schema adapter factory.

- ```rust
  pub fn with_pushdown_filters(self: Self, pushdown_filters: bool) -> Self { /* ... */ }
  ```
  If true, the predicate will be used during the parquet scan.

- ```rust
  pub fn with_reorder_filters(self: Self, reorder_filters: bool) -> Self { /* ... */ }
  ```
  If true, the `RowFilter` made by `pushdown_filters` may try to

##### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParquetExec { /* ... */ }
    ```

- **MaybeSendSync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(exec: ParquetExec) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Sync**
- **RefUnwindSafe**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **ExecutionPlan**
  - ```rust
    fn name(self: &Self) -> &''static str { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```
    Return a reference to Any that can be used for downcasting

  - ```rust
    fn properties(self: &Self) -> &PlanProperties { /* ... */ }
    ```

  - ```rust
    fn children(self: &Self) -> Vec<&Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, _: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn repartitioned(self: &Self, target_partitions: usize, config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```
    Redistribute files across partitions according to their size

  - ```rust
    fn execute(self: &Self, partition_index: usize, ctx: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `ParquetExecBuilder`

**Attributes:**

- `#[deprecated(since = "46.0.0", note =
"use DataSourceExec with ParquetSource instead")]`
- `#[allow(unused, deprecated)]`

**⚠️ Deprecated since 46.0.0**: use DataSourceExec with ParquetSource instead

[`ParquetExecBuilder`], deprecated builder for [`ParquetExec`].

ParquetExec is replaced with `DataSourceExec` and it includes `ParquetSource`

See example on [`ParquetSource`].

```rust
pub struct ParquetExecBuilder {
    // Some fields omitted
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

#### Implementations

##### Methods

- ```rust
  pub fn new(file_scan_config: FileScanConfig) -> Self { /* ... */ }
  ```
  Create a new builder to read the provided file scan configuration

- ```rust
  pub fn new_with_options(file_scan_config: FileScanConfig, table_parquet_options: TableParquetOptions) -> Self { /* ... */ }
  ```
  Create a new builder to read the data specified in the file scan

- ```rust
  pub fn with_file_groups(self: Self, file_groups: Vec<FileGroup>) -> Self { /* ... */ }
  ```
  Update the list of files groups to read

- ```rust
  pub fn with_predicate(self: Self, predicate: Arc<dyn PhysicalExpr>) -> Self { /* ... */ }
  ```
  Set the filter predicate when reading.

- ```rust
  pub fn with_metadata_size_hint(self: Self, metadata_size_hint: usize) -> Self { /* ... */ }
  ```
  Set the metadata size hint

- ```rust
  pub fn with_table_parquet_options(self: Self, table_parquet_options: TableParquetOptions) -> Self { /* ... */ }
  ```
  Set the options for controlling how the ParquetExec reads parquet files.

- ```rust
  pub fn with_parquet_file_reader_factory(self: Self, parquet_file_reader_factory: Arc<dyn ParquetFileReaderFactory>) -> Self { /* ... */ }
  ```
  Set optional user defined parquet file reader factory.

- ```rust
  pub fn with_schema_adapter_factory(self: Self, schema_adapter_factory: Arc<dyn SchemaAdapterFactory>) -> Self { /* ... */ }
  ```
  Set optional schema adapter factory.

- ```rust
  pub fn build_arc(self: Self) -> Arc<ParquetExec> { /* ... */ }
  ```
  Convenience: build an `Arc`d `ParquetExec` from this builder

- ```rust
  pub fn build(self: Self) -> ParquetExec { /* ... */ }
  ```
  Build a [`ParquetExec`]

##### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(exec: ParquetExec) -> Self { /* ... */ }
    ```

- **MaybeSendSync**
## Re-exports

### Re-export `ParquetAccessPlan`

```rust
pub use access_plan::ParquetAccessPlan;
```

### Re-export `RowGroupAccess`

```rust
pub use access_plan::RowGroupAccess;
```

### Re-export `ParquetFileMetrics`

```rust
pub use metrics::ParquetFileMetrics;
```

### Re-export `PagePruningAccessPlanFilter`

```rust
pub use page_filter::PagePruningAccessPlanFilter;
```

### Re-export `DefaultParquetFileReaderFactory`

```rust
pub use reader::DefaultParquetFileReaderFactory;
```

### Re-export `ParquetFileReaderFactory`

```rust
pub use reader::ParquetFileReaderFactory;
```

### Re-export `build_row_filter`

```rust
pub use row_filter::build_row_filter;
```

### Re-export `can_expr_be_pushed_down_with_schemas`

```rust
pub use row_filter::can_expr_be_pushed_down_with_schemas;
```

### Re-export `RowGroupAccessPlanFilter`

```rust
pub use row_group_filter::RowGroupAccessPlanFilter;
```

### Re-export `plan_to_parquet`

```rust
pub use writer::plan_to_parquet;
```

### Re-export `file_format::*`

```rust
pub use file_format::*;
```


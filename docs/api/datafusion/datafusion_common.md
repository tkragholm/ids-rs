# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_common`

## Modules

## Module `alias`

```rust
pub mod alias { /* ... */ }
```

### Types

#### Struct `AliasGenerator`

A utility struct that can be used to generate unique aliases when optimizing queries

```rust
pub struct AliasGenerator {
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
  Create a new [`AliasGenerator`]

- ```rust
  pub fn next(self: &Self, prefix: &str) -> String { /* ... */ }
  ```
  Return a unique alias with the provided prefix

###### Trait Implementations

- **ErasedDestructor**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Allocation**
- **Sync**
- **Send**
## Module `cast`

This module provides DataFusion specific casting functions
that provide error handling. They are intended to "never fail"
but provide an error message rather than a panic, as the corresponding
kernels in arrow-rs such as `as_boolean_array` do.

```rust
pub mod cast { /* ... */ }
```

### Functions

#### Function `as_date32_array`

```rust
pub fn as_date32_array(array: &dyn Array) -> crate::Result<&arrow::array::Date32Array> { /* ... */ }
```

#### Function `as_date64_array`

```rust
pub fn as_date64_array(array: &dyn Array) -> crate::Result<&arrow::array::Date64Array> { /* ... */ }
```

#### Function `as_struct_array`

```rust
pub fn as_struct_array(array: &dyn Array) -> crate::Result<&arrow::array::StructArray> { /* ... */ }
```

#### Function `as_int8_array`

```rust
pub fn as_int8_array(array: &dyn Array) -> crate::Result<&arrow::array::Int8Array> { /* ... */ }
```

#### Function `as_uint8_array`

```rust
pub fn as_uint8_array(array: &dyn Array) -> crate::Result<&arrow::array::UInt8Array> { /* ... */ }
```

#### Function `as_int16_array`

```rust
pub fn as_int16_array(array: &dyn Array) -> crate::Result<&arrow::array::Int16Array> { /* ... */ }
```

#### Function `as_uint16_array`

```rust
pub fn as_uint16_array(array: &dyn Array) -> crate::Result<&arrow::array::UInt16Array> { /* ... */ }
```

#### Function `as_int32_array`

```rust
pub fn as_int32_array(array: &dyn Array) -> crate::Result<&arrow::array::Int32Array> { /* ... */ }
```

#### Function `as_uint32_array`

```rust
pub fn as_uint32_array(array: &dyn Array) -> crate::Result<&arrow::array::UInt32Array> { /* ... */ }
```

#### Function `as_int64_array`

```rust
pub fn as_int64_array(array: &dyn Array) -> crate::Result<&arrow::array::Int64Array> { /* ... */ }
```

#### Function `as_uint64_array`

```rust
pub fn as_uint64_array(array: &dyn Array) -> crate::Result<&arrow::array::UInt64Array> { /* ... */ }
```

#### Function `as_decimal128_array`

```rust
pub fn as_decimal128_array(array: &dyn Array) -> crate::Result<&arrow::array::Decimal128Array> { /* ... */ }
```

#### Function `as_decimal256_array`

```rust
pub fn as_decimal256_array(array: &dyn Array) -> crate::Result<&arrow::array::Decimal256Array> { /* ... */ }
```

#### Function `as_float16_array`

```rust
pub fn as_float16_array(array: &dyn Array) -> crate::Result<&arrow::array::Float16Array> { /* ... */ }
```

#### Function `as_float32_array`

```rust
pub fn as_float32_array(array: &dyn Array) -> crate::Result<&arrow::array::Float32Array> { /* ... */ }
```

#### Function `as_float64_array`

```rust
pub fn as_float64_array(array: &dyn Array) -> crate::Result<&arrow::array::Float64Array> { /* ... */ }
```

#### Function `as_string_array`

```rust
pub fn as_string_array(array: &dyn Array) -> crate::Result<&arrow::array::StringArray> { /* ... */ }
```

#### Function `as_string_view_array`

```rust
pub fn as_string_view_array(array: &dyn Array) -> crate::Result<&arrow::array::StringViewArray> { /* ... */ }
```

#### Function `as_large_string_array`

```rust
pub fn as_large_string_array(array: &dyn Array) -> crate::Result<&arrow::array::LargeStringArray> { /* ... */ }
```

#### Function `as_boolean_array`

```rust
pub fn as_boolean_array(array: &dyn Array) -> crate::Result<&arrow::array::BooleanArray> { /* ... */ }
```

#### Function `as_list_array`

```rust
pub fn as_list_array(array: &dyn Array) -> crate::Result<&arrow::array::ListArray> { /* ... */ }
```

#### Function `as_dictionary_array`

```rust
pub fn as_dictionary_array<T: ArrowDictionaryKeyType>(array: &dyn Array) -> crate::Result<&arrow::array::DictionaryArray<T>> { /* ... */ }
```

#### Function `as_generic_binary_array`

```rust
pub fn as_generic_binary_array<T: OffsetSizeTrait>(array: &dyn Array) -> crate::Result<&arrow::array::GenericBinaryArray<T>> { /* ... */ }
```

#### Function `as_generic_list_array`

```rust
pub fn as_generic_list_array<T: OffsetSizeTrait>(array: &dyn Array) -> crate::Result<&arrow::array::GenericListArray<T>> { /* ... */ }
```

#### Function `as_large_list_array`

```rust
pub fn as_large_list_array(array: &dyn Array) -> crate::Result<&arrow::array::LargeListArray> { /* ... */ }
```

#### Function `as_primitive_array`

```rust
pub fn as_primitive_array<T: ArrowPrimitiveType>(array: &dyn Array) -> crate::Result<&arrow::array::PrimitiveArray<T>> { /* ... */ }
```

#### Function `as_map_array`

```rust
pub fn as_map_array(array: &dyn Array) -> crate::Result<&arrow::array::MapArray> { /* ... */ }
```

#### Function `as_null_array`

```rust
pub fn as_null_array(array: &dyn Array) -> crate::Result<&arrow::array::NullArray> { /* ... */ }
```

#### Function `as_union_array`

```rust
pub fn as_union_array(array: &dyn Array) -> crate::Result<&arrow::array::UnionArray> { /* ... */ }
```

#### Function `as_time32_second_array`

```rust
pub fn as_time32_second_array(array: &dyn Array) -> crate::Result<&arrow::array::Time32SecondArray> { /* ... */ }
```

#### Function `as_time32_millisecond_array`

```rust
pub fn as_time32_millisecond_array(array: &dyn Array) -> crate::Result<&arrow::array::Time32MillisecondArray> { /* ... */ }
```

#### Function `as_time64_microsecond_array`

```rust
pub fn as_time64_microsecond_array(array: &dyn Array) -> crate::Result<&arrow::array::Time64MicrosecondArray> { /* ... */ }
```

#### Function `as_time64_nanosecond_array`

```rust
pub fn as_time64_nanosecond_array(array: &dyn Array) -> crate::Result<&arrow::array::Time64NanosecondArray> { /* ... */ }
```

#### Function `as_timestamp_nanosecond_array`

```rust
pub fn as_timestamp_nanosecond_array(array: &dyn Array) -> crate::Result<&arrow::array::TimestampNanosecondArray> { /* ... */ }
```

#### Function `as_timestamp_millisecond_array`

```rust
pub fn as_timestamp_millisecond_array(array: &dyn Array) -> crate::Result<&arrow::array::TimestampMillisecondArray> { /* ... */ }
```

#### Function `as_timestamp_microsecond_array`

```rust
pub fn as_timestamp_microsecond_array(array: &dyn Array) -> crate::Result<&arrow::array::TimestampMicrosecondArray> { /* ... */ }
```

#### Function `as_timestamp_second_array`

```rust
pub fn as_timestamp_second_array(array: &dyn Array) -> crate::Result<&arrow::array::TimestampSecondArray> { /* ... */ }
```

#### Function `as_interval_ym_array`

```rust
pub fn as_interval_ym_array(array: &dyn Array) -> crate::Result<&arrow::array::IntervalYearMonthArray> { /* ... */ }
```

#### Function `as_interval_dt_array`

```rust
pub fn as_interval_dt_array(array: &dyn Array) -> crate::Result<&arrow::array::IntervalDayTimeArray> { /* ... */ }
```

#### Function `as_interval_mdn_array`

```rust
pub fn as_interval_mdn_array(array: &dyn Array) -> crate::Result<&arrow::array::IntervalMonthDayNanoArray> { /* ... */ }
```

#### Function `as_binary_array`

```rust
pub fn as_binary_array(array: &dyn Array) -> crate::Result<&arrow::array::BinaryArray> { /* ... */ }
```

#### Function `as_binary_view_array`

```rust
pub fn as_binary_view_array(array: &dyn Array) -> crate::Result<&arrow::array::BinaryViewArray> { /* ... */ }
```

#### Function `as_large_binary_array`

```rust
pub fn as_large_binary_array(array: &dyn Array) -> crate::Result<&arrow::array::LargeBinaryArray> { /* ... */ }
```

#### Function `as_fixed_size_list_array`

```rust
pub fn as_fixed_size_list_array(array: &dyn Array) -> crate::Result<&arrow::array::FixedSizeListArray> { /* ... */ }
```

#### Function `as_fixed_size_binary_array`

```rust
pub fn as_fixed_size_binary_array(array: &dyn Array) -> crate::Result<&arrow::array::FixedSizeBinaryArray> { /* ... */ }
```

#### Function `as_generic_string_array`

```rust
pub fn as_generic_string_array<T: OffsetSizeTrait>(array: &dyn Array) -> crate::Result<&arrow::array::GenericStringArray<T>> { /* ... */ }
```

## Module `config`

Runtime configuration, via [`ConfigOptions`]

```rust
pub mod config { /* ... */ }
```

### Types

#### Struct `CatalogOptions`

Options related to catalog and directory scanning

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

```rust
pub struct CatalogOptions {
    pub create_default_catalog_and_schema: bool,
    pub default_catalog: String,
    pub default_schema: String,
    pub information_schema: bool,
    pub location: Option<String>,
    pub format: Option<String>,
    pub has_header: bool,
    pub newlines_in_values: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `create_default_catalog_and_schema` | `bool` | Whether the default catalog and schema should be created automatically. |
| `default_catalog` | `String` | The default catalog name - this impacts what SQL queries use if not specified |
| `default_schema` | `String` | The default schema name - this impacts what SQL queries use if not specified |
| `information_schema` | `bool` | Should DataFusion provide access to `information_schema`<br>virtual tables for displaying schema information |
| `location` | `Option<String>` | Location scanned to load tables for `default` schema |
| `format` | `Option<String>` | Type of `TableProvider` to use when loading `default` schema |
| `has_header` | `bool` | Default value for `format.has_header` for `CREATE EXTERNAL TABLE`<br>if not specified explicitly in the statement. |
| `newlines_in_values` | `bool` | Specifies whether newlines in (quoted) CSV values are supported.<br><br>This is the default value for `format.newlines_in_values` for `CREATE EXTERNAL TABLE`<br>if not specified explicitly in the statement.<br><br>Parsing newlines in quoted values may be affected by execution behaviour such as<br>parallel file scanning. Setting this to `true` ensures that newlines in values are<br>parsed successfully, which may reduce performance. |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CatalogOptions { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CatalogOptions) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **IntoEither**
- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> $crate::error::Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: $crate::config::Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **ErasedDestructor**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `SqlParserOptions`

Options related to SQL parser

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

```rust
pub struct SqlParserOptions {
    pub parse_float_as_decimal: bool,
    pub enable_ident_normalization: bool,
    pub enable_options_value_normalization: bool,
    pub dialect: String,
    pub support_varchar_with_length: bool,
    pub map_varchar_to_utf8view: bool,
    pub collect_spans: bool,
    pub recursion_limit: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `parse_float_as_decimal` | `bool` | When set to true, SQL parser will parse float as decimal type |
| `enable_ident_normalization` | `bool` | When set to true, SQL parser will normalize ident (convert ident to lowercase when not quoted) |
| `enable_options_value_normalization` | `bool` | When set to true, SQL parser will normalize options value (convert value to lowercase).<br>Note that this option is ignored and will be removed in the future. All case-insensitive values<br>are normalized automatically. |
| `dialect` | `String` | Configure the SQL dialect used by DataFusion's parser; supported values include: Generic,<br>MySQL, PostgreSQL, Hive, SQLite, Snowflake, Redshift, MsSQL, ClickHouse, BigQuery, Ansi, DuckDB and Databricks. |
| `support_varchar_with_length` | `bool` | If true, permit lengths for `VARCHAR` such as `VARCHAR(20)`, but<br>ignore the length. If false, error if a `VARCHAR` with a length is<br>specified. The Arrow type system does not have a notion of maximum<br>string length and thus DataFusion can not enforce such limits. |
| `map_varchar_to_utf8view` | `bool` | If true, `VARCHAR` is mapped to `Utf8View` during SQL planning.<br>If false, `VARCHAR` is mapped to `Utf8`  during SQL planning.<br>Default is false. |
| `collect_spans` | `bool` | When set to true, the source locations relative to the original SQL<br>query (i.e. [`Span`](https://docs.rs/sqlparser/latest/sqlparser/tokenizer/struct.Span.html)) will be collected<br>and recorded in the logical plan nodes. |
| `recursion_limit` | `usize` | Specifies the recursion depth limit when parsing complex SQL Queries |

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SqlParserOptions { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SqlParserOptions) -> bool { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> $crate::error::Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: $crate::config::Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

#### Struct `ExecutionOptions`

Options related to query execution

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

```rust
pub struct ExecutionOptions {
    pub batch_size: usize,
    pub coalesce_batches: bool,
    pub collect_statistics: bool,
    pub target_partitions: usize,
    pub time_zone: Option<String>,
    pub parquet: ParquetOptions,
    pub planning_concurrency: usize,
    pub skip_physical_aggregate_schema_check: bool,
    pub sort_spill_reservation_bytes: usize,
    pub sort_in_place_threshold_bytes: usize,
    pub meta_fetch_concurrency: usize,
    pub minimum_parallel_output_files: usize,
    pub soft_max_rows_per_output_file: usize,
    pub max_buffered_batches_per_output_file: usize,
    pub listing_table_ignore_subdirectory: bool,
    pub enable_recursive_ctes: bool,
    pub split_file_groups_by_statistics: bool,
    pub keep_partition_by_columns: bool,
    pub skip_partial_aggregation_probe_ratio_threshold: f64,
    pub skip_partial_aggregation_probe_rows_threshold: usize,
    pub use_row_number_estimates_to_optimize_partitioning: bool,
    pub enforce_batch_size_in_joins: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `batch_size` | `usize` | Default batch size while creating new batches, it's especially useful for<br>buffer-in-memory batches since creating tiny batches would result in too much<br>metadata memory consumption |
| `coalesce_batches` | `bool` | When set to true, record batches will be examined between each operator and<br>small batches will be coalesced into larger batches. This is helpful when there<br>are highly selective filters or joins that could produce tiny output batches. The<br>target batch size is determined by the configuration setting |
| `collect_statistics` | `bool` | Should DataFusion collect statistics after listing files |
| `target_partitions` | `usize` | Number of partitions for query execution. Increasing partitions can increase<br>concurrency.<br><br>Defaults to the number of CPU cores on the system |
| `time_zone` | `Option<String>` | The default time zone<br><br>Some functions, e.g. `EXTRACT(HOUR from SOME_TIME)`, shift the underlying datetime<br>according to this time zone, and then extract the hour |
| `parquet` | `ParquetOptions` | Parquet options |
| `planning_concurrency` | `usize` | Fan-out during initial physical planning.<br><br>This is mostly use to plan `UNION` children in parallel.<br><br>Defaults to the number of CPU cores on the system |
| `skip_physical_aggregate_schema_check` | `bool` | When set to true, skips verifying that the schema produced by<br>planning the input of `LogicalPlan::Aggregate` exactly matches the<br>schema of the input plan.<br><br>When set to false, if the schema does not match exactly<br>(including nullability and metadata), a planning error will be raised.<br><br>This is used to workaround bugs in the planner that are now caught by<br>the new schema verification step. |
| `sort_spill_reservation_bytes` | `usize` | Specifies the reserved memory for each spillable sort operation to<br>facilitate an in-memory merge.<br><br>When a sort operation spills to disk, the in-memory data must be<br>sorted and merged before being written to a file. This setting reserves<br>a specific amount of memory for that in-memory sort/merge process.<br><br>Note: This setting is irrelevant if the sort operation cannot spill<br>(i.e., if there's no `DiskManager` configured). |
| `sort_in_place_threshold_bytes` | `usize` | When sorting, below what size should data be concatenated<br>and sorted in a single RecordBatch rather than sorted in<br>batches and merged. |
| `meta_fetch_concurrency` | `usize` | Number of files to read in parallel when inferring schema and statistics |
| `minimum_parallel_output_files` | `usize` | Guarantees a minimum level of output files running in parallel.<br>RecordBatches will be distributed in round robin fashion to each<br>parallel writer. Each writer is closed and a new file opened once<br>soft_max_rows_per_output_file is reached. |
| `soft_max_rows_per_output_file` | `usize` | Target number of rows in output files when writing multiple.<br>This is a soft max, so it can be exceeded slightly. There also<br>will be one file smaller than the limit if the total<br>number of rows written is not roughly divisible by the soft max |
| `max_buffered_batches_per_output_file` | `usize` | This is the maximum number of RecordBatches buffered<br>for each output file being worked. Higher values can potentially<br>give faster write performance at the cost of higher peak<br>memory consumption |
| `listing_table_ignore_subdirectory` | `bool` | Should sub directories be ignored when scanning directories for data<br>files. Defaults to true (ignores subdirectories), consistent with<br>Hive. Note that this setting does not affect reading partitioned<br>tables (e.g. `/table/year=2021/month=01/data.parquet`). |
| `enable_recursive_ctes` | `bool` | Should DataFusion support recursive CTEs |
| `split_file_groups_by_statistics` | `bool` | Attempt to eliminate sorts by packing & sorting files with non-overlapping<br>statistics into the same file groups.<br>Currently experimental |
| `keep_partition_by_columns` | `bool` | Should DataFusion keep the columns used for partition_by in the output RecordBatches |
| `skip_partial_aggregation_probe_ratio_threshold` | `f64` | Aggregation ratio (number of distinct groups / number of input rows)<br>threshold for skipping partial aggregation. If the value is greater<br>then partial aggregation will skip aggregation for further input |
| `skip_partial_aggregation_probe_rows_threshold` | `usize` | Number of input rows partial aggregation partition should process, before<br>aggregation ratio check and trying to switch to skipping aggregation mode |
| `use_row_number_estimates_to_optimize_partitioning` | `bool` | Should DataFusion use row number estimates at the input to decide<br>whether increasing parallelism is beneficial or not. By default,<br>only exact row numbers (not estimates) are used for this decision.<br>Setting this flag to `true` will likely produce better plans.<br>if the source of statistics is accurate.<br>We plan to make this the default in the future. |
| `enforce_batch_size_in_joins` | `bool` | Should DataFusion enforce batch size in joins or not. By default,<br>DataFusion will not enforce batch size in joins. Enforcing batch size<br>in joins can reduce memory usage when joining large<br>tables with a highly-selective join filter, but is also slightly slower. |

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExecutionOptions { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
- **MaybeSendSync**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExecutionOptions) -> bool { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> $crate::error::Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: $crate::config::Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

#### Struct `ParquetOptions`

Options for reading and writing parquet files

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

```rust
pub struct ParquetOptions {
    pub enable_page_index: bool,
    pub pruning: bool,
    pub skip_metadata: bool,
    pub metadata_size_hint: Option<usize>,
    pub pushdown_filters: bool,
    pub reorder_filters: bool,
    pub schema_force_view_types: bool,
    pub binary_as_string: bool,
    pub coerce_int96: Option<String>,
    pub data_pagesize_limit: usize,
    pub write_batch_size: usize,
    pub writer_version: String,
    pub skip_arrow_metadata: bool,
    pub compression: Option<String>,
    pub dictionary_enabled: Option<bool>,
    pub dictionary_page_size_limit: usize,
    pub statistics_enabled: Option<String>,
    pub max_statistics_size: Option<usize>,
    pub max_row_group_size: usize,
    pub created_by: String,
    pub column_index_truncate_length: Option<usize>,
    pub statistics_truncate_length: Option<usize>,
    pub data_page_row_count_limit: usize,
    pub encoding: Option<String>,
    pub bloom_filter_on_read: bool,
    pub bloom_filter_on_write: bool,
    pub bloom_filter_fpp: Option<f64>,
    pub bloom_filter_ndv: Option<u64>,
    pub allow_single_file_parallelism: bool,
    pub maximum_parallel_row_group_writers: usize,
    pub maximum_buffered_record_batches_per_stream: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enable_page_index` | `bool` | (reading) If true, reads the Parquet data page level metadata (the<br>Page Index), if present, to reduce the I/O and number of<br>rows decoded. |
| `pruning` | `bool` | (reading) If true, the parquet reader attempts to skip entire row groups based<br>on the predicate in the query and the metadata (min/max values) stored in<br>the parquet file |
| `skip_metadata` | `bool` | (reading) If true, the parquet reader skip the optional embedded metadata that may be in<br>the file Schema. This setting can help avoid schema conflicts when querying<br>multiple parquet files with schemas containing compatible types but different metadata |
| `metadata_size_hint` | `Option<usize>` | (reading) If specified, the parquet reader will try and fetch the last `size_hint`<br>bytes of the parquet file optimistically. If not specified, two reads are required:<br>One read to fetch the 8-byte parquet footer and<br>another to fetch the metadata length encoded in the footer |
| `pushdown_filters` | `bool` | (reading) If true, filter expressions are be applied during the parquet decoding operation to<br>reduce the number of rows decoded. This optimization is sometimes called "late materialization". |
| `reorder_filters` | `bool` | (reading) If true, filter expressions evaluated during the parquet decoding operation<br>will be reordered heuristically to minimize the cost of evaluation. If false,<br>the filters are applied in the same order as written in the query |
| `schema_force_view_types` | `bool` | (reading) If true, parquet reader will read columns of `Utf8/Utf8Large` with `Utf8View`,<br>and `Binary/BinaryLarge` with `BinaryView`. |
| `binary_as_string` | `bool` | (reading) If true, parquet reader will read columns of<br>`Binary/LargeBinary` with `Utf8`, and `BinaryView` with `Utf8View`.<br><br>Parquet files generated by some legacy writers do not correctly set<br>the UTF8 flag for strings, causing string columns to be loaded as<br>BLOB instead. |
| `coerce_int96` | `Option<String>` | (reading) If true, parquet reader will read columns of<br>physical type int96 as originating from a different resolution<br>than nanosecond. This is useful for reading data from systems like Spark<br>which stores microsecond resolution timestamps in an int96 allowing it<br>to write values with a larger date range than 64-bit timestamps with<br>nanosecond resolution. |
| `data_pagesize_limit` | `usize` | (writing) Sets best effort maximum size of data page in bytes |
| `write_batch_size` | `usize` | (writing) Sets write_batch_size in bytes |
| `writer_version` | `String` | (writing) Sets parquet writer version<br>valid values are "1.0" and "2.0" |
| `skip_arrow_metadata` | `bool` | (writing) Skip encoding the embedded arrow metadata in the KV_meta<br><br>This is analogous to the `ArrowWriterOptions::with_skip_arrow_metadata`.<br>Refer to <https://docs.rs/parquet/53.3.0/parquet/arrow/arrow_writer/struct.ArrowWriterOptions.html#method.with_skip_arrow_metadata> |
| `compression` | `Option<String>` | (writing) Sets default parquet compression codec.<br>Valid values are: uncompressed, snappy, gzip(level),<br>lzo, brotli(level), lz4, zstd(level), and lz4_raw.<br>These values are not case sensitive. If NULL, uses<br>default parquet writer setting<br><br>Note that this default setting is not the same as<br>the default parquet writer setting. |
| `dictionary_enabled` | `Option<bool>` | (writing) Sets if dictionary encoding is enabled. If NULL, uses<br>default parquet writer setting |
| `dictionary_page_size_limit` | `usize` | (writing) Sets best effort maximum dictionary page size, in bytes |
| `statistics_enabled` | `Option<String>` | (writing) Sets if statistics are enabled for any column<br>Valid values are: "none", "chunk", and "page"<br>These values are not case sensitive. If NULL, uses<br>default parquet writer setting |
| `max_statistics_size` | `Option<usize>` | (writing) Sets max statistics size for any column. If NULL, uses<br>default parquet writer setting<br>max_statistics_size is deprecated, currently it is not being used |
| `max_row_group_size` | `usize` | (writing) Target maximum number of rows in each row group (defaults to 1M<br>rows). Writing larger row groups requires more memory to write, but<br>can get better compression and be faster to read. |
| `created_by` | `String` | (writing) Sets "created by" property |
| `column_index_truncate_length` | `Option<usize>` | (writing) Sets column index truncate length |
| `statistics_truncate_length` | `Option<usize>` | (writing) Sets statictics truncate length. If NULL, uses<br>default parquet writer setting |
| `data_page_row_count_limit` | `usize` | (writing) Sets best effort maximum number of rows in data page |
| `encoding` | `Option<String>` | (writing)  Sets default encoding for any column.<br>Valid values are: plain, plain_dictionary, rle,<br>bit_packed, delta_binary_packed, delta_length_byte_array,<br>delta_byte_array, rle_dictionary, and byte_stream_split.<br>These values are not case sensitive. If NULL, uses<br>default parquet writer setting |
| `bloom_filter_on_read` | `bool` | (writing) Use any available bloom filters when reading parquet files |
| `bloom_filter_on_write` | `bool` | (writing) Write bloom filters for all columns when creating parquet files |
| `bloom_filter_fpp` | `Option<f64>` | (writing) Sets bloom filter false positive probability. If NULL, uses<br>default parquet writer setting |
| `bloom_filter_ndv` | `Option<u64>` | (writing) Sets bloom filter number of distinct values. If NULL, uses<br>default parquet writer setting |
| `allow_single_file_parallelism` | `bool` | (writing) Controls whether DataFusion will attempt to speed up writing<br>parquet files by serializing them in parallel. Each column<br>in each row group in each output file are serialized in parallel<br>leveraging a maximum possible core count of n_files*n_row_groups*n_columns. |
| `maximum_parallel_row_group_writers` | `usize` | (writing) By default parallel parquet writer is tuned for minimum<br>memory usage in a streaming execution plan. You may see<br>a performance benefit when writing large parquet files<br>by increasing maximum_parallel_row_group_writers and<br>maximum_buffered_record_batches_per_stream if your system<br>has idle cores and can tolerate additional memory usage.<br>Boosting these values is likely worthwhile when<br>writing out already in-memory data, such as from a cached<br>data frame. |
| `maximum_buffered_record_batches_per_stream` | `usize` | (writing) By default parallel parquet writer is tuned for minimum<br>memory usage in a streaming execution plan. You may see<br>a performance benefit when writing large parquet files<br>by increasing maximum_parallel_row_group_writers and<br>maximum_buffered_record_batches_per_stream if your system<br>has idle cores and can tolerate additional memory usage.<br>Boosting these values is likely worthwhile when<br>writing out already in-memory data, such as from a cached<br>data frame. |

##### Implementations

###### Methods

- ```rust
  pub fn into_writer_properties_builder(self: &Self) -> Result<WriterPropertiesBuilder> { /* ... */ }
  ```
  Convert the global session options, [`ParquetOptions`], into a single write action's [`WriterPropertiesBuilder`].

###### Trait Implementations

- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> $crate::error::Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: $crate::config::Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Allocation**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParquetOptions { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParquetOptions) -> bool { /* ... */ }
    ```

#### Struct `OptimizerOptions`

Options related to query optimization

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

```rust
pub struct OptimizerOptions {
    pub enable_distinct_aggregation_soft_limit: bool,
    pub enable_round_robin_repartition: bool,
    pub enable_topk_aggregation: bool,
    pub filter_null_join_keys: bool,
    pub repartition_aggregations: bool,
    pub repartition_file_min_size: usize,
    pub repartition_joins: bool,
    pub allow_symmetric_joins_without_pruning: bool,
    pub repartition_file_scans: bool,
    pub repartition_windows: bool,
    pub repartition_sorts: bool,
    pub prefer_existing_sort: bool,
    pub skip_failed_rules: bool,
    pub max_passes: usize,
    pub top_down_join_key_reordering: bool,
    pub prefer_hash_join: bool,
    pub hash_join_single_partition_threshold: usize,
    pub hash_join_single_partition_threshold_rows: usize,
    pub default_filter_selectivity: u8,
    pub prefer_existing_union: bool,
    pub expand_views_at_output: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `enable_distinct_aggregation_soft_limit` | `bool` | When set to true, the optimizer will push a limit operation into<br>grouped aggregations which have no aggregate expressions, as a soft limit,<br>emitting groups once the limit is reached, before all rows in the group are read. |
| `enable_round_robin_repartition` | `bool` | When set to true, the physical plan optimizer will try to add round robin<br>repartitioning to increase parallelism to leverage more CPU cores |
| `enable_topk_aggregation` | `bool` | When set to true, the optimizer will attempt to perform limit operations<br>during aggregations, if possible |
| `filter_null_join_keys` | `bool` | When set to true, the optimizer will insert filters before a join between<br>a nullable and non-nullable column to filter out nulls on the nullable side. This<br>filter can add additional overhead when the file format does not fully support<br>predicate push down. |
| `repartition_aggregations` | `bool` | Should DataFusion repartition data using the aggregate keys to execute aggregates<br>in parallel using the provided `target_partitions` level |
| `repartition_file_min_size` | `usize` | Minimum total files size in bytes to perform file scan repartitioning. |
| `repartition_joins` | `bool` | Should DataFusion repartition data using the join keys to execute joins in parallel<br>using the provided `target_partitions` level |
| `allow_symmetric_joins_without_pruning` | `bool` | Should DataFusion allow symmetric hash joins for unbounded data sources even when<br>its inputs do not have any ordering or filtering If the flag is not enabled,<br>the SymmetricHashJoin operator will be unable to prune its internal buffers,<br>resulting in certain join types - such as Full, Left, LeftAnti, LeftSemi, Right,<br>RightAnti, and RightSemi - being produced only at the end of the execution.<br>This is not typical in stream processing. Additionally, without proper design for<br>long runner execution, all types of joins may encounter out-of-memory errors. |
| `repartition_file_scans` | `bool` | When set to `true`, file groups will be repartitioned to achieve maximum parallelism.<br>Currently Parquet and CSV formats are supported.<br><br>If set to `true`, all files will be repartitioned evenly (i.e., a single large file<br>might be partitioned into smaller chunks) for parallel scanning.<br>If set to `false`, different files will be read in parallel, but repartitioning won't<br>happen within a single file. |
| `repartition_windows` | `bool` | Should DataFusion repartition data using the partitions keys to execute window<br>functions in parallel using the provided `target_partitions` level |
| `repartition_sorts` | `bool` | Should DataFusion execute sorts in a per-partition fashion and merge<br>afterwards instead of coalescing first and sorting globally.<br>With this flag is enabled, plans in the form below<br><br>```text<br>     "SortExec: [a@0 ASC]",<br>     "  CoalescePartitionsExec",<br>     "    RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",<br>```<br>would turn into the plan below which performs better in multithreaded environments<br><br>```text<br>     "SortPreservingMergeExec: [a@0 ASC]",<br>     "  SortExec: [a@0 ASC]",<br>     "    RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",<br>``` |
| `prefer_existing_sort` | `bool` | When true, DataFusion will opportunistically remove sorts when the data is already sorted,<br>(i.e. setting `preserve_order` to true on `RepartitionExec`  and<br>using `SortPreservingMergeExec`)<br><br>When false, DataFusion will maximize plan parallelism using<br>`RepartitionExec` even if this requires subsequently resorting data using a `SortExec`. |
| `skip_failed_rules` | `bool` | When set to true, the logical plan optimizer will produce warning<br>messages if any optimization rules produce errors and then proceed to the next<br>rule. When set to false, any rules that produce errors will cause the query to fail |
| `max_passes` | `usize` | Number of times that the optimizer will attempt to optimize the plan |
| `top_down_join_key_reordering` | `bool` | When set to true, the physical plan optimizer will run a top down<br>process to reorder the join keys |
| `prefer_hash_join` | `bool` | When set to true, the physical plan optimizer will prefer HashJoin over SortMergeJoin.<br>HashJoin can work more efficiently than SortMergeJoin but consumes more memory |
| `hash_join_single_partition_threshold` | `usize` | The maximum estimated size in bytes for one input side of a HashJoin<br>will be collected into a single partition |
| `hash_join_single_partition_threshold_rows` | `usize` | The maximum estimated size in rows for one input side of a HashJoin<br>will be collected into a single partition |
| `default_filter_selectivity` | `u8` | The default filter selectivity used by Filter Statistics<br>when an exact selectivity cannot be determined. Valid values are<br>between 0 (no selectivity) and 100 (all rows are selected). |
| `prefer_existing_union` | `bool` | When set to true, the optimizer will not attempt to convert Union to Interleave |
| `expand_views_at_output` | `bool` | When set to true, if the returned type is a view type<br>then the output will be coerced to a non-view.<br>Coerces `Utf8View` to `LargeUtf8`, and `BinaryView` to `LargeBinary`. |

##### Implementations

###### Trait Implementations

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> OptimizerOptions { /* ... */ }
    ```

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OptimizerOptions) -> bool { /* ... */ }
    ```

- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> $crate::error::Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: $crate::config::Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **ErasedDestructor**
- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `ExplainOptions`

Options controlling explain output

See also: [`SessionConfig`]

[`SessionConfig`]: https://docs.rs/datafusion/latest/datafusion/prelude/struct.SessionConfig.html

```rust
pub struct ExplainOptions {
    pub logical_plan_only: bool,
    pub physical_plan_only: bool,
    pub show_statistics: bool,
    pub show_sizes: bool,
    pub show_schema: bool,
    pub format: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `logical_plan_only` | `bool` | When set to true, the explain statement will only print logical plans |
| `physical_plan_only` | `bool` | When set to true, the explain statement will only print physical plans |
| `show_statistics` | `bool` | When set to true, the explain statement will print operator statistics<br>for physical plans |
| `show_sizes` | `bool` | When set to true, the explain statement will print the partition sizes |
| `show_schema` | `bool` | When set to true, the explain statement will print schema information |
| `format` | `String` | Display format of explain. Default is "indent".<br>When set to "tree", it will print the plan in a tree-rendered format. |

##### Implementations

###### Trait Implementations

- **Allocation**
- **ErasedDestructor**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExplainOptions { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> $crate::error::Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: $crate::config::Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExplainOptions) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Sync**
#### Struct `ConfigEntry`

A key value pair, with a corresponding description

```rust
pub struct ConfigEntry {
    pub key: String,
    pub value: Option<String>,
    pub description: &''static str,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `String` | A unique string to identify this config value |
| `value` | `Option<String>` | The value if any |
| `description` | `&''static str` | A description of this configuration entry |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **MaybeSendSync**
- **UnwindSafe**
- **ErasedDestructor**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `ConfigOptions`

**Attributes:**

- `#[non_exhaustive]`

Configuration options struct, able to store both built-in configuration and custom options

```rust
pub struct ConfigOptions {
    pub catalog: CatalogOptions,
    pub execution: ExecutionOptions,
    pub optimizer: OptimizerOptions,
    pub sql_parser: SqlParserOptions,
    pub explain: ExplainOptions,
    pub extensions: Extensions,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `catalog` | `CatalogOptions` | Catalog options |
| `execution` | `ExecutionOptions` | Execution options |
| `optimizer` | `OptimizerOptions` | Optimizer options |
| `sql_parser` | `SqlParserOptions` | SQL parser options |
| `explain` | `ExplainOptions` | Explain options |
| `extensions` | `Extensions` | Optional extensions registered using [`Extensions::insert`] |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new [`ConfigOptions`] with default values

- ```rust
  pub fn with_extensions(self: Self, extensions: Extensions) -> Self { /* ... */ }
  ```
  Set extensions to provided value

- ```rust
  pub fn set(self: &mut Self, key: &str, value: &str) -> Result<()> { /* ... */ }
  ```
  Set a configuration option

- ```rust
  pub fn from_env() -> Result<Self> { /* ... */ }
  ```
  Create new ConfigOptions struct, taking values from

- ```rust
  pub fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self> { /* ... */ }
  ```
  Create new ConfigOptions struct, taking values from a string hash map.

- ```rust
  pub fn entries(self: &Self) -> Vec<ConfigEntry> { /* ... */ }
  ```
  Returns the [`ConfigEntry`] stored within this [`ConfigOptions`]

- ```rust
  pub fn generate_config_markdown() -> String { /* ... */ }
  ```
  Generate documentation that can be included in the user guide

###### Trait Implementations

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Default**
  - ```rust
    fn default() -> ConfigOptions { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: Visit>(self: &Self, v: &mut V, _key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ConfigOptions { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Extensions`

A type-safe container for [`ConfigExtension`]

```rust
pub struct Extensions(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new, empty [`Extensions`]

- ```rust
  pub fn insert<T: ConfigExtension>(self: &mut Self, extension: T) { /* ... */ }
  ```
  Registers a [`ConfigExtension`] with this [`ConfigOptions`]

- ```rust
  pub fn get<T: ConfigExtension>(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Retrieves the extension of the given type if any

- ```rust
  pub fn get_mut<T: ConfigExtension>(self: &mut Self) -> Option<&mut T> { /* ... */ }
  ```
  Retrieves the extension of the given type if any

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **ErasedDestructor**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Extensions { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Extensions { /* ... */ }
    ```

#### Enum `ConfigFileType`

These file types have special built in behavior for configuration.
Use TableOptions::Extensions for configuring other file types.

```rust
pub enum ConfigFileType {
    CSV,
    PARQUET,
    JSON,
}
```

##### Variants

###### `CSV`

###### `PARQUET`

###### `JSON`

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ConfigFileType { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **Allocation**
- **ErasedDestructor**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
#### Struct `TableOptions`

Represents the configuration options available for handling different table formats within a data processing application.
This struct encompasses options for various file formats including CSV, Parquet, and JSON, allowing for flexible configuration
of parsing and writing behaviors specific to each format. Additionally, it supports extending functionality through custom extensions.

```rust
pub struct TableOptions {
    pub csv: CsvOptions,
    pub parquet: TableParquetOptions,
    pub json: JsonOptions,
    pub current_format: Option<ConfigFileType>,
    pub extensions: Extensions,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `csv` | `CsvOptions` | Configuration options for CSV file handling. This includes settings like the delimiter,<br>quote character, and whether the first row is considered as headers. |
| `parquet` | `TableParquetOptions` | Configuration options for Parquet file handling. This includes settings for compression,<br>encoding, and other Parquet-specific file characteristics. |
| `json` | `JsonOptions` | Configuration options for JSON file handling. |
| `current_format` | `Option<ConfigFileType>` | The current file format that the table operations should assume. This option allows<br>for dynamic switching between the supported file types (e.g., CSV, Parquet, JSON). |
| `extensions` | `Extensions` | Optional extensions that can be used to extend or customize the behavior of the table<br>options. Extensions can be registered using `Extensions::insert` and might include<br>custom file handling logic, additional configuration parameters, or other enhancements. |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Constructs a new instance of `TableOptions` with default settings.

- ```rust
  pub fn default_from_session_config(config: &ConfigOptions) -> Self { /* ... */ }
  ```
  Creates a new `TableOptions` instance initialized with settings from a given session config.

- ```rust
  pub fn combine_with_session_config(self: &Self, config: &ConfigOptions) -> Self { /* ... */ }
  ```
  Updates the current `TableOptions` with settings from a given session config.

- ```rust
  pub fn set_config_format(self: &mut Self, format: ConfigFileType) { /* ... */ }
  ```
  Sets the file format for the table.

- ```rust
  pub fn with_extensions(self: Self, extensions: Extensions) -> Self { /* ... */ }
  ```
  Sets the extensions for this `TableOptions` instance.

- ```rust
  pub fn set(self: &mut Self, key: &str, value: &str) -> Result<()> { /* ... */ }
  ```
  Sets a specific configuration option.

- ```rust
  pub fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self> { /* ... */ }
  ```
  Initializes a new `TableOptions` from a hash map of string settings.

- ```rust
  pub fn alter_with_string_hash_map(self: &mut Self, settings: &HashMap<String, String>) -> Result<()> { /* ... */ }
  ```
  Modifies the current `TableOptions` instance with settings from a hash map.

- ```rust
  pub fn entries(self: &Self) -> Vec<ConfigEntry> { /* ... */ }
  ```
  Retrieves all configuration entries from this `TableOptions`.

###### Trait Implementations

- **Send**
- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **ConfigField**
  - ```rust
    fn visit<V: Visit>(self: &Self, v: &mut V, _key_prefix: &str, _description: &''static str) { /* ... */ }
    ```
    Visits configuration settings for the current file format, or all formats if none is selected.

  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> Result<()> { /* ... */ }
    ```
    Sets a configuration value for a specific key within `TableOptions`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TableOptions { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> TableOptions { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
#### Struct `TableParquetOptions`

Options that control how Parquet files are read, including global options
that apply to all columns and optional column-specific overrides

Closely tied to [`ParquetWriterOptions`](crate::file_options::parquet_writer::ParquetWriterOptions).
Properties not included in [`TableParquetOptions`] may not be configurable at the external API
(e.g. sorting_columns).

```rust
pub struct TableParquetOptions {
    pub global: ParquetOptions,
    pub column_specific_options: std::collections::HashMap<String, ParquetColumnOptions>,
    pub key_value_metadata: std::collections::HashMap<String, Option<String>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `global` | `ParquetOptions` | Global Parquet options that propagates to all columns. |
| `column_specific_options` | `std::collections::HashMap<String, ParquetColumnOptions>` | Column specific options. Default usage is parquet.XX::column. |
| `key_value_metadata` | `std::collections::HashMap<String, Option<String>>` | Additional file-level metadata to include. Inserted into the key_value_metadata<br>for the written [`FileMetaData`](https://docs.rs/parquet/latest/parquet/file/metadata/struct.FileMetaData.html).<br><br>Multiple entries are permitted<br>```sql<br>OPTIONS (<br>   'format.metadata::key1' '',<br>   'format.metadata::key2' 'value',<br>   'format.metadata::key3' 'value has spaces',<br>   'format.metadata::key4' 'value has special chars :: :',<br>   'format.metadata::key_dupe' 'original will be overwritten',<br>   'format.metadata::key_dupe' 'final'<br>)<br>``` |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Return new default TableParquetOptions

- ```rust
  pub fn with_skip_arrow_metadata(self: Self, skip: bool) -> Self { /* ... */ }
  ```
  Set whether the encoding of the arrow metadata should occur

- ```rust
  pub fn arrow_schema(self: &mut Self, schema: &Arc<Schema>) { /* ... */ }
  ```
  Add the arrow schema to the parquet kv_metadata.

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TableParquetOptions) -> bool { /* ... */ }
    ```

- **ConfigField**
  - ```rust
    fn visit<V: Visit>(self: &Self, v: &mut V, key_prefix: &str, description: &''static str) { /* ... */ }
    ```

  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> Result<()> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TableParquetOptions { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> TableParquetOptions { /* ... */ }
    ```

- **Send**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(parquet_table_options: &TableParquetOptions) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(table_parquet_options: &TableParquetOptions) -> Result<Self> { /* ... */ }
    ```
    Convert the session's [`TableParquetOptions`] into a single write action's [`WriterPropertiesBuilder`].

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **ErasedDestructor**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **Allocation**
- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `ParquetColumnOptions`

Options controlling parquet format for individual columns.

See [`ParquetOptions`] for more details

```rust
pub struct ParquetColumnOptions {
    pub bloom_filter_enabled: Option<bool>,
    pub encoding: Option<String>,
    pub dictionary_enabled: Option<bool>,
    pub compression: Option<String>,
    pub statistics_enabled: Option<String>,
    pub bloom_filter_fpp: Option<f64>,
    pub bloom_filter_ndv: Option<u64>,
    pub max_statistics_size: Option<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `bloom_filter_enabled` | `Option<bool>` | Sets if bloom filter is enabled for the column path. |
| `encoding` | `Option<String>` | Sets encoding for the column path.<br>Valid values are: plain, plain_dictionary, rle,<br>bit_packed, delta_binary_packed, delta_length_byte_array,<br>delta_byte_array, rle_dictionary, and byte_stream_split.<br>These values are not case-sensitive. If NULL, uses<br>default parquet options |
| `dictionary_enabled` | `Option<bool>` | Sets if dictionary encoding is enabled for the column path. If NULL, uses<br>default parquet options |
| `compression` | `Option<String>` | Sets default parquet compression codec for the column path.<br>Valid values are: uncompressed, snappy, gzip(level),<br>lzo, brotli(level), lz4, zstd(level), and lz4_raw.<br>These values are not case-sensitive. If NULL, uses<br>default parquet options |
| `statistics_enabled` | `Option<String>` | Sets if statistics are enabled for the column<br>Valid values are: "none", "chunk", and "page"<br>These values are not case sensitive. If NULL, uses<br>default parquet options |
| `bloom_filter_fpp` | `Option<f64>` | Sets bloom filter false positive probability for the column path. If NULL, uses<br>default parquet options |
| `bloom_filter_ndv` | `Option<u64>` | Sets bloom filter number of distinct values. If NULL, uses<br>default parquet options |
| `max_statistics_size` | `Option<usize>` | Sets max statistics size for the column path. If NULL, uses<br>default parquet options<br>max_statistics_size is deprecated, currently it is not being used |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

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

- **Allocation**
- **IntoEither**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParquetColumnOptions { /* ... */ }
    ```

- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ParquetColumnOptions) -> bool { /* ... */ }
    ```

#### Struct `CsvOptions`

Options controlling CSV format

```rust
pub struct CsvOptions {
    pub has_header: Option<bool>,
    pub delimiter: u8,
    pub quote: u8,
    pub terminator: Option<u8>,
    pub escape: Option<u8>,
    pub double_quote: Option<bool>,
    pub newlines_in_values: Option<bool>,
    pub compression: crate::parsers::CompressionTypeVariant,
    pub schema_infer_max_rec: Option<usize>,
    pub date_format: Option<String>,
    pub datetime_format: Option<String>,
    pub timestamp_format: Option<String>,
    pub timestamp_tz_format: Option<String>,
    pub time_format: Option<String>,
    pub null_value: Option<String>,
    pub null_regex: Option<String>,
    pub comment: Option<u8>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `has_header` | `Option<bool>` | Specifies whether there is a CSV header (i.e. the first line<br>consists of is column names). The value `None` indicates that<br>the configuration should be consulted. |
| `delimiter` | `u8` |  |
| `quote` | `u8` |  |
| `terminator` | `Option<u8>` |  |
| `escape` | `Option<u8>` |  |
| `double_quote` | `Option<bool>` |  |
| `newlines_in_values` | `Option<bool>` | Specifies whether newlines in (quoted) values are supported.<br><br>Parsing newlines in quoted values may be affected by execution behaviour such as<br>parallel file scanning. Setting this to `true` ensures that newlines in values are<br>parsed successfully, which may reduce performance.<br><br>The default behaviour depends on the `datafusion.catalog.newlines_in_values` setting. |
| `compression` | `crate::parsers::CompressionTypeVariant` |  |
| `schema_infer_max_rec` | `Option<usize>` |  |
| `date_format` | `Option<String>` |  |
| `datetime_format` | `Option<String>` |  |
| `timestamp_format` | `Option<String>` |  |
| `timestamp_tz_format` | `Option<String>` |  |
| `time_format` | `Option<String>` |  |
| `null_value` | `Option<String>` |  |
| `null_regex` | `Option<String>` |  |
| `comment` | `Option<u8>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn with_compression(self: Self, compression_type_variant: CompressionTypeVariant) -> Self { /* ... */ }
  ```
  Set a limit in terms of records to scan to infer the schema

- ```rust
  pub fn with_schema_infer_max_rec(self: Self, max_rec: usize) -> Self { /* ... */ }
  ```
  Set a limit in terms of records to scan to infer the schema

- ```rust
  pub fn with_has_header(self: Self, has_header: bool) -> Self { /* ... */ }
  ```
  Set true to indicate that the first line is a header.

- ```rust
  pub fn has_header(self: &Self) -> Option<bool> { /* ... */ }
  ```
  Returns true if the first line is a header. If format options does not

- ```rust
  pub fn with_delimiter(self: Self, delimiter: u8) -> Self { /* ... */ }
  ```
  The character separating values within a row.

- ```rust
  pub fn with_quote(self: Self, quote: u8) -> Self { /* ... */ }
  ```
  The quote character in a row.

- ```rust
  pub fn with_terminator(self: Self, terminator: Option<u8>) -> Self { /* ... */ }
  ```
  The character that terminates a row.

- ```rust
  pub fn with_escape(self: Self, escape: Option<u8>) -> Self { /* ... */ }
  ```
  The escape character in a row.

- ```rust
  pub fn with_double_quote(self: Self, double_quote: bool) -> Self { /* ... */ }
  ```
  Set true to indicate that the CSV quotes should be doubled.

- ```rust
  pub fn with_newlines_in_values(self: Self, newlines_in_values: bool) -> Self { /* ... */ }
  ```
  Specifies whether newlines in (quoted) values are supported.

- ```rust
  pub fn with_file_compression_type(self: Self, compression: CompressionTypeVariant) -> Self { /* ... */ }
  ```
  Set a `CompressionTypeVariant` of CSV

- ```rust
  pub fn delimiter(self: &Self) -> u8 { /* ... */ }
  ```
  The delimiter character.

- ```rust
  pub fn quote(self: &Self) -> u8 { /* ... */ }
  ```
  The quote character.

- ```rust
  pub fn terminator(self: &Self) -> Option<u8> { /* ... */ }
  ```
  The terminator character.

- ```rust
  pub fn escape(self: &Self) -> Option<u8> { /* ... */ }
  ```
  The escape character.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &CsvOptions) -> Result<Self> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CsvOptions { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Send**
- **Sync**
- **Allocation**
- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CsvOptions) -> bool { /* ... */ }
    ```

- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> $crate::error::Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: $crate::config::Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `JsonOptions`

Options controlling JSON format

```rust
pub struct JsonOptions {
    pub compression: crate::parsers::CompressionTypeVariant,
    pub schema_infer_max_rec: Option<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `compression` | `crate::parsers::CompressionTypeVariant` |  |
| `schema_infer_max_rec` | `Option<usize>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &JsonOptions) -> Result<Self> { /* ... */ }
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

- **IntoEither**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> JsonOptions { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &JsonOptions) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Allocation**
- **ConfigField**
  - ```rust
    fn set(self: &mut Self, key: &str, value: &str) -> $crate::error::Result<()> { /* ... */ }
    ```

  - ```rust
    fn visit<V: $crate::config::Visit>(self: &Self, v: &mut V, key_prefix: &str, _description: &''static str) { /* ... */ }
    ```

#### Enum `FormatOptions`

**Attributes:**

- `#[allow(clippy::large_enum_variant)]`

```rust
pub enum FormatOptions {
    CSV(CsvOptions),
    JSON(JsonOptions),
    PARQUET(TableParquetOptions),
    AVRO,
    ARROW,
}
```

##### Variants

###### `CSV`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CsvOptions` |  |

###### `JSON`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `JsonOptions` |  |

###### `PARQUET`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TableParquetOptions` |  |

###### `AVRO`

###### `ARROW`

##### Implementations

###### Trait Implementations

- **Send**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **RefUnwindSafe**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FormatOptions { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Allocation**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FormatOptions) -> bool { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

### Traits

#### Trait `ConfigExtension`

[`ConfigExtension`] provides a mechanism to store third-party configuration
within DataFusion [`ConfigOptions`]

This mechanism can be used to pass configuration to user defined functions
or optimizer passes

# Example
```
use datafusion_common::{
    config::ConfigExtension, extensions_options,
    config::ConfigOptions,
};
 // Define a new configuration struct using the `extensions_options` macro
 extensions_options! {
    /// My own config options.
    pub struct MyConfig {
        /// Should "foo" be replaced by "bar"?
        pub foo_to_bar: bool, default = true

        /// How many "baz" should be created?
        pub baz_count: usize, default = 1337
    }
 }

 impl ConfigExtension for MyConfig {
    const PREFIX: &'static str = "my_config";
 }

 // set up config struct and register extension
 let mut config = ConfigOptions::default();
 config.extensions.insert(MyConfig::default());

 // overwrite config default
 config.set("my_config.baz_count", "42").unwrap();

 // check config state
 let my_config = config.extensions.get::<MyConfig>().unwrap();
 assert!(my_config.foo_to_bar,);
 assert_eq!(my_config.baz_count, 42,);
```

# Note:
Unfortunately associated constants are not currently object-safe, and so this
extends the object-safe [`ExtensionOptions`]

```rust
pub trait ConfigExtension: ExtensionOptions {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Constants

- `PREFIX`: Configuration namespace prefix to use

#### Trait `ExtensionOptions`

An object-safe API for storing arbitrary configuration.

See [`ConfigExtension`] for user defined configuration

```rust
pub trait ExtensionOptions: Send + Sync + fmt::Debug + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_any`: Return `self` as [`Any`]
- `as_any_mut`: Return `self` as [`Any`]
- `cloned`: Return a deep clone of this [`ExtensionOptions`]
- `set`: Set the given `key`, `value` pair
- `entries`: Returns the [`ConfigEntry`] stored in this [`ExtensionOptions`]

#### Trait `ConfigField`

A trait implemented by `config_namespace` and for field types that provides
the ability to walk and mutate the configuration tree

```rust
pub trait ConfigField {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `visit`
- `set`

##### Implementations

This trait is implemented for the following types:

- `CatalogOptions`
- `SqlParserOptions`
- `ExecutionOptions`
- `ParquetOptions`
- `OptimizerOptions`
- `ExplainOptions`
- `ConfigOptions`
- `Option<F>` with <F: ConfigField + Default>
- `String`
- `bool`
- `usize`
- `f64`
- `u64`
- `u8`
- `crate::parsers::CompressionTypeVariant`
- `TableOptions`
- `TableParquetOptions`
- `ParquetColumnOptions`
- `HashMap<String, ParquetColumnOptions>`
- `CsvOptions`
- `JsonOptions`

#### Trait `Visit`

An implementation trait used to recursively walk configuration

```rust
pub trait Visit {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `some`
- `none`

#### Trait `FormatOptionsExt`

```rust
pub trait FormatOptionsExt: Display {
    /* Associated items */
}
```

## Module `cse`

Common Subexpression Elimination logic implemented in [`CSE`] can be controlled with
a [`CSEController`], that defines how to eliminate common subtrees from a particular
[`TreeNode`] tree.

```rust
pub mod cse { /* ... */ }
```

### Types

#### Enum `FoundCommonNodes`

The result of potentially rewriting a list of [`TreeNode`]s to eliminate common
subtrees.

```rust
pub enum FoundCommonNodes<N> {
    No {
        original_nodes_list: Vec<Vec<N>>,
    },
    Yes {
        common_nodes: Vec<(N, String)>,
        new_nodes_list: Vec<Vec<N>>,
        original_nodes_list: Vec<Vec<N>>,
    },
}
```

##### Variants

###### `No`

No common [`TreeNode`]s were found

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `original_nodes_list` | `Vec<Vec<N>>` |  |

###### `Yes`

Common [`TreeNode`]s were found

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `common_nodes` | `Vec<(N, String)>` | extracted common [`TreeNode`] |
| `new_nodes_list` | `Vec<Vec<N>>` | new [`TreeNode`]s with common subtrees replaced |
| `original_nodes_list` | `Vec<Vec<N>>` | original [`TreeNode`]s |

##### Implementations

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Allocation**
- **Send**
- **RefUnwindSafe**
- **MaybeSendSync**
- **IntoEither**
- **ErasedDestructor**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `CSE`

The main entry point of Common Subexpression Elimination.

[`CSE`] requires a [`CSEController`], that defines how common subtrees of a particular
[`TreeNode`] tree can be eliminated. The elimination process can be started with the
[`CSE::extract_common_nodes()`] method.

```rust
pub struct CSE<N, C: CSEController<Node = N>> {
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
  pub fn new(controller: C) -> Self { /* ... */ }
  ```

- ```rust
  pub fn extract_common_nodes(self: &mut Self, nodes_list: Vec<Vec<N>>) -> Result<FoundCommonNodes<N>> { /* ... */ }
  ```
  Extracts common [`TreeNode`]s and rewrites `nodes_list`.

###### Trait Implementations

- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `HashNode`

Hashes the direct content of an [`TreeNode`] without recursing into its children.

This method is useful to incrementally compute hashes, such as in [`CSE`] which builds
a deep hash of a node and its descendants during the bottom-up phase of the first
traversal and so avoid computing the hash of the node and then the hash of its
descendants separately.

If a node doesn't have any children then the value returned by `hash_node()` is
similar to '.hash()`, but not necessarily returns the same value.

```rust
pub trait HashNode {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `hash_node`

##### Implementations

This trait is implemented for the following types:

- `std::sync::Arc<T>` with <T: HashNode + ?Sized>

#### Trait `Normalizeable`

The `Normalizeable` trait defines a method to determine whether a node can be normalized.

Normalization is the process of converting a node into a canonical form that can be used
to compare nodes for equality. This is useful in optimizations like Common Subexpression Elimination (CSE),
where semantically equivalent nodes (e.g., `a + b` and `b + a`) should be treated as equal.

```rust
pub trait Normalizeable {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `can_normalize`

#### Trait `NormalizeEq`

The `NormalizeEq` trait extends `Eq` and `Normalizeable` to provide a method for comparing
normalized nodes in optimizations like Common Subexpression Elimination (CSE).

The `normalize_eq` method ensures that two nodes that are semantically equivalent (after normalization)
are considered equal in CSE optimization, even if their original forms differ.

This trait allows for equality comparisons between nodes with equivalent semantics, regardless of their
internal representations.

```rust
pub trait NormalizeEq: Eq + Normalizeable {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `normalize_eq`

#### Trait `CSEController`

The [`TreeNode`] specific definition of elimination.

```rust
pub trait CSEController {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Node`: The type of the tree nodes.

###### Required Methods

- `conditional_children`: Splits the children to normal and conditionally evaluated ones or returns `None`
- `is_valid`
- `is_ignored`
- `generate_alias`
- `rewrite`

##### Provided Methods

- ```rust
  fn rewrite_f_down(self: &mut Self, _node: &<Self as >::Node) { /* ... */ }
  ```

- ```rust
  fn rewrite_f_up(self: &mut Self, _node: &<Self as >::Node) { /* ... */ }
  ```

## Module `diagnostic`

```rust
pub mod diagnostic { /* ... */ }
```

### Types

#### Struct `Diagnostic`

Additional contextual information intended for end users, to help them
understand what went wrong by providing human-readable messages, and
locations in the source query that relate to the error in some way.

You can think of a single [`Diagnostic`] as a single "block" of output from
rustc. i.e. either an error or a warning, optionally with some notes and
help messages.

Example:

```rust
# use datafusion_common::{Location, Span, Diagnostic};
let span = Some(Span {
    start: Location{ line: 2, column: 1 },
    end: Location{ line: 4, column: 15 }
});
let diagnostic = Diagnostic::new_error("Something went wrong", span)
    .with_help("Have you tried turning it on and off again?", None);
```

```rust
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Option<crate::Span>,
    pub notes: Vec<DiagnosticNote>,
    pub helps: Vec<DiagnosticHelp>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `kind` | `DiagnosticKind` |  |
| `message` | `String` |  |
| `span` | `Option<crate::Span>` |  |
| `notes` | `Vec<DiagnosticNote>` |  |
| `helps` | `Vec<DiagnosticHelp>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new_error</* synthetic */ impl Into<String>: Into<String>>(message: impl Into<String>, span: Option<Span>) -> Self { /* ... */ }
  ```
  Creates a new [`Diagnostic`] for a fatal error that prevents the SQL

- ```rust
  pub fn new_warning</* synthetic */ impl Into<String>: Into<String>>(message: impl Into<String>, span: Option<Span>) -> Self { /* ... */ }
  ```
  Creates a new [`Diagnostic`] for a NON-fatal warning, such as a

- ```rust
  pub fn add_note</* synthetic */ impl Into<String>: Into<String>>(self: &mut Self, message: impl Into<String>, span: Option<Span>) { /* ... */ }
  ```
  Adds a "note" to the [`Diagnostic`], which can have zero or many. A "note"

- ```rust
  pub fn add_help</* synthetic */ impl Into<String>: Into<String>>(self: &mut Self, message: impl Into<String>, span: Option<Span>) { /* ... */ }
  ```
  Adds a "help" to the [`Diagnostic`], which can have zero or many. A

- ```rust
  pub fn with_note</* synthetic */ impl Into<String>: Into<String>>(self: Self, message: impl Into<String>, span: Option<Span>) -> Self { /* ... */ }
  ```
  Like [`Diagnostic::add_note`], but returns `self` to allow chaining.

- ```rust
  pub fn with_help</* synthetic */ impl Into<String>: Into<String>>(self: Self, message: impl Into<String>, span: Option<Span>) -> Self { /* ... */ }
  ```
  Like [`Diagnostic::add_help`], but returns `self` to allow chaining.

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Allocation**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Diagnostic { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `DiagnosticNote`

A note enriches a [`Diagnostic`] with extra information, possibly referring
to different locations in the original SQL query, that helps contextualize
the error and helps the end user understand why it occurred.

Example:
SELECT id, name FROM users GROUP BY id
Note:      ^^^^ 'name' is not in the GROUP BY clause

```rust
pub struct DiagnosticNote {
    pub message: String,
    pub span: Option<crate::Span>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `message` | `String` |  |
| `span` | `Option<crate::Span>` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Allocation**
- **MaybeSendSync**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiagnosticNote { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `DiagnosticHelp`

A "help" enriches a [`Diagnostic`] with extra information, possibly
referring to different locations in the original SQL query, that helps the
user understand how they might fix the error or warning.

Example:
SELECT id, name FROM users GROUP BY id
Help: Add 'name' here                 ^^^^

```rust
pub struct DiagnosticHelp {
    pub message: String,
    pub span: Option<crate::Span>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `message` | `String` |  |
| `span` | `Option<crate::Span>` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiagnosticHelp { /* ... */ }
    ```

- **Allocation**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **Sync**
- **Unpin**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

#### Enum `DiagnosticKind`

A [`Diagnostic`] can either be a hard error that prevents the query from
being planned and executed, or a warning that indicates potential issues,
performance problems, or causes for unexpected results, but is non-fatal.
This enum expresses these two possibilities.

```rust
pub enum DiagnosticKind {
    Error,
    Warning,
}
```

##### Variants

###### `Error`

###### `Warning`

##### Implementations

###### Trait Implementations

- **Copy**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DiagnosticKind) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **Eq**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiagnosticKind { /* ... */ }
    ```

## Module `display`

Types for plan display

```rust
pub mod display { /* ... */ }
```

### Types

#### Enum `PlanType`

Represents which type of plan, when storing multiple
for use in EXPLAIN plans

```rust
pub enum PlanType {
    InitialLogicalPlan,
    AnalyzedLogicalPlan {
        analyzer_name: String,
    },
    FinalAnalyzedLogicalPlan,
    OptimizedLogicalPlan {
        optimizer_name: String,
    },
    FinalLogicalPlan,
    InitialPhysicalPlan,
    InitialPhysicalPlanWithStats,
    InitialPhysicalPlanWithSchema,
    OptimizedPhysicalPlan {
        optimizer_name: String,
    },
    FinalPhysicalPlan,
    FinalPhysicalPlanWithStats,
    FinalPhysicalPlanWithSchema,
    PhysicalPlanError,
}
```

##### Variants

###### `InitialLogicalPlan`

The initial LogicalPlan provided to DataFusion

###### `AnalyzedLogicalPlan`

The LogicalPlan which results from applying an analyzer pass

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `analyzer_name` | `String` | The name of the analyzer which produced this plan |

###### `FinalAnalyzedLogicalPlan`

The LogicalPlan after all analyzer passes have been applied

###### `OptimizedLogicalPlan`

The LogicalPlan which results from applying an optimizer pass

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `optimizer_name` | `String` | The name of the optimizer which produced this plan |

###### `FinalLogicalPlan`

The final, fully optimized LogicalPlan that was converted to a physical plan

###### `InitialPhysicalPlan`

The initial physical plan, prepared for execution

###### `InitialPhysicalPlanWithStats`

The initial physical plan with stats, prepared for execution

###### `InitialPhysicalPlanWithSchema`

The initial physical plan with schema, prepared for execution

###### `OptimizedPhysicalPlan`

The ExecutionPlan which results from applying an optimizer pass

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `optimizer_name` | `String` | The name of the optimizer which produced this plan |

###### `FinalPhysicalPlan`

The final, fully optimized physical plan which would be executed

###### `FinalPhysicalPlanWithStats`

The final with stats, fully optimized physical plan which would be executed

###### `FinalPhysicalPlanWithSchema`

The final with schema, fully optimized physical plan which would be executed

###### `PhysicalPlanError`

An error creating the physical plan

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PlanType) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Eq**
- **Send**
- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PlanType) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PlanType { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Sync**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Allocation**
#### Struct `StringifiedPlan`

Represents some sort of execution plan, in String form

```rust
pub struct StringifiedPlan {
    pub plan_type: PlanType,
    pub plan: std::sync::Arc<String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `plan_type` | `PlanType` | An identifier of what type of plan this string represents |
| `plan` | `std::sync::Arc<String>` | The string representation of the plan |

##### Implementations

###### Methods

- ```rust
  pub fn new</* synthetic */ impl Into<String>: Into<String>>(plan_type: PlanType, plan: impl Into<String>) -> Self { /* ... */ }
  ```
  Create a new Stringified plan of `plan_type` with string

- ```rust
  pub fn should_display(self: &Self, verbose_mode: bool) -> bool { /* ... */ }
  ```
  Returns true if this plan should be displayed. Generally

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StringifiedPlan) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StringifiedPlan { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &StringifiedPlan) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **IntoEither**
- **UnwindSafe**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
### Traits

#### Trait `ToStringifiedPlan`

Trait for something that can be formatted as a stringified plan

```rust
pub trait ToStringifiedPlan {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `to_stringified`: Create a stringified plan with the specified type

### Re-exports

#### Re-export `graphviz::*`

```rust
pub use graphviz::*;
```

## Module `error`

DataFusion error types

```rust
pub mod error { /* ... */ }
```

### Types

#### Type Alias `Result`

Result type for operations that could result in an [DataFusionError]

```rust
pub type Result<T, E = DataFusionError> = result::Result<T, E>;
```

#### Type Alias `SharedResult`

Result type for operations that could result in an [DataFusionError] and needs to be shared (wrapped into `Arc`).

```rust
pub type SharedResult<T> = result::Result<T, std::sync::Arc<DataFusionError>>;
```

#### Type Alias `GenericError`

Error type for generic operations that could result in DataFusionError::External

```rust
pub type GenericError = Box<dyn Error + Send + Sync>;
```

#### Enum `DataFusionError`

DataFusion error

```rust
pub enum DataFusionError {
    ArrowError(arrow::error::ArrowError, Option<String>),
    ParquetError(parquet::errors::ParquetError),
    ObjectStore(object_store::Error),
    IoError(io::Error),
    SQL(sqlparser::parser::ParserError, Option<String>),
    NotImplemented(String),
    Internal(String),
    Plan(String),
    Configuration(String),
    SchemaError(SchemaError, Box<Option<String>>),
    Execution(String),
    ExecutionJoin(tokio::task::JoinError),
    ResourcesExhausted(String),
    External(GenericError),
    Context(String, Box<DataFusionError>),
    Substrait(String),
    Diagnostic(Box<crate::Diagnostic>, Box<DataFusionError>),
    Collection(Vec<DataFusionError>),
    Shared(std::sync::Arc<DataFusionError>),
}
```

##### Variants

###### `ArrowError`

Error returned by arrow.

2nd argument is for optional backtrace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `arrow::error::ArrowError` |  |
| 1 | `Option<String>` |  |

###### `ParquetError`

Error when reading / writing Parquet data.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `parquet::errors::ParquetError` |  |

###### `ObjectStore`

Error when reading / writing to / from an object_store (e.g. S3 or LocalFile)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `object_store::Error` |  |

###### `IoError`

Error when an I/O operation fails

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `io::Error` |  |

###### `SQL`

Error when SQL is syntactically incorrect.

2nd argument is for optional backtrace

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `sqlparser::parser::ParserError` |  |
| 1 | `Option<String>` |  |

###### `NotImplemented`

Error when a feature is not yet implemented.

These errors are sometimes returned for features that are still in
development and are not entirely complete. Often, these errors are
tracked in our issue tracker.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Internal`

Error due to bugs in DataFusion

This error should not happen in normal usage of DataFusion. It results
from something that wasn't expected/anticipated by the implementation
and that is most likely a bug (the error message even encourages users
to open a bug report). A user should not be able to trigger internal
errors under normal circumstances by feeding in malformed queries, bad
data, etc.

Note that I/O errors (or any error that happens due to external systems)
do NOT fall under this category. See other variants such as
[`Self::IoError`] and [`Self::External`].

DataFusions has internal invariants that the compiler is not always able
to check. This error is raised when one of those invariants does not
hold for some reason.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Plan`

Error during planning of the query.

This error happens when the user provides a bad query or plan, for
example the user attempts to call a function that doesn't exist, or if
the types of a function call are not supported.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Configuration`

Error for invalid or unsupported configuration options.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `SchemaError`

Error when there is a problem with the query related to schema.

This error can be returned in cases such as when schema inference is not
possible and when column names are not unique.

2nd argument is for optional backtrace
Boxing the optional backtrace to prevent <https://rust-lang.github.io/rust-clippy/master/index.html#/result_large_err>

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `SchemaError` |  |
| 1 | `Box<Option<String>>` |  |

###### `Execution`

Error during execution of the query.

This error is returned when an error happens during execution due to a
malformed input. For example, the user passed malformed arguments to a
SQL method, opened a CSV file that is broken, or tried to divide an
integer by zero.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `ExecutionJoin`

[`JoinError`] during execution of the query.

This error can't occur for unjoined tasks, such as execution shutdown.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `tokio::task::JoinError` |  |

###### `ResourcesExhausted`

Error when resources (such as memory of scratch disk space) are exhausted.

This error is thrown when a consumer cannot acquire additional memory
or other resources needed to execute the query from the Memory Manager.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `External`

Errors originating from outside DataFusion's core codebase.

For example, a custom S3Error from the crate datafusion-objectstore-s3

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `GenericError` |  |

###### `Context`

Error with additional context

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |
| 1 | `Box<DataFusionError>` |  |

###### `Substrait`

Errors from either mapping LogicalPlans to/from Substrait plans
or serializing/deserializing protobytes to Substrait plans

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

###### `Diagnostic`

Error wrapped together with additional contextual information intended
for end users, to help them understand what went wrong by providing
human-readable messages, and locations in the source query that relate
to the error in some way.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<crate::Diagnostic>` |  |
| 1 | `Box<DataFusionError>` |  |

###### `Collection`

A collection of one or more [`DataFusionError`]. Useful in cases where
DataFusion can recover from an erroneous state, and produce more errors
before terminating. e.g. when planning a SELECT clause, DataFusion can
synchronize to the next `SelectItem` if the previous one had errors. The
end result is that the user can see errors about all `SelectItem`,
instead of just the first one.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<DataFusionError>` |  |

###### `Shared`

A [`DataFusionError`] which shares an underlying [`DataFusionError`].

This is useful when the same underlying [`DataFusionError`] is passed
to multiple receivers. For example, when the source of a repartition
errors and the error is propagated to multiple consumers.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<DataFusionError>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn find_root(self: &Self) -> &Self { /* ... */ }
  ```
  Get deepest underlying [`DataFusionError`]

- ```rust
  pub fn context</* synthetic */ impl Into<String>: Into<String>>(self: Self, description: impl Into<String>) -> Self { /* ... */ }
  ```
  wraps self in Self::Context with a description

- ```rust
  pub fn strip_backtrace(self: &Self) -> String { /* ... */ }
  ```
  Strips backtrace out of the error message

- ```rust
  pub fn get_back_trace() -> String { /* ... */ }
  ```
  To enable optional rust backtrace in DataFusion:

- ```rust
  pub fn builder() -> DataFusionErrorBuilder { /* ... */ }
  ```
  Return a [`DataFusionErrorBuilder`] to build a [`DataFusionError`]

- ```rust
  pub fn message(self: &Self) -> Cow<''_, str> { /* ... */ }
  ```

- ```rust
  pub fn with_diagnostic(self: Self, diagnostic: Diagnostic) -> Self { /* ... */ }
  ```
  Wraps the error with contextual information intended for end users

- ```rust
  pub fn with_diagnostic_fn<F: FnOnce(&DataFusionError) -> Diagnostic>(self: Self, f: F) -> Self { /* ... */ }
  ```
  Wraps the error with contextual information intended for end users.

- ```rust
  pub fn diagnostic(self: &Self) -> Option<&Diagnostic> { /* ... */ }
  ```
  Gets the [`Diagnostic`] associated with the error, if any. If there is

- ```rust
  pub fn iter(self: &Self) -> impl Iterator<Item = &DataFusionError> { /* ... */ }
  ```
  Return an iterator over this [`DataFusionError`] and any other

###### Trait Implementations

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **Unpin**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(_e: std::fmt::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: io::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: ArrowError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: DataFusionError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: &Arc<DataFusionError>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: ParquetError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: object_store::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: object_store::path::Error) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: ParserError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(err: GenericError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: DataFusionError) -> Self { /* ... */ }
    ```

- **Error**
  - ```rust
    fn source(self: &Self) -> Option<&dyn Error + ''static> { /* ... */ }
    ```

#### Enum `SchemaError`

Schema-related errors

```rust
pub enum SchemaError {
    AmbiguousReference {
        field: crate::Column,
    },
    DuplicateQualifiedField {
        qualifier: Box<crate::TableReference>,
        name: String,
    },
    DuplicateUnqualifiedField {
        name: String,
    },
    FieldNotFound {
        field: Box<crate::Column>,
        valid_fields: Vec<crate::Column>,
    },
}
```

##### Variants

###### `AmbiguousReference`

Schema contains a (possibly) qualified and unqualified field with same unqualified name

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `field` | `crate::Column` |  |

###### `DuplicateQualifiedField`

Schema contains duplicate qualified field name

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `qualifier` | `Box<crate::TableReference>` |  |
| `name` | `String` |  |

###### `DuplicateUnqualifiedField`

Schema contains duplicate unqualified field name

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `String` |  |

###### `FieldNotFound`

No field with this name

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `field` | `Box<crate::Column>` |  |
| `valid_fields` | `Vec<crate::Column>` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Unpin**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Error**
#### Struct `DataFusionErrorBuilder`

A builder for [`DataFusionError`]

This builder can be used to collect multiple errors and return them as a
[`DataFusionError::Collection`].

# Example: no errors
```
# use datafusion_common::DataFusionError;
let mut builder = DataFusionError::builder();
// ok_or returns the value if no errors have been added
assert_eq!(builder.error_or(42).unwrap(), 42);
```

# Example: with errors
```
# use datafusion_common::{assert_contains, DataFusionError};
let mut builder = DataFusionError::builder();
builder.add_error(DataFusionError::Internal("foo".to_owned()));
// ok_or returns the value if no errors have been added
assert_contains!(builder.error_or(42).unwrap_err().to_string(), "Internal error: foo");
```

```rust
pub struct DataFusionErrorBuilder(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new [`DataFusionErrorBuilder`]

- ```rust
  pub fn add_error(self: &mut Self, error: DataFusionError) { /* ... */ }
  ```
  Add an error to the in progress list

- ```rust
  pub fn with_error(self: Self, error: DataFusionError) -> Self { /* ... */ }
  ```
  Add an error to the in progress list, returning the builder

- ```rust
  pub fn error_or<T>(self: Self, ok: T) -> Result<T, DataFusionError> { /* ... */ }
  ```
  Returns `Ok(ok)` if no errors were added to the builder,

###### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> DataFusionErrorBuilder { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `field_not_found`

Create a "field not found" DataFusion::SchemaError

```rust
pub fn field_not_found<R: Into<crate::TableReference>>(qualifier: Option<R>, name: &str, schema: &crate::DFSchema) -> DataFusionError { /* ... */ }
```

#### Function `unqualified_field_not_found`

Convenience wrapper over [`field_not_found`] for when there is no qualifier

```rust
pub fn unqualified_field_not_found(name: &str, schema: &crate::DFSchema) -> DataFusionError { /* ... */ }
```

#### Function `add_possible_columns_to_diag`

```rust
pub fn add_possible_columns_to_diag(diagnostic: &mut crate::Diagnostic, field: &crate::Column, valid_fields: &[crate::Column]) { /* ... */ }
```

### Re-exports

#### Re-export `schema_err`

```rust
pub use schema_err as _schema_err;
```

## Module `file_options`

Options related to how files should be written

```rust
pub mod file_options { /* ... */ }
```

### Modules

## Module `arrow_writer`

Options related to how Arrow files should be written

```rust
pub mod arrow_writer { /* ... */ }
```

### Types

#### Struct `ArrowWriterOptions`

```rust
pub struct ArrowWriterOptions {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **MaybeSendSync**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArrowWriterOptions { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

## Module `avro_writer`

Options related to how avro files should be written

```rust
pub mod avro_writer { /* ... */ }
```

### Types

#### Struct `AvroWriterOptions`

```rust
pub struct AvroWriterOptions {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AvroWriterOptions { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
## Module `csv_writer`

Options related to how csv files should be written

```rust
pub mod csv_writer { /* ... */ }
```

### Types

#### Struct `CsvWriterOptions`

Options for writing CSV files

```rust
pub struct CsvWriterOptions {
    pub writer_options: arrow::csv::WriterBuilder,
    pub compression: crate::parsers::CompressionTypeVariant,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `writer_options` | `arrow::csv::WriterBuilder` | Struct from the arrow crate which contains all csv writing related settings |
| `compression` | `crate::parsers::CompressionTypeVariant` | Compression to apply after ArrowWriter serializes RecordBatches.<br>This compression is applied by DataFusion not the ArrowWriter itself. |

##### Implementations

###### Methods

- ```rust
  pub fn new(writer_options: WriterBuilder, compression: CompressionTypeVariant) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **Allocation**
- **MaybeSendSync**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &CsvOptions) -> Result<Self> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CsvWriterOptions { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
## Module `file_type`

File type abstraction

```rust
pub mod file_type { /* ... */ }
```

### Traits

#### Trait `GetExt`

Define each `FileType`/`FileCompressionType`'s extension

```rust
pub trait GetExt {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_ext`: File extension getter

#### Trait `FileType`

Defines the functionality needed for logical planning for
a type of file which will be read or written to storage.

```rust
pub trait FileType: GetExt + Display + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_any`: Returns the table source as [`Any`] so that it can be

### Constants and Statics

#### Constant `DEFAULT_ARROW_EXTENSION`

The default file extension of arrow files

```rust
pub const DEFAULT_ARROW_EXTENSION: &str = ".arrow";
```

#### Constant `DEFAULT_AVRO_EXTENSION`

The default file extension of avro files

```rust
pub const DEFAULT_AVRO_EXTENSION: &str = ".avro";
```

#### Constant `DEFAULT_CSV_EXTENSION`

The default file extension of csv files

```rust
pub const DEFAULT_CSV_EXTENSION: &str = ".csv";
```

#### Constant `DEFAULT_JSON_EXTENSION`

The default file extension of json files

```rust
pub const DEFAULT_JSON_EXTENSION: &str = ".json";
```

#### Constant `DEFAULT_PARQUET_EXTENSION`

The default file extension of parquet files

```rust
pub const DEFAULT_PARQUET_EXTENSION: &str = ".parquet";
```

## Module `json_writer`

Options related to how json files should be written

```rust
pub mod json_writer { /* ... */ }
```

### Types

#### Struct `JsonWriterOptions`

Options for writing JSON files

```rust
pub struct JsonWriterOptions {
    pub compression: crate::parsers::CompressionTypeVariant,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `compression` | `crate::parsers::CompressionTypeVariant` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(compression: CompressionTypeVariant) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **MaybeSendSync**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: &JsonOptions) -> Result<Self> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> JsonWriterOptions { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Allocation**
## Module `parquet_writer`

**Attributes:**

- `#[cfg(feature = "parquet")]`

Options related to how parquet files should be written

```rust
pub mod parquet_writer { /* ... */ }
```

### Types

#### Struct `ParquetWriterOptions`

Options for writing parquet files

```rust
pub struct ParquetWriterOptions {
    pub writer_options: parquet::file::properties::WriterProperties,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `writer_options` | `parquet::file::properties::WriterProperties` | parquet-rs writer properties |

##### Implementations

###### Methods

- ```rust
  pub fn new(writer_options: WriterProperties) -> Self { /* ... */ }
  ```

- ```rust
  pub fn writer_options(self: &Self) -> &WriterProperties { /* ... */ }
  ```

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParquetWriterOptions { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(parquet_table_options: &TableParquetOptions) -> Result<Self> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Allocation**
- **ErasedDestructor**
- **UnwindSafe**
### Functions

#### Function `parse_compression_string`

Parses datafusion.execution.parquet.compression String to a parquet::basic::Compression

```rust
pub fn parse_compression_string(str_setting: &str) -> crate::Result<parquet::basic::Compression> { /* ... */ }
```

## Module `format`

```rust
pub mod format { /* ... */ }
```

### Constants and Statics

#### Constant `DEFAULT_FORMAT_OPTIONS`

The default [`FormatOptions`] to use within DataFusion

```rust
pub const DEFAULT_FORMAT_OPTIONS: arrow::util::display::FormatOptions<''static> = _;
```

#### Constant `DEFAULT_CAST_OPTIONS`

The default [`CastOptions`] to use within DataFusion

```rust
pub const DEFAULT_CAST_OPTIONS: arrow::compute::CastOptions<''static> = _;
```

#### Constant `DEFAULT_CLI_FORMAT_OPTIONS`

```rust
pub const DEFAULT_CLI_FORMAT_OPTIONS: arrow::util::display::FormatOptions<''static> = _;
```

## Module `hash_utils`

Functionality used both on logical and physical plans

```rust
pub mod hash_utils { /* ... */ }
```

### Traits

#### Trait `HashValue`

```rust
pub trait HashValue {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `hash_one`

##### Implementations

This trait is implemented for the following types:

- `&T` with <T: HashValue + ?Sized>
- `i8`
- `i16`
- `i32`
- `i64`
- `i128`
- `i256`
- `u8`
- `u16`
- `u32`
- `u64`
- `bool`
- `str`
- `[u8]`
- `arrow::array::types::IntervalDayTime`
- `arrow::array::types::IntervalMonthDayNano`
- `half::f16`
- `f32`
- `f64`

### Functions

#### Function `combine_hashes`

**Attributes:**

- `#[inline]`

```rust
pub fn combine_hashes(l: u64, r: u64) -> u64 { /* ... */ }
```

#### Function `create_hashes`

**Attributes:**

- `#[cfg(not(feature = "force_hash_collisions"))]`

Creates hash values for every row, based on the values in the
columns.

The number of rows to hash is determined by `hashes_buffer.len()`.
`hashes_buffer` should be pre-sized appropriately

```rust
pub fn create_hashes<''a>(arrays: &[ArrayRef], random_state: &ahash::RandomState, hashes_buffer: &''a mut Vec<u64>) -> crate::error::Result<&''a mut Vec<u64>> { /* ... */ }
```

## Module `instant`

WASM-compatible `Instant` wrapper.

```rust
pub mod instant { /* ... */ }
```

### Types

#### Type Alias `Instant`

**Attributes:**

- `#[allow(clippy::disallowed_types)]`
- `#[cfg(not(target_family = "wasm"))]`

DataFusion wrapper around [`std::time::Instant`]. This is only a type alias.

```rust
pub type Instant = std::time::Instant;
```

## Module `parsers`

Interval parsing logic

```rust
pub mod parsers { /* ... */ }
```

### Types

#### Enum `CompressionTypeVariant`

Readable file compression type

```rust
pub enum CompressionTypeVariant {
    GZIP,
    BZIP2,
    XZ,
    ZSTD,
    UNCOMPRESSED,
}
```

##### Variants

###### `GZIP`

Gzip-ed file

###### `BZIP2`

Bzip2-ed file

###### `XZ`

Xz-ed file (liblzma)

###### `ZSTD`

Zstd-ed file,

###### `UNCOMPRESSED`

Uncompressed file

##### Implementations

###### Methods

- ```rust
  pub const fn is_compressed(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, ParserError> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CompressionTypeVariant) -> bool { /* ... */ }
    ```

- **Unpin**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CompressionTypeVariant { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **ConfigField**
  - ```rust
    fn visit<V: Visit>(self: &Self, v: &mut V, key: &str, description: &''static str) { /* ... */ }
    ```

  - ```rust
    fn set(self: &mut Self, _: &str, value: &str) -> Result<()> { /* ... */ }
    ```

- **Copy**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **StructuralPartialEq**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

## Module `rounding`

Floating point rounding mode utility library
TODO: Remove this custom implementation and the "libc" dependency when
      floating-point rounding mode manipulation functions become available
      in Rust.

```rust
pub mod rounding { /* ... */ }
```

### Traits

#### Trait `FloatBits`

A trait to manipulate floating-point types with bitwise operations.
Provides functions to convert a floating-point value to/from its bitwise
representation as well as utility methods to handle special values.

```rust
pub trait FloatBits {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Item`: The integer type used for bitwise operations.

###### Associated Constants

- `TINY_BITS`: The smallest positive floating-point value representable by this type.
- `NEG_TINY_BITS`: The smallest (in magnitude) negative floating-point value representable by this type.
- `CLEAR_SIGN_MASK`: A mask to clear the sign bit of the floating-point value's bitwise representation.
- `ONE`: The integer value 1, used in bitwise operations.
- `ZERO`: The integer value 0, used in bitwise operations.

###### Required Methods

- `to_bits`: Converts the floating-point value to its bitwise representation.
- `from_bits`: Converts the bitwise representation to the corresponding floating-point value.
- `float_is_nan`: Returns true if the floating-point value is NaN (not a number).
- `infinity`: Returns the positive infinity value for this floating-point type.
- `neg_infinity`: Returns the negative infinity value for this floating-point type.

##### Implementations

This trait is implemented for the following types:

- `f32`
- `f64`

### Functions

#### Function `next_up`

Returns the next representable floating-point value greater than the input value.

This function takes a floating-point value that implements the FloatBits trait,
calculates the next representable value greater than the input, and returns it.

If the input value is NaN or positive infinity, the function returns the input value.

# Examples

```
use datafusion_common::rounding::next_up;

let f: f32 = 1.0;
let next_f = next_up(f);
assert_eq!(next_f, 1.0000001);
```

```rust
pub fn next_up<F: FloatBits + Copy>(float: F) -> F { /* ... */ }
```

#### Function `next_down`

Returns the next representable floating-point value smaller than the input value.

This function takes a floating-point value that implements the FloatBits trait,
calculates the next representable value smaller than the input, and returns it.

If the input value is NaN or negative infinity, the function returns the input value.

# Examples

```
use datafusion_common::rounding::next_down;

let f: f32 = 1.0;
let next_f = next_down(f);
assert_eq!(next_f, 0.99999994);
```

```rust
pub fn next_down<F: FloatBits + Copy>(float: F) -> F { /* ... */ }
```

#### Function `alter_fp_rounding_mode`

```rust
pub fn alter_fp_rounding_mode<const UPPER: bool, F>(lhs: &crate::ScalarValue, rhs: &crate::ScalarValue, operation: F) -> crate::Result<crate::ScalarValue>
where
    F: FnOnce(&crate::ScalarValue, &crate::ScalarValue) -> crate::Result<crate::ScalarValue> { /* ... */ }
```

## Module `scalar`

[`ScalarValue`]: stores single  values

```rust
pub mod scalar { /* ... */ }
```

### Types

#### Enum `ScalarValue`

A dynamically typed, nullable single value.

While an arrow  [`Array`]) stores one or more values of the same type, in a
single column, a `ScalarValue` stores a single value of a single type, the
equivalent of 1 row and one column.

```text
 ┌────────┐
 │ value1 │
 │ value2 │                  ┌────────┐
 │ value3 │                  │ value2 │
 │  ...   │                  └────────┘
 │ valueN │
 └────────┘

   Array                     ScalarValue

stores multiple,             stores a single,
possibly null, values of     possible null, value
the same type
```

# Performance

In general, performance will be better using arrow [`Array`]s rather than
[`ScalarValue`], as it is far more efficient to process multiple values at
once (vectorized processing).

# Example
```
# use datafusion_common::ScalarValue;
// Create single scalar value for an Int32 value
let s1 = ScalarValue::Int32(Some(10));

// You can also create values using the From impl:
let s2 = ScalarValue::from(10i32);
assert_eq!(s1, s2);
```

# Null Handling

`ScalarValue` represents null values in the same way as Arrow. Nulls are
"typed" in the sense that a null value in an [`Int32Array`] is different
from a null value in a [`Float64Array`], and is different from the values in
a [`NullArray`].

```
# fn main() -> datafusion_common::Result<()> {
# use std::collections::hash_set::Difference;
# use datafusion_common::ScalarValue;
# use arrow::datatypes::DataType;
// You can create a 'null' Int32 value directly:
let s1 = ScalarValue::Int32(None);

// You can also create a null value for a given datatype:
let s2 = ScalarValue::try_from(&DataType::Int32)?;
assert_eq!(s1, s2);

// Note that this is DIFFERENT than a `ScalarValue::Null`
let s3 = ScalarValue::Null;
assert_ne!(s1, s3);
# Ok(())
# }
```

# Nested Types

`List` / `LargeList` / `FixedSizeList` / `Struct` / `Map` are represented as a
single element array of the corresponding type.

## Example: Creating [`ScalarValue::Struct`] using [`ScalarStructBuilder`]
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::{ScalarValue, scalar::ScalarStructBuilder};
// Build a struct like: {a: 1, b: "foo"}
let field_a = Field::new("a", DataType::Int32, false);
let field_b = Field::new("b", DataType::Utf8, false);

let s1 = ScalarStructBuilder::new()
   .with_scalar(field_a, ScalarValue::from(1i32))
   .with_scalar(field_b, ScalarValue::from("foo"))
   .build();
```

## Example: Creating a null [`ScalarValue::Struct`] using [`ScalarStructBuilder`]
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field};
# use datafusion_common::{ScalarValue, scalar::ScalarStructBuilder};
// Build a struct representing a NULL value
let fields = vec![
    Field::new("a", DataType::Int32, false),
    Field::new("b", DataType::Utf8, false),
];

let s1 = ScalarStructBuilder::new_null(fields);
```

## Example: Creating [`ScalarValue::Struct`] directly
```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, Fields};
# use arrow::array::{ArrayRef, Int32Array, StructArray, StringArray};
# use datafusion_common::ScalarValue;
// Build a struct like: {a: 1, b: "foo"}
// Field description
let fields = Fields::from(vec![
  Field::new("a", DataType::Int32, false),
  Field::new("b", DataType::Utf8, false),
]);
// one row arrays for each field
let arrays: Vec<ArrayRef> = vec![
  Arc::new(Int32Array::from(vec![1])),
  Arc::new(StringArray::from(vec!["foo"])),
];
// no nulls for this array
let nulls = None;
let arr = StructArray::new(fields, arrays, nulls);

// Create a ScalarValue::Struct directly
let s1 = ScalarValue::Struct(Arc::new(arr));
```


# Further Reading
See [datatypes](https://arrow.apache.org/docs/python/api/datatypes.html) for
details on datatypes and the [format](https://github.com/apache/arrow/blob/master/format/Schema.fbs#L354-L375)
for the definitive reference.

```rust
pub enum ScalarValue {
    Null,
    Boolean(Option<bool>),
    Float16(Option<half::f16>),
    Float32(Option<f32>),
    Float64(Option<f64>),
    Decimal128(Option<i128>, u8, i8),
    Decimal256(Option<arrow::datatypes::i256>, u8, i8),
    Int8(Option<i8>),
    Int16(Option<i16>),
    Int32(Option<i32>),
    Int64(Option<i64>),
    UInt8(Option<u8>),
    UInt16(Option<u16>),
    UInt32(Option<u32>),
    UInt64(Option<u64>),
    Utf8(Option<String>),
    Utf8View(Option<String>),
    LargeUtf8(Option<String>),
    Binary(Option<Vec<u8>>),
    BinaryView(Option<Vec<u8>>),
    FixedSizeBinary(i32, Option<Vec<u8>>),
    LargeBinary(Option<Vec<u8>>),
    FixedSizeList(std::sync::Arc<FixedSizeListArray>),
    List(std::sync::Arc<ListArray>),
    LargeList(std::sync::Arc<LargeListArray>),
    Struct(std::sync::Arc<StructArray>),
    Map(std::sync::Arc<MapArray>),
    Date32(Option<i32>),
    Date64(Option<i64>),
    Time32Second(Option<i32>),
    Time32Millisecond(Option<i32>),
    Time64Microsecond(Option<i64>),
    Time64Nanosecond(Option<i64>),
    TimestampSecond(Option<i64>, Option<std::sync::Arc<str>>),
    TimestampMillisecond(Option<i64>, Option<std::sync::Arc<str>>),
    TimestampMicrosecond(Option<i64>, Option<std::sync::Arc<str>>),
    TimestampNanosecond(Option<i64>, Option<std::sync::Arc<str>>),
    IntervalYearMonth(Option<i32>),
    IntervalDayTime(Option<arrow::array::types::IntervalDayTime>),
    IntervalMonthDayNano(Option<arrow::array::types::IntervalMonthDayNano>),
    DurationSecond(Option<i64>),
    DurationMillisecond(Option<i64>),
    DurationMicrosecond(Option<i64>),
    DurationNanosecond(Option<i64>),
    Union(Option<(i8, Box<ScalarValue>)>, arrow::datatypes::UnionFields, arrow::datatypes::UnionMode),
    Dictionary(Box<arrow::datatypes::DataType>, Box<ScalarValue>),
}
```

##### Variants

###### `Null`

represents `DataType::Null` (castable to/from any other type)

###### `Boolean`

true or false value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<bool>` |  |

###### `Float16`

16bit float

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<half::f16>` |  |

###### `Float32`

32bit float

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<f32>` |  |

###### `Float64`

64bit float

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<f64>` |  |

###### `Decimal128`

128bit decimal, using the i128 to represent the decimal, precision scale

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i128>` |  |
| 1 | `u8` |  |
| 2 | `i8` |  |

###### `Decimal256`

256bit decimal, using the i256 to represent the decimal, precision scale

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<arrow::datatypes::i256>` |  |
| 1 | `u8` |  |
| 2 | `i8` |  |

###### `Int8`

signed 8bit int

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i8>` |  |

###### `Int16`

signed 16bit int

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i16>` |  |

###### `Int32`

signed 32bit int

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i32>` |  |

###### `Int64`

signed 64bit int

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |

###### `UInt8`

unsigned 8bit int

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<u8>` |  |

###### `UInt16`

unsigned 16bit int

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<u16>` |  |

###### `UInt32`

unsigned 32bit int

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<u32>` |  |

###### `UInt64`

unsigned 64bit int

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<u64>` |  |

###### `Utf8`

utf-8 encoded string.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<String>` |  |

###### `Utf8View`

utf-8 encoded string but from view types.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<String>` |  |

###### `LargeUtf8`

utf-8 encoded string representing a LargeString's arrow type.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<String>` |  |

###### `Binary`

binary

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<Vec<u8>>` |  |

###### `BinaryView`

binary but from view types.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<Vec<u8>>` |  |

###### `FixedSizeBinary`

fixed size binary

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `i32` |  |
| 1 | `Option<Vec<u8>>` |  |

###### `LargeBinary`

large binary

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<Vec<u8>>` |  |

###### `FixedSizeList`

Fixed size list scalar.

The array must be a FixedSizeListArray with length 1.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<FixedSizeListArray>` |  |

###### `List`

Represents a single element of a [`ListArray`] as an [`ArrayRef`]

The array must be a ListArray with length 1.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<ListArray>` |  |

###### `LargeList`

The array must be a LargeListArray with length 1.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<LargeListArray>` |  |

###### `Struct`

Represents a single element [`StructArray`] as an [`ArrayRef`]. See
[`ScalarValue`] for examples of how to create instances of this type.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<StructArray>` |  |

###### `Map`

Represents a single element [`MapArray`] as an [`ArrayRef`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<MapArray>` |  |

###### `Date32`

Date stored as a signed 32bit int days since UNIX epoch 1970-01-01

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i32>` |  |

###### `Date64`

Date stored as a signed 64bit int milliseconds since UNIX epoch 1970-01-01

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |

###### `Time32Second`

Time stored as a signed 32bit int as seconds since midnight

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i32>` |  |

###### `Time32Millisecond`

Time stored as a signed 32bit int as milliseconds since midnight

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i32>` |  |

###### `Time64Microsecond`

Time stored as a signed 64bit int as microseconds since midnight

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |

###### `Time64Nanosecond`

Time stored as a signed 64bit int as nanoseconds since midnight

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |

###### `TimestampSecond`

Timestamp Second

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |
| 1 | `Option<std::sync::Arc<str>>` |  |

###### `TimestampMillisecond`

Timestamp Milliseconds

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |
| 1 | `Option<std::sync::Arc<str>>` |  |

###### `TimestampMicrosecond`

Timestamp Microseconds

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |
| 1 | `Option<std::sync::Arc<str>>` |  |

###### `TimestampNanosecond`

Timestamp Nanoseconds

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |
| 1 | `Option<std::sync::Arc<str>>` |  |

###### `IntervalYearMonth`

Number of elapsed whole months

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i32>` |  |

###### `IntervalDayTime`

Number of elapsed days and milliseconds (no leap seconds)
stored as 2 contiguous 32-bit signed integers

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<arrow::array::types::IntervalDayTime>` |  |

###### `IntervalMonthDayNano`

A triple of the number of elapsed months, days, and nanoseconds.
Months and days are encoded as 32-bit signed integers.
Nanoseconds is encoded as a 64-bit signed integer (no leap seconds).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<arrow::array::types::IntervalMonthDayNano>` |  |

###### `DurationSecond`

Duration in seconds

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |

###### `DurationMillisecond`

Duration in milliseconds

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |

###### `DurationMicrosecond`

Duration in microseconds

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |

###### `DurationNanosecond`

Duration in nanoseconds

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<i64>` |  |

###### `Union`

A nested datatype that can represent slots of differing types. Components:
`.0`: a tuple of union `type_id` and the single value held by this Scalar
`.1`: the list of fields, zero-to-one of which will by set in `.0`
`.2`: the physical storage of the source/destination UnionArray from which this Scalar came

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<(i8, Box<ScalarValue>)>` |  |
| 1 | `arrow::datatypes::UnionFields` |  |
| 2 | `arrow::datatypes::UnionMode` |  |

###### `Dictionary`

Dictionary type: index type and value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<arrow::datatypes::DataType>` |  |
| 1 | `Box<ScalarValue>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new_primitive<T: ArrowPrimitiveType>(a: Option<<T as >::Native>, d: &DataType) -> Result<Self> { /* ... */ }
  ```
  Create a [`Result<ScalarValue>`] with the provided value and datatype

- ```rust
  pub fn try_new_decimal128(value: i128, precision: u8, scale: i8) -> Result<Self> { /* ... */ }
  ```
  Create a decimal Scalar from value/precision and scale.

- ```rust
  pub fn try_new_null(data_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Create a Null instance of ScalarValue for this datatype

- ```rust
  pub fn new_utf8</* synthetic */ impl Into<String>: Into<String>>(val: impl Into<String>) -> Self { /* ... */ }
  ```
  Returns a [`ScalarValue::Utf8`] representing `val`

- ```rust
  pub fn new_utf8view</* synthetic */ impl Into<String>: Into<String>>(val: impl Into<String>) -> Self { /* ... */ }
  ```
  Returns a [`ScalarValue::Utf8View`] representing `val`

- ```rust
  pub fn new_interval_ym(years: i32, months: i32) -> Self { /* ... */ }
  ```
  Returns a [`ScalarValue::IntervalYearMonth`] representing

- ```rust
  pub fn new_interval_dt(days: i32, millis: i32) -> Self { /* ... */ }
  ```
  Returns a [`ScalarValue::IntervalDayTime`] representing

- ```rust
  pub fn new_interval_mdn(months: i32, days: i32, nanos: i64) -> Self { /* ... */ }
  ```
  Returns a [`ScalarValue::IntervalMonthDayNano`] representing

- ```rust
  pub fn new_timestamp<T: ArrowTimestampType>(value: Option<i64>, tz_opt: Option<Arc<str>>) -> Self { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing

- ```rust
  pub fn new_pi(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing PI

- ```rust
  pub fn new_pi_upper(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing PI's upper bound

- ```rust
  pub fn new_negative_pi_lower(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing -PI's lower bound

- ```rust
  pub fn new_frac_pi_2_upper(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing FRAC_PI_2's upper bound

- ```rust
  pub fn new_neg_frac_pi_2_lower(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```

- ```rust
  pub fn new_negative_pi(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing -PI

- ```rust
  pub fn new_frac_pi_2(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing PI/2

- ```rust
  pub fn new_neg_frac_pi_2(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing -PI/2

- ```rust
  pub fn new_infinity(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing infinity

- ```rust
  pub fn new_neg_infinity(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns a [`ScalarValue`] representing negative infinity

- ```rust
  pub fn new_zero(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Create a zero value in the given type.

- ```rust
  pub fn new_one(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Create an one value in the given type.

- ```rust
  pub fn new_negative_one(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Create a negative one value in the given type.

- ```rust
  pub fn new_ten(datatype: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```

- ```rust
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```
  return the [`DataType`] of this `ScalarValue`

- ```rust
  pub fn arithmetic_negate(self: &Self) -> Result<Self> { /* ... */ }
  ```
  Calculate arithmetic negation for a scalar value

- ```rust
  pub fn add<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<ScalarValue> { /* ... */ }
  ```
  Wrapping addition of `ScalarValue`

- ```rust
  pub fn add_checked<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<ScalarValue> { /* ... */ }
  ```
  Checked addition of `ScalarValue`

- ```rust
  pub fn sub<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<ScalarValue> { /* ... */ }
  ```
  Wrapping subtraction of `ScalarValue`

- ```rust
  pub fn sub_checked<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<ScalarValue> { /* ... */ }
  ```
  Checked subtraction of `ScalarValue`

- ```rust
  pub fn mul<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<ScalarValue> { /* ... */ }
  ```
  Wrapping multiplication of `ScalarValue`

- ```rust
  pub fn mul_checked<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<ScalarValue> { /* ... */ }
  ```
  Checked multiplication of `ScalarValue`

- ```rust
  pub fn div<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<ScalarValue> { /* ... */ }
  ```
  Performs `lhs / rhs`

- ```rust
  pub fn rem<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<ScalarValue> { /* ... */ }
  ```
  Performs `lhs % rhs`

- ```rust
  pub fn is_unsigned(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn is_null(self: &Self) -> bool { /* ... */ }
  ```
  whether this value is null or not.

- ```rust
  pub fn distance(self: &Self, other: &ScalarValue) -> Option<usize> { /* ... */ }
  ```
  Absolute distance between two numeric values (of the same type). This method will return

- ```rust
  pub fn to_array(self: &Self) -> Result<ArrayRef> { /* ... */ }
  ```
  Converts a scalar value into an 1-row array.

- ```rust
  pub fn to_scalar(self: &Self) -> Result<Scalar<ArrayRef>> { /* ... */ }
  ```
  Converts a scalar into an arrow [`Scalar`] (which implements

- ```rust
  pub fn iter_to_array</* synthetic */ impl IntoIterator<Item = ScalarValue>: IntoIterator<Item = ScalarValue>>(scalars: impl IntoIterator<Item = ScalarValue>) -> Result<ArrayRef> { /* ... */ }
  ```
  Converts an iterator of references [`ScalarValue`] into an [`ArrayRef`]

- ```rust
  pub fn new_list(values: &[ScalarValue], data_type: &DataType, nullable: bool) -> Arc<ListArray> { /* ... */ }
  ```
  Converts `Vec<ScalarValue>` where each element has type corresponding to

- ```rust
  pub fn new_list_nullable(values: &[ScalarValue], data_type: &DataType) -> Arc<ListArray> { /* ... */ }
  ```
  Same as [`ScalarValue::new_list`] but with nullable set to true.

- ```rust
  pub fn new_null_list(data_type: DataType, nullable: bool, null_len: usize) -> Self { /* ... */ }
  ```
  Create ListArray with Null with specific data type

- ```rust
  pub fn new_list_from_iter</* synthetic */ impl IntoIterator<Item = ScalarValue> + ExactSizeIterator: IntoIterator<Item = ScalarValue> + ExactSizeIterator>(values: impl IntoIterator<Item = ScalarValue> + ExactSizeIterator, data_type: &DataType, nullable: bool) -> Arc<ListArray> { /* ... */ }
  ```
  Converts `IntoIterator<Item = ScalarValue>` where each element has type corresponding to

- ```rust
  pub fn new_large_list(values: &[ScalarValue], data_type: &DataType) -> Arc<LargeListArray> { /* ... */ }
  ```
  Converts `Vec<ScalarValue>` where each element has type corresponding to

- ```rust
  pub fn to_array_of_size(self: &Self, size: usize) -> Result<ArrayRef> { /* ... */ }
  ```
  Converts a scalar value into an array of `size` rows.

- ```rust
  pub fn convert_array_to_scalar_vec(array: &dyn Array) -> Result<Vec<Vec<Self>>> { /* ... */ }
  ```
  Retrieve ScalarValue for each row in `array`

- ```rust
  pub fn raw_data(self: &Self) -> Result<ArrayRef> { /* ... */ }
  ```

- ```rust
  pub fn try_from_array(array: &dyn Array, index: usize) -> Result<Self> { /* ... */ }
  ```
  Converts a value in `array` at `index` into a ScalarValue

- ```rust
  pub fn try_from_string(value: String, target_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Try to parse `value` into a ScalarValue of type `target_type`

- ```rust
  pub fn try_as_str(self: &Self) -> Option<Option<&str>> { /* ... */ }
  ```
  Returns the Some(`&str`) representation of `ScalarValue` of logical string type

- ```rust
  pub fn cast_to(self: &Self, target_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Try to cast this value to a ScalarValue of type `data_type`

- ```rust
  pub fn cast_to_with_options(self: &Self, target_type: &DataType, cast_options: &CastOptions<''static>) -> Result<Self> { /* ... */ }
  ```
  Try to cast this value to a ScalarValue of type `data_type` with [`CastOptions`]

- ```rust
  pub fn eq_array(self: &Self, array: &ArrayRef, index: usize) -> Result<bool> { /* ... */ }
  ```
  Compares a single row of array @ index for equality with self,

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Estimate size if bytes including `Self`. For values with internal containers such as `String`

- ```rust
  pub fn size_of_vec(vec: &Vec<Self>) -> usize { /* ... */ }
  ```
  Estimates [size](Self::size) of [`Vec`] in bytes.

- ```rust
  pub fn size_of_vec_deque(vec_deque: &VecDeque<Self>) -> usize { /* ... */ }
  ```
  Estimates [size](Self::size) of [`VecDeque`] in bytes.

- ```rust
  pub fn size_of_hashset<S>(set: &HashSet<Self, S>) -> usize { /* ... */ }
  ```
  Estimates [size](Self::size) of [`HashSet`] in bytes.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ScalarValue { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ScalarValue) -> Result<Self> { /* ... */ }
    ```

  - ```rust
    fn try_from(datatype: DataType) -> Result<Self> { /* ... */ }
    ```
    Create a Null instance of ScalarValue for this datatype

  - ```rust
    fn try_from(data_type: &DataType) -> Result<Self> { /* ... */ }
    ```
    Create a Null instance of ScalarValue for this datatype

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **Eq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: f64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<f64>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: f32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<f32>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i8) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<i8>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<i16>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<i32>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: i64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<i64>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: bool) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<bool>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u8) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<u8>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u16) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<u16>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u32) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<u32>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: u64) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<u64>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: &str) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Option<&str>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Vec<(&str, ScalarValue)>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: String) -> Self { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `ScalarType`

Trait used to map a NativeType to a ScalarValue

```rust
pub trait ScalarType<T: ArrowNativeType> {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `scalar`: returns a scalar from an optional T

##### Implementations

This trait is implemented for the following types:

- `arrow::datatypes::Float32Type`
- `arrow::datatypes::TimestampSecondType`
- `arrow::datatypes::TimestampMillisecondType`
- `arrow::datatypes::TimestampMicrosecondType`
- `arrow::datatypes::TimestampNanosecondType`
- `arrow::datatypes::Date32Type`

### Functions

#### Function `get_dict_value`

**Attributes:**

- `#[inline]`

Return a reference to the values array and the index into it for a
dictionary array

# Errors

Errors if the array cannot be downcasted to DictionaryArray

```rust
pub fn get_dict_value<K: ArrowDictionaryKeyType>(array: &dyn Array, index: usize) -> crate::error::Result<(&ArrayRef, Option<usize>)> { /* ... */ }
```

### Re-exports

#### Re-export `ScalarStructBuilder`

```rust
pub use struct_builder::ScalarStructBuilder;
```

## Module `spans`

```rust
pub mod spans { /* ... */ }
```

### Types

#### Struct `Location`

Represents a location, determined by a line and a column number, in the
original SQL query.

```rust
pub struct Location {
    pub line: u64,
    pub column: u64,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `line` | `u64` | Line number, starting from 1.<br><br>Note: Line 0 is used for empty spans |
| `column` | `u64` | Line column, starting from 1.<br><br>Note: Column 0 is used for empty spans |

##### Implementations

###### Trait Implementations

- **Send**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: sqlparser::tokenizer::Location) -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Location { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **UnwindSafe**
- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Location) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Location) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Allocation**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Location) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
#### Struct `Span`

Represents an interval of characters in the original SQL query.

```rust
pub struct Span {
    pub start: Location,
    pub end: Location,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `Location` |  |
| `end` | `Location` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(start: Location, end: Location) -> Self { /* ... */ }
  ```
  Creates a new [`Span`] from a start and an end [`Location`].

- ```rust
  pub fn try_from_sqlparser_span(span: sqlparser::tokenizer::Span) -> Option<Span> { /* ... */ }
  ```
  Convert a [`Span`](sqlparser::tokenizer::Span) from the parser, into a

- ```rust
  pub fn union(self: &Self, other: &Span) -> Span { /* ... */ }
  ```
  Returns the smallest Span that contains both `self` and `other`

- ```rust
  pub fn union_opt(self: &Self, other: &Option<Span>) -> Span { /* ... */ }
  ```
  Same as [Span::union] for `Option<Span>`.

- ```rust
  pub fn union_iter<I: IntoIterator<Item = Span>>(iter: I) -> Option<Span> { /* ... */ }
  ```
  Return the [Span::union] of all spans in the iterator.

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Span) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Span) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Span { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **MaybeSendSync**
- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Span) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
#### Struct `Spans`

A collection of [`Span`], meant to be used as a field of entities whose
location in the original SQL query is desired to be tracked. Sometimes an
entity can have multiple spans. e.g. if you want to track the position of
the column a that comes from SELECT 1 AS a UNION ALL SELECT 2 AS a you'll
need two spans.

```rust
pub struct Spans(pub Vec<Span>);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Span>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new empty [`Spans`] with no [`Span`].

- ```rust
  pub fn first(self: &Self) -> Option<Span> { /* ... */ }
  ```
  Returns the first [`Span`], if any. This is useful when you know that

- ```rust
  pub fn get_spans(self: &Self) -> &[Span] { /* ... */ }
  ```
  Returns a slice of the [`Span`]s.

- ```rust
  pub fn add_span(self: &mut Self, span: Span) { /* ... */ }
  ```
  Adds a [`Span`] to the collection.

- ```rust
  pub fn iter(self: &Self) -> impl Iterator<Item = &Span> { /* ... */ }
  ```
  Iterates over the [`Span`]s.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, _other: &Self) -> Ordering { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, _other: &Self) -> bool { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **IntoEither**
- **Eq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Spans { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, _state: &mut H) { /* ... */ }
    ```

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

## Module `stats`

This module provides data structures to represent statistics

```rust
pub mod stats { /* ... */ }
```

### Types

#### Enum `Precision`

Represents a value with a degree of certainty. `Precision` is used to
propagate information the precision of statistical values.

```rust
pub enum Precision<T: Debug + Clone + PartialEq + Eq + PartialOrd> {
    Exact(T),
    Inexact(T),
    Absent,
}
```

##### Variants

###### `Exact`

The exact value is known

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Inexact`

The value is not known exactly, but is likely close to this value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Absent`

Nothing is known about the value

##### Implementations

###### Methods

- ```rust
  pub fn get_value(self: &Self) -> Option<&T> { /* ... */ }
  ```
  If we have some value (exact or inexact), it returns that value.

- ```rust
  pub fn map<U, F>(self: Self, f: F) -> Precision<U>
where
    F: Fn(T) -> U,
    U: Debug + Clone + PartialEq + Eq + PartialOrd { /* ... */ }
  ```
  Transform the value in this [`Precision`] object, if one exists, using

- ```rust
  pub fn is_exact(self: &Self) -> Option<bool> { /* ... */ }
  ```
  Returns `Some(true)` if we have an exact value, `Some(false)` if we

- ```rust
  pub fn max(self: &Self, other: &Precision<T>) -> Precision<T> { /* ... */ }
  ```
  Returns the maximum of two (possibly inexact) values, conservatively

- ```rust
  pub fn min(self: &Self, other: &Precision<T>) -> Precision<T> { /* ... */ }
  ```
  Returns the minimum of two (possibly inexact) values, conservatively

- ```rust
  pub fn to_inexact(self: Self) -> Self { /* ... */ }
  ```
  Demotes the precision state from exact to inexact (if present).

- ```rust
  pub fn add(self: &Self, other: &Precision<usize>) -> Precision<usize> { /* ... */ }
  ```
  Calculates the sum of two (possibly inexact) [`usize`] values,

- ```rust
  pub fn sub(self: &Self, other: &Precision<usize>) -> Precision<usize> { /* ... */ }
  ```
  Calculates the difference of two (possibly inexact) [`usize`] values,

- ```rust
  pub fn multiply(self: &Self, other: &Precision<usize>) -> Precision<usize> { /* ... */ }
  ```
  Calculates the multiplication of two (possibly inexact) [`usize`] values,

- ```rust
  pub fn with_estimated_selectivity(self: Self, selectivity: f64) -> Self { /* ... */ }
  ```
  Return the estimate of applying a filter with estimated selectivity

- ```rust
  pub fn add(self: &Self, other: &Precision<ScalarValue>) -> Precision<ScalarValue> { /* ... */ }
  ```
  Calculates the sum of two (possibly inexact) [`ScalarValue`] values,

- ```rust
  pub fn sub(self: &Self, other: &Precision<ScalarValue>) -> Precision<ScalarValue> { /* ... */ }
  ```
  Calculates the difference of two (possibly inexact) [`ScalarValue`] values,

- ```rust
  pub fn multiply(self: &Self, other: &Precision<ScalarValue>) -> Precision<ScalarValue> { /* ... */ }
  ```
  Calculates the multiplication of two (possibly inexact) [`ScalarValue`] values,

- ```rust
  pub fn cast_to(self: &Self, data_type: &DataType) -> Result<Precision<ScalarValue>> { /* ... */ }
  ```
  Casts the value to the given data type, propagating exactness information.

###### Trait Implementations

- **Sync**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: Precision<usize>) -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Allocation**
- **Default**
  - ```rust
    fn default() -> Precision<T> { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Eq**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Precision<T>) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Precision<T> { /* ... */ }
    ```

- **Copy**
- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `Statistics`

Statistics for a relation
Fields are optional and can be inexact because the sources
sometimes provide approximate estimates for performance reasons
and the transformations output are not always predictable.

```rust
pub struct Statistics {
    pub num_rows: Precision<usize>,
    pub total_byte_size: Precision<usize>,
    pub column_statistics: Vec<ColumnStatistics>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `num_rows` | `Precision<usize>` | The number of table rows. |
| `total_byte_size` | `Precision<usize>` | Total bytes of the table rows. |
| `column_statistics` | `Vec<ColumnStatistics>` | Statistics on a column level.<br><br>It must contains a [`ColumnStatistics`] for each field in the schema of<br>the table to which the [`Statistics`] refer. |

##### Implementations

###### Methods

- ```rust
  pub fn new_unknown(schema: &Schema) -> Self { /* ... */ }
  ```
  Returns a [`Statistics`] instance for the given schema by assigning

- ```rust
  pub fn unknown_column(schema: &Schema) -> Vec<ColumnStatistics> { /* ... */ }
  ```
  Returns an unbounded `ColumnStatistics` for each field in the schema.

- ```rust
  pub fn with_num_rows(self: Self, num_rows: Precision<usize>) -> Self { /* ... */ }
  ```
  Set the number of rows

- ```rust
  pub fn with_total_byte_size(self: Self, total_byte_size: Precision<usize>) -> Self { /* ... */ }
  ```
  Set the total size, in bytes

- ```rust
  pub fn add_column_statistics(self: Self, column_stats: ColumnStatistics) -> Self { /* ... */ }
  ```
  Add a column to the column statistics

- ```rust
  pub fn to_inexact(self: Self) -> Self { /* ... */ }
  ```
  If the exactness of a [`Statistics`] instance is lost, this function relaxes

- ```rust
  pub fn project(self: Self, projection: Option<&Vec<usize>>) -> Self { /* ... */ }
  ```
  Project the statistics to the given column indices.

- ```rust
  pub fn with_fetch(self: Self, schema: SchemaRef, fetch: Option<usize>, skip: usize, n_partitions: usize) -> Result<Self> { /* ... */ }
  ```
  Calculates the statistics after applying `fetch` and `skip` operations.

- ```rust
  pub fn try_merge_iter<''a, I>(items: I, schema: &Schema) -> Result<Statistics>
where
    I: IntoIterator<Item = &''a Statistics> { /* ... */ }
  ```
  Summarize zero or more statistics into a single `Statistics` instance.

- ```rust
  pub fn try_merge(self: Self, other: &Statistics) -> Result<Self> { /* ... */ }
  ```
  Merge this Statistics value with another Statistics value.

###### Trait Implementations

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Statistics) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **StructuralPartialEq**
- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Returns a new [`Statistics`] instance with all fields set to unknown

- **Send**
- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Statistics { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `ColumnStatistics`

Statistics for a column within a relation

```rust
pub struct ColumnStatistics {
    pub null_count: Precision<usize>,
    pub max_value: Precision<crate::ScalarValue>,
    pub min_value: Precision<crate::ScalarValue>,
    pub sum_value: Precision<crate::ScalarValue>,
    pub distinct_count: Precision<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `null_count` | `Precision<usize>` | Number of null values on column |
| `max_value` | `Precision<crate::ScalarValue>` | Maximum value of column |
| `min_value` | `Precision<crate::ScalarValue>` | Minimum value of column |
| `sum_value` | `Precision<crate::ScalarValue>` | Sum value of a column |
| `distinct_count` | `Precision<usize>` | Number of distinct values |

##### Implementations

###### Methods

- ```rust
  pub fn is_singleton(self: &Self) -> bool { /* ... */ }
  ```
  Column contains a single non null value (e.g constant).

- ```rust
  pub fn new_unknown() -> Self { /* ... */ }
  ```
  Returns a [`ColumnStatistics`] instance having all [`Precision::Absent`] parameters.

- ```rust
  pub fn with_null_count(self: Self, null_count: Precision<usize>) -> Self { /* ... */ }
  ```
  Set the null count

- ```rust
  pub fn with_max_value(self: Self, max_value: Precision<ScalarValue>) -> Self { /* ... */ }
  ```
  Set the max value

- ```rust
  pub fn with_min_value(self: Self, min_value: Precision<ScalarValue>) -> Self { /* ... */ }
  ```
  Set the min value

- ```rust
  pub fn with_sum_value(self: Self, sum_value: Precision<ScalarValue>) -> Self { /* ... */ }
  ```
  Set the sum value

- ```rust
  pub fn with_distinct_count(self: Self, distinct_count: Precision<usize>) -> Self { /* ... */ }
  ```
  Set the distinct count

- ```rust
  pub fn to_inexact(self: Self) -> Self { /* ... */ }
  ```
  If the exactness of a [`ColumnStatistics`] instance is lost, this

###### Trait Implementations

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **MaybeSendSync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ColumnStatistics) -> bool { /* ... */ }
    ```

- **Eq**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ColumnStatistics { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> ColumnStatistics { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `test_util`

Utility functions to make testing DataFusion based crates easier

```rust
pub mod test_util { /* ... */ }
```

### Functions

#### Function `batches_to_string`

```rust
pub fn batches_to_string(batches: &[arrow::array::RecordBatch]) -> String { /* ... */ }
```

#### Function `batches_to_sort_string`

```rust
pub fn batches_to_sort_string(batches: &[arrow::array::RecordBatch]) -> String { /* ... */ }
```

#### Function `datafusion_test_data`

Returns the datafusion test data directory, which is by default rooted at `datafusion/core/tests/data`.

The default can be overridden by the optional environment
variable `DATAFUSION_TEST_DATA`

panics when the directory can not be found.

Example:
```
let testdata = datafusion_common::test_util::datafusion_test_data();
let csvdata = format!("{}/window_1.csv", testdata);
assert!(std::path::PathBuf::from(csvdata).exists());
```

```rust
pub fn datafusion_test_data() -> String { /* ... */ }
```

#### Function `arrow_test_data`

Returns the arrow test data directory, which is by default stored
in a git submodule rooted at `testing/data`.

The default can be overridden by the optional environment
variable `ARROW_TEST_DATA`

panics when the directory can not be found.

Example:
```
let testdata = datafusion_common::test_util::arrow_test_data();
let csvdata = format!("{}/csv/aggregate_test_100.csv", testdata);
assert!(std::path::PathBuf::from(csvdata).exists());
```

```rust
pub fn arrow_test_data() -> String { /* ... */ }
```

#### Function `parquet_test_data`

**Attributes:**

- `#[cfg(feature = "parquet")]`

Returns the parquet test data directory, which is by default
stored in a git submodule rooted at
`parquet-testing/data`.

The default can be overridden by the optional environment variable
`PARQUET_TEST_DATA`

panics when the directory can not be found.

Example:
```
let testdata = datafusion_common::test_util::parquet_test_data();
let filename = format!("{}/binary.parquet", testdata);
assert!(std::path::PathBuf::from(filename).exists());
```

```rust
pub fn parquet_test_data() -> String { /* ... */ }
```

#### Function `get_data_dir`

Returns a directory path for finding test data.

udf_env: name of an environment variable

submodule_dir: fallback path (relative to CARGO_MANIFEST_DIR)

 Returns either:
The path referred to in `udf_env` if that variable is set and refers to a directory
The submodule_data directory relative to CARGO_MANIFEST_PATH

```rust
pub fn get_data_dir(udf_env: &str, submodule_data: &str) -> Result<std::path::PathBuf, Box<dyn Error>> { /* ... */ }
```

## Module `tree_node`

[`TreeNode`] for visiting and rewriting expression and plan trees

```rust
pub mod tree_node { /* ... */ }
```

### Types

#### Enum `TreeNodeRecursion`

Controls how [`TreeNode`] recursions should proceed.

```rust
pub enum TreeNodeRecursion {
    Continue,
    Jump,
    Stop,
}
```

##### Variants

###### `Continue`

Continue recursion with the next node.

###### `Jump`

In top-down traversals, skip recursing into children but continue with
the next node, which actually means pruning of the subtree.

In bottom-up traversals, bypass calling bottom-up closures till the next
leaf node.

In combined traversals, if it is the `f_down` (pre-order) phase, execution
"jumps" to the next `f_up` (post-order) phase by shortcutting its children.
If it is the `f_up` (post-order) phase, execution "jumps" to the next `f_down`
(pre-order) phase by shortcutting its parent nodes until the first parent node
having unvisited children path.

###### `Stop`

Stop recursion.

##### Implementations

###### Methods

- ```rust
  pub fn visit_children<F: FnOnce() -> Result<TreeNodeRecursion>>(self: Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
  ```
  Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`]

- ```rust
  pub fn visit_sibling<F: FnOnce() -> Result<TreeNodeRecursion>>(self: Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
  ```
  Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`]

- ```rust
  pub fn visit_parent<F: FnOnce() -> Result<TreeNodeRecursion>>(self: Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
  ```
  Continues visiting nodes with `f` depending on the current [`TreeNodeRecursion`]

###### Trait Implementations

- **Freeze**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TreeNodeRecursion) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TreeNodeRecursion { /* ... */ }
    ```

- **Copy**
- **IntoEither**
- **MaybeSendSync**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `Transformed`

Result of tree walk / transformation APIs

`Transformed` is a wrapper around the tree node data (e.g. `Expr` or
`LogicalPlan`). It is used to indicate whether the node was transformed
and how the recursion should proceed.

[`TreeNode`] API users control the transformation by returning:
- The resulting (possibly transformed) node,
- `transformed`: flag indicating whether any change was made to the node
- `tnr`: [`TreeNodeRecursion`] specifying how to proceed with the recursion.

At the end of the transformation, the return value will contain:
- The final (possibly transformed) tree,
- `transformed`: flag indicating whether any change was made to the node
- `tnr`: [`TreeNodeRecursion`] specifying how the recursion ended.

See also
* [`Transformed::update_data`] to modify the node without changing the `transformed` flag
* [`Transformed::map_data`] for fallable operation that return the same type
* [`Transformed::transform_data`] to chain fallable transformations
* [`TransformedResult`] for working with `Result<Transformed<U>>`

# Examples

Use [`Transformed::yes`] and [`Transformed::no`] to signal that a node was
rewritten and the recursion should continue:

```
# use datafusion_common::tree_node::Transformed;
# // note use i64 instead of Expr as Expr is not in datafusion-common
# fn orig_expr() -> i64 { 1 }
# fn make_new_expr(i: i64) -> i64 { 2 }
let expr = orig_expr();

// Create a new `Transformed` object signaling the node was not rewritten
let ret = Transformed::no(expr.clone());
assert!(!ret.transformed);

// Create a new `Transformed` object signaling the node was rewritten
let ret = Transformed::yes(expr);
assert!(ret.transformed)
```

Access the node within the `Transformed` object:
```
# use datafusion_common::tree_node::Transformed;
# // note use i64 instead of Expr as Expr is not in datafusion-common
# fn orig_expr() -> i64 { 1 }
# fn make_new_expr(i: i64) -> i64 { 2 }
let expr = orig_expr();

// `Transformed` object signaling the node was not rewritten
let ret = Transformed::no(expr.clone());
// Access the inner object using .data
assert_eq!(expr, ret.data);
```

Transform the node within the `Transformed` object.

```
# use datafusion_common::tree_node::Transformed;
# // note use i64 instead of Expr as Expr is not in datafusion-common
# fn orig_expr() -> i64 { 1 }
# fn make_new_expr(i: i64) -> i64 { 2 }
let expr = orig_expr();
let ret = Transformed::no(expr.clone())
  .transform_data(|expr| {
   // closure returns a result and potentially transforms the node
   // in this example, it does transform the node
   let new_expr = make_new_expr(expr);
   Ok(Transformed::yes(new_expr))
 }).unwrap();
// transformed flag is the union of the original ans closure's  transformed flag
assert!(ret.transformed);
```
# Example APIs that use `TreeNode`
- [`TreeNode`],
- [`TreeNode::rewrite`],
- [`TreeNode::transform_down`],
- [`TreeNode::transform_up`],
- [`TreeNode::transform_down_up`]

```rust
pub struct Transformed<T> {
    pub data: T,
    pub transformed: bool,
    pub tnr: TreeNodeRecursion,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `data` | `T` |  |
| `transformed` | `bool` |  |
| `tnr` | `TreeNodeRecursion` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(data: T, transformed: bool, tnr: TreeNodeRecursion) -> Self { /* ... */ }
  ```
  Create a new `Transformed` object with the given information.

- ```rust
  pub fn new_transformed(data: T, transformed: bool) -> Self { /* ... */ }
  ```
  Create a `Transformed` with `transformed` and [`TreeNodeRecursion::Continue`].

- ```rust
  pub fn yes(data: T) -> Self { /* ... */ }
  ```
  Wrapper for transformed data with [`TreeNodeRecursion::Continue`] statement.

- ```rust
  pub fn no(data: T) -> Self { /* ... */ }
  ```
  Wrapper for unchanged data with [`TreeNodeRecursion::Continue`] statement.

- ```rust
  pub fn update_data<U, F: FnOnce(T) -> U>(self: Self, f: F) -> Transformed<U> { /* ... */ }
  ```
  Applies an infallible `f` to the data of this [`Transformed`] object,

- ```rust
  pub fn map_data<U, F: FnOnce(T) -> Result<U>>(self: Self, f: F) -> Result<Transformed<U>> { /* ... */ }
  ```
  Applies a fallible `f` (returns `Result`) to the data of this

- ```rust
  pub fn transform_data<U, F: FnOnce(T) -> Result<Transformed<U>>>(self: Self, f: F) -> Result<Transformed<U>> { /* ... */ }
  ```
  Applies a fallible transforming `f` to the data of this [`Transformed`]

- ```rust
  pub fn transform_children<F: FnOnce(T) -> Result<Transformed<T>>>(self: Self, f: F) -> Result<Transformed<T>> { /* ... */ }
  ```
  Maps the [`Transformed`] object to the result of the given `f` depending on the

- ```rust
  pub fn transform_sibling<F: FnOnce(T) -> Result<Transformed<T>>>(self: Self, f: F) -> Result<Transformed<T>> { /* ... */ }
  ```
  Maps the [`Transformed`] object to the result of the given `f` depending on the

- ```rust
  pub fn transform_parent<F: FnOnce(T) -> Result<Transformed<T>>>(self: Self, f: F) -> Result<Transformed<T>> { /* ... */ }
  ```
  Maps the [`Transformed`] object to the result of the given `f` depending on the

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **RefUnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **MaybeSendSync**
- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Transformed<T>) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
### Traits

#### Trait `TreeNode`

API for inspecting and rewriting tree data structures.

The `TreeNode` API is used to express algorithms separately from traversing
the structure of `TreeNode`s, avoiding substantial code duplication.

This trait is implemented for plans ([`ExecutionPlan`], [`LogicalPlan`]) and
expression trees ([`PhysicalExpr`], [`Expr`]) as well as Plan+Payload
combinations [`PlanContext`] and [`ExprContext`].

# Overview
There are three categories of TreeNode APIs:

1. "Inspecting" APIs to traverse a tree of `&TreeNodes`:
   [`apply`], [`visit`], [`exists`].

2. "Transforming" APIs that traverse and consume a tree of `TreeNode`s
   producing possibly changed `TreeNode`s: [`transform`], [`transform_up`],
   [`transform_down`], [`transform_down_up`], and [`rewrite`].

3. Internal APIs used to implement the `TreeNode` API: [`apply_children`],
   and [`map_children`].

| Traversal Order | Inspecting | Transforming |
| --- | --- | --- |
| top-down | [`apply`], [`exists`] | [`transform_down`]|
| bottom-up | | [`transform`] , [`transform_up`]|
| combined with separate `f_down` and `f_up` closures | | [`transform_down_up`] |
| combined with `f_down()` and `f_up()` in an object | [`visit`]  | [`rewrite`] |

**Note**:while there is currently no in-place mutation API that uses `&mut
TreeNode`, the transforming APIs are efficient and optimized to avoid
cloning.

[`apply`]: Self::apply
[`visit`]: Self::visit
[`exists`]: Self::exists
[`transform`]: Self::transform
[`transform_up`]: Self::transform_up
[`transform_down`]: Self::transform_down
[`transform_down_up`]: Self::transform_down_up
[`rewrite`]: Self::rewrite
[`apply_children`]: Self::apply_children
[`map_children`]: Self::map_children

# Terminology
The following terms are used in this trait

* `f_down`: Invoked before any children of the current node are visited.
* `f_up`: Invoked after all children of the current node are visited.
* `f`: closure that is applied to the current node.
* `map_*`: applies a transformation to rewrite owned nodes
* `apply_*`:  invokes a function on borrowed nodes
* `transform_`: applies a transformation to rewrite owned nodes

<!-- Since these are in the datafusion-common crate, can't use intra doc links) -->
[`ExecutionPlan`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.ExecutionPlan.html
[`PhysicalExpr`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/trait.PhysicalExpr.html
[`LogicalPlan`]: https://docs.rs/datafusion-expr/latest/datafusion_expr/logical_plan/enum.LogicalPlan.html
[`Expr`]: https://docs.rs/datafusion-expr/latest/datafusion_expr/expr/enum.Expr.html
[`PlanContext`]: https://docs.rs/datafusion/latest/datafusion/physical_plan/tree_node/struct.PlanContext.html
[`ExprContext`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/tree_node/struct.ExprContext.html

```rust
pub trait TreeNode: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `apply_children`: Low-level API used to implement other APIs.
- `map_children`: Low-level API used to implement other APIs.

##### Provided Methods

- ```rust
  fn visit<''n, V: TreeNodeVisitor<''n, Node = Self>>(self: &''n Self, visitor: &mut V) -> Result<TreeNodeRecursion> { /* ... */ }
  ```
  Visit the tree node with a [`TreeNodeVisitor`], performing a

- ```rust
  fn rewrite<R: TreeNodeRewriter<Node = Self>>(self: Self, rewriter: &mut R) -> Result<Transformed<Self>> { /* ... */ }
  ```
  Rewrite the tree node with a [`TreeNodeRewriter`], performing a

- ```rust
  fn apply<''n, F: FnMut(&''n Self) -> Result<TreeNodeRecursion>>(self: &''n Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
  ```
  Applies `f` to the node then each of its children, recursively (a

- ```rust
  fn transform<F: FnMut(Self) -> Result<Transformed<Self>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
  ```
  Recursively rewrite the node's children and then the node using `f`

- ```rust
  fn transform_down<F: FnMut(Self) -> Result<Transformed<Self>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
  ```
  Recursively rewrite the tree using `f` in a top-down (pre-order)

- ```rust
  fn transform_up<F: FnMut(Self) -> Result<Transformed<Self>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
  ```
  Recursively rewrite the node using `f` in a bottom-up (post-order)

- ```rust
  fn transform_down_up<FD: FnMut(Self) -> Result<Transformed<Self>>, FU: FnMut(Self) -> Result<Transformed<Self>>>(self: Self, f_down: FD, f_up: FU) -> Result<Transformed<Self>> { /* ... */ }
  ```
  Transforms the node using `f_down` while traversing the tree top-down

- ```rust
  fn exists<F: FnMut(&Self) -> Result<bool>>(self: &Self, f: F) -> Result<bool> { /* ... */ }
  ```
  Returns true if `f` returns true for any node in the tree.

##### Implementations

This trait is implemented for the following types:

- `std::sync::Arc<T>` with <T: DynTreeNode + ?Sized>
- `T` with <T: ConcreteTreeNode>

#### Trait `TreeNodeVisitor`

A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively
inspecting [`TreeNode`]s via [`TreeNode::visit`].

See [`TreeNode`] for more details on available APIs

When passed to [`TreeNode::visit`], [`TreeNodeVisitor::f_down`] and
[`TreeNodeVisitor::f_up`] are invoked recursively on the tree.
See [`TreeNodeRecursion`] for more details on controlling the traversal.

# Return Value
The returns value of `f_up` and `f_down` specifies how the tree walk should
proceed. See [`TreeNodeRecursion`] for details. If an [`Err`] is returned,
the recursion stops immediately.

Note: If using the default implementations of [`TreeNodeVisitor::f_up`] or
[`TreeNodeVisitor::f_down`] that do nothing, consider using
[`TreeNode::apply`] instead.

# See Also:
* [`TreeNode::rewrite`] to rewrite owned `TreeNode`s

```rust
pub trait TreeNodeVisitor<''n>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Node`: The node type which is visitable.

##### Provided Methods

- ```rust
  fn f_down(self: &mut Self, _node: &''n <Self as >::Node) -> Result<TreeNodeRecursion> { /* ... */ }
  ```
  Invoked while traversing down the tree, before any children are visited.

- ```rust
  fn f_up(self: &mut Self, _node: &''n <Self as >::Node) -> Result<TreeNodeRecursion> { /* ... */ }
  ```
  Invoked while traversing up the tree after children are visited. Default

#### Trait `TreeNodeRewriter`

A [Visitor](https://en.wikipedia.org/wiki/Visitor_pattern) for recursively
rewriting [`TreeNode`]s via [`TreeNode::rewrite`].

For example you can implement this trait on a struct to rewrite `Expr` or
`LogicalPlan` that needs to track state during the rewrite.

See [`TreeNode`] for more details on available APIs

When passed to [`TreeNode::rewrite`], [`TreeNodeRewriter::f_down`] and
[`TreeNodeRewriter::f_up`] are invoked recursively on the tree.
See [`TreeNodeRecursion`] for more details on controlling the traversal.

# Return Value
The returns value of `f_up` and `f_down` specifies how the tree walk should
proceed. See [`TreeNodeRecursion`] for details. If an [`Err`] is returned,
the recursion stops immediately.

Note: If using the default implementations of [`TreeNodeRewriter::f_up`] or
[`TreeNodeRewriter::f_down`] that do nothing, consider using
[`TreeNode::transform_up`] or [`TreeNode::transform_down`] instead.

# See Also:
* [`TreeNode::visit`] to inspect borrowed `TreeNode`s

```rust
pub trait TreeNodeRewriter: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `Node`: The node type which is rewritable.

##### Provided Methods

- ```rust
  fn f_down(self: &mut Self, node: <Self as >::Node) -> Result<Transformed<<Self as >::Node>> { /* ... */ }
  ```
  Invoked while traversing down the tree before any children are rewritten.

- ```rust
  fn f_up(self: &mut Self, node: <Self as >::Node) -> Result<Transformed<<Self as >::Node>> { /* ... */ }
  ```
  Invoked while traversing up the tree after all children have been rewritten.

#### Trait `TreeNodeContainer`

[`TreeNodeContainer`] contains elements that a function can be applied on or mapped.
The elements of the container are siblings so the continuation rules are similar to
[`TreeNodeRecursion::visit_sibling`] / [`Transformed::transform_sibling`].

```rust
pub trait TreeNodeContainer<''a, T: ''a>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `apply_elements`: Applies `f` to all elements of the container.
- `map_elements`: Maps all elements of the container with `f`.

##### Implementations

This trait is implemented for the following types:

- `Box<C>` with <''a, T: ''a, C: TreeNodeContainer<''a, T>>
- `std::sync::Arc<C>` with <''a, T: ''a, C: TreeNodeContainer<''a, T> + Clone>
- `Option<C>` with <''a, T: ''a, C: TreeNodeContainer<''a, T>>
- `Vec<C>` with <''a, T: ''a, C: TreeNodeContainer<''a, T>>
- `std::collections::HashMap<K, C>` with <''a, T: ''a, K: Eq + Hash, C: TreeNodeContainer<''a, T>>
- `(C0, C1)` with <''a, T: ''a, C0: TreeNodeContainer<''a, T>, C1: TreeNodeContainer<''a, T>>
- `(C0, C1, C2)` with <''a, T: ''a, C0: TreeNodeContainer<''a, T>, C1: TreeNodeContainer<''a, T>, C2: TreeNodeContainer<''a, T>>

#### Trait `TreeNodeRefContainer`

[`TreeNodeRefContainer`] contains references to elements that a function can be
applied on. The elements of the container are siblings so the continuation rules are
similar to [`TreeNodeRecursion::visit_sibling`].

This container is similar to [`TreeNodeContainer`], but the lifetime of the reference
elements (`T`) are not derived from the container's lifetime.
A typical usage of this container is in `Expr::apply_children` when we need to
construct a temporary container to be able to call `apply_ref_elements` on a
collection of tree node references. But in that case the container's temporary
lifetime is different to the lifetime of tree nodes that we put into it.
Please find an example use case in `Expr::apply_children` with the `Expr::Case` case.

Most of the cases we don't need to create a temporary container with
`TreeNodeRefContainer`, but we can just call `TreeNodeContainer::apply_elements`.
Please find an example use case in `Expr::apply_children` with the `Expr::GroupingSet`
case.

```rust
pub trait TreeNodeRefContainer<''a, T: ''a>: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `apply_ref_elements`: Applies `f` to all elements of the container.

##### Implementations

This trait is implemented for the following types:

- `Vec<&''a C>` with <''a, T: ''a, C: TreeNodeContainer<''a, T>>
- `(&''a C0, &''a C1)` with <''a, T: ''a, C0: TreeNodeContainer<''a, T>, C1: TreeNodeContainer<''a, T>>
- `(&''a C0, &''a C1, &''a C2)` with <''a, T: ''a, C0: TreeNodeContainer<''a, T>, C1: TreeNodeContainer<''a, T>, C2: TreeNodeContainer<''a, T>>

#### Trait `TreeNodeIterator`

Transformation helper to process a sequence of iterable tree nodes that are siblings.

```rust
pub trait TreeNodeIterator: Iterator {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `apply_until_stop`: Apples `f` to each item in this iterator
- `map_until_stop_and_collect`: Apples `f` to each item in this iterator

##### Implementations

This trait is implemented for the following types:

- `I` with <I: Iterator>

#### Trait `TransformedResult`

Transformation helper to access [`Transformed`] fields in a [`Result`] easily.

# Example
Access the internal data of a `Result<Transformed<T>>`
as a `Result<T>` using the `data` method:
```
# use datafusion_common::Result;
# use datafusion_common::tree_node::{Transformed, TransformedResult};
# // note use i64 instead of Expr as Expr is not in datafusion-common
# fn update_expr() -> i64 { 1 }
# fn main() -> Result<()> {
let transformed: Result<Transformed<_>> = Ok(Transformed::yes(update_expr()));
// access the internal data of the transformed result, or return the error
let transformed_expr = transformed.data()?;
# Ok(())
# }
```

```rust
pub trait TransformedResult<T> {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `data`
- `transformed`
- `tnr`

##### Implementations

This trait is implemented for the following types:

- `crate::Result<Transformed<T>>` with <T>

#### Trait `DynTreeNode`

Helper trait for implementing [`TreeNode`] that have children stored as
`Arc`s. If some trait object, such as `dyn T`, implements this trait,
its related `Arc<dyn T>` will automatically implement [`TreeNode`].

```rust
pub trait DynTreeNode {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `arc_children`: Returns all children of the specified `TreeNode`.
- `with_new_arc_children`: Constructs a new node with the specified children.

#### Trait `ConcreteTreeNode`

Instead of implementing [`TreeNode`], it's recommended to implement a [`ConcreteTreeNode`] for
trees that contain nodes with payloads. This approach ensures safe execution of algorithms
involving payloads, by enforcing rules for detaching and reattaching child nodes.

```rust
pub trait ConcreteTreeNode: Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `children`: Provides read-only access to child nodes.
- `take_children`: Detaches the node from its children, returning the node itself and its detached children.
- `with_new_children`: Reattaches updated child nodes to the node, returning the updated node.

## Module `types`

```rust
pub mod types { /* ... */ }
```

### Re-exports

#### Re-export `builtin::*`

```rust
pub use builtin::*;
```

#### Re-export `field::*`

```rust
pub use field::*;
```

#### Re-export `logical::*`

```rust
pub use logical::*;
```

#### Re-export `native::*`

```rust
pub use native::*;
```

## Module `utils`

This module provides the bisect function, which implements binary search.

```rust
pub mod utils { /* ... */ }
```

### Modules

## Module `expr`

Expression utilities

```rust
pub mod expr { /* ... */ }
```

### Constants and Statics

#### Constant `COUNT_STAR_EXPANSION`

The value to which `COUNT(*)` is expanded to in
`COUNT(<constant>)` expressions

```rust
pub const COUNT_STAR_EXPANSION: crate::ScalarValue = _;
```

## Module `memory`

This module provides a function to estimate the memory size of a HashTable prior to allocation

```rust
pub mod memory { /* ... */ }
```

### Functions

#### Function `estimate_memory_size`

Estimates the memory size required for a hash table prior to allocation.

# Parameters
- `num_elements`: The number of elements expected in the hash table.
- `fixed_size`: A fixed overhead size associated with the collection
  (e.g., HashSet or HashTable).
- `T`: The type of elements stored in the hash table.

# Details
This function calculates the estimated memory size by considering:
- An overestimation of buckets to keep approximately 1/8 of them empty.
- The total memory size is computed as:
  - The size of each entry (`T`) multiplied by the estimated number of
    buckets.
  - One byte overhead for each bucket.
  - The fixed size overhead of the collection.
- If the estimation overflows, we return a [`DataFusionError`]

# Examples
---

## From within a struct

```rust
# use datafusion_common::utils::memory::estimate_memory_size;
# use datafusion_common::Result;

struct MyStruct<T> {
    values: Vec<T>,
    other_data: usize,
}

impl<T> MyStruct<T> {
    fn size(&self) -> Result<usize> {
        let num_elements = self.values.len();
        let fixed_size = std::mem::size_of_val(self) +
          std::mem::size_of_val(&self.values);

        estimate_memory_size::<T>(num_elements, fixed_size)
    }
}
```
---
## With a simple collection

```rust
# use datafusion_common::utils::memory::estimate_memory_size;
# use std::collections::HashMap;

let num_rows = 100;
let fixed_size = std::mem::size_of::<HashMap<u64, u64>>();
let estimated_hashtable_size =
  estimate_memory_size::<(u64, u64)>(num_rows,fixed_size)
    .expect("Size estimation failed");
```

```rust
pub fn estimate_memory_size<T>(num_elements: usize, fixed_size: usize) -> crate::Result<usize> { /* ... */ }
```

## Module `proxy`

[`VecAllocExt`] and [`RawTableAllocExt`] to help tracking of memory allocations

```rust
pub mod proxy { /* ... */ }
```

### Traits

#### Trait `VecAllocExt`

Extension trait for [`Vec`] to account for allocations.

```rust
pub trait VecAllocExt {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `T`: Item type.

###### Required Methods

- `push_accounted`: [Push](Vec::push) new element to vector and increase
- `allocated_size`: Return the amount of memory allocated by this Vec to store elements

##### Implementations

This trait is implemented for the following types:

- `Vec<T>` with <T>

#### Trait `RawTableAllocExt`

Extension trait for hash browns [`RawTable`] to account for allocations.

```rust
pub trait RawTableAllocExt {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `T`: Item type.

###### Required Methods

- `insert_accounted`: [Insert](RawTable::insert) new element into table and increase

##### Implementations

This trait is implemented for the following types:

- `hashbrown::raw::RawTable<T>` with <T>

#### Trait `HashTableAllocExt`

Extension trait for hash browns [`HashTable`] to account for allocations.

```rust
pub trait HashTableAllocExt {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Associated Types

- `T`: Item type.

###### Required Methods

- `insert_accounted`: Insert new element into table and increase

##### Implementations

This trait is implemented for the following types:

- `hashbrown::hash_table::HashTable<T>` with <T>

## Module `string_utils`

Utilities for working with strings

```rust
pub mod string_utils { /* ... */ }
```

### Functions

#### Function `string_array_to_vec`

Convenient function to convert an Arrow string array to a vector of strings

```rust
pub fn string_array_to_vec(array: &dyn Array) -> Vec<Option<&str>> { /* ... */ }
```

## Module `datafusion_strsim`

Adopted from strsim-rs for string similarity metrics

```rust
pub mod datafusion_strsim { /* ... */ }
```

### Functions

#### Function `levenshtein`

Calculates the minimum number of insertions, deletions, and substitutions
required to change one string into the other.

```
use datafusion_common::utils::datafusion_strsim::levenshtein;

assert_eq!(3, levenshtein("kitten", "sitting"));
```

```rust
pub fn levenshtein(a: &str, b: &str) -> usize { /* ... */ }
```

#### Function `normalized_levenshtein`

Calculates the normalized Levenshtein distance between two strings.
The normalized distance is a value between 0.0 and 1.0, where 1.0 indicates
that the strings are identical and 0.0 indicates no similarity.

```
use datafusion_common::utils::datafusion_strsim::normalized_levenshtein;

assert!((normalized_levenshtein("kitten", "sitting") - 0.57142).abs() < 0.00001);

assert!(normalized_levenshtein("", "second").abs() < 0.00001);

assert!((normalized_levenshtein("kitten", "sitten") - 0.833).abs() < 0.001);
```

```rust
pub fn normalized_levenshtein(a: &str, b: &str) -> f64 { /* ... */ }
```

### Types

#### Struct `SingleRowListArrayBuilder`

Creates single element [`ListArray`], [`LargeListArray`] and
[`FixedSizeListArray`] from other arrays

For example this builder can convert `[1, 2, 3]` into `[[1, 2, 3]]`

# Example
```
# use std::sync::Arc;
# use arrow::array::{Array, ListArray};
# use arrow::array::types::Int64Type;
# use datafusion_common::utils::SingleRowListArrayBuilder;
// Array is [1, 2, 3]
let arr = ListArray::from_iter_primitive::<Int64Type, _, _>(vec![
      Some(vec![Some(1), Some(2), Some(3)]),
]);
// Wrap as a list array: [[1, 2, 3]]
let list_arr = SingleRowListArrayBuilder::new(Arc::new(arr)).build_list_array();
assert_eq!(list_arr.len(), 1);
```

```rust
pub struct SingleRowListArrayBuilder {
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
  pub fn new(arr: ArrayRef) -> Self { /* ... */ }
  ```
  Create a new instance of [`SingleRowListArrayBuilder`]

- ```rust
  pub fn with_nullable(self: Self, nullable: bool) -> Self { /* ... */ }
  ```
  Set the nullable flag

- ```rust
  pub fn with_field_name(self: Self, field_name: Option<String>) -> Self { /* ... */ }
  ```
  sets the field name for the resulting array

- ```rust
  pub fn with_field(self: Self, field: &Field) -> Self { /* ... */ }
  ```
  Copies field name and nullable from the specified field

- ```rust
  pub fn build_list_array(self: Self) -> ListArray { /* ... */ }
  ```
  Build a single element [`ListArray`]

- ```rust
  pub fn build_list_scalar(self: Self) -> ScalarValue { /* ... */ }
  ```
  Build a single element [`ListArray`] and wrap as [`ScalarValue::List`]

- ```rust
  pub fn build_large_list_array(self: Self) -> LargeListArray { /* ... */ }
  ```
  Build a single element [`LargeListArray`]

- ```rust
  pub fn build_large_list_scalar(self: Self) -> ScalarValue { /* ... */ }
  ```
  Build a single element [`LargeListArray`] and wrap as [`ScalarValue::LargeList`]

- ```rust
  pub fn build_fixed_size_list_array(self: Self, list_size: usize) -> FixedSizeListArray { /* ... */ }
  ```
  Build a single element [`FixedSizeListArray`]

- ```rust
  pub fn build_fixed_size_list_scalar(self: Self, list_size: usize) -> ScalarValue { /* ... */ }
  ```
  Build a single element [`FixedSizeListArray`] and wrap as [`ScalarValue::FixedSizeList`]

###### Trait Implementations

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SingleRowListArrayBuilder { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `ListCoercion`

Information about how to coerce lists.

```rust
pub enum ListCoercion {
    FixedSizedListToList,
}
```

##### Variants

###### `FixedSizedListToList`

[`DataType::FixedSizeList`] should be coerced to [`DataType::List`].

##### Implementations

###### Trait Implementations

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ListCoercion) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **IntoEither**
- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Equivalent**
  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

  - ```rust
    fn equivalent(self: &Self, key: &K) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ListCoercion) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ListCoercion { /* ... */ }
    ```

- **Eq**
### Functions

#### Function `project_schema`

Applies an optional projection to a [`SchemaRef`], returning the
projected schema

Example:
```
use arrow::datatypes::{SchemaRef, Schema, Field, DataType};
use datafusion_common::project_schema;

// Schema with columns 'a', 'b', and 'c'
let schema = SchemaRef::new(Schema::new(vec![
  Field::new("a", DataType::Int32, true),
  Field::new("b", DataType::Int64, true),
  Field::new("c", DataType::Utf8, true),
]));

// Pick columns 'c' and 'b'
let projection = Some(vec![2,1]);
let projected_schema = project_schema(
   &schema,
   projection.as_ref()
 ).unwrap();

let expected_schema = SchemaRef::new(Schema::new(vec![
  Field::new("c", DataType::Utf8, true),
  Field::new("b", DataType::Int64, true),
]));

assert_eq!(projected_schema, expected_schema);
```

```rust
pub fn project_schema(schema: &arrow::datatypes::SchemaRef, projection: Option<&Vec<usize>>) -> crate::Result<arrow::datatypes::SchemaRef> { /* ... */ }
```

#### Function `extract_row_at_idx_to_buf`

Extracts a row at the specified index from a set of columns and stores it in the provided buffer.

```rust
pub fn extract_row_at_idx_to_buf(columns: &[arrow::array::ArrayRef], idx: usize, buf: &mut Vec<crate::ScalarValue>) -> crate::Result<()> { /* ... */ }
```

#### Function `get_row_at_idx`

Given column vectors, returns row at `idx`.

```rust
pub fn get_row_at_idx(columns: &[arrow::array::ArrayRef], idx: usize) -> crate::Result<Vec<crate::ScalarValue>> { /* ... */ }
```

#### Function `compare_rows`

This function compares two tuples depending on the given sort options.

```rust
pub fn compare_rows(x: &[crate::ScalarValue], y: &[crate::ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> crate::Result<std::cmp::Ordering> { /* ... */ }
```

#### Function `bisect`

This function searches for a tuple of given values (`target`) among the given
rows (`item_columns`) using the bisection algorithm. It assumes that `item_columns`
is sorted according to `sort_options` and returns the insertion index of `target`.
Template argument `SIDE` being `true`/`false` means left/right insertion.

```rust
pub fn bisect<const SIDE: bool>(item_columns: &[arrow::array::ArrayRef], target: &[crate::ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> crate::Result<usize> { /* ... */ }
```

#### Function `find_bisect_point`

This function searches for a tuple of given values (`target`) among a slice of
the given rows (`item_columns`) using the bisection algorithm. The slice starts
at the index `low` and ends at the index `high`. The boolean-valued function
`compare_fn` specifies whether we bisect on the left (by returning `false`),
or on the right (by returning `true`) when we compare the target value with
the current value as we iteratively bisect the input.

```rust
pub fn find_bisect_point<F>(item_columns: &[arrow::array::ArrayRef], target: &[crate::ScalarValue], compare_fn: F, low: usize, high: usize) -> crate::Result<usize>
where
    F: Fn(&[crate::ScalarValue], &[crate::ScalarValue]) -> crate::Result<bool> { /* ... */ }
```

#### Function `linear_search`

This function searches for a tuple of given values (`target`) among the given
rows (`item_columns`) via a linear scan. It assumes that `item_columns` is sorted
according to `sort_options` and returns the insertion index of `target`.
Template argument `SIDE` being `true`/`false` means left/right insertion.

```rust
pub fn linear_search<const SIDE: bool>(item_columns: &[arrow::array::ArrayRef], target: &[crate::ScalarValue], sort_options: &[arrow::compute::SortOptions]) -> crate::Result<usize> { /* ... */ }
```

#### Function `search_in_slice`

This function searches for a tuple of given values (`target`) among a slice of
the given rows (`item_columns`) via a linear scan. The slice starts at the index
`low` and ends at the index `high`. The boolean-valued function `compare_fn`
specifies the stopping criterion.

```rust
pub fn search_in_slice<F>(item_columns: &[arrow::array::ArrayRef], target: &[crate::ScalarValue], compare_fn: F, low: usize, high: usize) -> crate::Result<usize>
where
    F: Fn(&[crate::ScalarValue], &[crate::ScalarValue]) -> crate::Result<bool> { /* ... */ }
```

#### Function `evaluate_partition_ranges`

Given a list of 0 or more already sorted columns, finds the
partition ranges that would partition equally across columns.

See [`partition`] for more details.

```rust
pub fn evaluate_partition_ranges(num_rows: usize, partition_columns: &[arrow::compute::SortColumn]) -> crate::Result<Vec<std::ops::Range<usize>>> { /* ... */ }
```

#### Function `quote_identifier`

Wraps identifier string in double quotes, escaping any double quotes in
the identifier by replacing it with two double quotes

e.g. identifier `tab.le"name` becomes `"tab.le""name"`

```rust
pub fn quote_identifier(s: &str) -> std::borrow::Cow<''_, str> { /* ... */ }
```

#### Function `get_at_indices`

This function "takes" the elements at `indices` from the slice `items`.

```rust
pub fn get_at_indices<T: Clone, I: Borrow<usize>, /* synthetic */ impl IntoIterator<Item = I>: IntoIterator<Item = I>>(items: &[T], indices: impl IntoIterator<Item = I>) -> crate::Result<Vec<T>> { /* ... */ }
```

#### Function `longest_consecutive_prefix`

This function finds the longest prefix of the form 0, 1, 2, ... within the
collection `sequence`. Examples:
- For 0, 1, 2, 4, 5; we would produce 3, meaning 0, 1, 2 is the longest satisfying
  prefix.
- For 1, 2, 3, 4; we would produce 0, meaning there is no such prefix.

```rust
pub fn longest_consecutive_prefix<T: Borrow<usize>, /* synthetic */ impl IntoIterator<Item = T>: IntoIterator<Item = T>>(sequence: impl IntoIterator<Item = T>) -> usize { /* ... */ }
```

#### Function `array_into_list_array_nullable`

**Attributes:**

- `#[deprecated(since = "44.0.0", note =
"please use `SingleRowListArrayBuilder` instead")]`

**⚠️ Deprecated since 44.0.0**: please use `SingleRowListArrayBuilder` instead

Wrap an array into a single element `ListArray`.
For example `[1, 2, 3]` would be converted into `[[1, 2, 3]]`
The field in the list array is nullable.

```rust
pub fn array_into_list_array_nullable(arr: arrow::array::ArrayRef) -> arrow::array::ListArray { /* ... */ }
```

#### Function `array_into_list_array`

**Attributes:**

- `#[deprecated(since = "44.0.0", note =
"please use `SingleRowListArrayBuilder` instead")]`

**⚠️ Deprecated since 44.0.0**: please use `SingleRowListArrayBuilder` instead

Wrap an array into a single element `ListArray`.
For example `[1, 2, 3]` would be converted into `[[1, 2, 3]]`

```rust
pub fn array_into_list_array(arr: arrow::array::ArrayRef, nullable: bool) -> arrow::array::ListArray { /* ... */ }
```

#### Function `array_into_list_array_with_field_name`

**Attributes:**

- `#[deprecated(since = "44.0.0", note =
"please use `SingleRowListArrayBuilder` instead")]`

**⚠️ Deprecated since 44.0.0**: please use `SingleRowListArrayBuilder` instead

```rust
pub fn array_into_list_array_with_field_name(arr: arrow::array::ArrayRef, nullable: bool, field_name: &str) -> arrow::array::ListArray { /* ... */ }
```

#### Function `array_into_large_list_array`

**Attributes:**

- `#[deprecated(since = "44.0.0", note =
"please use `SingleRowListArrayBuilder` instead")]`

**⚠️ Deprecated since 44.0.0**: please use `SingleRowListArrayBuilder` instead

Wrap an array into a single element `LargeListArray`.
For example `[1, 2, 3]` would be converted into `[[1, 2, 3]]`

```rust
pub fn array_into_large_list_array(arr: arrow::array::ArrayRef) -> arrow::array::LargeListArray { /* ... */ }
```

#### Function `array_into_large_list_array_with_field_name`

**Attributes:**

- `#[deprecated(since = "44.0.0", note =
"please use `SingleRowListArrayBuilder` instead")]`

**⚠️ Deprecated since 44.0.0**: please use `SingleRowListArrayBuilder` instead

```rust
pub fn array_into_large_list_array_with_field_name(arr: arrow::array::ArrayRef, field_name: &str) -> arrow::array::LargeListArray { /* ... */ }
```

#### Function `array_into_fixed_size_list_array`

**Attributes:**

- `#[deprecated(since = "44.0.0", note =
"please use `SingleRowListArrayBuilder` instead")]`

**⚠️ Deprecated since 44.0.0**: please use `SingleRowListArrayBuilder` instead

```rust
pub fn array_into_fixed_size_list_array(arr: arrow::array::ArrayRef, list_size: usize) -> arrow::array::FixedSizeListArray { /* ... */ }
```

#### Function `array_into_fixed_size_list_array_with_field_name`

**Attributes:**

- `#[deprecated(since = "44.0.0", note =
"please use `SingleRowListArrayBuilder` instead")]`

**⚠️ Deprecated since 44.0.0**: please use `SingleRowListArrayBuilder` instead

```rust
pub fn array_into_fixed_size_list_array_with_field_name(arr: arrow::array::ArrayRef, list_size: usize, field_name: &str) -> arrow::array::FixedSizeListArray { /* ... */ }
```

#### Function `arrays_into_list_array`

Wrap arrays into a single element `ListArray`.

Example:
```
use arrow::array::{Int32Array, ListArray, ArrayRef};
use arrow::datatypes::{Int32Type, Field};
use std::sync::Arc;

let arr1 = Arc::new(Int32Array::from(vec![1, 2, 3])) as ArrayRef;
let arr2 = Arc::new(Int32Array::from(vec![4, 5, 6])) as ArrayRef;

let list_arr = datafusion_common::utils::arrays_into_list_array([arr1, arr2]).unwrap();

let expected = ListArray::from_iter_primitive::<Int32Type, _, _>(
   vec![
    Some(vec![Some(1), Some(2), Some(3)]),
    Some(vec![Some(4), Some(5), Some(6)]),
   ]
);

assert_eq!(list_arr, expected);

```rust
pub fn arrays_into_list_array</* synthetic */ impl IntoIterator<Item = ArrayRef>: IntoIterator<Item = arrow::array::ArrayRef>>(arr: impl IntoIterator<Item = arrow::array::ArrayRef>) -> crate::Result<arrow::array::ListArray> { /* ... */ }
```

#### Function `list_to_arrays`

Helper function to convert a ListArray into a vector of ArrayRefs.

```rust
pub fn list_to_arrays<O: OffsetSizeTrait>(a: &arrow::array::ArrayRef) -> Vec<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `fixed_size_list_to_arrays`

Helper function to convert a FixedSizeListArray into a vector of ArrayRefs.

```rust
pub fn fixed_size_list_to_arrays(a: &arrow::array::ArrayRef) -> Vec<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `base_type`

Get the base type of a data type.

Example
```
use arrow::datatypes::{DataType, Field};
use datafusion_common::utils::base_type;
use std::sync::Arc;

let data_type = DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
assert_eq!(base_type(&data_type), DataType::Int32);

let data_type = DataType::Int32;
assert_eq!(base_type(&data_type), DataType::Int32);
```

```rust
pub fn base_type(data_type: &arrow::datatypes::DataType) -> arrow::datatypes::DataType { /* ... */ }
```

#### Function `coerced_type_with_base_type_only`

A helper function to coerce base type in List.

Example
```
use arrow::datatypes::{DataType, Field};
use datafusion_common::utils::coerced_type_with_base_type_only;
use std::sync::Arc;

let data_type = DataType::List(Arc::new(Field::new_list_field(DataType::Int32, true)));
let base_type = DataType::Float64;
let coerced_type = coerced_type_with_base_type_only(&data_type, &base_type, None);
assert_eq!(coerced_type, DataType::List(Arc::new(Field::new_list_field(DataType::Float64, true))));

```rust
pub fn coerced_type_with_base_type_only(data_type: &arrow::datatypes::DataType, base_type: &arrow::datatypes::DataType, array_coercion: Option<&ListCoercion>) -> arrow::datatypes::DataType { /* ... */ }
```

#### Function `coerced_fixed_size_list_to_list`

Recursively coerce and `FixedSizeList` elements to `List`

```rust
pub fn coerced_fixed_size_list_to_list(data_type: &arrow::datatypes::DataType) -> arrow::datatypes::DataType { /* ... */ }
```

#### Function `list_ndims`

Compute the number of dimensions in a list data type.

```rust
pub fn list_ndims(data_type: &arrow::datatypes::DataType) -> u64 { /* ... */ }
```

#### Function `merge_and_order_indices`

Merges collections `first` and `second`, removes duplicates and sorts the
result, returning it as a [`Vec`].

```rust
pub fn merge_and_order_indices<T: Borrow<usize>, S: Borrow<usize>, /* synthetic */ impl IntoIterator<Item = T>: IntoIterator<Item = T>, /* synthetic */ impl IntoIterator<Item = S>: IntoIterator<Item = S>>(first: impl IntoIterator<Item = T>, second: impl IntoIterator<Item = S>) -> Vec<usize> { /* ... */ }
```

#### Function `set_difference`

Calculates the set difference between sequences `first` and `second`,
returning the result as a [`Vec`]. Preserves the ordering of `first`.

```rust
pub fn set_difference<T: Borrow<usize>, S: Borrow<usize>, /* synthetic */ impl IntoIterator<Item = T>: IntoIterator<Item = T>, /* synthetic */ impl IntoIterator<Item = S>: IntoIterator<Item = S>>(first: impl IntoIterator<Item = T>, second: impl IntoIterator<Item = S>) -> Vec<usize> { /* ... */ }
```

#### Function `is_sorted`

**Attributes:**

- `#[deprecated(since = "45.0.0", note = "Use std::Iterator::is_sorted instead")]`

**⚠️ Deprecated since 45.0.0**: Use std::Iterator::is_sorted instead

Checks whether the given index sequence is monotonically non-decreasing.

```rust
pub fn is_sorted<T: Borrow<usize>, /* synthetic */ impl IntoIterator<Item = T>: IntoIterator<Item = T>>(sequence: impl IntoIterator<Item = T>) -> bool { /* ... */ }
```

#### Function `find_indices`

Find indices of each element in `targets` inside `items`. If one of the
elements is absent in `items`, returns an error.

```rust
pub fn find_indices<T: PartialEq, S: Borrow<T>, /* synthetic */ impl IntoIterator<Item = S>: IntoIterator<Item = S>>(items: &[T], targets: impl IntoIterator<Item = S>) -> crate::Result<Vec<usize>> { /* ... */ }
```

#### Function `transpose`

Transposes the given vector of vectors.

```rust
pub fn transpose<T>(original: Vec<Vec<T>>) -> Vec<Vec<T>> { /* ... */ }
```

#### Function `combine_limit`

Computes the `skip` and `fetch` parameters of a single limit that would be
equivalent to two consecutive limits with the given `skip`/`fetch` parameters.

There are multiple cases to consider:

# Case 0: Parent and child are disjoint (`child_fetch <= skip`).

```text
  Before merging:
                    |........skip........|---fetch-->|     Parent limit
   |...child_skip...|---child_fetch-->|                    Child limit
```

  After merging:
```text
   |.........(child_skip + skip).........|
```

# Case 1: Parent is beyond child's range (`skip < child_fetch <= skip + fetch`).

  Before merging:
```text
                    |...skip...|------------fetch------------>|   Parent limit
   |...child_skip...|-------------child_fetch------------>|       Child limit
```

  After merging:
```text
   |....(child_skip + skip)....|---(child_fetch - skip)-->|
```

 # Case 2: Parent is within child's range (`skip + fetch < child_fetch`).

  Before merging:
```text
                    |...skip...|---fetch-->|                   Parent limit
   |...child_skip...|-------------child_fetch------------>|    Child limit
```

  After merging:
```text
   |....(child_skip + skip)....|---fetch-->|
```

```rust
pub fn combine_limit(parent_skip: usize, parent_fetch: Option<usize>, child_skip: usize, child_fetch: Option<usize>) -> (usize, Option<usize>) { /* ... */ }
```

#### Function `get_available_parallelism`

Returns the estimated number of threads available for parallel execution.

This is a wrapper around `std::thread::available_parallelism`, providing a default value
of `1` if the system's parallelism cannot be determined.

```rust
pub fn get_available_parallelism() -> usize { /* ... */ }
```

#### Function `take_function_args`

Converts a collection of function arguments into an fixed-size array of length N
producing a reasonable error message in case of unexpected number of arguments.

# Example
```
# use datafusion_common::Result;
# use datafusion_common::utils::take_function_args;
# use datafusion_common::ScalarValue;
fn my_function(args: &[ScalarValue]) -> Result<()> {
  // function expects 2 args, so create a 2-element array
  let [arg1, arg2] = take_function_args("my_function", args)?;
  // ... do stuff..
  Ok(())
}

// Calling the function with 1 argument produces an error:
let args = vec![ScalarValue::Int32(Some(10))];
let err = my_function(&args).unwrap_err();
assert_eq!(err.to_string(), "Execution error: my_function function requires 2 arguments, got 1");
// Calling the function with 2 arguments works great
let args = vec![ScalarValue::Int32(Some(10)), ScalarValue::Int32(Some(20))];
my_function(&args).unwrap();
```

```rust
pub fn take_function_args<const N: usize, T, /* synthetic */ impl IntoIterator<Item = T>: IntoIterator<Item = T>>(function_name: &str, args: impl IntoIterator<Item = T>) -> crate::Result<[T; N]> { /* ... */ }
```

## Types

### Type Alias `HashMap`

```rust
pub type HashMap<K, V, S = hashbrown::hash_map::DefaultHashBuilder> = hashbrown::HashMap<K, V, S>;
```

### Type Alias `HashSet`

```rust
pub type HashSet<T, S = hashbrown::hash_map::DefaultHashBuilder> = hashbrown::HashSet<T, S>;
```

## Macros

### Macro `config_namespace`

**Attributes:**

- `#[macro_export]`

 A macro that wraps a configuration struct and automatically derives
 [`Default`] and [`ConfigField`] for it, allowing it to be used
 in the [`ConfigOptions`] configuration tree.

 `transform` is used to normalize values before parsing.

 For example,

 ```ignore
 config_namespace! {
    /// Amazing config
    pub struct MyConfig {
        /// Field 1 doc
        field1: String, transform = str::to_lowercase, default = "".to_string()

        /// Field 2 doc
        field2: usize, default = 232

        /// Field 3 doc
        field3: Option<usize>, default = None
    }
}
 ```

 Will generate

 ```ignore
 /// Amazing config
 #[derive(Debug, Clone)]
 #[non_exhaustive]
 pub struct MyConfig {
     /// Field 1 doc
     field1: String,
     /// Field 2 doc
     field2: usize,
     /// Field 3 doc
     field3: Option<usize>,
 }
 impl ConfigField for MyConfig {
     fn set(&mut self, key: &str, value: &str) -> Result<()> {
         let (key, rem) = key.split_once('.').unwrap_or((key, ""));
         match key {
             "field1" => {
                 let value = str::to_lowercase(value);
                 self.field1.set(rem, value.as_ref())
             },
             "field2" => self.field2.set(rem, value.as_ref()),
             "field3" => self.field3.set(rem, value.as_ref()),
             _ => _internal_err!(
                 "Config value \"{}\" not found on MyConfig",
                 key
             ),
         }
     }

     fn visit<V: Visit>(&self, v: &mut V, key_prefix: &str, _description: &'static str) {
         let key = format!("{}.field1", key_prefix);
         let desc = "Field 1 doc";
         self.field1.visit(v, key.as_str(), desc);
         let key = format!("{}.field2", key_prefix);
         let desc = "Field 2 doc";
         self.field2.visit(v, key.as_str(), desc);
         let key = format!("{}.field3", key_prefix);
         let desc = "Field 3 doc";
         self.field3.visit(v, key.as_str(), desc);
     }
 }

 impl Default for MyConfig {
     fn default() -> Self {
         Self {
             field1: "".to_string(),
             field2: 232,
             field3: None,
         }
     }
 }
 ```

 NB: Misplaced commas may result in nonsensical errors

```rust
pub macro_rules! config_namespace {
    /* macro_rules! config_namespace {
    (
        $(#[doc = $struct_d:tt])* // Struct-level documentation attributes
        $(#[deprecated($($struct_depr:tt)*)])? // Optional struct-level deprecated attribute
        $(#[allow($($struct_de:tt)*)])?
        $vis:vis struct $struct_name:ident {
            $(
                $(#[doc = $d:tt])* // Field-level documentation attributes
                $(#[deprecated($($field_depr:tt)*)])? // Optional field-level deprecated attribute
                $(#[allow($($field_de:tt)*)])?
                $field_vis:vis $field_name:ident : $field_type:ty,
                $(warn = $warn:expr,)?
                $(transform = $transform:expr,)?
                default = $default:expr
            )*$(,)*
        }
    ) => { ... };
} */
}
```

### Macro `config_field`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! config_field {
    /* macro_rules! config_field {
    ($t:ty) => { ... };
    ($t:ty, $arg:ident => $transform:expr) => { ... };
} */
}
```

### Macro `extensions_options`

**Attributes:**

- `#[macro_export]`

Convenience macro to create [`ExtensionsOptions`].

The created structure implements the following traits:

- [`Clone`]
- [`Debug`]
- [`Default`]
- [`ExtensionOptions`]

# Usage
The syntax is:

```text
extensions_options! {
     /// Struct docs (optional).
    [<vis>] struct <StructName> {
        /// Field docs (optional)
        [<vis>] <field_name>: <field_type>, default = <default_value>

        ... more fields
    }
}
```

The placeholders are:
- `[<vis>]`: Optional visibility modifier like `pub` or `pub(crate)`.
- `<StructName>`: Struct name like `MyStruct`.
- `<field_name>`: Field name like `my_field`.
- `<field_type>`: Field type like `u8`.
- `<default_value>`: Default value matching the field type like `42`.

# Example
See also a full example on the [`ConfigExtension`] documentation

```
use datafusion_common::extensions_options;

extensions_options! {
    /// My own config options.
    pub struct MyConfig {
        /// Should "foo" be replaced by "bar"?
        pub foo_to_bar: bool, default = true

        /// How many "baz" should be created?
        pub baz_count: usize, default = 1337
    }
}
```


[`Debug`]: std::fmt::Debug
[`ExtensionsOptions`]: crate::config::ExtensionOptions

```rust
pub macro_rules! extensions_options {
    /* macro_rules! extensions_options {
    (
     $(#[doc = $struct_d:tt])*
     $vis:vis struct $struct_name:ident {
        $(
        $(#[doc = $d:tt])*
        $field_vis:vis $field_name:ident : $field_type:ty, default = $default:expr
        )*$(,)*
    }
    ) => { ... };
} */
}
```

### Macro `context`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! context {
    /* macro_rules! context {
    ($desc:expr, $err:expr) => { ... };
} */
}
```

### Macro `unwrap_or_internal_err`

**Attributes:**

- `#[macro_export]`

Unwrap an `Option` if possible. Otherwise return an `DataFusionError::Internal`.
In normal usage of DataFusion the unwrap should always succeed.

Example: `let values = unwrap_or_internal_err!(values)`

```rust
pub macro_rules! unwrap_or_internal_err {
    /* macro_rules! unwrap_or_internal_err {
    ($Value: ident) => { ... };
} */
}
```

### Macro `sql_datafusion_err`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! sql_datafusion_err {
    /* macro_rules! sql_datafusion_err {
    ($ERR:expr) => { ... };
} */
}
```

### Macro `sql_err`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! sql_err {
    /* macro_rules! sql_err {
    ($ERR:expr) => { ... };
} */
}
```

### Macro `arrow_datafusion_err`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! arrow_datafusion_err {
    /* macro_rules! arrow_datafusion_err {
    ($ERR:expr) => { ... };
} */
}
```

### Macro `arrow_err`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! arrow_err {
    /* macro_rules! arrow_err {
    ($ERR:expr) => { ... };
} */
}
```

### Macro `schema_datafusion_err`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! schema_datafusion_err {
    /* macro_rules! schema_datafusion_err {
    ($ERR:expr) => { ... };
} */
}
```

### Macro `schema_err`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! schema_err {
    /* macro_rules! schema_err {
    ($ERR:expr) => { ... };
} */
}
```

### Macro `assert_batches_eq`

**Attributes:**

- `#[macro_export]`

Compares formatted output of a record batch with an expected
vector of strings, with the result of pretty formatting record
batches. This is a macro so errors appear on the correct line

Designed so that failure output can be directly copy/pasted
into the test code as expected results.

Expects to be called about like this:

`assert_batches_eq!(expected_lines: &[&str], batches: &[RecordBatch])`

# Example
```
# use std::sync::Arc;
# use arrow::record_batch::RecordBatch;
# use arrow::array::{ArrayRef, Int32Array};
# use datafusion_common::assert_batches_eq;
let col: ArrayRef = Arc::new(Int32Array::from(vec![1, 2]));
 let batch = RecordBatch::try_from_iter([("column", col)]).unwrap();
// Expected output is a vec of strings
let expected = vec![
    "+--------+",
    "| column |",
    "+--------+",
    "| 1      |",
    "| 2      |",
    "+--------+",
];
// compare the formatted output of the record batch with the expected output
assert_batches_eq!(expected, &[batch]);
```

```rust
pub macro_rules! assert_batches_eq {
    /* macro_rules! assert_batches_eq {
    ($EXPECTED_LINES: expr, $CHUNKS: expr) => { ... };
} */
}
```

### Macro `assert_batches_sorted_eq`

**Attributes:**

- `#[macro_export]`

Compares formatted output of a record batch with an expected
vector of strings in a way that order does not matter.
This is a macro so errors appear on the correct line

See [`assert_batches_eq`] for more details and example.

Expects to be called about like this:

`assert_batch_sorted_eq!(expected_lines: &[&str], batches: &[RecordBatch])`

```rust
pub macro_rules! assert_batches_sorted_eq {
    /* macro_rules! assert_batches_sorted_eq {
    ($EXPECTED_LINES: expr, $CHUNKS: expr) => { ... };
} */
}
```

### Macro `assert_contains`

**Attributes:**

- `#[macro_export]`

A macro to assert that one string is contained within another with
a nice error message if they are not.

Usage: `assert_contains!(actual, expected)`

Is a macro so test error
messages are on the same line as the failure;

Both arguments must be convertable into Strings ([`Into`]<[`String`]>)

```rust
pub macro_rules! assert_contains {
    /* macro_rules! assert_contains {
    ($ACTUAL: expr, $EXPECTED: expr) => { ... };
} */
}
```

### Macro `assert_not_contains`

**Attributes:**

- `#[macro_export]`

A macro to assert that one string is NOT contained within another with
a nice error message if they are are.

Usage: `assert_not_contains!(actual, unexpected)`

Is a macro so test error
messages are on the same line as the failure;

Both arguments must be convertable into Strings ([`Into`]<[`String`]>)

```rust
pub macro_rules! assert_not_contains {
    /* macro_rules! assert_not_contains {
    ($ACTUAL: expr, $UNEXPECTED: expr) => { ... };
} */
}
```

### Macro `create_array`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! create_array {
    /* macro_rules! create_array {
    (Boolean, $values: expr) => { ... };
    (Int8, $values: expr) => { ... };
    (Int16, $values: expr) => { ... };
    (Int32, $values: expr) => { ... };
    (Int64, $values: expr) => { ... };
    (UInt8, $values: expr) => { ... };
    (UInt16, $values: expr) => { ... };
    (UInt32, $values: expr) => { ... };
    (UInt64, $values: expr) => { ... };
    (Float16, $values: expr) => { ... };
    (Float32, $values: expr) => { ... };
    (Float64, $values: expr) => { ... };
    (Utf8, $values: expr) => { ... };
} */
}
```

### Macro `record_batch`

**Attributes:**

- `#[macro_export]`

Creates a record batch from literal slice of values, suitable for rapid
testing and development.

Example:
```
use datafusion_common::{record_batch, create_array};
let batch = record_batch!(
    ("a", Int32, vec![1, 2, 3]),
    ("b", Float64, vec![Some(4.0), None, Some(5.0)]),
    ("c", Utf8, vec!["alpha", "beta", "gamma"])
);
```

```rust
pub macro_rules! record_batch {
    /* macro_rules! record_batch {
    ($(($name: expr, $type: ident, $values: expr)),*) => { ... };
} */
}
```

### Macro `config_datafusion_err`

**Attributes:**

- `#[macro_export]`

Macro wraps `$ERR` to add backtrace feature

```rust
pub macro_rules! config_datafusion_err {
    /* macro_rules! config_datafusion_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `exec_datafusion_err`

**Attributes:**

- `#[macro_export]`

Macro wraps `$ERR` to add backtrace feature

```rust
pub macro_rules! exec_datafusion_err {
    /* macro_rules! exec_datafusion_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `internal_datafusion_err`

**Attributes:**

- `#[macro_export]`

Macro wraps `$ERR` to add backtrace feature

```rust
pub macro_rules! internal_datafusion_err {
    /* macro_rules! internal_datafusion_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `not_impl_datafusion_err`

**Attributes:**

- `#[macro_export]`

Macro wraps `$ERR` to add backtrace feature

```rust
pub macro_rules! not_impl_datafusion_err {
    /* macro_rules! not_impl_datafusion_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `plan_datafusion_err`

**Attributes:**

- `#[macro_export]`

Macro wraps `$ERR` to add backtrace feature

```rust
pub macro_rules! plan_datafusion_err {
    /* macro_rules! plan_datafusion_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `resources_datafusion_err`

**Attributes:**

- `#[macro_export]`

Macro wraps `$ERR` to add backtrace feature

```rust
pub macro_rules! resources_datafusion_err {
    /* macro_rules! resources_datafusion_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `substrait_datafusion_err`

**Attributes:**

- `#[macro_export]`

Macro wraps `$ERR` to add backtrace feature

```rust
pub macro_rules! substrait_datafusion_err {
    /* macro_rules! substrait_datafusion_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `downcast_value`

**Attributes:**

- `#[macro_export]`

Downcast an Arrow Array to a concrete type, return an `DataFusionError::Internal` if the cast is
not possible. In normal usage of DataFusion the downcast should always succeed.

Example: `let array = downcast_value!(values, Int32Array)`

```rust
pub macro_rules! downcast_value {
    /* macro_rules! downcast_value {
    ($Value: expr, $Type: ident) => { ... };
    ($Value: expr, $Type: ident, $T: tt) => { ... };
} */
}
```

### Macro `plan_err`

**Attributes:**

- `#[macro_export]`

Macro wraps Err(`$ERR`) to add backtrace feature

```rust
pub macro_rules! plan_err {
    /* macro_rules! plan_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `internal_err`

**Attributes:**

- `#[macro_export]`

Macro wraps Err(`$ERR`) to add backtrace feature

```rust
pub macro_rules! internal_err {
    /* macro_rules! internal_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `not_impl_err`

**Attributes:**

- `#[macro_export]`

Macro wraps Err(`$ERR`) to add backtrace feature

```rust
pub macro_rules! not_impl_err {
    /* macro_rules! not_impl_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `exec_err`

**Attributes:**

- `#[macro_export]`

Macro wraps Err(`$ERR`) to add backtrace feature

```rust
pub macro_rules! exec_err {
    /* macro_rules! exec_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `config_err`

**Attributes:**

- `#[macro_export]`

Macro wraps Err(`$ERR`) to add backtrace feature

```rust
pub macro_rules! config_err {
    /* macro_rules! config_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `substrait_err`

**Attributes:**

- `#[macro_export]`

Macro wraps Err(`$ERR`) to add backtrace feature

```rust
pub macro_rules! substrait_err {
    /* macro_rules! substrait_err {
    ($($args:expr),*) => { ... };
} */
}
```

### Macro `resources_err`

**Attributes:**

- `#[macro_export]`

Macro wraps Err(`$ERR`) to add backtrace feature

```rust
pub macro_rules! resources_err {
    /* macro_rules! resources_err {
    ($($args:expr),*) => { ... };
} */
}
```

## Re-exports

### Re-export `arrow`

Reexport arrow crate

```rust
pub use arrow;
```

### Re-export `Column`

```rust
pub use column::Column;
```

### Re-export `qualified_name`

```rust
pub use dfschema::qualified_name;
```

### Re-export `DFSchema`

```rust
pub use dfschema::DFSchema;
```

### Re-export `DFSchemaRef`

```rust
pub use dfschema::DFSchemaRef;
```

### Re-export `ExprSchema`

```rust
pub use dfschema::ExprSchema;
```

### Re-export `SchemaExt`

```rust
pub use dfschema::SchemaExt;
```

### Re-export `ToDFSchema`

```rust
pub use dfschema::ToDFSchema;
```

### Re-export `Diagnostic`

```rust
pub use diagnostic::Diagnostic;
```

### Re-export `field_not_found`

```rust
pub use error::field_not_found;
```

### Re-export `unqualified_field_not_found`

```rust
pub use error::unqualified_field_not_found;
```

### Re-export `DataFusionError`

```rust
pub use error::DataFusionError;
```

### Re-export `Result`

```rust
pub use error::Result;
```

### Re-export `SchemaError`

```rust
pub use error::SchemaError;
```

### Re-export `SharedResult`

```rust
pub use error::SharedResult;
```

### Re-export `GetExt`

```rust
pub use file_options::file_type::GetExt;
```

### Re-export `DEFAULT_ARROW_EXTENSION`

```rust
pub use file_options::file_type::DEFAULT_ARROW_EXTENSION;
```

### Re-export `DEFAULT_AVRO_EXTENSION`

```rust
pub use file_options::file_type::DEFAULT_AVRO_EXTENSION;
```

### Re-export `DEFAULT_CSV_EXTENSION`

```rust
pub use file_options::file_type::DEFAULT_CSV_EXTENSION;
```

### Re-export `DEFAULT_JSON_EXTENSION`

```rust
pub use file_options::file_type::DEFAULT_JSON_EXTENSION;
```

### Re-export `DEFAULT_PARQUET_EXTENSION`

```rust
pub use file_options::file_type::DEFAULT_PARQUET_EXTENSION;
```

### Re-export `aggregate_functional_dependencies`

```rust
pub use functional_dependencies::aggregate_functional_dependencies;
```

### Re-export `get_required_group_by_exprs_indices`

```rust
pub use functional_dependencies::get_required_group_by_exprs_indices;
```

### Re-export `get_target_functional_dependencies`

```rust
pub use functional_dependencies::get_target_functional_dependencies;
```

### Re-export `Constraint`

```rust
pub use functional_dependencies::Constraint;
```

### Re-export `Constraints`

```rust
pub use functional_dependencies::Constraints;
```

### Re-export `Dependency`

```rust
pub use functional_dependencies::Dependency;
```

### Re-export `FunctionalDependence`

```rust
pub use functional_dependencies::FunctionalDependence;
```

### Re-export `FunctionalDependencies`

```rust
pub use functional_dependencies::FunctionalDependencies;
```

### Re-export `JoinConstraint`

```rust
pub use join_type::JoinConstraint;
```

### Re-export `JoinSide`

```rust
pub use join_type::JoinSide;
```

### Re-export `JoinType`

```rust
pub use join_type::JoinType;
```

### Re-export `ParamValues`

```rust
pub use param_value::ParamValues;
```

### Re-export `ScalarType`

```rust
pub use scalar::ScalarType;
```

### Re-export `ScalarValue`

```rust
pub use scalar::ScalarValue;
```

### Re-export `SchemaReference`

```rust
pub use schema_reference::SchemaReference;
```

### Re-export `Location`

```rust
pub use spans::Location;
```

### Re-export `Span`

```rust
pub use spans::Span;
```

### Re-export `Spans`

```rust
pub use spans::Spans;
```

### Re-export `ColumnStatistics`

```rust
pub use stats::ColumnStatistics;
```

### Re-export `Statistics`

```rust
pub use stats::Statistics;
```

### Re-export `ResolvedTableReference`

```rust
pub use table_reference::ResolvedTableReference;
```

### Re-export `TableReference`

```rust
pub use table_reference::TableReference;
```

### Re-export `RecursionUnnestOption`

```rust
pub use unnest::RecursionUnnestOption;
```

### Re-export `UnnestOptions`

```rust
pub use unnest::UnnestOptions;
```

### Re-export `project_schema`

```rust
pub use utils::project_schema;
```


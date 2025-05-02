# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_execution`

DataFusion execution configuration and runtime structures

## Modules

## Module `cache`

```rust
pub mod cache { /* ... */ }
```

### Modules

## Module `cache_manager`

```rust
pub mod cache_manager { /* ... */ }
```

### Types

#### Type Alias `FileStatisticsCache`

The cache of listing files statistics.
if set [`CacheManagerConfig::with_files_statistics_cache`]
Will avoid infer same file statistics repeatedly during the session lifetime,
this cache will store in [`crate::runtime_env::RuntimeEnv`].

```rust
pub type FileStatisticsCache = std::sync::Arc<dyn CacheAccessor<object_store::path::Path, std::sync::Arc<datafusion_common::Statistics>, Extra = object_store::ObjectMeta>>;
```

#### Type Alias `ListFilesCache`

```rust
pub type ListFilesCache = std::sync::Arc<dyn CacheAccessor<object_store::path::Path, std::sync::Arc<Vec<object_store::ObjectMeta>>, Extra = object_store::ObjectMeta>>;
```

#### Struct `CacheManager`

```rust
pub struct CacheManager {
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
  pub fn try_new(config: &CacheManagerConfig) -> Result<Arc<Self>> { /* ... */ }
  ```

- ```rust
  pub fn get_file_statistic_cache(self: &Self) -> Option<FileStatisticsCache> { /* ... */ }
  ```
  Get the cache of listing files statistics.

- ```rust
  pub fn get_list_files_cache(self: &Self) -> Option<ListFilesCache> { /* ... */ }
  ```
  Get the cache of objectMeta under same path.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Default**
  - ```rust
    fn default() -> CacheManager { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `CacheManagerConfig`

```rust
pub struct CacheManagerConfig {
    pub table_files_statistics_cache: Option<FileStatisticsCache>,
    pub list_files_cache: Option<ListFilesCache>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `table_files_statistics_cache` | `Option<FileStatisticsCache>` | Enable cache of files statistics when listing files.<br>Avoid get same file statistics repeatedly in same datafusion session.<br>Default is disable. Fow now only supports Parquet files. |
| `list_files_cache` | `Option<ListFilesCache>` | Enable cache of file metadata when listing files.<br>This setting avoids listing file meta of the same path repeatedly<br>in same session, which may be expensive in certain situations (e.g. remote object storage).<br>Note that if this option is enabled, DataFusion will not see any updates to the underlying<br>location.  <br>Default is disable. |

##### Implementations

###### Methods

- ```rust
  pub fn with_files_statistics_cache(self: Self, cache: Option<FileStatisticsCache>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_list_files_cache(self: Self, cache: Option<ListFilesCache>) -> Self { /* ... */ }
  ```

###### Trait Implementations

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CacheManagerConfig { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> CacheManagerConfig { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
## Module `cache_unit`

```rust
pub mod cache_unit { /* ... */ }
```

### Types

#### Struct `DefaultFileStatisticsCache`

Collected statistics for files
Cache is invalided when file size or last modification has changed

```rust
pub struct DefaultFileStatisticsCache {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **Unpin**
- **CacheAccessor**
  - ```rust
    fn get(self: &Self, k: &Path) -> Option<Arc<Statistics>> { /* ... */ }
    ```
    Get `Statistics` for file location.

  - ```rust
    fn get_with_extra(self: &Self, k: &Path, e: &<Self as >::Extra) -> Option<Arc<Statistics>> { /* ... */ }
    ```
    Get `Statistics` for file location. Returns None if file has changed or not found.

  - ```rust
    fn put(self: &Self, _key: &Path, _value: Arc<Statistics>) -> Option<Arc<Statistics>> { /* ... */ }
    ```
    Save collected file statistics

  - ```rust
    fn put_with_extra(self: &Self, key: &Path, value: Arc<Statistics>, e: &<Self as >::Extra) -> Option<Arc<Statistics>> { /* ... */ }
    ```

  - ```rust
    fn remove(self: &mut Self, k: &Path) -> Option<Arc<Statistics>> { /* ... */ }
    ```

  - ```rust
    fn contains_key(self: &Self, k: &Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn clear(self: &Self) { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> String { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Default**
  - ```rust
    fn default() -> DefaultFileStatisticsCache { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `DefaultListFilesCache`

Collected files metadata for listing files.
Cache will not invalided until user call remove or clear.

```rust
pub struct DefaultListFilesCache {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> DefaultListFilesCache { /* ... */ }
    ```

- **Send**
- **CacheAccessor**
  - ```rust
    fn get(self: &Self, k: &Path) -> Option<Arc<Vec<ObjectMeta>>> { /* ... */ }
    ```

  - ```rust
    fn get_with_extra(self: &Self, _k: &Path, _e: &<Self as >::Extra) -> Option<Arc<Vec<ObjectMeta>>> { /* ... */ }
    ```

  - ```rust
    fn put(self: &Self, key: &Path, value: Arc<Vec<ObjectMeta>>) -> Option<Arc<Vec<ObjectMeta>>> { /* ... */ }
    ```

  - ```rust
    fn put_with_extra(self: &Self, _key: &Path, _value: Arc<Vec<ObjectMeta>>, _e: &<Self as >::Extra) -> Option<Arc<Vec<ObjectMeta>>> { /* ... */ }
    ```

  - ```rust
    fn remove(self: &mut Self, k: &Path) -> Option<Arc<Vec<ObjectMeta>>> { /* ... */ }
    ```

  - ```rust
    fn contains_key(self: &Self, k: &Path) -> bool { /* ... */ }
    ```

  - ```rust
    fn len(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn clear(self: &Self) { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> String { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `CacheAccessor`

The cache accessor, users usually working on this interface while manipulating caches.
This interface does not get `mut` references and thus has to handle its own
locking via internal mutability. It can be accessed via multiple concurrent queries
during planning and execution.

```rust
pub trait CacheAccessor<K, V>: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Associated Types

- `Extra`

###### Required Methods

- `get`: Get value from cache.
- `get_with_extra`: Get value from cache.
- `put`: Put value into cache. Returns the old value associated with the key if there was one.
- `put_with_extra`: Put value into cache. Returns the old value associated with the key if there was one.
- `remove`: Remove an entry from the cache, returning value if they existed in the map.
- `contains_key`: Check if the cache contains a specific key.
- `len`: Fetch the total number of cache entries.
- `clear`: Remove all entries from the cache.
- `name`: Return the cache name.

##### Provided Methods

- ```rust
  fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Check if the Cache collection is empty or not.

##### Implementations

This trait is implemented for the following types:

- `DefaultFileStatisticsCache`
- `DefaultListFilesCache`

## Module `config`

```rust
pub mod config { /* ... */ }
```

### Types

#### Struct `SessionConfig`

Configuration options for [`SessionContext`].

Can be passed to [`SessionContext::new_with_config`] to customize the configuration of DataFusion.

Options can be set using namespaces keys with `.` as the separator, where the
namespace determines which configuration struct the value to routed to. All
built-in options are under the `datafusion` namespace.

For example, the key `datafusion.execution.batch_size` will set [ExecutionOptions::batch_size][datafusion_common::config::ExecutionOptions::batch_size],
because [ConfigOptions::execution] is [ExecutionOptions][datafusion_common::config::ExecutionOptions]. Similarly, the key
`datafusion.execution.parquet.pushdown_filters` will set [ParquetOptions::pushdown_filters][datafusion_common::config::ParquetOptions::pushdown_filters],
since [ExecutionOptions::parquet][datafusion_common::config::ExecutionOptions::parquet] is [ParquetOptions][datafusion_common::config::ParquetOptions].

Some options have convenience methods. For example [SessionConfig::with_batch_size] is
shorthand for setting `datafusion.execution.batch_size`.

```
use datafusion_execution::config::SessionConfig;
use datafusion_common::ScalarValue;

let config = SessionConfig::new()
   .set("datafusion.execution.batch_size", &ScalarValue::UInt64(Some(1234)))
   .set_bool("datafusion.execution.parquet.pushdown_filters", true);

assert_eq!(config.batch_size(), 1234);
assert_eq!(config.options().execution.batch_size, 1234);
assert_eq!(config.options().execution.parquet.pushdown_filters, true);
```

You can also directly mutate the options via [SessionConfig::options_mut].
So the following is equivalent to the above:

```
# use datafusion_execution::config::SessionConfig;
# use datafusion_common::ScalarValue;
#
let mut config = SessionConfig::new();
config.options_mut().execution.batch_size = 1234;
config.options_mut().execution.parquet.pushdown_filters = true;
#
# assert_eq!(config.batch_size(), 1234);
# assert_eq!(config.options().execution.batch_size, 1234);
# assert_eq!(config.options().execution.parquet.pushdown_filters, true);
```

## Built-in options

| Namespace | Config struct |
| --------- | ------------- |
| `datafusion.catalog` | [CatalogOptions][datafusion_common::config::CatalogOptions] |
| `datafusion.execution` | [ExecutionOptions][datafusion_common::config::ExecutionOptions] |
| `datafusion.execution.parquet` | [ParquetOptions][datafusion_common::config::ParquetOptions] |
| `datafusion.optimizer` | [OptimizerOptions][datafusion_common::config::OptimizerOptions] |
| `datafusion.sql_parser` | [SqlParserOptions][datafusion_common::config::SqlParserOptions] |
| `datafusion.explain` | [ExplainOptions][datafusion_common::config::ExplainOptions] |

## Custom configuration

Configuration options can be extended. See [SessionConfig::with_extension] for details.

[`SessionContext`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html
[`SessionContext::new_with_config`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.new_with_config

```rust
pub struct SessionConfig {
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
  Create an execution config with default setting

- ```rust
  pub fn from_env() -> Result<Self> { /* ... */ }
  ```
  Create an execution config with config options read from the environment

- ```rust
  pub fn from_string_hash_map(settings: &HashMap<String, String>) -> Result<Self> { /* ... */ }
  ```
  Create new ConfigOptions struct, taking values from a string hash map.

- ```rust
  pub fn options(self: &Self) -> &ConfigOptions { /* ... */ }
  ```
  Return a handle to the configuration options.

- ```rust
  pub fn options_mut(self: &mut Self) -> &mut ConfigOptions { /* ... */ }
  ```
  Return a mutable handle to the configuration options.

- ```rust
  pub fn set(self: Self, key: &str, value: &ScalarValue) -> Self { /* ... */ }
  ```
  Set a configuration option

- ```rust
  pub fn set_bool(self: Self, key: &str, value: bool) -> Self { /* ... */ }
  ```
  Set a boolean configuration option

- ```rust
  pub fn set_u64(self: Self, key: &str, value: u64) -> Self { /* ... */ }
  ```
  Set a generic `u64` configuration option

- ```rust
  pub fn set_usize(self: Self, key: &str, value: usize) -> Self { /* ... */ }
  ```
  Set a generic `usize` configuration option

- ```rust
  pub fn set_str(self: Self, key: &str, value: &str) -> Self { /* ... */ }
  ```
  Set a generic `str` configuration option

- ```rust
  pub fn with_batch_size(self: Self, n: usize) -> Self { /* ... */ }
  ```
  Customize batch size

- ```rust
  pub fn with_target_partitions(self: Self, n: usize) -> Self { /* ... */ }
  ```
  Customize [`target_partitions`]

- ```rust
  pub fn with_option_extension<T: ConfigExtension>(self: Self, extension: T) -> Self { /* ... */ }
  ```
  Insert new [ConfigExtension]

- ```rust
  pub fn target_partitions(self: &Self) -> usize { /* ... */ }
  ```
  Get [`target_partitions`]

- ```rust
  pub fn information_schema(self: &Self) -> bool { /* ... */ }
  ```
  Is the information schema enabled?

- ```rust
  pub fn create_default_catalog_and_schema(self: &Self) -> bool { /* ... */ }
  ```
  Should the context create the default catalog and schema?

- ```rust
  pub fn repartition_joins(self: &Self) -> bool { /* ... */ }
  ```
  Are joins repartitioned during execution?

- ```rust
  pub fn repartition_aggregations(self: &Self) -> bool { /* ... */ }
  ```
  Are aggregates repartitioned during execution?

- ```rust
  pub fn repartition_window_functions(self: &Self) -> bool { /* ... */ }
  ```
  Are window functions repartitioned during execution?

- ```rust
  pub fn repartition_sorts(self: &Self) -> bool { /* ... */ }
  ```
  Do we execute sorts in a per-partition fashion and merge afterwards,

- ```rust
  pub fn prefer_existing_sort(self: &Self) -> bool { /* ... */ }
  ```
  Prefer existing sort (true) or maximize parallelism (false). See

- ```rust
  pub fn collect_statistics(self: &Self) -> bool { /* ... */ }
  ```
  Are statistics collected during execution?

- ```rust
  pub fn with_default_catalog_and_schema</* synthetic */ impl Into<String>: Into<String>, /* synthetic */ impl Into<String>: Into<String>>(self: Self, catalog: impl Into<String>, schema: impl Into<String>) -> Self { /* ... */ }
  ```
  Selects a name for the default catalog and schema

- ```rust
  pub fn with_create_default_catalog_and_schema(self: Self, create: bool) -> Self { /* ... */ }
  ```
  Controls whether the default catalog and schema will be automatically created

- ```rust
  pub fn with_information_schema(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the inclusion of `information_schema` virtual tables

- ```rust
  pub fn with_repartition_joins(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the use of repartitioning for joins to improve parallelism

- ```rust
  pub fn with_repartition_aggregations(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the use of repartitioning for aggregations to improve parallelism

- ```rust
  pub fn with_repartition_file_min_size(self: Self, size: usize) -> Self { /* ... */ }
  ```
  Sets minimum file range size for repartitioning scans

- ```rust
  pub fn with_allow_symmetric_joins_without_pruning(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the allowing unordered symmetric hash join

- ```rust
  pub fn with_repartition_file_scans(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the use of repartitioning for file scans

- ```rust
  pub fn with_repartition_windows(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the use of repartitioning for window functions to improve parallelism

- ```rust
  pub fn with_repartition_sorts(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the use of per-partition sorting to improve parallelism

- ```rust
  pub fn with_prefer_existing_sort(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Prefer existing sort (true) or maximize parallelism (false). See

- ```rust
  pub fn with_prefer_existing_union(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Prefer existing union (true). See [prefer_existing_union] for more details

- ```rust
  pub fn with_parquet_pruning(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the use of pruning predicate for parquet readers to skip row groups

- ```rust
  pub fn parquet_pruning(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if pruning predicate should be used to skip parquet row groups

- ```rust
  pub fn parquet_bloom_filter_pruning(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if bloom filter should be used to skip parquet row groups

- ```rust
  pub fn with_parquet_bloom_filter_pruning(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the use of bloom filter for parquet readers to skip row groups

- ```rust
  pub fn parquet_page_index_pruning(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if page index should be used to skip parquet data pages

- ```rust
  pub fn with_parquet_page_index_pruning(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the use of page index for parquet readers to skip parquet data pages

- ```rust
  pub fn with_collect_statistics(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the collection of statistics after listing files

- ```rust
  pub fn batch_size(self: &Self) -> usize { /* ... */ }
  ```
  Get the currently configured batch size

- ```rust
  pub fn with_coalesce_batches(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the coalescence of small batches into larger batches

- ```rust
  pub fn coalesce_batches(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if record batches will be examined between each operator

- ```rust
  pub fn with_round_robin_repartition(self: Self, enabled: bool) -> Self { /* ... */ }
  ```
  Enables or disables the round robin repartition for increasing parallelism

- ```rust
  pub fn round_robin_repartition(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the physical plan optimizer will try to

- ```rust
  pub fn with_sort_spill_reservation_bytes(self: Self, sort_spill_reservation_bytes: usize) -> Self { /* ... */ }
  ```
  Set the size of [`sort_spill_reservation_bytes`] to control

- ```rust
  pub fn with_sort_in_place_threshold_bytes(self: Self, sort_in_place_threshold_bytes: usize) -> Self { /* ... */ }
  ```
  Set the size of [`sort_in_place_threshold_bytes`] to control

- ```rust
  pub fn with_enforce_batch_size_in_joins(self: Self, enforce_batch_size_in_joins: bool) -> Self { /* ... */ }
  ```
  Enables or disables the enforcement of batch size in joins

- ```rust
  pub fn enforce_batch_size_in_joins(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the joins will be enforced to output batches of the configured size

- ```rust
  pub fn to_props(self: &Self) -> HashMap<String, String> { /* ... */ }
  ```
  Convert configuration options to name-value pairs with values

- ```rust
  pub fn with_extension<T>(self: Self, ext: Arc<T>) -> Self
where
    T: Send + Sync + ''static { /* ... */ }
  ```
  Add extensions.

- ```rust
  pub fn set_extension<T>(self: &mut Self, ext: Arc<T>)
where
    T: Send + Sync + ''static { /* ... */ }
  ```
  Set extension. Pretty much the same as [`with_extension`](Self::with_extension), but take

- ```rust
  pub fn get_extension<T>(self: &Self) -> Option<Arc<T>>
where
    T: Send + Sync + ''static { /* ... */ }
  ```
  Get extension, if any for the specified type `T` exists.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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
- **Unpin**
- **UnwindSafe**
- **Send**
- **MaybeSendSync**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SessionConfig { /* ... */ }
    ```

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

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(options: ConfigOptions) -> Self { /* ... */ }
    ```

## Module `disk_manager`

[`DiskManager`]: Manages files generated during query execution

```rust
pub mod disk_manager { /* ... */ }
```

### Types

#### Enum `DiskManagerConfig`

Configuration for temporary disk access

```rust
pub enum DiskManagerConfig {
    Existing(std::sync::Arc<DiskManager>),
    NewOs,
    NewSpecified(Vec<std::path::PathBuf>),
    Disabled,
}
```

##### Variants

###### `Existing`

Use the provided [DiskManager] instance

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<DiskManager>` |  |

###### `NewOs`

Create a new [DiskManager] that creates temporary files within
a temporary directory chosen by the OS

###### `NewSpecified`

Create a new [DiskManager] that creates temporary files within
the specified directories

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<std::path::PathBuf>` |  |

###### `Disabled`

Disable disk manager, attempts to create temporary files will error

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create temporary files in a temporary directory chosen by the OS

- ```rust
  pub fn new_existing(existing: Arc<DiskManager>) -> Self { /* ... */ }
  ```
  Create temporary files using the provided disk manager

- ```rust
  pub fn new_specified(paths: Vec<PathBuf>) -> Self { /* ... */ }
  ```
  Create temporary files in the specified directories

###### Trait Implementations

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DiskManagerConfig { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
#### Struct `DiskManager`

Manages files generated during query execution, e.g. spill files generated
while processing dataset larger than available memory.

```rust
pub struct DiskManager {
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
  pub fn try_new(config: DiskManagerConfig) -> Result<Arc<Self>> { /* ... */ }
  ```
  Create a DiskManager given the configuration

- ```rust
  pub fn with_max_temp_directory_size(self: Self, max_temp_directory_size: u64) -> Result<Self> { /* ... */ }
  ```

- ```rust
  pub fn used_disk_space(self: &Self) -> u64 { /* ... */ }
  ```

- ```rust
  pub fn tmp_files_enabled(self: &Self) -> bool { /* ... */ }
  ```
  Return true if this disk manager supports creating temporary

- ```rust
  pub fn create_tmp_file(self: &Arc<Self>, request_description: &str) -> Result<RefCountedTempFile> { /* ... */ }
  ```
  Return a temporary file from a randomized choice in the configured locations

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
#### Struct `RefCountedTempFile`

A wrapper around a [`NamedTempFile`] that also contains
a reference to its parent temporary directory.

# Note
After any modification to the underlying file (e.g., writing data to it), the caller
must invoke [`Self::update_disk_usage`] to update the global disk usage counter.
This ensures the disk manager can properly enforce usage limits configured by
[`DiskManager::with_max_temp_directory_size`].

```rust
pub struct RefCountedTempFile {
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
  pub fn path(self: &Self) -> &Path { /* ... */ }
  ```

- ```rust
  pub fn inner(self: &Self) -> &NamedTempFile { /* ... */ }
  ```

- ```rust
  pub fn update_disk_usage(self: &mut Self) -> Result<()> { /* ... */ }
  ```
  Updates the global disk usage counter after modifications to the underlying file.

###### Trait Implementations

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Send**
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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
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
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
## Module `memory_pool`

[`MemoryPool`] for memory management during query execution, [`proxy`] for
help with allocation accounting.

```rust
pub mod memory_pool { /* ... */ }
```

### Modules

## Module `proxy`

```rust
pub mod proxy { /* ... */ }
```

### Re-exports

#### Re-export `HashTableAllocExt`

```rust
pub use datafusion_common::utils::proxy::HashTableAllocExt;
```

#### Re-export `RawTableAllocExt`

```rust
pub use datafusion_common::utils::proxy::RawTableAllocExt;
```

#### Re-export `VecAllocExt`

```rust
pub use datafusion_common::utils::proxy::VecAllocExt;
```

## Module `units`

```rust
pub mod units { /* ... */ }
```

### Constants and Statics

#### Constant `TB`

```rust
pub const TB: u64 = _;
```

#### Constant `GB`

```rust
pub const GB: u64 = _;
```

#### Constant `MB`

```rust
pub const MB: u64 = _;
```

#### Constant `KB`

```rust
pub const KB: u64 = _;
```

### Types

#### Struct `MemoryConsumer`

A memory consumer is a named allocation traced by a particular
[`MemoryReservation`] in a [`MemoryPool`]. All allocations are registered to
a particular `MemoryConsumer`;

Each `MemoryConsumer` is identifiable by a process-unique id, and is therefor not cloneable,
If you want a clone of a `MemoryConsumer`, you should look into [`MemoryConsumer::clone_with_new_id`],
but note that this `MemoryConsumer` may be treated as a separate entity based on the used pool,
and is only guaranteed to share the name and inner properties.

For help with allocation accounting, see the [`proxy`] module.

[proxy]: datafusion_common::utils::proxy

```rust
pub struct MemoryConsumer {
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
  pub fn new</* synthetic */ impl Into<String>: Into<String>>(name: impl Into<String>) -> Self { /* ... */ }
  ```
  Create a new empty [`MemoryConsumer`] that can be grown using [`MemoryReservation`]

- ```rust
  pub fn clone_with_new_id(self: &Self) -> Self { /* ... */ }
  ```
  Returns a clone of this [`MemoryConsumer`] with a new unique id,

- ```rust
  pub fn id(self: &Self) -> usize { /* ... */ }
  ```
  Return the unique id of this [`MemoryConsumer`]

- ```rust
  pub fn with_can_spill(self: Self, can_spill: bool) -> Self { /* ... */ }
  ```
  Set whether this allocation can be spilled to disk

- ```rust
  pub fn can_spill(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this allocation can spill to disk

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Returns the name associated with this allocation

- ```rust
  pub fn register(self: Self, pool: &Arc<dyn MemoryPool>) -> MemoryReservation { /* ... */ }
  ```
  Registers this [`MemoryConsumer`] with the provided [`MemoryPool`] returning

###### Trait Implementations

- **Send**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Allocation**
- **UnwindSafe**
- **Eq**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
#### Struct `MemoryReservation`

A [`MemoryReservation`] tracks an individual reservation of a
number of bytes of memory in a [`MemoryPool`] that is freed back
to the pool on drop.

The reservation can be grown or shrunk over time.

```rust
pub struct MemoryReservation {
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
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Returns the size of this reservation in bytes

- ```rust
  pub fn consumer(self: &Self) -> &MemoryConsumer { /* ... */ }
  ```
  Returns [MemoryConsumer] for this [MemoryReservation]

- ```rust
  pub fn free(self: &mut Self) -> usize { /* ... */ }
  ```
  Frees all bytes from this reservation back to the underlying

- ```rust
  pub fn shrink(self: &mut Self, capacity: usize) { /* ... */ }
  ```
  Frees `capacity` bytes from this reservation

- ```rust
  pub fn try_shrink(self: &mut Self, capacity: usize) -> Result<usize> { /* ... */ }
  ```
  Tries to free `capacity` bytes from this reservation

- ```rust
  pub fn resize(self: &mut Self, capacity: usize) { /* ... */ }
  ```
  Sets the size of this reservation to `capacity`

- ```rust
  pub fn try_resize(self: &mut Self, capacity: usize) -> Result<()> { /* ... */ }
  ```
  Try to set the size of this reservation to `capacity`

- ```rust
  pub fn grow(self: &mut Self, capacity: usize) { /* ... */ }
  ```
  Increase the size of this reservation by `capacity` bytes

- ```rust
  pub fn try_grow(self: &mut Self, capacity: usize) -> Result<()> { /* ... */ }
  ```
  Try to increase the size of this reservation by `capacity`

- ```rust
  pub fn split(self: &mut Self, capacity: usize) -> MemoryReservation { /* ... */ }
  ```
  Splits off `capacity` bytes from this [`MemoryReservation`]

- ```rust
  pub fn new_empty(self: &Self) -> Self { /* ... */ }
  ```
  Returns a new empty [`MemoryReservation`] with the same [`MemoryConsumer`]

- ```rust
  pub fn take(self: &mut Self) -> MemoryReservation { /* ... */ }
  ```
  Splits off all the bytes from this [`MemoryReservation`] into

###### Trait Implementations

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Sync**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `MemoryPool`

Tracks and potentially limits memory use across operators during execution.

# Memory Management Overview

DataFusion is a streaming query engine, processing most queries without
buffering the entire input. Most operators require a fixed amount of memory
based on the schema and target batch size. However, certain operations such
as sorting and grouping/joining, require buffering intermediate results,
which can require memory proportional to the number of input rows.

Rather than tracking all allocations, DataFusion takes a pragmatic approach:
Intermediate memory used as data streams through the system is not accounted
(it assumed to be "small") but the large consumers of memory must register
and constrain their use. This design trades off the additional code
complexity of memory tracking with limiting resource usage.

When limiting memory with a `MemoryPool` you should typically reserve some
overhead (e.g. 10%) for the "small" memory allocations that are not tracked.

# Memory Management Design

As explained above, DataFusion's design ONLY limits operators that require
"large" amounts of memory (proportional to number of input rows), such as
`GroupByHashExec`. It does NOT track and limit memory used internally by
other operators such as `DataSourceExec` or the `RecordBatch`es that flow
between operators. Furthermore, operators should not reserve memory for the
batches they produce. Instead, if a parent operator needs to hold batches
from its children in memory for an extended period, it is the parent
operator's responsibility to reserve the necessary memory for those batches.

In order to avoid allocating memory until the OS or the container system
kills the process, DataFusion `ExecutionPlan`s (operators) that consume
large amounts of memory must first request their desired allocation from a
[`MemoryPool`] before allocating more.  The request is typically managed via
a  [`MemoryReservation`] and [`MemoryConsumer`].

If the allocation is successful, the operator should proceed and allocate
the desired memory. If the allocation fails, the operator must either first
free memory (e.g. by spilling to local disk) and try again, or error.

Note that a `MemoryPool` can be shared by concurrently executing plans,
which can be used to control memory usage in a multi-tenant system.

# How MemoryPool works by example

Scenario 1:
For `Filter` operator, `RecordBatch`es will stream through it, so it
don't have to keep track of memory usage through [`MemoryPool`].

Scenario 2:
For `CrossJoin` operator, if the input size gets larger, the intermediate
state will also grow. So `CrossJoin` operator will use [`MemoryPool`] to
limit the memory usage.
2.1 `CrossJoin` operator has read a new batch, asked memory pool for
additional memory. Memory pool updates the usage and returns success.
2.2 `CrossJoin` has read another batch, and tries to reserve more memory
again, memory pool does not have enough memory. Since `CrossJoin` operator
has not implemented spilling, it will stop execution and return an error.

Scenario 3:
For `Aggregate` operator, its intermediate states will also accumulate as
the input size gets larger, but with spilling capability. When it tries to
reserve more memory from the memory pool, and the memory pool has already
reached the memory limit, it will return an error. Then, `Aggregate`
operator will spill the intermediate buffers to disk, and release memory
from the memory pool, and continue to retry memory reservation.

# Implementing `MemoryPool`

You can implement a custom allocation policy by implementing the
[`MemoryPool`] trait and configuring a `SessionContext` appropriately.
However, DataFusion comes with the following simple memory pool implementations that
handle many common cases:

* [`UnboundedMemoryPool`]: no memory limits (the default)

* [`GreedyMemoryPool`]: Limits memory usage to a fixed size using a "first
  come first served" policy

* [`FairSpillPool`]: Limits memory usage to a fixed size, allocating memory
  to all spilling operators fairly

* [`TrackConsumersPool`]: Wraps another [`MemoryPool`] and tracks consumers,
  providing better error messages on the largest memory users.

```rust
pub trait MemoryPool: Send + Sync + std::fmt::Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `grow`: Infallibly grow the provided `reservation` by `additional` bytes
- `shrink`: Infallibly shrink the provided `reservation` by `shrink` bytes
- `try_grow`: Attempt to grow the provided `reservation` by `additional` bytes
- `reserved`: Return the total amount of memory reserved

##### Provided Methods

- ```rust
  fn register(self: &Self, _consumer: &MemoryConsumer) { /* ... */ }
  ```
  Registers a new [`MemoryConsumer`]

- ```rust
  fn unregister(self: &Self, _consumer: &MemoryConsumer) { /* ... */ }
  ```
  Records the destruction of a [`MemoryReservation`] with [`MemoryConsumer`]

##### Implementations

This trait is implemented for the following types:

- `UnboundedMemoryPool`
- `GreedyMemoryPool`
- `FairSpillPool`
- `TrackConsumersPool<I>` with <I: MemoryPool>

### Functions

#### Function `human_readable_size`

Present size in human-readable form

```rust
pub fn human_readable_size(size: usize) -> String { /* ... */ }
```

### Re-exports

#### Re-export `pool::*`

```rust
pub use pool::*;
```

## Module `object_store`

ObjectStoreRegistry holds all the object stores at Runtime with a scheme for each store.
This allows the user to extend DataFusion with different storage systems such as S3 or HDFS
and query data inside these systems.

```rust
pub mod object_store { /* ... */ }
```

### Types

#### Struct `ObjectStoreUrl`

A parsed URL identifying a particular [`ObjectStore`] instance

For example:
* `file://` for local file system
* `s3://bucket` for AWS S3 bucket
* `oss://bucket` for Aliyun OSS bucket

```rust
pub struct ObjectStoreUrl {
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
  pub fn parse</* synthetic */ impl AsRef<str>: AsRef<str>>(s: impl AsRef<str>) -> Result<Self> { /* ... */ }
  ```
  Parse an [`ObjectStoreUrl`] from a string

- ```rust
  pub fn local_filesystem() -> Self { /* ... */ }
  ```
  An [`ObjectStoreUrl`] for the local filesystem (`file://`)

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Returns this [`ObjectStoreUrl`] as a string

###### Trait Implementations

- **ErasedDestructor**
- **Allocation**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ObjectStoreUrl { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ObjectStoreUrl) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Url { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &ObjectStoreUrl) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ObjectStoreUrl) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **Unpin**
#### Struct `DefaultObjectStoreRegistry`

The default [`ObjectStoreRegistry`]

```rust
pub struct DefaultObjectStoreRegistry {
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
  This will register [`LocalFileSystem`] to handle `file://` paths

###### Trait Implementations

- **Sync**
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

- **MaybeSendSync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **ErasedDestructor**
- **ObjectStoreRegistry**
  - ```rust
    fn register_store(self: &Self, url: &Url, store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>> { /* ... */ }
    ```

  - ```rust
    fn get_store(self: &Self, url: &Url) -> Result<Arc<dyn ObjectStore>> { /* ... */ }
    ```

### Traits

#### Trait `ObjectStoreRegistry`

[`ObjectStoreRegistry`] maps a URL to an [`ObjectStore`] instance,
and allows DataFusion to read from different [`ObjectStore`]
instances. For example DataFusion might be configured so that

1. `s3://my_bucket/lineitem/` mapped to the `/lineitem` path on an
   AWS S3 object store bound to `my_bucket`

2. `s3://my_other_bucket/lineitem/` mapped to the (same)
   `/lineitem` path on a *different* AWS S3 object store bound to
   `my_other_bucket`

When given a [`ListingTableUrl`], DataFusion tries to find an
appropriate [`ObjectStore`]. For example

```sql
create external table unicorns stored as parquet location 's3://my_bucket/lineitem/';
```

In this particular case, the url `s3://my_bucket/lineitem/` will be provided to
[`ObjectStoreRegistry::get_store`] and one of three things will happen:

- If an [`ObjectStore`] has been registered with [`ObjectStoreRegistry::register_store`] with
  `s3://my_bucket`, that [`ObjectStore`] will be returned

- If an AWS S3 object store can be ad-hoc discovered by the url `s3://my_bucket/lineitem/`, this
  object store will be registered with key `s3://my_bucket` and returned.

- Otherwise an error will be returned, indicating that no suitable [`ObjectStore`] could
  be found

This allows for two different use-cases:

1. Systems where object store buckets are explicitly created using DDL, can register these
   buckets using [`ObjectStoreRegistry::register_store`]

2. Systems relying on ad-hoc discovery, without corresponding DDL, can create [`ObjectStore`]
   lazily by providing a custom implementation of [`ObjectStoreRegistry`]

<!-- is in a different crate so normal rustdoc links don't work -->
[`ListingTableUrl`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTableUrl.html
[`ObjectStore`]: object_store::ObjectStore

```rust
pub trait ObjectStoreRegistry: Send + Sync + std::fmt::Debug + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `register_store`: If a store with the same key existed before, it is replaced and returned
- `get_store`: Get a suitable store for the provided URL. For example:

##### Implementations

This trait is implemented for the following types:

- `DefaultObjectStoreRegistry`

## Module `runtime_env`

Execution [`RuntimeEnv`] environment that manages access to object
store, memory manager, disk manager.

```rust
pub mod runtime_env { /* ... */ }
```

### Types

#### Struct `RuntimeEnv`

Execution runtime environment that manages system resources such
as memory, disk, cache and storage.

A [`RuntimeEnv`] can be created using [`RuntimeEnvBuilder`] and has the
following resource management functionality:

* [`MemoryPool`]: Manage memory
* [`DiskManager`]: Manage temporary files on local disk
* [`CacheManager`]: Manage temporary cache data during the session lifetime
* [`ObjectStoreRegistry`]: Manage mapping URLs to object store instances

# Example: Create default `RuntimeEnv`
```
# use datafusion_execution::runtime_env::RuntimeEnv;
let runtime_env = RuntimeEnv::default();
```

# Example: Create a `RuntimeEnv` from [`RuntimeEnvBuilder`] with a new memory pool
```
# use std::sync::Arc;
# use datafusion_execution::memory_pool::GreedyMemoryPool;
# use datafusion_execution::runtime_env::{RuntimeEnv, RuntimeEnvBuilder};
// restrict to using at most 100MB of memory
let pool_size = 100 * 1024 * 1024;
let runtime_env = RuntimeEnvBuilder::new()
  .with_memory_pool(Arc::new(GreedyMemoryPool::new(pool_size)))
  .build()
  .unwrap();
```

```rust
pub struct RuntimeEnv {
    pub memory_pool: std::sync::Arc<dyn MemoryPool>,
    pub disk_manager: std::sync::Arc<crate::disk_manager::DiskManager>,
    pub cache_manager: std::sync::Arc<crate::cache::cache_manager::CacheManager>,
    pub object_store_registry: std::sync::Arc<dyn ObjectStoreRegistry>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `memory_pool` | `std::sync::Arc<dyn MemoryPool>` | Runtime memory management |
| `disk_manager` | `std::sync::Arc<crate::disk_manager::DiskManager>` | Manage temporary files during query execution |
| `cache_manager` | `std::sync::Arc<crate::cache::cache_manager::CacheManager>` | Manage temporary cache during query execution |
| `object_store_registry` | `std::sync::Arc<dyn ObjectStoreRegistry>` | Object Store Registry |

##### Implementations

###### Methods

- ```rust
  pub fn new(config: RuntimeConfig) -> Result<Self> { /* ... */ }
  ```

- ```rust
  pub fn try_new(config: RuntimeConfig) -> Result<Self> { /* ... */ }
  ```
  Create env based on configuration

- ```rust
  pub fn register_object_store(self: &Self, url: &Url, object_store: Arc<dyn ObjectStore>) -> Option<Arc<dyn ObjectStore>> { /* ... */ }
  ```
  Registers a custom `ObjectStore` to be used with a specific url.

- ```rust
  pub fn object_store</* synthetic */ impl AsRef<Url>: AsRef<Url>>(self: &Self, url: impl AsRef<Url>) -> Result<Arc<dyn ObjectStore>> { /* ... */ }
  ```
  Retrieves a `ObjectStore` instance for a url by consulting the

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
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

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RuntimeEnv { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Type Alias `RuntimeConfig`

**Attributes:**

- `#[deprecated(since = "43.0.0", note =
"please use `RuntimeEnvBuilder` instead")]`

**⚠️ Deprecated since 43.0.0**: please use `RuntimeEnvBuilder` instead

Please see: <https://github.com/apache/datafusion/issues/12156>
This a type alias for backwards compatibility.

```rust
pub type RuntimeConfig = RuntimeEnvBuilder;
```

#### Struct `RuntimeEnvBuilder`

Execution runtime configuration builder.

See example on [`RuntimeEnv`]

```rust
pub struct RuntimeEnvBuilder {
    pub disk_manager: crate::disk_manager::DiskManagerConfig,
    pub memory_pool: Option<std::sync::Arc<dyn MemoryPool>>,
    pub cache_manager: crate::cache::cache_manager::CacheManagerConfig,
    pub object_store_registry: std::sync::Arc<dyn ObjectStoreRegistry>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `disk_manager` | `crate::disk_manager::DiskManagerConfig` | DiskManager to manage temporary disk file usage |
| `memory_pool` | `Option<std::sync::Arc<dyn MemoryPool>>` | [`MemoryPool`] from which to allocate memory<br><br>Defaults to using an [`UnboundedMemoryPool`] if `None` |
| `cache_manager` | `crate::cache::cache_manager::CacheManagerConfig` | CacheManager to manage cache data |
| `object_store_registry` | `std::sync::Arc<dyn ObjectStoreRegistry>` | ObjectStoreRegistry to get object store based on url |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  New with default values

- ```rust
  pub fn with_disk_manager(self: Self, disk_manager: DiskManagerConfig) -> Self { /* ... */ }
  ```
  Customize disk manager

- ```rust
  pub fn with_memory_pool(self: Self, memory_pool: Arc<dyn MemoryPool>) -> Self { /* ... */ }
  ```
  Customize memory policy

- ```rust
  pub fn with_cache_manager(self: Self, cache_manager: CacheManagerConfig) -> Self { /* ... */ }
  ```
  Customize cache policy

- ```rust
  pub fn with_object_store_registry(self: Self, object_store_registry: Arc<dyn ObjectStoreRegistry>) -> Self { /* ... */ }
  ```
  Customize object store registry

- ```rust
  pub fn with_memory_limit(self: Self, max_memory: usize, memory_fraction: f64) -> Self { /* ... */ }
  ```
  Specify the total memory to use while running the DataFusion

- ```rust
  pub fn with_temp_file_path</* synthetic */ impl Into<PathBuf>: Into<PathBuf>>(self: Self, path: impl Into<PathBuf>) -> Self { /* ... */ }
  ```
  Use the specified path to create any needed temporary files

- ```rust
  pub fn build(self: Self) -> Result<RuntimeEnv> { /* ... */ }
  ```
  Build a RuntimeEnv

- ```rust
  pub fn build_arc(self: Self) -> Result<Arc<RuntimeEnv>> { /* ... */ }
  ```
  Convenience method to create a new `Arc<RuntimeEnv>`

###### Trait Implementations

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RuntimeEnvBuilder { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Freeze**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `registry`

```rust
pub mod registry { /* ... */ }
```

### Re-exports

#### Re-export `FunctionRegistry`

```rust
pub use datafusion_expr::registry::FunctionRegistry;
```

#### Re-export `MemoryFunctionRegistry`

```rust
pub use datafusion_expr::registry::MemoryFunctionRegistry;
```

#### Re-export `SerializerRegistry`

```rust
pub use datafusion_expr::registry::SerializerRegistry;
```

## Re-exports

### Re-export `DiskManager`

```rust
pub use disk_manager::DiskManager;
```

### Re-export `FunctionRegistry`

```rust
pub use registry::FunctionRegistry;
```

### Re-export `RecordBatchStream`

```rust
pub use stream::RecordBatchStream;
```

### Re-export `SendableRecordBatchStream`

```rust
pub use stream::SendableRecordBatchStream;
```

### Re-export `TaskContext`

```rust
pub use task::TaskContext;
```


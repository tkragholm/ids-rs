# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_datasource_json`

## Modules

## Module `file_format`

[`JsonFormat`]: Line delimited JSON [`FileFormat`] abstractions

```rust
pub mod file_format { /* ... */ }
```

### Types

#### Struct `JsonFormatFactory`

Factory struct used to create [JsonFormat]

```rust
pub struct JsonFormatFactory {
    pub options: Option<datafusion_common::config::JsonOptions>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `options` | `Option<datafusion_common::config::JsonOptions>` | the options carried by format factory |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates an instance of [JsonFormatFactory]

- ```rust
  pub fn new_with_options(options: JsonOptions) -> Self { /* ... */ }
  ```
  Creates an instance of [JsonFormatFactory] with customized default options

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Default**
  - ```rust
    fn default() -> JsonFormatFactory { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **GetExt**
  - ```rust
    fn get_ext(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Sync**
- **UnwindSafe**
- **MaybeSendSync**
- **Unpin**
- **Freeze**
- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Allocation**
- **Send**
- **FileFormatFactory**
  - ```rust
    fn create(self: &Self, state: &dyn Session, format_options: &HashMap<String, String>) -> Result<Arc<dyn FileFormat>> { /* ... */ }
    ```

  - ```rust
    fn default(self: &Self) -> Arc<dyn FileFormat> { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

#### Struct `JsonFormat`

New line delimited JSON `FileFormat` implementation.

```rust
pub struct JsonFormat {
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
  pub fn with_options(self: Self, options: JsonOptions) -> Self { /* ... */ }
  ```
  Set JSON options

- ```rust
  pub fn options(self: &Self) -> &JsonOptions { /* ... */ }
  ```
  Retrieve JSON options

- ```rust
  pub fn with_schema_infer_max_rec(self: Self, max_rec: usize) -> Self { /* ... */ }
  ```
  Set a limit in terms of records to scan to infer the schema

- ```rust
  pub fn with_file_compression_type(self: Self, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Set a [`FileCompressionType`] of JSON

###### Trait Implementations

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> JsonFormat { /* ... */ }
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
    fn infer_schema<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, store: &''life2 Arc<dyn ObjectStore>, objects: &''life3 [ObjectMeta]) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<SchemaRef>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn infer_stats<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, _store: &''life2 Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &''life3 ObjectMeta) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Statistics>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn create_physical_plan<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, conf: FileScanConfig, _filters: Option<&''life2 Arc<dyn PhysicalExpr>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
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
    fn file_source(self: &Self) -> Arc<dyn FileSource> { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `JsonSerializer`

Define a struct for serializing Json records to a stream

```rust
pub struct JsonSerializer {
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
  Constructor for the JsonSerializer object

###### Trait Implementations

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **IntoEither**
- **BatchSerializer**
  - ```rust
    fn serialize(self: &Self, batch: RecordBatch, _initial: bool) -> Result<Bytes> { /* ... */ }
    ```

- **Freeze**
#### Struct `JsonSink`

Implements [`DataSink`] for writing to a Json file.

```rust
pub struct JsonSink {
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
  pub fn new(config: FileSinkConfig, writer_options: JsonWriterOptions) -> Self { /* ... */ }
  ```
  Create from config.

- ```rust
  pub fn writer_options(self: &Self) -> &JsonWriterOptions { /* ... */ }
  ```
  Retrieve the writer options

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

- **UnwindSafe**
- **Sync**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **Send**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `JsonDecoder`

```rust
pub struct JsonDecoder {
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
  pub fn new(decoder: json::reader::Decoder) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
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
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **UnwindSafe**
- **IntoEither**
- **Decoder**
  - ```rust
    fn decode(self: &mut Self, buf: &[u8]) -> Result<usize, ArrowError> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> Result<Option<RecordBatch>, ArrowError> { /* ... */ }
    ```

  - ```rust
    fn can_flush_early(self: &Self) -> bool { /* ... */ }
    ```

## Module `source`

Execution plan for reading line-delimited JSON files

```rust
pub mod source { /* ... */ }
```

### Types

#### Struct `NdJsonExec`

**Attributes:**

- `#[deprecated(since = "46.0.0", note = "use DataSourceExec instead")]`

**⚠️ Deprecated since 46.0.0**: use DataSourceExec instead

Execution plan for scanning NdJson data source

```rust
pub struct NdJsonExec {
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
  pub fn new(base_config: FileScanConfig, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Create a new JSON reader execution plan provided base configurations

- ```rust
  pub fn base_config(self: &Self) -> &FileScanConfig { /* ... */ }
  ```
  Ref to the base configs

- ```rust
  pub fn file_compression_type(self: &Self) -> &FileCompressionType { /* ... */ }
  ```
  Ref to file compression type

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NdJsonExec { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **ExecutionPlan**
  - ```rust
    fn name(self: &Self) -> &''static str { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

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
    fn repartitioned(self: &Self, target_partitions: usize, config: &datafusion_common::config::ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `JsonOpener`

A [`FileOpener`] that opens a JSON file and yields a [`FileOpenFuture`]

```rust
pub struct JsonOpener {
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
  pub fn new(batch_size: usize, projected_schema: SchemaRef, file_compression_type: FileCompressionType, object_store: Arc<dyn ObjectStore>) -> Self { /* ... */ }
  ```
  Returns a  [`JsonOpener`]

###### Trait Implementations

- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **ErasedDestructor**
- **FileOpener**
  - ```rust
    fn open(self: &Self, file_meta: FileMeta) -> Result<FileOpenFuture> { /* ... */ }
    ```
    Open a partitioned NDJSON file.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **Freeze**
- **Send**
#### Struct `JsonSource`

JsonSource holds the extra configuration that is necessary for [`JsonOpener`]

```rust
pub struct JsonSource {
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
  Initialize a JsonSource with default values

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> JsonSource { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Default**
  - ```rust
    fn default() -> JsonSource { /* ... */ }
    ```

- **FileSource**
  - ```rust
    fn create_file_opener(self: &Self, object_store: Arc<dyn ObjectStore>, base_config: &FileScanConfig, _partition: usize) -> Arc<dyn FileOpener> { /* ... */ }
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
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn file_type(self: &Self) -> &str { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
### Functions

#### Function `plan_to_json`

```rust
pub async fn plan_to_json</* synthetic */ impl AsRef<str>: AsRef<str>>(task_ctx: std::sync::Arc<datafusion_execution::TaskContext>, plan: std::sync::Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> datafusion_common::error::Result<()> { /* ... */ }
```

## Re-exports

### Re-export `file_format::*`

```rust
pub use file_format::*;
```


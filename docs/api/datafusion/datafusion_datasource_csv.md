# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_datasource_csv`

## Modules

## Module `file_format`

[`CsvFormat`], Comma Separated Value (CSV) [`FileFormat`] abstractions

```rust
pub mod file_format { /* ... */ }
```

### Types

#### Struct `CsvFormatFactory`

Factory used to create [`CsvFormat`]

```rust
pub struct CsvFormatFactory {
    pub options: Option<datafusion_common::config::CsvOptions>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `options` | `Option<datafusion_common::config::CsvOptions>` | the options for csv file read |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates an instance of [`CsvFormatFactory`]

- ```rust
  pub fn new_with_options(options: CsvOptions) -> Self { /* ... */ }
  ```
  Creates an instance of [`CsvFormatFactory`] with customized default options

###### Trait Implementations

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

- **Unpin**
- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **GetExt**
  - ```rust
    fn get_ext(self: &Self) -> String { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **Allocation**
- **Default**
  - ```rust
    fn default() -> CsvFormatFactory { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **IntoEither**
#### Struct `CsvFormat`

Character Separated Value [`FileFormat`] implementation.

```rust
pub struct CsvFormat {
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
  pub async fn read_to_delimited_chunks_from_stream<''a>(self: &Self, stream: BoxStream<''a, Result<Bytes>>) -> BoxStream<''a, Result<Bytes>> { /* ... */ }
  ```
  Convert a stream of bytes into a stream of of [`Bytes`] containing newline

- ```rust
  pub fn with_options(self: Self, options: CsvOptions) -> Self { /* ... */ }
  ```
  Set the csv options

- ```rust
  pub fn options(self: &Self) -> &CsvOptions { /* ... */ }
  ```
  Retrieve the csv options

- ```rust
  pub fn with_schema_infer_max_rec(self: Self, max_rec: usize) -> Self { /* ... */ }
  ```
  Set a limit in terms of records to scan to infer the schema

- ```rust
  pub fn with_has_header(self: Self, has_header: bool) -> Self { /* ... */ }
  ```
  Set true to indicate that the first line is a header.

- ```rust
  pub fn with_null_regex(self: Self, null_regex: Option<String>) -> Self { /* ... */ }
  ```
  Set the regex to use for null values in the CSV reader.

- ```rust
  pub fn has_header(self: &Self) -> Option<bool> { /* ... */ }
  ```
  Returns `Some(true)` if the first line is a header, `Some(false)` if

- ```rust
  pub fn with_comment(self: Self, comment: Option<u8>) -> Self { /* ... */ }
  ```
  Lines beginning with this byte are ignored.

- ```rust
  pub fn with_delimiter(self: Self, delimiter: u8) -> Self { /* ... */ }
  ```
  The character separating values within a row.

- ```rust
  pub fn with_quote(self: Self, quote: u8) -> Self { /* ... */ }
  ```
  The quote character in a row.

- ```rust
  pub fn with_escape(self: Self, escape: Option<u8>) -> Self { /* ... */ }
  ```
  The escape character in a row.

- ```rust
  pub fn with_terminator(self: Self, terminator: Option<u8>) -> Self { /* ... */ }
  ```
  The character used to indicate the end of a row.

- ```rust
  pub fn with_newlines_in_values(self: Self, newlines_in_values: bool) -> Self { /* ... */ }
  ```
  Specifies whether newlines in (quoted) values are supported.

- ```rust
  pub fn with_file_compression_type(self: Self, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Set a `FileCompressionType` of CSV

- ```rust
  pub fn delimiter(self: &Self) -> u8 { /* ... */ }
  ```
  The delimiter character.

- ```rust
  pub fn quote(self: &Self) -> u8 { /* ... */ }
  ```
  The quote character.

- ```rust
  pub fn escape(self: &Self) -> Option<u8> { /* ... */ }
  ```
  The escape character.

- ```rust
  pub async fn infer_schema_from_stream</* synthetic */ impl Stream<Item = Result<Bytes>>: Stream<Item = Result<Bytes>>>(self: &Self, state: &dyn Session, records_to_read: usize, stream: impl Stream<Item = Result<Bytes>>) -> Result<(Schema, usize)> { /* ... */ }
  ```
  Return the inferred schema reading up to records_to_read from a

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn infer_stats<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, _store: &''life2 Arc<dyn ObjectStore>, table_schema: SchemaRef, _object: &''life3 ObjectMeta) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Statistics>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn create_physical_plan<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, state: &''life1 dyn Session, conf: FileScanConfig, _filters: Option<&''life2 Arc<dyn PhysicalExpr>>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn create_writer_physical_plan<''life0, ''life1, ''async_trait>(self: &''life0 Self, input: Arc<dyn ExecutionPlan>, state: &''life1 dyn Session, conf: FileSinkConfig, order_requirements: Option<LexRequirement>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn file_source(self: &Self) -> Arc<dyn FileSource> { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> CsvFormat { /* ... */ }
    ```

- **IntoEither**
- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
#### Struct `CsvDecoder`

```rust
pub struct CsvDecoder {
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
  pub fn new(decoder: arrow::csv::reader::Decoder) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
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

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Allocation**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `CsvSerializer`

Define a struct for serializing CSV records to a stream

```rust
pub struct CsvSerializer {
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
  Constructor for the CsvSerializer object

- ```rust
  pub fn with_builder(self: Self, builder: WriterBuilder) -> Self { /* ... */ }
  ```
  Method for setting the CSV writer builder

- ```rust
  pub fn with_header(self: Self, header: bool) -> Self { /* ... */ }
  ```
  Method for setting the CSV writer header status

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
- **BatchSerializer**
  - ```rust
    fn serialize(self: &Self, batch: RecordBatch, initial: bool) -> Result<Bytes> { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **Sync**
- **ErasedDestructor**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
#### Struct `CsvSink`

Implements [`DataSink`] for writing to a CSV file.

```rust
pub struct CsvSink {
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
  pub fn new(config: FileSinkConfig, writer_options: CsvWriterOptions) -> Self { /* ... */ }
  ```
  Create from config.

- ```rust
  pub fn writer_options(self: &Self) -> &CsvWriterOptions { /* ... */ }
  ```
  Retrieve the writer options

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
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

- **Freeze**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

## Module `source`

Execution plan for reading CSV files

```rust
pub mod source { /* ... */ }
```

### Types

#### Struct `CsvExec`

**Attributes:**

- `#[deprecated(since = "46.0.0", note = "use DataSourceExec instead")]`

**⚠️ Deprecated since 46.0.0**: use DataSourceExec instead

Old Csv source, deprecated with DataSourceExec implementation and CsvSource

See examples on `CsvSource`

```rust
pub struct CsvExec {
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
  pub fn new(base_config: FileScanConfig, has_header: bool, delimiter: u8, quote: u8, terminator: Option<u8>, escape: Option<u8>, comment: Option<u8>, newlines_in_values: bool, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Create a new CSV reader execution plan provided base and specific configurations

- ```rust
  pub fn builder(file_scan_config: FileScanConfig) -> CsvExecBuilder { /* ... */ }
  ```
  Return a [`CsvExecBuilder`].

- ```rust
  pub fn base_config(self: &Self) -> &FileScanConfig { /* ... */ }
  ```
  Ref to the base configs

- ```rust
  pub fn has_header(self: &Self) -> bool { /* ... */ }
  ```
  true if the first line of each file is a header

- ```rust
  pub fn newlines_in_values(self: &Self) -> bool { /* ... */ }
  ```
  Specifies whether newlines in (quoted) values are supported.

###### Trait Implementations

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

- **Freeze**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CsvExec { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Struct `CsvExecBuilder`

**Attributes:**

- `#[deprecated(since = "46.0.0", note = "use FileScanConfig instead")]`

**⚠️ Deprecated since 46.0.0**: use FileScanConfig instead

Builder for [`CsvExec`].

See example on [`CsvExec`].

```rust
pub struct CsvExecBuilder {
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
  pub fn new(file_scan_config: FileScanConfig) -> Self { /* ... */ }
  ```
  Create a new builder to read the provided file scan configuration.

- ```rust
  pub fn with_has_header(self: Self, has_header: bool) -> Self { /* ... */ }
  ```
  Set whether the first row defines the column names.

- ```rust
  pub fn with_delimeter(self: Self, delimiter: u8) -> Self { /* ... */ }
  ```
  Set the column delimeter.

- ```rust
  pub fn with_quote(self: Self, quote: u8) -> Self { /* ... */ }
  ```
  Set the quote character.

- ```rust
  pub fn with_terminator(self: Self, terminator: Option<u8>) -> Self { /* ... */ }
  ```
  Set the line terminator. If not set, the default is CRLF.

- ```rust
  pub fn with_escape(self: Self, escape: Option<u8>) -> Self { /* ... */ }
  ```
  Set the escape character.

- ```rust
  pub fn with_comment(self: Self, comment: Option<u8>) -> Self { /* ... */ }
  ```
  Set the comment character.

- ```rust
  pub fn with_newlines_in_values(self: Self, newlines_in_values: bool) -> Self { /* ... */ }
  ```
  Set whether newlines in (quoted) values are supported.

- ```rust
  pub fn with_file_compression_type(self: Self, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Set the file compression type.

- ```rust
  pub fn build(self: Self) -> CsvExec { /* ... */ }
  ```
  Build a [`CsvExec`].

###### Trait Implementations

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> CsvExecBuilder { /* ... */ }
    ```

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
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

#### Struct `CsvSource`

A Config for [`CsvOpener`]

# Example: create a `DataSourceExec` for CSV
```
# use std::sync::Arc;
# use arrow::datatypes::Schema;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource_csv::source::CsvSource;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_datasource::source::DataSourceExec;

# let object_store_url = ObjectStoreUrl::local_filesystem();
# let file_schema = Arc::new(Schema::empty());

let source = Arc::new(CsvSource::new(
        true,
        b',',
        b'"',
    )
    .with_terminator(Some(b'#')
));
// Create a DataSourceExec for reading the first 100MB of `file1.csv`
let config = FileScanConfigBuilder::new(object_store_url, file_schema, source)
    .with_file(PartitionedFile::new("file1.csv", 100*1024*1024))
    .with_newlines_in_values(true) // The file contains newlines in values;
    .build();
let exec = (DataSourceExec::from_data_source(config));
```

```rust
pub struct CsvSource {
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
  pub fn new(has_header: bool, delimiter: u8, quote: u8) -> Self { /* ... */ }
  ```
  Returns a [`CsvSource`]

- ```rust
  pub fn has_header(self: &Self) -> bool { /* ... */ }
  ```
  true if the first line of each file is a header

- ```rust
  pub fn delimiter(self: &Self) -> u8 { /* ... */ }
  ```
  A column delimiter

- ```rust
  pub fn quote(self: &Self) -> u8 { /* ... */ }
  ```
  The quote character

- ```rust
  pub fn terminator(self: &Self) -> Option<u8> { /* ... */ }
  ```
  The line terminator

- ```rust
  pub fn comment(self: &Self) -> Option<u8> { /* ... */ }
  ```
  Lines beginning with this byte are ignored.

- ```rust
  pub fn escape(self: &Self) -> Option<u8> { /* ... */ }
  ```
  The escape character

- ```rust
  pub fn with_escape(self: &Self, escape: Option<u8>) -> Self { /* ... */ }
  ```
  Initialize a CsvSource with escape

- ```rust
  pub fn with_terminator(self: &Self, terminator: Option<u8>) -> Self { /* ... */ }
  ```
  Initialize a CsvSource with terminator

- ```rust
  pub fn with_comment(self: &Self, comment: Option<u8>) -> Self { /* ... */ }
  ```
  Initialize a CsvSource with comment

###### Trait Implementations

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> CsvSource { /* ... */ }
    ```

- **Freeze**
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
    fn with_schema(self: &Self, schema: SchemaRef) -> Arc<dyn FileSource> { /* ... */ }
    ```

  - ```rust
    fn with_statistics(self: &Self, statistics: Statistics) -> Arc<dyn FileSource> { /* ... */ }
    ```

  - ```rust
    fn with_projection(self: &Self, config: &FileScanConfig) -> Arc<dyn FileSource> { /* ... */ }
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

  - ```rust
    fn fmt_extra(self: &Self, t: DisplayFormatType, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CsvSource { /* ... */ }
    ```

- **MaybeSendSync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **ErasedDestructor**
#### Struct `CsvOpener`

A [`FileOpener`] that opens a CSV file and yields a [`FileOpenFuture`]

```rust
pub struct CsvOpener {
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
  pub fn new(config: Arc<CsvSource>, file_compression_type: FileCompressionType, object_store: Arc<dyn ObjectStore>) -> Self { /* ... */ }
  ```
  Returns a [`CsvOpener`]

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Unpin**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **FileOpener**
  - ```rust
    fn open(self: &Self, file_meta: FileMeta) -> Result<FileOpenFuture> { /* ... */ }
    ```
    Open a partitioned CSV file.

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
### Functions

#### Function `plan_to_csv`

```rust
pub async fn plan_to_csv</* synthetic */ impl AsRef<str>: AsRef<str>>(task_ctx: std::sync::Arc<datafusion_execution::TaskContext>, plan: std::sync::Arc<dyn ExecutionPlan>, path: impl AsRef<str>) -> datafusion_common::Result<()> { /* ... */ }
```

## Functions

### Function `partitioned_csv_config`

Returns a [`FileScanConfig`] for given `file_groups`

```rust
pub fn partitioned_csv_config(schema: arrow::datatypes::SchemaRef, file_groups: Vec<datafusion_datasource::file_groups::FileGroup>, file_source: std::sync::Arc<dyn FileSource>) -> datafusion_datasource::file_scan_config::FileScanConfig { /* ... */ }
```

## Re-exports

### Re-export `file_format::*`

```rust
pub use file_format::*;
```


# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_datasource`

A table that uses the `ObjectStore` listing capability
to get the list of files to process.

## Modules

## Module `decoder`

Module containing helper methods for the various file formats
See write.rs for write related helper methods

```rust
pub mod decoder { /* ... */ }
```

### Types

#### Enum `DeserializerOutput`

Possible outputs of a [`BatchDeserializer`].

```rust
pub enum DeserializerOutput {
    RecordBatch(::arrow::array::RecordBatch),
    RequiresMoreData,
    InputExhausted,
}
```

##### Variants

###### `RecordBatch`

A successfully deserialized [`RecordBatch`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `::arrow::array::RecordBatch` |  |

###### `RequiresMoreData`

The deserializer requires more data to make progress.

###### `InputExhausted`

The input data has been exhausted.

##### Implementations

###### Trait Implementations

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DeserializerOutput) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **MaybeSendSync**
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

- **ErasedDestructor**
#### Struct `DecoderDeserializer`

A generic, decoder-based deserialization scheme for processing encoded data.

This struct is responsible for converting a stream of bytes, which represent
encoded data, into a stream of `RecordBatch` objects, following the specified
schema and formatting options. It also handles any buffering necessary to satisfy
the `Decoder` interface.

```rust
pub struct DecoderDeserializer<T: Decoder> {
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
  pub fn new(decoder: T) -> Self { /* ... */ }
  ```
  Creates a new `DecoderDeserializer` with the provided decoder.

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **BatchDeserializer**
  - ```rust
    fn digest(self: &mut Self, message: Bytes) -> usize { /* ... */ }
    ```

  - ```rust
    fn next(self: &mut Self) -> Result<DeserializerOutput, ArrowError> { /* ... */ }
    ```

  - ```rust
    fn finish(self: &mut Self) { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **Send**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

### Traits

#### Trait `BatchDeserializer`

Trait defining a scheme for deserializing byte streams into structured data.
Implementors of this trait are responsible for converting raw bytes into
`RecordBatch` objects.

```rust
pub trait BatchDeserializer<T>: Send + fmt::Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `digest`: Feeds a message for deserialization, updating the internal state of
- `next`: Attempts to deserialize any pending messages and returns a
- `finish`: Informs the deserializer that no more messages will be provided for

##### Implementations

This trait is implemented for the following types:

- `DecoderDeserializer<T>` with <T: Decoder>

#### Trait `Decoder`

A general interface for decoders such as [`arrow::json::reader::Decoder`] and
[`arrow::csv::reader::Decoder`]. Defines an interface similar to
[`Decoder::decode`] and [`Decoder::flush`] methods, but also includes
a method to check if the decoder can flush early. Intended to be used in
conjunction with [`DecoderDeserializer`].

[`arrow::json::reader::Decoder`]: ::arrow::json::reader::Decoder
[`arrow::csv::reader::Decoder`]: ::arrow::csv::reader::Decoder
[`Decoder::decode`]: ::arrow::json::reader::Decoder::decode
[`Decoder::flush`]: ::arrow::json::reader::Decoder::flush

```rust
pub trait Decoder: Send + fmt::Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `decode`: See [`arrow::json::reader::Decoder::decode`].
- `flush`: See [`arrow::json::reader::Decoder::flush`].
- `can_flush_early`: Whether the decoder can flush early in its current state.

### Functions

#### Function `deserialize_stream`

Deserializes a stream of bytes into a stream of [`RecordBatch`] objects using the
provided deserializer.

Returns a boxed stream of `Result<RecordBatch, ArrowError>`. The stream yields [`RecordBatch`]
objects as they are produced by the deserializer, or an [`ArrowError`] if an error
occurs while polling the input or deserializing.

```rust
pub fn deserialize_stream<''a, /* synthetic */ impl Stream<Item = Result<Bytes>> + Unpin + Send + 'a: Stream<Item = datafusion_common::Result<bytes::Bytes>> + Unpin + Send + ''a, /* synthetic */ impl BatchDeserializer<Bytes> + 'a: BatchDeserializer<bytes::Bytes> + ''a>(input: impl Stream<Item = datafusion_common::Result<bytes::Bytes>> + Unpin + Send + ''a, deserializer: impl BatchDeserializer<bytes::Bytes> + ''a) -> futures::stream::BoxStream<''a, datafusion_common::Result<::arrow::array::RecordBatch, arrow::error::ArrowError>> { /* ... */ }
```

## Module `display`

```rust
pub mod display { /* ... */ }
```

### Types

#### Struct `FileGroupDisplay`

A wrapper to customize partitioned group of files display

Prints in the format:
```text
[file1, file2,...]
```

```rust
pub struct FileGroupDisplay<''a>(pub &''a crate::file_groups::FileGroup);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a crate::file_groups::FileGroup` |  |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Unpin**
- **Sync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut Formatter<''_>) -> FmtResult { /* ... */ }
    ```

## Module `file`

Common behaviors that every file format needs to implement

```rust
pub mod file { /* ... */ }
```

### Traits

#### Trait `FileSource`

Common file format behaviors needs to implement.

See implementation examples such as `ParquetSource`, `CsvSource`

```rust
pub trait FileSource: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `create_file_opener`: Creates a `dyn FileOpener` based on given parameters
- `as_any`: Any
- `with_batch_size`: Initialize new type with batch size configuration
- `with_schema`: Initialize new instance with a new schema
- `with_projection`: Initialize new instance with projection information
- `with_statistics`: Initialize new instance with projected statistics
- `metrics`: Return execution plan metrics
- `statistics`: Return projected statistics
- `file_type`: String representation of file source such as "csv", "json", "parquet"

##### Provided Methods

- ```rust
  fn fmt_extra(self: &Self, _t: DisplayFormatType, _f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
  ```
  Format FileType specific information

- ```rust
  fn repartitioned(self: &Self, target_partitions: usize, repartition_file_min_size: usize, output_ordering: Option<LexOrdering>, config: &FileScanConfig) -> datafusion_common::Result<Option<FileScanConfig>> { /* ... */ }
  ```
  If supported by the [`FileSource`], redistribute files across partitions according to their size.

## Module `file_compression_type`

File Compression type abstraction

```rust
pub mod file_compression_type { /* ... */ }
```

### Types

#### Struct `FileCompressionType`

Readable file compression type

```rust
pub struct FileCompressionType {
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
  pub fn get_variant(self: &Self) -> &CompressionTypeVariant { /* ... */ }
  ```
  Read only access to self.variant

- ```rust
  pub const fn is_compressed(self: &Self) -> bool { /* ... */ }
  ```
  The file is compressed or not

- ```rust
  pub fn convert_to_compress_stream<''a>(self: &Self, s: BoxStream<''a, Result<Bytes>>) -> Result<BoxStream<''a, Result<Bytes>>> { /* ... */ }
  ```
  Given a `Stream`, create a `Stream` which data are compressed with `FileCompressionType`.

- ```rust
  pub fn convert_async_writer(self: &Self, w: BufWriter) -> Result<Box<dyn AsyncWrite + Send + Unpin>> { /* ... */ }
  ```
  Wrap the given `BufWriter` so that it performs compressed writes

- ```rust
  pub fn convert_stream<''a>(self: &Self, s: BoxStream<''a, Result<Bytes>>) -> Result<BoxStream<''a, Result<Bytes>>> { /* ... */ }
  ```
  Given a `Stream`, create a `Stream` which data are decompressed with `FileCompressionType`.

- ```rust
  pub fn convert_read<T: std::io::Read + Send + ''static>(self: &Self, r: T) -> Result<Box<dyn std::io::Read + Send>> { /* ... */ }
  ```
  Given a `Read`, create a `Read` which data are decompressed with `FileCompressionType`.

###### Trait Implementations

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> Result<Self> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **GetExt**
  - ```rust
    fn get_ext(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

  - ```rust
    fn from(t: CompressionTypeVariant) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(t: FileCompressionType) -> Self { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Eq**
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

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FileCompressionType { /* ... */ }
    ```

- **Allocation**
- **ErasedDestructor**
- **Copy**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FileCompressionType) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
### Traits

#### Trait `FileTypeExt`

Trait for extending the functionality of the `FileType` enum.

```rust
pub trait FileTypeExt {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_ext_with_compression`: Given a `FileCompressionType`, return the `FileType`'s extension with compression suffix

## Module `file_format`

Module containing helper methods for the various file formats
See write.rs for write related helper methods

```rust
pub mod file_format { /* ... */ }
```

### Types

#### Enum `FilePushdownSupport`

An enum to distinguish between different states when determining if certain filters can be
pushed down to file scanning

```rust
pub enum FilePushdownSupport {
    NoSupport,
    NotSupportedForFilter,
    Supported,
}
```

##### Variants

###### `NoSupport`

The file format/system being asked does not support any sort of pushdown. This should be
used even if the file format theoretically supports some sort of pushdown, but it's not
enabled or implemented yet.

###### `NotSupportedForFilter`

The file format/system being asked *does* support pushdown, but it can't make it work for
the provided filter/expression

###### `Supported`

The file format/system being asked *does* support pushdown and *can* make it work for the
provided filter/expression

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Allocation**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FilePushdownSupport) -> bool { /* ... */ }
    ```

- **IntoEither**
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

#### Struct `DefaultFileType`

A container of [FileFormatFactory] which also implements [FileType].
This enables converting a dyn FileFormat to a dyn FileType.
The former trait is a superset of the latter trait, which includes execution time
relevant methods. [FileType] is only used in logical planning and only implements
the subset of methods required during logical planning.

```rust
pub struct DefaultFileType {
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
  pub fn new(file_format_factory: Arc<dyn FileFormatFactory>) -> Self { /* ... */ }
  ```
  Constructs a [DefaultFileType] wrapper from a [FileFormatFactory]

- ```rust
  pub fn as_format_factory(self: &Self) -> &Arc<dyn FileFormatFactory> { /* ... */ }
  ```
  get a reference to the inner [FileFormatFactory] struct

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **MaybeSendSync**
- **GetExt**
  - ```rust
    fn get_ext(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **FileType**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

- **Send**
- **Unpin**
### Traits

#### Trait `FileFormat`

This trait abstracts all the file format specific implementations
from the [`TableProvider`]. This helps code re-utilization across
providers that support the same file formats.

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

```rust
pub trait FileFormat: Send + Sync + fmt::Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_any`: Returns the table provider as [`Any`](std::any::Any) so that it can be
- `get_ext`: Returns the extension for this FileFormat, e.g. "file.csv" -> csv
- `get_ext_with_compression`: Returns the extension for this FileFormat when compressed, e.g. "file.csv.gz" -> csv
- `infer_schema`: Infer the common schema of the provided objects. The objects will usually
- `infer_stats`: Infer the statistics for the provided object. The cost and accuracy of the
- `create_physical_plan`: Take a list of files and convert it to the appropriate executor
- `file_source`: Return the related FileSource such as `CsvSource`, `JsonSource`, etc.

##### Provided Methods

- ```rust
  fn create_writer_physical_plan<''life0, ''life1, ''async_trait>(self: &''life0 Self, _input: Arc<dyn ExecutionPlan>, _state: &''life1 dyn Session, _conf: FileSinkConfig, _order_requirements: Option<LexRequirement>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
  ```
  Take a list of files and the configuration to convert it to the

- ```rust
  fn supports_filters_pushdown(self: &Self, _file_schema: &Schema, _table_schema: &Schema, _filters: &[&Expr]) -> Result<FilePushdownSupport> { /* ... */ }
  ```
  Check if the specified file format has support for pushing down the provided filters within

#### Trait `FileFormatFactory`

Factory for creating [`FileFormat`] instances based on session and command level options

Users can provide their own `FileFormatFactory` to support arbitrary file formats

```rust
pub trait FileFormatFactory: Sync + Send + GetExt + fmt::Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `create`: Initialize a [FileFormat] and configure based on session and command level options
- `default`: Initialize a [FileFormat] with all options set to default values
- `as_any`: Returns the table source as [`Any`] so that it can be

### Functions

#### Function `format_as_file_type`

Converts a [FileFormatFactory] to a [FileType]

```rust
pub fn format_as_file_type(file_format_factory: std::sync::Arc<dyn FileFormatFactory>) -> std::sync::Arc<dyn FileType> { /* ... */ }
```

#### Function `file_type_to_format`

Converts a [FileType] to a [FileFormatFactory].
Returns an error if the [FileType] cannot be
downcasted to a [DefaultFileType].

```rust
pub fn file_type_to_format(file_type: &std::sync::Arc<dyn FileType>) -> datafusion_common::Result<std::sync::Arc<dyn FileFormatFactory>> { /* ... */ }
```

### Constants and Statics

#### Constant `DEFAULT_SCHEMA_INFER_MAX_RECORD`

Default max records to scan to infer the schema

```rust
pub const DEFAULT_SCHEMA_INFER_MAX_RECORD: usize = 1000;
```

## Module `file_groups`

Logic for managing groups of [`PartitionedFile`]s in DataFusion

```rust
pub mod file_groups { /* ... */ }
```

### Types

#### Struct `FileGroupPartitioner`

Repartition input files into `target_partitions` partitions, if total file size exceed
`repartition_file_min_size`

This partitions evenly by file byte range, and does not have any knowledge
of how data is laid out in specific files. The specific `FileOpener` are
responsible for the actual partitioning on specific data source type. (e.g.
the `CsvOpener` will read lines overlap with byte range as well as
handle boundaries to ensure all lines will be read exactly once)

# Example

For example, if there are two files `A` and `B` that we wish to read with 4
partitions (with 4 threads) they will be divided as follows:

```text
                                   ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                     ┌─────────────────┐
                                   │ │                 │ │
                                     │     File A      │
                                   │ │  Range: 0-2MB   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┌─────────────────┐                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │   Range 2-4MB   │ │
│                 │                  │                 │
│                 │                │ └─────────────────┘ │
│  File A (7MB)   │   ────────▶     ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │  Range: 4-6MB   │ │
│                 │                  │                 │
│                 │                │ └─────────────────┘ │
└─────────────────┘                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
┌─────────────────┐                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│  File B (1MB)   │                  ┌─────────────────┐
│                 │                │ │     File A      │ │
└─────────────────┘                  │  Range: 6-7MB   │
                                   │ └─────────────────┘ │
                                     ┌─────────────────┐
                                   │ │  File B (1MB)   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

                                   If target_partitions = 4,
                                     divides into 4 groups
```

# Maintaining Order

Within each group files are read sequentially. Thus, if the overall order of
tuples must be preserved, multiple files can not be mixed in the same group.

In this case, the code will split the largest files evenly into any
available empty groups, but the overall distribution may not be as even
as if the order did not need to be preserved.

```text
                                  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
                                     ┌─────────────────┐
                                   │ │                 │ │
                                     │     File A      │
                                   │ │  Range: 0-2MB   │ │
                                     │                 │
┌─────────────────┐                │ └─────────────────┘ │
│                 │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │   Range 2-4MB   │ │
│  File A (6MB)   │   ────────▶      │                 │
│    (ordered)    │                │ └─────────────────┘ │
│                 │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│                 │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
│                 │                  ┌─────────────────┐
│                 │                │ │                 │ │
│                 │                  │     File A      │
│                 │                │ │  Range: 4-6MB   │ │
└─────────────────┘                  │                 │
┌─────────────────┐                │ └─────────────────┘ │
│  File B (1MB)   │                 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
│    (ordered)    │                ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┐
└─────────────────┘                  ┌─────────────────┐
                                   │ │  File B (1MB)   │ │
                                     │                 │
                                   │ └─────────────────┘ │
                                    ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─

                                   If target_partitions = 4,
                                     divides into 4 groups
```

```rust
pub struct FileGroupPartitioner {
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
  Creates a new [`FileGroupPartitioner`] with default values:

- ```rust
  pub fn with_target_partitions(self: Self, target_partitions: usize) -> Self { /* ... */ }
  ```
  Set the target partitions

- ```rust
  pub fn with_repartition_file_min_size(self: Self, repartition_file_min_size: usize) -> Self { /* ... */ }
  ```
  Set the minimum size at which to repartition a file

- ```rust
  pub fn with_preserve_order_within_groups(self: Self, preserve_order_within_groups: bool) -> Self { /* ... */ }
  ```
  Set whether the order of tuples within a file must be preserved

- ```rust
  pub fn repartition_file_groups(self: &Self, file_groups: &[FileGroup]) -> Option<Vec<FileGroup>> { /* ... */ }
  ```
  Repartition input files according to the settings on this [`FileGroupPartitioner`].

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **IntoEither**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FileGroupPartitioner { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `FileGroup`

Represents a group of partitioned files that'll be processed by a single thread.
Maintains optional statistics across all files in the group.

```rust
pub struct FileGroup {
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
  pub fn new(files: Vec<PartitionedFile>) -> Self { /* ... */ }
  ```
  Creates a new FileGroup from a vector of PartitionedFile objects

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of files in this group

- ```rust
  pub fn with_statistics(self: Self, statistics: Arc<Statistics>) -> Self { /* ... */ }
  ```
  Set the statistics for this group

- ```rust
  pub fn files(self: &Self) -> &[PartitionedFile] { /* ... */ }
  ```
  Returns a slice of the files in this group

- ```rust
  pub fn iter(self: &Self) -> impl Iterator<Item = &PartitionedFile> { /* ... */ }
  ```

- ```rust
  pub fn into_inner(self: Self) -> Vec<PartitionedFile> { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn pop(self: &mut Self) -> Option<PartitionedFile> { /* ... */ }
  ```
  Removes the last element from the files vector and returns it, or None if empty

- ```rust
  pub fn push(self: &mut Self, file: PartitionedFile) { /* ... */ }
  ```
  Adds a file to the group

- ```rust
  pub fn statistics(self: &Self) -> Option<&Statistics> { /* ... */ }
  ```
  Get the statistics for this group

- ```rust
  pub fn split_files(self: Self, n: usize) -> Vec<FileGroup> { /* ... */ }
  ```
  Partition the list of files into `n` groups

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **Freeze**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(files: Vec<PartitionedFile>) -> Self { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FileGroup { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &<Self as >::Output { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **IndexMut**
  - ```rust
    fn index_mut(self: &mut Self, index: usize) -> &mut <Self as >::Output { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = PartitionedFile>>(iter: I) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
## Module `file_meta`

```rust
pub mod file_meta { /* ... */ }
```

### Types

#### Struct `FileMeta`

A single file or part of a file that should be read, along with its schema, statistics

```rust
pub struct FileMeta {
    pub object_meta: object_store::ObjectMeta,
    pub range: Option<crate::FileRange>,
    pub extensions: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub metadata_size_hint: Option<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `object_meta` | `object_store::ObjectMeta` | Path for the file (e.g. URL, filesystem path, etc) |
| `range` | `Option<crate::FileRange>` | An optional file range for a more fine-grained parallel execution |
| `extensions` | `Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>` | An optional field for user defined per object metadata |
| `metadata_size_hint` | `Option<usize>` | Size hint for the metadata of this file |

##### Implementations

###### Methods

- ```rust
  pub fn location(self: &Self) -> &Path { /* ... */ }
  ```
  The full path to the object

###### Trait Implementations

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **ErasedDestructor**
- **IntoEither**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(object_meta: ObjectMeta) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `file_scan_config`

[`FileScanConfig`] to configure scanning of possibly partitioned
file sources.

```rust
pub mod file_scan_config { /* ... */ }
```

### Types

#### Struct `FileScanConfig`

The base configurations for a [`DataSourceExec`], the a physical plan for
any given file format.

Use [`Self::build`] to create a [`DataSourceExec`] from a ``FileScanConfig`.

# Example
```
# use std::any::Any;
# use std::sync::Arc;
# use arrow::datatypes::{Field, Fields, DataType, Schema, SchemaRef};
# use object_store::ObjectStore;
# use datafusion_common::Statistics;
# use datafusion_datasource::file::FileSource;
# use datafusion_datasource::file_groups::FileGroup;
# use datafusion_datasource::PartitionedFile;
# use datafusion_datasource::file_scan_config::{FileScanConfig, FileScanConfigBuilder};
# use datafusion_datasource::file_stream::FileOpener;
# use datafusion_datasource::source::DataSourceExec;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_physical_plan::ExecutionPlan;
# use datafusion_physical_plan::metrics::ExecutionPlanMetricsSet;
# let file_schema = Arc::new(Schema::new(vec![
#  Field::new("c1", DataType::Int32, false),
#  Field::new("c2", DataType::Int32, false),
#  Field::new("c3", DataType::Int32, false),
#  Field::new("c4", DataType::Int32, false),
# ]));
# // Note: crate mock ParquetSource, as ParquetSource is not in the datasource crate
# struct ParquetSource {
#    projected_statistics: Option<Statistics>
# };
# impl FileSource for ParquetSource {
#  fn create_file_opener(&self, _: Arc<dyn ObjectStore>, _: &FileScanConfig, _: usize) -> Arc<dyn FileOpener> { unimplemented!() }
#  fn as_any(&self) -> &dyn Any { self  }
#  fn with_batch_size(&self, _: usize) -> Arc<dyn FileSource> { unimplemented!() }
#  fn with_schema(&self, _: SchemaRef) -> Arc<dyn FileSource> { unimplemented!() }
#  fn with_projection(&self, _: &FileScanConfig) -> Arc<dyn FileSource> { unimplemented!() }
#  fn with_statistics(&self, statistics: Statistics) -> Arc<dyn FileSource> { Arc::new(Self {projected_statistics: Some(statistics)} ) }
#  fn metrics(&self) -> &ExecutionPlanMetricsSet { unimplemented!() }
#  fn statistics(&self) -> datafusion_common::Result<Statistics> { Ok(self.projected_statistics.clone().expect("projected_statistics should be set")) }
#  fn file_type(&self) -> &str { "parquet" }
#  }
# impl ParquetSource {
#  fn new() -> Self { Self {projected_statistics: None} }
# }
// create FileScan config for reading parquet files from file://
let object_store_url = ObjectStoreUrl::local_filesystem();
let file_source = Arc::new(ParquetSource::new());
let config = FileScanConfigBuilder::new(object_store_url, file_schema, file_source)
  .with_limit(Some(1000))            // read only the first 1000 records
  .with_projection(Some(vec![2, 3])) // project columns 2 and 3
   // Read /tmp/file1.parquet with known size of 1234 bytes in a single group
  .with_file(PartitionedFile::new("file1.parquet", 1234))
  // Read /tmp/file2.parquet 56 bytes and /tmp/file3.parquet 78 bytes
  // in a  single row group
  .with_file_group(FileGroup::new(vec![
   PartitionedFile::new("file2.parquet", 56),
   PartitionedFile::new("file3.parquet", 78),
  ])).build();
// create an execution plan from the config
let plan: Arc<dyn ExecutionPlan> = DataSourceExec::from_data_source(config);
```

```rust
pub struct FileScanConfig {
    pub object_store_url: datafusion_execution::object_store::ObjectStoreUrl,
    pub file_schema: arrow::datatypes::SchemaRef,
    pub file_groups: Vec<crate::file_groups::FileGroup>,
    pub constraints: datafusion_common::Constraints,
    pub projection: Option<Vec<usize>>,
    pub limit: Option<usize>,
    pub table_partition_cols: Vec<arrow::datatypes::Field>,
    pub output_ordering: Vec<datafusion_physical_expr::LexOrdering>,
    pub file_compression_type: crate::file_compression_type::FileCompressionType,
    pub new_lines_in_values: bool,
    pub file_source: std::sync::Arc<dyn FileSource>,
    pub batch_size: Option<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `object_store_url` | `datafusion_execution::object_store::ObjectStoreUrl` | Object store URL, used to get an [`ObjectStore`] instance from<br>[`RuntimeEnv::object_store`]<br><br>This `ObjectStoreUrl` should be the prefix of the absolute url for files<br>as `file://` or `s3://my_bucket`. It should not include the path to the<br>file itself. The relevant URL prefix must be registered via<br>[`RuntimeEnv::register_object_store`]<br><br>[`ObjectStore`]: object_store::ObjectStore<br>[`RuntimeEnv::register_object_store`]: datafusion_execution::runtime_env::RuntimeEnv::register_object_store<br>[`RuntimeEnv::object_store`]: datafusion_execution::runtime_env::RuntimeEnv::object_store |
| `file_schema` | `arrow::datatypes::SchemaRef` | Schema before `projection` is applied. It contains the all columns that may<br>appear in the files. It does not include table partition columns<br>that may be added.<br>Note that this is **not** the schema of the physical files.<br>This is the schema that the physical file schema will be<br>mapped onto, and the schema that the [`DataSourceExec`] will return. |
| `file_groups` | `Vec<crate::file_groups::FileGroup>` | List of files to be processed, grouped into partitions<br><br>Each file must have a schema of `file_schema` or a subset. If<br>a particular file has a subset, the missing columns are<br>padded with NULLs.<br><br>DataFusion may attempt to read each partition of files<br>concurrently, however files *within* a partition will be read<br>sequentially, one after the next. |
| `constraints` | `datafusion_common::Constraints` | Table constraints |
| `projection` | `Option<Vec<usize>>` | Columns on which to project the data. Indexes that are higher than the<br>number of columns of `file_schema` refer to `table_partition_cols`. |
| `limit` | `Option<usize>` | The maximum number of records to read from this plan. If `None`,<br>all records after filtering are returned. |
| `table_partition_cols` | `Vec<arrow::datatypes::Field>` | The partitioning columns |
| `output_ordering` | `Vec<datafusion_physical_expr::LexOrdering>` | All equivalent lexicographical orderings that describe the schema. |
| `file_compression_type` | `crate::file_compression_type::FileCompressionType` | File compression type |
| `new_lines_in_values` | `bool` | Are new lines in values supported for CSVOptions |
| `file_source` | `std::sync::Arc<dyn FileSource>` | File source such as `ParquetSource`, `CsvSource`, `JsonSource`, etc. |
| `batch_size` | `Option<usize>` | Batch size while creating new batches<br>Defaults to [`datafusion_common::config::ExecutionOptions`] batch_size. |

##### Implementations

###### Methods

- ```rust
  pub fn new(object_store_url: ObjectStoreUrl, file_schema: SchemaRef, file_source: Arc<dyn FileSource>) -> Self { /* ... */ }
  ```
  Create a new [`FileScanConfig`] with default settings for scanning files.

- ```rust
  pub fn with_source(self: Self, file_source: Arc<dyn FileSource>) -> Self { /* ... */ }
  ```
  Set the file source

- ```rust
  pub fn with_constraints(self: Self, constraints: Constraints) -> Self { /* ... */ }
  ```
  Set the table constraints of the files

- ```rust
  pub fn with_statistics(self: Self, statistics: Statistics) -> Self { /* ... */ }
  ```
  Set the statistics of the files

- ```rust
  pub fn projected_stats(self: &Self) -> Statistics { /* ... */ }
  ```

- ```rust
  pub fn projected_schema(self: &Self) -> Arc<Schema> { /* ... */ }
  ```

- ```rust
  pub fn projected_constraints(self: &Self) -> Constraints { /* ... */ }
  ```

- ```rust
  pub fn with_projection(self: Self, projection: Option<Vec<usize>>) -> Self { /* ... */ }
  ```
  Set the projection of the files

- ```rust
  pub fn with_limit(self: Self, limit: Option<usize>) -> Self { /* ... */ }
  ```
  Set the limit of the files

- ```rust
  pub fn with_file(self: Self, file: PartitionedFile) -> Self { /* ... */ }
  ```
  Add a file as a single group

- ```rust
  pub fn with_file_groups(self: Self, file_groups: Vec<FileGroup>) -> Self { /* ... */ }
  ```
  Add the file groups

- ```rust
  pub fn with_file_group(self: Self, file_group: FileGroup) -> Self { /* ... */ }
  ```
  Add a new file group

- ```rust
  pub fn with_table_partition_cols(self: Self, table_partition_cols: Vec<Field>) -> Self { /* ... */ }
  ```
  Set the partitioning columns of the files

- ```rust
  pub fn with_output_ordering(self: Self, output_ordering: Vec<LexOrdering>) -> Self { /* ... */ }
  ```
  Set the output ordering of the files

- ```rust
  pub fn with_file_compression_type(self: Self, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Set the file compression type

- ```rust
  pub fn with_newlines_in_values(self: Self, new_lines_in_values: bool) -> Self { /* ... */ }
  ```
  Set the new_lines_in_values property

- ```rust
  pub fn with_batch_size(self: Self, batch_size: Option<usize>) -> Self { /* ... */ }
  ```
  Set the batch_size property

- ```rust
  pub fn newlines_in_values(self: &Self) -> bool { /* ... */ }
  ```
  Specifies whether newlines in (quoted) values are supported.

- ```rust
  pub fn project(self: &Self) -> (SchemaRef, Constraints, Statistics, Vec<LexOrdering>) { /* ... */ }
  ```
  Project the schema, constraints, and the statistics on the given column indices

- ```rust
  pub fn projected_file_column_names(self: &Self) -> Option<Vec<String>> { /* ... */ }
  ```

- ```rust
  pub fn projected_file_schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  Projects only file schema, ignoring partition columns

- ```rust
  pub fn file_column_projection_indices(self: &Self) -> Option<Vec<usize>> { /* ... */ }
  ```

- ```rust
  pub fn split_groups_by_statistics_with_target_partitions(table_schema: &SchemaRef, file_groups: &[FileGroup], sort_order: &LexOrdering, target_partitions: usize) -> Result<Vec<FileGroup>> { /* ... */ }
  ```
  Splits file groups into new groups based on statistics to enable efficient parallel processing.

- ```rust
  pub fn split_groups_by_statistics(table_schema: &SchemaRef, file_groups: &[FileGroup], sort_order: &LexOrdering) -> Result<Vec<FileGroup>> { /* ... */ }
  ```
  Attempts to do a bin-packing on files into file groups, such that any two files

- ```rust
  pub fn build(self: Self) -> Arc<DataSourceExec> { /* ... */ }
  ```
  Returns a new [`DataSourceExec`] to scan the files specified by this config

- ```rust
  pub fn file_source(self: &Self) -> &Arc<dyn FileSource> { /* ... */ }
  ```
  Returns the file_source

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> FmtResult { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(config: FileScanConfig) -> Self { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **DataSource**
  - ```rust
    fn open(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut Formatter<''_>) -> FmtResult { /* ... */ }
    ```

  - ```rust
    fn repartitioned(self: &Self, target_partitions: usize, repartition_file_min_size: usize, output_ordering: Option<LexOrdering>) -> Result<Option<Arc<dyn DataSource>>> { /* ... */ }
    ```
    If supported by the underlying [`FileSource`], redistribute files across partitions according to their size.

  - ```rust
    fn output_partitioning(self: &Self) -> Partitioning { /* ... */ }
    ```

  - ```rust
    fn eq_properties(self: &Self) -> EquivalenceProperties { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn DataSource>> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> ExecutionPlanMetricsSet { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut Formatter<''_>) -> FmtResult { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FileScanConfig { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `FileScanConfigBuilder`

A builder for [`FileScanConfig`]'s.

Example:

```rust
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_datasource::file_scan_config::{FileScanConfigBuilder, FileScanConfig};
# use datafusion_datasource::file_compression_type::FileCompressionType;
# use datafusion_datasource::file_groups::FileGroup;
# use datafusion_datasource::PartitionedFile;
# use datafusion_execution::object_store::ObjectStoreUrl;
# use datafusion_common::Statistics;
# use datafusion_datasource::file::FileSource;

# fn main() {
# fn with_source(file_source: Arc<dyn FileSource>) {
    // Create a schema for our Parquet files
    let schema = Arc::new(Schema::new(vec![
        Field::new("id", DataType::Int32, false),
        Field::new("value", DataType::Utf8, false),
    ]));

    // Create a builder for scanning Parquet files from a local filesystem
    let config = FileScanConfigBuilder::new(
        ObjectStoreUrl::local_filesystem(),
        schema,
        file_source,
    )
    // Set a limit of 1000 rows
    .with_limit(Some(1000))
    // Project only the first column
    .with_projection(Some(vec![0]))
    // Add partition columns
    .with_table_partition_cols(vec![
        Field::new("date", DataType::Utf8, false),
    ])
    // Add a file group with two files
    .with_file_group(FileGroup::new(vec![
        PartitionedFile::new("data/date=2024-01-01/file1.parquet", 1024),
        PartitionedFile::new("data/date=2024-01-01/file2.parquet", 2048),
    ]))
    // Set compression type
    .with_file_compression_type(FileCompressionType::UNCOMPRESSED)
    // Build the final config
    .build();
# }
# }
```

```rust
pub struct FileScanConfigBuilder {
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
  pub fn new(object_store_url: ObjectStoreUrl, file_schema: SchemaRef, file_source: Arc<dyn FileSource>) -> Self { /* ... */ }
  ```
  Create a new [`FileScanConfigBuilder`] with default settings for scanning files.

- ```rust
  pub fn with_limit(self: Self, limit: Option<usize>) -> Self { /* ... */ }
  ```
  Set the maximum number of records to read from this plan. If `None`,

- ```rust
  pub fn with_source(self: Self, file_source: Arc<dyn FileSource>) -> Self { /* ... */ }
  ```
  Set the file source for scanning files.

- ```rust
  pub fn with_projection(self: Self, projection: Option<Vec<usize>>) -> Self { /* ... */ }
  ```
  Set the columns on which to project the data. Indexes that are higher than the

- ```rust
  pub fn with_table_partition_cols(self: Self, table_partition_cols: Vec<Field>) -> Self { /* ... */ }
  ```
  Set the partitioning columns

- ```rust
  pub fn with_constraints(self: Self, constraints: Constraints) -> Self { /* ... */ }
  ```
  Set the table constraints

- ```rust
  pub fn with_statistics(self: Self, statistics: Statistics) -> Self { /* ... */ }
  ```
  Set the estimated overall statistics of the files, taking `filters` into account.

- ```rust
  pub fn with_file_groups(self: Self, file_groups: Vec<FileGroup>) -> Self { /* ... */ }
  ```
  Set the list of files to be processed, grouped into partitions.

- ```rust
  pub fn with_file_group(self: Self, file_group: FileGroup) -> Self { /* ... */ }
  ```
  Add a new file group

- ```rust
  pub fn with_file(self: Self, file: PartitionedFile) -> Self { /* ... */ }
  ```
  Add a file as a single group

- ```rust
  pub fn with_output_ordering(self: Self, output_ordering: Vec<LexOrdering>) -> Self { /* ... */ }
  ```
  Set the output ordering of the files

- ```rust
  pub fn with_file_compression_type(self: Self, file_compression_type: FileCompressionType) -> Self { /* ... */ }
  ```
  Set the file compression type

- ```rust
  pub fn with_newlines_in_values(self: Self, new_lines_in_values: bool) -> Self { /* ... */ }
  ```
  Set whether new lines in values are supported for CSVOptions

- ```rust
  pub fn with_batch_size(self: Self, batch_size: Option<usize>) -> Self { /* ... */ }
  ```
  Set the batch_size property

- ```rust
  pub fn build(self: Self) -> FileScanConfig { /* ... */ }
  ```
  Build the final [`FileScanConfig`] with all the configured settings.

###### Trait Implementations

- **ErasedDestructor**
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

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(config: FileScanConfig) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
    fn clone(self: &Self) -> FileScanConfigBuilder { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `PartitionColumnProjector`

A helper that projects partition columns into the file record batches.

One interesting trick is the usage of a cache for the key buffers of the partition column
dictionaries. Indeed, the partition columns are constant, so the dictionaries that represent them
have all their keys equal to 0. This enables us to re-use the same "all-zero" buffer across batches,
which makes the space consumption of the partition columns O(batch_size) instead of O(record_count).

```rust
pub struct PartitionColumnProjector {
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
  pub fn new(projected_schema: SchemaRef, table_partition_cols: &[String]) -> Self { /* ... */ }
  ```

- ```rust
  pub fn project(self: &mut Self, file_batch: RecordBatch, partition_values: &[ScalarValue]) -> Result<RecordBatch> { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **Sync**
- **RefUnwindSafe**
- **Unpin**
- **MaybeSendSync**
- **UnwindSafe**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `wrap_partition_type_in_dict`

Convert type to a type suitable for use as a `ListingTable`
partition column. Returns `Dictionary(UInt16, val_type)`, which is
a reasonable trade off between a reasonable number of partition
values and space efficiency.

This use this to specify types for partition columns. However
you MAY also choose not to dictionary-encode the data or to use a
different dictionary type.

Use [`wrap_partition_value_in_dict`] to wrap a [`ScalarValue`] in the same say.

```rust
pub fn wrap_partition_type_in_dict(val_type: arrow::datatypes::DataType) -> arrow::datatypes::DataType { /* ... */ }
```

#### Function `wrap_partition_value_in_dict`

Convert a [`ScalarValue`] of partition columns to a type, as
described in the documentation of [`wrap_partition_type_in_dict`],
which can wrap the types.

```rust
pub fn wrap_partition_value_in_dict(val: datafusion_common::ScalarValue) -> datafusion_common::ScalarValue { /* ... */ }
```

## Module `file_sink_config`

```rust
pub mod file_sink_config { /* ... */ }
```

### Types

#### Struct `FileSinkConfig`

The base configurations to provide when creating a physical plan for
writing to any given file format.

```rust
pub struct FileSinkConfig {
    pub original_url: String,
    pub object_store_url: datafusion_execution::object_store::ObjectStoreUrl,
    pub file_group: crate::file_groups::FileGroup,
    pub table_paths: Vec<crate::ListingTableUrl>,
    pub output_schema: arrow::datatypes::SchemaRef,
    pub table_partition_cols: Vec<(String, arrow::datatypes::DataType)>,
    pub insert_op: datafusion_expr::dml::InsertOp,
    pub keep_partition_by_columns: bool,
    pub file_extension: String,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `original_url` | `String` | The unresolved URL specified by the user |
| `object_store_url` | `datafusion_execution::object_store::ObjectStoreUrl` | Object store URL, used to get an ObjectStore instance |
| `file_group` | `crate::file_groups::FileGroup` | A collection of files organized into groups.<br>Each FileGroup contains one or more PartitionedFile objects. |
| `table_paths` | `Vec<crate::ListingTableUrl>` | Vector of partition paths |
| `output_schema` | `arrow::datatypes::SchemaRef` | The schema of the output file |
| `table_partition_cols` | `Vec<(String, arrow::datatypes::DataType)>` | A vector of column names and their corresponding data types,<br>representing the partitioning columns for the file |
| `insert_op` | `datafusion_expr::dml::InsertOp` | Controls how new data should be written to the file, determining whether<br>to append to, overwrite, or replace records in existing files. |
| `keep_partition_by_columns` | `bool` | Controls whether partition columns are kept for the file |
| `file_extension` | `String` | File extension without a dot(.) |

##### Implementations

###### Methods

- ```rust
  pub fn output_schema(self: &Self) -> &SchemaRef { /* ... */ }
  ```
  Get output schema

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Send**
- **MaybeSendSync**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FileSinkConfig { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
### Traits

#### Trait `FileSink`

General behaviors for files that do `DataSink` operations

```rust
pub trait FileSink: DataSink {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `config`: Retrieves the file sink configuration.
- `spawn_writer_tasks_and_join`: Spawns writer tasks and joins them to perform file writing operations.

##### Provided Methods

- ```rust
  fn write_all<''life0, ''life1, ''async_trait>(self: &''life0 Self, data: SendableRecordBatchStream, context: &''life1 Arc<TaskContext>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<u64>> + ::core::marker::Send + ''async_trait>>
where
    Self: ::core::marker::Sync + ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
  ```
  File sink implementation of the [`DataSink::write_all`] method.

## Module `file_stream`

A generic stream over file format readers that can be used by
any file format that read its files from start to end.

Note: Most traits here need to be marked `Sync + Send` to be
compliant with the `SendableRecordBatchStream` trait.

```rust
pub mod file_stream { /* ... */ }
```

### Types

#### Struct `FileStream`

A stream that iterates record batch by record batch, file over file.

```rust
pub struct FileStream {
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
  pub fn new(config: &FileScanConfig, partition: usize, file_opener: Arc<dyn FileOpener>, metrics: &ExecutionPlanMetricsSet) -> Result<Self> { /* ... */ }
  ```
  Create a new `FileStream` using the give `FileOpener` to scan underlying files

- ```rust
  pub fn with_on_error(self: Self, on_error: OnError) -> Self { /* ... */ }
  ```
  Specify the behavior when an error occurs opening or scanning a file

###### Trait Implementations

- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **UnwindSafe**
- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **StreamExt**
- **TryStreamExt**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RecordBatchStream**
  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Type Alias `FileOpenFuture`

A fallible future that resolves to a stream of [`RecordBatch`]

```rust
pub type FileOpenFuture = futures::future::BoxFuture<''static, datafusion_common::error::Result<futures::stream::BoxStream<''static, datafusion_common::error::Result<arrow::record_batch::RecordBatch, arrow::error::ArrowError>>>>;
```

#### Enum `OnError`

Describes the behavior of the `FileStream` if file opening or scanning fails

```rust
pub enum OnError {
    Fail,
    Skip,
}
```

##### Variants

###### `Fail`

Fail the entire stream and return the underlying error

###### `Skip`

Continue scanning, ignoring the failed file

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Unpin**
- **MaybeSendSync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
#### Enum `NextOpen`

Represents the state of the next `FileOpenFuture`. Since we need to poll
this future while scanning the current file, we need to store the result if it
is ready

```rust
pub enum NextOpen {
    Pending(FileOpenFuture),
    Ready(datafusion_common::error::Result<futures::stream::BoxStream<''static, datafusion_common::error::Result<arrow::record_batch::RecordBatch, arrow::error::ArrowError>>>),
}
```

##### Variants

###### `Pending`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `FileOpenFuture` |  |

###### `Ready`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_common::error::Result<futures::stream::BoxStream<''static, datafusion_common::error::Result<arrow::record_batch::RecordBatch, arrow::error::ArrowError>>>` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **Sync**
- **Unpin**
- **ErasedDestructor**
- **Send**
- **Freeze**
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

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `FileStreamState`

```rust
pub enum FileStreamState {
    Idle,
    Open {
        future: FileOpenFuture,
        partition_values: Vec<datafusion_common::ScalarValue>,
    },
    Scan {
        partition_values: Vec<datafusion_common::ScalarValue>,
        reader: futures::stream::BoxStream<''static, datafusion_common::error::Result<arrow::record_batch::RecordBatch, arrow::error::ArrowError>>,
        next: Option<(NextOpen, Vec<datafusion_common::ScalarValue>)>,
    },
    Error,
    Limit,
}
```

##### Variants

###### `Idle`

The idle state, no file is currently being read

###### `Open`

Currently performing asynchronous IO to obtain a stream of RecordBatch
for a given file

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `future` | `FileOpenFuture` | A [`FileOpenFuture`] returned by [`FileOpener::open`] |
| `partition_values` | `Vec<datafusion_common::ScalarValue>` | The partition values for this file |

###### `Scan`

Scanning the [`BoxStream`] returned by the completion of a [`FileOpenFuture`]
returned by [`FileOpener::open`]

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `partition_values` | `Vec<datafusion_common::ScalarValue>` | Partitioning column values for the current batch_iter |
| `reader` | `futures::stream::BoxStream<''static, datafusion_common::error::Result<arrow::record_batch::RecordBatch, arrow::error::ArrowError>>` | The reader instance |
| `next` | `Option<(NextOpen, Vec<datafusion_common::ScalarValue>)>` | A [`FileOpenFuture`] for the next file to be processed,<br>and its corresponding partition column values, if any.<br>This allows the next file to be opened in parallel while the<br>current file is read. |

###### `Error`

Encountered an error

###### `Limit`

Reached the row limit

##### Implementations

###### Trait Implementations

- **IntoEither**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `StartableTime`

A timer that can be started and stopped.

```rust
pub struct StartableTime {
    pub metrics: datafusion_physical_plan::metrics::Time,
    pub start: Option<datafusion_common::instant::Instant>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `metrics` | `datafusion_physical_plan::metrics::Time` |  |
| `start` | `Option<datafusion_common::instant::Instant>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn start(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn stop(self: &mut Self) { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **IntoEither**
- **Freeze**
- **MaybeSendSync**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

#### Struct `FileStreamMetrics`

**Attributes:**

- `#[allow(rustdoc::broken_intra_doc_links)]`

Metrics for [`FileStream`]

Note that all of these metrics are in terms of wall clock time
(not cpu time) so they include time spent waiting on I/O as well
as other operators.

[`FileStream`]: <https://github.com/apache/datafusion/blob/main/datafusion/core/src/datasource/physical_plan/file_stream.rs>

```rust
pub struct FileStreamMetrics {
    pub time_opening: StartableTime,
    pub time_scanning_until_data: StartableTime,
    pub time_scanning_total: StartableTime,
    pub time_processing: StartableTime,
    pub file_open_errors: datafusion_physical_plan::metrics::Count,
    pub file_scan_errors: datafusion_physical_plan::metrics::Count,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `time_opening` | `StartableTime` | Wall clock time elapsed for file opening.<br><br>Time between when [`FileOpener::open`] is called and when the<br>[`FileStream`] receives a stream for reading.<br><br>If there are multiple files being scanned, the stream<br>will open the next file in the background while scanning the<br>current file. This metric will only capture time spent opening<br>while not also scanning.<br>[`FileStream`]: <https://github.com/apache/datafusion/blob/main/datafusion/core/src/datasource/physical_plan/file_stream.rs> |
| `time_scanning_until_data` | `StartableTime` | Wall clock time elapsed for file scanning + first record batch of decompression + decoding<br><br>Time between when the [`FileStream`] requests data from the<br>stream and when the first [`RecordBatch`] is produced.<br>[`FileStream`]: <https://github.com/apache/datafusion/blob/main/datafusion/core/src/datasource/physical_plan/file_stream.rs> |
| `time_scanning_total` | `StartableTime` | Total elapsed wall clock time for scanning + record batch decompression / decoding<br><br>Sum of time between when the [`FileStream`] requests data from<br>the stream and when a [`RecordBatch`] is produced for all<br>record batches in the stream. Note that this metric also<br>includes the time of the parent operator's execution. |
| `time_processing` | `StartableTime` | Wall clock time elapsed for data decompression + decoding<br><br>Time spent waiting for the FileStream's input. |
| `file_open_errors` | `datafusion_physical_plan::metrics::Count` | Count of errors opening file.<br><br>If using `OnError::Skip` this will provide a count of the number of files<br>which were skipped and will not be included in the scan results. |
| `file_scan_errors` | `datafusion_physical_plan::metrics::Count` | Count of errors scanning file<br><br>If using `OnError::Skip` this will provide a count of the number of files<br>which were skipped and will not be included in the scan results. |

##### Implementations

###### Methods

- ```rust
  pub fn new(metrics: &ExecutionPlanMetricsSet, partition: usize) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Allocation**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Traits

#### Trait `FileOpener`

Generic API for opening a file using an [`ObjectStore`] and resolving to a
stream of [`RecordBatch`]

[`ObjectStore`]: object_store::ObjectStore

```rust
pub trait FileOpener: Unpin + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `open`: Asynchronously open the specified file and return a stream

## Module `memory`

Execution plan for reading in-memory batches of data

```rust
pub mod memory { /* ... */ }
```

### Types

#### Struct `MemoryExec`

**Attributes:**

- `#[deprecated(since = "46.0.0", note =
"use MemorySourceConfig and DataSourceExec instead")]`

**⚠️ Deprecated since 46.0.0**: use MemorySourceConfig and DataSourceExec instead

Execution plan for reading in-memory batches of data

```rust
pub struct MemoryExec {
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
  pub fn try_new(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self> { /* ... */ }
  ```
  Create a new execution plan for reading in-memory record batches

- ```rust
  pub fn try_new_as_values(schema: SchemaRef, data: Vec<Vec<Arc<dyn PhysicalExpr>>>) -> Result<Self> { /* ... */ }
  ```
  Create a new execution plan from a list of constant values (`ValuesExec`)

- ```rust
  pub fn try_new_from_batches(schema: SchemaRef, batches: Vec<RecordBatch>) -> Result<Self> { /* ... */ }
  ```
  Create a new plan using the provided schema and batches.

- ```rust
  pub fn with_constraints(self: Self, constraints: Constraints) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_show_sizes(self: Self, show_sizes: bool) -> Self { /* ... */ }
  ```
  Set `show_sizes` to determine whether to display partition sizes

- ```rust
  pub fn constraints(self: &Self) -> &Constraints { /* ... */ }
  ```
  Ref to constraints

- ```rust
  pub fn partitions(self: &Self) -> &[Vec<RecordBatch>] { /* ... */ }
  ```
  Ref to partitions

- ```rust
  pub fn projection(self: &Self) -> &Option<Vec<usize>> { /* ... */ }
  ```
  Ref to projection

- ```rust
  pub fn show_sizes(self: &Self) -> bool { /* ... */ }
  ```
  Show sizes

- ```rust
  pub fn sort_information(self: &Self) -> &[LexOrdering] { /* ... */ }
  ```
  Ref to sort information

- ```rust
  pub fn try_with_sort_information(self: Self, sort_information: Vec<LexOrdering>) -> Result<Self> { /* ... */ }
  ```
  A memory table can be ordered by multiple expressions simultaneously.

- ```rust
  pub fn original_schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  Arc clone of ref to original schema

###### Trait Implementations

- **IntoEither**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```
    We recompute the statistics dynamically from the arrow metadata as it is pretty cheap to do so

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MemoryExec { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
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

#### Struct `MemorySourceConfig`

Data source configuration for reading in-memory batches of data

```rust
pub struct MemorySourceConfig {
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
  pub fn try_new(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self> { /* ... */ }
  ```
  Create a new `MemorySourceConfig` for reading in-memory record batches

- ```rust
  pub fn try_new_exec(partitions: &[Vec<RecordBatch>], schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Arc<DataSourceExec>> { /* ... */ }
  ```
  Create a new `DataSourceExec` plan for reading in-memory record batches

- ```rust
  pub fn try_new_as_values(schema: SchemaRef, data: Vec<Vec<Arc<dyn PhysicalExpr>>>) -> Result<Arc<DataSourceExec>> { /* ... */ }
  ```
  Create a new execution plan from a list of constant values (`ValuesExec`)

- ```rust
  pub fn try_new_from_batches(schema: SchemaRef, batches: Vec<RecordBatch>) -> Result<Arc<DataSourceExec>> { /* ... */ }
  ```
  Create a new plan using the provided schema and batches.

- ```rust
  pub fn with_limit(self: Self, limit: Option<usize>) -> Self { /* ... */ }
  ```
  Set the limit of the files

- ```rust
  pub fn with_show_sizes(self: Self, show_sizes: bool) -> Self { /* ... */ }
  ```
  Set `show_sizes` to determine whether to display partition sizes

- ```rust
  pub fn partitions(self: &Self) -> &[Vec<RecordBatch>] { /* ... */ }
  ```
  Ref to partitions

- ```rust
  pub fn projection(self: &Self) -> &Option<Vec<usize>> { /* ... */ }
  ```
  Ref to projection

- ```rust
  pub fn show_sizes(self: &Self) -> bool { /* ... */ }
  ```
  Show sizes

- ```rust
  pub fn sort_information(self: &Self) -> &[LexOrdering] { /* ... */ }
  ```
  Ref to sort information

- ```rust
  pub fn try_with_sort_information(self: Self, sort_information: Vec<LexOrdering>) -> Result<Self> { /* ... */ }
  ```
  A memory table can be ordered by multiple expressions simultaneously.

- ```rust
  pub fn original_schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  Arc clone of ref to original schema

###### Trait Implementations

- **MaybeSendSync**
- **RefUnwindSafe**
- **DataSource**
  - ```rust
    fn open(self: &Self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

  - ```rust
    fn output_partitioning(self: &Self) -> Partitioning { /* ... */ }
    ```

  - ```rust
    fn eq_properties(self: &Self) -> EquivalenceProperties { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn DataSource>> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MemorySourceConfig { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
#### Type Alias `PartitionData`

Type alias for partition data

```rust
pub type PartitionData = std::sync::Arc<tokio::sync::RwLock<Vec<arrow::array::RecordBatch>>>;
```

#### Struct `MemSink`

Implements for writing to a [`MemTable`]

[`MemTable`]: <https://docs.rs/datafusion/latest/datafusion/datasource/memory/struct.MemTable.html>

```rust
pub struct MemSink {
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
  pub fn try_new(batches: Vec<PartitionData>, schema: SchemaRef) -> Result<Self> { /* ... */ }
  ```
  Creates a new [`MemSink`].

###### Trait Implementations

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **DataSink**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn schema(self: &Self) -> &SchemaRef { /* ... */ }
    ```

  - ```rust
    fn write_all<''life0, ''life1, ''async_trait>(self: &''life0 Self, data: SendableRecordBatchStream, _context: &''life1 Arc<TaskContext>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<u64>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `schema_adapter`

[`SchemaAdapter`] and [`SchemaAdapterFactory`] to adapt file-level record batches to a table schema.

Adapter provides a method of translating the RecordBatches that come out of the
physical format into how they should be used by DataFusion.  For instance, a schema
can be stored external to a parquet file that maps parquet logical types to arrow types.

```rust
pub mod schema_adapter { /* ... */ }
```

### Types

#### Struct `DefaultSchemaAdapterFactory`

Default  [`SchemaAdapterFactory`] for mapping schemas.

This can be used to adapt file-level record batches to a table schema and
implement schema evolution.

Given an input file schema and a table schema, this factory returns
[`SchemaAdapter`] that return [`SchemaMapper`]s that:

1. Reorder columns
2. Cast columns to the correct type
3. Fill missing columns with nulls

# Errors:

* If a column in the table schema is non-nullable but is not present in the
  file schema (i.e. it is missing), the returned mapper tries to fill it with
  nulls resulting in a schema error.

# Illustration of Schema Mapping

```text
┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─                  ┌ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─
 ┌───────┐   ┌───────┐ │                  ┌───────┐   ┌───────┐   ┌───────┐ │
││  1.0  │   │ "foo" │                   ││ NULL  │   │ "foo" │   │ "1.0" │
 ├───────┤   ├───────┤ │ Schema mapping   ├───────┤   ├───────┤   ├───────┤ │
││  2.0  │   │ "bar" │                   ││  NULL │   │ "bar" │   │ "2.0" │
 └───────┘   └───────┘ │────────────────▶ └───────┘   └───────┘   └───────┘ │
│                                        │
 column "c"  column "b"│                  column "a"  column "b"  column "c"│
│ Float64       Utf8                     │  Int32        Utf8        Utf8
 ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘                  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ┘
    Input Record Batch                         Output Record Batch

    Schema {                                   Schema {
     "c": Float64,                              "a": Int32,
     "b": Utf8,                                 "b": Utf8,
    }                                           "c": Utf8,
                                               }
```

# Example of using the `DefaultSchemaAdapterFactory` to map [`RecordBatch`]s

Note `SchemaMapping` also supports mapping partial batches, which is used as
part of predicate pushdown.

```
# use std::sync::Arc;
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_datasource::schema_adapter::{DefaultSchemaAdapterFactory, SchemaAdapterFactory};
# use datafusion_common::record_batch;
// Table has fields "a",  "b" and "c"
let table_schema = Schema::new(vec![
    Field::new("a", DataType::Int32, true),
    Field::new("b", DataType::Utf8, true),
    Field::new("c", DataType::Utf8, true),
]);

// create an adapter to map the table schema to the file schema
let adapter = DefaultSchemaAdapterFactory::from_schema(Arc::new(table_schema));

// The file schema has fields "c" and "b" but "b" is stored as an 'Float64'
// instead of 'Utf8'
let file_schema = Schema::new(vec![
   Field::new("c", DataType::Utf8, true),
   Field::new("b", DataType::Float64, true),
]);

// Get a mapping from the file schema to the table schema
let (mapper, _indices) = adapter.map_schema(&file_schema).unwrap();

let file_batch = record_batch!(
    ("c", Utf8, vec!["foo", "bar"]),
    ("b", Float64, vec![1.0, 2.0])
).unwrap();

let mapped_batch = mapper.map_batch(file_batch).unwrap();

// the mapped batch has the correct schema and the "b" column has been cast to Utf8
let expected_batch = record_batch!(
   ("a", Int32, vec![None, None]),  // missing column filled with nulls
   ("b", Utf8, vec!["1.0", "2.0"]), // b was cast to string and order was changed
   ("c", Utf8, vec!["foo", "bar"])
).unwrap();
assert_eq!(mapped_batch, expected_batch);
```

```rust
pub struct DefaultSchemaAdapterFactory;
```

##### Implementations

###### Methods

- ```rust
  pub fn from_schema(table_schema: SchemaRef) -> Box<dyn SchemaAdapter> { /* ... */ }
  ```
  Create a new factory for mapping batches from a file schema to a table

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
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
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> DefaultSchemaAdapterFactory { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DefaultSchemaAdapterFactory { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **SchemaAdapterFactory**
  - ```rust
    fn create(self: &Self, projected_table_schema: SchemaRef, _table_schema: SchemaRef) -> Box<dyn SchemaAdapter> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `SchemaMapping`

The SchemaMapping struct holds a mapping from the file schema to the table
schema and any necessary type conversions.

[`map_batch`] is used by the ParquetOpener to produce a RecordBatch which
has the projected schema, since that's the schema which is supposed to come
out of the execution of this query. Thus `map_batch` uses
`projected_table_schema` as it can only operate on the projected fields.

[`map_batch`]: Self::map_batch

```rust
pub struct SchemaMapping {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **SchemaMapper**
  - ```rust
    fn map_batch(self: &Self, batch: RecordBatch) -> datafusion_common::Result<RecordBatch> { /* ... */ }
    ```
    Adapts a `RecordBatch` to match the `projected_table_schema` using the stored mapping and

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **ErasedDestructor**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Traits

#### Trait `SchemaAdapterFactory`

Factory for creating [`SchemaAdapter`]

This interface provides a way to implement custom schema adaptation logic
for DataSourceExec (for example, to fill missing columns with default value
other than null).

Most users should use [`DefaultSchemaAdapterFactory`]. See that struct for
more details and examples.

```rust
pub trait SchemaAdapterFactory: Debug + Send + Sync + ''static {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `create`: Create a [`SchemaAdapter`]

##### Implementations

This trait is implemented for the following types:

- `DefaultSchemaAdapterFactory`

#### Trait `SchemaAdapter`

Creates [`SchemaMapper`]s to map file-level [`RecordBatch`]es to a table
schema, which may have a schema obtained from merging multiple file-level
schemas.

This is useful for implementing schema evolution in partitioned datasets.

See [`DefaultSchemaAdapterFactory`] for more details and examples.

```rust
pub trait SchemaAdapter: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `map_column_index`: Map a column index in the table schema to a column index in a particular
- `map_schema`: Creates a mapping for casting columns from the file schema to the table

#### Trait `SchemaMapper`

Maps, columns from a specific file schema to the table schema.

See [`DefaultSchemaAdapterFactory`] for more details and examples.

```rust
pub trait SchemaMapper: Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `map_batch`: Adapts a `RecordBatch` to match the `table_schema`

##### Implementations

This trait is implemented for the following types:

- `SchemaMapping`

## Module `sink`

Execution plan for writing data to [`DataSink`]s

```rust
pub mod sink { /* ... */ }
```

### Types

#### Struct `DataSinkExec`

Execution plan for writing record batches to a [`DataSink`]

Returns a single row with the number of values written

```rust
pub struct DataSinkExec {
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
  pub fn new(input: Arc<dyn ExecutionPlan>, sink: Arc<dyn DataSink>, sort_order: Option<LexRequirement>) -> Self { /* ... */ }
  ```
  Create a plan to write to `sink`

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input execution plan

- ```rust
  pub fn sink(self: &Self) -> &dyn DataSink { /* ... */ }
  ```
  Returns insert sink

- ```rust
  pub fn sort_order(self: &Self) -> &Option<LexRequirement> { /* ... */ }
  ```
  Optional sort order for output data

###### Trait Implementations

- **Freeze**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DataSinkExec { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn required_input_distribution(self: &Self) -> Vec<Distribution> { /* ... */ }
    ```

  - ```rust
    fn required_input_ordering(self: &Self) -> Vec<Option<LexRequirement>> { /* ... */ }
    ```

  - ```rust
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn children(self: &Self) -> Vec<&Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```
    Execute the plan and return a stream of `RecordBatch`es for

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```
    Returns the metrics of the underlying [DataSink]

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
### Traits

#### Trait `DataSink`

`DataSink` implements writing streams of [`RecordBatch`]es to
user defined destinations.

The `Display` impl is used to format the sink for explain plan
output.

```rust
pub trait DataSink: DisplayAs + Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_any`: Returns the data sink as [`Any`](std::any::Any) so that it can be
- `schema`: Returns the sink schema
- `write_all`: Writes the data to the sink, returns the number of values written

##### Provided Methods

- ```rust
  fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
  ```
  Return a snapshot of the [MetricsSet] for this

##### Implementations

This trait is implemented for the following types:

- `MemSink`

## Module `source`

[`DataSource`] and [`DataSourceExec`]

```rust
pub mod source { /* ... */ }
```

### Types

#### Struct `DataSourceExec`

[`ExecutionPlan`] handles different file formats like JSON, CSV, AVRO, ARROW, PARQUET

`DataSourceExec` implements common functionality such as applying projections,
and caching plan properties.

The [`DataSource`] trait describes where to find the data for this data
source (for example what files or what in memory partitions). Format
specifics are implemented with the [`FileSource`] trait.

[`FileSource`]: crate::file::FileSource

```rust
pub struct DataSourceExec {
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
  pub fn from_data_source</* synthetic */ impl DataSource + 'static: DataSource + ''static>(data_source: impl DataSource + ''static) -> Arc<Self> { /* ... */ }
  ```

- ```rust
  pub fn new(data_source: Arc<dyn DataSource>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn data_source(self: &Self) -> &Arc<dyn DataSource> { /* ... */ }
  ```
  Return the source object

- ```rust
  pub fn with_data_source(self: Self, data_source: Arc<dyn DataSource>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_constraints(self: Self, constraints: Constraints) -> Self { /* ... */ }
  ```
  Assign constraints

- ```rust
  pub fn with_partitioning(self: Self, partitioning: Partitioning) -> Self { /* ... */ }
  ```
  Assign output partitioning

- ```rust
  pub fn downcast_to_file_source<T: ''static>(self: &Self) -> Option<(&FileScanConfig, &T)> { /* ... */ }
  ```
  Downcast the `DataSourceExec`'s `data_source` to a specific file source

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DataSourceExec { /* ... */ }
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

- **Sync**
- **UnwindSafe**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **MaybeSendSync**
- **RefUnwindSafe**
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
    fn with_new_children(self: Arc<Self>, _: Vec<Arc<dyn ExecutionPlan>>) -> datafusion_common::Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn repartitioned(self: &Self, target_partitions: usize, config: &ConfigOptions) -> datafusion_common::Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> datafusion_common::Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> datafusion_common::Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> datafusion_common::Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **ErasedDestructor**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `DataSource`

Common behaviors in Data Sources for both from Files and Memory.

# See Also
* [`DataSourceExec`] for physical plan implementation
* [`FileSource`] for file format implementations (Parquet, Json, etc)

# Notes
Requires `Debug` to assist debugging

[`FileSource`]: crate::file::FileSource

```rust
pub trait DataSource: Send + Sync + Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `open`
- `as_any`
- `fmt_as`: Format this source for display in explain plans
- `output_partitioning`
- `eq_properties`
- `statistics`
- `with_fetch`: Return a copy of this DataSource with a new fetch limit
- `fetch`
- `try_swapping_with_projection`

##### Provided Methods

- ```rust
  fn repartitioned(self: &Self, _target_partitions: usize, _repartition_file_min_size: usize, _output_ordering: Option<LexOrdering>) -> datafusion_common::Result<Option<Arc<dyn DataSource>>> { /* ... */ }
  ```
  Return a copy of this DataSource with a new partitioning scheme

- ```rust
  fn metrics(self: &Self) -> ExecutionPlanMetricsSet { /* ... */ }
  ```

##### Implementations

This trait is implemented for the following types:

- `FileScanConfig`
- `MemorySourceConfig`

## Module `url`

```rust
pub mod url { /* ... */ }
```

### Types

#### Struct `ListingTableUrl`

A parsed URL identifying files for a listing table, see [`ListingTableUrl::parse`]
for more information on the supported expressions

```rust
pub struct ListingTableUrl {
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
  Parse a provided string as a `ListingTableUrl`

- ```rust
  pub fn try_new(url: Url, glob: Option<Pattern>) -> Result<Self> { /* ... */ }
  ```
  Creates a new [`ListingTableUrl`] from a url and optional glob expression

- ```rust
  pub fn scheme(self: &Self) -> &str { /* ... */ }
  ```
  Returns the URL scheme

- ```rust
  pub fn prefix(self: &Self) -> &Path { /* ... */ }
  ```
  Return the URL path not excluding any glob expression

- ```rust
  pub fn contains(self: &Self, path: &Path, ignore_subdirectory: bool) -> bool { /* ... */ }
  ```
  Returns `true` if `path` matches this [`ListingTableUrl`]

- ```rust
  pub fn is_collection(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if `path` refers to a collection of objects

- ```rust
  pub fn file_extension(self: &Self) -> Option<&str> { /* ... */ }
  ```
  Returns the file extension of the last path segment if it exists

- ```rust
  pub fn strip_prefix<''a, ''b: ''a>(self: &''a Self, path: &''b Path) -> Option<impl Iterator<Item = &''b str> + ''a> { /* ... */ }
  ```
  Strips the prefix of this [`ListingTableUrl`] from the provided path, returning

- ```rust
  pub async fn list_all_files<''a>(self: &''a Self, ctx: &''a dyn Session, store: &''a dyn ObjectStore, file_extension: &''a str) -> Result<BoxStream<''a, Result<ObjectMeta>>> { /* ... */ }
  ```
  List all files identified by this [`ListingTableUrl`] for the provided `file_extension`

- ```rust
  pub fn as_str(self: &Self) -> &str { /* ... */ }
  ```
  Returns this [`ListingTableUrl`] as a string

- ```rust
  pub fn object_store(self: &Self) -> ObjectStoreUrl { /* ... */ }
  ```
  Return the [`ObjectStoreUrl`] for this [`ListingTableUrl`]

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn as_ref(self: &Self) -> &Url { /* ... */ }
    ```

- **RefUnwindSafe**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ListingTableUrl) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Allocation**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ListingTableUrl { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `write`

Module containing helper methods/traits related to enabling
write support for the various file formats

```rust
pub mod write { /* ... */ }
```

### Modules

## Module `demux`

Module containing helper methods/traits related to enabling
dividing input stream into multiple output files at execution time

```rust
pub mod demux { /* ... */ }
```

### Types

#### Type Alias `DemuxedStreamReceiver`

```rust
pub type DemuxedStreamReceiver = tokio::sync::mpsc::UnboundedReceiver<(object_store::path::Path, tokio::sync::mpsc::Receiver<arrow::array::RecordBatch>)>;
```

## Module `orchestration`

Module containing helper methods/traits related to
orchestrating file serialization, streaming to object store,
parallelization, and abort handling

```rust
pub mod orchestration { /* ... */ }
```

### Functions

#### Function `spawn_writer_tasks_and_join`

Orchestrates multipart put of a dynamic number of output files from a single input stream
for any statelessly serialized file type. That is, any file type for which each [RecordBatch]
can be serialized independently of all other [RecordBatch]s.

```rust
pub async fn spawn_writer_tasks_and_join(context: &std::sync::Arc<datafusion_execution::TaskContext>, serializer: std::sync::Arc<dyn BatchSerializer>, compression: crate::file_compression_type::FileCompressionType, object_store: std::sync::Arc<dyn ObjectStore>, demux_task: datafusion_common_runtime::SpawnedTask<datafusion_common::error::Result<()>>, file_stream_rx: super::demux::DemuxedStreamReceiver) -> datafusion_common::error::Result<u64> { /* ... */ }
```

### Types

#### Struct `SharedBuffer`

A buffer with interior mutability shared by the SerializedFileWriter and
ObjectStore writer

```rust
pub struct SharedBuffer {
    pub buffer: std::sync::Arc<futures::lock::Mutex<Vec<u8>>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `buffer` | `std::sync::Arc<futures::lock::Mutex<Vec<u8>>>` | The inner buffer for reading and writing<br><br>The lock is used to obtain internal mutability, so no worry about the<br>lock contention. |

##### Implementations

###### Methods

- ```rust
  pub fn new(capacity: usize) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SharedBuffer { /* ... */ }
    ```

- **Write**
  - ```rust
    fn write(self: &mut Self, buf: &[u8]) -> std::io::Result<usize> { /* ... */ }
    ```

  - ```rust
    fn flush(self: &mut Self) -> std::io::Result<()> { /* ... */ }
    ```

- **Unpin**
- **TWriteTransport**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Sync**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **WriteBytesExt**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VarIntWriter**
  - ```rust
    fn write_varint<VI>(self: &mut Self, n: VI) -> Result<usize, Error>
where
    VI: VarInt { /* ... */ }
    ```

- **FixedIntWriter**
  - ```rust
    fn write_fixedint<FI>(self: &mut Self, n: FI) -> Result<usize, Error>
where
    FI: FixedInt { /* ... */ }
    ```

- **Send**
### Traits

#### Trait `BatchSerializer`

A trait that defines the methods required for a RecordBatch serializer.

```rust
pub trait BatchSerializer: Sync + Send {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `serialize`: Asynchronously serializes a `RecordBatch` and returns the serialized bytes.

### Functions

#### Function `create_writer`

Returns an [`AsyncWrite`] which writes to the given object store location
with the specified compression.
We drop the `AbortableWrite` struct and the writer will not try to cleanup on failure.
Users can configure automatic cleanup with their cloud provider.

```rust
pub async fn create_writer(file_compression_type: crate::file_compression_type::FileCompressionType, location: &object_store::path::Path, object_store: std::sync::Arc<dyn ObjectStore>) -> datafusion_common::error::Result<Box<dyn AsyncWrite + Send + Unpin>> { /* ... */ }
```

#### Function `get_writer_schema`

Converts table schema to writer schema, which may differ in the case
of hive style partitioning where some columns are removed from the
underlying files.

```rust
pub fn get_writer_schema(config: &crate::file_sink_config::FileSinkConfig) -> std::sync::Arc<arrow::datatypes::Schema> { /* ... */ }
```

## Types

### Type Alias `PartitionedFileStream`

Stream of files get listed from object store

```rust
pub type PartitionedFileStream = std::pin::Pin<Box<dyn Stream<Item = datafusion_common::Result<PartitionedFile>> + Send + Sync + ''static>>;
```

### Struct `FileRange`

Only scan a subset of Row Groups from the Parquet file whose data "midpoint"
lies within the [start, end) byte offsets. This option can be used to scan non-overlapping
sections of a Parquet file in parallel.

```rust
pub struct FileRange {
    pub start: i64,
    pub end: i64,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `i64` | Range start |
| `end` | `i64` | Range end |

#### Implementations

##### Methods

- ```rust
  pub fn contains(self: &Self, offset: i64) -> bool { /* ... */ }
  ```
  returns true if this file range contains the specified offset

##### Trait Implementations

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
- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **UnwindSafe**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> FileRange { /* ... */ }
    ```

- **Sync**
- **ErasedDestructor**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &FileRange) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **MaybeSendSync**
- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &FileRange) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &FileRange) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
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

### Struct `PartitionedFile`

A single file or part of a file that should be read, along with its schema, statistics
and partition column values that need to be appended to each row.

```rust
pub struct PartitionedFile {
    pub object_meta: object_store::ObjectMeta,
    pub partition_values: Vec<datafusion_common::ScalarValue>,
    pub range: Option<FileRange>,
    pub statistics: Option<std::sync::Arc<datafusion_common::Statistics>>,
    pub extensions: Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>,
    pub metadata_size_hint: Option<usize>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `object_meta` | `object_store::ObjectMeta` | Path for the file (e.g. URL, filesystem path, etc) |
| `partition_values` | `Vec<datafusion_common::ScalarValue>` | Values of partition columns to be appended to each row.<br><br>These MUST have the same count, order, and type than the [`table_partition_cols`].<br><br>You may use [`wrap_partition_value_in_dict`] to wrap them if you have used [`wrap_partition_type_in_dict`] to wrap the column type.<br><br><br>[`wrap_partition_type_in_dict`]: https://github.com/apache/datafusion/blob/main/datafusion/core/src/datasource/physical_plan/file_scan_config.rs#L55<br>[`wrap_partition_value_in_dict`]: https://github.com/apache/datafusion/blob/main/datafusion/core/src/datasource/physical_plan/file_scan_config.rs#L62<br>[`table_partition_cols`]: https://github.com/apache/datafusion/blob/main/datafusion/core/src/datasource/file_format/options.rs#L190 |
| `range` | `Option<FileRange>` | An optional file range for a more fine-grained parallel execution |
| `statistics` | `Option<std::sync::Arc<datafusion_common::Statistics>>` | Optional statistics that describe the data in this file if known.<br><br>DataFusion relies on these statistics for planning (in particular to sort file groups),<br>so if they are incorrect, incorrect answers may result. |
| `extensions` | `Option<std::sync::Arc<dyn std::any::Any + Send + Sync>>` | An optional field for user defined per object metadata |
| `metadata_size_hint` | `Option<usize>` | The estimated size of the parquet metadata, in bytes |

#### Implementations

##### Methods

- ```rust
  pub fn new</* synthetic */ impl Into<String>: Into<String>>(path: impl Into<String>, size: u64) -> Self { /* ... */ }
  ```
  Create a simple file without metadata or partition

- ```rust
  pub fn new_with_range(path: String, size: u64, start: i64, end: i64) -> Self { /* ... */ }
  ```
  Create a file range without metadata or partition

- ```rust
  pub fn with_metadata_size_hint(self: Self, metadata_size_hint: usize) -> Self { /* ... */ }
  ```
  Provide a hint to the size of the file metadata. If a hint is provided

- ```rust
  pub fn from_path(path: String) -> Result<Self> { /* ... */ }
  ```
  Return a file reference from the given path

- ```rust
  pub fn path(self: &Self) -> &Path { /* ... */ }
  ```
  Return the path of this partitioned file

- ```rust
  pub fn with_range(self: Self, start: i64, end: i64) -> Self { /* ... */ }
  ```
  Update the file to only scan the specified range (in bytes)

- ```rust
  pub fn with_extensions(self: Self, extensions: Arc<dyn std::any::Any + Send + Sync>) -> Self { /* ... */ }
  ```
  Update the user defined extensions for this file.

- ```rust
  pub fn with_statistics(self: Self, statistics: Arc<Statistics>) -> Self { /* ... */ }
  ```

##### Trait Implementations

- **MaybeSendSync**
- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PartitionedFile { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **RefUnwindSafe**
- **FromIterator**
  - ```rust
    fn from_iter<I: IntoIterator<Item = PartitionedFile>>(iter: I) -> Self { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(object_meta: ObjectMeta) -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Freeze**
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

- **ErasedDestructor**
### Enum `RangeCalculation`

Represents the possible outcomes of a range calculation.

This enum is used to encapsulate the result of calculating the range of
bytes to read from an object (like a file) in an object store.

Variants:
- `Range(Option<Range<usize>>)`:
  Represents a range of bytes to be read. It contains an `Option` wrapping a
  `Range<usize>`. `None` signifies that the entire object should be read,
  while `Some(range)` specifies the exact byte range to read.
- `TerminateEarly`:
  Indicates that the range calculation determined no further action is
  necessary, possibly because the calculated range is empty or invalid.

```rust
pub enum RangeCalculation {
    Range(Option<std::ops::Range<u64>>),
    TerminateEarly,
}
```

#### Variants

##### `Range`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Option<std::ops::Range<u64>>` |  |

##### `TerminateEarly`

#### Implementations

##### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Unpin**
- **Send**
- **Allocation**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
## Functions

### Function `calculate_range`

Calculates an appropriate byte range for reading from an object based on the
provided metadata.

This asynchronous function examines the `FileMeta` of an object in an object store
and determines the range of bytes to be read. The range calculation may adjust
the start and end points to align with meaningful data boundaries (like newlines).

Returns a `Result` wrapping a `RangeCalculation`, which is either a calculated byte range or an indication to terminate early.

Returns an `Error` if any part of the range calculation fails, such as issues in reading from the object store or invalid range boundaries.

```rust
pub async fn calculate_range(file_meta: &file_meta::FileMeta, store: &std::sync::Arc<dyn ObjectStore>, terminator: Option<u8>) -> datafusion_common::Result<RangeCalculation> { /* ... */ }
```

### Function `generate_test_files`

Generates test files with min-max statistics in different overlap patterns.

Used by tests and benchmarks.

# Overlap Factors

The `overlap_factor` parameter controls how much the value ranges in generated test files overlap:
- `0.0`: No overlap between files (completely disjoint ranges)
- `0.2`: Low overlap (20% of the range size overlaps with adjacent files)
- `0.5`: Medium overlap (50% of ranges overlap)
- `0.8`: High overlap (80% of ranges overlap between files)

# Examples

With 5 files and different overlap factors showing `[min, max]` ranges:

overlap_factor = 0.0 (no overlap):

File 0: [0, 20]
File 1: [20, 40]
File 2: [40, 60]
File 3: [60, 80]
File 4: [80, 100]

overlap_factor = 0.5 (50% overlap):

File 0: [0, 40]
File 1: [20, 60]
File 2: [40, 80]
File 3: [60, 100]
File 4: [80, 120]

overlap_factor = 0.8 (80% overlap):

File 0: [0, 100]
File 1: [20, 120]
File 2: [40, 140]
File 3: [60, 160]
File 4: [80, 180]

```rust
pub fn generate_test_files(num_files: usize, overlap_factor: f64) -> Vec<crate::file_groups::FileGroup> { /* ... */ }
```

### Function `verify_sort_integrity`

Used by tests and benchmarks

```rust
pub fn verify_sort_integrity(file_groups: &[crate::file_groups::FileGroup]) -> bool { /* ... */ }
```

## Re-exports

### Re-export `ListingTableUrl`

```rust
pub use self::url::ListingTableUrl;
```

### Re-export `add_row_stats`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use statistics::add_row_stats;
```

### Re-export `compute_all_files_statistics`

```rust
pub use statistics::compute_all_files_statistics;
```


# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_catalog`

Interfaces and default implementations of catalogs and schemas.

Implementations
* Information schema: [`information_schema`]
* Simple memory based catalog: [`MemoryCatalogProviderList`], [`MemoryCatalogProvider`], [`MemorySchemaProvider`]
* Listing schema: [`listing_schema`]

## Modules

## Module `cte_worktable`

CteWorkTable implementation used for recursive queries

```rust
pub mod cte_worktable { /* ... */ }
```

### Types

#### Struct `CteWorkTable`

The temporary working table where the previous iteration of a recursive query is stored
Naming is based on PostgreSQL's implementation.
See here for more details: www.postgresql.org/docs/11/queries-with.html#id-1.5.6.12.5.4

```rust
pub struct CteWorkTable {
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
  pub fn new(name: &str, table_schema: SchemaRef) -> Self { /* ... */ }
  ```
  construct a new CteWorkTable with the given name and schema

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  The user-provided name of the CTE

- ```rust
  pub fn schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  The schema of the recursive term of the query

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Allocation**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TableProvider**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn get_logical_plan(self: &Self) -> Option<Cow<''_, LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

  - ```rust
    fn table_type(self: &Self) -> TableType { /* ... */ }
    ```

  - ```rust
    fn scan<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, _projection: Option<&''life2 Vec<usize>>, _filters: &''life3 [Expr], _limit: Option<usize>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn supports_filters_pushdown(self: &Self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `default_table_source`

Default TableSource implementation used in DataFusion physical plans

```rust
pub mod default_table_source { /* ... */ }
```

### Types

#### Struct `DefaultTableSource`

Implements [`TableSource`] for a [`TableProvider`]

This structure adapts a [`TableProvider`] (a physical plan trait) to the
[`TableSource`] (logical plan trait).

It is used so logical plans in the `datafusion_expr` crate do not have a
direct dependency on physical plans, such as [`TableProvider`]s.

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/datasource/provider/trait.TableProvider.html

```rust
pub struct DefaultTableSource {
    pub table_provider: std::sync::Arc<dyn TableProvider>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `table_provider` | `std::sync::Arc<dyn TableProvider>` | table provider |

##### Implementations

###### Methods

- ```rust
  pub fn new(table_provider: Arc<dyn TableProvider>) -> Self { /* ... */ }
  ```
  Create a new DefaultTableSource to wrap a TableProvider

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **TableSource**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```
    Returns the table source as [`Any`] so that it can be

  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```
    Get a reference to the schema for this table

  - ```rust
    fn constraints(self: &Self) -> Option<&Constraints> { /* ... */ }
    ```
    Get a reference to applicable constraints, if any exists.

  - ```rust
    fn table_type(self: &Self) -> TableType { /* ... */ }
    ```
    Get the type of this table for metadata/catalog purposes.

  - ```rust
    fn supports_filters_pushdown(self: &Self, filter: &[&Expr]) -> datafusion_common::Result<Vec<TableProviderFilterPushDown>> { /* ... */ }
    ```
    Tests whether the table provider can make use of any or all filter expressions

  - ```rust
    fn get_logical_plan(self: &Self) -> Option<Cow<''_, datafusion_expr::LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn get_column_default(self: &Self, column: &str) -> Option<&Expr> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **IntoEither**
- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
### Functions

#### Function `provider_as_source`

Wrap TableProvider in TableSource

```rust
pub fn provider_as_source(table_provider: std::sync::Arc<dyn TableProvider>) -> std::sync::Arc<dyn TableSource> { /* ... */ }
```

#### Function `source_as_provider`

Attempt to downcast a TableSource to DefaultTableSource and access the
TableProvider. This will only work with a TableSource created by DataFusion.

```rust
pub fn source_as_provider(source: &std::sync::Arc<dyn TableSource>) -> datafusion_common::Result<std::sync::Arc<dyn TableProvider>> { /* ... */ }
```

## Module `information_schema`

[`InformationSchemaProvider`] that implements the SQL [Information Schema] for DataFusion.

[Information Schema]: https://en.wikipedia.org/wiki/Information_schema

```rust
pub mod information_schema { /* ... */ }
```

### Types

#### Struct `InformationSchemaProvider`

Implements the `information_schema` virtual schema and tables

The underlying tables in the `information_schema` are created on
demand. This means that if more tables are added to the underlying
providers, they will appear the next time the `information_schema`
table is queried.

```rust
pub struct InformationSchemaProvider {
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
  pub fn new(catalog_list: Arc<dyn CatalogProviderList>) -> Self { /* ... */ }
  ```
  Creates a new [`InformationSchemaProvider`] for the provided `catalog_list`

###### Trait Implementations

- **ErasedDestructor**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Send**
- **SchemaProvider**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn table_names(self: &Self) -> Vec<String> { /* ... */ }
    ```

  - ```rust
    fn table<''life0, ''life1, ''async_trait>(self: &''life0 Self, name: &''life1 str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<Arc<dyn TableProvider>>, DataFusionError>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn table_exist(self: &Self, name: &str) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Constants and Statics

#### Constant `INFORMATION_SCHEMA`

```rust
pub const INFORMATION_SCHEMA: &str = "information_schema";
```

#### Constant `INFORMATION_SCHEMA_TABLES`

All information schema tables

```rust
pub const INFORMATION_SCHEMA_TABLES: &[&str] = _;
```

## Module `listing_schema`

[`ListingSchemaProvider`]: [`SchemaProvider`] that scans ObjectStores for tables automatically

```rust
pub mod listing_schema { /* ... */ }
```

### Types

#### Struct `ListingSchemaProvider`

A [`SchemaProvider`] that scans an [`ObjectStore`] to automatically discover tables

A subfolder relationship is assumed, i.e. given:
- authority = `s3://host.example.com:3000`
- path = `/data/tpch`
- factory = `DeltaTableFactory`

A table called "customer" will be registered for the folder:
`s3://host.example.com:3000/data/tpch/customer`

assuming it contains valid deltalake data, i.e:
- `s3://host.example.com:3000/data/tpch/customer/part-00000-xxxx.snappy.parquet`
- `s3://host.example.com:3000/data/tpch/customer/_delta_log/`

[`ObjectStore`]: object_store::ObjectStore

```rust
pub struct ListingSchemaProvider {
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
  pub fn new(authority: String, path: object_store::path::Path, factory: Arc<dyn TableProviderFactory>, store: Arc<dyn ObjectStore>, format: String) -> Self { /* ... */ }
  ```
  Create a new `ListingSchemaProvider`

- ```rust
  pub async fn refresh(self: &Self, state: &dyn Session) -> datafusion_common::Result<()> { /* ... */ }
  ```
  Reload table information from ObjectStore

###### Trait Implementations

- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **SchemaProvider**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn table_names(self: &Self) -> Vec<String> { /* ... */ }
    ```

  - ```rust
    fn table<''life0, ''life1, ''async_trait>(self: &''life0 Self, name: &''life1 str) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Option<Arc<dyn TableProvider>>, DataFusionError>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn register_table(self: &Self, name: String, table: Arc<dyn TableProvider>) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>> { /* ... */ }
    ```

  - ```rust
    fn deregister_table(self: &Self, name: &str) -> datafusion_common::Result<Option<Arc<dyn TableProvider>>> { /* ... */ }
    ```

  - ```rust
    fn table_exist(self: &Self, name: &str) -> bool { /* ... */ }
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

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
## Module `memory`

```rust
pub mod memory { /* ... */ }
```

### Re-exports

#### Re-export `MemorySourceConfig`

```rust
pub use datafusion_datasource::memory::MemorySourceConfig;
```

#### Re-export `DataSourceExec`

```rust
pub use datafusion_datasource::source::DataSourceExec;
```

#### Re-export `catalog::*`

```rust
pub use catalog::*;
```

#### Re-export `schema::*`

```rust
pub use schema::*;
```

#### Re-export `table::*`

```rust
pub use table::*;
```

## Module `stream`

TableProvider for stream sources, such as FIFO files

```rust
pub mod stream { /* ... */ }
```

### Types

#### Struct `StreamTableFactory`

A [`TableProviderFactory`] for [`StreamTable`]

```rust
pub struct StreamTableFactory {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **UnwindSafe**
- **ErasedDestructor**
- **Allocation**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> StreamTableFactory { /* ... */ }
    ```

- **Send**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **TableProviderFactory**
  - ```rust
    fn create<''life0, ''life1, ''life2, ''async_trait>(self: &''life0 Self, state: &''life1 dyn Session, cmd: &''life2 CreateExternalTable) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn TableProvider>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait { /* ... */ }
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

#### Enum `StreamEncoding`

The data encoding for [`StreamTable`]

```rust
pub enum StreamEncoding {
    Csv,
    Json,
}
```

##### Variants

###### `Csv`

CSV records

###### `Json`

Newline-delimited JSON records

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StreamEncoding { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **FromStr**
  - ```rust
    fn from_str(s: &str) -> std::result::Result<Self, <Self as >::Err> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

#### Struct `FileStreamProvider`

Stream data from the file at `location`

* Data will be read sequentially from the provided `location`
* New data will be appended to the end of the file

The encoding can be configured with [`Self::with_encoding`] and
defaults to [`StreamEncoding::Csv`]

```rust
pub struct FileStreamProvider {
    pub schema: arrow::datatypes::SchemaRef,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `schema` | `arrow::datatypes::SchemaRef` | Get a reference to the schema for this file stream |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new_file(schema: SchemaRef, location: PathBuf) -> Self { /* ... */ }
  ```
  Stream data from the file at `location`

- ```rust
  pub fn with_batch_size(self: Self, batch_size: usize) -> Self { /* ... */ }
  ```
  Set the batch size (the number of rows to load at one time)

- ```rust
  pub fn with_header(self: Self, header: bool) -> Self { /* ... */ }
  ```
  Specify whether the file has a header (only applicable for [`StreamEncoding::Csv`])

- ```rust
  pub fn with_encoding(self: Self, encoding: StreamEncoding) -> Self { /* ... */ }
  ```
  Specify an encoding for the stream

###### Trait Implementations

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **Unpin**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StreamProvider**
  - ```rust
    fn schema(self: &Self) -> &SchemaRef { /* ... */ }
    ```

  - ```rust
    fn reader(self: &Self) -> Result<Box<dyn RecordBatchReader>> { /* ... */ }
    ```

  - ```rust
    fn writer(self: &Self) -> Result<Box<dyn RecordBatchWriter>> { /* ... */ }
    ```

  - ```rust
    fn stream_write_display(self: &Self, _t: DisplayFormatType, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
#### Struct `StreamConfig`

The configuration for a [`StreamTable`]

```rust
pub struct StreamConfig {
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
  pub fn new(source: Arc<dyn StreamProvider>) -> Self { /* ... */ }
  ```
  Create a new `StreamConfig` from a `StreamProvider`

- ```rust
  pub fn with_order(self: Self, order: Vec<Vec<SortExpr>>) -> Self { /* ... */ }
  ```
  Specify a sort order for the stream

- ```rust
  pub fn with_constraints(self: Self, constraints: Constraints) -> Self { /* ... */ }
  ```
  Assign constraints

###### Trait Implementations

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **Unpin**
- **UnwindSafe**
#### Struct `StreamTable`

A [`TableProvider`] for an unbounded stream source

Currently only reading from / appending to a single file in-place is supported, but
other stream sources and sinks may be added in future.

Applications looking to read/write datasets comprising multiple files, e.g. [Hadoop]-style
data stored in object storage, should instead consider [`ListingTable`].

[Hadoop]: https://hadoop.apache.org/
[`ListingTable`]: https://docs.rs/datafusion/latest/datafusion/datasource/listing/struct.ListingTable.html

```rust
pub struct StreamTable(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(config: Arc<StreamConfig>) -> Self { /* ... */ }
  ```
  Create a new [`StreamTable`] for the given [`StreamConfig`]

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **TableProvider**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

  - ```rust
    fn constraints(self: &Self) -> Option<&Constraints> { /* ... */ }
    ```

  - ```rust
    fn table_type(self: &Self) -> TableType { /* ... */ }
    ```

  - ```rust
    fn scan<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, projection: Option<&''life2 Vec<usize>>, _filters: &''life3 [Expr], limit: Option<usize>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

  - ```rust
    fn insert_into<''life0, ''life1, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, input: Arc<dyn ExecutionPlan>, _insert_op: InsertOp) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **Send**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `StreamProvider`

The StreamProvider trait is used as a generic interface for reading and writing from streaming
data sources (such as FIFO, Websocket, Kafka, etc.).  Implementations of the provider are
responsible for providing a `RecordBatchReader` and optionally a `RecordBatchWriter`.

```rust
pub trait StreamProvider: std::fmt::Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `schema`: Get a reference to the schema for this stream
- `reader`: Provide `RecordBatchReader`
- `stream_write_display`: Display implementation when using as a DataSink

##### Provided Methods

- ```rust
  fn writer(self: &Self) -> Result<Box<dyn RecordBatchWriter>> { /* ... */ }
  ```
  Provide `RecordBatchWriter`

##### Implementations

This trait is implemented for the following types:

- `FileStreamProvider`

## Module `streaming`

A simplified [`TableProvider`] for streaming partitioned datasets

```rust
pub mod streaming { /* ... */ }
```

### Types

#### Struct `StreamingTable`

A [`TableProvider`] that streams a set of [`PartitionStream`]

```rust
pub struct StreamingTable {
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
  pub fn try_new(schema: SchemaRef, partitions: Vec<Arc<dyn PartitionStream>>) -> Result<Self> { /* ... */ }
  ```
  Try to create a new [`StreamingTable`] returning an error if the schema is incorrect

- ```rust
  pub fn with_infinite_table(self: Self, infinite: bool) -> Self { /* ... */ }
  ```
  Sets streaming table can be infinite.

###### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TableProvider**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

  - ```rust
    fn table_type(self: &Self) -> TableType { /* ... */ }
    ```

  - ```rust
    fn scan<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, _state: &''life1 dyn Session, projection: Option<&''life2 Vec<usize>>, _filters: &''life3 [Expr], limit: Option<usize>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `view`

View data source which uses a LogicalPlan as it's input.

```rust
pub mod view { /* ... */ }
```

### Types

#### Struct `ViewTable`

An implementation of `TableProvider` that uses another logical plan.

```rust
pub struct ViewTable {
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
  pub fn new(logical_plan: LogicalPlan, definition: Option<String>) -> Self { /* ... */ }
  ```
  Create new view that is executed at query runtime.

- ```rust
  pub fn try_new(logical_plan: LogicalPlan, definition: Option<String>) -> Result<Self> { /* ... */ }
  ```

- ```rust
  pub fn definition(self: &Self) -> Option<&String> { /* ... */ }
  ```
  Get definition ref

- ```rust
  pub fn logical_plan(self: &Self) -> &LogicalPlan { /* ... */ }
  ```
  Get logical_plan ref

###### Trait Implementations

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **IntoEither**
- **Send**
- **Freeze**
- **MaybeSendSync**
- **TableProvider**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn get_logical_plan(self: &Self) -> Option<Cow<''_, LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

  - ```rust
    fn table_type(self: &Self) -> TableType { /* ... */ }
    ```

  - ```rust
    fn get_table_definition(self: &Self) -> Option<&str> { /* ... */ }
    ```

  - ```rust
    fn supports_filters_pushdown(self: &Self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>> { /* ... */ }
    ```

  - ```rust
    fn scan<''life0, ''life1, ''life2, ''life3, ''async_trait>(self: &''life0 Self, state: &''life1 dyn Session, projection: Option<&''life2 Vec<usize>>, filters: &''life3 [Expr], limit: Option<usize>) -> ::core::pin::Pin<Box<dyn ::core::future::Future<Output = Result<Arc<dyn ExecutionPlan>>> + ::core::marker::Send + ''async_trait>>
where
    Self: ''async_trait,
    ''life0: ''async_trait,
    ''life1: ''async_trait,
    ''life2: ''async_trait,
    ''life3: ''async_trait { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Re-exports

### Re-export `Session`

```rust
pub use datafusion_session::Session;
```

### Re-export `MemTable`

```rust
pub use memory::MemTable;
```

### Re-export `MemoryCatalogProvider`

```rust
pub use memory::MemoryCatalogProvider;
```

### Re-export `MemoryCatalogProviderList`

```rust
pub use memory::MemoryCatalogProviderList;
```

### Re-export `MemorySchemaProvider`

```rust
pub use memory::MemorySchemaProvider;
```

### Re-export `catalog::*`

```rust
pub use catalog::*;
```

### Re-export `dynamic_file::catalog::*`

```rust
pub use dynamic_file::catalog::*;
```

### Re-export `async::*`

```rust
pub use async::*;
```

### Re-export `schema::*`

```rust
pub use schema::*;
```

### Re-export `table::*`

```rust
pub use table::*;
```


# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_physical_expr_common`

Physical Expr Common packages for [DataFusion]
This package contains high level PhysicalExpr trait

[DataFusion]: <https://crates.io/crates/datafusion>

## Modules

## Module `binary_map`

[`ArrowBytesMap`] and [`ArrowBytesSet`] for storing maps/sets of values from
StringArray / LargeStringArray / BinaryArray / LargeBinaryArray.

```rust
pub mod binary_map { /* ... */ }
```

### Types

#### Enum `OutputType`

Should the output be a String or Binary?

```rust
pub enum OutputType {
    Utf8,
    Utf8View,
    Binary,
    BinaryView,
}
```

##### Variants

###### `Utf8`

`StringArray` or `LargeStringArray`

###### `Utf8View`

`StringViewArray`

###### `Binary`

`BinaryArray` or `LargeBinaryArray`

###### `BinaryView`

`BinaryViewArray`

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Copy**
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &OutputType) -> bool { /* ... */ }
    ```

- **Eq**
- **ErasedDestructor**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> OutputType { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **MaybeSendSync**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
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

- **UnwindSafe**
- **Send**
- **Unpin**
- **Allocation**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `ArrowBytesSet`

HashSet optimized for storing string or binary values that can produce that
the final set as a GenericStringArray with minimal copies.

```rust
pub struct ArrowBytesSet<O: OffsetSizeTrait>(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(output_type: OutputType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn take(self: &mut Self) -> Self { /* ... */ }
  ```
  Return the contents of this set and replace it with a new empty

- ```rust
  pub fn insert(self: &mut Self, values: &ArrayRef) { /* ... */ }
  ```
  Inserts each value from `values` into the set

- ```rust
  pub fn into_state(self: Self) -> ArrayRef { /* ... */ }
  ```
  Converts this set into a `StringArray`/`LargeStringArray` or

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of distinct values (including nulls) seen so far

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn non_null_len(self: &Self) -> usize { /* ... */ }
  ```
  returns the total number of distinct values (not including nulls) seen so far

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Return the total size, in bytes, of memory used to store the data in

###### Trait Implementations

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
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
- **Unpin**
- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
#### Struct `ArrowBytesMap`

Optimized map for storing Arrow "bytes" types (`String`, `LargeString`,
`Binary`, and `LargeBinary`) values that can produce the set of keys on
output as `GenericBinaryArray` without copies.

Equivalent to `HashSet<String, V>` but with better performance if you need
to emit the keys as an Arrow `StringArray` / `BinaryArray`. For other
purposes it is the same as a `HashMap<String, V>`

# Generic Arguments

* `O`: OffsetSize (String/LargeString)
* `V`: payload type

# Description

This is a specialized HashMap with the following properties:

1. Optimized for storing and emitting Arrow byte types  (e.g.
   `StringArray` / `BinaryArray`) very efficiently by minimizing copying of
   the string values themselves, both when inserting and when emitting the
   final array.


2. Retains the insertion order of entries in the final array. The values are
   in the same order as they were inserted.

Note this structure can be used as a `HashSet` by specifying the value type
as `()`, as is done by [`ArrowBytesSet`].

This map is used by the special `COUNT DISTINCT` aggregate function to
store the distinct values, and by the `GROUP BY` operator to store
group values when they are a single string array.

# Example

The following diagram shows how the map would store the four strings
"Foo", NULL, "Bar", "TheQuickBrownFox":

* `hashtable` stores entries for each distinct string that has been
  inserted. The entries contain the payload as well as information about the
  value (either an offset or the actual bytes, see `Entry` docs for more
  details)

* `offsets` stores offsets into `buffer` for each distinct string value,
  following the same convention as the offsets in a `StringArray` or
  `LargeStringArray`.

* `buffer` stores the actual byte data

* `null`: stores the index and payload of the null value, in this case the
  second value (index 1)

```text
┌───────────────────────────────────┐    ┌─────┐    ┌────┐
│                ...                │    │  0  │    │FooB│
│ ┌──────────────────────────────┐  │    │  0  │    │arTh│
│ │      <Entry for "Bar">       │  │    │  3  │    │eQui│
│ │            len: 3            │  │    │  3  │    │ckBr│
│ │   offset_or_inline: "Bar"    │  │    │  6  │    │ownF│
│ │         payload:...          │  │    │     │    │ox  │
│ └──────────────────────────────┘  │    │     │    │    │
│                ...                │    └─────┘    └────┘
│ ┌──────────────────────────────┐  │
│ │<Entry for "TheQuickBrownFox">│  │    offsets    buffer
│ │           len: 16            │  │
│ │     offset_or_inline: 6      │  │    ┌───────────────┐
│ │         payload: ...         │  │    │    Some(1)    │
│ └──────────────────────────────┘  │    │ payload: ...  │
│                ...                │    └───────────────┘
└───────────────────────────────────┘
                                             null
              HashTable
```

# Entry Format

Entries stored in a [`ArrowBytesMap`] represents a value that is either
stored inline or in the buffer

This helps the case where there are many short (less than 8 bytes) strings
that are the same (e.g. "MA", "CA", "NY", "TX", etc)

```text
                                                               ┌──────────────────┐
                                                 ─ ─ ─ ─ ─ ─ ─▶│...               │
                                                │              │TheQuickBrownFox  │
                                                               │...               │
                                                │              │                  │
                                                               └──────────────────┘
                                                │               buffer of u8

                                                │
                       ┌────────────────┬───────────────┬───────────────┐
 Storing               │                │ starting byte │  length, in   │
 "TheQuickBrownFox"    │   hash value   │   offset in   │  bytes (not   │
 (long string)         │                │    buffer     │  characters)  │
                       └────────────────┴───────────────┴───────────────┘
                             8 bytes          8 bytes       4 or 8


                        ┌───────────────┬─┬─┬─┬─┬─┬─┬─┬─┬───────────────┐
Storing "foobar"        │               │ │ │ │ │ │ │ │ │  length, in   │
(short string)          │  hash value   │?│?│f│o│o│b│a│r│  bytes (not   │
                        │               │ │ │ │ │ │ │ │ │  characters)  │
                        └───────────────┴─┴─┴─┴─┴─┴─┴─┴─┴───────────────┘
                             8 bytes         8 bytes        4 or 8
```

```rust
pub struct ArrowBytesMap<O, V> {
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
  pub fn new(output_type: OutputType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn take(self: &mut Self) -> Self { /* ... */ }
  ```
  Return the contents of this map and replace it with a new empty map with

- ```rust
  pub fn insert_if_new<MP, OP>(self: &mut Self, values: &ArrayRef, make_payload_fn: MP, observe_payload_fn: OP)
where
    MP: FnMut(Option<&[u8]>) -> V,
    OP: FnMut(V) { /* ... */ }
  ```
  Inserts each value from `values` into the map, invoking `payload_fn` for

- ```rust
  pub fn into_state(self: Self) -> ArrayRef { /* ... */ }
  ```
  Converts this set into a `StringArray`, `LargeStringArray`,

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Total number of entries (including null, if present)

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Is the set empty?

- ```rust
  pub fn non_null_len(self: &Self) -> usize { /* ... */ }
  ```
  Number of non null entries

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Return the total size, in bytes, of memory used to store the data in

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
- **UnwindSafe**
- **Unpin**
- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Constants and Statics

#### Constant `INITIAL_BUFFER_CAPACITY`

The initial size, in bytes, of the string data

```rust
pub const INITIAL_BUFFER_CAPACITY: usize = _;
```

## Module `binary_view_map`

[`ArrowBytesViewMap`] and [`ArrowBytesViewSet`] for storing maps/sets of values from
`StringViewArray`/`BinaryViewArray`.
Much of the code is from `binary_map.rs`, but with simpler implementation because we directly use the
[`GenericByteViewBuilder`].

```rust
pub mod binary_view_map { /* ... */ }
```

### Types

#### Struct `ArrowBytesViewSet`

HashSet optimized for storing string or binary values that can produce that
the final set as a `GenericBinaryViewArray` with minimal copies.

```rust
pub struct ArrowBytesViewSet(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(output_type: OutputType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn insert(self: &mut Self, values: &ArrayRef) { /* ... */ }
  ```
  Inserts each value from `values` into the set

- ```rust
  pub fn take(self: &mut Self) -> Self { /* ... */ }
  ```
  Return the contents of this map and replace it with a new empty map with

- ```rust
  pub fn into_state(self: Self) -> ArrayRef { /* ... */ }
  ```
  Converts this set into a `StringViewArray` or `BinaryViewArray`

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the total number of distinct values (including nulls) seen so far

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn non_null_len(self: &Self) -> usize { /* ... */ }
  ```
  returns the total number of distinct values (not including nulls) seen so far

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Return the total size, in bytes, of memory used to store the data in

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **IntoEither**
- **ErasedDestructor**
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

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **MaybeSendSync**
#### Struct `ArrowBytesViewMap`

Optimized map for storing Arrow "byte view" types (`StringView`, `BinaryView`)
values that can produce the set of keys on
output as `GenericBinaryViewArray` without copies.

Equivalent to `HashSet<String, V>` but with better performance if you need
to emit the keys as an Arrow `StringViewArray` / `BinaryViewArray`. For other
purposes it is the same as a `HashMap<String, V>`

# Generic Arguments

* `V`: payload type

# Description

This is a specialized HashMap with the following properties:

1. Optimized for storing and emitting Arrow byte types  (e.g.
   `StringViewArray` / `BinaryViewArray`) very efficiently by minimizing copying of
   the string values themselves, both when inserting and when emitting the
   final array.

2. Retains the insertion order of entries in the final array. The values are
   in the same order as they were inserted.

Note this structure can be used as a `HashSet` by specifying the value type
as `()`, as is done by [`ArrowBytesViewSet`].

This map is used by the special `COUNT DISTINCT` aggregate function to
store the distinct values, and by the `GROUP BY` operator to store
group values when they are a single string array.

```rust
pub struct ArrowBytesViewMap<V> {
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
  pub fn new(output_type: OutputType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn take(self: &mut Self) -> Self { /* ... */ }
  ```
  Return the contents of this map and replace it with a new empty map with

- ```rust
  pub fn insert_if_new<MP, OP>(self: &mut Self, values: &ArrayRef, make_payload_fn: MP, observe_payload_fn: OP)
where
    MP: FnMut(Option<&[u8]>) -> V,
    OP: FnMut(V) { /* ... */ }
  ```
  Inserts each value from `values` into the map, invoking `payload_fn` for

- ```rust
  pub fn into_state(self: Self) -> ArrayRef { /* ... */ }
  ```
  Converts this set into a `StringViewArray`, or `BinaryViewArray`,

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Total number of entries (including null, if present)

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Is the set empty?

- ```rust
  pub fn non_null_len(self: &Self) -> usize { /* ... */ }
  ```
  Number of non null entries

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Return the total size, in bytes, of memory used to store the data in

###### Trait Implementations

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `datum`

```rust
pub mod datum { /* ... */ }
```

### Functions

#### Function `apply`

Applies a binary [`Datum`] kernel `f` to `lhs` and `rhs`

This maps arrow-rs' [`Datum`] kernels to DataFusion's [`ColumnarValue`] abstraction

```rust
pub fn apply</* synthetic */ impl Fn(&dyn Datum, &dyn Datum) -> Result<ArrayRef, ArrowError>: Fn(&dyn Datum, &dyn Datum) -> datafusion_common::Result<arrow::array::ArrayRef, arrow::error::ArrowError>>(lhs: &datafusion_expr_common::columnar_value::ColumnarValue, rhs: &datafusion_expr_common::columnar_value::ColumnarValue, f: impl Fn(&dyn Datum, &dyn Datum) -> datafusion_common::Result<arrow::array::ArrayRef, arrow::error::ArrowError>) -> datafusion_common::Result<datafusion_expr_common::columnar_value::ColumnarValue> { /* ... */ }
```

#### Function `apply_cmp`

Applies a binary [`Datum`] comparison kernel `f` to `lhs` and `rhs`

```rust
pub fn apply_cmp</* synthetic */ impl Fn(&dyn Datum, &dyn Datum) -> Result<BooleanArray, ArrowError>: Fn(&dyn Datum, &dyn Datum) -> datafusion_common::Result<arrow::array::BooleanArray, arrow::error::ArrowError>>(lhs: &datafusion_expr_common::columnar_value::ColumnarValue, rhs: &datafusion_expr_common::columnar_value::ColumnarValue, f: impl Fn(&dyn Datum, &dyn Datum) -> datafusion_common::Result<arrow::array::BooleanArray, arrow::error::ArrowError>) -> datafusion_common::Result<datafusion_expr_common::columnar_value::ColumnarValue> { /* ... */ }
```

#### Function `apply_cmp_for_nested`

Applies a binary [`Datum`] comparison kernel `f` to `lhs` and `rhs` for nested type like
List, FixedSizeList, LargeList, Struct, Union, Map, or a dictionary of a nested type

```rust
pub fn apply_cmp_for_nested(op: datafusion_expr_common::operator::Operator, lhs: &datafusion_expr_common::columnar_value::ColumnarValue, rhs: &datafusion_expr_common::columnar_value::ColumnarValue) -> datafusion_common::Result<datafusion_expr_common::columnar_value::ColumnarValue> { /* ... */ }
```

#### Function `compare_with_eq`

Compare with eq with either nested or non-nested

```rust
pub fn compare_with_eq(lhs: &dyn Datum, rhs: &dyn Datum, is_nested: bool) -> datafusion_common::Result<arrow::array::BooleanArray> { /* ... */ }
```

#### Function `compare_op_for_nested`

Compare on nested type List, Struct, and so on

```rust
pub fn compare_op_for_nested(op: datafusion_expr_common::operator::Operator, lhs: &dyn Datum, rhs: &dyn Datum) -> datafusion_common::Result<arrow::array::BooleanArray> { /* ... */ }
```

## Module `physical_expr`

```rust
pub mod physical_expr { /* ... */ }
```

### Types

#### Type Alias `PhysicalExprRef`

Shared [`PhysicalExpr`].

```rust
pub type PhysicalExprRef = std::sync::Arc<dyn PhysicalExpr>;
```

### Traits

#### Trait `PhysicalExpr`

[`PhysicalExpr`]s represent expressions such as `A + 1` or `CAST(c1 AS int)`.

`PhysicalExpr` knows its type, nullability and can be evaluated directly on
a [`RecordBatch`] (see [`Self::evaluate`]).

`PhysicalExpr` are the physical counterpart to [`Expr`] used in logical
planning. They are typically created from [`Expr`] by a [`PhysicalPlanner`]
invoked from a higher level API

Some important examples of `PhysicalExpr` are:
* [`Column`]: Represents a column at a given index in a RecordBatch

To create `PhysicalExpr` from  `Expr`, see
* [`SessionContext::create_physical_expr`]: A high level API
* [`create_physical_expr`]: A low level API

# Formatting `PhysicalExpr` as strings
There are three ways to format `PhysicalExpr` as a string:
* [`Debug`]: Standard Rust debugging format (e.g. `Constant { value: ... }`)
* [`Display`]: Detailed SQL-like format that shows expression structure (e.g. (`Utf8 ("foobar")`). This is often used for debugging and tests
* [`Self::fmt_sql`]: SQL-like human readable format (e.g. ('foobar')`), See also [`sql_fmt`]

[`SessionContext::create_physical_expr`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.create_physical_expr
[`PhysicalPlanner`]: https://docs.rs/datafusion/latest/datafusion/physical_planner/trait.PhysicalPlanner.html
[`Expr`]: https://docs.rs/datafusion/latest/datafusion/logical_expr/enum.Expr.html
[`create_physical_expr`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/fn.create_physical_expr.html
[`Column`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/expressions/struct.Column.html

```rust
pub trait PhysicalExpr: Send + Sync + Display + Debug + DynEq + DynHash {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `as_any`: Returns the physical expression as [`Any`] so that it can be
- `data_type`: Get the data type of this expression, given the schema of the input
- `nullable`: Determine whether this expression is nullable, given the schema of the input
- `evaluate`: Evaluate an expression against a RecordBatch
- `children`: Get a list of child PhysicalExpr that provide the input for this expr.
- `with_new_children`: Returns a new PhysicalExpr where all children were replaced by new exprs.
- `fmt_sql`: Format this `PhysicalExpr` in nice human readable "SQL" format

##### Provided Methods

- ```rust
  fn evaluate_selection(self: &Self, batch: &RecordBatch, selection: &BooleanArray) -> Result<ColumnarValue> { /* ... */ }
  ```
  Evaluate an expression against a RecordBatch after first applying a

- ```rust
  fn evaluate_bounds(self: &Self, _children: &[&Interval]) -> Result<Interval> { /* ... */ }
  ```
  Computes the output interval for the expression, given the input

- ```rust
  fn propagate_constraints(self: &Self, _interval: &Interval, _children: &[&Interval]) -> Result<Option<Vec<Interval>>> { /* ... */ }
  ```
  Updates bounds for child expressions, given a known interval for this

- ```rust
  fn evaluate_statistics(self: &Self, children: &[&Distribution]) -> Result<Distribution> { /* ... */ }
  ```
  Computes the output statistics for the expression, given the input

- ```rust
  fn propagate_statistics(self: &Self, parent: &Distribution, children: &[&Distribution]) -> Result<Option<Vec<Distribution>>> { /* ... */ }
  ```
  Updates children statistics using the given parent statistic for this

- ```rust
  fn get_properties(self: &Self, _children: &[ExprProperties]) -> Result<ExprProperties> { /* ... */ }
  ```
  Calculates the properties of this [`PhysicalExpr`] based on its

- ```rust
  fn snapshot(self: &Self) -> Result<Option<Arc<dyn PhysicalExpr>>> { /* ... */ }
  ```
  Take a snapshot of this `PhysicalExpr`, if it is dynamic.

#### Trait `DynEq`

[`PhysicalExpr`] can't be constrained by [`Eq`] directly because it must remain object
safe. To ease implementation, blanket implementation is provided for [`Eq`] types.

```rust
pub trait DynEq {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `dyn_eq`

##### Implementations

This trait is implemented for the following types:

- `T` with <T: Eq + Any>

#### Trait `DynHash`

[`PhysicalExpr`] can't be constrained by [`Hash`] directly because it must remain
object safe. To ease implementation blanket implementation is provided for [`Hash`]
types.

```rust
pub trait DynHash {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `dyn_hash`

##### Implementations

This trait is implemented for the following types:

- `T` with <T: Hash + Any>

### Functions

#### Function `with_new_children_if_necessary`

Returns a copy of this expr if we change any child according to the pointer comparison.
The size of `children` must be equal to the size of `PhysicalExpr::children()`.

```rust
pub fn with_new_children_if_necessary(expr: std::sync::Arc<dyn PhysicalExpr>, children: Vec<std::sync::Arc<dyn PhysicalExpr>>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

#### Function `down_cast_any_ref`

**Attributes:**

- `#[deprecated(since = "44.0.0")]`

**⚠️ Deprecated since 44.0.0**

```rust
pub fn down_cast_any_ref(any: &dyn Any) -> &dyn Any { /* ... */ }
```

#### Function `format_physical_expr_list`

Returns [`Display`] able a list of [`PhysicalExpr`]

Example output: `[a + 1, b]`

```rust
pub fn format_physical_expr_list<T>(exprs: T) -> impl Display
where
    T: IntoIterator,
    <T as >::Item: Display,
    <T as >::IntoIter: Clone { /* ... */ }
```

#### Function `fmt_sql`

Prints a [`PhysicalExpr`] in a SQL-like format

# Example
```
# // The boiler plate needed to create a `PhysicalExpr` for the example
# use std::any::Any;
# use std::fmt::Formatter;
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use arrow::datatypes::{DataType, Schema};
# use datafusion_common::Result;
# use datafusion_expr_common::columnar_value::ColumnarValue;
# use datafusion_physical_expr_common::physical_expr::{fmt_sql, DynEq, PhysicalExpr};
# #[derive(Debug, Hash, PartialOrd, PartialEq)]
# struct MyExpr {};
# impl PhysicalExpr for MyExpr {fn as_any(&self) -> &dyn Any { unimplemented!() }
# fn data_type(&self, input_schema: &Schema) -> Result<DataType> { unimplemented!() }
# fn nullable(&self, input_schema: &Schema) -> Result<bool> { unimplemented!() }
# fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue> { unimplemented!() }
# fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>>{ unimplemented!() }
# fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>> { unimplemented!() }
# fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result { write!(f, "CASE a > b THEN 1 ELSE 0 END") }
# }
# impl std::fmt::Display for MyExpr {fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result { unimplemented!() } }
# impl DynEq for MyExpr {fn dyn_eq(&self, other: &dyn Any) -> bool { unimplemented!() } }
# fn make_physical_expr() -> Arc<dyn PhysicalExpr> { Arc::new(MyExpr{}) }
let expr: Arc<dyn PhysicalExpr> = make_physical_expr();
// wrap the expression in `sql_fmt` which can be used with
// `format!`, `to_string()`, etc
let expr_as_sql = fmt_sql(expr.as_ref());
assert_eq!(
  "The SQL: CASE a > b THEN 1 ELSE 0 END",
  format!("The SQL: {expr_as_sql}")
);
```

```rust
pub fn fmt_sql(expr: &dyn PhysicalExpr) -> impl Display + ''_ { /* ... */ }
```

#### Function `snapshot_physical_expr`

Take a snapshot of the given `PhysicalExpr` if it is dynamic.

Take a snapshot of this `PhysicalExpr` if it is dynamic.
This is used to capture the current state of `PhysicalExpr`s that may contain
dynamic references to other operators in order to serialize it over the wire
or treat it via downcast matching.

See the documentation of [`PhysicalExpr::snapshot`] for more details.

# Returns

Returns an `Option<Arc<dyn PhysicalExpr>>` which is the snapshot of the
`PhysicalExpr` if it is dynamic. If the `PhysicalExpr` does not have
any dynamic references or state, it returns `None`.

```rust
pub fn snapshot_physical_expr(expr: std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

## Module `sort_expr`

Sort expressions

```rust
pub mod sort_expr { /* ... */ }
```

### Types

#### Struct `PhysicalSortExpr`

Represents Sort operation for a column in a RecordBatch

Example:
```
# use std::any::Any;
# use std::fmt::{Display, Formatter};
# use std::hash::Hasher;
# use std::sync::Arc;
# use arrow::array::RecordBatch;
# use datafusion_common::Result;
# use arrow::compute::SortOptions;
# use arrow::datatypes::{DataType, Schema};
# use datafusion_expr_common::columnar_value::ColumnarValue;
# use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
# use datafusion_physical_expr_common::sort_expr::PhysicalSortExpr;
# // this crate doesn't have a physical expression implementation
# // so make a really simple one
# #[derive(Clone, Debug, PartialEq, Eq, Hash)]
# struct MyPhysicalExpr;
# impl PhysicalExpr for MyPhysicalExpr {
#  fn as_any(&self) -> &dyn Any {todo!() }
#  fn data_type(&self, input_schema: &Schema) -> Result<DataType> {todo!()}
#  fn nullable(&self, input_schema: &Schema) -> Result<bool> {todo!() }
#  fn evaluate(&self, batch: &RecordBatch) -> Result<ColumnarValue> {todo!() }
#  fn children(&self) -> Vec<&Arc<dyn PhysicalExpr>> {todo!()}
#  fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn PhysicalExpr>>) -> Result<Arc<dyn PhysicalExpr>> {todo!()}
# fn fmt_sql(&self, f: &mut Formatter<'_>) -> std::fmt::Result { todo!() }
# }
# impl Display for MyPhysicalExpr {
#    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result { write!(f, "a") }
# }
# fn col(name: &str) -> Arc<dyn PhysicalExpr> { Arc::new(MyPhysicalExpr) }
// Sort by a ASC
let options = SortOptions::default();
let sort_expr = PhysicalSortExpr::new(col("a"), options);
assert_eq!(sort_expr.to_string(), "a ASC");

// Sort by a DESC NULLS LAST
let sort_expr = PhysicalSortExpr::new_default(col("a"))
  .desc()
  .nulls_last();
assert_eq!(sort_expr.to_string(), "a DESC NULLS LAST");
```

```rust
pub struct PhysicalSortExpr {
    pub expr: std::sync::Arc<dyn PhysicalExpr>,
    pub options: arrow::compute::kernels::sort::SortOptions,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `std::sync::Arc<dyn PhysicalExpr>` | Physical expression representing the column to sort |
| `options` | `arrow::compute::kernels::sort::SortOptions` | Option to specify how the given column should be sorted |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Arc<dyn PhysicalExpr>, options: SortOptions) -> Self { /* ... */ }
  ```
  Create a new PhysicalSortExpr

- ```rust
  pub fn new_default(expr: Arc<dyn PhysicalExpr>) -> Self { /* ... */ }
  ```
  Create a new PhysicalSortExpr with default [`SortOptions`]

- ```rust
  pub fn asc(self: Self) -> Self { /* ... */ }
  ```
  Set the sort sort options to ASC

- ```rust
  pub fn desc(self: Self) -> Self { /* ... */ }
  ```
  Set the sort sort options to DESC

- ```rust
  pub fn nulls_first(self: Self) -> Self { /* ... */ }
  ```
  Set the sort sort options to NULLS FIRST

- ```rust
  pub fn nulls_last(self: Self) -> Self { /* ... */ }
  ```
  Set the sort sort options to NULLS LAST

- ```rust
  pub fn fmt_sql(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
  ```
  Like [`PhysicalExpr::fmt_sql`] prints a [`PhysicalSortExpr`] in a SQL-like format.

- ```rust
  pub fn evaluate_to_sort_column(self: &Self, batch: &RecordBatch) -> Result<SortColumn> { /* ... */ }
  ```
  evaluate the sort expression into SortColumn that can be passed into arrow sort kernel

- ```rust
  pub fn satisfy(self: &Self, requirement: &PhysicalSortRequirement, schema: &Schema) -> bool { /* ... */ }
  ```
  Checks whether this sort expression satisfies the given `requirement`.

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PhysicalSortExpr { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PhysicalSortExpr) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &dyn PhysicalExpr + ''static { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **MaybeSendSync**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = PhysicalSortExpr>>(iter: T) -> Self { /* ... */ }
    ```

- **Sync**
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
    fn from(value: PhysicalSortRequirement) -> Self { /* ... */ }
    ```
    If options is `None`, the default sort options `ASC, NULLS LAST` is used.

  - ```rust
    fn from(value: PhysicalSortExpr) -> Self { /* ... */ }
    ```

- **ErasedDestructor**
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

#### Struct `PhysicalSortRequirement`

Represents sort requirement associated with a plan

If the requirement includes [`SortOptions`] then both the
expression *and* the sort options must match.

If the requirement does not include [`SortOptions`]) then only the
expressions must match.

# Examples

With sort options (`A`, `DESC NULLS FIRST`):
* `ORDER BY A DESC NULLS FIRST` matches
* `ORDER BY A ASC  NULLS FIRST` does not match (`ASC` vs `DESC`)
* `ORDER BY B DESC NULLS FIRST` does not match (different expr)

Without sort options (`A`, None):
* `ORDER BY A DESC NULLS FIRST` matches
* `ORDER BY A ASC  NULLS FIRST` matches (`ASC` and `NULL` options ignored)
* `ORDER BY B DESC NULLS FIRST` does not match  (different expr)

```rust
pub struct PhysicalSortRequirement {
    pub expr: std::sync::Arc<dyn PhysicalExpr>,
    pub options: Option<arrow::compute::kernels::sort::SortOptions>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `std::sync::Arc<dyn PhysicalExpr>` | Physical expression representing the column to sort |
| `options` | `Option<arrow::compute::kernels::sort::SortOptions>` | Option to specify how the given column should be sorted.<br>If unspecified, there are no constraints on sort options. |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Arc<dyn PhysicalExpr>, options: Option<SortOptions>) -> Self { /* ... */ }
  ```
  Creates a new requirement.

- ```rust
  pub fn with_expr(self: Self, expr: Arc<dyn PhysicalExpr>) -> Self { /* ... */ }
  ```
  Replace the required expression for this requirement with the new one

- ```rust
  pub fn compatible(self: &Self, other: &PhysicalSortRequirement) -> bool { /* ... */ }
  ```
  Returns whether this requirement is equal or more specific than `other`.

- ```rust
  pub fn from_sort_exprs<''a, /* synthetic */ impl IntoIterator<Item = &'a PhysicalSortExpr>: IntoIterator<Item = &''a PhysicalSortExpr>>(ordering: impl IntoIterator<Item = &''a PhysicalSortExpr>) -> LexRequirement { /* ... */ }
  ```

- ```rust
  pub fn to_sort_exprs</* synthetic */ impl IntoIterator<Item = PhysicalSortRequirement>: IntoIterator<Item = PhysicalSortRequirement>>(requirements: impl IntoIterator<Item = PhysicalSortRequirement>) -> LexOrdering { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: PhysicalSortRequirement) -> Self { /* ... */ }
    ```
    If options is `None`, the default sort options `ASC, NULLS LAST` is used.

  - ```rust
    fn from(value: PhysicalSortExpr) -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = PhysicalSortRequirement>>(iter: T) -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PhysicalSortRequirement { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PhysicalSortRequirement) -> bool { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **UnwindSafe**
- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `LexOrdering`

`LexOrdering` contains a `Vec<PhysicalSortExpr>`, which represents
 a lexicographical ordering.

 For example, `vec![a ASC, b DESC]` represents a lexicographical ordering
 that first sorts by column `a` in ascending order, then by column `b` in
 descending order.

```rust
pub struct LexOrdering {
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
  pub fn new(inner: Vec<PhysicalSortExpr>) -> Self { /* ... */ }
  ```
  Creates a new [`LexOrdering`] from a vector

- ```rust
  pub fn empty() -> &''static LexOrdering { /* ... */ }
  ```
  Return an empty LexOrdering (no expressions)

- ```rust
  pub fn capacity(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements that can be stored in the LexOrdering

- ```rust
  pub fn clear(self: &mut Self) { /* ... */ }
  ```
  Clears the LexOrdering, removing all elements.

- ```rust
  pub fn take_exprs(self: Self) -> Vec<PhysicalSortExpr> { /* ... */ }
  ```
  Takes ownership of the actual vector of `PhysicalSortExpr`s in the LexOrdering.

- ```rust
  pub fn contains(self: &Self, expr: &PhysicalSortExpr) -> bool { /* ... */ }
  ```
  Returns `true` if the LexOrdering contains `expr`

- ```rust
  pub fn extend<I: IntoIterator<Item = PhysicalSortExpr>>(self: &mut Self, iter: I) { /* ... */ }
  ```
  Add all elements from `iter` to the LexOrdering.

- ```rust
  pub fn retain<F>(self: &mut Self, f: F)
where
    F: FnMut(&PhysicalSortExpr) -> bool { /* ... */ }
  ```
  Remove all elements from the LexOrdering where `f` evaluates to `false`.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the LexOrdering contains no elements.

- ```rust
  pub fn iter(self: &Self) -> core::slice::Iter<''_, PhysicalSortExpr> { /* ... */ }
  ```
  Returns an iterator over each `&PhysicalSortExpr` in the LexOrdering.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements in the LexOrdering.

- ```rust
  pub fn pop(self: &mut Self) -> Option<PhysicalSortExpr> { /* ... */ }
  ```
  Removes the last element from the LexOrdering and returns it, or `None` if it is empty.

- ```rust
  pub fn push(self: &mut Self, physical_sort_expr: PhysicalSortExpr) { /* ... */ }
  ```
  Appends an element to the back of the LexOrdering.

- ```rust
  pub fn truncate(self: &mut Self, len: usize) { /* ... */ }
  ```
  Truncates the LexOrdering, keeping only the first `len` elements.

- ```rust
  pub fn merge(self: Self, other: LexOrdering) -> Self { /* ... */ }
  ```
  Merge the contents of `other` into `self`, removing duplicates.

- ```rust
  pub fn from_lex_requirement(requirement: LexRequirement) -> LexOrdering { /* ... */ }
  ```
  Converts a `LexRequirement` into a `LexOrdering`.

- ```rust
  pub fn collapse(self: Self) -> Self { /* ... */ }
  ```
  Collapse a `LexOrdering` into a new duplicate-free `LexOrdering` based on expression.

- ```rust
  pub fn transform<F>(self: &mut Self, f: F)
where
    F: FnMut(&mut PhysicalSortExpr) { /* ... */ }
  ```
  Transforms each `PhysicalSortExpr` in the `LexOrdering`

###### Trait Implementations

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> LexOrdering { /* ... */ }
    ```

- **Index**
  - ```rust
    fn index(self: &Self, index: usize) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range: Range<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range_from: RangeFrom<usize>) -> &<Self as >::Output { /* ... */ }
    ```

  - ```rust
    fn index(self: &Self, range_to: RangeTo<usize>) -> &<Self as >::Output { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
- **UnwindSafe**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **AsRef**
  - ```rust
    fn as_ref(self: &Self) -> &LexOrdering { /* ... */ }
    ```

- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> LexOrdering { /* ... */ }
    ```

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LexOrdering) -> bool { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Receiver**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: Vec<PhysicalSortExpr>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: LexRequirement) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: LexOrdering) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: LexOrdering) -> Self { /* ... */ }
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

- **Sync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

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

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = PhysicalSortExpr>>(iter: T) -> Self { /* ... */ }
    ```

#### Type Alias `LexOrderingRef`

**Attributes:**

- `#[deprecated(since = "43.0.0", note = "use &LexOrdering instead")]`

**⚠️ Deprecated since 43.0.0**: use &LexOrdering instead

`LexOrderingRef` is an alias for the type &`[PhysicalSortExpr]`, which represents
 a reference to a lexicographical ordering.

```rust
pub type LexOrderingRef<''a> = &''a [PhysicalSortExpr];
```

#### Struct `LexRequirement`

`LexRequirement` is an struct containing a `Vec<PhysicalSortRequirement>`, which
 represents a lexicographical ordering requirement.

```rust
pub struct LexRequirement {
    pub inner: Vec<PhysicalSortRequirement>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `inner` | `Vec<PhysicalSortRequirement>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(inner: Vec<PhysicalSortRequirement>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn iter(self: &Self) -> impl Iterator<Item = &PhysicalSortRequirement> { /* ... */ }
  ```

- ```rust
  pub fn push(self: &mut Self, physical_sort_requirement: PhysicalSortRequirement) { /* ... */ }
  ```

- ```rust
  pub fn from_lex_ordering(ordering: LexOrdering) -> Self { /* ... */ }
  ```
  Create a new [`LexRequirement`] from a [`LexOrdering`]

- ```rust
  pub fn collapse(self: Self) -> Self { /* ... */ }
  ```
  Constructs a duplicate-free `LexOrderingReq` by filtering out

###### Trait Implementations

- **FromIterator**
  - ```rust
    fn from_iter<T: IntoIterator<Item = PhysicalSortRequirement>>(iter: T) -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Receiver**
- **Deref**
  - ```rust
    fn deref(self: &Self) -> &<Self as >::Target { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> LexRequirement { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **IntoIterator**
  - ```rust
    fn into_iter(self: Self) -> <Self as >::IntoIter { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> LexRequirement { /* ... */ }
    ```

- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &LexRequirement) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: LexRequirement) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: LexOrdering) -> Self { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
#### Type Alias `LexRequirementRef`

`LexRequirementRef` is an alias for the type &`[PhysicalSortRequirement]`, which
 represents a reference to a lexicographical ordering requirement.
 #[deprecated(since = "43.0.0", note = "use &LexRequirement instead")]

```rust
pub type LexRequirementRef<''a> = &''a [PhysicalSortRequirement];
```

### Functions

#### Function `format_physical_sort_requirement_list`

Writes a list of [`PhysicalSortRequirement`]s to a `std::fmt::Formatter`.

Example output: `[a + 1, b]`

```rust
pub fn format_physical_sort_requirement_list(exprs: &[PhysicalSortRequirement]) -> impl Display + ''_ { /* ... */ }
```

## Module `tree_node`

This module provides common traits for visiting or rewriting tree nodes easily.

```rust
pub mod tree_node { /* ... */ }
```

### Types

#### Struct `ExprContext`

A node object encapsulating a [`PhysicalExpr`] node with a payload. Since there are
two ways to access child plans—directly from the plan  and through child nodes—it's
recommended to perform mutable operations via [`Self::update_expr_from_children`].

```rust
pub struct ExprContext<T: Sized> {
    pub expr: std::sync::Arc<dyn PhysicalExpr>,
    pub data: T,
    pub children: Vec<Self>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `std::sync::Arc<dyn PhysicalExpr>` | The physical expression associated with this context. |
| `data` | `T` | Custom data payload of the node. |
| `children` | `Vec<Self>` | Child contexts of this node. |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Arc<dyn PhysicalExpr>, data: T, children: Vec<Self>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn update_expr_from_children(self: Self) -> Result<Self> { /* ... */ }
  ```

- ```rust
  pub fn new_default(plan: Arc<dyn PhysicalExpr>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn new_unknown(expr: Arc<dyn PhysicalExpr>) -> Self { /* ... */ }
  ```
  Constructs a new `ExprPropertiesNode` with unknown properties for a

###### Trait Implementations

- **TreeNode**
  - ```rust
    fn apply_children<''n, F>(self: &''n Self, f: F) -> Result<TreeNodeRecursion, DataFusionError>
where
    F: FnMut(&''n T) -> Result<TreeNodeRecursion, DataFusionError> { /* ... */ }
    ```

  - ```rust
    fn map_children<F>(self: Self, f: F) -> Result<Transformed<T>, DataFusionError>
where
    F: FnMut(T) -> Result<Transformed<T>, DataFusionError> { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **ConcreteTreeNode**
  - ```rust
    fn children(self: &Self) -> &[Self] { /* ... */ }
    ```

  - ```rust
    fn take_children(self: Self) -> (Self, Vec<Self>) { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Self, children: Vec<Self>) -> Result<Self> { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `utils`

```rust
pub mod utils { /* ... */ }
```

### Types

#### Type Alias `ExprPropertiesNode`

Represents a [`PhysicalExpr`] node with associated properties (order and
range) in a context where properties are tracked.

```rust
pub type ExprPropertiesNode = crate::tree_node::ExprContext<datafusion_expr_common::sort_properties::ExprProperties>;
```

### Functions

#### Function `scatter`

Scatter `truthy` array by boolean mask. When the mask evaluates `true`, next values of `truthy`
are taken, when the mask evaluates `false` values null values are filled.

# Arguments
* `mask` - Boolean values used to determine where to put the `truthy` values
* `truthy` - All values of this array are to scatter according to `mask` into final result.

```rust
pub fn scatter(mask: &arrow::array::BooleanArray, truthy: &dyn Array) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `reverse_order_bys`

Reverses the ORDER BY expression, which is useful during equivalent window
expression construction. For instance, 'ORDER BY a ASC, NULLS LAST' turns into
'ORDER BY a DESC, NULLS FIRST'.

```rust
pub fn reverse_order_bys(order_bys: &crate::sort_expr::LexOrdering) -> crate::sort_expr::LexOrdering { /* ... */ }
```


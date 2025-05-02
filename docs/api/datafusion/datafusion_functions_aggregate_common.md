# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_functions_aggregate_common`

Common Aggregate functionality for [DataFusion]

This crate contains traits and utilities commonly used to implement aggregate functions
They are kept in their own crate to avoid physical expressions depending on logical expressions.

[DataFusion]: <https://crates.io/crates/datafusion>

## Modules

## Module `accumulator`

```rust
pub mod accumulator { /* ... */ }
```

### Types

#### Struct `AccumulatorArgs`

[`AccumulatorArgs`] contains information about how an aggregate
function was called, including the types of its arguments and any optional
ordering expressions.

```rust
pub struct AccumulatorArgs<''a> {
    pub return_type: &''a arrow::datatypes::DataType,
    pub schema: &''a arrow::datatypes::Schema,
    pub ignore_nulls: bool,
    pub ordering_req: &''a datafusion_physical_expr_common::sort_expr::LexOrdering,
    pub is_reversed: bool,
    pub name: &''a str,
    pub is_distinct: bool,
    pub exprs: &''a [std::sync::Arc<dyn PhysicalExpr>],
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `return_type` | `&''a arrow::datatypes::DataType` | The return type of the aggregate function. |
| `schema` | `&''a arrow::datatypes::Schema` | The schema of the input arguments |
| `ignore_nulls` | `bool` | Whether to ignore nulls.<br><br>SQL allows the user to specify `IGNORE NULLS`, for example:<br><br>```sql<br>SELECT FIRST_VALUE(column1) IGNORE NULLS FROM t;<br>``` |
| `ordering_req` | `&''a datafusion_physical_expr_common::sort_expr::LexOrdering` | The expressions in the `ORDER BY` clause passed to this aggregator.<br><br>SQL allows the user to specify the ordering of arguments to the<br>aggregate using an `ORDER BY`. For example:<br><br>```sql<br>SELECT FIRST_VALUE(column1 ORDER BY column2) FROM t;<br>```<br><br>If no `ORDER BY` is specified, `ordering_req` will be empty. |
| `is_reversed` | `bool` | Whether the aggregation is running in reverse order |
| `name` | `&''a str` | The name of the aggregate expression |
| `is_distinct` | `bool` | Whether the aggregate function is distinct.<br><br>```sql<br>SELECT COUNT(DISTINCT column1) FROM t;<br>``` |
| `exprs` | `&''a [std::sync::Arc<dyn PhysicalExpr>]` | The physical expression of arguments the aggregate function takes. |

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **RefUnwindSafe**
- **Unpin**
- **ErasedDestructor**
- **Sync**
- **Freeze**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Type Alias `AccumulatorFactoryFunction`

Factory that returns an accumulator for the given aggregate function.

```rust
pub type AccumulatorFactoryFunction = std::sync::Arc<dyn Fn(AccumulatorArgs<''_>) -> datafusion_common::Result<Box<dyn Accumulator>> + Send + Sync>;
```

#### Struct `StateFieldsArgs`

[`StateFieldsArgs`] contains information about the fields that an
aggregate function's accumulator should have. Used for `AggregateUDFImpl::state_fields`.

```rust
pub struct StateFieldsArgs<''a> {
    pub name: &''a str,
    pub input_types: &''a [arrow::datatypes::DataType],
    pub return_type: &''a arrow::datatypes::DataType,
    pub ordering_fields: &''a [arrow::datatypes::Field],
    pub is_distinct: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `&''a str` | The name of the aggregate function. |
| `input_types` | `&''a [arrow::datatypes::DataType]` | The input types of the aggregate function. |
| `return_type` | `&''a arrow::datatypes::DataType` | The return type of the aggregate function. |
| `ordering_fields` | `&''a [arrow::datatypes::Field]` | The ordering fields of the aggregate function. |
| `is_distinct` | `bool` | Whether the aggregate function is distinct. |

##### Implementations

###### Trait Implementations

- **IntoEither**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Allocation**
- **Freeze**
- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `aggregate`

```rust
pub mod aggregate { /* ... */ }
```

### Modules

## Module `count_distinct`

```rust
pub mod count_distinct { /* ... */ }
```

### Re-exports

#### Re-export `BytesDistinctCountAccumulator`

```rust
pub use bytes::BytesDistinctCountAccumulator;
```

#### Re-export `BytesViewDistinctCountAccumulator`

```rust
pub use bytes::BytesViewDistinctCountAccumulator;
```

#### Re-export `FloatDistinctCountAccumulator`

```rust
pub use native::FloatDistinctCountAccumulator;
```

#### Re-export `PrimitiveDistinctCountAccumulator`

```rust
pub use native::PrimitiveDistinctCountAccumulator;
```

## Module `groups_accumulator`

Utilities for implementing GroupsAccumulator
Adapter that makes [`GroupsAccumulator`] out of [`Accumulator`]

```rust
pub mod groups_accumulator { /* ... */ }
```

### Modules

## Module `accumulate`

[`GroupsAccumulator`] helpers: [`NullState`] and [`accumulate_indices`]

[`GroupsAccumulator`]: datafusion_expr_common::groups_accumulator::GroupsAccumulator

```rust
pub mod accumulate { /* ... */ }
```

### Types

#### Struct `NullState`

Track the accumulator null state per row: if any values for that
group were null and if any values have been seen at all for that group.

This is part of the inner loop for many [`GroupsAccumulator`]s,
and thus the performance is critical and so there are multiple
specialized implementations, invoked depending on the specific
combinations of the input.

Typically there are 4 potential combinations of inputs must be
special cased for performance:

* With / Without filter
* With / Without nulls in the input

If the input has nulls, then the accumulator must potentially
handle each input null value specially (e.g. for `SUM` to mark the
corresponding sum as null)

If there are filters present, `NullState` tracks if it has seen
*any* value for that group (as some values may be filtered
out). Without a filter, the accumulator is only passed groups that
had at least one value to accumulate so they do not need to track
if they have seen values for a particular group.

[`GroupsAccumulator`]: datafusion_expr_common::groups_accumulator::GroupsAccumulator

```rust
pub struct NullState {
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

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  return the size of all buffers allocated by this null state, not including self

- ```rust
  pub fn accumulate<T, F>(self: &mut Self, group_indices: &[usize], values: &PrimitiveArray<T>, opt_filter: Option<&BooleanArray>, total_num_groups: usize, value_fn: F)
where
    T: ArrowPrimitiveType + Send,
    F: FnMut(usize, <T as >::Native) + Send { /* ... */ }
  ```
  Invokes `value_fn(group_index, value)` for each non null, non

- ```rust
  pub fn accumulate_boolean<F>(self: &mut Self, group_indices: &[usize], values: &BooleanArray, opt_filter: Option<&BooleanArray>, total_num_groups: usize, value_fn: F)
where
    F: FnMut(usize, bool) + Send { /* ... */ }
  ```
  Invokes `value_fn(group_index, value)` for each non null, non

- ```rust
  pub fn build(self: &mut Self, emit_to: EmitTo) -> NullBuffer { /* ... */ }
  ```
  Creates the a [`NullBuffer`] representing which group_indices

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Send**
- **ErasedDestructor**
- **MaybeSendSync**
- **IntoEither**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
### Functions

#### Function `accumulate`

Invokes `value_fn(group_index, value)` for each non null, non
filtered value of `value`,

# Arguments:

* `group_indices`:  To which groups do the rows in `values` belong, (aka group_index)
* `values`: the input arguments to the accumulator
* `opt_filter`: if present, only rows for which is Some(true) are included
* `value_fn`: function invoked for  (group_index, value) where value is non null

# Example

```text
 ┌─────────┐   ┌─────────┐   ┌ ─ ─ ─ ─ ┐
 │ ┌─────┐ │   │ ┌─────┐ │     ┌─────┐
 │ │  2  │ │   │ │ 200 │ │   │ │  t  │ │
 │ ├─────┤ │   │ ├─────┤ │     ├─────┤
 │ │  2  │ │   │ │ 100 │ │   │ │  f  │ │
 │ ├─────┤ │   │ ├─────┤ │     ├─────┤
 │ │  0  │ │   │ │ 200 │ │   │ │  t  │ │
 │ ├─────┤ │   │ ├─────┤ │     ├─────┤
 │ │  1  │ │   │ │ 200 │ │   │ │NULL │ │
 │ ├─────┤ │   │ ├─────┤ │     ├─────┤
 │ │  0  │ │   │ │ 300 │ │   │ │  t  │ │
 │ └─────┘ │   │ └─────┘ │     └─────┘
 └─────────┘   └─────────┘   └ ─ ─ ─ ─ ┘

group_indices   values        opt_filter
```

In the example above, `value_fn` is invoked for each (group_index,
value) pair where `opt_filter[i]` is true and values is non null

```text
value_fn(2, 200)
value_fn(0, 200)
value_fn(0, 300)
```

```rust
pub fn accumulate<T, F>(group_indices: &[usize], values: &arrow::array::PrimitiveArray<T>, opt_filter: Option<&arrow::array::BooleanArray>, value_fn: F)
where
    T: ArrowPrimitiveType + Send,
    F: FnMut(usize, <T as >::Native) + Send { /* ... */ }
```

#### Function `accumulate_multiple`

Accumulates with multiple accumulate(value) columns. (e.g. `corr(c1, c2)`)

This method assumes that for any input record index, if any of the value column
is null, or it's filtered out by `opt_filter`, then the record would be ignored.
(won't be accumulated by `value_fn`)

# Arguments

* `group_indices` - To which groups do the rows in `value_columns` belong
* `value_columns` - The input arrays to accumulate
* `opt_filter` - Optional filter array. If present, only rows where filter is `Some(true)` are included
* `value_fn` - Callback function for each valid row, with parameters:
    * `group_idx`: The group index for the current row
    * `batch_idx`: The index of the current row in the input arrays
    * `columns`: Reference to all input arrays for accessing values

```rust
pub fn accumulate_multiple<T, F>(group_indices: &[usize], value_columns: &[&arrow::array::PrimitiveArray<T>], opt_filter: Option<&arrow::array::BooleanArray>, value_fn: F)
where
    T: ArrowPrimitiveType + Send,
    F: FnMut(usize, usize, &[&arrow::array::PrimitiveArray<T>]) + Send { /* ... */ }
```

#### Function `accumulate_indices`

This function is called to update the accumulator state per row
when the value is not needed (e.g. COUNT)

`F`: Invoked like `value_fn(group_index) for all non null values
passing the filter. Note that no tracking is done for null inputs
or which groups have seen any values

See [`NullState::accumulate`], for more details on other
arguments.

```rust
pub fn accumulate_indices<F>(group_indices: &[usize], nulls: Option<&arrow::buffer::NullBuffer>, opt_filter: Option<&arrow::array::BooleanArray>, index_fn: F)
where
    F: FnMut(usize) + Send { /* ... */ }
```

## Module `bool_op`

```rust
pub mod bool_op { /* ... */ }
```

### Types

#### Struct `BooleanGroupsAccumulator`

An accumulator that implements a single operation over a
[`BooleanArray`] where the accumulated state is also boolean (such
as [`BitAndAssign`])

F: The function to apply to two elements. The first argument is
the existing value and should be updated with the second value
(e.g. [`BitAndAssign`] style).

[`BitAndAssign`]: std::ops::BitAndAssign

```rust
pub struct BooleanGroupsAccumulator<F> {
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
  pub fn new(bool_fn: F, identity: bool) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Send**
- **MaybeSendSync**
- **GroupsAccumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self, emit_to: EmitTo) -> Result<ArrayRef> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self, emit_to: EmitTo) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn convert_to_state(self: &Self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```

  - ```rust
    fn supports_convert_to_state(self: &Self) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Allocation**
- **RefUnwindSafe**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
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

- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `nulls`

[`set_nulls`], other utilities for working with nulls

```rust
pub mod nulls { /* ... */ }
```

### Functions

#### Function `set_nulls`

Sets the validity mask for a `PrimitiveArray` to `nulls`
replacing any existing null mask

See [`set_nulls_dyn`] for a version that works with `Array`

```rust
pub fn set_nulls<T: ArrowNumericType + Send>(array: arrow::array::PrimitiveArray<T>, nulls: Option<arrow::buffer::NullBuffer>) -> arrow::array::PrimitiveArray<T> { /* ... */ }
```

#### Function `filtered_null_mask`

Compute an output validity mask for an array that has been filtered

This can be used to compute nulls for the output of
[`GroupsAccumulator::convert_to_state`], which quickly applies an optional
filter to the input rows by setting any filtered rows to NULL in the output.
Subsequent applications of  aggregate functions that ignore NULLs (most of
them) will thus ignore the filtered rows as well.

# Output element is `true` (and thus output is non-null)

A `true` in the output represents non null output for all values that were *both*:

* `true` in any `opt_filter` (aka values that passed the filter)

* `non null` in `input`

# Output element is `false` (and thus output is null)

A `false` in the output represents an input that was *either*:

* `null`

* filtered (aka the value was `false` or `null` in the filter)

# Example

```text
┌─────┐           ┌─────┐            ┌─────┐
│true │           │NULL │            │false│
│true │    │      │true │            │true │
│true │ ───┼───   │false│  ────────▶ │false│       filtered_nulls
│false│    │      │NULL │            │false│
│false│           │true │            │false│
└─────┘           └─────┘            └─────┘
array           opt_filter           output
 .nulls()

false = NULL    true  = pass          false = NULL       Meanings
true  = valid   false = filter        true  = valid
                NULL  = filter
```

[`GroupsAccumulator::convert_to_state`]: datafusion_expr_common::groups_accumulator::GroupsAccumulator

```rust
pub fn filtered_null_mask(opt_filter: Option<&arrow::array::BooleanArray>, input: &dyn Array) -> Option<arrow::buffer::NullBuffer> { /* ... */ }
```

#### Function `apply_filter_as_nulls`

Applies optional filter to input, returning a new array of the same type
with the same data, but with any values that were filtered out set to null

```rust
pub fn apply_filter_as_nulls(input: &dyn Array, opt_filter: Option<&arrow::array::BooleanArray>) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `set_nulls_dyn`

Replaces the nulls in the input array with the given `NullBuffer`

TODO: replace when upstreamed in arrow-rs: <https://github.com/apache/arrow-rs/issues/6528>

```rust
pub fn set_nulls_dyn(input: &dyn Array, nulls: Option<arrow::buffer::NullBuffer>) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `prim_op`

```rust
pub mod prim_op { /* ... */ }
```

### Types

#### Struct `PrimitiveGroupsAccumulator`

An accumulator that implements a single operation over
[`ArrowPrimitiveType`] where the accumulated state is the same as
the input type (such as `Sum`)

F: The function to apply to two elements. The first argument is
the existing value and should be updated with the second value
(e.g. [`BitAndAssign`] style).

[`BitAndAssign`]: std::ops::BitAndAssign

```rust
pub struct PrimitiveGroupsAccumulator<T, F> {
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
  pub fn new(data_type: &DataType, prim_fn: F) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_starting_value(self: Self, starting_value: <T as >::Native) -> Self { /* ... */ }
  ```
  Set the starting values for new groups

###### Trait Implementations

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **GroupsAccumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self, emit_to: EmitTo) -> Result<ArrayRef> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self, emit_to: EmitTo) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn convert_to_state(self: &Self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```
    Converts an input batch directly to a state batch

  - ```rust
    fn supports_convert_to_state(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
### Types

#### Struct `GroupsAccumulatorAdapter`

 An adapter that implements [`GroupsAccumulator`] for any [`Accumulator`]

 While [`Accumulator`] are simpler to implement and can support
 more general calculations (like retractable window functions),
 they are not as fast as a specialized `GroupsAccumulator`. This
 interface bridges the gap so the group by operator only operates
 in terms of [`Accumulator`].

 Internally, this adapter creates a new [`Accumulator`] for each group which
 stores the state for that group. This both requires an allocation for each
 Accumulator, internal indices, as well as whatever internal allocations the
 Accumulator itself requires.

 For example, a `MinAccumulator` that computes the minimum string value with
 a [`ScalarValue::Utf8`]. That will require at least two allocations per group
 (one for the `MinAccumulator` and one for the `ScalarValue::Utf8`).

 ```text
                       ┌─────────────────────────────────┐
                       │MinAccumulator {                 │
                ┌─────▶│ min: ScalarValue::Utf8("A")     │───────┐
                │      │}                                │       │
                │      └─────────────────────────────────┘       └───────▶   "A"
    ┌─────┐     │      ┌─────────────────────────────────┐
    │  0  │─────┘      │MinAccumulator {                 │
    ├─────┤     ┌─────▶│ min: ScalarValue::Utf8("Z")     │───────────────▶   "Z"
    │  1  │─────┘      │}                                │
    └─────┘            └─────────────────────────────────┘                   ...
      ...                 ...
    ┌─────┐            ┌────────────────────────────────┐
    │ N-2 │            │MinAccumulator {                │
    ├─────┤            │  min: ScalarValue::Utf8("A")   │────────────────▶   "A"
    │ N-1 │─────┐      │}                               │
    └─────┘     │      └────────────────────────────────┘
                │      ┌────────────────────────────────┐        ┌───────▶   "Q"
                │      │MinAccumulator {                │        │
                └─────▶│  min: ScalarValue::Utf8("Q")   │────────┘
                       │}                               │
                       └────────────────────────────────┘


  Logical group         Current Min/Max value for that group stored
     number             as a ScalarValue which points to an
                        individually allocated String

```

 # Optimizations

 The adapter minimizes the number of calls to [`Accumulator::update_batch`]
 by first collecting the input rows for each group into a contiguous array
 using [`compute::take`]


```rust
pub struct GroupsAccumulatorAdapter {
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
  pub fn new<F>(factory: F) -> Self
where
    F: Fn() -> Result<Box<dyn Accumulator>> + Send + ''static { /* ... */ }
  ```
  Create a new adapter that will create a new [`Accumulator`]

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **GroupsAccumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self, emit_to: EmitTo) -> Result<ArrayRef> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self, emit_to: EmitTo) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn convert_to_state(self: &Self, values: &[ArrayRef], opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```

  - ```rust
    fn supports_convert_to_state(self: &Self) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **IntoEither**
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

- `allocated_size`: Return the amount of memory allocated by this Vec (not

##### Implementations

This trait is implemented for the following types:

- `Vec<T>` with <T>

## Module `merge_arrays`

```rust
pub mod merge_arrays { /* ... */ }
```

### Functions

#### Function `merge_ordered_arrays`

This functions merges `values` array (`&[Vec<ScalarValue>]`) into single array `Vec<ScalarValue>`
Merging done according to ordering values stored inside `ordering_values` (`&[Vec<Vec<ScalarValue>>]`)
Inner `Vec<ScalarValue>` in the `ordering_values` can be thought as ordering information for the
each `ScalarValue` in the `values` array.
Desired ordering specified by `sort_options` argument (Should have same size with inner `Vec<ScalarValue>`
of the `ordering_values` array).

As an example
values can be \[
     \[1, 2, 3, 4, 5\],
     \[1, 2, 3, 4\],
     \[1, 2, 3, 4, 5, 6\],
\]
In this case we will be merging three arrays (doesn't have to be same size)
and produce a merged array with size 15 (sum of 5+4+6)
Merging will be done according to ordering at `ordering_values` vector.
As an example `ordering_values` can be [
     \[(1, a), (2, b), (3, b), (4, a), (5, b) \],
     \[(1, a), (2, b), (3, b), (4, a) \],
     \[(1, b), (2, c), (3, d), (4, e), (5, a), (6, b) \],
]
For each ScalarValue in the `values` we have a corresponding `Vec<ScalarValue>` (like timestamp of it)
for the example above `sort_options` will have size two, that defines ordering requirement of the merge.
Inner `Vec<ScalarValue>`s of the `ordering_values` will be compared according `sort_options` (Their sizes should match)

```rust
pub fn merge_ordered_arrays(values: &mut [std::collections::VecDeque<datafusion_common::ScalarValue>], ordering_values: &mut [std::collections::VecDeque<Vec<datafusion_common::ScalarValue>>], sort_options: &[arrow::compute::SortOptions]) -> datafusion_common::Result<(Vec<datafusion_common::ScalarValue>, Vec<Vec<datafusion_common::ScalarValue>>)> { /* ... */ }
```

## Module `order`

```rust
pub mod order { /* ... */ }
```

### Types

#### Enum `AggregateOrderSensitivity`

Represents the sensitivity of an aggregate expression to ordering.

```rust
pub enum AggregateOrderSensitivity {
    Insensitive,
    HardRequirement,
    Beneficial,
}
```

##### Variants

###### `Insensitive`

Indicates that the aggregate expression is insensitive to ordering.
Ordering at the input is not important for the result of the aggregator.

###### `HardRequirement`

Indicates that the aggregate expression has a hard requirement on ordering.
The aggregator can not produce a correct result unless its ordering
requirement is satisfied.

###### `Beneficial`

Indicates that ordering is beneficial for the aggregate expression in terms
of evaluation efficiency. The aggregator can produce its result efficiently
when its required ordering is satisfied; however, it can still produce the
correct result (albeit less efficiently) when its required ordering is not met.

##### Implementations

###### Methods

- ```rust
  pub fn is_insensitive(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn is_beneficial(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn hard_requires(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

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

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AggregateOrderSensitivity { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AggregateOrderSensitivity) -> bool { /* ... */ }
    ```

- **Eq**
- **UnwindSafe**
- **MaybeSendSync**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
## Module `stats`

```rust
pub mod stats { /* ... */ }
```

### Types

#### Enum `StatsType`

TODO: Move this to functions-aggregate module
Enum used for differentiating population and sample for statistical functions

```rust
pub enum StatsType {
    Population,
    Sample,
}
```

##### Variants

###### `Population`

Population

###### `Sample`

Sample

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StatsType { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Copy**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **ErasedDestructor**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Freeze**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **RefUnwindSafe**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StatsType) -> bool { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Eq**
- **Unpin**
## Module `tdigest`

An implementation of the [TDigest sketch algorithm] providing approximate
quantile calculations.

The TDigest code in this module is modified from
<https://github.com/MnO2/t-digest>, itself a rust reimplementation of
[Facebook's Folly TDigest] implementation.

Alterations include reduction of runtime heap allocations, broader type
support, (de-)serialization support, reduced type conversions and null value
tolerance.

[TDigest sketch algorithm]: https://arxiv.org/abs/1902.04023
[Facebook's Folly TDigest]: https://github.com/facebook/folly/blob/main/folly/stats/TDigest.h

```rust
pub mod tdigest { /* ... */ }
```

### Types

#### Struct `Centroid`

Centroid implementation to the cluster mentioned in the paper.

```rust
pub struct Centroid {
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
  pub fn new(mean: f64, weight: f64) -> Self { /* ... */ }
  ```

- ```rust
  pub fn mean(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub fn weight(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub fn add(self: &mut Self, sum: f64, weight: f64) -> f64 { /* ... */ }
  ```

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

- **UnwindSafe**
- **Allocation**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Centroid) -> Option<Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Centroid) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **Eq**
- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Centroid { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Centroid) -> Ordering { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **IntoEither**
- **StructuralPartialEq**
#### Struct `TDigest`

T-Digest to be operated on.

```rust
pub struct TDigest {
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
  pub fn new(max_size: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn new_with_centroid(max_size: usize, centroid: Centroid) -> Self { /* ... */ }
  ```

- ```rust
  pub fn count(self: &Self) -> u64 { /* ... */ }
  ```

- ```rust
  pub fn max(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub fn min(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub fn max_size(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Size in bytes including `Self`.

- ```rust
  pub fn merge_unsorted_f64(self: &Self, unsorted_values: Vec<f64>) -> TDigest { /* ... */ }
  ```

- ```rust
  pub fn merge_sorted_f64(self: &Self, sorted_values: &[f64]) -> TDigest { /* ... */ }
  ```

- ```rust
  pub fn merge_digests<''a, /* synthetic */ impl IntoIterator<Item = &'a TDigest>: IntoIterator<Item = &''a TDigest>>(digests: impl IntoIterator<Item = &''a TDigest>) -> TDigest { /* ... */ }
  ```

- ```rust
  pub fn estimate_quantile(self: &Self, q: f64) -> f64 { /* ... */ }
  ```
  To estimate the value located at `q` quantile

- ```rust
  pub fn to_scalar_state(self: &Self) -> Vec<ScalarValue> { /* ... */ }
  ```
  This method decomposes the [`TDigest`] and its [`Centroid`] instances

- ```rust
  pub fn from_scalar_state(state: &[ScalarValue]) -> Self { /* ... */ }
  ```
  Unpack the serialized state of a [`TDigest`] produced by

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **RefUnwindSafe**
- **StructuralPartialEq**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TDigest) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TDigest { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **ErasedDestructor**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

### Traits

#### Trait `TryIntoF64`

This trait is implemented for each type a [`TDigest`] can operate on,
allowing it to support both numerical rust types (obtained from
`PrimitiveArray` instances), and [`ScalarValue`] instances.

```rust
pub trait TryIntoF64 {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `try_as_f64`: A fallible conversion of a possibly null `self` into a [`f64`].

##### Implementations

This trait is implemented for the following types:

- `f64`
- `f32`
- `i64`
- `i32`
- `i16`
- `i8`
- `u64`
- `u32`
- `u16`
- `u8`

### Constants and Statics

#### Constant `DEFAULT_MAX_SIZE`

```rust
pub const DEFAULT_MAX_SIZE: usize = 100;
```

## Module `utils`

```rust
pub mod utils { /* ... */ }
```

### Types

#### Struct `Hashable`

A wrapper around a type to provide hash for floats

```rust
pub struct Hashable<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Hashable<T> { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
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

- **Hash**
  - ```rust
    fn hash<H: std::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
#### Struct `DecimalAverager`

Computes averages for `Decimal128`/`Decimal256` values, checking for overflow

This is needed because different precisions for Decimal128/Decimal256 can
store different ranges of values and thus sum/count may not fit in
the target type.

For example, the precision is 3, the max of value is `999` and the min
value is `-999`

```rust
pub struct DecimalAverager<T: DecimalType> {
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
  pub fn try_new(sum_scale: i8, target_precision: u8, target_scale: i8) -> Result<Self> { /* ... */ }
  ```
  Create a new `DecimalAverager`:

- ```rust
  pub fn avg(self: &Self, sum: <T as >::Native, count: <T as >::Native) -> Result<<T as >::Native> { /* ... */ }
  ```
  Returns the `sum`/`count` as a i128/i256 Decimal128/Decimal256 with

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **ErasedDestructor**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Functions

#### Function `get_accum_scalar_values_as_arrays`

Convert scalar values from an accumulator into arrays.

```rust
pub fn get_accum_scalar_values_as_arrays(accum: &mut dyn Accumulator) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>> { /* ... */ }
```

#### Function `adjust_output_array`

**Attributes:**

- `#[deprecated(since = "44.0.0", note = "use PrimitiveArray::with_datatype")]`

**⚠️ Deprecated since 44.0.0**: use PrimitiveArray::with_datatype

Adjust array type metadata if needed

Since `Decimal128Arrays` created from `Vec<NativeType>` have
default precision and scale, this function adjusts the output to
match `data_type`, if necessary

```rust
pub fn adjust_output_array(data_type: &arrow::datatypes::DataType, array: arrow::array::ArrayRef) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `ordering_fields`

Construct corresponding fields for lexicographical ordering requirement expression

```rust
pub fn ordering_fields(ordering_req: &datafusion_physical_expr_common::sort_expr::LexOrdering, data_types: &[arrow::datatypes::DataType]) -> Vec<arrow::datatypes::Field> { /* ... */ }
```

#### Function `get_sort_options`

Selects the sort option attribute from all the given `PhysicalSortExpr`s.

```rust
pub fn get_sort_options(ordering_req: &datafusion_physical_expr_common::sort_expr::LexOrdering) -> Vec<arrow::compute::SortOptions> { /* ... */ }
```


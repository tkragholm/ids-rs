# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_expr_common`

Logical Expr types and traits for [DataFusion]

This crate contains types and traits that are used by both Logical and Physical expressions.
They are kept in their own crate to avoid physical expressions depending on logical expressions.


[DataFusion]: <https://crates.io/crates/datafusion>

## Modules

## Module `accumulator`

Accumulator module contains the trait definition for aggregation function's accumulators.

```rust
pub mod accumulator { /* ... */ }
```

### Traits

#### Trait `Accumulator`

Tracks an aggregate function's state.

`Accumulator`s are stateful objects that implement a single group. They
aggregate values from multiple rows together into a final output aggregate.

[`GroupsAccumulator]` is an additional more performant (but also complex) API
that manages state for multiple groups at once.

An accumulator knows how to:
* update its state from inputs via [`update_batch`]

* compute the final value from its internal state via [`evaluate`]

* retract an update to its state from given inputs via
  [`retract_batch`] (when used as a window aggregate [window
  function])

* convert its internal state to a vector of aggregate values via
  [`state`] and combine the state from multiple accumulators
  via [`merge_batch`], as part of efficient multi-phase grouping.

[`GroupsAccumulator`]: crate::GroupsAccumulator
[`update_batch`]: Self::update_batch
[`retract_batch`]: Self::retract_batch
[`state`]: Self::state
[`evaluate`]: Self::evaluate
[`merge_batch`]: Self::merge_batch
[window function]: https://en.wikipedia.org/wiki/Window_function_(SQL)

```rust
pub trait Accumulator: Send + Sync + Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `update_batch`: Updates the accumulator's state from its input.
- `evaluate`: Returns the final aggregate value, consuming the internal state.
- `size`: Returns the allocated size required for this accumulator, in
- `state`: Returns the intermediate state of the accumulator, consuming the
- `merge_batch`: Updates the accumulator's state from an `Array` containing one

##### Provided Methods

- ```rust
  fn retract_batch(self: &mut Self, _values: &[ArrayRef]) -> Result<()> { /* ... */ }
  ```
  Retracts (removed) an update (caused by the given inputs) to

- ```rust
  fn supports_retract_batch(self: &Self) -> bool { /* ... */ }
  ```
  Does the accumulator support incrementally updating its value

## Module `columnar_value`

[`ColumnarValue`] represents the result of evaluating an expression.

```rust
pub mod columnar_value { /* ... */ }
```

### Types

#### Enum `ColumnarValue`

The result of evaluating an expression.

[`ColumnarValue::Scalar`] represents a single value repeated any number of
times. This is an important performance optimization for handling values
that do not change across rows.

[`ColumnarValue::Array`] represents a column of data, stored as an  Arrow
[`ArrayRef`]

A slice of `ColumnarValue`s logically represents a table, with each column
having the same number of rows. This means that all `Array`s are the same
length.

# Example

A `ColumnarValue::Array` with an array of 5 elements and a
`ColumnarValue::Scalar` with the value 100

```text
┌──────────────┐
│ ┌──────────┐ │
│ │   "A"    │ │
│ ├──────────┤ │
│ │   "B"    │ │
│ ├──────────┤ │
│ │   "C"    │ │
│ ├──────────┤ │
│ │   "D"    │ │        ┌──────────────┐
│ ├──────────┤ │        │ ┌──────────┐ │
│ │   "E"    │ │        │ │   100    │ │
│ └──────────┘ │        │ └──────────┘ │
└──────────────┘        └──────────────┘

 ColumnarValue::        ColumnarValue::
      Array                 Scalar
```

Logically represents the following table:

| Column 1| Column 2 |
| ------- | -------- |
| A | 100 |
| B | 100 |
| C | 100 |
| D | 100 |
| E | 100 |

# Performance Notes

When implementing functions or operators, it is important to consider the
performance implications of handling scalar values.

Because all functions must handle [`ArrayRef`], it is
convenient to convert [`ColumnarValue::Scalar`]s using
[`Self::into_array`]. For example,  [`ColumnarValue::values_to_arrays`]
converts multiple columnar values into arrays of the same length.

However, it is often much more performant to provide a different,
implementation that handles scalar values differently

```rust
pub enum ColumnarValue {
    Array(arrow::array::ArrayRef),
    Scalar(datafusion_common::ScalarValue),
}
```

##### Variants

###### `Array`

Array of values

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `arrow::array::ArrayRef` |  |

###### `Scalar`

A single value

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_common::ScalarValue` |  |

##### Implementations

###### Methods

- ```rust
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```

- ```rust
  pub fn into_array(self: Self, num_rows: usize) -> Result<ArrayRef> { /* ... */ }
  ```
  Convert a columnar value into an Arrow [`ArrayRef`] with the specified

- ```rust
  pub fn to_array(self: &Self, num_rows: usize) -> Result<ArrayRef> { /* ... */ }
  ```
  Convert a columnar value into an Arrow [`ArrayRef`] with the specified

- ```rust
  pub fn create_null_array(num_rows: usize) -> Self { /* ... */ }
  ```
  Null columnar values are implemented as a null array in order to pass batch

- ```rust
  pub fn values_to_arrays(args: &[ColumnarValue]) -> Result<Vec<ArrayRef>> { /* ... */ }
  ```
  Converts  [`ColumnarValue`]s to [`ArrayRef`]s with the same length.

- ```rust
  pub fn cast_to(self: &Self, cast_type: &DataType, cast_options: Option<&CastOptions<''static>>) -> Result<ColumnarValue> { /* ... */ }
  ```
  Cast's this [ColumnarValue] to the specified `DataType`

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Send**
- **UnwindSafe**
- **MaybeSendSync**
- **Display**
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

- **IntoEither**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ColumnarValue { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

  - ```rust
    fn from(value: ArrayRef) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: ScalarValue) -> Self { /* ... */ }
    ```

- **Freeze**
## Module `groups_accumulator`

Vectorized [`GroupsAccumulator`]

```rust
pub mod groups_accumulator { /* ... */ }
```

### Types

#### Enum `EmitTo`

Describes how many rows should be emitted during grouping.

```rust
pub enum EmitTo {
    All,
    First(usize),
}
```

##### Variants

###### `All`

Emit all groups

###### `First`

Emit only the first `n` groups and shift all existing group
indexes down by `n`.

For example, if `n=10`, group_index `0, 1, ... 9` are emitted
and group indexes `10, 11, 12, ...` become `0, 1, 2, ...`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

##### Implementations

###### Methods

- ```rust
  pub fn take_needed<T>(self: &Self, v: &mut Vec<T>) -> Vec<T> { /* ... */ }
  ```
  Removes the number of rows from `v` required to emit the right

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **ErasedDestructor**
- **Send**
- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EmitTo { /* ... */ }
    ```

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

- **Allocation**
- **MaybeSendSync**
### Traits

#### Trait `GroupsAccumulator`

`GroupsAccumulator` implements a single aggregate (e.g. AVG) and
stores the state for *all* groups internally.

Logically, a [`GroupsAccumulator`] stores a mapping from each group index to
the state of the aggregate for that group. For example an implementation for
`min` might look like

```text
   ┌─────┐
   │  0  │───────────▶   100
   ├─────┤
   │  1  │───────────▶   200
   └─────┘
     ...                 ...
   ┌─────┐
   │ N-2 │───────────▶    50
   ├─────┤
   │ N-1 │───────────▶   200
   └─────┘


 Logical group      Current Min
    number          value for that
                    group
```

# Notes on Implementing `GroupsAccumulator`

All aggregates must first implement the simpler [`Accumulator`] trait, which
handles state for a single group. Implementing `GroupsAccumulator` is
optional and is harder to implement than `Accumulator`, but can be much
faster for queries with many group values.  See the [Aggregating Millions of
Groups Fast blog] for more background.

[`NullState`] can help keep the state for groups that have not seen any
values and produce the correct output for those groups.

[`NullState`]: https://docs.rs/datafusion/latest/datafusion/physical_expr/struct.NullState.html

# Details
Each group is assigned a `group_index` by the hash table and each
accumulator manages the specific state, one per `group_index`.

`group_index`es are contiguous (there aren't gaps), and thus it is
expected that each `GroupsAccumulator` will use something like `Vec<..>`
to store the group states.

[`Accumulator`]: crate::accumulator::Accumulator
[Aggregating Millions of Groups Fast blog]: https://arrow.apache.org/blog/2023/08/05/datafusion_fast_grouping/

```rust
pub trait GroupsAccumulator: Send {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `update_batch`: Updates the accumulator's state from its arguments, encoded as
- `evaluate`: Returns the final aggregate value for each group as a single
- `state`: Returns the intermediate aggregate state for this accumulator,
- `merge_batch`: Merges intermediate state (the output from [`Self::state`])
- `size`: Amount of memory used to store the state of this accumulator,

##### Provided Methods

- ```rust
  fn convert_to_state(self: &Self, _values: &[ArrayRef], _opt_filter: Option<&BooleanArray>) -> Result<Vec<ArrayRef>> { /* ... */ }
  ```
  Converts an input batch directly to the intermediate aggregate state.

- ```rust
  fn supports_convert_to_state(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if [`Self::convert_to_state`] is implemented to support

## Module `interval_arithmetic`

Interval arithmetic library

```rust
pub mod interval_arithmetic { /* ... */ }
```

### Types

#### Struct `Interval`

The `Interval` type represents a closed interval used for computing
reliable bounds for mathematical expressions.

Conventions:

1. **Closed bounds**: The interval always encompasses its endpoints. We
   accommodate operations resulting in open intervals by incrementing or
   decrementing the interval endpoint value to its successor/predecessor.

2. **Unbounded endpoints**: If the `lower` or `upper` bounds are indeterminate,
   they are labeled as *unbounded*. This is represented using a `NULL`.

3. **Overflow handling**: If the `lower` or `upper` endpoints exceed their
   limits after any operation, they either become unbounded or they are fixed
   to the maximum/minimum value of the datatype, depending on the direction
   of the overflowing endpoint, opting for the safer choice.

4. **Floating-point special cases**:
   - `INF` values are converted to `NULL`s while constructing an interval to
     ensure consistency, with other data types.
   - `NaN` (Not a Number) results are conservatively result in unbounded
     endpoints.

```rust
pub struct Interval {
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
  pub fn try_new(lower: ScalarValue, upper: ScalarValue) -> Result<Self> { /* ... */ }
  ```
  Attempts to create a new `Interval` from the given lower and upper bounds.

- ```rust
  pub fn make<T>(lower: Option<T>, upper: Option<T>) -> Result<Self>
where
    ScalarValue: From<Option<T>> { /* ... */ }
  ```
  Convenience function to create a new `Interval` from the given (optional)

- ```rust
  pub fn make_zero(data_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Creates a singleton zero interval if the datatype supported.

- ```rust
  pub fn make_unbounded(data_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Creates an unbounded interval from both sides if the datatype supported.

- ```rust
  pub fn make_symmetric_unit_interval(data_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Creates an interval between -1 to 1.

- ```rust
  pub fn make_symmetric_pi_interval(data_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Create an interval from -π to π.

- ```rust
  pub fn make_symmetric_half_pi_interval(data_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Create an interval from -π/2 to π/2.

- ```rust
  pub fn make_non_negative_infinity_interval(data_type: &DataType) -> Result<Self> { /* ... */ }
  ```
  Create an interval from 0 to infinity.

- ```rust
  pub fn lower(self: &Self) -> &ScalarValue { /* ... */ }
  ```
  Returns a reference to the lower bound.

- ```rust
  pub fn upper(self: &Self) -> &ScalarValue { /* ... */ }
  ```
  Returns a reference to the upper bound.

- ```rust
  pub fn into_bounds(self: Self) -> (ScalarValue, ScalarValue) { /* ... */ }
  ```
  Converts this `Interval` into its boundary scalar values. It's useful

- ```rust
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```
  This function returns the data type of this interval.

- ```rust
  pub fn is_unbounded(self: &Self) -> bool { /* ... */ }
  ```
  Checks if the interval is unbounded (on either side).

- ```rust
  pub fn cast_to(self: &Self, data_type: &DataType, cast_options: &CastOptions<''_>) -> Result<Self> { /* ... */ }
  ```
  Casts this interval to `data_type` using `cast_options`.

- ```rust
  pub fn gt<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Decide if this interval is certainly greater than, possibly greater than,

- ```rust
  pub fn gt_eq<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Decide if this interval is certainly greater than or equal to, possibly

- ```rust
  pub fn lt<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Decide if this interval is certainly less than, possibly less than, or

- ```rust
  pub fn lt_eq<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Decide if this interval is certainly less than or equal to, possibly

- ```rust
  pub fn equal<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Decide if this interval is certainly equal to, possibly equal to, or

- ```rust
  pub fn and<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Compute the logical conjunction of this (boolean) interval with the

- ```rust
  pub fn or<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Compute the logical disjunction of this boolean interval with the

- ```rust
  pub fn not(self: &Self) -> Result<Self> { /* ... */ }
  ```
  Compute the logical negation of this (boolean) interval.

- ```rust
  pub fn intersect<T: Borrow<Self>>(self: &Self, other: T) -> Result<Option<Self>> { /* ... */ }
  ```
  Compute the intersection of this interval with the given interval.

- ```rust
  pub fn union<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Compute the union of this interval with the given interval.

- ```rust
  pub fn contains_value<T: Borrow<ScalarValue>>(self: &Self, other: T) -> Result<bool> { /* ... */ }
  ```
  Decide if this interval contains a [`ScalarValue`] (`other`) by returning `true` or `false`.

- ```rust
  pub fn contains<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Decide if this interval is a superset of, overlaps with, or

- ```rust
  pub fn add<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Add the given interval (`other`) to this interval. Say we have intervals

- ```rust
  pub fn sub<T: Borrow<Interval>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Subtract the given interval (`other`) from this interval. Say we have

- ```rust
  pub fn mul<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Multiply the given interval (`other`) with this interval. Say we have

- ```rust
  pub fn div<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Divide this interval by the given interval (`other`). Say we have intervals

- ```rust
  pub fn width(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```
  Computes the width of this interval; i.e. the difference between its

- ```rust
  pub fn cardinality(self: &Self) -> Option<u64> { /* ... */ }
  ```
  Returns the cardinality of this interval, which is the number of all

- ```rust
  pub fn arithmetic_negate(self: &Self) -> Result<Self> { /* ... */ }
  ```
  Reflects an [`Interval`] around the point zero.

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Interval { /* ... */ }
    ```

- **IntoEither**
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

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Interval) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
- **Send**
- **MaybeSendSync**
- **Unpin**
- **Eq**
#### Enum `NullableInterval`

An [Interval] that also tracks null status using a boolean interval.

This represents values that may be in a particular range or be null.

# Examples

```
use arrow::datatypes::DataType;
use datafusion_common::ScalarValue;
use datafusion_expr_common::interval_arithmetic::Interval;
use datafusion_expr_common::interval_arithmetic::NullableInterval;

// [1, 2) U {NULL}
let maybe_null = NullableInterval::MaybeNull {
   values: Interval::try_new(
           ScalarValue::Int32(Some(1)),
           ScalarValue::Int32(Some(2)),
       ).unwrap(),
};

// (0, ∞)
let not_null = NullableInterval::NotNull {
  values: Interval::try_new(
           ScalarValue::Int32(Some(0)),
           ScalarValue::Int32(None),
       ).unwrap(),
};

// {NULL}
let null_interval = NullableInterval::Null { datatype: DataType::Int32 };

// {4}
let single_value = NullableInterval::from(ScalarValue::Int32(Some(4)));
```

```rust
pub enum NullableInterval {
    Null {
        datatype: arrow::datatypes::DataType,
    },
    MaybeNull {
        values: Interval,
    },
    NotNull {
        values: Interval,
    },
}
```

##### Variants

###### `Null`

The value is always null. This is typed so it can be used in physical
expressions, which don't do type coercion.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `datatype` | `arrow::datatypes::DataType` |  |

###### `MaybeNull`

The value may or may not be null. If it is non-null, its is within the
specified range.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `values` | `Interval` |  |

###### `NotNull`

The value is definitely not null, and is within the specified range.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `values` | `Interval` |  |

##### Implementations

###### Methods

- ```rust
  pub fn values(self: &Self) -> Option<&Interval> { /* ... */ }
  ```
  Get the values interval, or None if this interval is definitely null.

- ```rust
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```
  Get the data type

- ```rust
  pub fn is_certainly_true(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the value is definitely true (and not null).

- ```rust
  pub fn is_certainly_false(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the value is definitely false (and not null).

- ```rust
  pub fn apply_operator(self: &Self, op: &Operator, rhs: &Self) -> Result<Self> { /* ... */ }
  ```
  Apply the given operator to this interval and the given interval.

- ```rust
  pub fn contains<T: Borrow<Self>>(self: &Self, other: T) -> Result<Self> { /* ... */ }
  ```
  Decide if this interval is a superset of, overlaps with, or

- ```rust
  pub fn single_value(self: &Self) -> Option<ScalarValue> { /* ... */ }
  ```
  If the interval has collapsed to a single value, return that value.

###### Trait Implementations

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: ScalarValue) -> Self { /* ... */ }
    ```
    Create an interval that represents a single value.

- **MaybeSendSync**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NullableInterval { /* ... */ }
    ```

- **Unpin**
- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
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

- **UnwindSafe**
- **ErasedDestructor**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &NullableInterval) -> bool { /* ... */ }
    ```

### Functions

#### Function `apply_operator`

Applies the given binary operator the `lhs` and `rhs` arguments.

```rust
pub fn apply_operator(op: &crate::operator::Operator, lhs: &Interval, rhs: &Interval) -> datafusion_common::Result<Interval> { /* ... */ }
```

#### Function `satisfy_greater`

This function updates the given intervals by enforcing (i.e. propagating)
the inequality `left > right` (or the `left >= right` inequality, if `strict`
is `true`).

Returns a `Result` wrapping an `Option` containing the tuple of resulting
intervals. If the comparison is infeasible, returns `None`.

Example usage:
```
use datafusion_common::DataFusionError;
use datafusion_expr_common::interval_arithmetic::{satisfy_greater, Interval};

let left = Interval::make(Some(-1000.0_f32), Some(1000.0_f32))?;
let right = Interval::make(Some(500.0_f32), Some(2000.0_f32))?;
let strict = false;
assert_eq!(
    satisfy_greater(&left, &right, strict)?,
    Some((
        Interval::make(Some(500.0_f32), Some(1000.0_f32))?,
        Interval::make(Some(500.0_f32), Some(1000.0_f32))?
    ))
);
Ok::<(), DataFusionError>(())
```

NOTE: This function only works with intervals of the same data type.
      Attempting to compare intervals of different data types will lead
      to an error.

```rust
pub fn satisfy_greater(left: &Interval, right: &Interval, strict: bool) -> datafusion_common::Result<Option<(Interval, Interval)>> { /* ... */ }
```

#### Function `cardinality_ratio`

This function computes the selectivity of an operation by computing the
cardinality ratio of the given input/output intervals. If this can not be
calculated for some reason, it returns `1.0` meaning fully selective (no
filtering).

```rust
pub fn cardinality_ratio(initial_interval: &Interval, final_interval: &Interval) -> f64 { /* ... */ }
```

## Module `operator`

```rust
pub mod operator { /* ... */ }
```

### Types

#### Enum `Operator`

Operators applied to expressions

```rust
pub enum Operator {
    Eq,
    NotEq,
    Lt,
    LtEq,
    Gt,
    GtEq,
    Plus,
    Minus,
    Multiply,
    Divide,
    Modulo,
    And,
    Or,
    IsDistinctFrom,
    IsNotDistinctFrom,
    RegexMatch,
    RegexIMatch,
    RegexNotMatch,
    RegexNotIMatch,
    LikeMatch,
    ILikeMatch,
    NotLikeMatch,
    NotILikeMatch,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseShiftRight,
    BitwiseShiftLeft,
    StringConcat,
    AtArrow,
    ArrowAt,
    Arrow,
    LongArrow,
    HashArrow,
    HashLongArrow,
    AtAt,
    IntegerDivide,
    HashMinus,
    AtQuestion,
    Question,
    QuestionAnd,
    QuestionPipe,
}
```

##### Variants

###### `Eq`

Expressions are equal

###### `NotEq`

Expressions are not equal

###### `Lt`

Left side is smaller than right side

###### `LtEq`

Left side is smaller or equal to right side

###### `Gt`

Left side is greater than right side

###### `GtEq`

Left side is greater or equal to right side

###### `Plus`

Addition

###### `Minus`

Subtraction

###### `Multiply`

Multiplication operator, like `*`

###### `Divide`

Division operator, like `/`

###### `Modulo`

Remainder operator, like `%`

###### `And`

Logical AND, like `&&`

###### `Or`

Logical OR, like `||`

###### `IsDistinctFrom`

`IS DISTINCT FROM` (see [`distinct`])

[`distinct`]: arrow::compute::kernels::cmp::distinct

###### `IsNotDistinctFrom`

`IS NOT DISTINCT FROM` (see [`not_distinct`])

[`not_distinct`]: arrow::compute::kernels::cmp::not_distinct

###### `RegexMatch`

Case sensitive regex match

###### `RegexIMatch`

Case insensitive regex match

###### `RegexNotMatch`

Case sensitive regex not match

###### `RegexNotIMatch`

Case insensitive regex not match

###### `LikeMatch`

Case sensitive pattern match

###### `ILikeMatch`

Case insensitive pattern match

###### `NotLikeMatch`

Case sensitive pattern not match

###### `NotILikeMatch`

Case insensitive pattern not match

###### `BitwiseAnd`

Bitwise and, like `&`

###### `BitwiseOr`

Bitwise or, like `|`

###### `BitwiseXor`

Bitwise xor, such as `^` in MySQL or `#` in PostgreSQL

###### `BitwiseShiftRight`

Bitwise right, like `>>`

###### `BitwiseShiftLeft`

Bitwise left, like `<<`

###### `StringConcat`

String concat

###### `AtArrow`

At arrow, like `@>`.

Currently only supported to be used with lists:
```sql
select [1,3] <@ [1,2,3]
```

###### `ArrowAt`

Arrow at, like `<@`.

Currently only supported to be used with lists:
```sql
select [1,2,3] @> [1,3]
```

###### `Arrow`

Arrow, like `->`.

Not implemented in DataFusion yet.

###### `LongArrow`

Long arrow, like `->>`

Not implemented in DataFusion yet.

###### `HashArrow`

Hash arrow, like `#>`

Not implemented in DataFusion yet.

###### `HashLongArrow`

Hash long arrow, like `#>>`

Not implemented in DataFusion yet.

###### `AtAt`

At at, like `@@`

Not implemented in DataFusion yet.

###### `IntegerDivide`

Integer division operator, like `DIV` from MySQL or `//` from DuckDB

Not implemented in DataFusion yet.

###### `HashMinus`

Hash Minis, like `#-`

Not implemented in DataFusion yet.

###### `AtQuestion`

At question, like `@?`

Not implemented in DataFusion yet.

###### `Question`

Question, like `?`

Not implemented in DataFusion yet.

###### `QuestionAnd`

Question and, like `?&`

Not implemented in DataFusion yet.

###### `QuestionPipe`

Question pipe, like `?|`

Not implemented in DataFusion yet.

##### Implementations

###### Methods

- ```rust
  pub fn negate(self: &Self) -> Option<Operator> { /* ... */ }
  ```
  If the operator can be negated, return the negated operator

- ```rust
  pub fn is_numerical_operators(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the operator is a numerical operator.

- ```rust
  pub fn supports_propagation(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the comparison operator can be used in interval arithmetic and constraint

- ```rust
  pub fn is_comparison_operator(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the comparison operator can be used in interval arithmetic and constraint

- ```rust
  pub fn is_logic_operator(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the operator is a logic operator.

- ```rust
  pub fn swap(self: &Self) -> Option<Operator> { /* ... */ }
  ```
  Return the operator where swapping lhs and rhs wouldn't change the result.

- ```rust
  pub fn precedence(self: &Self) -> u8 { /* ... */ }
  ```
  Get the operator precedence

###### Trait Implementations

- **Allocation**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Operator) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

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

- **MaybeSendSync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Operator) -> bool { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Operator { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
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

- **ErasedDestructor**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

## Module `signature`

Signature module contains foundational types that are used to represent signatures, types,
and return types of functions in DataFusion.

```rust
pub mod signature { /* ... */ }
```

### Types

#### Enum `Volatility`

A function's volatility, which defines the functions eligibility for certain optimizations

```rust
pub enum Volatility {
    Immutable,
    Stable,
    Volatile,
}
```

##### Variants

###### `Immutable`

An immutable function will always return the same output when given the same
input. DataFusion will attempt to inline immutable functions during planning.

###### `Stable`

A stable function may return different values given the same input across different
queries but must return the same value for a given input within a query. An example of
this is the `Now` function. DataFusion will attempt to inline `Stable` functions
during planning, when possible.
For query `select col1, now() from t1`, it might take a while to execute but
`now()` column will be the same for each output row, which is evaluated
during planning.

###### `Volatile`

A volatile function may change the return value from evaluation to evaluation.
Multiple invocations of a volatile function may return different results when used in the
same query. An example of this is the random() function. DataFusion
can not evaluate such functions during planning.
In the query `select col1, random() from t1`, `random()` function will be evaluated
for each output row, resulting in a unique random value for each row.

##### Implementations

###### Trait Implementations

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Volatility { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Volatility) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Volatility) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Ord**
  - ```rust
    fn cmp(self: &Self, other: &Volatility) -> $crate::cmp::Ordering { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Allocation**
- **Unpin**
- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Comparable**
  - ```rust
    fn compare(self: &Self, key: &K) -> Ordering { /* ... */ }
    ```

#### Enum `TypeSignature`

A function's type signature defines the types of arguments the function supports.

Functions typically support only a few different types of arguments compared to the
different datatypes in Arrow. To make functions easy to use, when possible DataFusion
automatically coerces (add casts to) function arguments so they match the type signature.

For example, a function like `cos` may only be implemented for `Float64` arguments. To support a query
that calls `cos` with a different argument type, such as `cos(int_column)`, type coercion automatically
adds a cast such as `cos(CAST int_column AS DOUBLE)` during planning.

# Data Types

## Timestamps

Types to match are represented using Arrow's [`DataType`].  [`DataType::Timestamp`] has an optional variable
timezone specification. To specify a function can handle a timestamp with *ANY* timezone, use
the [`TIMEZONE_WILDCARD`]. For example:

```
# use arrow::datatypes::{DataType, TimeUnit};
# use datafusion_expr_common::signature::{TIMEZONE_WILDCARD, TypeSignature};
let type_signature = TypeSignature::Exact(vec![
  // A nanosecond precision timestamp with ANY timezone
  // matches  Timestamp(Nanosecond, Some("+0:00"))
  // matches  Timestamp(Nanosecond, Some("+5:00"))
  // does not match  Timestamp(Nanosecond, None)
  DataType::Timestamp(TimeUnit::Nanosecond, Some(TIMEZONE_WILDCARD.into())),
]);
```

```rust
pub enum TypeSignature {
    Variadic(Vec<arrow::datatypes::DataType>),
    UserDefined,
    VariadicAny,
    Uniform(usize, Vec<arrow::datatypes::DataType>),
    Exact(Vec<arrow::datatypes::DataType>),
    Coercible(Vec<Coercion>),
    Comparable(usize),
    Any(usize),
    OneOf(Vec<TypeSignature>),
    ArraySignature(ArrayFunctionSignature),
    Numeric(usize),
    String(usize),
    Nullary,
}
```

##### Variants

###### `Variadic`

One or more arguments of a common type out of a list of valid types.

For functions that take no arguments (e.g. `random()` see [`TypeSignature::Nullary`]).

# Examples

A function such as `concat` is `Variadic(vec![DataType::Utf8,
DataType::LargeUtf8])`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<arrow::datatypes::DataType>` |  |

###### `UserDefined`

The acceptable signature and coercions rules are special for this
function.

If this signature is specified,
DataFusion will call [`ScalarUDFImpl::coerce_types`] to prepare argument types.

[`ScalarUDFImpl::coerce_types`]: https://docs.rs/datafusion/latest/datafusion/logical_expr/trait.ScalarUDFImpl.html#method.coerce_types

###### `VariadicAny`

One or more arguments with arbitrary types

###### `Uniform`

One or more arguments of an arbitrary but equal type out of a list of valid types.

# Examples

1. A function of one argument of f64 is `Uniform(1, vec![DataType::Float64])`
2. A function of one argument of f64 or f32 is `Uniform(1, vec![DataType::Float32, DataType::Float64])`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |
| 1 | `Vec<arrow::datatypes::DataType>` |  |

###### `Exact`

One or more arguments with exactly the specified types in order.

For functions that take no arguments (e.g. `random()`) use [`TypeSignature::Nullary`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<arrow::datatypes::DataType>` |  |

###### `Coercible`

One or more arguments belonging to the [`TypeSignatureClass`], in order.

[`Coercion`] contains not only the desired type but also the allowed casts.
For example, if you expect a function has string type, but you also allow it to be casted from binary type.

For functions that take no arguments (e.g. `random()`) see [`TypeSignature::Nullary`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Coercion>` |  |

###### `Comparable`

One or more arguments coercible to a single, comparable type.

Each argument will be coerced to a single type using the
coercion rules described in [`comparison_coercion_numeric`].

# Examples

If the `nullif(1, 2)` function is called with `i32` and `i64` arguments
the types will both be coerced to `i64` before the function is invoked.

If the `nullif('1', 2)` function is called with `Utf8` and `i64` arguments
the types will both be coerced to `Utf8` before the function is invoked.

Note:
- For functions that take no arguments (e.g. `random()` see [`TypeSignature::Nullary`]).
- If all arguments have type [`DataType::Null`], they are coerced to `Utf8`

[`comparison_coercion_numeric`]: crate::type_coercion::binary::comparison_coercion_numeric

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `Any`

One or more arguments of arbitrary types.

For functions that take no arguments (e.g. `random()`) use [`TypeSignature::Nullary`].

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `OneOf`

Matches exactly one of a list of [`TypeSignature`]s.

Coercion is attempted to match the signatures in order, and stops after
the first success, if any.

# Examples

Since `make_array` takes 0 or more arguments with arbitrary types, its `TypeSignature`
is `OneOf(vec![Any(0), VariadicAny])`.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<TypeSignature>` |  |

###### `ArraySignature`

A function that has an [`ArrayFunctionSignature`]

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ArrayFunctionSignature` |  |

###### `Numeric`

One or more arguments of numeric types.

See [`NativeType::is_numeric`] to know which type is considered numeric

For functions that take no arguments (e.g. `random()`) use [`TypeSignature::Nullary`].

[`NativeType::is_numeric`]: datafusion_common::types::NativeType::is_numeric

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `String`

One or arguments of all the same string types.

The precedence of type from high to low is Utf8View, LargeUtf8 and Utf8.
Null is considered as `Utf8` by default
Dictionary with string value type is also handled.

For example, if a function is called with (utf8, large_utf8), all
arguments will be coerced to  `LargeUtf8`

For functions that take no arguments (e.g. `random()` use [`TypeSignature::Nullary`]).

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `usize` |  |

###### `Nullary`

No arguments

##### Implementations

###### Methods

- ```rust
  pub fn is_one_of(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn to_string_repr(self: &Self) -> Vec<String> { /* ... */ }
  ```

- ```rust
  pub fn join_types<T: Display>(types: &[T], delimiter: &str) -> String { /* ... */ }
  ```
  Helper function to join types with specified delimiter.

- ```rust
  pub fn supports_zero_argument(self: &Self) -> bool { /* ... */ }
  ```
  Check whether 0 input argument is valid for given `TypeSignature`

- ```rust
  pub fn used_to_support_zero_arguments(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the signature currently supports or used to supported 0

- ```rust
  pub fn get_possible_types(self: &Self) -> Vec<Vec<DataType>> { /* ... */ }
  ```

- ```rust
  pub fn get_example_types(self: &Self) -> Vec<Vec<DataType>> { /* ... */ }
  ```
  Return example acceptable types for this `TypeSignature`'

###### Trait Implementations

- **Unpin**
- **MaybeSendSync**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TypeSignature { /* ... */ }
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

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TypeSignature) -> bool { /* ... */ }
    ```

- **Eq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &TypeSignature) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `TypeSignatureClass`

Represents the class of types that can be used in a function signature.

This is used to specify what types are valid for function arguments in a more flexible way than
just listing specific DataTypes. For example, TypeSignatureClass::Timestamp matches any timestamp
type regardless of timezone or precision.

Used primarily with TypeSignature::Coercible to define function signatures that can accept
arguments that can be coerced to a particular class of types.

```rust
pub enum TypeSignatureClass {
    Timestamp,
    Time,
    Interval,
    Duration,
    Native(datafusion_common::types::LogicalTypeRef),
    Integer,
}
```

##### Variants

###### `Timestamp`

###### `Time`

###### `Interval`

###### `Duration`

###### `Native`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_common::types::LogicalTypeRef` |  |

###### `Integer`

##### Implementations

###### Methods

- ```rust
  pub fn matches_native_type(self: &TypeSignatureClass, logical_type: &NativeType) -> bool { /* ... */ }
  ```
  Does the specified `NativeType` match this type signature class?

- ```rust
  pub fn default_casted_type(self: &Self, native_type: &NativeType, origin_type: &DataType) -> datafusion_common::Result<DataType> { /* ... */ }
  ```
  What type would `origin_type` be casted to when casting to the specified native type?

###### Trait Implementations

- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TypeSignatureClass) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TypeSignatureClass { /* ... */ }
    ```

- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &TypeSignatureClass) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `ArrayFunctionSignature`

```rust
pub enum ArrayFunctionSignature {
    Array {
        arguments: Vec<ArrayFunctionArgument>,
        array_coercion: Option<datafusion_common::utils::ListCoercion>,
    },
    RecursiveArray,
    MapArray,
}
```

##### Variants

###### `Array`

A function takes at least one List/LargeList/FixedSizeList argument.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `arguments` | `Vec<ArrayFunctionArgument>` | A full list of the arguments accepted by this function. |
| `array_coercion` | `Option<datafusion_common::utils::ListCoercion>` | Additional information about how array arguments should be coerced. |

###### `RecursiveArray`

A function takes a single argument that must be a List/LargeList/FixedSizeList
which gets coerced to List, with element type recursively coerced to List too if it is list-like.

###### `MapArray`

Specialized Signature for MapArray
The function takes a single argument that must be a MapArray

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **RefUnwindSafe**
- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArrayFunctionSignature { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArrayFunctionSignature) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArrayFunctionSignature) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Allocation**
- **IntoEither**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **MaybeSendSync**
#### Enum `ArrayFunctionArgument`

```rust
pub enum ArrayFunctionArgument {
    Element,
    Index,
    Array,
    String,
}
```

##### Variants

###### `Element`

A non-list or list argument. The list dimensions should be one less than the Array's list
dimensions.

###### `Index`

An Int64 index argument.

###### `Array`

An argument of type List/LargeList/FixedSizeList. All Array arguments must be coercible
to the same type.

###### `String`

##### Implementations

###### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ArrayFunctionArgument) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ArrayFunctionArgument) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Allocation**
- **RefUnwindSafe**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Send**
- **Eq**
- **ErasedDestructor**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ArrayFunctionArgument { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
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

#### Enum `Coercion`

Represents type coercion rules for function arguments, specifying both the desired type
and optional implicit coercion rules for source types.

# Examples

```
use datafusion_expr_common::signature::{Coercion, TypeSignatureClass};
use datafusion_common::types::{NativeType, logical_binary, logical_string};

// Exact coercion that only accepts timestamp types
let exact = Coercion::new_exact(TypeSignatureClass::Timestamp);

// Implicit coercion that accepts string types but can coerce from binary types
let implicit = Coercion::new_implicit(
    TypeSignatureClass::Native(logical_string()),
    vec![TypeSignatureClass::Native(logical_binary())],
    NativeType::String
);
```

There are two variants:

* `Exact` - Only accepts arguments that exactly match the desired type
* `Implicit` - Accepts the desired type and can coerce from specified source types

```rust
pub enum Coercion {
    Exact {
        desired_type: TypeSignatureClass,
    },
    Implicit {
        desired_type: TypeSignatureClass,
        implicit_coercion: ImplicitCoercion,
    },
}
```

##### Variants

###### `Exact`

Coercion that only accepts arguments exactly matching the desired type.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `desired_type` | `TypeSignatureClass` | The required type for the argument |

###### `Implicit`

Coercion that accepts the desired type and can implicitly coerce from other types.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `desired_type` | `TypeSignatureClass` | The primary desired type for the argument |
| `implicit_coercion` | `ImplicitCoercion` | Rules for implicit coercion from other types |

##### Implementations

###### Methods

- ```rust
  pub fn new_exact(desired_type: TypeSignatureClass) -> Self { /* ... */ }
  ```

- ```rust
  pub fn new_implicit(desired_type: TypeSignatureClass, allowed_source_types: Vec<TypeSignatureClass>, default_casted_type: NativeType) -> Self { /* ... */ }
  ```
  Create a new coercion with implicit coercion rules.

- ```rust
  pub fn allowed_source_types(self: &Self) -> &[TypeSignatureClass] { /* ... */ }
  ```

- ```rust
  pub fn default_casted_type(self: &Self) -> Option<&NativeType> { /* ... */ }
  ```

- ```rust
  pub fn desired_type(self: &Self) -> &TypeSignatureClass { /* ... */ }
  ```

- ```rust
  pub fn implicit_coercion(self: &Self) -> Option<&ImplicitCoercion> { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
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

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Coercion) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: std::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Coercion { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **MaybeSendSync**
- **RefUnwindSafe**
- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `ImplicitCoercion`

Defines rules for implicit type coercion, specifying which source types can be
coerced and the default type to use when coercing.

This is used by functions to specify which types they can accept via implicit
coercion in addition to their primary desired type.

# Examples

```
use arrow::datatypes::TimeUnit;

use datafusion_expr_common::signature::{Coercion, ImplicitCoercion, TypeSignatureClass};
use datafusion_common::types::{NativeType, logical_binary};

// Allow coercing from binary types to timestamp, coerce to specific timestamp unit and timezone
let implicit = Coercion::new_implicit(
    TypeSignatureClass::Timestamp,
    vec![TypeSignatureClass::Native(logical_binary())],
    NativeType::Timestamp(TimeUnit::Second, None),
);
```

```rust
pub struct ImplicitCoercion {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ImplicitCoercion) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Sync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **ErasedDestructor**
- **Eq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: std::hash::Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ImplicitCoercion { /* ... */ }
    ```

- **Freeze**
#### Struct `Signature`

Defines the supported argument types ([`TypeSignature`]) and [`Volatility`] for a function.

DataFusion will automatically coerce (cast) argument types to one of the supported
function signatures, if possible.

```rust
pub struct Signature {
    pub type_signature: TypeSignature,
    pub volatility: Volatility,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `type_signature` | `TypeSignature` | The data types that the function accepts. See [TypeSignature] for more information. |
| `volatility` | `Volatility` | The volatility of the function. See [Volatility] for more information. |

##### Implementations

###### Methods

- ```rust
  pub fn new(type_signature: TypeSignature, volatility: Volatility) -> Self { /* ... */ }
  ```
  Creates a new Signature from a given type signature and volatility.

- ```rust
  pub fn variadic(common_types: Vec<DataType>, volatility: Volatility) -> Self { /* ... */ }
  ```
  An arbitrary number of arguments with the same type, from those listed in `common_types`.

- ```rust
  pub fn user_defined(volatility: Volatility) -> Self { /* ... */ }
  ```
  User-defined coercion rules for the function.

- ```rust
  pub fn numeric(arg_count: usize, volatility: Volatility) -> Self { /* ... */ }
  ```
  A specified number of numeric arguments

- ```rust
  pub fn string(arg_count: usize, volatility: Volatility) -> Self { /* ... */ }
  ```
  A specified number of numeric arguments

- ```rust
  pub fn variadic_any(volatility: Volatility) -> Self { /* ... */ }
  ```
  An arbitrary number of arguments of any type.

- ```rust
  pub fn uniform(arg_count: usize, valid_types: Vec<DataType>, volatility: Volatility) -> Self { /* ... */ }
  ```
  A fixed number of arguments of the same type, from those listed in `valid_types`.

- ```rust
  pub fn exact(exact_types: Vec<DataType>, volatility: Volatility) -> Self { /* ... */ }
  ```
  Exactly matches the types in `exact_types`, in order.

- ```rust
  pub fn coercible(target_types: Vec<Coercion>, volatility: Volatility) -> Self { /* ... */ }
  ```
  Target coerce types in order

- ```rust
  pub fn comparable(arg_count: usize, volatility: Volatility) -> Self { /* ... */ }
  ```
  Used for function that expects comparable data types, it will try to coerced all the types into single final one.

- ```rust
  pub fn nullary(volatility: Volatility) -> Self { /* ... */ }
  ```

- ```rust
  pub fn any(arg_count: usize, volatility: Volatility) -> Self { /* ... */ }
  ```
  A specified number of arguments of any type

- ```rust
  pub fn one_of(type_signatures: Vec<TypeSignature>, volatility: Volatility) -> Self { /* ... */ }
  ```
  Any one of a list of [TypeSignature]s.

- ```rust
  pub fn array_and_element(volatility: Volatility) -> Self { /* ... */ }
  ```
  Specialized Signature for ArrayAppend and similar functions

- ```rust
  pub fn array_and_element_and_optional_index(volatility: Volatility) -> Self { /* ... */ }
  ```
  Specialized Signature for Array functions with an optional index

- ```rust
  pub fn array_and_index(volatility: Volatility) -> Self { /* ... */ }
  ```
  Specialized Signature for ArrayElement and similar functions

- ```rust
  pub fn array(volatility: Volatility) -> Self { /* ... */ }
  ```
  Specialized Signature for ArrayEmpty and similar functions

###### Trait Implementations

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Signature) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **UnwindSafe**
- **ErasedDestructor**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Signature) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Signature { /* ... */ }
    ```

- **Sync**
- **Eq**
### Constants and Statics

#### Constant `TIMEZONE_WILDCARD`

Constant that is used as a placeholder for any valid timezone.
This is used where a function can accept a timestamp type with any
valid timezone, it exists to avoid the need to enumerate all possible
timezones. See [`TypeSignature`] for more details.

Type coercion always ensures that functions will be executed using
timestamp arrays that have a valid time zone. Functions must never
return results with this timezone.

```rust
pub const TIMEZONE_WILDCARD: &str = "+TZ";
```

#### Constant `FIXED_SIZE_LIST_WILDCARD`

Constant that is used as a placeholder for any valid fixed size list.
This is used where a function can accept a fixed size list type with any
valid length. It exists to avoid the need to enumerate all possible fixed size list lengths.

```rust
pub const FIXED_SIZE_LIST_WILDCARD: i32 = i32::MIN;
```

## Module `sort_properties`

```rust
pub mod sort_properties { /* ... */ }
```

### Types

#### Enum `SortProperties`

To propagate [`SortOptions`] across the `PhysicalExpr`, it is insufficient
to simply use `Option<SortOptions>`: There must be a differentiation between
unordered columns and literal values, since literals may not break the ordering
when they are used as a child of some binary expression when the other child has
some ordering. On the other hand, unordered columns cannot maintain ordering when
they take part in such operations.

Example: ((a_ordered + b_unordered) + c_ordered) expression cannot end up with
sorted data; however the ((a_ordered + 999) + c_ordered) expression can. Therefore,
we need two different variants for literals and unordered columns as literals are
often more ordering-friendly under most mathematical operations.

```rust
pub enum SortProperties {
    Ordered(arrow::compute::SortOptions),
    Unordered,
    Singleton,
}
```

##### Variants

###### `Ordered`

Use the ordinary [`SortOptions`] struct to represent ordered data:

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `arrow::compute::SortOptions` |  |

###### `Unordered`

###### `Singleton`

##### Implementations

###### Methods

- ```rust
  pub fn add(self: &Self, rhs: &Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn sub(self: &Self, rhs: &Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn gt_or_gteq(self: &Self, rhs: &Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn and_or(self: &Self, rhs: &Self) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> SortProperties { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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

- **Default**
  - ```rust
    fn default() -> SortProperties { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Freeze**
- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &SortProperties) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **IntoEither**
#### Struct `ExprProperties`

Represents the properties of a `PhysicalExpr`, including its sorting,
range, and whether it preserves lexicographical ordering.

```rust
pub struct ExprProperties {
    pub sort_properties: SortProperties,
    pub range: crate::interval_arithmetic::Interval,
    pub preserves_lex_ordering: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `sort_properties` | `SortProperties` | Properties that describe the sorting behavior of the expression,<br>such as whether it is ordered, unordered, or a singleton value. |
| `range` | `crate::interval_arithmetic::Interval` | A closed interval representing the range of possible values for<br>the expression. Used to compute reliable bounds. |
| `preserves_lex_ordering` | `bool` | Indicates whether the expression preserves lexicographical ordering<br>of its inputs. For example, string concatenation preserves ordering,<br>while addition does not. |

##### Implementations

###### Methods

- ```rust
  pub fn new_unknown() -> Self { /* ... */ }
  ```
  Creates a new `ExprProperties` instance with unknown sort properties,

- ```rust
  pub fn with_order(self: Self, order: SortProperties) -> Self { /* ... */ }
  ```
  Sets the sorting properties of the expression and returns the modified instance.

- ```rust
  pub fn with_range(self: Self, range: Interval) -> Self { /* ... */ }
  ```
  Sets the range of the expression and returns the modified instance.

- ```rust
  pub fn with_preserves_lex_ordering(self: Self, preserves_lex_ordering: bool) -> Self { /* ... */ }
  ```
  Sets whether the expression maintains lexicographical ordering and returns the modified instance.

###### Trait Implementations

- **Sync**
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExprProperties { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
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

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
## Module `statistics`

```rust
pub mod statistics { /* ... */ }
```

### Types

#### Enum `Distribution`

This object defines probabilistic distributions that encode uncertain
information about a single, scalar value. Currently, we support five core
statistical distributions. New variants will be added over time.

This object is the lowest-level object in the statistics hierarchy, and it
is the main unit of calculus when evaluating expressions in a statistical
context. Notions like column and table statistics are built on top of this
object and the operations it supports.

```rust
pub enum Distribution {
    Uniform(UniformDistribution),
    Exponential(ExponentialDistribution),
    Gaussian(GaussianDistribution),
    Bernoulli(BernoulliDistribution),
    Generic(GenericDistribution),
}
```

##### Variants

###### `Uniform`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `UniformDistribution` |  |

###### `Exponential`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ExponentialDistribution` |  |

###### `Gaussian`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `GaussianDistribution` |  |

###### `Bernoulli`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `BernoulliDistribution` |  |

###### `Generic`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `GenericDistribution` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new_uniform(interval: Interval) -> Result<Self> { /* ... */ }
  ```
  Constructs a new [`Uniform`] distribution from the given [`Interval`].

- ```rust
  pub fn new_exponential(rate: ScalarValue, offset: ScalarValue, positive_tail: bool) -> Result<Self> { /* ... */ }
  ```
  Constructs a new [`Exponential`] distribution from the given rate/offset

- ```rust
  pub fn new_gaussian(mean: ScalarValue, variance: ScalarValue) -> Result<Self> { /* ... */ }
  ```
  Constructs a new [`Gaussian`] distribution from the given mean/variance

- ```rust
  pub fn new_bernoulli(p: ScalarValue) -> Result<Self> { /* ... */ }
  ```
  Constructs a new [`Bernoulli`] distribution from the given success

- ```rust
  pub fn new_generic(mean: ScalarValue, median: ScalarValue, variance: ScalarValue, range: Interval) -> Result<Self> { /* ... */ }
  ```
  Constructs a new [`Generic`] distribution from the given mean, median,

- ```rust
  pub fn new_from_interval(range: Interval) -> Result<Self> { /* ... */ }
  ```
  Constructs a new [`Generic`] distribution from the given range. Other

- ```rust
  pub fn mean(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```
  Extracts the mean value of this uncertain quantity, depending on its

- ```rust
  pub fn median(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```
  Extracts the median value of this uncertain quantity, depending on its

- ```rust
  pub fn variance(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```
  Extracts the variance value of this uncertain quantity, depending on

- ```rust
  pub fn range(self: &Self) -> Result<Interval> { /* ... */ }
  ```
  Extracts the range of this uncertain quantity, depending on its

- ```rust
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```
  Returns the data type of the statistical parameters comprising this

- ```rust
  pub fn target_type(args: &[&ScalarValue]) -> Result<DataType> { /* ... */ }
  ```

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

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

- **ErasedDestructor**
- **IntoEither**
- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Distribution { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Distribution) -> bool { /* ... */ }
    ```

#### Struct `UniformDistribution`

Uniform distribution, represented by its range. If the given range extends
towards infinity, the distribution will be improper -- which is OK. For a
more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Continuous_uniform_distribution>
<https://en.wikipedia.org/wiki/Prior_probability#Improper_priors>

```rust
pub struct UniformDistribution {
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
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```

- ```rust
  pub fn mean(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```
  Computes the mean value of this distribution. In case of improper

- ```rust
  pub fn median(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```

- ```rust
  pub fn variance(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```
  Computes the variance value of this distribution. In case of improper

- ```rust
  pub fn range(self: &Self) -> &Interval { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> UniformDistribution { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &UniformDistribution) -> bool { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
#### Struct `ExponentialDistribution`

Exponential distribution with an optional shift. The probability density
function (PDF) is defined as follows:

For a positive tail (when `positive_tail` is `true`):

`f(x; λ, offset) = λ exp(-λ (x - offset))    for x ≥ offset`

For a negative tail (when `positive_tail` is `false`):

`f(x; λ, offset) = λ exp(-λ (offset - x))    for x ≤ offset`


In both cases, the PDF is `0` outside the specified domain.

For more information, see:

<https://en.wikipedia.org/wiki/Exponential_distribution>

```rust
pub struct ExponentialDistribution {
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
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```

- ```rust
  pub fn rate(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn offset(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn positive_tail(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn mean(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```

- ```rust
  pub fn median(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```

- ```rust
  pub fn variance(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```

- ```rust
  pub fn range(self: &Self) -> Result<Interval> { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Unpin**
- **MaybeSendSync**
- **RefUnwindSafe**
- **UnwindSafe**
- **IntoEither**
- **Sync**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExponentialDistribution { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExponentialDistribution) -> bool { /* ... */ }
    ```

#### Struct `GaussianDistribution`

Gaussian (normal) distribution, represented by its mean and variance.
For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Normal_distribution>

```rust
pub struct GaussianDistribution {
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
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```

- ```rust
  pub fn mean(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn variance(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn median(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn range(self: &Self) -> Result<Interval> { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Freeze**
- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GaussianDistribution) -> bool { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GaussianDistribution { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **MaybeSendSync**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `BernoulliDistribution`

Bernoulli distribution with success probability `p`. If `p` has a null value,
the success probability is unknown. For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Bernoulli_distribution>

```rust
pub struct BernoulliDistribution {
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
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```

- ```rust
  pub fn p_value(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn mean(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn median(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```
  Computes the median value of this distribution. In case of an unknown

- ```rust
  pub fn variance(self: &Self) -> Result<ScalarValue> { /* ... */ }
  ```
  Computes the variance value of this distribution. In case of an unknown

- ```rust
  pub fn range(self: &Self) -> Interval { /* ... */ }
  ```

###### Trait Implementations

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> BernoulliDistribution { /* ... */ }
    ```

- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BernoulliDistribution) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Sync**
#### Struct `GenericDistribution`

A generic distribution whose functional form is not available, which is
approximated via some summary statistics. For a more in-depth discussion, see:

<https://en.wikipedia.org/wiki/Summary_statistics>

```rust
pub struct GenericDistribution {
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
  pub fn data_type(self: &Self) -> DataType { /* ... */ }
  ```

- ```rust
  pub fn mean(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn median(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn variance(self: &Self) -> &ScalarValue { /* ... */ }
  ```

- ```rust
  pub fn range(self: &Self) -> &Interval { /* ... */ }
  ```

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Sync**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GenericDistribution { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **RefUnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GenericDistribution) -> bool { /* ... */ }
    ```

### Functions

#### Function `combine_bernoullis`

This function takes a logical operator and two Bernoulli distributions,
and it returns a new Bernoulli distribution that represents the result of
the operation. Currently, only `AND` and `OR` operations are supported.

```rust
pub fn combine_bernoullis(op: &crate::operator::Operator, left: &BernoulliDistribution, right: &BernoulliDistribution) -> datafusion_common::Result<BernoulliDistribution> { /* ... */ }
```

#### Function `combine_gaussians`

Applies the given operation to the given Gaussian distributions. Currently,
this function handles only addition and subtraction operations. If the
result is not a Gaussian random variable, it returns `None`. For details,
see:

<https://en.wikipedia.org/wiki/Sum_of_normally_distributed_random_variables>

```rust
pub fn combine_gaussians(op: &crate::operator::Operator, left: &GaussianDistribution, right: &GaussianDistribution) -> datafusion_common::Result<Option<GaussianDistribution>> { /* ... */ }
```

#### Function `create_bernoulli_from_comparison`

Creates a new `Bernoulli` distribution by computing the resulting probability.
Expects `op` to be a comparison operator, with `left` and `right` having
numeric distributions. The resulting distribution has the `Float64` data
type.

```rust
pub fn create_bernoulli_from_comparison(op: &crate::operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<Distribution> { /* ... */ }
```

#### Function `new_generic_from_binary_op`

Creates a new [`Generic`] distribution that represents the result of the
given binary operation on two unknown quantities represented by their
[`Distribution`] objects. The function computes the mean, median and
variance if possible.

```rust
pub fn new_generic_from_binary_op(op: &crate::operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<Distribution> { /* ... */ }
```

#### Function `compute_mean`

Computes the mean value for the result of the given binary operation on
two unknown quantities represented by their [`Distribution`] objects.

```rust
pub fn compute_mean(op: &crate::operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue> { /* ... */ }
```

#### Function `compute_median`

Computes the median value for the result of the given binary operation on
two unknown quantities represented by its [`Distribution`] objects. Currently,
the median is calculable only for addition and subtraction operations on:
- [`Uniform`] and [`Uniform`] distributions, and
- [`Gaussian`] and [`Gaussian`] distributions.

```rust
pub fn compute_median(op: &crate::operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue> { /* ... */ }
```

#### Function `compute_variance`

Computes the variance value for the result of the given binary operation on
two unknown quantities represented by their [`Distribution`] objects.

```rust
pub fn compute_variance(op: &crate::operator::Operator, left: &Distribution, right: &Distribution) -> datafusion_common::Result<datafusion_common::ScalarValue> { /* ... */ }
```

## Module `type_coercion`

```rust
pub mod type_coercion { /* ... */ }
```

### Modules

## Module `aggregates`

```rust
pub mod aggregates { /* ... */ }
```

### Functions

#### Function `check_arg_count`

Validate the length of `input_types` matches the `signature` for `agg_fun`.

This method DOES NOT validate the argument types - only that (at least one,
in the case of [`TypeSignature::OneOf`]) signature matches the desired
number of input types.

```rust
pub fn check_arg_count(func_name: &str, input_types: &[arrow::datatypes::DataType], signature: &crate::signature::TypeSignature) -> datafusion_common::Result<()> { /* ... */ }
```

#### Function `sum_return_type`

Function return type of a sum

```rust
pub fn sum_return_type(arg_type: &arrow::datatypes::DataType) -> datafusion_common::Result<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `variance_return_type`

Function return type of variance

```rust
pub fn variance_return_type(arg_type: &arrow::datatypes::DataType) -> datafusion_common::Result<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `covariance_return_type`

Function return type of covariance

```rust
pub fn covariance_return_type(arg_type: &arrow::datatypes::DataType) -> datafusion_common::Result<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `correlation_return_type`

Function return type of correlation

```rust
pub fn correlation_return_type(arg_type: &arrow::datatypes::DataType) -> datafusion_common::Result<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `avg_return_type`

Function return type of an average

```rust
pub fn avg_return_type(func_name: &str, arg_type: &arrow::datatypes::DataType) -> datafusion_common::Result<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `avg_sum_type`

Internal sum type of an average

```rust
pub fn avg_sum_type(arg_type: &arrow::datatypes::DataType) -> datafusion_common::Result<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `is_sum_support_arg_type`

```rust
pub fn is_sum_support_arg_type(arg_type: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_avg_support_arg_type`

```rust
pub fn is_avg_support_arg_type(arg_type: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_variance_support_arg_type`

```rust
pub fn is_variance_support_arg_type(arg_type: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_covariance_support_arg_type`

```rust
pub fn is_covariance_support_arg_type(arg_type: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_correlation_support_arg_type`

```rust
pub fn is_correlation_support_arg_type(arg_type: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_integer_arg_type`

```rust
pub fn is_integer_arg_type(arg_type: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `coerce_avg_type`

```rust
pub fn coerce_avg_type(func_name: &str, arg_types: &[arrow::datatypes::DataType]) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>> { /* ... */ }
```

### Constants and Statics

#### Static `STRINGS`

```rust
pub static STRINGS: &[arrow::datatypes::DataType] = _;
```

#### Static `SIGNED_INTEGERS`

```rust
pub static SIGNED_INTEGERS: &[arrow::datatypes::DataType] = _;
```

#### Static `UNSIGNED_INTEGERS`

```rust
pub static UNSIGNED_INTEGERS: &[arrow::datatypes::DataType] = _;
```

#### Static `INTEGERS`

```rust
pub static INTEGERS: &[arrow::datatypes::DataType] = _;
```

#### Static `NUMERICS`

```rust
pub static NUMERICS: &[arrow::datatypes::DataType] = _;
```

#### Static `TIMESTAMPS`

```rust
pub static TIMESTAMPS: &[arrow::datatypes::DataType] = _;
```

#### Static `DATES`

```rust
pub static DATES: &[arrow::datatypes::DataType] = _;
```

#### Static `BINARYS`

```rust
pub static BINARYS: &[arrow::datatypes::DataType] = _;
```

#### Static `TIMES`

```rust
pub static TIMES: &[arrow::datatypes::DataType] = _;
```

## Module `binary`

Coercion rules for matching argument types for binary operators

```rust
pub mod binary { /* ... */ }
```

### Types

#### Struct `BinaryTypeCoercer`

Provides type information about a binary expression, coercing different
input types into a sensible output type.

```rust
pub struct BinaryTypeCoercer<''a> {
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
  pub fn new(lhs: &''a DataType, op: &''a Operator, rhs: &''a DataType) -> Self { /* ... */ }
  ```
  Creates a new [`BinaryTypeCoercer`], for reasoning about the input

- ```rust
  pub fn set_lhs_spans(self: &mut Self, spans: Spans) { /* ... */ }
  ```
  Sets the spans information for the left side of the binary expression,

- ```rust
  pub fn set_op_spans(self: &mut Self, spans: Spans) { /* ... */ }
  ```
  Sets the spans information for the operator of the binary expression, so

- ```rust
  pub fn set_rhs_spans(self: &mut Self, spans: Spans) { /* ... */ }
  ```
  Sets the spans information for the right side of the binary expression,

- ```rust
  pub fn get_result_type(self: &''a Self) -> Result<DataType> { /* ... */ }
  ```
  Returns the resulting type of a binary expression evaluating the `op` with the left and right hand types

- ```rust
  pub fn get_input_types(self: &''a Self) -> Result<(DataType, DataType)> { /* ... */ }
  ```
  Returns the coerced input types for a binary expression evaluating the `op` with the left and right hand types

###### Trait Implementations

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Freeze**
- **MaybeSendSync**
- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
### Functions

#### Function `type_union_resolution`

Coerce dissimilar data types to a single data type.
UNION, INTERSECT, EXCEPT, CASE, ARRAY, VALUES, and the GREATEST and LEAST functions are
examples that has the similar resolution rules.
See <https://www.postgresql.org/docs/current/typeconv-union-case.html> for more information.
The rules in the document provide a clue, but adhering strictly to them doesn't precisely
align with the behavior of Postgres. Therefore, we've made slight adjustments to the rules
to better match the behavior of both Postgres and DuckDB. For example, we expect adjusted
decimal precision and scale when coercing decimal types.

This function doesn't preserve correct field name and nullability for the struct type, we only care about data type.

Returns Option because we might want to continue on the code even if the data types are not coercible to the common type

```rust
pub fn type_union_resolution(data_types: &[arrow::datatypes::DataType]) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `try_type_union_resolution`

Handle type union resolution including struct type and others.

```rust
pub fn try_type_union_resolution(data_types: &[arrow::datatypes::DataType]) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>> { /* ... */ }
```

#### Function `try_type_union_resolution_with_struct`

```rust
pub fn try_type_union_resolution_with_struct(data_types: &[arrow::datatypes::DataType]) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>> { /* ... */ }
```

#### Function `comparison_coercion`

Coerce `lhs_type` and `rhs_type` to a common type for the purposes of a
comparison operation

Example comparison operations are `lhs = rhs` and `lhs > rhs`

Binary comparison kernels require the two arguments to be the (exact) same
data type. However, users can write queries where the two arguments are
different data types. In such cases, the data types are automatically cast
(coerced) to a single data type to pass to the kernels.

# Numeric comparisons

When comparing numeric values, the lower precision type is coerced to the
higher precision type to avoid losing data. For example when comparing
`Int32` to `Int64` the coerced type is `Int64` so the `Int32` argument will
be cast.

# Numeric / String comparisons

When comparing numeric values and strings, both values will be coerced to
strings.  For example when comparing `'2' > 1`,  the arguments will be
coerced to `Utf8` for comparison

```rust
pub fn comparison_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `comparison_coercion_numeric`

Similar to [`comparison_coercion`] but prefers numeric if compares with
numeric and string

# Numeric comparisons

When comparing numeric values and strings, the values will be coerced to the
numeric type.  For example, `'2' > 1` if `1` is an `Int32`, the arguments
will be coerced to `Int32`.

```rust
pub fn comparison_coercion_numeric(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `binary_numeric_coercion`

Coerce `lhs_type` and `rhs_type` to a common type where both are numeric

```rust
pub fn binary_numeric_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `decimal_coercion`

Decimal coercion rules.

```rust
pub fn decimal_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `string_coercion`

Coercion rules for string view types (Utf8/LargeUtf8/Utf8View):
If at least one argument is a string view, we coerce to string view
based on the observation that StringArray to StringViewArray is cheap but not vice versa.

Between Utf8 and LargeUtf8, we coerce to LargeUtf8.

```rust
pub fn string_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `binary_to_string_coercion`

Coercion rules for binary (Binary/LargeBinary) to string (Utf8/LargeUtf8):
If one argument is binary and the other is a string then coerce to string
(e.g. for `like`)

```rust
pub fn binary_to_string_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `like_coercion`

Coercion rules for like operations.
This is a union of string coercion rules and dictionary coercion rules

```rust
pub fn like_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `regex_coercion`

Coercion rules for regular expression comparison operations.
This is a union of string coercion rules and dictionary coercion rules

```rust
pub fn regex_coercion(lhs_type: &arrow::datatypes::DataType, rhs_type: &arrow::datatypes::DataType) -> Option<arrow::datatypes::DataType> { /* ... */ }
```


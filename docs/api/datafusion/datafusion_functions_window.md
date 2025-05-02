# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_functions_window`

Window Function packages for [DataFusion].

This crate contains a collection of various window function packages for DataFusion,
implemented using the extension API.

[DataFusion]: https://crates.io/crates/datafusion


## Modules

## Module `macros`

**Attributes:**

- `#[macro_use]`

Convenience macros for defining a user-defined window function
and associated expression API (fluent style).

See [`define_udwf_and_expr!`] for usage examples.

[`define_udwf_and_expr!`]: crate::define_udwf_and_expr!

```rust
pub mod macros { /* ... */ }
```

## Module `cume_dist`

`cume_dist` window function implementation

```rust
pub mod cume_dist { /* ... */ }
```

### Types

#### Struct `CumeDist`

CumeDist calculates the cume_dist in the window function with order by

```rust
pub struct CumeDist {
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

###### Trait Implementations

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **WindowUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```
    Return a reference to Any that can be used for downcasting

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn partition_evaluator(self: &Self, _partition_evaluator_args: PartitionEvaluatorArgs<''_>) -> Result<Box<dyn PartitionEvaluator>> { /* ... */ }
    ```

  - ```rust
    fn field(self: &Self, field_args: WindowUDFFieldArgs<''_>) -> Result<Field> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **Unpin**
- **MaybeSendSync**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

### Functions

#### Function `cume_dist_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`cume_dist`].

Calculates the cumulative distribution of a value in a group of values.

```rust
pub fn cume_dist_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `cume_dist`

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`CumeDist` user-defined window function.

Calculates the cumulative distribution of a value in a group of values.

```rust
pub fn cume_dist() -> datafusion_expr::Expr { /* ... */ }
```

## Module `lead_lag`

`lead` and `lag` window function implementations

```rust
pub mod lead_lag { /* ... */ }
```

### Types

#### Struct `WindowShift`

window shift expression

```rust
pub struct WindowShift {
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
  pub fn lag() -> Self { /* ... */ }
  ```

- ```rust
  pub fn lead() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **ErasedDestructor**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Freeze**
- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **WindowUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn expressions(self: &Self, expr_args: ExpressionArgs<''_>) -> Vec<Arc<dyn PhysicalExpr>> { /* ... */ }
    ```
    Handles the case where `NULL` expression is passed as an

  - ```rust
    fn partition_evaluator(self: &Self, partition_evaluator_args: PartitionEvaluatorArgs<''_>) -> Result<Box<dyn PartitionEvaluator>> { /* ... */ }
    ```

  - ```rust
    fn field(self: &Self, field_args: WindowUDFFieldArgs<''_>) -> Result<Field> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDWF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

### Functions

#### Function `lag_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`lag`].

Returns the row value that precedes the current row by a specified offset within partition. If no such row exists, then returns the default value.

```rust
pub fn lag_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `lead_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`lead`].

Returns the value from a row that follows the current row by a specified offset within the partition. If no such row exists, then returns the default value.

```rust
pub fn lead_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `lag`

Create an expression to represent the `lag` window function

returns value evaluated at the row that is offset rows before the current row within the partition;
if there is no such row, instead return default (which must be of the same type as value).
Both offset and default are evaluated with respect to the current row.
If omitted, offset defaults to 1 and default to null

```rust
pub fn lag(arg: datafusion_expr::Expr, shift_offset: Option<i64>, default_value: Option<datafusion_common::ScalarValue>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `lead`

Create an expression to represent the `lead` window function

returns value evaluated at the row that is offset rows after the current row within the partition;
if there is no such row, instead return default (which must be of the same type as value).
Both offset and default are evaluated with respect to the current row.
If omitted, offset defaults to 1 and default to null

```rust
pub fn lead(arg: datafusion_expr::Expr, shift_offset: Option<i64>, default_value: Option<datafusion_common::ScalarValue>) -> datafusion_expr::Expr { /* ... */ }
```

## Module `nth_value`

`nth_value` window function implementation

```rust
pub mod nth_value { /* ... */ }
```

### Types

#### Enum `NthValueKind`

Tag to differentiate special use cases of the NTH_VALUE built-in window function.

```rust
pub enum NthValueKind {
    First,
    Last,
    Nth,
}
```

##### Variants

###### `First`

###### `Last`

###### `Nth`

##### Implementations

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Sync**
- **UnwindSafe**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Allocation**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **ErasedDestructor**
- **Copy**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> NthValueKind { /* ... */ }
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `NthValue`

```rust
pub struct NthValue {
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
  pub fn new(kind: NthValueKind) -> Self { /* ... */ }
  ```
  Create a new `nth_value` function

- ```rust
  pub fn first() -> Self { /* ... */ }
  ```

- ```rust
  pub fn last() -> Self { /* ... */ }
  ```

- ```rust
  pub fn nth() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **WindowUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn partition_evaluator(self: &Self, partition_evaluator_args: PartitionEvaluatorArgs<''_>) -> Result<Box<dyn PartitionEvaluator>> { /* ... */ }
    ```

  - ```rust
    fn field(self: &Self, field_args: WindowUDFFieldArgs<''_>) -> Result<Field> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDWF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **MaybeSendSync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `NthValueState`

```rust
pub struct NthValueState {
    pub finalized_result: Option<datafusion_common::ScalarValue>,
    pub kind: NthValueKind,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `finalized_result` | `Option<datafusion_common::ScalarValue>` |  |
| `kind` | `NthValueKind` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **MaybeSendSync**
- **RefUnwindSafe**
- **ErasedDestructor**
- **Sync**
- **Unpin**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> NthValueState { /* ... */ }
    ```

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

- **IntoEither**
### Functions

#### Function `first_value_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`first_value`].

returns the first value in the window frame

```rust
pub fn first_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `last_value_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`last_value`].

returns the last value in the window frame

```rust
pub fn last_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `nth_value_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`nth_value`].

returns the nth value in the window frame

```rust
pub fn nth_value_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `first_value`

Create an expression to represent the `first_value` window function


```rust
pub fn first_value(arg: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `last_value`

Create an expression to represent the `last_value` window function


```rust
pub fn last_value(arg: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `nth_value`

Create an expression to represent the `nth_value` window function


```rust
pub fn nth_value(arg: datafusion_expr::Expr, n: i64) -> datafusion_expr::Expr { /* ... */ }
```

## Module `ntile`

`ntile` window function implementation

```rust
pub mod ntile { /* ... */ }
```

### Types

#### Struct `Ntile`

```rust
pub struct Ntile {
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
  Create a new `ntile` function

###### Trait Implementations

- **Sync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **WindowUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn partition_evaluator(self: &Self, partition_evaluator_args: PartitionEvaluatorArgs<''_>) -> Result<Box<dyn PartitionEvaluator>> { /* ... */ }
    ```

  - ```rust
    fn field(self: &Self, field_args: WindowUDFFieldArgs<''_>) -> Result<Field> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Unpin**
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
- **MaybeSendSync**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
### Functions

#### Function `ntile_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`ntile`].

integer ranging from 1 to the argument value, dividing the partition as equally as possible

```rust
pub fn ntile_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `ntile`

```rust
pub fn ntile(arg: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

## Module `rank`

Implementation of `rank`, `dense_rank`, and `percent_rank` window functions,
which can be evaluated at runtime during query execution.

```rust
pub mod rank { /* ... */ }
```

### Types

#### Struct `Rank`

Rank calculates the rank in the window function with order by

```rust
pub struct Rank {
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
  pub fn new(name: String, rank_type: RankType) -> Self { /* ... */ }
  ```
  Create a new `rank` function with the specified name and rank type

- ```rust
  pub fn basic() -> Self { /* ... */ }
  ```
  Create a `rank` window function

- ```rust
  pub fn dense_rank() -> Self { /* ... */ }
  ```
  Create a `dense_rank` window function

- ```rust
  pub fn percent_rank() -> Self { /* ... */ }
  ```
  Create a `percent_rank` window function

###### Trait Implementations

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **RefUnwindSafe**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **WindowUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn partition_evaluator(self: &Self, _partition_evaluator_args: PartitionEvaluatorArgs<''_>) -> Result<Box<dyn PartitionEvaluator>> { /* ... */ }
    ```

  - ```rust
    fn field(self: &Self, field_args: WindowUDFFieldArgs<''_>) -> Result<Field> { /* ... */ }
    ```

  - ```rust
    fn sort_options(self: &Self) -> Option<SortOptions> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

#### Enum `RankType`

```rust
pub enum RankType {
    Basic,
    Dense,
    Percent,
}
```

##### Variants

###### `Basic`

###### `Dense`

###### `Percent`

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Copy**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RankType { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Send**
#### Struct `RankState`

State for the RANK(rank) built-in window function.

```rust
pub struct RankState {
    pub last_rank_data: Option<Vec<datafusion_common::ScalarValue>>,
    pub last_rank_boundary: usize,
    pub current_group_count: usize,
    pub n_rank: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `last_rank_data` | `Option<Vec<datafusion_common::ScalarValue>>` | The last values for rank as these values change, we increase n_rank |
| `last_rank_boundary` | `usize` | The index where last_rank_boundary is started |
| `current_group_count` | `usize` | Keep the number of entries in current rank |
| `n_rank` | `usize` | Rank number kept from the start |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> RankState { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RankState { /* ... */ }
    ```

### Functions

#### Function `rank_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`rank`].

Returns rank of the current row with gaps. Same as `row_number` of its first peer

```rust
pub fn rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `rank`

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`Rank` user-defined window function.

Returns rank of the current row with gaps. Same as `row_number` of its first peer

```rust
pub fn rank() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `dense_rank_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`dense_rank`].

Returns rank of the current row without gaps. This function counts peer groups

```rust
pub fn dense_rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `dense_rank`

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`DenseRank` user-defined window function.

Returns rank of the current row without gaps. This function counts peer groups

```rust
pub fn dense_rank() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `percent_rank_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`percent_rank`].

Returns the relative rank of the current row: (rank - 1) / (total rows - 1)

```rust
pub fn percent_rank_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `percent_rank`

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`PercentRank` user-defined window function.

Returns the relative rank of the current row: (rank - 1) / (total rows - 1)

```rust
pub fn percent_rank() -> datafusion_expr::Expr { /* ... */ }
```

## Module `row_number`

`row_number` window function implementation

```rust
pub mod row_number { /* ... */ }
```

### Types

#### Struct `RowNumber`

row_number expression

```rust
pub struct RowNumber {
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
  Create a new `row_number` function

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **WindowUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn partition_evaluator(self: &Self, _partition_evaluator_args: PartitionEvaluatorArgs<''_>) -> Result<Box<dyn PartitionEvaluator>> { /* ... */ }
    ```

  - ```rust
    fn field(self: &Self, field_args: WindowUDFFieldArgs<''_>) -> Result<Field> { /* ... */ }
    ```

  - ```rust
    fn sort_options(self: &Self) -> Option<SortOptions> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Functions

#### Function `row_number_udwf`

Returns a [`WindowUDF`](datafusion_expr::WindowUDF) for [`row_number`].

Returns a unique row number for each row in window partition beginning at 1.

```rust
pub fn row_number_udwf() -> std::sync::Arc<datafusion_expr::WindowUDF> { /* ... */ }
```

#### Function `row_number`

Create a [`WindowFunction`](datafusion_expr::Expr::WindowFunction) expression for
`RowNumber` user-defined window function.

Returns a unique row number for each row in window partition beginning at 1.

```rust
pub fn row_number() -> datafusion_expr::Expr { /* ... */ }
```

## Module `planner`

SQL planning extensions like [`WindowFunctionPlanner`]

```rust
pub mod planner { /* ... */ }
```

### Types

#### Struct `WindowFunctionPlanner`

```rust
pub struct WindowFunctionPlanner;
```

##### Implementations

###### Trait Implementations

- **Allocation**
- **UnwindSafe**
- **Send**
- **MaybeSendSync**
- **ExprPlanner**
  - ```rust
    fn plan_window(self: &Self, raw_expr: RawWindowExpr) -> Result<PlannerResult<RawWindowExpr>> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `expr_fn`

Fluent-style API for creating `Expr`s

```rust
pub mod expr_fn { /* ... */ }
```

### Re-exports

#### Re-export `cume_dist`

```rust
pub use super::cume_dist::cume_dist;
```

#### Re-export `lag`

```rust
pub use super::lead_lag::lag;
```

#### Re-export `lead`

```rust
pub use super::lead_lag::lead;
```

#### Re-export `first_value`

```rust
pub use super::nth_value::first_value;
```

#### Re-export `last_value`

```rust
pub use super::nth_value::last_value;
```

#### Re-export `nth_value`

```rust
pub use super::nth_value::nth_value;
```

#### Re-export `ntile`

```rust
pub use super::ntile::ntile;
```

#### Re-export `dense_rank`

```rust
pub use super::rank::dense_rank;
```

#### Re-export `percent_rank`

```rust
pub use super::rank::percent_rank;
```

#### Re-export `rank`

```rust
pub use super::rank::rank;
```

#### Re-export `row_number`

```rust
pub use super::row_number::row_number;
```

## Functions

### Function `all_default_window_functions`

Returns all default window functions

```rust
pub fn all_default_window_functions() -> Vec<std::sync::Arc<datafusion_expr::WindowUDF>> { /* ... */ }
```

### Function `register_all`

Registers all enabled packages with a [`FunctionRegistry`]

```rust
pub fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()> { /* ... */ }
```

## Macros

### Macro `get_or_init_udwf`

**Attributes:**

- `#[macro_export]`

Lazily initializes a user-defined window function exactly once
when called concurrently. Repeated calls return a reference to the
same instance.

# Parameters

* `$UDWF`: The struct which defines the [`Signature`](datafusion_expr::Signature)
  of the user-defined window function.
* `$OUT_FN_NAME`: The basename to generate a unique function name like
  `$OUT_FN_NAME_udwf`.
* `$DOC`: Doc comments for UDWF.
* (optional) `$CTOR`: Pass a custom constructor. When omitted it
  automatically resolves to `$UDWF::default()`.

# Example

```
# use std::any::Any;
# use datafusion_common::arrow::datatypes::{DataType, Field};
# use datafusion_expr::{PartitionEvaluator, Signature, Volatility, WindowUDFImpl};
#
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window::get_or_init_udwf;
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// Defines the `simple_udwf()` user-defined window function.
get_or_init_udwf!(
    SimpleUDWF,
    simple,
    "Simple user-defined window function doc comment."
);
#
# assert_eq!(simple_udwf().name(), "simple_user_defined_window_function");
#
#  #[derive(Debug)]
#  struct SimpleUDWF {
#      signature: Signature,
#  }
#
#  impl Default for SimpleUDWF {
#      fn default() -> Self {
#          Self {
#             signature: Signature::any(0, Volatility::Immutable),
#          }
#      }
#  }
#
#  impl WindowUDFImpl for SimpleUDWF {
#      fn as_any(&self) -> &dyn Any {
#          self
#      }
#      fn name(&self) -> &str {
#          "simple_user_defined_window_function"
#      }
#      fn signature(&self) -> &Signature {
#          &self.signature
#      }
#      fn partition_evaluator(
#          &self,
#         _partition_evaluator_args: PartitionEvaluatorArgs,
#      ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#          unimplemented!()
#      }
#      fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<Field> {
#          Ok(Field::new(field_args.name(), DataType::Int64, false))
#      }
#  }
#
```

```rust
pub macro_rules! get_or_init_udwf {
    /* macro_rules! get_or_init_udwf {
    ($UDWF:ident, $OUT_FN_NAME:ident, $DOC:expr) => { ... };
    ($UDWF:ident, $OUT_FN_NAME:ident, $DOC:expr, $CTOR:path) => { ... };
} */
}
```

### Macro `create_udwf_expr`

**Attributes:**

- `#[macro_export]`

Create a [`WindowFunction`] expression that exposes a fluent API
which you can use to build more complex expressions.

[`WindowFunction`]: datafusion_expr::Expr::WindowFunction

# Parameters

* `$UDWF`: The struct which defines the [`Signature`] of the
  user-defined window function.
* `$OUT_FN_NAME`: The basename to generate a unique function name like
  `$OUT_FN_NAME_udwf`.
* `$DOC`: Doc comments for UDWF.
* (optional) `[$($PARAM:ident),+]`: An array of 1 or more parameters
  for the generated function. The type of parameters is [`Expr`].
  When omitted this creates a function with zero parameters.

[`Signature`]: datafusion_expr::Signature
[`Expr`]: datafusion_expr::Expr

# Example

1. With Zero Parameters
```
# use std::any::Any;
# use datafusion_common::arrow::datatypes::{DataType, Field};
# use datafusion_expr::{PartitionEvaluator, Signature, Volatility, WindowUDFImpl};
# use datafusion_functions_window::{create_udwf_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;

# get_or_init_udwf!(
#     RowNumber,
#     row_number,
#     "Returns a unique row number for each row in window partition beginning at 1."
# );
/// Creates `row_number()` API which has zero parameters:
///
///     ```
///     /// Returns a unique row number for each row in window partition
///     /// beginning at 1.
///     pub fn row_number() -> datafusion_expr::Expr {
///        row_number_udwf().call(vec![])
///     }
///     ```
create_udwf_expr!(
    RowNumber,
    row_number,
    "Returns a unique row number for each row in window partition beginning at 1."
);
#
# assert_eq!(
#     row_number().name_for_alias().unwrap(),
#     "row_number() ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
# );
#
# #[derive(Debug)]
# struct RowNumber {
#     signature: Signature,
# }
# impl Default for RowNumber {
#     fn default() -> Self {
#         Self {
#             signature: Signature::any(0, Volatility::Immutable),
#         }
#     }
# }
# impl WindowUDFImpl for RowNumber {
#     fn as_any(&self) -> &dyn Any {
#         self
#     }
#     fn name(&self) -> &str {
#         "row_number"
#     }
#     fn signature(&self) -> &Signature {
#         &self.signature
#     }
#     fn partition_evaluator(
#         &self,
#         _partition_evaluator_args: PartitionEvaluatorArgs,
#     ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#         unimplemented!()
#     }
#     fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<Field> {
#         Ok(Field::new(field_args.name(), DataType::UInt64, false))
#     }
# }
```

2. With Multiple Parameters
```
# use std::any::Any;
#
# use datafusion_expr::{
#     PartitionEvaluator, Signature, TypeSignature, Volatility, WindowUDFImpl,
# };
#
# use datafusion_functions_window::{create_udwf_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
#
# use datafusion_common::arrow::datatypes::Field;
# use datafusion_common::ScalarValue;
# use datafusion_expr::{col, lit};
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
# get_or_init_udwf!(Lead, lead, "user-defined window function");
#
/// Creates `lead(expr, offset, default)` with 3 parameters:
///
///     ```
///     /// Returns a value evaluated at the row that is offset rows
///     /// after the current row within the partition.
///     pub fn lead(
///         expr: datafusion_expr::Expr,
///         offset: datafusion_expr::Expr,
///         default: datafusion_expr::Expr,
///     ) -> datafusion_expr::Expr {
///         lead_udwf().call(vec![expr, offset, default])
///     }
///     ```
create_udwf_expr!(
    Lead,
    lead,
    [expr, offset, default],
    "Returns a value evaluated at the row that is offset rows after the current row within the partition."
);
#
# assert_eq!(
#     lead(col("a"), lit(1i64), lit(ScalarValue::Null))
#         .name_for_alias()
#         .unwrap(),
#     "lead(a,Int64(1),NULL) ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
# );
#
# #[derive(Debug)]
# struct Lead {
#     signature: Signature,
# }
#
# impl Default for Lead {
#     fn default() -> Self {
#         Self {
#             signature: Signature::one_of(
#                 vec![
#                     TypeSignature::Any(1),
#                     TypeSignature::Any(2),
#                     TypeSignature::Any(3),
#                 ],
#                 Volatility::Immutable,
#             ),
#         }
#     }
# }
#
# impl WindowUDFImpl for Lead {
#     fn as_any(&self) -> &dyn Any {
#         self
#     }
#     fn name(&self) -> &str {
#         "lead"
#     }
#     fn signature(&self) -> &Signature {
#         &self.signature
#     }
#     fn partition_evaluator(
#         &self,
#         partition_evaluator_args: PartitionEvaluatorArgs,
#     ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#         unimplemented!()
#     }
#     fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<Field> {
#         Ok(Field::new(
#             field_args.name(),
#             field_args.get_input_type(0).unwrap(),
#             false,
#         ))
#     }
# }
```

```rust
pub macro_rules! create_udwf_expr {
    /* macro_rules! create_udwf_expr {
    ($UDWF:ident, $OUT_FN_NAME:ident, $DOC:expr) => { ... };
    ($UDWF:ident, $OUT_FN_NAME:ident, [$($PARAM:ident),+], $DOC:expr) => { ... };
} */
}
```

### Macro `define_udwf_and_expr`

**Attributes:**

- `#[macro_export]`

Defines a user-defined window function.

Combines [`get_or_init_udwf!`] and [`create_udwf_expr!`] into a
single macro for convenience.

# Arguments

* `$UDWF`: The struct which defines the [`Signature`] of the
  user-defined window function.
* `$OUT_FN_NAME`: The basename to generate a unique function name like
  `$OUT_FN_NAME_udwf`.
* (optional) `[$($PARAM:ident),+]`: An array of 1 or more parameters
  for the generated function. The type of parameters is [`Expr`].
  When omitted this creates a function with zero parameters.
* `$DOC`: Doc comments for UDWF.
* (optional) `$CTOR`: Pass a custom constructor. When omitted it
  automatically resolves to `$UDWF::default()`.

[`Signature`]: datafusion_expr::Signature
[`Expr`]: datafusion_expr::Expr

# Usage

## Expression API With Zero parameters
1. Uses default constructor for UDWF.

```
# use std::any::Any;
# use datafusion_common::arrow::datatypes::{DataType, Field};
# use datafusion_expr::{PartitionEvaluator, Signature, Volatility, WindowUDFImpl};
#
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window::{define_udwf_and_expr, get_or_init_udwf, create_udwf_expr};
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// 1. Defines the `simple_udwf()` user-defined window function.
///
/// 2. Defines the expression API:
///     ```
///     pub fn simple() -> datafusion_expr::Expr {
///         simple_udwf().call(vec![])
///     }
///     ```
define_udwf_and_expr!(
    SimpleUDWF,
    simple,
    "a simple user-defined window function"
);
#
# assert_eq!(simple_udwf().name(), "simple_user_defined_window_function");
#
#  #[derive(Debug)]
#  struct SimpleUDWF {
#      signature: Signature,
#  }
#
#  impl Default for SimpleUDWF {
#      fn default() -> Self {
#          Self {
#             signature: Signature::any(0, Volatility::Immutable),
#          }
#      }
#  }
#
#  impl WindowUDFImpl for SimpleUDWF {
#      fn as_any(&self) -> &dyn Any {
#          self
#      }
#      fn name(&self) -> &str {
#          "simple_user_defined_window_function"
#      }
#      fn signature(&self) -> &Signature {
#          &self.signature
#      }
#      fn partition_evaluator(
#          &self,
#          partition_evaluator_args: PartitionEvaluatorArgs,
#      ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#          unimplemented!()
#      }
#      fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<Field> {
#          Ok(Field::new(field_args.name(), DataType::Int64, false))
#      }
#  }
#
```

2. Uses a custom constructor for UDWF.

```
# use std::any::Any;
# use datafusion_common::arrow::datatypes::{DataType, Field};
# use datafusion_expr::{PartitionEvaluator, Signature, Volatility, WindowUDFImpl};
# use datafusion_functions_window::{create_udwf_expr, define_udwf_and_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// 1. Defines the `row_number_udwf()` user-defined window function.
///
/// 2. Defines the expression API:
///     ```
///     pub fn row_number() -> datafusion_expr::Expr {
///         row_number_udwf().call(vec![])
///     }
///     ```
define_udwf_and_expr!(
    RowNumber,
    row_number,
    "Returns a unique row number for each row in window partition beginning at 1.",
    RowNumber::new // <-- custom constructor
);
#
# assert_eq!(
#     row_number().name_for_alias().unwrap(),
#     "row_number() ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
# );
#
# #[derive(Debug)]
# struct RowNumber {
#     signature: Signature,
# }
# impl RowNumber {
#     fn new() -> Self {
#         Self {
#             signature: Signature::any(0, Volatility::Immutable),
#         }
#     }
# }
# impl WindowUDFImpl for RowNumber {
#     fn as_any(&self) -> &dyn Any {
#         self
#     }
#     fn name(&self) -> &str {
#         "row_number"
#     }
#     fn signature(&self) -> &Signature {
#         &self.signature
#     }
#     fn partition_evaluator(
#         &self,
#         _partition_evaluator_args: PartitionEvaluatorArgs,
#     ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#         unimplemented!()
#     }
#     fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<Field> {
#         Ok(Field::new(field_args.name(), DataType::UInt64, false))
#     }
# }
```

## Expression API With Multiple Parameters
3. Uses default constructor for UDWF

```
# use std::any::Any;
#
# use datafusion_expr::{
#     PartitionEvaluator, Signature, TypeSignature, Volatility, WindowUDFImpl,
# };
#
# use datafusion_functions_window::{create_udwf_expr, define_udwf_and_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
#
# use datafusion_common::arrow::datatypes::Field;
# use datafusion_common::ScalarValue;
# use datafusion_expr::{col, lit};
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// 1. Defines the `lead_udwf()` user-defined window function.
///
/// 2. Defines the expression API:
///     ```
///     pub fn lead(
///         expr: datafusion_expr::Expr,
///         offset: datafusion_expr::Expr,
///         default: datafusion_expr::Expr,
///     ) -> datafusion_expr::Expr {
///         lead_udwf().call(vec![expr, offset, default])
///     }
///     ```
define_udwf_and_expr!(
    Lead,
    lead,
    [expr, offset, default],        // <- 3 parameters
    "user-defined window function"
);
#
# assert_eq!(
#     lead(col("a"), lit(1i64), lit(ScalarValue::Null))
#         .name_for_alias()
#         .unwrap(),
#     "lead(a,Int64(1),NULL) ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
# );
#
# #[derive(Debug)]
# struct Lead {
#     signature: Signature,
# }
#
# impl Default for Lead {
#     fn default() -> Self {
#         Self {
#             signature: Signature::one_of(
#                 vec![
#                     TypeSignature::Any(1),
#                     TypeSignature::Any(2),
#                     TypeSignature::Any(3),
#                 ],
#                 Volatility::Immutable,
#             ),
#         }
#     }
# }
#
# impl WindowUDFImpl for Lead {
#     fn as_any(&self) -> &dyn Any {
#         self
#     }
#     fn name(&self) -> &str {
#         "lead"
#     }
#     fn signature(&self) -> &Signature {
#         &self.signature
#     }
#     fn partition_evaluator(
#         &self,
#         _partition_evaluator_args: PartitionEvaluatorArgs,
#     ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#         unimplemented!()
#     }
#     fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<Field> {
#         Ok(Field::new(
#             field_args.name(),
#             field_args.get_input_type(0).unwrap(),
#             false,
#         ))
#     }
# }
```
4. Uses custom constructor for UDWF

```
# use std::any::Any;
#
# use datafusion_expr::{
#     PartitionEvaluator, Signature, TypeSignature, Volatility, WindowUDFImpl,
# };
#
# use datafusion_functions_window::{create_udwf_expr, define_udwf_and_expr, get_or_init_udwf};
# use datafusion_functions_window_common::field::WindowUDFFieldArgs;
#
# use datafusion_common::arrow::datatypes::Field;
# use datafusion_common::ScalarValue;
# use datafusion_expr::{col, lit};
# use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
#
/// 1. Defines the `lead_udwf()` user-defined window function.
///
/// 2. Defines the expression API:
///     ```
///     pub fn lead(
///         expr: datafusion_expr::Expr,
///         offset: datafusion_expr::Expr,
///         default: datafusion_expr::Expr,
///     ) -> datafusion_expr::Expr {
///         lead_udwf().call(vec![expr, offset, default])
///     }
///     ```
define_udwf_and_expr!(
    Lead,
    lead,
    [expr, offset, default],        // <- 3 parameters
    "user-defined window function",
    Lead::new                       // <- Custom constructor
);
#
# assert_eq!(
#     lead(col("a"), lit(1i64), lit(ScalarValue::Null))
#         .name_for_alias()
#         .unwrap(),
#     "lead(a,Int64(1),NULL) ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
# );
#
# #[derive(Debug)]
# struct Lead {
#     signature: Signature,
# }
#
# impl Lead {
#     fn new() -> Self {
#         Self {
#             signature: Signature::one_of(
#                 vec![
#                     TypeSignature::Any(1),
#                     TypeSignature::Any(2),
#                     TypeSignature::Any(3),
#                 ],
#                 Volatility::Immutable,
#             ),
#         }
#     }
# }
#
# impl WindowUDFImpl for Lead {
#     fn as_any(&self) -> &dyn Any {
#         self
#     }
#     fn name(&self) -> &str {
#         "lead"
#     }
#     fn signature(&self) -> &Signature {
#         &self.signature
#     }
#     fn partition_evaluator(
#         &self,
#         _partition_evaluator_args: PartitionEvaluatorArgs,
#     ) -> datafusion_common::Result<Box<dyn PartitionEvaluator>> {
#         unimplemented!()
#     }
#     fn field(&self, field_args: WindowUDFFieldArgs) -> datafusion_common::Result<Field> {
#         Ok(Field::new(
#             field_args.name(),
#             field_args.get_input_type(0).unwrap(),
#             false,
#         ))
#     }
# }
```

```rust
pub macro_rules! define_udwf_and_expr {
    /* macro_rules! define_udwf_and_expr {
    ($UDWF:ident, $OUT_FN_NAME:ident, $DOC:expr) => { ... };
    ($UDWF:ident, $OUT_FN_NAME:ident, $DOC:expr, $CTOR:path) => { ... };
    ($UDWF:ident, $OUT_FN_NAME:ident, [$($PARAM:ident),+], $DOC:expr) => { ... };
    ($UDWF:ident, $OUT_FN_NAME:ident, [$($PARAM:ident),+], $DOC:expr, $CTOR:path) => { ... };
} */
}
```


# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_functions_aggregate`

 Aggregate Function packages for [DataFusion].

 This crate contains a collection of various aggregate function packages for DataFusion,
 implemented using the extension API. Users may wish to control which functions
 are available to control the binary size of their application as well as
 use dialect specific implementations of functions (e.g. Spark vs Postgres)

 Each package is implemented as a separate
 module, activated by a feature flag.

 [DataFusion]: https://crates.io/crates/datafusion

 # Available Packages
 See the list of [modules](#modules) in this crate for available packages.

 # Using A Package
 You can register all functions in all packages using the [`register_all`] function.

 Each package also exports an `expr_fn` submodule to help create [`Expr`]s that invoke
 functions using a fluent style. For example:

[`Expr`]: datafusion_expr::Expr

 # Implementing A New Package

 To add a new package to this crate, you should follow the model of existing
 packages. The high level steps are:

 1. Create a new module with the appropriate [AggregateUDF] implementations.

 2. Use the macros in [`macros`] to create standard entry points.

 3. Add a new feature to `Cargo.toml`, with any optional dependencies

 4. Use the `make_package!` macro to expose the module when the
    feature is enabled.

## Modules

## Module `macros`

**Attributes:**

- `#[macro_use]`

```rust
pub mod macros { /* ... */ }
```

## Module `approx_distinct`

Defines physical expressions that can evaluated at runtime during query execution

```rust
pub mod approx_distinct { /* ... */ }
```

### Types

#### Struct `ApproxDistinct`

```rust
pub struct ApproxDistinct {
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **AggregateUDFImpl**
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
    fn return_type(self: &Self, _: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `approx_distinct`

approximate number of distinct input values

```rust
pub fn approx_distinct(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `approx_distinct_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxDistinct`]

```rust
pub fn approx_distinct_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `approx_median`

Defines physical expressions for APPROX_MEDIAN that can be evaluated MEDIAN at runtime during query execution

```rust
pub mod approx_median { /* ... */ }
```

### Types

#### Struct `ApproxMedian`

APPROX_MEDIAN aggregate expression

```rust
pub struct ApproxMedian {
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
  Create a new APPROX_MEDIAN aggregate function

###### Trait Implementations

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Freeze**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```
    Return a reference to Any that can be used for downcasting

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

### Functions

#### Function `approx_median`

Computes the approximate median of a set of numbers

```rust
pub fn approx_median(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `approx_median_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxMedian`]

```rust
pub fn approx_median_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `approx_percentile_cont`

```rust
pub mod approx_percentile_cont { /* ... */ }
```

### Types

#### Struct `ApproxPercentileCont`

```rust
pub struct ApproxPercentileCont {
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
  Create a new [`ApproxPercentileCont`] aggregate function.

###### Trait Implementations

- **MaybeSendSync**
- **IntoEither**
- **Unpin**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```
    See [`TDigest::to_scalar_state()`] for a description of the serialized

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
#### Struct `ApproxPercentileAccumulator`

```rust
pub struct ApproxPercentileAccumulator {
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
  pub fn new(percentile: f64, return_type: DataType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn new_with_max_size(percentile: f64, return_type: DataType, max_size: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn merge_digests(self: &mut Self, digests: &[TDigest]) { /* ... */ }
  ```

- ```rust
  pub fn convert_to_float(values: &ArrayRef) -> Result<Vec<f64>> { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **Sync**
- **IntoEither**
- **RefUnwindSafe**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Functions

#### Function `approx_percentile_cont_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxPercentileCont`]

```rust
pub fn approx_percentile_cont_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `approx_percentile_cont`

Computes the approximate percentile continuous of a set of numbers

```rust
pub fn approx_percentile_cont(expression: datafusion_expr::Expr, percentile: datafusion_expr::Expr, centroids: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

## Module `approx_percentile_cont_with_weight`

```rust
pub mod approx_percentile_cont_with_weight { /* ... */ }
```

### Types

#### Struct `ApproxPercentileContWithWeight`

APPROX_PERCENTILE_CONT_WITH_WEIGHT aggregate expression

```rust
pub struct ApproxPercentileContWithWeight {
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
  Create a new [`ApproxPercentileContWithWeight`] aggregate function.

###### Trait Implementations

- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **AggregateUDFImpl**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```
    See [`TDigest::to_scalar_state()`] for a description of the serialized

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `ApproxPercentileWithWeightAccumulator`

```rust
pub struct ApproxPercentileWithWeightAccumulator {
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
  pub fn new(approx_percentile_cont_accumulator: ApproxPercentileAccumulator) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **IntoEither**
- **Allocation**
- **Unpin**
- **Send**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Functions

#### Function `approx_percentile_cont_with_weight`

Computes the approximate percentile continuous with weight of a set of numbers

```rust
pub fn approx_percentile_cont_with_weight(expression: datafusion_expr::Expr, weight: datafusion_expr::Expr, percentile: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `approx_percentile_cont_with_weight_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ApproxPercentileContWithWeight`]

```rust
pub fn approx_percentile_cont_with_weight_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `array_agg`

`ARRAY_AGG` aggregate implementation: [`ArrayAgg`]

```rust
pub mod array_agg { /* ... */ }
```

### Types

#### Struct `ArrayAgg`

ARRAY_AGG aggregate expression

```rust
pub struct ArrayAgg {
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

- **IntoEither**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> datafusion_expr::ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `ArrayAggAccumulator`

```rust
pub struct ArrayAggAccumulator {
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
  pub fn try_new(datatype: &DataType) -> Result<Self> { /* ... */ }
  ```
  new array_agg accumulator based on given item data type

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Accumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

### Functions

#### Function `array_agg`

input values, including nulls, concatenated into an array

```rust
pub fn array_agg(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_agg_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`ArrayAgg`]

```rust
pub fn array_agg_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `average`

Defines `Avg` & `Mean` aggregate & accumulators

```rust
pub mod average { /* ... */ }
```

### Types

#### Struct `Avg`

```rust
pub struct Avg {
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

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
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

- **MaybeSendSync**
- **UnwindSafe**
- **AggregateUDFImpl**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

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

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `AvgAccumulator`

An accumulator to compute the average

```rust
pub struct AvgAccumulator {
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

- **Accumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn retract_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn supports_retract_batch(self: &Self) -> bool { /* ... */ }
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
    fn default() -> AvgAccumulator { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Allocation**
- **Unpin**
- **MaybeSendSync**
- **ErasedDestructor**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Functions

#### Function `avg`

Returns the avg of a group of values.

```rust
pub fn avg(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `avg_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Avg`]

```rust
pub fn avg_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `bit_and_or_xor`

Defines `BitAnd`, `BitOr`, `BitXor` and `BitXor DISTINCT` aggregate accumulators

```rust
pub mod bit_and_or_xor { /* ... */ }
```

### Functions

#### Function `bit_and`

Returns the bitwiseBitwiseOperationType::Andof a group of values

```rust
pub fn bit_and(expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `bit_and_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`bit_and`]

```rust
pub fn bit_and_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `bit_or`

Returns the bitwiseBitwiseOperationType::Orof a group of values

```rust
pub fn bit_or(expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `bit_or_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`bit_or`]

```rust
pub fn bit_or_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `bit_xor`

Returns the bitwiseBitwiseOperationType::Xorof a group of values

```rust
pub fn bit_xor(expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `bit_xor_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`bit_xor`]

```rust
pub fn bit_xor_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `bool_and_or`

Defines physical expressions that can evaluated at runtime during query execution

```rust
pub mod bool_and_or { /* ... */ }
```

### Types

#### Struct `BoolAnd`

BOOL_AND aggregate expression

```rust
pub struct BoolAnd {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **AggregateUDFImpl**
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
    fn return_type(self: &Self, _: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, _args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn order_sensitivity(self: &Self) -> AggregateOrderSensitivity { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **ErasedDestructor**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
#### Struct `BoolOr`

BOOL_OR aggregate expression

```rust
pub struct BoolOr {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **AggregateUDFImpl**
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
    fn return_type(self: &Self, _: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, _args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn order_sensitivity(self: &Self) -> AggregateOrderSensitivity { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **RefUnwindSafe**
- **MaybeSendSync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BoolOr { /* ... */ }
    ```

### Functions

#### Function `bool_and`

The values to combine with `AND`

```rust
pub fn bool_and(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `bool_and_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`BoolAnd`]

```rust
pub fn bool_and_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `bool_or`

The values to combine with `OR`

```rust
pub fn bool_or(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `bool_or_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`BoolOr`]

```rust
pub fn bool_or_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `correlation`

[`Correlation`]: correlation sample aggregations.

```rust
pub mod correlation { /* ... */ }
```

### Types

#### Struct `Correlation`

```rust
pub struct Correlation {
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
  Create a new COVAR_POP aggregate function

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **Unpin**
- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **AggregateUDFImpl**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, _args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
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

- **RefUnwindSafe**
#### Struct `CorrelationAccumulator`

An accumulator to compute correlation

```rust
pub struct CorrelationAccumulator {
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
  pub fn try_new() -> Result<Self> { /* ... */ }
  ```
  Creates a new `CorrelationAccumulator`

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **MaybeSendSync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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
- **Send**
- **Accumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn retract_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

#### Struct `CorrelationGroupsAccumulator`

```rust
pub struct CorrelationGroupsAccumulator {
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

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **GroupsAccumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self, emit_to: EmitTo) -> Result<ArrayRef> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self, emit_to: EmitTo) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> CorrelationGroupsAccumulator { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `corr`

Correlation between two numeric values.

```rust
pub fn corr(y: datafusion_expr::Expr, x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `corr_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Correlation`]

```rust
pub fn corr_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `count`

```rust
pub mod count { /* ... */ }
```

### Types

#### Struct `Count`

```rust
pub struct Count {
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
- **Unpin**
- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn is_nullable(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn default_value(self: &Self, _data_type: &DataType) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn value_from_stats(self: &Self, statistics_args: &StatisticsArgs<''_>) -> Option<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn set_monotonicity(self: &Self, _data_type: &DataType) -> SetMonotonicity { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

### Functions

#### Function `count`

Count the number of non-null values in the column

```rust
pub fn count(expr: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `count_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Count`]

```rust
pub fn count_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `count_distinct`

```rust
pub fn count_distinct(expr: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `count_all`

Creates aggregation to count all rows.

In SQL this is `SELECT COUNT(*) ... `

The expression is equivalent to `COUNT(*)`, `COUNT()`, `COUNT(1)`, and is
aliased to a column named `"count(*)"` for backward compatibility.

Example
```
# use datafusion_functions_aggregate::count::count_all;
# use datafusion_expr::col;
// create `count(*)` expression
let expr = count_all();
assert_eq!(expr.schema_name().to_string(), "count(*)");
// if you need to refer to this column, use the `schema_name` function
let expr = col(expr.schema_name().to_string());
```

```rust
pub fn count_all() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `count_all_window`

Creates window aggregation to count all rows.

In SQL this is `SELECT COUNT(*) OVER (..) ... `

The expression is equivalent to `COUNT(*)`, `COUNT()`, `COUNT(1)`

Example
```
# use datafusion_functions_aggregate::count::count_all_window;
# use datafusion_expr::col;
// create `count(*)` OVER ... window function expression
let expr = count_all_window();
assert_eq!(
  expr.schema_name().to_string(),
  "count(Int64(1)) ROWS BETWEEN UNBOUNDED PRECEDING AND UNBOUNDED FOLLOWING"
);
// if you need to refer to this column, use the `schema_name` function
let expr = col(expr.schema_name().to_string());
```

```rust
pub fn count_all_window() -> datafusion_expr::Expr { /* ... */ }
```

## Module `covariance`

[`CovarianceSample`]: covariance sample aggregations.

```rust
pub mod covariance { /* ... */ }
```

### Types

#### Struct `CovarianceSample`

```rust
pub struct CovarianceSample {
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
- **Sync**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **IntoEither**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `CovariancePopulation`

```rust
pub struct CovariancePopulation {
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **Unpin**
- **UnwindSafe**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

#### Struct `CovarianceAccumulator`

An accumulator to compute covariance
The algorithm used is an online implementation and numerically stable. It is derived from the following paper
for calculating variance:
Welford, B. P. (1962). "Note on a method for calculating corrected sums of squares and products".
Technometrics. 4 (3): 419–420. doi:10.2307/1266577. JSTOR 1266577.

The algorithm has been analyzed here:
Ling, Robert F. (1974). "Comparison of Several Algorithms for Computing Sample Means and Variances".
Journal of the American Statistical Association. 69 (348): 859–866. doi:10.2307/2286154. JSTOR 2286154.

Though it is not covered in the original paper but is based on the same idea, as a result the algorithm is online,
parallelize and numerically stable.

```rust
pub struct CovarianceAccumulator {
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
  pub fn try_new(s_type: StatsType) -> Result<Self> { /* ... */ }
  ```
  Creates a new `CovarianceAccumulator`

- ```rust
  pub fn get_count(self: &Self) -> u64 { /* ... */ }
  ```

- ```rust
  pub fn get_mean1(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub fn get_mean2(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub fn get_algo_const(self: &Self) -> f64 { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **IntoEither**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn retract_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `covar_samp`

Computes the sample covariance.

```rust
pub fn covar_samp(y: datafusion_expr::Expr, x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `covar_samp_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`CovarianceSample`]

```rust
pub fn covar_samp_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `covar_pop`

Computes the population covariance.

```rust
pub fn covar_pop(y: datafusion_expr::Expr, x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `covar_pop_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`CovariancePopulation`]

```rust
pub fn covar_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `first_last`

Defines the FIRST_VALUE/LAST_VALUE aggregations.

```rust
pub mod first_last { /* ... */ }
```

### Types

#### Struct `FirstValue`

```rust
pub struct FirstValue {
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

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **IntoEither**
- **AggregateUDFImpl**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn with_beneficial_ordering(self: Arc<Self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>> { /* ... */ }
    ```

  - ```rust
    fn order_sensitivity(self: &Self) -> AggregateOrderSensitivity { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> datafusion_expr::ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
#### Struct `FirstValueAccumulator`

```rust
pub struct FirstValueAccumulator {
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
  pub fn try_new(data_type: &DataType, ordering_dtypes: &[DataType], ordering_req: LexOrdering, ignore_nulls: bool) -> Result<Self> { /* ... */ }
  ```
  Creates a new `FirstValueAccumulator` for the given `data_type`.

- ```rust
  pub fn with_requirement_satisfied(self: Self, requirement_satisfied: bool) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

#### Struct `LastValue`

```rust
pub struct LastValue {
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

- **Freeze**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **IntoEither**
- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **AggregateUDFImpl**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn with_beneficial_ordering(self: Arc<Self>, beneficial_ordering: bool) -> Result<Option<Arc<dyn AggregateUDFImpl>>> { /* ... */ }
    ```

  - ```rust
    fn order_sensitivity(self: &Self) -> AggregateOrderSensitivity { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> datafusion_expr::ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Functions

#### Function `first_value_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`FirstValue`]

```rust
pub fn first_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `last_value_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`LastValue`]

```rust
pub fn last_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `first_value`

Returns the first value in a group of values.

```rust
pub fn first_value(expression: datafusion_expr::Expr, order_by: Option<Vec<datafusion_expr::SortExpr>>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `last_value`

Returns the last value in a group of values.

```rust
pub fn last_value(expression: datafusion_expr::Expr, order_by: Option<Vec<datafusion_expr::SortExpr>>) -> datafusion_expr::Expr { /* ... */ }
```

## Module `grouping`

Defines physical expressions that can evaluated at runtime during query execution

```rust
pub mod grouping { /* ... */ }
```

### Types

#### Struct `Grouping`

```rust
pub struct Grouping {
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
  Create a new GROUPING aggregate function.

###### Trait Implementations

- **RefUnwindSafe**
- **AggregateUDFImpl**
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **Sync**
- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `grouping`

Returns 1 if the data is aggregated across the specified column or 0 for not aggregated in the result set.

```rust
pub fn grouping(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `grouping_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Grouping`]

```rust
pub fn grouping_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `hyperloglog`

# HyperLogLog

`hyperloglog` is a module that contains a modified version
of [redis's implementation](https://github.com/redis/redis/blob/4930d19e70c391750479951022e207e19111eb55/src/hyperloglog.c)
with some modification based on strong assumption of usage
within datafusion, so that function can
be efficiently implemented.

Specifically, like Redis's version, this HLL structure uses
2**14 = 16384 registers, which means the standard error is
1.04/(16384**0.5) = 0.8125%. Unlike Redis, the register takes
up full [`u8`] size instead of a raw int* and thus saves some
tricky bit shifting techniques used in the original version.
This results in a memory usage increase from 12Kib to 16Kib.
Also only the dense version is adopted, so there's no automatic
conversion, largely to simplify the code.

This module also borrows some code structure from [pdatastructs.rs](https://github.com/crepererum/pdatastructs.rs/blob/3997ed50f6b6871c9e53c4c5e0f48f431405fc63/src/hyperloglog.rs).

```rust
pub mod hyperloglog { /* ... */ }
```

## Module `median`

```rust
pub mod median { /* ... */ }
```

### Types

#### Struct `Median`

MEDIAN aggregate expression. If using the non-distinct variation, then this uses a
lot of memory because all values need to be stored in memory before a result can be
computed. If an approximation is sufficient then APPROX_MEDIAN provides a much more
efficient solution.

If using the distinct variation, the memory usage will be similarly high if the
cardinality is high as it stores all distinct values in memory before computing the
result, but if cardinality is low then memory usage will also be lower.

```rust
pub struct Median {
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

- **Freeze**
- **RefUnwindSafe**
- **Send**
- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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
- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `median`

Computes the median of a set of numbers

```rust
pub fn median(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `median_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Median`]

```rust
pub fn median_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `min_max`

[`Max`] and [`MaxAccumulator`] accumulator for the `max` function
[`Min`] and [`MinAccumulator`] accumulator for the `min` function

```rust
pub mod min_max { /* ... */ }
```

### Types

#### Struct `Max`

```rust
pub struct Max {
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **IntoEither**
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

- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn create_sliding_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn is_descending(self: &Self) -> Option<bool> { /* ... */ }
    ```

  - ```rust
    fn order_sensitivity(self: &Self) -> datafusion_expr::utils::AggregateOrderSensitivity { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> datafusion_expr::ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn value_from_stats(self: &Self, statistics_args: &StatisticsArgs<''_>) -> Option<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn set_monotonicity(self: &Self, _data_type: &DataType) -> SetMonotonicity { /* ... */ }
    ```

#### Struct `MaxAccumulator`

An accumulator to compute the maximum value

```rust
pub struct MaxAccumulator {
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
  pub fn try_new(datatype: &DataType) -> Result<Self> { /* ... */ }
  ```
  new max accumulator

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Accumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
- **Freeze**
- **ErasedDestructor**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `SlidingMaxAccumulator`

```rust
pub struct SlidingMaxAccumulator {
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
  pub fn try_new(datatype: &DataType) -> Result<Self> { /* ... */ }
  ```
  new max accumulator

###### Trait Implementations

- **IntoEither**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Accumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn retract_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn supports_retract_batch(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
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

- **RefUnwindSafe**
#### Struct `Min`

```rust
pub struct Min {
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **Unpin**
- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn create_sliding_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn is_descending(self: &Self) -> Option<bool> { /* ... */ }
    ```

  - ```rust
    fn value_from_stats(self: &Self, statistics_args: &StatisticsArgs<''_>) -> Option<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn order_sensitivity(self: &Self) -> datafusion_expr::utils::AggregateOrderSensitivity { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> datafusion_expr::ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn set_monotonicity(self: &Self, _data_type: &DataType) -> SetMonotonicity { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `MinAccumulator`

An accumulator to compute the minimum value

```rust
pub struct MinAccumulator {
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
  pub fn try_new(datatype: &DataType) -> Result<Self> { /* ... */ }
  ```
  new min accumulator

###### Trait Implementations

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **RefUnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `SlidingMinAccumulator`

```rust
pub struct SlidingMinAccumulator {
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
  pub fn try_new(datatype: &DataType) -> Result<Self> { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn retract_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn supports_retract_batch(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Unpin**
#### Struct `MovingMin`

Keep track of the minimum value in a sliding window.

The implementation is taken from <https://github.com/spebern/moving_min_max/blob/master/src/lib.rs>

`moving min max` provides one data structure for keeping track of the
minimum value and one for keeping track of the maximum value in a sliding
window.

Each element is stored with the current min/max. One stack to push and another one for pop. If pop stack is empty,
push to this stack all elements popped from first stack while updating their current min/max. Now pop from
the second stack (MovingMin/Max struct works as a queue). To find the minimum element of the queue,
look at the smallest/largest two elements of the individual stacks, then take the minimum of those two values.

The complexity of the operations are
- O(1) for getting the minimum/maximum
- O(1) for push
- amortized O(1) for pop

```
# use datafusion_functions_aggregate::min_max::MovingMin;
let mut moving_min = MovingMin::<i32>::new();
moving_min.push(2);
moving_min.push(1);
moving_min.push(3);

assert_eq!(moving_min.min(), Some(&1));
assert_eq!(moving_min.pop(), Some(2));

assert_eq!(moving_min.min(), Some(&1));
assert_eq!(moving_min.pop(), Some(1));

assert_eq!(moving_min.min(), Some(&3));
assert_eq!(moving_min.pop(), Some(3));

assert_eq!(moving_min.min(), None);
assert_eq!(moving_min.pop(), None);
```

```rust
pub struct MovingMin<T> {
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
  Creates a new `MovingMin` to keep track of the minimum in a sliding

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```
  Creates a new `MovingMin` to keep track of the minimum in a sliding

- ```rust
  pub fn min(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Returns the minimum of the sliding window or `None` if the window is

- ```rust
  pub fn push(self: &mut Self, val: T) { /* ... */ }
  ```
  Pushes a new element into the sliding window.

- ```rust
  pub fn pop(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes and returns the last value of the sliding window.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements stored in the sliding window.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the moving window contains no elements.

###### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **RefUnwindSafe**
- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Struct `MovingMax`

Keep track of the maximum value in a sliding window.

See [`MovingMin`] for more details.

```
# use datafusion_functions_aggregate::min_max::MovingMax;
let mut moving_max = MovingMax::<i32>::new();
moving_max.push(2);
moving_max.push(3);
moving_max.push(1);

assert_eq!(moving_max.max(), Some(&3));
assert_eq!(moving_max.pop(), Some(2));

assert_eq!(moving_max.max(), Some(&3));
assert_eq!(moving_max.pop(), Some(3));

assert_eq!(moving_max.max(), Some(&1));
assert_eq!(moving_max.pop(), Some(1));

assert_eq!(moving_max.max(), None);
assert_eq!(moving_max.pop(), None);
```

```rust
pub struct MovingMax<T> {
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
  Creates a new `MovingMax` to keep track of the maximum in a sliding window.

- ```rust
  pub fn with_capacity(capacity: usize) -> Self { /* ... */ }
  ```
  Creates a new `MovingMax` to keep track of the maximum in a sliding window with

- ```rust
  pub fn max(self: &Self) -> Option<&T> { /* ... */ }
  ```
  Returns the maximum of the sliding window or `None` if the window is empty.

- ```rust
  pub fn push(self: &mut Self, val: T) { /* ... */ }
  ```
  Pushes a new element into the sliding window.

- ```rust
  pub fn pop(self: &mut Self) -> Option<T> { /* ... */ }
  ```
  Removes and returns the last value of the sliding window.

- ```rust
  pub fn len(self: &Self) -> usize { /* ... */ }
  ```
  Returns the number of elements stored in the sliding window.

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if the moving window contains no elements.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Allocation**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **IntoEither**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
### Functions

#### Function `max_batch`

dynamically-typed max(array) -> ScalarValue

```rust
pub fn max_batch(values: &arrow::array::ArrayRef) -> datafusion_common::Result<datafusion_common::ScalarValue> { /* ... */ }
```

#### Function `max`

Returns the maximum of a group of values.

```rust
pub fn max(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `max_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Max`]

```rust
pub fn max_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `min`

Returns the minimum of a group of values.

```rust
pub fn min(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `min_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Min`]

```rust
pub fn min_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `nth_value`

Defines NTH_VALUE aggregate expression which may specify ordering requirement
that can evaluated at runtime during query execution

```rust
pub mod nth_value { /* ... */ }
```

### Types

#### Struct `NthValueAgg`

Expression for a `NTH_VALUE(... ORDER BY ..., ...)` aggregation. In a multi
partition setting, partial aggregations are computed for every partition,
and then their results are merged.

```rust
pub struct NthValueAgg {
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
  Create a new `NthValueAgg` aggregate function

###### Trait Implementations

- **RefUnwindSafe**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Unpin**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **AggregateUDFImpl**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Sync**
#### Struct `NthValueAccumulator`

```rust
pub struct NthValueAccumulator {
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
  pub fn try_new(n: i64, datatype: &DataType, ordering_dtypes: &[DataType], ordering_req: LexOrdering) -> Result<Self> { /* ... */ }
  ```
  Create a new order-sensitive NTH_VALUE accumulator based on the given

###### Trait Implementations

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **ErasedDestructor**
- **Accumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```
    Updates its state with the `values`. Assumes data in the `values` satisfies the required

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
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

- **Unpin**
- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

### Functions

#### Function `nth_value_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`NthValueAgg`]

```rust
pub fn nth_value_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `nth_value`

Returns the nth value in a group of values.

```rust
pub fn nth_value(expr: datafusion_expr::Expr, n: i64, order_by: Vec<datafusion_expr::SortExpr>) -> datafusion_expr::Expr { /* ... */ }
```

## Module `regr`

Defines physical expressions that can evaluated at runtime during query execution

```rust
pub mod regr { /* ... */ }
```

### Types

#### Struct `Regr`

```rust
pub struct Regr {
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
  pub fn new(regr_type: RegrType, func_name: &''static str) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Sync**
- **AggregateUDFImpl**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Send**
#### Enum `RegrType`

**Attributes:**

- `#[allow(clippy::upper_case_acronyms)]`

```rust
pub enum RegrType {
    Slope,
    Intercept,
    Count,
    R2,
    AvgX,
    AvgY,
    SXX,
    SYY,
    SXY,
}
```

##### Variants

###### `Slope`

Variant for `regr_slope` aggregate expression
Returns the slope of the linear regression line for non-null pairs in aggregate columns.
Given input column Y and X: `regr_slope(Y, X)` returns the slope (k in Y = k*X + b) using minimal
RSS (Residual Sum of Squares) fitting.

###### `Intercept`

Variant for `regr_intercept` aggregate expression
Returns the intercept of the linear regression line for non-null pairs in aggregate columns.
Given input column Y and X: `regr_intercept(Y, X)` returns the intercept (b in Y = k*X + b) using minimal
RSS fitting.

###### `Count`

Variant for `regr_count` aggregate expression
Returns the number of input rows for which both expressions are not null.
Given input column Y and X: `regr_count(Y, X)` returns the count of non-null pairs.

###### `R2`

Variant for `regr_r2` aggregate expression
Returns the coefficient of determination (R-squared value) of the linear regression line for non-null pairs in aggregate columns.
The R-squared value represents the proportion of variance in Y that is predictable from X.

###### `AvgX`

Variant for `regr_avgx` aggregate expression
Returns the average of the independent variable for non-null pairs in aggregate columns.
Given input column X: `regr_avgx(Y, X)` returns the average of X values.

###### `AvgY`

Variant for `regr_avgy` aggregate expression
Returns the average of the dependent variable for non-null pairs in aggregate columns.
Given input column Y: `regr_avgy(Y, X)` returns the average of Y values.

###### `SXX`

Variant for `regr_sxx` aggregate expression
Returns the sum of squares of the independent variable for non-null pairs in aggregate columns.
Given input column X: `regr_sxx(Y, X)` returns the sum of squares of deviations of X from its mean.

###### `SYY`

Variant for `regr_syy` aggregate expression
Returns the sum of squares of the dependent variable for non-null pairs in aggregate columns.
Given input column Y: `regr_syy(Y, X)` returns the sum of squares of deviations of Y from its mean.

###### `SXY`

Variant for `regr_sxy` aggregate expression
Returns the sum of products of pairs of numbers for non-null pairs in aggregate columns.
Given input column Y and X: `regr_sxy(Y, X)` returns the sum of products of the deviations of Y and X from their respective means.

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &RegrType) -> bool { /* ... */ }
    ```

- **Send**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RegrType { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
#### Struct `RegrAccumulator`

`RegrAccumulator` is used to compute linear regression aggregate functions
by maintaining statistics needed to compute them in an online fashion.

This struct uses Welford's online algorithm for calculating variance and covariance:
<https://en.wikipedia.org/wiki/Algorithms_for_calculating_variance#Welford's_online_algorithm>

Given the statistics, the following aggregate functions can be calculated:

- `regr_slope(y, x)`: Slope of the linear regression line, calculated as:
  cov_pop(x, y) / var_pop(x).
  It represents the expected change in Y for a one-unit change in X.

- `regr_intercept(y, x)`: Intercept of the linear regression line, calculated as:
  mean_y - (regr_slope(y, x) * mean_x).
  It represents the expected value of Y when X is 0.

- `regr_count(y, x)`: Count of the non-null(both x and y) input rows.

- `regr_r2(y, x)`: R-squared value (coefficient of determination), calculated as:
  (cov_pop(x, y) ^ 2) / (var_pop(x) * var_pop(y)).
  It provides a measure of how well the model's predictions match the observed data.

- `regr_avgx(y, x)`: Average of the independent variable X, calculated as: mean_x.

- `regr_avgy(y, x)`: Average of the dependent variable Y, calculated as: mean_y.

- `regr_sxx(y, x)`: Sum of squares of the independent variable X, calculated as:
  m2_x.

- `regr_syy(y, x)`: Sum of squares of the dependent variable Y, calculated as:
  m2_y.

- `regr_sxy(y, x)`: Sum of products of paired values, calculated as:
  algo_const.

Here's how the statistics maintained in this struct are calculated:
- `cov_pop(x, y)`: algo_const / count.
- `var_pop(x)`: m2_x / count.
- `var_pop(y)`: m2_y / count.

```rust
pub struct RegrAccumulator {
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
  pub fn try_new(regr_type: &RegrType) -> Result<Self> { /* ... */ }
  ```
  Creates a new `RegrAccumulator`

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn supports_retract_batch(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn retract_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

### Functions

#### Function `regr_slope`

Compute a linear regression of type [RegrType::Slope]

```rust
pub fn regr_slope(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_slope_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_slope`]

```rust
pub fn regr_slope_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `regr_intercept`

Compute a linear regression of type [RegrType::Intercept]

```rust
pub fn regr_intercept(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_intercept_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_intercept`]

```rust
pub fn regr_intercept_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `regr_count`

Compute a linear regression of type [RegrType::Count]

```rust
pub fn regr_count(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_count_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_count`]

```rust
pub fn regr_count_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `regr_r2`

Compute a linear regression of type [RegrType::R2]

```rust
pub fn regr_r2(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_r2_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_r2`]

```rust
pub fn regr_r2_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `regr_avgx`

Compute a linear regression of type [RegrType::AvgX]

```rust
pub fn regr_avgx(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_avgx_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_avgx`]

```rust
pub fn regr_avgx_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `regr_avgy`

Compute a linear regression of type [RegrType::AvgY]

```rust
pub fn regr_avgy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_avgy_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_avgy`]

```rust
pub fn regr_avgy_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `regr_sxx`

Compute a linear regression of type [RegrType::SXX]

```rust
pub fn regr_sxx(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_sxx_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_sxx`]

```rust
pub fn regr_sxx_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `regr_syy`

Compute a linear regression of type [RegrType::SYY]

```rust
pub fn regr_syy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_syy_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_syy`]

```rust
pub fn regr_syy_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `regr_sxy`

Compute a linear regression of type [RegrType::SXY]

```rust
pub fn regr_sxy(expr_y: datafusion_expr::Expr, expr_x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regr_sxy_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`regr_sxy`]

```rust
pub fn regr_sxy_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `stddev`

Defines physical expressions that can evaluated at runtime during query execution

```rust
pub mod stddev { /* ... */ }
```

### Types

#### Struct `Stddev`

STDDEV and STDDEV_SAMP (standard deviation) aggregate expression

```rust
pub struct Stddev {
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
  Create a new STDDEV aggregate function

###### Trait Implementations

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **AggregateUDFImpl**
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, acc_args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
#### Struct `StddevPop`

STDDEV_POP population aggregate expression

```rust
pub struct StddevPop {
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
  Create a new STDDEV_POP aggregate function

###### Trait Implementations

- **MaybeSendSync**
- **Unpin**
- **AggregateUDFImpl**
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
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, acc_args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `StddevAccumulator`

An accumulator to compute the average

```rust
pub struct StddevAccumulator {
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
  pub fn try_new(s_type: StatsType) -> Result<Self> { /* ... */ }
  ```
  Creates a new `StddevAccumulator`

- ```rust
  pub fn get_m2(self: &Self) -> f64 { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn retract_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn supports_retract_batch(self: &Self) -> bool { /* ... */ }
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
- **Freeze**
- **RefUnwindSafe**
- **Allocation**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `StddevGroupsAccumulator`

```rust
pub struct StddevGroupsAccumulator {
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
  pub fn new(s_type: StatsType) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **GroupsAccumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&arrow::array::BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&arrow::array::BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self, emit_to: datafusion_expr::EmitTo) -> Result<ArrayRef> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self, emit_to: datafusion_expr::EmitTo) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **MaybeSendSync**
### Functions

#### Function `stddev`

Compute the standard deviation of a set of numbers

```rust
pub fn stddev(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `stddev_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Stddev`]

```rust
pub fn stddev_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `stddev_pop`

Compute the population standard deviation of a set of numbers

```rust
pub fn stddev_pop(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `stddev_pop_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`StddevPop`]

```rust
pub fn stddev_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `string_agg`

[`StringAgg`] accumulator for the `string_agg` function

```rust
pub mod string_agg { /* ... */ }
```

### Types

#### Struct `StringAgg`

STRING_AGG aggregate expression

```rust
pub struct StringAgg {
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
  Create a new StringAgg aggregate function

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

- **AggregateUDFImpl**
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
### Functions

#### Function `string_agg`

Concatenates the values of string expressions and places separator values between them

```rust
pub fn string_agg(expr: datafusion_expr::Expr, delimiter: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `string_agg_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`StringAgg`]

```rust
pub fn string_agg_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `sum`

Defines `SUM` and `SUM DISTINCT` aggregate accumulators

```rust
pub mod sum { /* ... */ }
```

### Types

#### Struct `Sum`

```rust
pub struct Sum {
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

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **AggregateUDFImpl**
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
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn create_sliding_accumulator(self: &Self, args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn order_sensitivity(self: &Self) -> AggregateOrderSensitivity { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn set_monotonicity(self: &Self, data_type: &DataType) -> SetMonotonicity { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Send**
- **Sync**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
### Functions

#### Function `sum`

Returns the sum of a group of values.

```rust
pub fn sum(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `sum_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`Sum`]

```rust
pub fn sum_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `variance`

[`VarianceSample`]: variance sample aggregations.
[`VariancePopulation`]: variance population aggregations.

```rust
pub mod variance { /* ... */ }
```

### Types

#### Struct `VarianceSample`

```rust
pub struct VarianceSample {
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **IntoEither**
- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, acc_args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Sync**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `VariancePopulation`

```rust
pub struct VariancePopulation {
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

- **Freeze**
- **AggregateUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, acc_args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **RefUnwindSafe**
#### Struct `VarianceAccumulator`

An accumulator to compute variance
The algorithm used is an online implementation and numerically stable. It is based on this paper:
Welford, B. P. (1962). "Note on a method for calculating corrected sums of squares and products".
Technometrics. 4 (3): 419–420. doi:10.2307/1266577. JSTOR 1266577.

The algorithm has been analyzed here:
Ling, Robert F. (1974). "Comparison of Several Algorithms for Computing Sample Means and Variances".
Journal of the American Statistical Association. 69 (348): 859–866. doi:10.2307/2286154. JSTOR 2286154.

```rust
pub struct VarianceAccumulator {
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
  pub fn try_new(s_type: StatsType) -> Result<Self> { /* ... */ }
  ```
  Creates a new `VarianceAccumulator`

- ```rust
  pub fn get_count(self: &Self) -> u64 { /* ... */ }
  ```

- ```rust
  pub fn get_mean(self: &Self) -> f64 { /* ... */ }
  ```

- ```rust
  pub fn get_m2(self: &Self) -> f64 { /* ... */ }
  ```

###### Trait Implementations

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **UnwindSafe**
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

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Accumulator**
  - ```rust
    fn state(self: &mut Self) -> Result<Vec<ScalarValue>> { /* ... */ }
    ```

  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn retract_batch(self: &mut Self, values: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, states: &[ArrayRef]) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self) -> Result<ScalarValue> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

  - ```rust
    fn supports_retract_batch(self: &Self) -> bool { /* ... */ }
    ```

- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `VarianceGroupsAccumulator`

```rust
pub struct VarianceGroupsAccumulator {
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
  pub fn new(s_type: StatsType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn variance(self: &mut Self, emit_to: datafusion_expr::EmitTo) -> (Vec<f64>, NullBuffer) { /* ... */ }
  ```

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

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **ErasedDestructor**
- **GroupsAccumulator**
  - ```rust
    fn update_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn merge_batch(self: &mut Self, values: &[ArrayRef], group_indices: &[usize], _opt_filter: Option<&BooleanArray>, total_num_groups: usize) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn evaluate(self: &mut Self, emit_to: datafusion_expr::EmitTo) -> Result<ArrayRef> { /* ... */ }
    ```

  - ```rust
    fn state(self: &mut Self, emit_to: datafusion_expr::EmitTo) -> Result<Vec<ArrayRef>> { /* ... */ }
    ```

  - ```rust
    fn size(self: &Self) -> usize { /* ... */ }
    ```

- **IntoEither**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `var_sample`

Computes the sample variance.

```rust
pub fn var_sample(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `var_samp_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`VarianceSample`]

```rust
pub fn var_samp_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

#### Function `var_pop`

Computes the population variance.

```rust
pub fn var_pop(expression: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `var_pop_udaf`

AggregateFunction that returns a [`AggregateUDF`](datafusion_expr::AggregateUDF) for [`VariancePopulation`]

```rust
pub fn var_pop_udaf() -> std::sync::Arc<datafusion_expr::AggregateUDF> { /* ... */ }
```

## Module `planner`

SQL planning extensions like [`AggregateFunctionPlanner`]

```rust
pub mod planner { /* ... */ }
```

### Types

#### Struct `AggregateFunctionPlanner`

```rust
pub struct AggregateFunctionPlanner;
```

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **ExprPlanner**
  - ```rust
    fn plan_aggregate(self: &Self, raw_expr: RawAggregateExpr) -> Result<PlannerResult<RawAggregateExpr>> { /* ... */ }
    ```

- **Send**
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

- **Allocation**
## Module `expr_fn`

Fluent-style API for creating `Expr`s

```rust
pub mod expr_fn { /* ... */ }
```

### Re-exports

#### Re-export `approx_distinct`

```rust
pub use super::approx_distinct::approx_distinct;
```

#### Re-export `approx_median`

```rust
pub use super::approx_median::approx_median;
```

#### Re-export `approx_percentile_cont`

```rust
pub use super::approx_percentile_cont::approx_percentile_cont;
```

#### Re-export `approx_percentile_cont_with_weight`

```rust
pub use super::approx_percentile_cont_with_weight::approx_percentile_cont_with_weight;
```

#### Re-export `array_agg`

```rust
pub use super::array_agg::array_agg;
```

#### Re-export `avg`

```rust
pub use super::average::avg;
```

#### Re-export `bit_and`

```rust
pub use super::bit_and_or_xor::bit_and;
```

#### Re-export `bit_or`

```rust
pub use super::bit_and_or_xor::bit_or;
```

#### Re-export `bit_xor`

```rust
pub use super::bit_and_or_xor::bit_xor;
```

#### Re-export `bool_and`

```rust
pub use super::bool_and_or::bool_and;
```

#### Re-export `bool_or`

```rust
pub use super::bool_and_or::bool_or;
```

#### Re-export `corr`

```rust
pub use super::correlation::corr;
```

#### Re-export `count`

```rust
pub use super::count::count;
```

#### Re-export `count_distinct`

```rust
pub use super::count::count_distinct;
```

#### Re-export `covar_pop`

```rust
pub use super::covariance::covar_pop;
```

#### Re-export `covar_samp`

```rust
pub use super::covariance::covar_samp;
```

#### Re-export `first_value`

```rust
pub use super::first_last::first_value;
```

#### Re-export `last_value`

```rust
pub use super::first_last::last_value;
```

#### Re-export `grouping`

```rust
pub use super::grouping::grouping;
```

#### Re-export `median`

```rust
pub use super::median::median;
```

#### Re-export `max`

```rust
pub use super::min_max::max;
```

#### Re-export `min`

```rust
pub use super::min_max::min;
```

#### Re-export `nth_value`

```rust
pub use super::nth_value::nth_value;
```

#### Re-export `regr_avgx`

```rust
pub use super::regr::regr_avgx;
```

#### Re-export `regr_avgy`

```rust
pub use super::regr::regr_avgy;
```

#### Re-export `regr_count`

```rust
pub use super::regr::regr_count;
```

#### Re-export `regr_intercept`

```rust
pub use super::regr::regr_intercept;
```

#### Re-export `regr_r2`

```rust
pub use super::regr::regr_r2;
```

#### Re-export `regr_slope`

```rust
pub use super::regr::regr_slope;
```

#### Re-export `regr_sxx`

```rust
pub use super::regr::regr_sxx;
```

#### Re-export `regr_sxy`

```rust
pub use super::regr::regr_sxy;
```

#### Re-export `regr_syy`

```rust
pub use super::regr::regr_syy;
```

#### Re-export `stddev`

```rust
pub use super::stddev::stddev;
```

#### Re-export `stddev_pop`

```rust
pub use super::stddev::stddev_pop;
```

#### Re-export `sum`

```rust
pub use super::sum::sum;
```

#### Re-export `var_pop`

```rust
pub use super::variance::var_pop;
```

#### Re-export `var_sample`

```rust
pub use super::variance::var_sample;
```

## Functions

### Function `all_default_aggregate_functions`

Returns all default aggregate functions

```rust
pub fn all_default_aggregate_functions() -> Vec<std::sync::Arc<datafusion_expr::AggregateUDF>> { /* ... */ }
```

### Function `register_all`

Registers all enabled packages with a [`FunctionRegistry`]

```rust
pub fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()> { /* ... */ }
```


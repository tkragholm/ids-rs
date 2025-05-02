# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_physical_expr`

## Modules

## Module `aggregate`

```rust
pub mod aggregate { /* ... */ }
```

### Modules

## Module `utils`

```rust
pub mod utils { /* ... */ }
```

### Re-exports

#### Re-export `adjust_output_array`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use datafusion_functions_aggregate_common::utils::adjust_output_array;
```

#### Re-export `get_accum_scalar_values_as_arrays`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use datafusion_functions_aggregate_common::utils::get_accum_scalar_values_as_arrays;
```

#### Re-export `get_sort_options`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use datafusion_functions_aggregate_common::utils::get_sort_options;
```

#### Re-export `ordering_fields`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use datafusion_functions_aggregate_common::utils::ordering_fields;
```

#### Re-export `DecimalAverager`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use datafusion_functions_aggregate_common::utils::DecimalAverager;
```

#### Re-export `Hashable`

**Attributes:**

- `#[allow(deprecated)]`

```rust
pub use datafusion_functions_aggregate_common::utils::Hashable;
```

### Types

#### Struct `AggregateExprBuilder`

Builder for physical [`AggregateFunctionExpr`]

`AggregateFunctionExpr` contains the information necessary to call
an aggregate expression.

```rust
pub struct AggregateExprBuilder {
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
  pub fn new(fun: Arc<AggregateUDF>, args: Vec<Arc<dyn PhysicalExpr>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: Self) -> Result<AggregateFunctionExpr> { /* ... */ }
  ```
  Constructs an `AggregateFunctionExpr` from the builder

- ```rust
  pub fn alias</* synthetic */ impl Into<String>: Into<String>>(self: Self, alias: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn human_display(self: Self, name: String) -> Self { /* ... */ }
  ```

- ```rust
  pub fn schema(self: Self, schema: SchemaRef) -> Self { /* ... */ }
  ```

- ```rust
  pub fn order_by(self: Self, order_by: LexOrdering) -> Self { /* ... */ }
  ```

- ```rust
  pub fn reversed(self: Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_reversed(self: Self, is_reversed: bool) -> Self { /* ... */ }
  ```

- ```rust
  pub fn distinct(self: Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_distinct(self: Self, is_distinct: bool) -> Self { /* ... */ }
  ```

- ```rust
  pub fn ignore_nulls(self: Self) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_ignore_nulls(self: Self, ignore_nulls: bool) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AggregateExprBuilder { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `AggregateFunctionExpr`

Physical aggregate expression of a UDAF.

Instances are constructed via [`AggregateExprBuilder`].

```rust
pub struct AggregateFunctionExpr {
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
  pub fn fun(self: &Self) -> &AggregateUDF { /* ... */ }
  ```
  Return the `AggregateUDF` used by this `AggregateFunctionExpr`

- ```rust
  pub fn expressions(self: &Self) -> Vec<Arc<dyn PhysicalExpr>> { /* ... */ }
  ```
  expressions that are passed to the Accumulator.

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Human readable name such as `"MIN(c2)"`.

- ```rust
  pub fn human_display(self: &Self) -> &str { /* ... */ }
  ```
  Simplified name for `tree` explain.

- ```rust
  pub fn is_distinct(self: &Self) -> bool { /* ... */ }
  ```
  Return if the aggregation is distinct

- ```rust
  pub fn ignore_nulls(self: &Self) -> bool { /* ... */ }
  ```
  Return if the aggregation ignores nulls

- ```rust
  pub fn is_reversed(self: &Self) -> bool { /* ... */ }
  ```
  Return if the aggregation is reversed

- ```rust
  pub fn is_nullable(self: &Self) -> bool { /* ... */ }
  ```
  Return if the aggregation is nullable

- ```rust
  pub fn field(self: &Self) -> Field { /* ... */ }
  ```
  the field of the final result of this aggregation.

- ```rust
  pub fn create_accumulator(self: &Self) -> Result<Box<dyn Accumulator>> { /* ... */ }
  ```
  the accumulator used to accumulate values from the expressions.

- ```rust
  pub fn state_fields(self: &Self) -> Result<Vec<Field>> { /* ... */ }
  ```
  the field of the final result of this aggregation.

- ```rust
  pub fn order_bys(self: &Self) -> Option<&LexOrdering> { /* ... */ }
  ```
  Order by requirements for the aggregate function

- ```rust
  pub fn order_sensitivity(self: &Self) -> AggregateOrderSensitivity { /* ... */ }
  ```
  Indicates whether aggregator can produce the correct result with any

- ```rust
  pub fn with_beneficial_ordering(self: Arc<Self>, beneficial_ordering: bool) -> Result<Option<AggregateFunctionExpr>> { /* ... */ }
  ```
  Sets the indicator whether ordering requirements of the aggregator is

- ```rust
  pub fn create_sliding_accumulator(self: &Self) -> Result<Box<dyn Accumulator>> { /* ... */ }
  ```
  Creates accumulator implementation that supports retract

- ```rust
  pub fn groups_accumulator_supported(self: &Self) -> bool { /* ... */ }
  ```
  If the aggregate expression has a specialized

- ```rust
  pub fn create_groups_accumulator(self: &Self) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
  ```
  Return a specialized [`GroupsAccumulator`] that manages state

- ```rust
  pub fn reverse_expr(self: &Self) -> Option<AggregateFunctionExpr> { /* ... */ }
  ```
  Construct an expression that calculates the aggregate in reverse.

- ```rust
  pub fn all_expressions(self: &Self) -> AggregatePhysicalExpressions { /* ... */ }
  ```
  Returns all expressions used in the [`AggregateFunctionExpr`].

- ```rust
  pub fn with_new_expressions(self: &Self, _args: Vec<Arc<dyn PhysicalExpr>>, _order_by_exprs: Vec<Arc<dyn PhysicalExpr>>) -> Option<AggregateFunctionExpr> { /* ... */ }
  ```
  Rewrites [`AggregateFunctionExpr`], with new expressions given. The argument should be consistent

- ```rust
  pub fn get_minmax_desc(self: &Self) -> Option<(Field, bool)> { /* ... */ }
  ```
  If this function is max, return (output_field, true)

- ```rust
  pub fn default_value(self: &Self, data_type: &DataType) -> Result<ScalarValue> { /* ... */ }
  ```
  Returns default value of the function given the input is Null

- ```rust
  pub fn set_monotonicity(self: &Self) -> SetMonotonicity { /* ... */ }
  ```
  Indicates whether the aggregation function is monotonic as a set

- ```rust
  pub fn get_result_ordering(self: &Self, aggr_func_idx: usize) -> Option<PhysicalSortExpr> { /* ... */ }
  ```
  Returns `PhysicalSortExpr` based on the set monotonicity of the function.

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Send**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AggregateFunctionExpr { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `AggregatePhysicalExpressions`

Stores the physical expressions used inside the `AggregateExpr`.

```rust
pub struct AggregatePhysicalExpressions {
    pub args: Vec<std::sync::Arc<dyn PhysicalExpr>>,
    pub order_by_exprs: Vec<std::sync::Arc<dyn PhysicalExpr>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `args` | `Vec<std::sync::Arc<dyn PhysicalExpr>>` | Aggregate function arguments |
| `order_by_exprs` | `Vec<std::sync::Arc<dyn PhysicalExpr>>` | Order by expressions |

##### Implementations

###### Trait Implementations

- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **RefUnwindSafe**
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

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
## Module `analysis`

Interval and selectivity in [`AnalysisContext`]

```rust
pub mod analysis { /* ... */ }
```

### Types

#### Struct `AnalysisContext`

The shared context used during the analysis of an expression. Includes
the boundaries for all known columns.

```rust
pub struct AnalysisContext {
    pub boundaries: Vec<ExprBoundaries>,
    pub selectivity: Option<f64>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `boundaries` | `Vec<ExprBoundaries>` |  |
| `selectivity` | `Option<f64>` | The estimated percentage of rows that this expression would select, if<br>it were to be used as a boolean predicate on a filter. The value will be<br>between 0.0 (selects nothing) and 1.0 (selects everything). |

##### Implementations

###### Methods

- ```rust
  pub fn new(boundaries: Vec<ExprBoundaries>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_selectivity(self: Self, selectivity: f64) -> Self { /* ... */ }
  ```

- ```rust
  pub fn try_from_statistics(input_schema: &Schema, statistics: &[ColumnStatistics]) -> Result<Self> { /* ... */ }
  ```
  Create a new analysis context from column statistics.

###### Trait Implementations

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AnalysisContext { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AnalysisContext) -> bool { /* ... */ }
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `ExprBoundaries`

Represents the boundaries (e.g. min and max values) of a particular column

This is used range analysis of expressions, to determine if the expression
limits the value of particular columns (e.g. analyzing an expression such as
`time < 50` would result in a boundary interval for `time` having a max
value of `50`).

```rust
pub struct ExprBoundaries {
    pub column: crate::expressions::Column,
    pub interval: Option<datafusion_expr::interval_arithmetic::Interval>,
    pub distinct_count: datafusion_common::stats::Precision<usize>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `column` | `crate::expressions::Column` |  |
| `interval` | `Option<datafusion_expr::interval_arithmetic::Interval>` | Minimum and maximum values this expression can have. A `None` value<br>indicates that evaluating the given column results in an empty set.<br>For example, if the column `a` has values in the range [10, 20],<br>and there is a filter asserting that `a > 50`, then the resulting interval<br>range of `a` will be `None`. |
| `distinct_count` | `datafusion_common::stats::Precision<usize>` | Maximum number of distinct values this expression can produce, if known. |

##### Implementations

###### Methods

- ```rust
  pub fn try_from_column(schema: &Schema, col_stats: &ColumnStatistics, col_index: usize) -> Result<Self> { /* ... */ }
  ```
  Create a new `ExprBoundaries` object from column level statistics.

- ```rust
  pub fn try_new_unbounded(schema: &Schema) -> Result<Vec<Self>> { /* ... */ }
  ```
  Create `ExprBoundaries` that represent no known bounds for all the

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExprBoundaries { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **StructuralPartialEq**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExprBoundaries) -> bool { /* ... */ }
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
- **Send**
### Functions

#### Function `analyze`

Attempts to refine column boundaries and compute a selectivity value.

The function accepts boundaries of the input columns in the `context` parameter.
It then tries to tighten these boundaries based on the provided `expr`.
The resulting selectivity value is calculated by comparing the initial and final boundaries.
The computation assumes that the data within the column is uniformly distributed and not sorted.

# Arguments

* `context` - The context holding input column boundaries.
* `expr` - The expression used to shrink the column boundaries.

# Returns

* `AnalysisContext` constructed by pruned boundaries and a selectivity value.

```rust
pub fn analyze(expr: &std::sync::Arc<dyn PhysicalExpr>, context: AnalysisContext, schema: &arrow::datatypes::Schema) -> datafusion_common::Result<AnalysisContext> { /* ... */ }
```

## Module `binary_map`

```rust
pub mod binary_map { /* ... */ }
```

### Re-exports

#### Re-export `ArrowBytesSet`

```rust
pub use datafusion_physical_expr_common::binary_map::ArrowBytesSet;
```

#### Re-export `OutputType`

```rust
pub use datafusion_physical_expr_common::binary_map::OutputType;
```

## Module `equivalence`

```rust
pub mod equivalence { /* ... */ }
```

### Functions

#### Function `collapse_lex_req`

**Attributes:**

- `#[deprecated(since = "45.0.0", note = "Use LexRequirement::collapse")]`

**⚠️ Deprecated since 45.0.0**: Use LexRequirement::collapse

This function constructs a duplicate-free `LexOrderingReq` by filtering out
duplicate entries that have same physical expression inside. For example,
`vec![a Some(ASC), a Some(DESC)]` collapses to `vec![a Some(ASC)]`.

It will also filter out entries that are ordered if the next entry is;
for instance, `vec![floor(a) Some(ASC), a Some(ASC)]` will be collapsed to
`vec![a Some(ASC)]`.

```rust
pub fn collapse_lex_req(input: crate::LexRequirement) -> crate::LexRequirement { /* ... */ }
```

#### Function `add_offset_to_expr`

Adds the `offset` value to `Column` indices inside `expr`. This function is
generally used during the update of the right table schema in join operations.

```rust
pub fn add_offset_to_expr(expr: std::sync::Arc<dyn PhysicalExpr>, offset: usize) -> std::sync::Arc<dyn PhysicalExpr> { /* ... */ }
```

### Re-exports

#### Re-export `AcrossPartitions`

```rust
pub use class::AcrossPartitions;
```

#### Re-export `ConstExpr`

```rust
pub use class::ConstExpr;
```

#### Re-export `EquivalenceClass`

```rust
pub use class::EquivalenceClass;
```

#### Re-export `EquivalenceGroup`

```rust
pub use class::EquivalenceGroup;
```

#### Re-export `OrderingEquivalenceClass`

```rust
pub use ordering::OrderingEquivalenceClass;
```

#### Re-export `ProjectionMapping`

```rust
pub use projection::ProjectionMapping;
```

#### Re-export `calculate_union`

```rust
pub use properties::calculate_union;
```

#### Re-export `join_equivalence_properties`

```rust
pub use properties::join_equivalence_properties;
```

#### Re-export `EquivalenceProperties`

```rust
pub use properties::EquivalenceProperties;
```

## Module `expressions`

Defines physical expressions that can evaluated at runtime during query execution

```rust
pub mod expressions { /* ... */ }
```

### Re-exports

#### Re-export `StatsType`

Module with some convenient methods used in expression building

```rust
pub use crate::aggregate::stats::StatsType;
```

#### Re-export `PhysicalSortExpr`

```rust
pub use crate::PhysicalSortExpr;
```

#### Re-export `binary`

```rust
pub use binary::binary;
```

#### Re-export `similar_to`

```rust
pub use binary::similar_to;
```

#### Re-export `BinaryExpr`

```rust
pub use binary::BinaryExpr;
```

#### Re-export `case`

```rust
pub use case::case;
```

#### Re-export `CaseExpr`

```rust
pub use case::CaseExpr;
```

#### Re-export `cast`

```rust
pub use cast::cast;
```

#### Re-export `CastExpr`

```rust
pub use cast::CastExpr;
```

#### Re-export `col`

```rust
pub use column::col;
```

#### Re-export `with_new_schema`

```rust
pub use column::with_new_schema;
```

#### Re-export `Column`

```rust
pub use column::Column;
```

#### Re-export `format_state_name`

```rust
pub use datafusion_expr::utils::format_state_name;
```

#### Re-export `in_list`

```rust
pub use in_list::in_list;
```

#### Re-export `InListExpr`

```rust
pub use in_list::InListExpr;
```

#### Re-export `is_not_null`

```rust
pub use is_not_null::is_not_null;
```

#### Re-export `IsNotNullExpr`

```rust
pub use is_not_null::IsNotNullExpr;
```

#### Re-export `is_null`

```rust
pub use is_null::is_null;
```

#### Re-export `IsNullExpr`

```rust
pub use is_null::IsNullExpr;
```

#### Re-export `like`

```rust
pub use like::like;
```

#### Re-export `LikeExpr`

```rust
pub use like::LikeExpr;
```

#### Re-export `lit`

```rust
pub use literal::lit;
```

#### Re-export `Literal`

```rust
pub use literal::Literal;
```

#### Re-export `negative`

```rust
pub use negative::negative;
```

#### Re-export `NegativeExpr`

```rust
pub use negative::NegativeExpr;
```

#### Re-export `NoOp`

```rust
pub use no_op::NoOp;
```

#### Re-export `not`

```rust
pub use not::not;
```

#### Re-export `NotExpr`

```rust
pub use not::NotExpr;
```

#### Re-export `try_cast`

```rust
pub use try_cast::try_cast;
```

#### Re-export `TryCastExpr`

```rust
pub use try_cast::TryCastExpr;
```

#### Re-export `UnKnownColumn`

```rust
pub use unknown_column::UnKnownColumn;
```

## Module `intervals`

Interval arithmetic and constraint propagation library

```rust
pub mod intervals { /* ... */ }
```

### Modules

## Module `cp_solver`

Constraint propagator/solver for custom [`PhysicalExpr`] graphs.

The constraint propagator/solver in DataFusion uses interval arithmetic to
perform mathematical operations on intervals, which represent a range of
possible values rather than a single point value. This allows for the
propagation of ranges through mathematical operations, and can be used to
compute bounds for a complicated expression. The key idea is that by
breaking down a complicated expression into simpler terms, and then
combining the bounds for those simpler terms, one can obtain bounds for the
overall expression.

This way of using interval arithmetic to compute bounds for a complex
expression by combining the bounds for the constituent terms within the
original expression allows us to reason about the range of possible values
of the expression. This information later can be used in range pruning of
the provably unnecessary parts of `RecordBatch`es.

# Example

For example, consider a mathematical expression such as `x^2 + y = 4` \[1\].
Since this expression would be a binary tree in [`PhysicalExpr`] notation,
this type of an hierarchical computation is well-suited for a graph based
implementation. In such an implementation, an equation system `f(x) = 0` is
represented by a directed acyclic expression graph (DAEG).

In order to use interval arithmetic to compute bounds for this expression,
one would first determine intervals that represent the possible values of
`x` and `y`` Let's say that the interval for `x` is `[1, 2]` and the interval
for `y` is `[-3, 1]`. In the chart below, you can see how the computation
takes place.

# References

1. Kabak, Mehmet Ozan. Analog Circuit Start-Up Behavior Analysis: An Interval
   Arithmetic Based Approach, Chapter 4. Stanford University, 2015.
2. Moore, Ramon E. Interval analysis. Vol. 4. Englewood Cliffs: Prentice-Hall, 1966.
3. F. Messine, "Deterministic global optimization using interval constraint
   propagation techniques," RAIRO-Operations Research, vol. 38, no. 04,
   pp. 277-293, 2004.

# Illustration

## Computing bounds for an expression using interval arithmetic

```text
            +-----+                         +-----+
       +----|  +  |----+               +----|  +  |----+
       |    |     |    |               |    |     |    |
       |    +-----+    |               |    +-----+    |
       |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |           |  y  |       |   2 | [1, 4]    |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
      |                               |
      |                               |
    +---+                           +---+
    | x | [1, 2]                    | x | [1, 2]
    +---+                           +---+

 (a) Bottom-up evaluation: Step 1 (b) Bottom up evaluation: Step 2

                                     [1 - 3, 4 + 1] = [-2, 5]
            +-----+                         +-----+
       +----|  +  |----+               +----|  +  |----+
       |    |     |    |               |    |     |    |
       |    +-----+    |               |    +-----+    |
       |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |[1, 4]     |  y  |       |   2 |[1, 4]     |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
      |              [-3, 1]          |              [-3, 1]
      |                               |
    +---+                           +---+
    | x | [1, 2]                    | x | [1, 2]
    +---+                           +---+

 (c) Bottom-up evaluation: Step 3 (d) Bottom-up evaluation: Step 4
```

## Top-down constraint propagation using inverse semantics

```text
   [-2, 5] ∩ [4, 4] = [4, 4]               [4, 4]
           +-----+                         +-----+
      +----|  +  |----+               +----|  +  |----+
      |    |     |    |               |    |     |    |
      |    +-----+    |               |    +-----+    |
      |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 | [1, 4]    |  y  |       |   2 | [1, 4]    |  y  | [0, 1]*
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
     |              [-3, 1]          |
     |                               |
   +---+                           +---+
   | x | [1, 2]                    | x | [1, 2]
   +---+                           +---+

 (a) Top-down propagation: Step 1 (b) Top-down propagation: Step 2

                                    [1 - 3, 4 + 1] = [-2, 5]
           +-----+                         +-----+
      +----|  +  |----+               +----|  +  |----+
      |    |     |    |               |    |     |    |
      |    +-----+    |               |    +-----+    |
      |               |               |               |
   +-----+           +-----+       +-----+           +-----+
   |   2 |[3, 4]**   |  y  |       |   2 |[3, 4]     |  y  |
   |[.]  |           |     |       |[.]  |           |     |
   +-----+           +-----+       +-----+           +-----+
     |              [0, 1]           |              [-3, 1]
     |                               |
   +---+                           +---+
   | x | [1, 2]                    | x | [sqrt(3), 2]***
   +---+                           +---+

 (c) Top-down propagation: Step 3  (d) Top-down propagation: Step 4

   * [-3, 1] ∩ ([4, 4] - [1, 4]) = [0, 1]
   ** [1, 4] ∩ ([4, 4] - [0, 1]) = [3, 4]
   *** [1, 2] ∩ [sqrt(3), sqrt(4)] = [sqrt(3), 2]
```

```rust
pub mod cp_solver { /* ... */ }
```

### Types

#### Struct `ExprIntervalGraph`

This object implements a directed acyclic expression graph (DAEG) that
is used to compute ranges for expressions through interval arithmetic.

```rust
pub struct ExprIntervalGraph {
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
  pub fn try_new(expr: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self> { /* ... */ }
  ```

- ```rust
  pub fn node_count(self: &Self) -> usize { /* ... */ }
  ```

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Estimate size of bytes including `Self`.

- ```rust
  pub fn gather_node_indices(self: &mut Self, exprs: &[Arc<dyn PhysicalExpr>]) -> Vec<(Arc<dyn PhysicalExpr>, usize)> { /* ... */ }
  ```
  This function associates stable node indices with [`PhysicalExpr`]s so

- ```rust
  pub fn update_ranges(self: &mut Self, leaf_bounds: &mut [(usize, Interval)], given_range: Interval) -> Result<PropagationResult> { /* ... */ }
  ```
  Updates intervals for all expressions in the DAEG by successive

- ```rust
  pub fn assign_intervals(self: &mut Self, assignments: &[(usize, Interval)]) { /* ... */ }
  ```
  This function assigns given ranges to expressions in the DAEG.

- ```rust
  pub fn update_intervals(self: &Self, assignments: &mut [(usize, Interval)]) { /* ... */ }
  ```
  This function fetches ranges of expressions from the DAEG. The argument

- ```rust
  pub fn evaluate_bounds(self: &mut Self) -> Result<&Interval> { /* ... */ }
  ```
  Computes bounds for an expression using interval arithmetic via a

- ```rust
  pub fn get_interval(self: &Self, index: usize) -> Interval { /* ... */ }
  ```
  Returns the interval associated with the node at the given `index`.

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExprIntervalGraph { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
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

- **Freeze**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Enum `PropagationResult`

This object encapsulates all possible constraint propagation results.

```rust
pub enum PropagationResult {
    CannotPropagate,
    Infeasible,
    Success,
}
```

##### Variants

###### `CannotPropagate`

###### `Infeasible`

###### `Success`

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
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

- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PropagationResult) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
#### Struct `ExprIntervalGraphNode`

This is a node in the DAEG; it encapsulates a reference to the actual
[`PhysicalExpr`] as well as an interval containing expression bounds.

```rust
pub struct ExprIntervalGraphNode {
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
  pub fn new_unbounded(expr: Arc<dyn PhysicalExpr>, dt: &DataType) -> Result<Self> { /* ... */ }
  ```
  Constructs a new DAEG node with an `[-∞, ∞]` range.

- ```rust
  pub fn new_with_interval(expr: Arc<dyn PhysicalExpr>, interval: Interval) -> Self { /* ... */ }
  ```
  Constructs a new DAEG node with the given range.

- ```rust
  pub fn interval(self: &Self) -> &Interval { /* ... */ }
  ```
  Get the interval object representing the range of the expression.

- ```rust
  pub fn make_node(node: &ExprTreeNode<NodeIndex>, schema: &Schema) -> Result<Self> { /* ... */ }
  ```
  This function creates a DAEG node from DataFusion's [`ExprTreeNode`]

###### Trait Implementations

- **MaybeSendSync**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExprIntervalGraphNode { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **IntoEither**
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

- **Freeze**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
### Functions

#### Function `propagate_arithmetic`

This function refines intervals `left_child` and `right_child` by applying
constraint propagation through `parent` via operation. The main idea is
that we can shrink ranges of variables x and y using parent interval p.

Assuming that x,y and p has ranges `[xL, xU]`, `[yL, yU]`, and `[pL, pU]`, we
apply the following operations:
- For plus operation, specifically, we would first do
    - `[xL, xU]` <- (`[pL, pU]` - `[yL, yU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[pL, pU]` - `[xL, xU]`) ∩ `[yL, yU]`.
- For minus operation, specifically, we would first do
    - `[xL, xU]` <- (`[yL, yU]` + `[pL, pU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[xL, xU]` - `[pL, pU]`) ∩ `[yL, yU]`.
- For multiplication operation, specifically, we would first do
    - `[xL, xU]` <- (`[pL, pU]` / `[yL, yU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[pL, pU]` / `[xL, xU]`) ∩ `[yL, yU]`.
- For division operation, specifically, we would first do
    - `[xL, xU]` <- (`[yL, yU]` * `[pL, pU]`) ∩ `[xL, xU]`, and then
    - `[yL, yU]` <- (`[xL, xU]` / `[pL, pU]`) ∩ `[yL, yU]`.

```rust
pub fn propagate_arithmetic(op: &datafusion_expr::Operator, parent: &datafusion_expr::interval_arithmetic::Interval, left_child: &datafusion_expr::interval_arithmetic::Interval, right_child: &datafusion_expr::interval_arithmetic::Interval) -> datafusion_common::Result<Option<(datafusion_expr::interval_arithmetic::Interval, datafusion_expr::interval_arithmetic::Interval)>> { /* ... */ }
```

#### Function `propagate_comparison`

This function refines intervals `left_child` and `right_child` by applying
comparison propagation through `parent` via operation. The main idea is
that we can shrink ranges of variables x and y using parent interval p.
Two intervals can be ordered in 6 ways for a Gt `>` operator:
```text
                          (1): Infeasible, short-circuit
left:   |        ================                                               |
right:  |                           ========================                    |

                            (2): Update both interval
left:   |              ======================                                   |
right:  |                             ======================                    |
                                         |
                                         V
left:   |                             =======                                   |
right:  |                             =======                                   |

                            (3): Update left interval
left:   |                  ==============================                       |
right:  |                           ==========                                  |
                                         |
                                         V
left:   |                           =====================                       |
right:  |                           ==========                                  |

                            (4): Update right interval
left:   |                           ==========                                  |
right:  |                   ===========================                         |
                                         |
                                         V
left:   |                           ==========                                  |
right   |                   ==================                                  |

                                  (5): No change
left:   |                       ============================                    |
right:  |               ===================                                     |

                                  (6): No change
left:   |                                    ====================               |
right:  |                ===============                                        |

        -inf --------------------------------------------------------------- +inf
```

```rust
pub fn propagate_comparison(op: &datafusion_expr::Operator, parent: &datafusion_expr::interval_arithmetic::Interval, left_child: &datafusion_expr::interval_arithmetic::Interval, right_child: &datafusion_expr::interval_arithmetic::Interval) -> datafusion_common::Result<Option<(datafusion_expr::interval_arithmetic::Interval, datafusion_expr::interval_arithmetic::Interval)>> { /* ... */ }
```

## Module `test_utils`

Test utilities for the interval arithmetic library

```rust
pub mod test_utils { /* ... */ }
```

### Functions

#### Function `gen_conjunctive_numerical_expr`

**Attributes:**

- `#[allow(clippy::too_many_arguments)]`

This test function generates a conjunctive statement with two numeric
terms with the following form:
left_col (op_1) a  >/>= right_col (op_2) b AND left_col (op_3) c </<= right_col (op_4) d

```rust
pub fn gen_conjunctive_numerical_expr(left_col: std::sync::Arc<dyn PhysicalExpr>, right_col: std::sync::Arc<dyn PhysicalExpr>, op: (datafusion_expr::Operator, datafusion_expr::Operator, datafusion_expr::Operator, datafusion_expr::Operator), a: datafusion_common::ScalarValue, b: datafusion_common::ScalarValue, c: datafusion_common::ScalarValue, d: datafusion_common::ScalarValue, bounds: (datafusion_expr::Operator, datafusion_expr::Operator)) -> std::sync::Arc<dyn PhysicalExpr> { /* ... */ }
```

#### Function `gen_conjunctive_temporal_expr`

**Attributes:**

- `#[allow(clippy::too_many_arguments)]`

This test function generates a conjunctive statement with
two scalar values with the following form:
left_col (op_1) a  > right_col (op_2) b AND left_col (op_3) c < right_col (op_4) d

```rust
pub fn gen_conjunctive_temporal_expr(left_col: std::sync::Arc<dyn PhysicalExpr>, right_col: std::sync::Arc<dyn PhysicalExpr>, op_1: datafusion_expr::Operator, op_2: datafusion_expr::Operator, op_3: datafusion_expr::Operator, op_4: datafusion_expr::Operator, a: datafusion_common::ScalarValue, b: datafusion_common::ScalarValue, c: datafusion_common::ScalarValue, d: datafusion_common::ScalarValue, schema: &arrow::datatypes::Schema) -> Result<std::sync::Arc<dyn PhysicalExpr>, datafusion_common::DataFusionError> { /* ... */ }
```

## Module `utils`

Utility functions for the interval arithmetic library

```rust
pub mod utils { /* ... */ }
```

### Functions

#### Function `check_support`

Indicates whether interval arithmetic is supported for the given expression.
Currently, we do not support all [`PhysicalExpr`]s for interval calculations.
We do not support every type of [`Operator`]s either. Over time, this check
will relax as more types of `PhysicalExpr`s and `Operator`s are supported.
Currently, [`CastExpr`], [`NegativeExpr`], [`BinaryExpr`], [`Column`] and [`Literal`] are supported.

```rust
pub fn check_support(expr: &std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::SchemaRef) -> bool { /* ... */ }
```

#### Function `get_inverse_op`

```rust
pub fn get_inverse_op(op: datafusion_expr::Operator) -> datafusion_common::Result<datafusion_expr::Operator> { /* ... */ }
```

#### Function `is_operator_supported`

Indicates whether interval arithmetic is supported for the given operator.

```rust
pub fn is_operator_supported(op: &datafusion_expr::Operator) -> bool { /* ... */ }
```

#### Function `is_datatype_supported`

Indicates whether interval arithmetic is supported for the given data type.

```rust
pub fn is_datatype_supported(data_type: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `convert_interval_type_to_duration`

Converts an [`Interval`] of time intervals to one of `Duration`s, if applicable. Otherwise, returns [`None`].

```rust
pub fn convert_interval_type_to_duration(interval: &datafusion_expr::interval_arithmetic::Interval) -> Option<datafusion_expr::interval_arithmetic::Interval> { /* ... */ }
```

#### Function `convert_duration_type_to_interval`

Converts an [`Interval`] of `Duration`s to one of time intervals, if applicable. Otherwise, returns [`None`].

```rust
pub fn convert_duration_type_to_interval(interval: &datafusion_expr::interval_arithmetic::Interval) -> Option<datafusion_expr::interval_arithmetic::Interval> { /* ... */ }
```

## Module `planner`

```rust
pub mod planner { /* ... */ }
```

### Functions

#### Function `create_physical_expr`

[PhysicalExpr] evaluate DataFusion expressions such as `A + 1`, or `CAST(c1
AS int)`.

[PhysicalExpr] are the physical counterpart to [Expr] used in logical
planning, and can be evaluated directly on a [RecordBatch]. They are
normally created from [Expr] by a [PhysicalPlanner] and can be created
directly using [create_physical_expr].

A Physical expression knows its type, nullability and how to evaluate itself.

[PhysicalPlanner]: https://docs.rs/datafusion/latest/datafusion/physical_planner/trait.PhysicalPlanner.html
[RecordBatch]: https://docs.rs/arrow/latest/arrow/record_batch/struct.RecordBatch.html

# Example: Create `PhysicalExpr` from `Expr`
```
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_common::DFSchema;
# use datafusion_expr::{Expr, col, lit};
# use datafusion_physical_expr::create_physical_expr;
# use datafusion_expr::execution_props::ExecutionProps;
// For a logical expression `a = 1`, we can create a physical expression
let expr = col("a").eq(lit(1));
// To create a PhysicalExpr we need 1. a schema
let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
let df_schema = DFSchema::try_from(schema).unwrap();
// 2. ExecutionProps
let props = ExecutionProps::new();
// We can now create a PhysicalExpr:
let physical_expr = create_physical_expr(&expr, &df_schema, &props).unwrap();
```

# Example: Executing a PhysicalExpr to obtain [ColumnarValue]
```
# use std::sync::Arc;
# use arrow::array::{cast::AsArray, BooleanArray, Int32Array, RecordBatch};
# use arrow::datatypes::{DataType, Field, Schema};
# use datafusion_common::{assert_batches_eq, DFSchema};
# use datafusion_expr::{Expr, col, lit, ColumnarValue};
# use datafusion_physical_expr::create_physical_expr;
# use datafusion_expr::execution_props::ExecutionProps;
# let expr = col("a").eq(lit(1));
# let schema = Schema::new(vec![Field::new("a", DataType::Int32, true)]);
# let df_schema = DFSchema::try_from(schema.clone()).unwrap();
# let props = ExecutionProps::new();
// Given a PhysicalExpr, for `a = 1` we can evaluate it against a RecordBatch like this:
let physical_expr = create_physical_expr(&expr, &df_schema, &props).unwrap();
// Input of [1,2,3]
let input_batch = RecordBatch::try_from_iter(vec![
  ("a", Arc::new(Int32Array::from(vec![1, 2, 3])) as _)
]).unwrap();
// The result is a ColumnarValue (either an Array or a Scalar)
let result = physical_expr.evaluate(&input_batch).unwrap();
// In this case, a BooleanArray with the result of the comparison
let ColumnarValue::Array(arr) = result else {
 panic!("Expected an array")
};
assert_eq!(arr.as_boolean(), &BooleanArray::from(vec![true, false, false]));
```

[ColumnarValue]: datafusion_expr::ColumnarValue

Create a physical expression from a logical expression ([Expr]).

# Arguments

* `e` - The logical expression
* `input_dfschema` - The DataFusion schema for the input, used to resolve `Column` references
  to qualified or unqualified fields by name.

```rust
pub fn create_physical_expr(e: &datafusion_expr::Expr, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

#### Function `create_physical_exprs`

Create vector of Physical Expression from a vector of logical expression

```rust
pub fn create_physical_exprs<''a, I>(exprs: I, input_dfschema: &datafusion_common::DFSchema, execution_props: &datafusion_expr::execution_props::ExecutionProps) -> datafusion_common::Result<Vec<std::sync::Arc<dyn PhysicalExpr>>>
where
    I: IntoIterator<Item = &''a datafusion_expr::Expr> { /* ... */ }
```

#### Function `logical2physical`

Convert a logical expression to a physical expression (without any simplification, etc)

```rust
pub fn logical2physical(expr: &datafusion_expr::Expr, schema: &arrow::datatypes::Schema) -> std::sync::Arc<dyn PhysicalExpr> { /* ... */ }
```

## Module `statistics`

Statistics and constraint propagation library

```rust
pub mod statistics { /* ... */ }
```

### Modules

## Module `stats_solver`

```rust
pub mod stats_solver { /* ... */ }
```

### Types

#### Struct `ExprStatisticsGraph`

This object implements a directed acyclic expression graph (DAEG) that
is used to compute statistics/distributions for expressions hierarchically.

```rust
pub struct ExprStatisticsGraph {
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
  pub fn try_new(expr: Arc<dyn PhysicalExpr>, schema: &Schema) -> Result<Self> { /* ... */ }
  ```

- ```rust
  pub fn assign_statistics(self: &mut Self, assignments: &[(usize, Distribution)]) { /* ... */ }
  ```
  This function assigns given distributions to expressions in the DAEG.

- ```rust
  pub fn evaluate_statistics(self: &mut Self) -> Result<&Distribution> { /* ... */ }
  ```
  Computes statistics/distributions for an expression via a bottom-up

- ```rust
  pub fn propagate_statistics(self: &mut Self, given_stats: Distribution) -> Result<PropagationResult> { /* ... */ }
  ```
  Runs a propagation mechanism in a top-down manner to update statistics

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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExprStatisticsGraph { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
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

- **RefUnwindSafe**
- **MaybeSendSync**
#### Struct `ExprStatisticsGraphNode`

This is a node in the DAEG; it encapsulates a reference to the actual
[`PhysicalExpr`] as well as its statistics/distribution.

```rust
pub struct ExprStatisticsGraphNode {
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
  pub fn distribution(self: &Self) -> &Distribution { /* ... */ }
  ```
  Get the [`Distribution`] object representing the statistics of the

- ```rust
  pub fn make_node(node: &ExprTreeNode<NodeIndex>, schema: &Schema) -> Result<Self> { /* ... */ }
  ```
  This function creates a DAEG node from DataFusion's [`ExprTreeNode`]

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
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

- **Unpin**
- **IntoEither**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExprStatisticsGraphNode { /* ... */ }
    ```

## Module `utils`

```rust
pub mod utils { /* ... */ }
```

### Types

#### Type Alias `ExprTreeNode`

```rust
pub type ExprTreeNode<T> = crate::tree_node::ExprContext<Option<T>>;
```

### Functions

#### Function `split_conjunction`

Assume the predicate is in the form of CNF, split the predicate to a Vec of PhysicalExprs.

For example, split "a1 = a2 AND b1 <= b2 AND c1 != c2" into ["a1 = a2", "b1 <= b2", "c1 != c2"]

```rust
pub fn split_conjunction(predicate: &std::sync::Arc<dyn PhysicalExpr>) -> Vec<&std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

#### Function `conjunction`

Create a conjunction of the given predicates.
If the input is empty, return a literal true.
If the input contains a single predicate, return the predicate.
Otherwise, return a conjunction of the predicates (e.g. `a AND b AND c`).

```rust
pub fn conjunction</* synthetic */ impl IntoIterator<Item = Arc<dyn PhysicalExpr>>: IntoIterator<Item = std::sync::Arc<dyn PhysicalExpr>>>(predicates: impl IntoIterator<Item = std::sync::Arc<dyn PhysicalExpr>>) -> std::sync::Arc<dyn PhysicalExpr> { /* ... */ }
```

#### Function `conjunction_opt`

Create a conjunction of the given predicates.
If the input is empty or the return None.
If the input contains a single predicate, return Some(predicate).
Otherwise, return a Some(..) of a conjunction of the predicates (e.g. `Some(a AND b AND c)`).

```rust
pub fn conjunction_opt</* synthetic */ impl IntoIterator<Item = Arc<dyn PhysicalExpr>>: IntoIterator<Item = std::sync::Arc<dyn PhysicalExpr>>>(predicates: impl IntoIterator<Item = std::sync::Arc<dyn PhysicalExpr>>) -> Option<std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

#### Function `split_disjunction`

Assume the predicate is in the form of DNF, split the predicate to a Vec of PhysicalExprs.

For example, split "a1 = a2 OR b1 <= b2 OR c1 != c2" into ["a1 = a2", "b1 <= b2", "c1 != c2"]

```rust
pub fn split_disjunction(predicate: &std::sync::Arc<dyn PhysicalExpr>) -> Vec<&std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

#### Function `map_columns_before_projection`

This function maps back requirement after ProjectionExec
to the Executor for its input.

```rust
pub fn map_columns_before_projection(parent_required: &[std::sync::Arc<dyn PhysicalExpr>], proj_exprs: &[(std::sync::Arc<dyn PhysicalExpr>, String)]) -> Vec<std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

#### Function `convert_to_expr`

This function returns all `Arc<dyn PhysicalExpr>`s inside the given
`PhysicalSortExpr` sequence.

```rust
pub fn convert_to_expr<T: Borrow<crate::PhysicalSortExpr>, /* synthetic */ impl IntoIterator<Item = T>: IntoIterator<Item = T>>(sequence: impl IntoIterator<Item = T>) -> Vec<std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

#### Function `get_indices_of_exprs_strict`

This function finds the indices of `targets` within `items` using strict
equality.

```rust
pub fn get_indices_of_exprs_strict<T: Borrow<std::sync::Arc<dyn PhysicalExpr>>, /* synthetic */ impl IntoIterator<Item = T>: IntoIterator<Item = T>>(targets: impl IntoIterator<Item = T>, items: &[std::sync::Arc<dyn PhysicalExpr>]) -> Vec<usize> { /* ... */ }
```

#### Function `build_dag`

```rust
pub fn build_dag<T, F>(expr: std::sync::Arc<dyn PhysicalExpr>, constructor: &F) -> datafusion_common::Result<(petgraph::graph::NodeIndex, petgraph::stable_graph::StableGraph<T, usize>)>
where
    F: Fn(&ExprTreeNode<petgraph::graph::NodeIndex>) -> datafusion_common::Result<T> { /* ... */ }
```

#### Function `collect_columns`

Recursively extract referenced [`Column`]s within a [`PhysicalExpr`].

```rust
pub fn collect_columns(expr: &std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::HashSet<crate::expressions::Column> { /* ... */ }
```

#### Function `reassign_predicate_columns`

Re-assign column indices referenced in predicate according to given schema.
This may be helpful when dealing with projections.

```rust
pub fn reassign_predicate_columns(pred: std::sync::Arc<dyn PhysicalExpr>, schema: &arrow::datatypes::SchemaRef, ignore_not_found: bool) -> datafusion_common::Result<std::sync::Arc<dyn PhysicalExpr>> { /* ... */ }
```

#### Function `merge_vectors`

Merge left and right sort expressions, checking for duplicates.

```rust
pub fn merge_vectors(left: &datafusion_physical_expr_common::sort_expr::LexOrdering, right: &datafusion_physical_expr_common::sort_expr::LexOrdering) -> datafusion_physical_expr_common::sort_expr::LexOrdering { /* ... */ }
```

### Re-exports

#### Re-export `Guarantee`

```rust
pub use guarantee::Guarantee;
```

#### Re-export `LiteralGuarantee`

```rust
pub use guarantee::LiteralGuarantee;
```

## Module `window`

```rust
pub mod window { /* ... */ }
```

### Types

#### Type Alias `BuiltInWindowExpr`

**Attributes:**

- `#[deprecated(since = "44.0.0", note = "use StandardWindowExpr")]`

**⚠️ Deprecated since 44.0.0**: use StandardWindowExpr

```rust
pub type BuiltInWindowExpr = StandardWindowExpr;
```

#### Type Alias `BuiltInWindowFunctionExpr`

**Attributes:**

- `#[deprecated(since = "44.0.0", note = "use StandardWindowFunctionExpr")]`

**⚠️ Deprecated since 44.0.0**: use StandardWindowFunctionExpr

```rust
pub type BuiltInWindowFunctionExpr = dyn StandardWindowFunctionExpr;
```

### Re-exports

#### Re-export `PlainAggregateWindowExpr`

```rust
pub use aggregate::PlainAggregateWindowExpr;
```

#### Re-export `SlidingAggregateWindowExpr`

```rust
pub use sliding_aggregate::SlidingAggregateWindowExpr;
```

#### Re-export `StandardWindowExpr`

```rust
pub use standard::StandardWindowExpr;
```

#### Re-export `StandardWindowFunctionExpr`

```rust
pub use standard_window_function_expr::StandardWindowFunctionExpr;
```

#### Re-export `PartitionBatches`

```rust
pub use window_expr::PartitionBatches;
```

#### Re-export `PartitionKey`

```rust
pub use window_expr::PartitionKey;
```

#### Re-export `PartitionWindowAggStates`

```rust
pub use window_expr::PartitionWindowAggStates;
```

#### Re-export `WindowExpr`

```rust
pub use window_expr::WindowExpr;
```

#### Re-export `WindowState`

```rust
pub use window_expr::WindowState;
```

## Module `execution_props`

```rust
pub mod execution_props { /* ... */ }
```

### Re-exports

#### Re-export `ExecutionProps`

```rust
pub use datafusion_expr::execution_props::ExecutionProps;
```

#### Re-export `VarProvider`

```rust
pub use datafusion_expr::var_provider::VarProvider;
```

#### Re-export `VarType`

```rust
pub use datafusion_expr::var_provider::VarType;
```

## Module `tree_node`

```rust
pub mod tree_node { /* ... */ }
```

### Re-exports

#### Re-export `ExprContext`

```rust
pub use datafusion_physical_expr_common::tree_node::ExprContext;
```

## Re-exports

### Re-export `GroupsAccumulatorAdapter`

```rust
pub use aggregate::groups_accumulator::GroupsAccumulatorAdapter;
```

### Re-export `NullState`

```rust
pub use aggregate::groups_accumulator::NullState;
```

### Re-export `analyze`

```rust
pub use analysis::analyze;
```

### Re-export `AnalysisContext`

```rust
pub use analysis::AnalysisContext;
```

### Re-export `ExprBoundaries`

```rust
pub use analysis::ExprBoundaries;
```

### Re-export `calculate_union`

```rust
pub use equivalence::calculate_union;
```

### Re-export `AcrossPartitions`

```rust
pub use equivalence::AcrossPartitions;
```

### Re-export `ConstExpr`

```rust
pub use equivalence::ConstExpr;
```

### Re-export `EquivalenceProperties`

```rust
pub use equivalence::EquivalenceProperties;
```

### Re-export `Distribution`

```rust
pub use partitioning::Distribution;
```

### Re-export `Partitioning`

```rust
pub use partitioning::Partitioning;
```

### Re-export `create_ordering`

```rust
pub use physical_expr::create_ordering;
```

### Re-export `create_physical_sort_expr`

```rust
pub use physical_expr::create_physical_sort_expr;
```

### Re-export `create_physical_sort_exprs`

```rust
pub use physical_expr::create_physical_sort_exprs;
```

### Re-export `physical_exprs_bag_equal`

```rust
pub use physical_expr::physical_exprs_bag_equal;
```

### Re-export `physical_exprs_contains`

```rust
pub use physical_expr::physical_exprs_contains;
```

### Re-export `physical_exprs_equal`

```rust
pub use physical_expr::physical_exprs_equal;
```

### Re-export `PhysicalExprRef`

```rust
pub use physical_expr::PhysicalExprRef;
```

### Re-export `PhysicalExpr`

```rust
pub use datafusion_physical_expr_common::physical_expr::PhysicalExpr;
```

### Re-export `LexOrdering`

```rust
pub use datafusion_physical_expr_common::sort_expr::LexOrdering;
```

### Re-export `LexRequirement`

```rust
pub use datafusion_physical_expr_common::sort_expr::LexRequirement;
```

### Re-export `PhysicalSortExpr`

```rust
pub use datafusion_physical_expr_common::sort_expr::PhysicalSortExpr;
```

### Re-export `PhysicalSortRequirement`

```rust
pub use datafusion_physical_expr_common::sort_expr::PhysicalSortRequirement;
```

### Re-export `create_physical_expr`

```rust
pub use planner::create_physical_expr;
```

### Re-export `create_physical_exprs`

```rust
pub use planner::create_physical_exprs;
```

### Re-export `ScalarFunctionExpr`

```rust
pub use scalar_function::ScalarFunctionExpr;
```

### Re-export `reverse_order_bys`

```rust
pub use datafusion_physical_expr_common::utils::reverse_order_bys;
```

### Re-export `conjunction`

```rust
pub use utils::conjunction;
```

### Re-export `conjunction_opt`

```rust
pub use utils::conjunction_opt;
```

### Re-export `split_conjunction`

```rust
pub use utils::split_conjunction;
```


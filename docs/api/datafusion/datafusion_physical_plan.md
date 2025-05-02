# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_physical_plan`

Traits for physical query plan, supporting parallel execution for partitioned relations.

Entrypoint of this crate is trait [ExecutionPlan].

## Modules

## Module `aggregates`

Aggregates functionalities

```rust
pub mod aggregates { /* ... */ }
```

### Modules

## Module `order`

```rust
pub mod order { /* ... */ }
```

### Types

#### Enum `GroupOrdering`

Ordering information for each group in the hash table

```rust
pub enum GroupOrdering {
    None,
    Partial(GroupOrderingPartial),
    Full(GroupOrderingFull),
}
```

##### Variants

###### `None`

Groups are not ordered

###### `Partial`

Groups are ordered by some pre-set of the group keys

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `GroupOrderingPartial` |  |

###### `Full`

Groups are entirely contiguous,

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `GroupOrderingFull` |  |

##### Implementations

###### Methods

- ```rust
  pub fn try_new(input_schema: &Schema, mode: &InputOrderMode, ordering: &LexOrdering) -> Result<Self> { /* ... */ }
  ```
  Create a `GroupOrdering` for the specified ordering

- ```rust
  pub fn emit_to(self: &Self) -> Option<EmitTo> { /* ... */ }
  ```

- ```rust
  pub fn input_done(self: &mut Self) { /* ... */ }
  ```
  Updates the state the input is done

- ```rust
  pub fn remove_groups(self: &mut Self, n: usize) { /* ... */ }
  ```
  remove the first n groups from the internal state, shifting

- ```rust
  pub fn new_groups(self: &mut Self, batch_group_values: &[ArrayRef], group_indices: &[usize], total_num_groups: usize) -> Result<()> { /* ... */ }
  ```
  Called when new groups are added in a batch

- ```rust
  pub fn size(self: &Self) -> usize { /* ... */ }
  ```
  Return the size of memory used by the ordering state, in bytes

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Send**
- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Re-exports

#### Re-export `GroupOrderingFull`

```rust
pub use full::GroupOrderingFull;
```

#### Re-export `GroupOrderingPartial`

```rust
pub use partial::GroupOrderingPartial;
```

### Types

#### Enum `AggregateMode`

Aggregation modes

See [`Accumulator::state`] for background information on multi-phase
aggregation and how these modes are used.

```rust
pub enum AggregateMode {
    Partial,
    Final,
    FinalPartitioned,
    Single,
    SinglePartitioned,
}
```

##### Variants

###### `Partial`

One of multiple layers of aggregation, any input partitioning

Partial aggregate that can be applied in parallel across input
partitions.

This is the first phase of a multi-phase aggregation.

###### `Final`

*Final* of multiple layers of aggregation, in exactly one partition

Final aggregate that produces a single partition of output by combining
the output of multiple partial aggregates.

This is the second phase of a multi-phase aggregation.

This mode requires that the input is a single partition

Note: Adjacent `Partial` and `Final` mode aggregation is equivalent to a `Single`
mode aggregation node. The `Final` mode is required since this is used in an
intermediate step. The [`CombinePartialFinalAggregate`] physical optimizer rule
will replace this combination with `Single` mode for more efficient execution.

[`CombinePartialFinalAggregate`]: https://docs.rs/datafusion/latest/datafusion/physical_optimizer/combine_partial_final_agg/struct.CombinePartialFinalAggregate.html

###### `FinalPartitioned`

*Final* of multiple layers of aggregation, input is *Partitioned*

Final aggregate that works on pre-partitioned data.

This mode requires that all rows with a particular grouping key are in
the same partitions, such as is the case with Hash repartitioning on the
group keys. If a group key is duplicated, duplicate groups would be
produced

###### `Single`

*Single* layer of Aggregation, input is exactly one partition

Applies the entire logical aggregation operation in a single operator,
as opposed to Partial / Final modes which apply the logical aggregation using
two operators.

This mode requires that the input is a single partition (like Final)

###### `SinglePartitioned`

*Single* layer of Aggregation, input is *Partitioned*

Applies the entire logical aggregation operation in a single operator,
as opposed to Partial / Final modes which apply the logical aggregation
using two operators.

This mode requires that the input has more than one partition, and is
partitioned by group key (like FinalPartitioned).

##### Implementations

###### Methods

- ```rust
  pub fn is_first_stage(self: &Self) -> bool { /* ... */ }
  ```
  Checks whether this aggregation step describes a "first stage" calculation.

###### Trait Implementations

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AggregateMode { /* ... */ }
    ```

- **Eq**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Allocation**
- **Freeze**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Copy**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AggregateMode) -> bool { /* ... */ }
    ```

#### Struct `PhysicalGroupBy`

Represents `GROUP BY` clause in the plan (including the more general GROUPING SET)
In the case of a simple `GROUP BY a, b` clause, this will contain the expression [a, b]
and a single group [false, false].
In the case of `GROUP BY GROUPING SETS/CUBE/ROLLUP` the planner will expand the expression
into multiple groups, using null expressions to align each group.
For example, with a group by clause `GROUP BY GROUPING SETS ((a,b),(a),(b))` the planner should
create a `PhysicalGroupBy` like
```text
PhysicalGroupBy {
    expr: [(col(a), a), (col(b), b)],
    null_expr: [(NULL, a), (NULL, b)],
    groups: [
        [false, false], // (a,b)
        [false, true],  // (a) <=> (a, NULL)
        [true, false]   // (b) <=> (NULL, b)
    ]
}
```

```rust
pub struct PhysicalGroupBy {
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
  pub fn new(expr: Vec<(Arc<dyn PhysicalExpr>, String)>, null_expr: Vec<(Arc<dyn PhysicalExpr>, String)>, groups: Vec<Vec<bool>>) -> Self { /* ... */ }
  ```
  Create a new `PhysicalGroupBy`

- ```rust
  pub fn new_single(expr: Vec<(Arc<dyn PhysicalExpr>, String)>) -> Self { /* ... */ }
  ```
  Create a GROUPING SET with only a single group. This is the "standard"

- ```rust
  pub fn exprs_nullable(self: &Self) -> Vec<bool> { /* ... */ }
  ```
  Calculate GROUP BY expressions nullable

- ```rust
  pub fn expr(self: &Self) -> &[(Arc<dyn PhysicalExpr>, String)] { /* ... */ }
  ```
  Returns the group expressions

- ```rust
  pub fn null_expr(self: &Self) -> &[(Arc<dyn PhysicalExpr>, String)] { /* ... */ }
  ```
  Returns the null expressions

- ```rust
  pub fn groups(self: &Self) -> &[Vec<bool>] { /* ... */ }
  ```
  Returns the group null masks

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this `PhysicalGroupBy` has no group expressions

- ```rust
  pub fn is_single(self: &Self) -> bool { /* ... */ }
  ```
  Check whether grouping set is single group

- ```rust
  pub fn input_exprs(self: &Self) -> Vec<Arc<dyn PhysicalExpr>> { /* ... */ }
  ```
  Calculate GROUP BY expressions according to input schema.

- ```rust
  pub fn output_exprs(self: &Self) -> Vec<Arc<dyn PhysicalExpr>> { /* ... */ }
  ```
  Return grouping expressions as they occur in the output schema.

- ```rust
  pub fn group_schema(self: &Self, schema: &Schema) -> Result<SchemaRef> { /* ... */ }
  ```

- ```rust
  pub fn as_final(self: &Self) -> PhysicalGroupBy { /* ... */ }
  ```
  Returns the `PhysicalGroupBy` for a final aggregation if `self` is used for a partial

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> PhysicalGroupBy { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PhysicalGroupBy { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PhysicalGroupBy) -> bool { /* ... */ }
    ```

- **Sync**
#### Struct `AggregateExec`

Hash aggregate execution plan

```rust
pub struct AggregateExec {
    pub input: std::sync::Arc<dyn ExecutionPlan>,
    pub input_schema: arrow::datatypes::SchemaRef,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `input` | `std::sync::Arc<dyn ExecutionPlan>` | Input plan, could be a partial aggregate or the input to the aggregate |
| `input_schema` | `arrow::datatypes::SchemaRef` | Input schema before any aggregation is applied. For partial aggregate this will be the<br>same as input.schema() but for the final aggregate it will be the same as the input<br>to the partial aggregate, i.e., partial and final aggregates have same `input_schema`.<br>We need the input schema of partial aggregate to be able to deserialize aggregate<br>expressions from protobuf for final aggregate. |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn with_new_aggr_exprs(self: &Self, aggr_expr: Vec<Arc<AggregateFunctionExpr>>) -> Self { /* ... */ }
  ```
  Function used in `OptimizeAggregateOrder` optimizer rule,

- ```rust
  pub fn cache(self: &Self) -> &PlanProperties { /* ... */ }
  ```

- ```rust
  pub fn try_new(mode: AggregateMode, group_by: PhysicalGroupBy, aggr_expr: Vec<Arc<AggregateFunctionExpr>>, filter_expr: Vec<Option<Arc<dyn PhysicalExpr>>>, input: Arc<dyn ExecutionPlan>, input_schema: SchemaRef) -> Result<Self> { /* ... */ }
  ```
  Create a new hash aggregate execution plan

- ```rust
  pub fn mode(self: &Self) -> &AggregateMode { /* ... */ }
  ```
  Aggregation mode (full, partial)

- ```rust
  pub fn with_limit(self: Self, limit: Option<usize>) -> Self { /* ... */ }
  ```
  Set the `limit` of this AggExec

- ```rust
  pub fn group_expr(self: &Self) -> &PhysicalGroupBy { /* ... */ }
  ```
  Grouping expressions

- ```rust
  pub fn output_group_expr(self: &Self) -> Vec<Arc<dyn PhysicalExpr>> { /* ... */ }
  ```
  Grouping expressions as they occur in the output schema

- ```rust
  pub fn aggr_expr(self: &Self) -> &[Arc<AggregateFunctionExpr>] { /* ... */ }
  ```
  Aggregate expressions

- ```rust
  pub fn filter_expr(self: &Self) -> &[Option<Arc<dyn PhysicalExpr>>] { /* ... */ }
  ```
  FILTER (WHERE clause) expression for each aggregate expression

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input plan

- ```rust
  pub fn input_schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  Get the input schema before any aggregates are applied

- ```rust
  pub fn limit(self: &Self) -> Option<usize> { /* ... */ }
  ```
  number of rows soft limit of the AggregateExec

- ```rust
  pub fn get_minmax_desc(self: &Self) -> Option<(Field, bool)> { /* ... */ }
  ```
  Finds the DataType and SortDirection for this Aggregate, if there is one

- ```rust
  pub fn is_unordered_unfiltered_group_by_distinct(self: &Self) -> bool { /* ... */ }
  ```
  true, if this Aggregate has a group-by with no required or explicit ordering,

- ```rust
  pub fn compute_properties(input: &Arc<dyn ExecutionPlan>, schema: SchemaRef, group_expr_mapping: &ProjectionMapping, mode: &AggregateMode, input_order_mode: &InputOrderMode, aggr_exprs: &[Arc<AggregateFunctionExpr>]) -> PlanProperties { /* ... */ }
  ```
  This function creates the cache object that stores the plan properties such as schema, equivalence properties, ordering, partitioning, etc.

- ```rust
  pub fn input_order_mode(self: &Self) -> &InputOrderMode { /* ... */ }
  ```

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AggregateExec { /* ... */ }
    ```

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

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ExecutionPlan**
  - ```rust
    fn name(self: &Self) -> &''static str { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```
    Return a reference to Any that can be used for down-casting

  - ```rust
    fn properties(self: &Self) -> &PlanProperties { /* ... */ }
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
    The output ordering of [`AggregateExec`] is determined by its `group_by`

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
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Type Alias `AccumulatorItem`

```rust
pub type AccumulatorItem = Box<dyn Accumulator>;
```

### Functions

#### Function `concat_slices`

Concatenates the given slices.

```rust
pub fn concat_slices<T: Clone>(lhs: &[T], rhs: &[T]) -> Vec<T> { /* ... */ }
```

#### Function `get_finer_aggregate_exprs_requirement`

Get the common requirement that satisfies all the aggregate expressions.

# Parameters

- `aggr_exprs`: A slice of `AggregateFunctionExpr` containing all the
  aggregate expressions.
- `group_by`: A reference to a `PhysicalGroupBy` instance representing the
  physical GROUP BY expression.
- `eq_properties`: A reference to an `EquivalenceProperties` instance
  representing equivalence properties for ordering.
- `agg_mode`: A reference to an `AggregateMode` instance representing the
  mode of aggregation.

# Returns

A `LexRequirement` instance, which is the requirement that satisfies all the
aggregate requirements. Returns an error in case of conflicting requirements.

```rust
pub fn get_finer_aggregate_exprs_requirement(aggr_exprs: &mut [std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>], group_by: &PhysicalGroupBy, eq_properties: &datafusion_physical_expr::EquivalenceProperties, agg_mode: &AggregateMode) -> datafusion_common::Result<datafusion_physical_expr::LexRequirement> { /* ... */ }
```

#### Function `aggregate_expressions`

Returns physical expressions for arguments to evaluate against a batch.

The expressions are different depending on `mode`:
* Partial: AggregateFunctionExpr::expressions
* Final: columns of `AggregateFunctionExpr::state_fields()`

```rust
pub fn aggregate_expressions(aggr_expr: &[std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>], mode: &AggregateMode, col_idx_base: usize) -> datafusion_common::Result<Vec<Vec<std::sync::Arc<dyn PhysicalExpr>>>> { /* ... */ }
```

#### Function `create_accumulators`

```rust
pub fn create_accumulators(aggr_expr: &[std::sync::Arc<datafusion_physical_expr::aggregate::AggregateFunctionExpr>]) -> datafusion_common::Result<Vec<AccumulatorItem>> { /* ... */ }
```

#### Function `finalize_aggregation`

returns a vector of ArrayRefs, where each entry corresponds to either the
final value (mode = Final, FinalPartitioned and Single) or states (mode = Partial)

```rust
pub fn finalize_aggregation(accumulators: &mut [AccumulatorItem], mode: &AggregateMode) -> datafusion_common::Result<Vec<arrow::array::ArrayRef>> { /* ... */ }
```

## Module `analyze`

Defines the ANALYZE operator

```rust
pub mod analyze { /* ... */ }
```

### Types

#### Struct `AnalyzeExec`

`EXPLAIN ANALYZE` execution plan operator. This operator runs its input,
discards the results, and then prints out an annotated plan with metrics

```rust
pub struct AnalyzeExec {
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
  pub fn new(verbose: bool, show_statistics: bool, input: Arc<dyn ExecutionPlan>, schema: SchemaRef) -> Self { /* ... */ }
  ```
  Create a new AnalyzeExec

- ```rust
  pub fn verbose(self: &Self) -> bool { /* ... */ }
  ```
  Access to verbose

- ```rust
  pub fn show_statistics(self: &Self) -> bool { /* ... */ }
  ```
  Access to show_statistics

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  The input plan

###### Trait Implementations

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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AnalyzeExec { /* ... */ }
    ```

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn required_input_distribution(self: &Self) -> Vec<Distribution> { /* ... */ }
    ```
    AnalyzeExec is handled specially so this value is ignored

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

## Module `coalesce_batches`

[`CoalesceBatchesExec`] combines small batches into larger batches.

```rust
pub mod coalesce_batches { /* ... */ }
```

### Types

#### Struct `CoalesceBatchesExec`

`CoalesceBatchesExec` combines small batches into larger batches for more
efficient vectorized processing by later operators.

The operator buffers batches until it collects `target_batch_size` rows and
then emits a single concatenated batch. When only a limited number of rows
are necessary (specified by the `fetch` parameter), the operator will stop
buffering and returns the final batch once the number of collected rows
reaches the `fetch` value.

See [`BatchCoalescer`] for more information

```rust
pub struct CoalesceBatchesExec {
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
  pub fn new(input: Arc<dyn ExecutionPlan>, target_batch_size: usize) -> Self { /* ... */ }
  ```
  Create a new CoalesceBatchesExec

- ```rust
  pub fn with_fetch(self: Self, fetch: Option<usize>) -> Self { /* ... */ }
  ```
  Update fetch with the argument

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  The input plan

- ```rust
  pub fn target_batch_size(self: &Self) -> usize { /* ... */ }
  ```
  Minimum number of rows for coalesces batches

###### Trait Implementations

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **ErasedDestructor**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

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

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CoalesceBatchesExec { /* ... */ }
    ```

- **MaybeSendSync**
## Module `coalesce_partitions`

Defines the merge plan for executing partitions in parallel and then merging the results
into a single partition

```rust
pub mod coalesce_partitions { /* ... */ }
```

### Types

#### Struct `CoalescePartitionsExec`

Merge execution plan executes partitions in parallel and combines them into a single
partition. No guarantees are made about the order of the resulting partition.

```rust
pub struct CoalescePartitionsExec {
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
  pub fn new(input: Arc<dyn ExecutionPlan>) -> Self { /* ... */ }
  ```
  Create a new CoalescePartitionsExec

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input execution plan

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CoalescePartitionsExec { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
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
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn supports_limit_pushdown(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```
    Tries to swap `projection` with its input, which is known to be a

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
## Module `common`

Defines common code used in execution plans

```rust
pub mod common { /* ... */ }
```

### Types

#### Struct `IPCWriter`

Write in Arrow IPC File format.

```rust
pub struct IPCWriter {
    pub path: std::path::PathBuf,
    pub writer: arrow::ipc::writer::FileWriter<std::fs::File>,
    pub num_batches: usize,
    pub num_rows: usize,
    pub num_bytes: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `path` | `std::path::PathBuf` | Path |
| `writer` | `arrow::ipc::writer::FileWriter<std::fs::File>` | Inner writer |
| `num_batches` | `usize` | Batches written |
| `num_rows` | `usize` | Rows written |
| `num_bytes` | `usize` | Bytes written |

##### Implementations

###### Methods

- ```rust
  pub fn new(path: &Path, schema: &Schema) -> Result<Self> { /* ... */ }
  ```
  Create new writer

- ```rust
  pub fn new_with_options(path: &Path, schema: &Schema, write_options: IpcWriteOptions) -> Result<Self> { /* ... */ }
  ```
  Create new writer with IPC write options

- ```rust
  pub fn write(self: &mut Self, batch: &RecordBatch) -> Result<()> { /* ... */ }
  ```
  Write one single batch

- ```rust
  pub fn finish(self: &mut Self) -> Result<()> { /* ... */ }
  ```
  Finish the writer

- ```rust
  pub fn path(self: &Self) -> &Path { /* ... */ }
  ```
  Path write to

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **ErasedDestructor**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
### Functions

#### Function `collect`

Create a vector of record batches from a stream

```rust
pub async fn collect(stream: super::SendableRecordBatchStream) -> datafusion_common::Result<Vec<arrow::record_batch::RecordBatch>> { /* ... */ }
```

#### Function `build_checked_file_list`

Recursively builds a list of files in a directory with a given extension

```rust
pub fn build_checked_file_list(dir: &str, ext: &str) -> datafusion_common::Result<Vec<String>> { /* ... */ }
```

#### Function `build_file_list`

Recursively builds a list of files in a directory with a given extension

```rust
pub fn build_file_list(dir: &str, ext: &str) -> datafusion_common::Result<Vec<String>> { /* ... */ }
```

#### Function `compute_record_batch_statistics`

Computes the statistics for an in-memory RecordBatch

Only computes statistics that are in arrows metadata (num rows, byte size and nulls)
and does not apply any kernel on the actual data.

```rust
pub fn compute_record_batch_statistics(batches: &[Vec<arrow::record_batch::RecordBatch>], schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> crate::Statistics { /* ... */ }
```

#### Function `can_project`

Checks if the given projection is valid for the given schema.

```rust
pub fn can_project(schema: &arrow::datatypes::SchemaRef, projection: Option<&Vec<usize>>) -> datafusion_common::Result<()> { /* ... */ }
```

## Module `display`

Implementation of physical plan display. See
[`crate::displayable`] for examples of how to format

```rust
pub mod display { /* ... */ }
```

### Types

#### Enum `DisplayFormatType`

Options for controlling how each [`ExecutionPlan`] should format itself

```rust
pub enum DisplayFormatType {
    Default,
    Verbose,
    TreeRender,
}
```

##### Variants

###### `Default`

Default, compact format. Example: `FilterExec: c12 < 10.0`

This format is designed to provide a detailed textual description
of all parts of the plan.

###### `Verbose`

Verbose, showing all available details.

This form is even more detailed than [`Self::Default`]

###### `TreeRender`

TreeRender, displayed in the `tree` explain type.

This format is inspired by DuckDB's explain plans. The information
presented should be "user friendly", and contain only the most relevant
information for understanding a plan. It should NOT contain the same level
of detail information as the  [`Self::Default`] format.

In this mode, each line has one of two formats:

1. A string without a `=`, which is printed in its own line

2. A string with a `=` that is treated as a `key=value pair`. Everything
   before the first `=` is treated as the key, and everything after the
   first `=` is treated as the value.

For example, if the output of `TreeRender` is this:
```text
Parquet
partition_sizes=[1]
```

It is rendered in the center of a box in the following way:

```text
┌───────────────────────────┐
│       DataSourceExec      │
│    --------------------   │
│    partition_sizes: [1]   │
│          Parquet          │
└───────────────────────────┘
 ```

##### Implementations

###### Trait Implementations

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Copy**
- **Unpin**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DisplayFormatType) -> bool { /* ... */ }
    ```

- **Send**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
- **Freeze**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DisplayFormatType { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `DisplayableExecutionPlan`

Wraps an `ExecutionPlan` with various methods for formatting


# Example
```
# use std::sync::Arc;
# use arrow::datatypes::{Field, Schema, DataType};
# use datafusion_expr::Operator;
# use datafusion_physical_expr::expressions::{binary, col, lit};
# use datafusion_physical_plan::{displayable, ExecutionPlan};
# use datafusion_physical_plan::empty::EmptyExec;
# use datafusion_physical_plan::filter::FilterExec;
# let schema = Schema::new(vec![Field::new("i", DataType::Int32, false)]);
# let plan = EmptyExec::new(Arc::new(schema));
# let i = col("i", &plan.schema()).unwrap();
# let predicate = binary(i, Operator::Eq, lit(1), &plan.schema()).unwrap();
# let plan: Arc<dyn ExecutionPlan> = Arc::new(FilterExec::try_new(predicate, Arc::new(plan)).unwrap());
// Get a one line description (Displayable)
let display_plan = displayable(plan.as_ref());

// you can use the returned objects to format plans
// where you can use `Display` such as  format! or println!
assert_eq!(
   &format!("The plan is: {}", display_plan.one_line()),
  "The plan is: FilterExec: i@0 = 1\n"
);
// You can also print out the plan and its children in indented mode
assert_eq!(display_plan.indent(false).to_string(),
  "FilterExec: i@0 = 1\
  \n  EmptyExec\
  \n"
);
```

```rust
pub struct DisplayableExecutionPlan<''a> {
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
  pub fn new(inner: &''a dyn ExecutionPlan) -> Self { /* ... */ }
  ```
  Create a wrapper around an [`ExecutionPlan`] which can be

- ```rust
  pub fn with_metrics(inner: &''a dyn ExecutionPlan) -> Self { /* ... */ }
  ```
  Create a wrapper around an [`ExecutionPlan`] which can be

- ```rust
  pub fn with_full_metrics(inner: &''a dyn ExecutionPlan) -> Self { /* ... */ }
  ```
  Create a wrapper around an [`ExecutionPlan`] which can be

- ```rust
  pub fn set_show_schema(self: Self, show_schema: bool) -> Self { /* ... */ }
  ```
  Enable display of schema

- ```rust
  pub fn set_show_statistics(self: Self, show_statistics: bool) -> Self { /* ... */ }
  ```
  Enable display of statistics

- ```rust
  pub fn indent(self: &Self, verbose: bool) -> impl fmt::Display + ''a { /* ... */ }
  ```
  Return a `format`able structure that produces a single line

- ```rust
  pub fn graphviz(self: &Self) -> impl fmt::Display + ''a { /* ... */ }
  ```
  Returns a `format`able structure that produces graphviz format for execution plan, which can

- ```rust
  pub fn tree_render(self: &Self) -> impl fmt::Display + ''a { /* ... */ }
  ```
  Formats the plan using a ASCII art like tree

- ```rust
  pub fn one_line(self: &Self) -> impl fmt::Display + ''a { /* ... */ }
  ```
  Return a single-line summary of the root of the plan

- ```rust
  pub fn to_stringified(self: &Self, verbose: bool, plan_type: PlanType, explain_format: DisplayFormatType) -> StringifiedPlan { /* ... */ }
  ```

###### Trait Implementations

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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DisplayableExecutionPlan<''a> { /* ... */ }
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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Send**
- **Unpin**
- **RefUnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
#### Struct `DefaultDisplay`

A new type wrapper to display `T` implementing`DisplayAs` using the `Default` mode

```rust
pub struct DefaultDisplay<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **IntoEither**
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

- **Unpin**
- **MaybeSendSync**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

#### Struct `VerboseDisplay`

A new type wrapper to display `T` implementing `DisplayAs` using the `Verbose` mode

```rust
pub struct VerboseDisplay<T>(pub T);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Allocation**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `ProjectSchemaDisplay`

A wrapper to customize partitioned file display

```rust
pub struct ProjectSchemaDisplay<''a>(pub &''a arrow::datatypes::SchemaRef);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a arrow::datatypes::SchemaRef` |  |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **IntoEither**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Allocation**
- **Sync**
- **Send**
### Traits

#### Trait `DisplayAs`

Trait for types which could have additional details when formatted in `Verbose` mode

```rust
pub trait DisplayAs {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `fmt_as`: Format according to `DisplayFormatType`, used when verbose representation looks

##### Implementations

This trait is implemented for the following types:

- `AggregateExec`
- `AnalyzeExec`
- `CoalesceBatchesExec`
- `CoalescePartitionsExec`
- `EmptyExec`
- `ExplainExec`
- `FilterExec`
- `CrossJoinExec`
- `HashJoinExec`
- `NestedLoopJoinExec`
- `SortMergeJoinExec`
- `SymmetricHashJoinExec`
- `GlobalLimitExec`
- `LocalLimitExec`
- `LazyMemoryExec`
- `PlaceholderRowExec`
- `ProjectionExec`
- `RecursiveQueryExec`
- `RepartitionExec`
- `PartialSortExec`
- `SortExec`
- `SortPreservingMergeExec`
- `StreamingTableExec`
- `UnionExec`
- `InterleaveExec`
- `UnnestExec`
- `ValuesExec`
- `BoundedWindowAggExec`
- `WindowAggExec`
- `WorkTableExec`

### Functions

#### Function `display_orderings`

```rust
pub fn display_orderings(f: &mut std::fmt::Formatter<''_>, orderings: &[datafusion_physical_expr::LexOrdering]) -> fmt::Result { /* ... */ }
```

## Module `empty`

EmptyRelation with produce_one_row=false execution plan

```rust
pub mod empty { /* ... */ }
```

### Types

#### Struct `EmptyExec`

Execution plan for empty relation with produce_one_row=false

```rust
pub struct EmptyExec {
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
  pub fn new(schema: SchemaRef) -> Self { /* ... */ }
  ```
  Create a new EmptyExec

- ```rust
  pub fn with_partitions(self: Self, partitions: usize) -> Self { /* ... */ }
  ```
  Create a new EmptyExec with specified partition number

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> EmptyExec { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

## Module `execution_plan`

```rust
pub mod execution_plan { /* ... */ }
```

### Types

#### Enum `InvariantLevel`

[`ExecutionPlan`] Invariant Level

What set of assertions ([Invariant]s)  holds for a particular `ExecutionPlan`

[Invariant]: https://en.wikipedia.org/wiki/Invariant_(mathematics)#Invariants_in_computer_science

```rust
pub enum InvariantLevel {
    Always,
    Executable,
}
```

##### Variants

###### `Always`

Invariants that are always true for the [`ExecutionPlan`] node
such as the number of expected children.

###### `Executable`

Invariants that must hold true for the [`ExecutionPlan`] node
to be "executable", such as ordering and/or distribution requirements
being fulfilled.

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
- **ErasedDestructor**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Copy**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InvariantLevel { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Allocation**
- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Enum `Boundedness`

Represents whether a stream of data **generated** by an operator is bounded (finite)
or unbounded (infinite).

This is used to determine whether an execution plan will eventually complete
processing all its data (bounded) or could potentially run forever (unbounded).

For unbounded streams, it also tracks whether the operator requires finite memory
to process the stream or if memory usage could grow unbounded.

Boundedness of the output stream is based on the the boundedness of the input stream and the nature of
the operator. For example, limit or topk with fetch operator can convert an unbounded stream to a bounded stream.

```rust
pub enum Boundedness {
    Bounded,
    Unbounded {
        requires_infinite_memory: bool,
    },
}
```

##### Variants

###### `Bounded`

The data stream is bounded (finite) and will eventually complete

###### `Unbounded`

The data stream is unbounded (infinite) and could run forever

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `requires_infinite_memory` | `bool` | Whether this operator requires infinite memory to process the unbounded stream.<br>If false, the operator can process an infinite stream with bounded memory.<br>If true, memory usage may grow unbounded while processing the stream.<br><br>For example, `Median` requires infinite memory to compute the median of an unbounded stream.<br>`Min/Max` requires infinite memory if the stream is unordered, but can be computed with bounded memory if the stream is ordered. |

##### Implementations

###### Methods

- ```rust
  pub fn is_unbounded(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **UnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Copy**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Boundedness) -> bool { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **IntoEither**
- **Unpin**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Boundedness { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `EmissionType`

Represents how an operator emits its output records.

This is used to determine whether an operator emits records incrementally as they arrive,
only emits a final result at the end, or can do both. Note that it generates the output -- record batch with `batch_size` rows
but it may still buffer data internally until it has enough data to emit a record batch or the source is exhausted.

For example, in the following plan:
```text
  SortExec [EmissionType::Final]
    |_ on: [col1 ASC]
    FilterExec [EmissionType::Incremental]
      |_ pred: col2 > 100
      DataSourceExec [EmissionType::Incremental]
        |_ file: "data.csv"
```
- DataSourceExec emits records incrementally as it reads from the file
- FilterExec processes and emits filtered records incrementally as they arrive
- SortExec must wait for all input records before it can emit the sorted result,
  since it needs to see all values to determine their final order

Left joins can emit both incrementally and finally:
- Incrementally emit matches as they are found
- Finally emit non-matches after all input is processed

```rust
pub enum EmissionType {
    Incremental,
    Final,
    Both,
}
```

##### Variants

###### `Incremental`

Records are emitted incrementally as they arrive and are processed

###### `Final`

Records are only emitted once all input has been processed

###### `Both`

Records can be emitted both incrementally and as a final result

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Allocation**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> EmissionType { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &EmissionType) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
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
- **IntoEither**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
#### Struct `PlanProperties`

Stores certain, often expensive to compute, plan properties used in query
optimization.

These properties are stored a single structure to permit this information to
be computed once and then those cached results used multiple times without
recomputation (aka a cache)

```rust
pub struct PlanProperties {
    pub eq_properties: datafusion_physical_expr::EquivalenceProperties,
    pub partitioning: Partitioning,
    pub emission_type: EmissionType,
    pub boundedness: Boundedness,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `eq_properties` | `datafusion_physical_expr::EquivalenceProperties` | See [ExecutionPlanProperties::equivalence_properties] |
| `partitioning` | `Partitioning` | See [ExecutionPlanProperties::output_partitioning] |
| `emission_type` | `EmissionType` | See [ExecutionPlanProperties::pipeline_behavior] |
| `boundedness` | `Boundedness` | See [ExecutionPlanProperties::boundedness] |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(eq_properties: EquivalenceProperties, partitioning: Partitioning, emission_type: EmissionType, boundedness: Boundedness) -> Self { /* ... */ }
  ```
  Construct a new `PlanPropertiesCache` from the

- ```rust
  pub fn with_partitioning(self: Self, partitioning: Partitioning) -> Self { /* ... */ }
  ```
  Overwrite output partitioning with its new value.

- ```rust
  pub fn with_eq_properties(self: Self, eq_properties: EquivalenceProperties) -> Self { /* ... */ }
  ```
  Overwrite equivalence properties with its new value.

- ```rust
  pub fn with_boundedness(self: Self, boundedness: Boundedness) -> Self { /* ... */ }
  ```
  Overwrite boundedness with its new value.

- ```rust
  pub fn with_emission_type(self: Self, emission_type: EmissionType) -> Self { /* ... */ }
  ```
  Overwrite emission type with its new value.

- ```rust
  pub fn with_constraints(self: Self, constraints: Constraints) -> Self { /* ... */ }
  ```
  Overwrite constraints with its new value.

- ```rust
  pub fn equivalence_properties(self: &Self) -> &EquivalenceProperties { /* ... */ }
  ```

- ```rust
  pub fn output_partitioning(self: &Self) -> &Partitioning { /* ... */ }
  ```

- ```rust
  pub fn output_ordering(self: &Self) -> Option<&LexOrdering> { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PlanProperties { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
#### Enum `CardinalityEffect`

Indicates the effect an execution plan operator will have on the cardinality
of its input stream

```rust
pub enum CardinalityEffect {
    Unknown,
    Equal,
    LowerEqual,
    GreaterEqual,
}
```

##### Variants

###### `Unknown`

Unknown effect. This is the default

###### `Equal`

The operator is guaranteed to produce exactly one row for
each input row

###### `LowerEqual`

The operator may produce fewer output rows than it receives input rows

###### `GreaterEqual`

The operator may produce more output rows than it receives input rows

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Allocation**
- **Sync**
- **Freeze**
- **Send**
- **RefUnwindSafe**
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

### Traits

#### Trait `ExecutionPlan`

Represent nodes in the DataFusion Physical Plan.

Calling [`execute`] produces an `async` [`SendableRecordBatchStream`] of
[`RecordBatch`] that incrementally computes a partition of the
`ExecutionPlan`'s output from its input. See [`Partitioning`] for more
details on partitioning.

Methods such as [`Self::schema`] and [`Self::properties`] communicate
properties of the output to the DataFusion optimizer, and methods such as
[`required_input_distribution`] and [`required_input_ordering`] express
requirements of the `ExecutionPlan` from its input.

[`ExecutionPlan`] can be displayed in a simplified form using the
return value from [`displayable`] in addition to the (normally
quite verbose) `Debug` output.

[`execute`]: ExecutionPlan::execute
[`required_input_distribution`]: ExecutionPlan::required_input_distribution
[`required_input_ordering`]: ExecutionPlan::required_input_ordering

```rust
pub trait ExecutionPlan: Debug + DisplayAs + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `name`: Short name for the ExecutionPlan, such as 'DataSourceExec'.
- `as_any`: Returns the execution plan as [`Any`] so that it can be
- `properties`: Return properties of the output of the `ExecutionPlan`, such as output
- `children`: Get a list of children `ExecutionPlan`s that act as inputs to this plan.
- `with_new_children`: Returns a new `ExecutionPlan` where all existing children were replaced
- `execute`: Begin execution of `partition`, returning a [`Stream`] of

##### Provided Methods

- ```rust
  fn static_name() -> &''static str
where
    Self: Sized { /* ... */ }
  ```
  Short name for the ExecutionPlan, such as 'DataSourceExec'.

- ```rust
  fn schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  Get the schema for this execution plan

- ```rust
  fn check_invariants(self: &Self, _check: InvariantLevel) -> Result<()> { /* ... */ }
  ```
  Returns an error if this individual node does not conform to its invariants.

- ```rust
  fn required_input_distribution(self: &Self) -> Vec<Distribution> { /* ... */ }
  ```
  Specifies the data distribution requirements for all the

- ```rust
  fn required_input_ordering(self: &Self) -> Vec<Option<LexRequirement>> { /* ... */ }
  ```
  Specifies the ordering required for all of the children of this

- ```rust
  fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
  ```
  Returns `false` if this `ExecutionPlan`'s implementation may reorder

- ```rust
  fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
  ```
  Specifies whether the `ExecutionPlan` benefits from increased

- ```rust
  fn repartitioned(self: &Self, _target_partitions: usize, _config: &ConfigOptions) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
  ```
  If supported, attempt to increase the partitioning of this `ExecutionPlan` to

- ```rust
  fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
  ```
  Return a snapshot of the set of [`Metric`]s for this

- ```rust
  fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
  ```
  Returns statistics for this `ExecutionPlan` node. If statistics are not

- ```rust
  fn supports_limit_pushdown(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` if a limit can be safely pushed down through this

- ```rust
  fn with_fetch(self: &Self, _limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
  ```
  Returns a fetching variant of this `ExecutionPlan` node, if it supports

- ```rust
  fn fetch(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Gets the fetch count for the operator, `None` means there is no fetch.

- ```rust
  fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
  ```
  Gets the effect on cardinality, if known

- ```rust
  fn try_swapping_with_projection(self: &Self, _projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
  ```
  Attempts to push down the given projection into the input of this `ExecutionPlan`.

##### Implementations

This trait is implemented for the following types:

- `AggregateExec`
- `AnalyzeExec`
- `CoalesceBatchesExec`
- `CoalescePartitionsExec`
- `EmptyExec`
- `ExplainExec`
- `FilterExec`
- `CrossJoinExec`
- `HashJoinExec`
- `NestedLoopJoinExec`
- `SortMergeJoinExec`
- `SymmetricHashJoinExec`
- `GlobalLimitExec`
- `LocalLimitExec`
- `LazyMemoryExec`
- `PlaceholderRowExec`
- `ProjectionExec`
- `RecursiveQueryExec`
- `RepartitionExec`
- `PartialSortExec`
- `SortExec`
- `SortPreservingMergeExec`
- `StreamingTableExec`
- `UnionExec`
- `InterleaveExec`
- `UnnestExec`
- `ValuesExec`
- `BoundedWindowAggExec`
- `WindowAggExec`
- `WorkTableExec`

#### Trait `ExecutionPlanProperties`

Extension trait provides an easy API to fetch various properties of
[`ExecutionPlan`] objects based on [`ExecutionPlan::properties`].

```rust
pub trait ExecutionPlanProperties {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `output_partitioning`: Specifies how the output of this `ExecutionPlan` is split into
- `output_ordering`: If the output of this `ExecutionPlan` within each partition is sorted,
- `boundedness`: Boundedness information of the stream corresponding to this `ExecutionPlan`.
- `pipeline_behavior`: Indicates how the stream of this `ExecutionPlan` emits its results.
- `equivalence_properties`: Get the [`EquivalenceProperties`] within the plan.

##### Implementations

This trait is implemented for the following types:

- `std::sync::Arc<dyn ExecutionPlan>`
- `&dyn ExecutionPlan`

### Functions

#### Function `need_data_exchange`

Indicate whether a data exchange is needed for the input of `plan`, which will be very helpful
especially for the distributed engine to judge whether need to deal with shuffling.
Currently there are 3 kinds of execution plan which needs data exchange
    1. RepartitionExec for changing the partition number between two `ExecutionPlan`s
    2. CoalescePartitionsExec for collapsing all of the partitions into one without ordering guarantee
    3. SortPreservingMergeExec for collapsing all of the sorted partitions into one with ordering guarantee

```rust
pub fn need_data_exchange(plan: std::sync::Arc<dyn ExecutionPlan>) -> bool { /* ... */ }
```

#### Function `with_new_children_if_necessary`

Returns a copy of this plan if we change any child according to the pointer comparison.
The size of `children` must be equal to the size of `ExecutionPlan::children()`.

```rust
pub fn with_new_children_if_necessary(plan: std::sync::Arc<dyn ExecutionPlan>, children: Vec<std::sync::Arc<dyn ExecutionPlan>>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>> { /* ... */ }
```

#### Function `displayable`

Return a [`DisplayableExecutionPlan`] wrapper around an
[`ExecutionPlan`] which can be displayed in various easier to
understand ways.

See examples on [`DisplayableExecutionPlan`]

```rust
pub fn displayable(plan: &dyn ExecutionPlan) -> crate::display::DisplayableExecutionPlan<''_> { /* ... */ }
```

#### Function `collect`

Execute the [ExecutionPlan] and collect the results in memory

```rust
pub async fn collect(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<Vec<arrow::array::RecordBatch>> { /* ... */ }
```

#### Function `execute_stream`

Execute the [ExecutionPlan] and return a single stream of `RecordBatch`es.

See [collect] to buffer the `RecordBatch`es in memory.

# Aborting Execution

Dropping the stream will abort the execution of the query, and free up
any allocated resources

```rust
pub fn execute_stream(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<SendableRecordBatchStream> { /* ... */ }
```

#### Function `collect_partitioned`

Execute the [ExecutionPlan] and collect the results in memory

```rust
pub async fn collect_partitioned(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<Vec<Vec<arrow::array::RecordBatch>>> { /* ... */ }
```

#### Function `execute_stream_partitioned`

Execute the [ExecutionPlan] and return a vec with one stream per output
partition

# Aborting Execution

Dropping the stream will abort the execution of the query, and free up
any allocated resources

```rust
pub fn execute_stream_partitioned(plan: std::sync::Arc<dyn ExecutionPlan>, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<Vec<SendableRecordBatchStream>> { /* ... */ }
```

#### Function `execute_input_stream`

Executes an input stream and ensures that the resulting stream adheres to
the `not null` constraints specified in the `sink_schema`.

# Arguments

* `input` - An execution plan
* `sink_schema` - The schema to be applied to the output stream
* `partition` - The partition index to be executed
* `context` - The task context

# Returns

* `Result<SendableRecordBatchStream>` - A stream of `RecordBatch`es if successful

This function first executes the given input plan for the specified partition
and context. It then checks if there are any columns in the input that might
violate the `not null` constraints specified in the `sink_schema`. If there are
such columns, it wraps the resulting stream to enforce the `not null` constraints
by invoking the [`check_not_null_constraints`] function on each batch of the stream.

```rust
pub fn execute_input_stream(input: std::sync::Arc<dyn ExecutionPlan>, sink_schema: arrow::datatypes::SchemaRef, partition: usize, context: std::sync::Arc<datafusion_execution::TaskContext>) -> datafusion_common::Result<SendableRecordBatchStream> { /* ... */ }
```

#### Function `check_not_null_constraints`

Checks a `RecordBatch` for `not null` constraints on specified columns.

# Arguments

* `batch` - The `RecordBatch` to be checked
* `column_indices` - A vector of column indices that should be checked for
  `not null` constraints.

# Returns

* `Result<RecordBatch>` - The original `RecordBatch` if all constraints are met

This function iterates over the specified column indices and ensures that none
of the columns contain null values. If any column contains null values, an error
is returned.

```rust
pub fn check_not_null_constraints(batch: arrow::array::RecordBatch, column_indices: &Vec<usize>) -> datafusion_common::Result<arrow::array::RecordBatch> { /* ... */ }
```

#### Function `get_plan_string`

Utility function yielding a string representation of the given [`ExecutionPlan`].

```rust
pub fn get_plan_string(plan: &std::sync::Arc<dyn ExecutionPlan>) -> Vec<String> { /* ... */ }
```

### Re-exports

#### Re-export `DefaultDisplay`

```rust
pub use crate::display::DefaultDisplay;
```

#### Re-export `DisplayAs`

```rust
pub use crate::display::DisplayAs;
```

#### Re-export `DisplayFormatType`

```rust
pub use crate::display::DisplayFormatType;
```

#### Re-export `VerboseDisplay`

```rust
pub use crate::display::VerboseDisplay;
```

#### Re-export `Metric`

```rust
pub use crate::metrics::Metric;
```

#### Re-export `InputOrderMode`

```rust
pub use crate::ordering::InputOrderMode;
```

#### Re-export `EmptyRecordBatchStream`

```rust
pub use crate::stream::EmptyRecordBatchStream;
```

#### Re-export `hash_utils`

```rust
pub use datafusion_common::hash_utils;
```

#### Re-export `project_schema`

```rust
pub use datafusion_common::utils::project_schema;
```

#### Re-export `internal_err`

```rust
pub use datafusion_common::internal_err;
```

#### Re-export `ColumnStatistics`

```rust
pub use datafusion_common::ColumnStatistics;
```

#### Re-export `Statistics`

```rust
pub use datafusion_common::Statistics;
```

#### Re-export `RecordBatchStream`

```rust
pub use datafusion_execution::RecordBatchStream;
```

#### Re-export `SendableRecordBatchStream`

```rust
pub use datafusion_execution::SendableRecordBatchStream;
```

#### Re-export `Accumulator`

```rust
pub use datafusion_expr::Accumulator;
```

#### Re-export `ColumnarValue`

```rust
pub use datafusion_expr::ColumnarValue;
```

#### Re-export `WindowExpr`

```rust
pub use datafusion_physical_expr::window::WindowExpr;
```

#### Re-export `expressions`

```rust
pub use datafusion_physical_expr::expressions;
```

#### Re-export `Distribution`

```rust
pub use datafusion_physical_expr::Distribution;
```

#### Re-export `Partitioning`

```rust
pub use datafusion_physical_expr::Partitioning;
```

#### Re-export `PhysicalExpr`

```rust
pub use datafusion_physical_expr::PhysicalExpr;
```

## Module `explain`

Defines the EXPLAIN operator

```rust
pub mod explain { /* ... */ }
```

### Types

#### Struct `ExplainExec`

Explain execution plan operator. This operator contains the string
values of the various plans it has when it is created, and passes
them to its output.

```rust
pub struct ExplainExec {
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
  pub fn new(schema: SchemaRef, stringified_plans: Vec<StringifiedPlan>, verbose: bool) -> Self { /* ... */ }
  ```
  Create a new ExplainExec

- ```rust
  pub fn stringified_plans(self: &Self) -> &[StringifiedPlan] { /* ... */ }
  ```
  The strings to be printed

- ```rust
  pub fn verbose(self: &Self) -> bool { /* ... */ }
  ```
  Access to verbose

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExplainExec { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

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

- **IntoEither**
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
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `filter`

```rust
pub mod filter { /* ... */ }
```

### Types

#### Struct `FilterExec`

FilterExec evaluates a boolean predicate against all input batches to determine which rows to
include in its output batches.

```rust
pub struct FilterExec {
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
  pub fn try_new(predicate: Arc<dyn PhysicalExpr>, input: Arc<dyn ExecutionPlan>) -> Result<Self> { /* ... */ }
  ```
  Create a FilterExec on an input

- ```rust
  pub fn with_default_selectivity(self: Self, default_selectivity: u8) -> Result<Self, DataFusionError> { /* ... */ }
  ```

- ```rust
  pub fn with_projection(self: &Self, projection: Option<Vec<usize>>) -> Result<Self> { /* ... */ }
  ```
  Return new instance of [FilterExec] with the given projection.

- ```rust
  pub fn predicate(self: &Self) -> &Arc<dyn PhysicalExpr> { /* ... */ }
  ```
  The expression to filter on. This expression must evaluate to a boolean value.

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  The input plan

- ```rust
  pub fn default_selectivity(self: &Self) -> u8 { /* ... */ }
  ```
  The default selectivity

- ```rust
  pub fn projection(self: &Self) -> Option<&Vec<usize>> { /* ... */ }
  ```
  Projection

###### Trait Implementations

- **ErasedDestructor**
- **EmbeddedProjection**
  - ```rust
    fn with_projection(self: &Self, projection: Option<Vec<usize>>) -> Result<Self> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> FilterExec { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

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

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```
    The output statistics of a filtering operation can be estimated if the

  - ```rust
    fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```
    Tries to swap `projection` with its input (`filter`). If possible, performs

- **Sync**
#### Type Alias `PhysicalExprPairRef`

Pair of `Arc<dyn PhysicalExpr>`s

```rust
pub type PhysicalExprPairRef<''a> = (&''a std::sync::Arc<dyn PhysicalExpr>, &''a std::sync::Arc<dyn PhysicalExpr>);
```

#### Type Alias `EqualAndNonEqual`

The equals Column-Pairs and Non-equals Column-Pairs in the Predicates

```rust
pub type EqualAndNonEqual<''a> = (Vec<PhysicalExprPairRef<''a>>, Vec<PhysicalExprPairRef<''a>>);
```

### Functions

#### Function `batch_filter`

```rust
pub fn batch_filter(batch: &arrow::record_batch::RecordBatch, predicate: &std::sync::Arc<dyn PhysicalExpr>) -> datafusion_common::Result<arrow::record_batch::RecordBatch> { /* ... */ }
```

## Module `joins`

DataFusion Join implementations

```rust
pub mod joins { /* ... */ }
```

### Modules

## Module `utils`

Join related functionality used both on logical and physical plans

```rust
pub mod utils { /* ... */ }
```

### Types

#### Struct `ColumnIndex`

Information about the index and placement (left or right) of the columns

```rust
pub struct ColumnIndex {
    pub index: usize,
    pub side: datafusion_common::JoinSide,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `index` | `usize` | Index of the column |
| `side` | `datafusion_common::JoinSide` | Whether the column is at the left or right side |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Unpin**
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ColumnIndex) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **StructuralPartialEq**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Allocation**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ColumnIndex { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
#### Enum `StatefulStreamResult`

Represents the result of a stateful operation.

This enumeration indicates whether the state produced a result that is
ready for use (`Ready`) or if the operation requires continuation (`Continue`).

Variants:
- `Ready(T)`: Indicates that the operation is complete with a result of type `T`.
- `Continue`: Indicates that the operation is not yet complete and requires further
  processing or more data. When this variant is returned, it typically means that the
  current invocation of the state did not produce a final result, and the operation
  should be invoked again later with more data and possibly with a different state.

```rust
pub enum StatefulStreamResult<T> {
    Ready(T),
    Continue,
}
```

##### Variants

###### `Ready`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

###### `Continue`

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Allocation**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Functions

#### Function `check_join_is_valid`

Checks whether the schemas "left" and "right" and columns "on" represent a valid join.
They are valid whenever their columns' intersection equals the set `on`

```rust
pub fn check_join_is_valid(left: &arrow::datatypes::Schema, right: &arrow::datatypes::Schema, on: JoinOnRef<''_>) -> datafusion_common::Result<()> { /* ... */ }
```

#### Function `adjust_right_output_partitioning`

Adjust the right out partitioning to new Column Index

```rust
pub fn adjust_right_output_partitioning(right_partitioning: &crate::Partitioning, left_columns_len: usize) -> crate::Partitioning { /* ... */ }
```

#### Function `calculate_join_output_ordering`

Calculate the output ordering of a given join operation.

```rust
pub fn calculate_join_output_ordering(left_ordering: &datafusion_physical_expr::LexOrdering, right_ordering: &datafusion_physical_expr::LexOrdering, join_type: datafusion_common::JoinType, on_columns: &[(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)], left_columns_len: usize, maintains_input_order: &[bool], probe_side: Option<datafusion_common::JoinSide>) -> Option<datafusion_physical_expr::LexOrdering> { /* ... */ }
```

#### Function `build_join_schema`

Creates a schema for a join operation.
The fields from the left side are first

```rust
pub fn build_join_schema(left: &arrow::datatypes::Schema, right: &arrow::datatypes::Schema, join_type: &datafusion_common::JoinType) -> (arrow::datatypes::Schema, Vec<ColumnIndex>) { /* ... */ }
```

### Re-exports

#### Re-export `JoinFilter`

```rust
pub use super::join_filter::JoinFilter;
```

#### Re-export `JoinHashMap`

```rust
pub use super::join_hash_map::JoinHashMap;
```

#### Re-export `JoinHashMapType`

```rust
pub use super::join_hash_map::JoinHashMapType;
```

#### Re-export `JoinOn`

```rust
pub use crate::joins::JoinOn;
```

#### Re-export `JoinOnRef`

```rust
pub use crate::joins::JoinOnRef;
```

### Types

#### Type Alias `JoinOn`

The on clause of the join, as vector of (left, right) columns.

```rust
pub type JoinOn = Vec<(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)>;
```

#### Type Alias `JoinOnRef`

Reference for JoinOn.

```rust
pub type JoinOnRef<''a> = &''a [(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)];
```

#### Enum `PartitionMode`

Hash join Partitioning mode

```rust
pub enum PartitionMode {
    Partitioned,
    CollectLeft,
    Auto,
}
```

##### Variants

###### `Partitioned`

Left/right children are partitioned using the left and right keys

###### `CollectLeft`

Left side will collected into one partition

###### `Auto`

DataFusion optimizer decides which PartitionMode
mode(Partitioned/CollectLeft) is optimal based on statistics. It will
also consider swapping the left and right inputs for the Join

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Allocation**
- **Unpin**
- **IntoEither**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PartitionMode) -> bool { /* ... */ }
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PartitionMode { /* ... */ }
    ```

- **Send**
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

- **Eq**
- **Copy**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
#### Enum `StreamJoinPartitionMode`

Partitioning mode to use for symmetric hash join

```rust
pub enum StreamJoinPartitionMode {
    Partitioned,
    SinglePartition,
}
```

##### Variants

###### `Partitioned`

Left/right children are partitioned using the left and right keys

###### `SinglePartition`

Both sides will collected into one partition

##### Implementations

###### Trait Implementations

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &StreamJoinPartitionMode) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> StreamJoinPartitionMode { /* ... */ }
    ```

- **Eq**
- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Allocation**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **StructuralPartialEq**
- **Unpin**
- **RefUnwindSafe**
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
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

### Re-exports

#### Re-export `CrossJoinExec`

```rust
pub use cross_join::CrossJoinExec;
```

#### Re-export `HashJoinExec`

```rust
pub use hash_join::HashJoinExec;
```

#### Re-export `NestedLoopJoinExec`

```rust
pub use nested_loop_join::NestedLoopJoinExec;
```

#### Re-export `SortMergeJoinExec`

```rust
pub use sort_merge_join::SortMergeJoinExec;
```

#### Re-export `SymmetricHashJoinExec`

```rust
pub use symmetric_hash_join::SymmetricHashJoinExec;
```

## Module `limit`

Defines the LIMIT plan

```rust
pub mod limit { /* ... */ }
```

### Types

#### Struct `GlobalLimitExec`

Limit execution plan

```rust
pub struct GlobalLimitExec {
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
  pub fn new(input: Arc<dyn ExecutionPlan>, skip: usize, fetch: Option<usize>) -> Self { /* ... */ }
  ```
  Create a new GlobalLimitExec

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input execution plan

- ```rust
  pub fn skip(self: &Self) -> usize { /* ... */ }
  ```
  Number of rows to skip before fetch

- ```rust
  pub fn fetch(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Maximum number of rows to fetch

###### Trait Implementations

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
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
    fn required_input_distribution(self: &Self) -> Vec<Distribution> { /* ... */ }
    ```

  - ```rust
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn supports_limit_pushdown(self: &Self) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> GlobalLimitExec { /* ... */ }
    ```

- **Sync**
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

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

#### Struct `LocalLimitExec`

LocalLimitExec applies a limit to a single partition

```rust
pub struct LocalLimitExec {
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
  pub fn new(input: Arc<dyn ExecutionPlan>, fetch: usize) -> Self { /* ... */ }
  ```
  Create a new LocalLimitExec partition

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input execution plan

- ```rust
  pub fn fetch(self: &Self) -> usize { /* ... */ }
  ```
  Maximum number of rows to fetch

###### Trait Implementations

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
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
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn supports_limit_pushdown(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **IntoEither**
#### Struct `LimitStream`

A Limit stream skips `skip` rows, and then fetch up to `fetch` rows.

```rust
pub struct LimitStream {
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
  pub fn new(input: SendableRecordBatchStream, skip: usize, fetch: Option<usize>, baseline_metrics: BaselineMetrics) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **RecordBatchStream**
  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```
    Get the schema

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
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

- **Sync**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **StreamExt**
- **TryStreamExt**
- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
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

- **Unpin**
## Module `memory`

Execution plan for reading in-memory batches of data

```rust
pub mod memory { /* ... */ }
```

### Types

#### Struct `MemoryStream`

Iterator over batches

```rust
pub struct MemoryStream {
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
  pub fn try_new(data: Vec<RecordBatch>, schema: SchemaRef, projection: Option<Vec<usize>>) -> Result<Self> { /* ... */ }
  ```
  Create an iterator for a vector of record batches

- ```rust
  pub fn with_reservation(self: Self, reservation: MemoryReservation) -> Self { /* ... */ }
  ```
  Set the memory reservation for the data

- ```rust
  pub fn with_fetch(self: Self, fetch: Option<usize>) -> Self { /* ... */ }
  ```
  Set the number of rows to produce

###### Trait Implementations

- **RefUnwindSafe**
- **StreamExt**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryStreamExt**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: std::pin::Pin<&mut Self>, _: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RecordBatchStream**
  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```
    Get the schema

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **MaybeSendSync**
#### Struct `LazyMemoryExec`

Execution plan for lazy in-memory batches of data

This plan generates output batches lazily, it doesn't have to buffer all batches
in memory up front (compared to `MemorySourceConfig`), thus consuming constant memory.

```rust
pub struct LazyMemoryExec {
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
  pub fn try_new(schema: SchemaRef, generators: Vec<Arc<RwLock<dyn LazyBatchGenerator>>>) -> Result<Self> { /* ... */ }
  ```
  Create a new lazy memory execution plan

###### Trait Implementations

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
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
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

  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

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
    fn execute(self: &Self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **UnwindSafe**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `LazyMemoryStream`

Stream that generates record batches on demand

```rust
pub struct LazyMemoryStream {
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

- **Sync**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **TryStreamExt**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **RecordBatchStream**
  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **UnwindSafe**
- **Send**
- **Freeze**
- **ErasedDestructor**
- **StreamExt**
- **Stream**
  - ```rust
    fn poll_next(self: std::pin::Pin<&mut Self>, _: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Traits

#### Trait `LazyBatchGenerator`

```rust
pub trait LazyBatchGenerator: Send + Sync + fmt::Debug + fmt::Display {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `generate_next_batch`: Generate the next batch, return `None` when no more batches are available

## Module `metrics`

Metrics for recording information about execution

```rust
pub mod metrics { /* ... */ }
```

### Types

#### Struct `Metric`

Something that tracks a value of interest (metric) of a DataFusion
[`ExecutionPlan`] execution.

Typically [`Metric`]s are not created directly, but instead
are created using [`MetricBuilder`] or methods on
[`ExecutionPlanMetricsSet`].

```
 use datafusion_physical_plan::metrics::*;

 let metrics = ExecutionPlanMetricsSet::new();
 assert!(metrics.clone_inner().output_rows().is_none());

 // Create a counter to increment using the MetricBuilder
 let partition = 1;
 let output_rows = MetricBuilder::new(&metrics)
     .output_rows(partition);

 // Counter can be incremented
 output_rows.add(13);

 // The value can be retrieved directly:
 assert_eq!(output_rows.value(), 13);

 // As well as from the metrics set
 assert_eq!(metrics.clone_inner().output_rows(), Some(13));
```

[`ExecutionPlan`]: super::ExecutionPlan

```rust
pub struct Metric {
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
  pub fn new(value: MetricValue, partition: Option<usize>) -> Self { /* ... */ }
  ```
  Create a new [`Metric`]. Consider using [`MetricBuilder`]

- ```rust
  pub fn new_with_labels(value: MetricValue, partition: Option<usize>, labels: Vec<Label>) -> Self { /* ... */ }
  ```
  Create a new [`Metric`]. Consider using [`MetricBuilder`]

- ```rust
  pub fn with_label(self: Self, label: Label) -> Self { /* ... */ }
  ```
  Add a new label to this metric

- ```rust
  pub fn labels(self: &Self) -> &[Label] { /* ... */ }
  ```
  What labels are present for this metric?

- ```rust
  pub fn value(self: &Self) -> &MetricValue { /* ... */ }
  ```
  Return a reference to the value of this metric

- ```rust
  pub fn value_mut(self: &mut Self) -> &mut MetricValue { /* ... */ }
  ```
  Return a mutable reference to the value of this metric

- ```rust
  pub fn partition(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Return a reference to the partition

###### Trait Implementations

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Send**
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

#### Struct `MetricsSet`

A snapshot of the metrics for a particular ([`ExecutionPlan`]).

[`ExecutionPlan`]: super::ExecutionPlan

```rust
pub struct MetricsSet {
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
  Create a new container of metrics

- ```rust
  pub fn push(self: &mut Self, metric: Arc<Metric>) { /* ... */ }
  ```
  Add the specified metric

- ```rust
  pub fn iter(self: &Self) -> impl Iterator<Item = &Arc<Metric>> { /* ... */ }
  ```
  Returns an iterator across all metrics

- ```rust
  pub fn output_rows(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Convenience: return the number of rows produced, aggregated

- ```rust
  pub fn spill_count(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Convenience: return the count of spills, aggregated

- ```rust
  pub fn spilled_bytes(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Convenience: return the total byte size of spills, aggregated

- ```rust
  pub fn spilled_rows(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Convenience: return the total rows of spills, aggregated

- ```rust
  pub fn elapsed_compute(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Convenience: return the amount of elapsed CPU time spent,

- ```rust
  pub fn sum<F>(self: &Self, f: F) -> Option<MetricValue>
where
    F: FnMut(&Metric) -> bool { /* ... */ }
  ```
  Sums the values for metrics for which `f(metric)` returns

- ```rust
  pub fn sum_by_name(self: &Self, metric_name: &str) -> Option<MetricValue> { /* ... */ }
  ```
  Returns the sum of all the metrics with the specified name

- ```rust
  pub fn aggregate_by_name(self: &Self) -> Self { /* ... */ }
  ```
  Returns a new derived `MetricsSet` where all metrics

- ```rust
  pub fn sorted_for_display(self: Self) -> Self { /* ... */ }
  ```
  Sort the order of metrics so the "most useful" show up first

- ```rust
  pub fn timestamps_removed(self: Self) -> Self { /* ... */ }
  ```
  Remove all timestamp metrics (for more compact display)

###### Trait Implementations

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> MetricsSet { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> MetricsSet { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Sync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```
    Format the [`MetricsSet`] as a single string

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

#### Struct `ExecutionPlanMetricsSet`

A set of [`Metric`]s for an individual "operator" (e.g. `&dyn
ExecutionPlan`).

This structure is intended as a convenience for [`ExecutionPlan`]
implementations so they can generate different streams for multiple
partitions but easily report them together.

Each `clone()` of this structure will add metrics to the same
underlying metrics set

[`ExecutionPlan`]: super::ExecutionPlan

```rust
pub struct ExecutionPlanMetricsSet {
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
  Create a new empty shared metrics set

- ```rust
  pub fn register(self: &Self, metric: Arc<Metric>) { /* ... */ }
  ```
  Add the specified metric to the underlying metric set

- ```rust
  pub fn clone_inner(self: &Self) -> MetricsSet { /* ... */ }
  ```
  Return a clone of the inner [`MetricsSet`]

###### Trait Implementations

- **Send**
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExecutionPlanMetricsSet { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> ExecutionPlanMetricsSet { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Struct `Label`

`name=value` pairs identifying a metric. This concept is called various things
in various different systems:

"labels" in
[prometheus](https://prometheus.io/docs/concepts/data_model/) and
"tags" in
[InfluxDB](https://docs.influxdata.com/influxdb/v1.8/write_protocols/line_protocol_tutorial/)
, "attributes" in [open
telemetry]<https://github.com/open-telemetry/opentelemetry-specification/blob/main/specification/metrics/datamodel.md>,
etc.

As the name and value are expected to mostly be constant strings,
use a [`Cow`] to avoid copying / allocations in this common case.

```rust
pub struct Label {
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
  pub fn new</* synthetic */ impl Into<Cow<'static, str>>: Into<Cow<''static, str>>, /* synthetic */ impl Into<Cow<'static, str>>: Into<Cow<''static, str>>>(name: impl Into<Cow<''static, str>>, value: impl Into<Cow<''static, str>>) -> Self { /* ... */ }
  ```
  Create a new [`Label`]

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Returns the name of this label

- ```rust
  pub fn value(self: &Self) -> &str { /* ... */ }
  ```
  Returns the value of this label

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Label) -> bool { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Label { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **IntoEither**
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

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Allocation**
- **MaybeSendSync**
- **StructuralPartialEq**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Re-exports

#### Re-export `BaselineMetrics`

```rust
pub use baseline::BaselineMetrics;
```

#### Re-export `RecordOutput`

```rust
pub use baseline::RecordOutput;
```

#### Re-export `SpillMetrics`

```rust
pub use baseline::SpillMetrics;
```

#### Re-export `MetricBuilder`

```rust
pub use builder::MetricBuilder;
```

#### Re-export `Count`

```rust
pub use value::Count;
```

#### Re-export `Gauge`

```rust
pub use value::Gauge;
```

#### Re-export `MetricValue`

```rust
pub use value::MetricValue;
```

#### Re-export `ScopedTimerGuard`

```rust
pub use value::ScopedTimerGuard;
```

#### Re-export `Time`

```rust
pub use value::Time;
```

#### Re-export `Timestamp`

```rust
pub use value::Timestamp;
```

## Module `placeholder_row`

EmptyRelation produce_one_row=true execution plan

```rust
pub mod placeholder_row { /* ... */ }
```

### Types

#### Struct `PlaceholderRowExec`

Execution plan for empty relation with produce_one_row=true

```rust
pub struct PlaceholderRowExec {
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
  pub fn new(schema: SchemaRef) -> Self { /* ... */ }
  ```
  Create a new PlaceholderRowExec

- ```rust
  pub fn with_partitions(self: Self, partitions: usize) -> Self { /* ... */ }
  ```
  Create a new PlaceholderRowExecPlaceholderRowExec with specified partition number

###### Trait Implementations

- **RefUnwindSafe**
- **ErasedDestructor**
- **IntoEither**
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
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PlaceholderRowExec { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
## Module `projection`

Defines the projection execution plan. A projection determines which columns or expressions
are returned from a query. The SQL statement `SELECT a, b, a+b FROM t1` is an example
of a projection on table `t1` where the expressions `a`, `b`, and `a+b` are the
projection expressions. `SELECT` without `FROM` will only evaluate expressions.

```rust
pub mod projection { /* ... */ }
```

### Types

#### Struct `ProjectionExec`

Execution plan for a projection

```rust
pub struct ProjectionExec {
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
  pub fn try_new(expr: Vec<(Arc<dyn PhysicalExpr>, String)>, input: Arc<dyn ExecutionPlan>) -> Result<Self> { /* ... */ }
  ```
  Create a projection on an input

- ```rust
  pub fn expr(self: &Self) -> &[(Arc<dyn PhysicalExpr>, String)] { /* ... */ }
  ```
  The projection expressions stored as tuples of (expression, output column name)

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  The input plan

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ProjectionExec { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
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
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn supports_limit_pushdown(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **UnwindSafe**
#### Struct `JoinData`

```rust
pub struct JoinData {
    pub projected_left_child: ProjectionExec,
    pub projected_right_child: ProjectionExec,
    pub join_filter: Option<crate::joins::utils::JoinFilter>,
    pub join_on: crate::joins::utils::JoinOn,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `projected_left_child` | `ProjectionExec` |  |
| `projected_right_child` | `ProjectionExec` |  |
| `join_filter` | `Option<crate::joins::utils::JoinFilter>` |  |
| `join_on` | `crate::joins::utils::JoinOn` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **IntoEither**
- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `EmbeddedProjection`

```rust
pub trait EmbeddedProjection: ExecutionPlan + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `with_projection`

##### Implementations

This trait is implemented for the following types:

- `FilterExec`
- `HashJoinExec`
- `NestedLoopJoinExec`

### Functions

#### Function `try_embed_projection`

Some projection can't be pushed down left input or right input of hash join because filter or on need may need some columns that won't be used in later.
By embed those projection to hash join, we can reduce the cost of build_batch_from_indices in hash join (build_batch_from_indices need to can compute::take() for each column) and avoid unnecessary output creation.

```rust
pub fn try_embed_projection<Exec: EmbeddedProjection + ''static>(projection: &ProjectionExec, execution_plan: &Exec) -> datafusion_common::Result<Option<std::sync::Arc<dyn ExecutionPlan>>> { /* ... */ }
```

#### Function `try_pushdown_through_join`

```rust
pub fn try_pushdown_through_join(projection: &ProjectionExec, join_left: &std::sync::Arc<dyn ExecutionPlan>, join_right: &std::sync::Arc<dyn ExecutionPlan>, join_on: crate::joins::utils::JoinOnRef<''_>, schema: arrow::datatypes::SchemaRef, filter: Option<&crate::joins::utils::JoinFilter>) -> datafusion_common::Result<Option<JoinData>> { /* ... */ }
```

#### Function `remove_unnecessary_projections`

This function checks if `plan` is a [`ProjectionExec`], and inspects its
input(s) to test whether it can push `plan` under its input(s). This function
will operate on the entire tree and may ultimately remove `plan` entirely
by leveraging source providers with built-in projection capabilities.

```rust
pub fn remove_unnecessary_projections(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn ExecutionPlan>>> { /* ... */ }
```

#### Function `all_alias_free_columns`

Given the expression set of a projection, checks if the projection causes
any renaming or constructs a non-`Column` physical expression.

```rust
pub fn all_alias_free_columns(exprs: &[(std::sync::Arc<dyn PhysicalExpr>, String)]) -> bool { /* ... */ }
```

#### Function `new_projections_for_columns`

Updates a source provider's projected columns according to the given
projection operator's expressions. To use this function safely, one must
ensure that all expressions are `Column` expressions without aliases.

```rust
pub fn new_projections_for_columns(projection: &ProjectionExec, source: &[usize]) -> Vec<usize> { /* ... */ }
```

#### Function `make_with_child`

Creates a new [`ProjectionExec`] instance with the given child plan and
projected expressions.

```rust
pub fn make_with_child(projection: &ProjectionExec, child: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<std::sync::Arc<dyn ExecutionPlan>> { /* ... */ }
```

#### Function `all_columns`

Returns `true` if all the expressions in the argument are `Column`s.

```rust
pub fn all_columns(exprs: &[(std::sync::Arc<dyn PhysicalExpr>, String)]) -> bool { /* ... */ }
```

#### Function `update_expr`

The function operates in two modes:

1) When `sync_with_child` is `true`:

   The function updates the indices of `expr` if the expression resides
   in the input plan. For instance, given the expressions `a@1 + b@2`
   and `c@0` with the input schema `c@2, a@0, b@1`, the expressions are
   updated to `a@0 + b@1` and `c@2`.

2) When `sync_with_child` is `false`:

   The function determines how the expression would be updated if a projection
   was placed before the plan associated with the expression. If the expression
   cannot be rewritten after the projection, it returns `None`. For example,
   given the expressions `c@0`, `a@1` and `b@2`, and the [`ProjectionExec`] with
   an output schema of `a, c_new`, then `c@0` becomes `c_new@1`, `a@1` becomes
   `a@0`, but `b@2` results in `None` since the projection does not include `b`.

```rust
pub fn update_expr(expr: &std::sync::Arc<dyn PhysicalExpr>, projected_exprs: &[(std::sync::Arc<dyn PhysicalExpr>, String)], sync_with_child: bool) -> datafusion_common::Result<Option<std::sync::Arc<dyn PhysicalExpr>>> { /* ... */ }
```

#### Function `physical_to_column_exprs`

Downcasts all the expressions in `exprs` to `Column`s. If any of the given
expressions is not a `Column`, returns `None`.

```rust
pub fn physical_to_column_exprs(exprs: &[(std::sync::Arc<dyn PhysicalExpr>, String)]) -> Option<Vec<(super::expressions::Column, String)>> { /* ... */ }
```

#### Function `new_join_children`

If pushing down the projection over this join's children seems possible,
this function constructs the new [`ProjectionExec`]s that will come on top
of the original children of the join.

```rust
pub fn new_join_children(projection_as_columns: &[(super::expressions::Column, String)], far_right_left_col_ind: i32, far_left_right_col_ind: i32, left_child: &std::sync::Arc<dyn ExecutionPlan>, right_child: &std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::Result<(ProjectionExec, ProjectionExec)> { /* ... */ }
```

#### Function `join_allows_pushdown`

Checks three conditions for pushing a projection down through a join:
- Projection must narrow the join output schema.
- Columns coming from left/right tables must be collected at the left/right
  sides of the output table.
- Left or right table is not lost after the projection.

```rust
pub fn join_allows_pushdown(projection_as_columns: &[(super::expressions::Column, String)], join_schema: &arrow::datatypes::SchemaRef, far_right_left_col_ind: i32, far_left_right_col_ind: i32) -> bool { /* ... */ }
```

#### Function `join_table_borders`

Returns the last index before encountering a column coming from the right table when traveling
through the projection from left to right, and the last index before encountering a column
coming from the left table when traveling through the projection from right to left.
If there is no column in the projection coming from the left side, it returns (-1, ...),
if there is no column in the projection coming from the right side, it returns (..., projection length).

```rust
pub fn join_table_borders(left_table_column_count: usize, projection_as_columns: &[(super::expressions::Column, String)]) -> (i32, i32) { /* ... */ }
```

#### Function `update_join_on`

Tries to update the equi-join `Column`'s of a join as if the input of
the join was replaced by a projection.

```rust
pub fn update_join_on(proj_left_exprs: &[(super::expressions::Column, String)], proj_right_exprs: &[(super::expressions::Column, String)], hash_join_on: &[(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)], left_field_size: usize) -> Option<Vec<(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)>> { /* ... */ }
```

#### Function `update_join_filter`

Tries to update the column indices of a [`JoinFilter`] as if the input of
the join was replaced by a projection.

```rust
pub fn update_join_filter(projection_left_exprs: &[(super::expressions::Column, String)], projection_right_exprs: &[(super::expressions::Column, String)], join_filter: &crate::joins::utils::JoinFilter, left_field_size: usize) -> Option<crate::joins::utils::JoinFilter> { /* ... */ }
```

## Module `recursive_query`

Defines the recursive query plan

```rust
pub mod recursive_query { /* ... */ }
```

### Types

#### Struct `RecursiveQueryExec`

Recursive query execution plan.

This plan has two components: a base part (the static term) and
a dynamic part (the recursive term). The execution will start from
the base, and as long as the previous iteration produced at least
a single new row (taking care of the distinction) the recursive
part will be continuously executed.

Before each execution of the dynamic part, the rows from the previous
iteration will be available in a "working table" (not a real table,
can be only accessed using a continuance operation).

Note that there won't be any limit or checks applied to detect
an infinite recursion, so it is up to the planner to ensure that
it won't happen.

```rust
pub struct RecursiveQueryExec {
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
  pub fn try_new(name: String, static_term: Arc<dyn ExecutionPlan>, recursive_term: Arc<dyn ExecutionPlan>, is_distinct: bool) -> Result<Self> { /* ... */ }
  ```
  Create a new RecursiveQueryExec

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Ref to name

- ```rust
  pub fn static_term(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Ref to static term

- ```rust
  pub fn recursive_term(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Ref to recursive term

- ```rust
  pub fn is_distinct(self: &Self) -> bool { /* ... */ }
  ```
  is distinct

###### Trait Implementations

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Send**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn required_input_distribution(self: &Self) -> Vec<crate::Distribution> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RecursiveQueryExec { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `repartition`

This file implements the [`RepartitionExec`]  operator, which maps N input
partitions to M output partitions based on a partitioning scheme, optionally
maintaining the order of the input rows in the output.

```rust
pub mod repartition { /* ... */ }
```

### Types

#### Struct `BatchPartitioner`

A utility that can be used to partition batches based on [`Partitioning`]

```rust
pub struct BatchPartitioner {
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
  pub fn try_new(partitioning: Partitioning, timer: metrics::Time) -> Result<Self> { /* ... */ }
  ```
  Create a new [`BatchPartitioner`] with the provided [`Partitioning`]

- ```rust
  pub fn partition<F>(self: &mut Self, batch: RecordBatch, f: F) -> Result<()>
where
    F: FnMut(usize, RecordBatch) -> Result<()> { /* ... */ }
  ```
  Partition the provided [`RecordBatch`] into one or more partitioned [`RecordBatch`]

###### Trait Implementations

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **RefUnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Freeze**
#### Struct `RepartitionExec`

 Maps `N` input partitions to `M` output partitions based on a
 [`Partitioning`] scheme.

 # Background

 DataFusion, like most other commercial systems, with the
 notable exception of DuckDB, uses the "Exchange Operator" based
 approach to parallelism which works well in practice given
 sufficient care in implementation.

 DataFusion's planner picks the target number of partitions and
 then [`RepartitionExec`] redistributes [`RecordBatch`]es to that number
 of output partitions.

 For example, given `target_partitions=3` (trying to use 3 cores)
 but scanning an input with 2 partitions, `RepartitionExec` can be
 used to get 3 even streams of `RecordBatch`es


```text
        ▲                  ▲                  ▲
        │                  │                  │
        │                  │                  │
        │                  │                  │
┌───────────────┐  ┌───────────────┐  ┌───────────────┐
│    GroupBy    │  │    GroupBy    │  │    GroupBy    │
│   (Partial)   │  │   (Partial)   │  │   (Partial)   │
└───────────────┘  └───────────────┘  └───────────────┘
        ▲                  ▲                  ▲
        └──────────────────┼──────────────────┘
                           │
              ┌─────────────────────────┐
              │     RepartitionExec     │
              │   (hash/round robin)    │
              └─────────────────────────┘
                         ▲   ▲
             ┌───────────┘   └───────────┐
             │                           │
             │                           │
        .─────────.                 .─────────.
     ,─'           '─.           ,─'           '─.
    ;      Input      :         ;      Input      :
    :   Partition 0   ;         :   Partition 1   ;
     ╲               ╱           ╲               ╱
      '─.         ,─'             '─.         ,─'
         `───────'                   `───────'
```

 # Error Handling

 If any of the input partitions return an error, the error is propagated to
 all output partitions and inputs are not polled again.

 # Output Ordering

 If more than one stream is being repartitioned, the output will be some
 arbitrary interleaving (and thus unordered) unless
 [`Self::with_preserve_order`] specifies otherwise.

 # Footnote

 The "Exchange Operator" was first described in the 1989 paper
 [Encapsulation of parallelism in the Volcano query processing
 system Paper](https://dl.acm.org/doi/pdf/10.1145/93605.98720)
 which uses the term "Exchange" for the concept of repartitioning
 data across threads.

```rust
pub struct RepartitionExec {
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
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input execution plan

- ```rust
  pub fn partitioning(self: &Self) -> &Partitioning { /* ... */ }
  ```
  Partitioning scheme to use

- ```rust
  pub fn preserve_order(self: &Self) -> bool { /* ... */ }
  ```
  Get preserve_order flag of the RepartitionExecutor

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Get name used to display this Exec

- ```rust
  pub fn try_new(input: Arc<dyn ExecutionPlan>, partitioning: Partitioning) -> Result<Self> { /* ... */ }
  ```
  Create a new RepartitionExec, that produces output `partitioning`, and

- ```rust
  pub fn with_preserve_order(self: Self) -> Self { /* ... */ }
  ```
  Specify if this repartitioning operation should preserve the order of

###### Trait Implementations

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
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
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

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RepartitionExec { /* ... */ }
    ```

- **Sync**
## Module `sorts`

Sort functionalities

```rust
pub mod sorts { /* ... */ }
```

### Modules

## Module `partial_sort`

 Partial Sort deals with input data that partially
 satisfies the required sort order. Such an input data can be
 partitioned into segments where each segment already has the
 required information for lexicographic sorting so sorting
 can be done without loading the entire dataset.

 Consider a sort plan having an input with ordering `a ASC, b ASC`

 ```text
 +---+---+---+
 | a | b | d |
 +---+---+---+
 | 0 | 0 | 3 |
 | 0 | 0 | 2 |
 | 0 | 1 | 1 |
 | 0 | 2 | 0 |
 +---+---+---+
```

 and required ordering for the plan is `a ASC, b ASC, d ASC`.
 The first 3 rows(segment) can be sorted as the segment already
 has the required information for the sort, but the last row
 requires further information as the input can continue with a
 batch with a starting row where a and b does not change as below

 ```text
 +---+---+---+
 | a | b | d |
 +---+---+---+
 | 0 | 2 | 4 |
 +---+---+---+
```

 The plan concats incoming data with such last rows of previous input
 and continues partial sorting of the segments.

```rust
pub mod partial_sort { /* ... */ }
```

### Types

#### Struct `PartialSortExec`

Partial Sort execution plan.

```rust
pub struct PartialSortExec {
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
  pub fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>, common_prefix_length: usize) -> Self { /* ... */ }
  ```
  Create a new partial sort execution plan

- ```rust
  pub fn preserve_partitioning(self: &Self) -> bool { /* ... */ }
  ```
  Whether this `PartialSortExec` preserves partitioning of the children

- ```rust
  pub fn with_preserve_partitioning(self: Self, preserve_partitioning: bool) -> Self { /* ... */ }
  ```
  Specify the partitioning behavior of this partial sort exec

- ```rust
  pub fn with_fetch(self: Self, fetch: Option<usize>) -> Self { /* ... */ }
  ```
  Modify how many rows to include in the result

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input schema

- ```rust
  pub fn expr(self: &Self) -> &LexOrdering { /* ... */ }
  ```
  Sort expressions

- ```rust
  pub fn fetch(self: &Self) -> Option<usize> { /* ... */ }
  ```
  If `Some(fetch)`, limits output to only the first "fetch" items

- ```rust
  pub fn common_prefix_length(self: &Self) -> usize { /* ... */ }
  ```
  Common prefix length

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
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
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn required_input_distribution(self: &Self) -> Vec<Distribution> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
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
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **UnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PartialSortExec { /* ... */ }
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

- **Freeze**
## Module `sort`

Sort that deals with an arbitrary size of the input.
It will do in-memory sorting if it has enough memory budget
but spills to disk if needed.

```rust
pub mod sort { /* ... */ }
```

### Types

#### Struct `SortExec`

Sort execution plan.

Support sorting datasets that are larger than the memory allotted
by the memory manager, by spilling to disk.

```rust
pub struct SortExec {
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
  pub fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>) -> Self { /* ... */ }
  ```
  Create a new sort execution plan that produces a single,

- ```rust
  pub fn preserve_partitioning(self: &Self) -> bool { /* ... */ }
  ```
  Whether this `SortExec` preserves partitioning of the children

- ```rust
  pub fn with_preserve_partitioning(self: Self, preserve_partitioning: bool) -> Self { /* ... */ }
  ```
  Specify the partitioning behavior of this sort exec

- ```rust
  pub fn with_fetch(self: &Self, fetch: Option<usize>) -> Self { /* ... */ }
  ```
  Modify how many rows to include in the result

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input schema

- ```rust
  pub fn expr(self: &Self) -> &LexOrdering { /* ... */ }
  ```
  Sort expressions

- ```rust
  pub fn fetch(self: &Self) -> Option<usize> { /* ... */ }
  ```
  If `Some(fetch)`, limits output to only the first "fetch" items

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
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
    fn required_input_distribution(self: &Self) -> Vec<Distribution> { /* ... */ }
    ```

  - ```rust
    fn children(self: &Self) -> Vec<&Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn cardinality_effect(self: &Self) -> CardinalityEffect { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```
    Tries to swap the projection with its input [`SortExec`]. If it can be done,

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SortExec { /* ... */ }
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

### Functions

#### Function `sort_batch`

```rust
pub fn sort_batch(batch: &arrow::array::RecordBatch, expressions: &datafusion_physical_expr::LexOrdering, fetch: Option<usize>) -> datafusion_common::Result<arrow::array::RecordBatch> { /* ... */ }
```

## Module `sort_preserving_merge`

[`SortPreservingMergeExec`] merges multiple sorted streams into one sorted stream.

```rust
pub mod sort_preserving_merge { /* ... */ }
```

### Types

#### Struct `SortPreservingMergeExec`

Sort preserving merge execution plan

# Overview

This operator implements a K-way merge. It is used to merge multiple sorted
streams into a single sorted stream and is highly optimized.

## Inputs:

1. A list of sort expressions
2. An input plan, where each partition is sorted with respect to
   these sort expressions.

## Output:

1. A single partition that is also sorted with respect to the expressions

## Diagram

```text
┌─────────────────────────┐
│ ┌───┬───┬───┬───┐       │
│ │ A │ B │ C │ D │ ...   │──┐
│ └───┴───┴───┴───┘       │  │
└─────────────────────────┘  │  ┌───────────────────┐    ┌───────────────────────────────┐
  Stream 1                   │  │                   │    │ ┌───┬───╦═══╦───┬───╦═══╗     │
                             ├─▶│SortPreservingMerge│───▶│ │ A │ B ║ B ║ C │ D ║ E ║ ... │
                             │  │                   │    │ └───┴─▲─╩═══╩───┴───╩═══╝     │
┌─────────────────────────┐  │  └───────────────────┘    └─┬─────┴───────────────────────┘
│ ╔═══╦═══╗               │  │
│ ║ B ║ E ║     ...       │──┘                             │
│ ╚═══╩═══╝               │              Stable sort if `enable_round_robin_repartition=false`:
└─────────────────────────┘              the merged stream places equal rows from stream 1
  Stream 2


 Input Partitions                                          Output Partition
   (sorted)                                                  (sorted)
```

# Error Handling

If any of the input partitions return an error, the error is propagated to
the output and inputs are not polled again.

```rust
pub struct SortPreservingMergeExec {
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
  pub fn new(expr: LexOrdering, input: Arc<dyn ExecutionPlan>) -> Self { /* ... */ }
  ```
  Create a new sort execution plan

- ```rust
  pub fn with_fetch(self: Self, fetch: Option<usize>) -> Self { /* ... */ }
  ```
  Sets the number of rows to fetch

- ```rust
  pub fn with_round_robin_repartition(self: Self, enable_round_robin_repartition: bool) -> Self { /* ... */ }
  ```
  Sets the selection strategy of tied winners of the loser tree algorithm

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input schema

- ```rust
  pub fn expr(self: &Self) -> &LexOrdering { /* ... */ }
  ```
  Sort expressions

- ```rust
  pub fn fetch(self: &Self) -> Option<usize> { /* ... */ }
  ```
  Fetch

###### Trait Implementations

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

- **MaybeSendSync**
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
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```
    Sets the number of rows to fetch

  - ```rust
    fn required_input_distribution(self: &Self) -> Vec<Distribution> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
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

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn supports_limit_pushdown(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```
    Tries to swap the projection with its input [`SortPreservingMergeExec`].

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SortPreservingMergeExec { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Unpin**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `streaming_merge`

Merge that deals with an arbitrary size of streaming inputs.
This is an order-preserving merge.

```rust
pub mod streaming_merge { /* ... */ }
```

### Types

#### Struct `StreamingMergeBuilder`

```rust
pub struct StreamingMergeBuilder<''a> {
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
  pub fn with_streams(self: Self, streams: Vec<SendableRecordBatchStream>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_schema(self: Self, schema: SchemaRef) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_expressions(self: Self, expressions: &''a LexOrdering) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_metrics(self: Self, metrics: BaselineMetrics) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_batch_size(self: Self, batch_size: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_fetch(self: Self, fetch: Option<usize>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_reservation(self: Self, reservation: MemoryReservation) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_round_robin_tie_breaker(self: Self, enable_round_robin_tie_breaker: bool) -> Self { /* ... */ }
  ```
  See [SortPreservingMergeExec::with_round_robin_repartition] for more

- ```rust
  pub fn build(self: Self) -> Result<SendableRecordBatchStream> { /* ... */ }
  ```

###### Trait Implementations

- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
## Module `spill`

Defines the spilling functions

```rust
pub mod spill { /* ... */ }
```

### Functions

#### Function `spill_record_batch_by_size`

**Attributes:**

- `#[deprecated(since = "46.0.0", note =
"This method is deprecated. Use `SpillManager::spill_record_batch_by_size` instead.")]`

**⚠️ Deprecated since 46.0.0**: This method is deprecated. Use `SpillManager::spill_record_batch_by_size` instead.

Spill the `RecordBatch` to disk as smaller batches
split by `batch_size_rows`

```rust
pub fn spill_record_batch_by_size(batch: &arrow::record_batch::RecordBatch, path: std::path::PathBuf, schema: arrow::datatypes::SchemaRef, batch_size_rows: usize) -> datafusion_common::Result<()> { /* ... */ }
```

#### Function `get_record_batch_memory_size`

Calculate total used memory of this batch.

This function is used to estimate the physical memory usage of the `RecordBatch`.
It only counts the memory of large data `Buffer`s, and ignores metadata like
types and pointers.
The implementation will add up all unique `Buffer`'s memory
size, due to:
- The data pointer inside `Buffer` are memory regions returned by global memory
  allocator, those regions can't have overlap.
- The actual used range of `ArrayRef`s inside `RecordBatch` can have overlap
  or reuse the same `Buffer`. For example: taking a slice from `Array`.

Example:
For a `RecordBatch` with two columns: `col1` and `col2`, two columns are pointing
to a sub-region of the same buffer.

{xxxxxxxxxxxxxxxxxxx} <--- buffer
      ^    ^  ^    ^
      |    |  |    |
col1->{    }  |    |
col2--------->{    }

In the above case, `get_record_batch_memory_size` will return the size of
the buffer, instead of the sum of `col1` and `col2`'s actual memory size.

Note: Current `RecordBatch`.get_array_memory_size()` will double count the
buffer memory size if multiple arrays within the batch are sharing the same
`Buffer`. This method provides temporary fix until the issue is resolved:
<https://github.com/apache/arrow-rs/issues/6439>

```rust
pub fn get_record_batch_memory_size(batch: &arrow::record_batch::RecordBatch) -> usize { /* ... */ }
```

## Module `stream`

Stream wrappers for physical operators

```rust
pub mod stream { /* ... */ }
```

### Types

#### Struct `RecordBatchReceiverStreamBuilder`

Builder for `RecordBatchReceiverStream` that propagates errors
and panic's correctly.

[`RecordBatchReceiverStreamBuilder`] is used to spawn one or more tasks
that produce [`RecordBatch`]es and send them to a single
`Receiver` which can improve parallelism.

This also handles propagating panic`s and canceling the tasks.

# Example

The following example spawns 2 tasks that will write [`RecordBatch`]es to
the `tx` end of the builder, after building the stream, we can receive
those batches with calling `.next()`

```
# use std::sync::Arc;
# use datafusion_common::arrow::datatypes::{Schema, Field, DataType};
# use datafusion_common::arrow::array::RecordBatch;
# use datafusion_physical_plan::stream::RecordBatchReceiverStreamBuilder;
# use futures::stream::StreamExt;
# use tokio::runtime::Builder;
# let rt = Builder::new_current_thread().build().unwrap();
#
# rt.block_on(async {
let schema = Arc::new(Schema::new(vec![Field::new("foo", DataType::Int8, false)]));
let mut builder = RecordBatchReceiverStreamBuilder::new(Arc::clone(&schema), 10);

// task 1
let tx_1 = builder.tx();
let schema_1 = Arc::clone(&schema);
builder.spawn(async move {
    // Your task needs to send batches to the tx
    tx_1.send(Ok(RecordBatch::new_empty(schema_1))).await.unwrap();

    Ok(())
});

// task 2
let tx_2 = builder.tx();
let schema_2 = Arc::clone(&schema);
builder.spawn(async move {
    // Your task needs to send batches to the tx
    tx_2.send(Ok(RecordBatch::new_empty(schema_2))).await.unwrap();

    Ok(())
});

let mut stream = builder.build();
while let Some(res_batch) = stream.next().await {
    // `res_batch` can either from task 1 or 2

    // do something with `res_batch`
}
# });
```

```rust
pub struct RecordBatchReceiverStreamBuilder {
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
  pub fn new(schema: SchemaRef, capacity: usize) -> Self { /* ... */ }
  ```
  Create new channels with the specified buffer size

- ```rust
  pub fn tx(self: &Self) -> Sender<Result<RecordBatch>> { /* ... */ }
  ```
  Get a handle for sending [`RecordBatch`] to the output

- ```rust
  pub fn spawn<F>(self: &mut Self, task: F)
where
    F: Future<Output = Result<()>> + Send + ''static { /* ... */ }
  ```
  Spawn task that will be aborted if this builder (or the stream

- ```rust
  pub fn spawn_blocking<F>(self: &mut Self, f: F)
where
    F: FnOnce() -> Result<()> + Send + ''static { /* ... */ }
  ```
  Spawn a blocking task tied to the builder and stream.

- ```rust
  pub fn build(self: Self) -> SendableRecordBatchStream { /* ... */ }
  ```
  Create a stream of all [`RecordBatch`] written to `tx`

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
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

- **Allocation**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Sync**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
#### Struct `RecordBatchStreamAdapter`

Combines a [`Stream`] with a [`SchemaRef`] implementing
[`SendableRecordBatchStream`] for the combination

See [`Self::new`] for an example

```rust
pub struct RecordBatchStreamAdapter<S> {
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
  pub fn new(schema: SchemaRef, stream: S) -> Self { /* ... */ }
  ```
  Creates a new [`RecordBatchStreamAdapter`] from the provided schema and stream.

###### Trait Implementations

- **TryStreamExt**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **RecordBatchStream**
  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

- **Send**
- **Freeze**
- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

  - ```rust
    fn size_hint(self: &Self) -> (usize, Option<usize>) { /* ... */ }
    ```

- **StreamExt**
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
- **IntoEither**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Struct `EmptyRecordBatchStream`

`EmptyRecordBatchStream` can be used to create a [`RecordBatchStream`]
that will produce no results

```rust
pub struct EmptyRecordBatchStream {
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
  pub fn new(schema: SchemaRef) -> Self { /* ... */ }
  ```
  Create an empty RecordBatchStream

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Send**
- **TryStream**
  - ```rust
    fn try_poll_next(self: Pin<&mut S>, cx: &mut Context<''_>) -> Poll<Option<Result<<S as TryStream>::Ok, <S as TryStream>::Error>>> { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **StreamExt**
- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **UnwindSafe**
- **TryStreamExt**
- **IntoEither**
- **RecordBatchStream**
  - ```rust
    fn schema(self: &Self) -> SchemaRef { /* ... */ }
    ```

- **Stream**
  - ```rust
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<''_>) -> Poll<Option<<Self as >::Item>> { /* ... */ }
    ```

## Module `streaming`

Generic plans for deferred execution: [`StreamingTableExec`] and [`PartitionStream`]

```rust
pub mod streaming { /* ... */ }
```

### Types

#### Struct `StreamingTableExec`

An [`ExecutionPlan`] for one or more [`PartitionStream`]s.

If your source can be represented as one or more [`PartitionStream`]s, you can
use this struct to implement [`ExecutionPlan`].

```rust
pub struct StreamingTableExec {
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
  pub fn try_new</* synthetic */ impl IntoIterator<Item = LexOrdering>: IntoIterator<Item = LexOrdering>>(schema: SchemaRef, partitions: Vec<Arc<dyn PartitionStream>>, projection: Option<&Vec<usize>>, projected_output_ordering: impl IntoIterator<Item = LexOrdering>, infinite: bool, limit: Option<usize>) -> Result<Self> { /* ... */ }
  ```
  Try to create a new [`StreamingTableExec`] returning an error if the schema is incorrect

- ```rust
  pub fn partitions(self: &Self) -> &Vec<Arc<dyn PartitionStream>> { /* ... */ }
  ```

- ```rust
  pub fn partition_schema(self: &Self) -> &SchemaRef { /* ... */ }
  ```

- ```rust
  pub fn projection(self: &Self) -> &Option<Arc<[usize]>> { /* ... */ }
  ```

- ```rust
  pub fn projected_schema(self: &Self) -> &Schema { /* ... */ }
  ```

- ```rust
  pub fn projected_output_ordering(self: &Self) -> impl IntoIterator<Item = LexOrdering> { /* ... */ }
  ```

- ```rust
  pub fn is_infinite(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn limit(self: &Self) -> Option<usize> { /* ... */ }
  ```

###### Trait Implementations

- **ErasedDestructor**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
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

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
    fn fetch(self: &Self) -> Option<usize> { /* ... */ }
    ```

  - ```rust
    fn children(self: &Self) -> Vec<&Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, ctx: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```
    Tries to embed `projection` to its input (`streaming table`).

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn with_fetch(self: &Self, limit: Option<usize>) -> Option<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> StreamingTableExec { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `PartitionStream`

A partition that can be converted into a [`SendableRecordBatchStream`]

Combined with [`StreamingTableExec`], you can use this trait to implement
[`ExecutionPlan`] for a custom source with less boiler plate than
implementing `ExecutionPlan` directly for many use cases.

```rust
pub trait PartitionStream: Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `schema`: Returns the schema of this partition
- `execute`: Returns a stream yielding this partitions values

## Module `tree_node`

This module provides common traits for visiting or rewriting tree nodes easily.

```rust
pub mod tree_node { /* ... */ }
```

### Types

#### Struct `PlanContext`

A node context object beneficial for writing optimizer rules.
This context encapsulating an [`ExecutionPlan`] node with a payload.

Since each wrapped node has it's children within both the [`PlanContext.plan.children()`],
as well as separately within the [`PlanContext.children`] (which are child nodes wrapped in the context),
it's important to keep these child plans in sync when performing mutations.

Since there are two ways to access child plans directly -— it's recommended
to perform mutable operations via [`Self::update_plan_from_children`].
After mutating the `PlanContext.children`, or after creating the `PlanContext`,
call `update_plan_from_children` to sync.

```rust
pub struct PlanContext<T: Sized> {
    pub plan: std::sync::Arc<dyn ExecutionPlan>,
    pub data: T,
    pub children: Vec<Self>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `plan` | `std::sync::Arc<dyn ExecutionPlan>` | The execution plan associated with this context. |
| `data` | `T` | Custom data payload of the node. |
| `children` | `Vec<Self>` | Child contexts of this node. |

##### Implementations

###### Methods

- ```rust
  pub fn new(plan: Arc<dyn ExecutionPlan>, data: T, children: Vec<Self>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn update_plan_from_children(self: Self) -> Result<Self> { /* ... */ }
  ```
  Update the [`PlanContext.plan.children()`] from the [`PlanContext.children`],

- ```rust
  pub fn new_default(plan: Arc<dyn ExecutionPlan>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **IntoEither**
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Sync**
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
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **ErasedDestructor**
## Module `union`

The Union operator combines multiple inputs with the same schema

```rust
pub mod union { /* ... */ }
```

### Types

#### Struct `UnionExec`

 `UnionExec`: `UNION ALL` execution plan.

 `UnionExec` combines multiple inputs with the same schema by
 concatenating the partitions.  It does not mix or copy data within
 or across partitions. Thus if the input partitions are sorted, the
 output partitions of the union are also sorted.

 For example, given a `UnionExec` of two inputs, with `N`
 partitions, and `M` partitions, there will be `N+M` output
 partitions. The first `N` output partitions are from Input 1
 partitions, and then next `M` output partitions are from Input 2.

 ```text
                       ▲       ▲           ▲         ▲
                       │       │           │         │
     Output            │  ...  │           │         │
   Partitions          │0      │N-1        │ N       │N+M-1
(passes through   ┌────┴───────┴───────────┴─────────┴───┐
 the N+M input    │              UnionExec               │
  partitions)     │                                      │
                  └──────────────────────────────────────┘
                                      ▲
                                      │
                                      │
       Input           ┌────────┬─────┴────┬──────────┐
     Partitions        │ ...    │          │     ...  │
                    0  │        │ N-1      │ 0        │  M-1
                  ┌────┴────────┴───┐  ┌───┴──────────┴───┐
                  │                 │  │                  │
                  │                 │  │                  │
                  │                 │  │                  │
                  │                 │  │                  │
                  │                 │  │                  │
                  │                 │  │                  │
                  │Input 1          │  │Input 2           │
                  └─────────────────┘  └──────────────────┘
 ```

```rust
pub struct UnionExec {
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
  pub fn new(inputs: Vec<Arc<dyn ExecutionPlan>>) -> Self { /* ... */ }
  ```
  Create a new UnionExec

- ```rust
  pub fn inputs(self: &Self) -> &Vec<Arc<dyn ExecutionPlan>> { /* ... */ }
  ```
  Get inputs of the execution plan

###### Trait Implementations

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnionExec { /* ... */ }
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
    fn check_invariants(self: &Self, _check: InvariantLevel) -> Result<()> { /* ... */ }
    ```

  - ```rust
    fn children(self: &Self) -> Vec<&Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn supports_limit_pushdown(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
    ```
    Tries to push `projection` down through `union`. If possible, performs the

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Struct `InterleaveExec`

Combines multiple input streams by interleaving them.

This only works if all inputs have the same hash-partitioning.

# Data Flow
```text
+---------+
|         |---+
| Input 1 |   |
|         |-------------+
+---------+   |         |
              |         |         +---------+
              +------------------>|         |
                +---------------->| Combine |-->
                | +-------------->|         |
                | |     |         +---------+
+---------+     | |     |
|         |-----+ |     |
| Input 2 |       |     |
|         |---------------+
+---------+       |     | |
                  |     | |       +---------+
                  |     +-------->|         |
                  |       +------>| Combine |-->
                  |         +---->|         |
                  |         |     +---------+
+---------+       |         |
|         |-------+         |
| Input 3 |                 |
|         |-----------------+
+---------+
```

```rust
pub struct InterleaveExec {
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
  pub fn try_new(inputs: Vec<Arc<dyn ExecutionPlan>>) -> Result<Self> { /* ... */ }
  ```
  Create a new InterleaveExec

- ```rust
  pub fn inputs(self: &Self) -> &Vec<Arc<dyn ExecutionPlan>> { /* ... */ }
  ```
  Get inputs of the execution plan

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
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
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> InterleaveExec { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

### Functions

#### Function `can_interleave`

If all the input partitions have the same Hash partition spec with the first_input_partition
The InterleaveExec is partition aware.

It might be too strict here in the case that the input partition specs are compatible but not exactly the same.
For example one input partition has the partition spec Hash('a','b','c') and
other has the partition spec Hash('a'), It is safe to derive the out partition with the spec Hash('a','b','c').

```rust
pub fn can_interleave<T: Borrow<std::sync::Arc<dyn ExecutionPlan>>, /* synthetic */ impl Iterator<Item = T>: Iterator<Item = T>>(inputs: impl Iterator<Item = T>) -> bool { /* ... */ }
```

## Module `unnest`

Define a plan for unnesting values in columns that contain a list type.

```rust
pub mod unnest { /* ... */ }
```

### Types

#### Struct `UnnestExec`

Unnest the given columns (either with type struct or list)
For list unnesting, each rows is vertically transformed into multiple rows
For struct unnesting, each columns is horizontally transformed into multiple columns,
Thus the original RecordBatch with dimension (n x m) may have new dimension (n' x m')

See [`UnnestOptions`] for more details and an example.

```rust
pub struct UnnestExec {
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
  pub fn new(input: Arc<dyn ExecutionPlan>, list_column_indices: Vec<ListUnnest>, struct_column_indices: Vec<usize>, schema: SchemaRef, options: UnnestOptions) -> Self { /* ... */ }
  ```
  Create a new [UnnestExec].

- ```rust
  pub fn input(self: &Self) -> &Arc<dyn ExecutionPlan> { /* ... */ }
  ```
  Input execution plan

- ```rust
  pub fn list_column_indices(self: &Self) -> &[ListUnnest] { /* ... */ }
  ```
  Indices of the list-typed columns in the input schema

- ```rust
  pub fn struct_column_indices(self: &Self) -> &[usize] { /* ... */ }
  ```
  Indices of the struct-typed columns in the input schema

- ```rust
  pub fn options(self: &Self) -> &UnnestOptions { /* ... */ }
  ```

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **MaybeSendSync**
- **RefUnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnnestExec { /* ... */ }
    ```

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
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn required_input_distribution(self: &Self) -> Vec<Distribution> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

#### Struct `ListUnnest`

```rust
pub struct ListUnnest {
    pub index_in_input_schema: usize,
    pub depth: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `index_in_input_schema` | `usize` |  |
| `depth` | `usize` |  |

##### Implementations

###### Trait Implementations

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ListUnnest) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **RefUnwindSafe**
- **Copy**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ListUnnest { /* ... */ }
    ```

- **Allocation**
## Module `values`

Values execution plan

```rust
pub mod values { /* ... */ }
```

### Types

#### Struct `ValuesExec`

**Attributes:**

- `#[deprecated(since = "45.0.0", note =
"Use `MemorySourceConfig::try_new_as_values` instead")]`

**⚠️ Deprecated since 45.0.0**: Use `MemorySourceConfig::try_new_as_values` instead

Execution plan for values list based relation (produces constant rows)

```rust
pub struct ValuesExec {
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
  pub fn try_new(schema: SchemaRef, data: Vec<Vec<Arc<dyn PhysicalExpr>>>) -> Result<Self> { /* ... */ }
  ```
  Create a new values exec from data as expr

- ```rust
  pub fn try_new_from_batches(schema: SchemaRef, batches: Vec<RecordBatch>) -> Result<Self> { /* ... */ }
  ```
  Create a new plan using the provided schema and batches.

- ```rust
  pub fn data(self: &Self) -> Vec<RecordBatch> { /* ... */ }
  ```
  Provides the data

###### Trait Implementations

- **Send**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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

- **MaybeSendSync**
- **Freeze**
- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ValuesExec { /* ... */ }
    ```

- **Sync**
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
    fn execute(self: &Self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `windows`

Physical expressions for window functions

```rust
pub mod windows { /* ... */ }
```

### Types

#### Struct `WindowUDFExpr`

Implements [`StandardWindowFunctionExpr`] for [`WindowUDF`]

```rust
pub struct WindowUDFExpr {
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
  pub fn fun(self: &Self) -> &Arc<WindowUDF> { /* ... */ }
  ```

###### Trait Implementations

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowUDFExpr { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StandardWindowFunctionExpr**
  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

  - ```rust
    fn field(self: &Self) -> Result<Field> { /* ... */ }
    ```

  - ```rust
    fn expressions(self: &Self) -> Vec<Arc<dyn PhysicalExpr>> { /* ... */ }
    ```

  - ```rust
    fn create_evaluator(self: &Self) -> Result<Box<dyn PartitionEvaluator>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> Option<Arc<dyn StandardWindowFunctionExpr>> { /* ... */ }
    ```

  - ```rust
    fn get_result_ordering(self: &Self, schema: &SchemaRef) -> Option<PhysicalSortExpr> { /* ... */ }
    ```

- **Freeze**
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

### Functions

#### Function `schema_add_window_field`

Build field from window function and add it into schema

```rust
pub fn schema_add_window_field(args: &[std::sync::Arc<dyn PhysicalExpr>], schema: &arrow::datatypes::Schema, window_fn: &datafusion_expr::WindowFunctionDefinition, fn_name: &str) -> datafusion_common::Result<std::sync::Arc<arrow::datatypes::Schema>> { /* ... */ }
```

#### Function `create_window_expr`

**Attributes:**

- `#[allow(clippy::too_many_arguments)]`

Create a physical expression for window function

```rust
pub fn create_window_expr(fun: &datafusion_expr::WindowFunctionDefinition, name: String, args: &[std::sync::Arc<dyn PhysicalExpr>], partition_by: &[std::sync::Arc<dyn PhysicalExpr>], order_by: &datafusion_physical_expr::LexOrdering, window_frame: std::sync::Arc<datafusion_expr::WindowFrame>, input_schema: &arrow::datatypes::Schema, ignore_nulls: bool) -> datafusion_common::Result<std::sync::Arc<dyn WindowExpr>> { /* ... */ }
```

#### Function `create_udwf_window_expr`

Creates a `StandardWindowFunctionExpr` suitable for a user defined window function

```rust
pub fn create_udwf_window_expr(fun: &std::sync::Arc<datafusion_expr::WindowUDF>, args: &[std::sync::Arc<dyn PhysicalExpr>], input_schema: &arrow::datatypes::Schema, name: String, ignore_nulls: bool) -> datafusion_common::Result<std::sync::Arc<dyn StandardWindowFunctionExpr>> { /* ... */ }
```

#### Function `get_ordered_partition_by_indices`

This function calculates the indices such that when partition by expressions reordered with the indices
resulting expressions define a preset for existing ordering.
For instance, if input is ordered by a, b, c and PARTITION BY b, a is used,
this vector will be [1, 0]. It means that when we iterate b, a columns with the order [1, 0]
resulting vector (a, b) is a preset of the existing ordering (a, b, c).

```rust
pub fn get_ordered_partition_by_indices(partition_by_exprs: &[std::sync::Arc<dyn PhysicalExpr>], input: &std::sync::Arc<dyn ExecutionPlan>) -> Vec<usize> { /* ... */ }
```

#### Function `get_best_fitting_window`

Constructs the best-fitting windowing operator (a `WindowAggExec` or a
`BoundedWindowExec`) for the given `input` according to the specifications
of `window_exprs` and `physical_partition_keys`. Here, best-fitting means
not requiring additional sorting and/or partitioning for the given input.
- A return value of `None` represents that there is no way to construct a
  windowing operator that doesn't need additional sorting/partitioning for
  the given input. Existing ordering should be changed to run the given
  windowing operation.
- A `Some(window exec)` value contains the optimal windowing operator (a
  `WindowAggExec` or a `BoundedWindowExec`) for the given input.

```rust
pub fn get_best_fitting_window(window_exprs: &[std::sync::Arc<dyn WindowExpr>], input: &std::sync::Arc<dyn ExecutionPlan>, physical_partition_keys: &[std::sync::Arc<dyn PhysicalExpr>]) -> datafusion_common::Result<Option<std::sync::Arc<dyn ExecutionPlan>>> { /* ... */ }
```

#### Function `get_window_mode`

Compares physical ordering (output ordering of the `input` operator) with
`partitionby_exprs` and `orderby_keys` to decide whether existing ordering
is sufficient to run the current window operator.
- A `None` return value indicates that we can not remove the sort in question
  (input ordering is not sufficient to run current window executor).
- A `Some((bool, InputOrderMode))` value indicates that the window operator
  can run with existing input ordering, so we can remove `SortExec` before it.

The `bool` field in the return value represents whether we should reverse window
operator to remove `SortExec` before it. The `InputOrderMode` field represents
the mode this window operator should work in to accommodate the existing ordering.

```rust
pub fn get_window_mode(partitionby_exprs: &[std::sync::Arc<dyn PhysicalExpr>], orderby_keys: &datafusion_physical_expr::LexOrdering, input: &std::sync::Arc<dyn ExecutionPlan>) -> Option<(bool, crate::InputOrderMode)> { /* ... */ }
```

### Re-exports

#### Re-export `BoundedWindowAggExec`

```rust
pub use bounded_window_agg_exec::BoundedWindowAggExec;
```

#### Re-export `PlainAggregateWindowExpr`

```rust
pub use datafusion_physical_expr::window::PlainAggregateWindowExpr;
```

#### Re-export `StandardWindowExpr`

```rust
pub use datafusion_physical_expr::window::StandardWindowExpr;
```

#### Re-export `WindowExpr`

```rust
pub use datafusion_physical_expr::window::WindowExpr;
```

#### Re-export `WindowAggExec`

```rust
pub use window_agg_exec::WindowAggExec;
```

## Module `work_table`

Defines the work table query plan

```rust
pub mod work_table { /* ... */ }
```

### Types

#### Struct `WorkTableExec`

A temporary "working table" operation where the input data will be
taken from the named handle during the execution and will be re-published
as is (kind of like a mirror).

Most notably used in the implementation of recursive queries where the
underlying relation does not exist yet but the data will come as the previous
term is evaluated. This table will be used such that the recursive plan
will register a receiver in the task context and this plan will use that
receiver to get the data and stream it back up so that the batches are available
in the next iteration.

```rust
pub struct WorkTableExec {
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
  pub fn new(name: String, schema: SchemaRef) -> Self { /* ... */ }
  ```
  Create a new execution plan for a worktable exec.

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Ref to name

- ```rust
  pub fn schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  Arc clone of ref to schema

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WorkTableExec { /* ... */ }
    ```

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **MaybeSendSync**
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
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn benefits_from_input_partitioning(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, _: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```
    Stream the batches that were written to the work table.

  - ```rust
    fn metrics(self: &Self) -> Option<MetricsSet> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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

## Module `udaf`

```rust
pub mod udaf { /* ... */ }
```

### Re-exports

#### Re-export `StatisticsArgs`

```rust
pub use datafusion_expr::StatisticsArgs;
```

#### Re-export `AggregateFunctionExpr`

```rust
pub use datafusion_physical_expr::aggregate::AggregateFunctionExpr;
```

## Module `coalesce`

```rust
pub mod coalesce { /* ... */ }
```

### Types

#### Struct `BatchCoalescer`

Concatenate multiple [`RecordBatch`]es

`BatchCoalescer` concatenates multiple small [`RecordBatch`]es, produced by
operations such as `FilterExec` and `RepartitionExec`, into larger ones for
more efficient processing by subsequent operations.

# Background

Generally speaking, larger [`RecordBatch`]es are more efficient to process
than smaller record batches (until the CPU cache is exceeded) because there
is fixed processing overhead per batch. DataFusion tries to operate on
batches of `target_batch_size` rows to amortize this overhead

```text
┌────────────────────┐
│    RecordBatch     │
│   num_rows = 23    │
└────────────────────┘                 ┌────────────────────┐
                                       │                    │
┌────────────────────┐     Coalesce    │                    │
│                    │      Batches    │                    │
│    RecordBatch     │                 │                    │
│   num_rows = 50    │  ─ ─ ─ ─ ─ ─ ▶  │                    │
│                    │                 │    RecordBatch     │
│                    │                 │   num_rows = 106   │
└────────────────────┘                 │                    │
                                       │                    │
┌────────────────────┐                 │                    │
│                    │                 │                    │
│    RecordBatch     │                 │                    │
│   num_rows = 33    │                 └────────────────────┘
│                    │
└────────────────────┘
```

# Notes:

1. Output rows are produced in the same order as the input rows

2. The output is a sequence of batches, with all but the last being at least
   `target_batch_size` rows.

3. Eventually this may also be able to handle other optimizations such as a
   combined filter/coalesce operation.


```rust
pub struct BatchCoalescer {
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
  pub fn new(schema: SchemaRef, target_batch_size: usize, fetch: Option<usize>) -> Self { /* ... */ }
  ```
  Create a new `BatchCoalescer`

- ```rust
  pub fn schema(self: &Self) -> SchemaRef { /* ... */ }
  ```
  Return the schema of the output batches

- ```rust
  pub fn push_batch(self: &mut Self, batch: RecordBatch) -> CoalescerState { /* ... */ }
  ```
  Push next batch, and returns [`CoalescerState`] indicating the current

- ```rust
  pub fn is_empty(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the there is no data buffered

- ```rust
  pub fn finish_batch(self: &mut Self) -> datafusion_common::Result<RecordBatch> { /* ... */ }
  ```
  Concatenates and returns all buffered batches, and clears the buffer.

###### Trait Implementations

- **Send**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Enum `CoalescerState`

Indicates the state of the [`BatchCoalescer`] buffer after the
[`BatchCoalescer::push_batch()`] operation.

The caller should take different actions, depending on the variant returned.

```rust
pub enum CoalescerState {
    Continue,
    LimitReached,
    TargetReached,
}
```

##### Variants

###### `Continue`

Neither the limit nor the target batch size is reached.

Action: continue pushing batches.

###### `LimitReached`

The limit has been reached.

Action: call [`BatchCoalescer::finish_batch()`] to get the final
buffered results as a batch and finish the query.

###### `TargetReached`

The specified minimum number of rows a batch should have is reached.

Action: call [`BatchCoalescer::finish_batch()`] to get the current
buffered results as a batch and then continue pushing batches.

##### Implementations

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

- **Allocation**
- **ErasedDestructor**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Unpin**
## Macros

### Macro `handle_state`

**Attributes:**

- `#[macro_export]`

The `handle_state` macro is designed to process the result of a state-changing
operation. It operates on a `StatefulStreamResult` by matching its variants and
executing corresponding actions. This macro is used to streamline code that deals
with state transitions, reducing boilerplate and improving readability.

# Cases

- `Ok(StatefulStreamResult::Continue)`: Continues the loop, indicating the
  stream join operation should proceed to the next step.
- `Ok(StatefulStreamResult::Ready(result))`: Returns a `Poll::Ready` with the
  result, either yielding a value or indicating the stream is awaiting more
  data.
- `Err(e)`: Returns a `Poll::Ready` containing an error, signaling an issue
  during the stream join operation.

# Arguments

* `$match_case`: An expression that evaluates to a `Result<StatefulStreamResult<_>>`.

```rust
pub macro_rules! handle_state {
    /* macro_rules! handle_state {
    ($match_case:expr) => { ... };
} */
}
```

## Re-exports

### Re-export `hash_utils`

```rust
pub use datafusion_common::hash_utils;
```

### Re-export `project_schema`

```rust
pub use datafusion_common::utils::project_schema;
```

### Re-export `internal_err`

```rust
pub use datafusion_common::internal_err;
```

### Re-export `ColumnStatistics`

```rust
pub use datafusion_common::ColumnStatistics;
```

### Re-export `Statistics`

```rust
pub use datafusion_common::Statistics;
```

### Re-export `RecordBatchStream`

```rust
pub use datafusion_execution::RecordBatchStream;
```

### Re-export `SendableRecordBatchStream`

```rust
pub use datafusion_execution::SendableRecordBatchStream;
```

### Re-export `Accumulator`

```rust
pub use datafusion_expr::Accumulator;
```

### Re-export `ColumnarValue`

```rust
pub use datafusion_expr::ColumnarValue;
```

### Re-export `WindowExpr`

```rust
pub use datafusion_physical_expr::window::WindowExpr;
```

### Re-export `expressions`

```rust
pub use datafusion_physical_expr::expressions;
```

### Re-export `Distribution`

```rust
pub use datafusion_physical_expr::Distribution;
```

### Re-export `Partitioning`

```rust
pub use datafusion_physical_expr::Partitioning;
```

### Re-export `PhysicalExpr`

```rust
pub use datafusion_physical_expr::PhysicalExpr;
```

### Re-export `DefaultDisplay`

```rust
pub use crate::display::DefaultDisplay;
```

### Re-export `DisplayAs`

```rust
pub use crate::display::DisplayAs;
```

### Re-export `DisplayFormatType`

```rust
pub use crate::display::DisplayFormatType;
```

### Re-export `VerboseDisplay`

```rust
pub use crate::display::VerboseDisplay;
```

### Re-export `collect`

```rust
pub use crate::execution_plan::collect;
```

### Re-export `collect_partitioned`

```rust
pub use crate::execution_plan::collect_partitioned;
```

### Re-export `displayable`

```rust
pub use crate::execution_plan::displayable;
```

### Re-export `execute_input_stream`

```rust
pub use crate::execution_plan::execute_input_stream;
```

### Re-export `execute_stream`

```rust
pub use crate::execution_plan::execute_stream;
```

### Re-export `execute_stream_partitioned`

```rust
pub use crate::execution_plan::execute_stream_partitioned;
```

### Re-export `get_plan_string`

```rust
pub use crate::execution_plan::get_plan_string;
```

### Re-export `with_new_children_if_necessary`

```rust
pub use crate::execution_plan::with_new_children_if_necessary;
```

### Re-export `ExecutionPlan`

```rust
pub use crate::execution_plan::ExecutionPlan;
```

### Re-export `ExecutionPlanProperties`

```rust
pub use crate::execution_plan::ExecutionPlanProperties;
```

### Re-export `PlanProperties`

```rust
pub use crate::execution_plan::PlanProperties;
```

### Re-export `Metric`

```rust
pub use crate::metrics::Metric;
```

### Re-export `InputOrderMode`

```rust
pub use crate::ordering::InputOrderMode;
```

### Re-export `EmptyRecordBatchStream`

```rust
pub use crate::stream::EmptyRecordBatchStream;
```

### Re-export `TopK`

```rust
pub use crate::topk::TopK;
```

### Re-export `accept`

```rust
pub use crate::visitor::accept;
```

### Re-export `visit_execution_plan`

```rust
pub use crate::visitor::visit_execution_plan;
```

### Re-export `ExecutionPlanVisitor`

```rust
pub use crate::visitor::ExecutionPlanVisitor;
```

### Re-export `SpillManager`

```rust
pub use spill::spill_manager::SpillManager;
```


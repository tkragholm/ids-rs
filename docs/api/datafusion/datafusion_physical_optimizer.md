# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_physical_optimizer`

## Modules

## Module `aggregate_statistics`

Utilizing exact statistics from sources to avoid scanning data

```rust
pub mod aggregate_statistics { /* ... */ }
```

### Types

#### Struct `AggregateStatistics`

Optimizer that uses available statistics for aggregate functions

```rust
pub struct AggregateStatistics {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Unpin**
- **ErasedDestructor**
- **Allocation**
- **Freeze**
- **Send**
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

- **Default**
  - ```rust
    fn default() -> AggregateStatistics { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```
    This rule will change the nullable properties of the schema, disable the schema check.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `coalesce_batches`

CoalesceBatches optimizer that groups batches together rows
in bigger batches to avoid overhead with small batches

```rust
pub mod coalesce_batches { /* ... */ }
```

### Types

#### Struct `CoalesceBatches`

Optimizer rule that introduces CoalesceBatchesExec to avoid overhead with small batches that
are produced by highly selective filters

```rust
pub struct CoalesceBatches {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **Send**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> CoalesceBatches { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **RefUnwindSafe**
- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `combine_partial_final_agg`

CombinePartialFinalAggregate optimizer rule checks the adjacent Partial and Final AggregateExecs
and try to combine them if necessary

```rust
pub mod combine_partial_final_agg { /* ... */ }
```

### Types

#### Struct `CombinePartialFinalAggregate`

CombinePartialFinalAggregate optimizer rule combines the adjacent Partial and Final AggregateExecs
into a Single AggregateExec if their grouping exprs and aggregate exprs equal.

This rule should be applied after the EnforceDistribution and EnforceSorting rules


```rust
pub struct CombinePartialFinalAggregate {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **Allocation**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> CombinePartialFinalAggregate { /* ... */ }
    ```

- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

## Module `enforce_distribution`

EnforceDistribution optimizer rule inspects the physical plan with respect
to distribution requirements and adds [`RepartitionExec`]s to satisfy them
when necessary. If increasing parallelism is beneficial (and also desirable
according to the configuration), this rule increases partition counts in
the physical plan.

```rust
pub mod enforce_distribution { /* ... */ }
```

### Types

#### Struct `EnforceDistribution`

The `EnforceDistribution` rule ensures that distribution requirements are
met. In doing so, this rule will increase the parallelism in the plan by
introducing repartitioning operators to the physical plan.

For example, given an input such as:


```text
┌─────────────────────────────────┐
│                                 │
│          ExecutionPlan          │
│                                 │
└─────────────────────────────────┘
            ▲         ▲
            │         │
      ┌─────┘         └─────┐
      │                     │
      │                     │
      │                     │
┌───────────┐         ┌───────────┐
│           │         │           │
│ batch A1  │         │ batch B1  │
│           │         │           │
├───────────┤         ├───────────┤
│           │         │           │
│ batch A2  │         │ batch B2  │
│           │         │           │
├───────────┤         ├───────────┤
│           │         │           │
│ batch A3  │         │ batch B3  │
│           │         │           │
└───────────┘         └───────────┘

     Input                 Input
       A                     B
```

This rule will attempt to add a `RepartitionExec` to increase parallelism
(to 3, in this case) and create the following arrangement:

```text
    ┌─────────────────────────────────┐
    │                                 │
    │          ExecutionPlan          │
    │                                 │
    └─────────────────────────────────┘
              ▲      ▲       ▲            Input now has 3
              │      │       │             partitions
      ┌───────┘      │       └───────┐
      │              │               │
      │              │               │
┌───────────┐  ┌───────────┐   ┌───────────┐
│           │  │           │   │           │
│ batch A1  │  │ batch A3  │   │ batch B3  │
│           │  │           │   │           │
├───────────┤  ├───────────┤   ├───────────┤
│           │  │           │   │           │
│ batch B2  │  │ batch B1  │   │ batch A2  │
│           │  │           │   │           │
└───────────┘  └───────────┘   └───────────┘
      ▲              ▲               ▲
      │              │               │
      └─────────┐    │    ┌──────────┘
                │    │    │
                │    │    │
    ┌─────────────────────────────────┐   batches are
    │       RepartitionExec(3)        │   repartitioned
    │           RoundRobin            │
    │                                 │
    └─────────────────────────────────┘
                ▲         ▲
                │         │
          ┌─────┘         └─────┐
          │                     │
          │                     │
          │                     │
    ┌───────────┐         ┌───────────┐
    │           │         │           │
    │ batch A1  │         │ batch B1  │
    │           │         │           │
    ├───────────┤         ├───────────┤
    │           │         │           │
    │ batch A2  │         │ batch B2  │
    │           │         │           │
    ├───────────┤         ├───────────┤
    │           │         │           │
    │ batch A3  │         │ batch B3  │
    │           │         │           │
    └───────────┘         └───────────┘


     Input                 Input
       A                     B
```

The `EnforceDistribution` rule
- is idempotent; i.e. it can be applied multiple times, each time producing
  the same result.
- always produces a valid plan in terms of distribution requirements. Its
  input plan can be valid or invalid with respect to distribution requirements,
  but the output plan will always be valid.
- produces a valid plan in terms of ordering requirements, *if* its input is
  a valid plan in terms of ordering requirements. If the input plan is invalid,
  this rule does not attempt to fix it as doing so is the responsibility of the
  `EnforceSorting` rule.

Note that distribution requirements are met in the strictest way. This may
result in more than strictly necessary [`RepartitionExec`]s in the plan, but
meeting the requirements in the strictest way may help avoid possible data
skew in joins.

For example for a hash join with keys (a, b, c), the required Distribution(a, b, c)
can be satisfied by several alternative partitioning ways: (a, b, c), (a, b),
(a, c), (b, c), (a), (b), (c) and ( ).

This rule only chooses the exact match and satisfies the Distribution(a, b, c)
by a HashPartition(a, b, c).

```rust
pub struct EnforceDistribution {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EnforceDistribution { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Type Alias `PlanWithKeyRequirements`

Keeps track of parent required key orderings.

```rust
pub type PlanWithKeyRequirements = datafusion_physical_plan::tree_node::PlanContext<Vec<std::sync::Arc<dyn PhysicalExpr>>>;
```

#### Type Alias `DistributionContext`

Keeps track of distribution changing operators (like `RepartitionExec`,
`SortPreservingMergeExec`, `CoalescePartitionsExec`) and their ancestors.
Using this information, we can optimize distribution of the plan if/when
necessary.

```rust
pub type DistributionContext = datafusion_physical_plan::tree_node::PlanContext<bool>;
```

### Functions

#### Function `adjust_input_keys_ordering`

When the physical planner creates the Joins, the ordering of join keys is from the original query.
That might not match with the output partitioning of the join node's children
A Top-Down process will use this method to adjust children's output partitioning based on the parent key reordering requirements:

Example:
    TopJoin on (a, b, c)
        bottom left join on(b, a, c)
        bottom right join on(c, b, a)

 Will be adjusted to:
    TopJoin on (a, b, c)
        bottom left join on(a, b, c)
        bottom right join on(a, b, c)

Example:
    TopJoin on (a, b, c)
        Agg1 group by (b, a, c)
        Agg2 group by (c, b, a)

Will be adjusted to:
    TopJoin on (a, b, c)
         Projection(b, a, c)
            Agg1 group by (a, b, c)
         Projection(c, b, a)
            Agg2 group by (a, b, c)

Following is the explanation of the reordering process:

1) If the current plan is Partitioned HashJoin, SortMergeJoin, check whether the requirements can be satisfied by adjusting join keys ordering:
   Requirements can not be satisfied, clear the current requirements, generate new requirements(to pushdown) based on the current join keys, return the unchanged plan.
   Requirements is already satisfied, clear the current requirements, generate new requirements(to pushdown) based on the current join keys, return the unchanged plan.
   Requirements can be satisfied by adjusting keys ordering, clear the current requirements, generate new requirements(to pushdown) based on the adjusted join keys, return the changed plan.

2) If the current plan is Aggregation, check whether the requirements can be satisfied by adjusting group by keys ordering:
   Requirements can not be satisfied, clear all the requirements, return the unchanged plan.
   Requirements is already satisfied, clear all the requirements, return the unchanged plan.
   Requirements can be satisfied by adjusting keys ordering, clear all the requirements, return the changed plan.

3) If the current plan is RepartitionExec, CoalescePartitionsExec or WindowAggExec, clear all the requirements, return the unchanged plan
4) If the current plan is Projection, transform the requirements to the columns before the Projection and push down requirements
5) For other types of operators, by default, pushdown the parent requirements to children.


```rust
pub fn adjust_input_keys_ordering(requirements: PlanWithKeyRequirements) -> datafusion_common::error::Result<datafusion_common::tree_node::Transformed<PlanWithKeyRequirements>> { /* ... */ }
```

#### Function `reorder_partitioned_join_keys`

```rust
pub fn reorder_partitioned_join_keys<F>(join_plan: PlanWithKeyRequirements, on: &[(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)], sort_options: &[arrow::compute::SortOptions], join_constructor: &F) -> datafusion_common::error::Result<PlanWithKeyRequirements>
where
    F: Fn((Vec<(datafusion_physical_expr::PhysicalExprRef, datafusion_physical_expr::PhysicalExprRef)>, Vec<arrow::compute::SortOptions>)) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>> { /* ... */ }
```

#### Function `reorder_aggregate_keys`

```rust
pub fn reorder_aggregate_keys(agg_node: PlanWithKeyRequirements, agg_exec: &datafusion_physical_plan::aggregates::AggregateExec) -> datafusion_common::error::Result<PlanWithKeyRequirements> { /* ... */ }
```

#### Function `reorder_join_keys_to_inputs`

When the physical planner creates the Joins, the ordering of join keys is from the original query.
That might not match with the output partitioning of the join node's children
This method will try to change the ordering of the join keys to match with the
partitioning of the join nodes' children. If it can not match with both sides, it will try to
match with one, either the left side or the right side.

Example:
    TopJoin on (a, b, c)
        bottom left join on(b, a, c)
        bottom right join on(c, b, a)

 Will be adjusted to:
    TopJoin on (b, a, c)
        bottom left join on(b, a, c)
        bottom right join on(c, b, a)

Compared to the Top-Down reordering process, this Bottom-Up approach is much simpler, but might not reach a best result.
The Bottom-Up approach will be useful in future if we plan to support storage partition-wised Joins.
In that case, the datasources/tables might be pre-partitioned and we can't adjust the key ordering of the datasources
and then can't apply the Top-Down reordering process.

```rust
pub fn reorder_join_keys_to_inputs(plan: std::sync::Arc<dyn ExecutionPlan>) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>> { /* ... */ }
```

#### Function `ensure_distribution`

This function checks whether we need to add additional data exchange
operators to satisfy distribution requirements. Since this function
takes care of such requirements, we should avoid manually adding data
exchange operators in other places.

```rust
pub fn ensure_distribution(dist_context: DistributionContext, config: &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<datafusion_common::tree_node::Transformed<DistributionContext>> { /* ... */ }
```

## Module `enforce_sorting`

EnforceSorting optimizer rule inspects the physical plan with respect
to local sorting requirements and does the following:
- Adds a [`SortExec`] when a requirement is not met,
- Removes an already-existing [`SortExec`] if it is possible to prove
  that this sort is unnecessary

The rule can work on valid *and* invalid physical plans with respect to
sorting requirements, but always produces a valid physical plan in this sense.

A non-realistic but easy to follow example for sort removals: Assume that we
somehow get the fragment

```text
SortExec: expr=[nullable_col@0 ASC]
  SortExec: expr=[non_nullable_col@1 ASC]
```

in the physical plan. The first sort is unnecessary since its result is overwritten
by another [`SortExec`]. Therefore, this rule removes it from the physical plan.

```rust
pub mod enforce_sorting { /* ... */ }
```

### Modules

## Module `replace_with_order_preserving_variants`

Optimizer rule that replaces executors that lose ordering with their
order-preserving variants when it is helpful; either in terms of
performance or to accommodate unbounded streams by fixing the pipeline.

```rust
pub mod replace_with_order_preserving_variants { /* ... */ }
```

### Types

#### Type Alias `OrderPreservationContext`

For a given `plan`, this object carries the information one needs from its
descendants to decide whether it is beneficial to replace order-losing (but
somewhat faster) variants of certain operators with their order-preserving
(but somewhat slower) cousins.

```rust
pub type OrderPreservationContext = datafusion_physical_plan::tree_node::PlanContext<bool>;
```

### Functions

#### Function `update_order_preservation_ctx_children_data`

Updates order-preservation data for all children of the given node.

```rust
pub fn update_order_preservation_ctx_children_data(opc: &mut OrderPreservationContext) { /* ... */ }
```

#### Function `replace_with_order_preserving_variants`

The `replace_with_order_preserving_variants` optimizer sub-rule tries to
remove `SortExec`s from the physical plan by replacing operators that do
not preserve ordering with their order-preserving variants; i.e. by replacing
ordinary `RepartitionExec`s with their sort-preserving variants or by replacing
`CoalescePartitionsExec`s with `SortPreservingMergeExec`s.

If this replacement is helpful for removing a `SortExec`, it updates the plan.
Otherwise, it leaves the plan unchanged.

NOTE: This optimizer sub-rule will only produce sort-preserving `RepartitionExec`s
if the query is bounded or if the config option `prefer_existing_sort` is
set to `true`.

The algorithm flow is simply like this:
1. Visit nodes of the physical plan bottom-up and look for `SortExec` nodes.
   During the traversal, keep track of operators that maintain ordering (or
   can maintain ordering when replaced by an order-preserving variant) until
   a `SortExec` is found.
2. When a `SortExec` is found, update the child of the `SortExec` by replacing
   operators that do not preserve ordering in the tree with their order
   preserving variants.
3. Check if the `SortExec` is still necessary in the updated plan by comparing
   its input ordering with the output ordering it imposes. We do this because
   replacing operators that lose ordering with their order-preserving variants
   enables us to preserve the previously lost ordering at the input of `SortExec`.
4. If the `SortExec` in question turns out to be unnecessary, remove it and
   use updated plan. Otherwise, use the original plan.
5. Continue the bottom-up traversal until another `SortExec` is seen, or the
   traversal is complete.

```rust
pub fn replace_with_order_preserving_variants(requirements: OrderPreservationContext, is_spr_better: bool, is_spm_better: bool, config: &datafusion_common::config::ConfigOptions) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<OrderPreservationContext>> { /* ... */ }
```

## Module `sort_pushdown`

```rust
pub mod sort_pushdown { /* ... */ }
```

### Types

#### Struct `ParentRequirements`

This is a "data class" we use within the [`EnforceSorting`] rule to push
down [`SortExec`] in the plan. In some cases, we can reduce the total
computational cost by pushing down `SortExec`s through some executors. The
object carries the parent required ordering and the (optional) `fetch` value
of the parent node as its data.

[`EnforceSorting`]: crate::enforce_sorting::EnforceSorting

```rust
pub struct ParentRequirements {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParentRequirements { /* ... */ }
    ```

- **Send**
- **Sync**
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

- **UnwindSafe**
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

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
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
    fn default() -> ParentRequirements { /* ... */ }
    ```

#### Type Alias `SortPushDown`

```rust
pub type SortPushDown = datafusion_physical_plan::tree_node::PlanContext<ParentRequirements>;
```

### Functions

#### Function `assign_initial_requirements`

Assigns the ordering requirement of the root node to the its children.

```rust
pub fn assign_initial_requirements(sort_push_down: &mut SortPushDown) { /* ... */ }
```

#### Function `pushdown_sorts`

```rust
pub fn pushdown_sorts(sort_push_down: SortPushDown) -> datafusion_common::Result<SortPushDown> { /* ... */ }
```

### Types

#### Struct `EnforceSorting`

This rule inspects [`SortExec`]'s in the given physical plan in order to
remove unnecessary sorts, and optimize sort performance across the plan.

```rust
pub struct EnforceSorting {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

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

- **Unpin**
- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **Default**
  - ```rust
    fn default() -> EnforceSorting { /* ... */ }
    ```

- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

#### Type Alias `PlanWithCorrespondingSort`

This context object is used within the [`EnforceSorting`] rule to track the closest
[`SortExec`] descendant(s) for every child of a plan. The data attribute
stores whether the plan is a `SortExec` or is connected to a `SortExec`
via its children.

```rust
pub type PlanWithCorrespondingSort = datafusion_physical_plan::tree_node::PlanContext<bool>;
```

#### Type Alias `PlanWithCorrespondingCoalescePartitions`

This object is used within the [`EnforceSorting`] rule to track the closest
[`CoalescePartitionsExec`] descendant(s) for every child of a plan. The data
attribute stores whether the plan is a `CoalescePartitionsExec` or is
connected to a `CoalescePartitionsExec` via its children.

The tracker halts at each [`SortExec`] (where the SPM will act to replace the coalesce).

This requires a bottom-up traversal was previously performed, updating the
children previously.

```rust
pub type PlanWithCorrespondingCoalescePartitions = datafusion_physical_plan::tree_node::PlanContext<bool>;
```

### Functions

#### Function `parallelize_sorts`

Transform [`CoalescePartitionsExec`] + [`SortExec`] into
[`SortExec`] + [`SortPreservingMergeExec`] as illustrated below:

The [`CoalescePartitionsExec`] + [`SortExec`] cascades
combine the partitions first, and then sort:
```text
  ┌ ─ ─ ─ ─ ─ ┐                                                                                   
   ┌─┬─┬─┐                                                                                        
  ││B│A│D│... ├──┐                                                                                
   └─┴─┴─┘       │                                                                                
  └ ─ ─ ─ ─ ─ ┘  │  ┌────────────────────────┐   ┌ ─ ─ ─ ─ ─ ─ ┐   ┌────────┐    ┌ ─ ─ ─ ─ ─ ─ ─ ┐
   Partition 1   │  │        Coalesce        │    ┌─┬─┬─┬─┬─┐      │        │     ┌─┬─┬─┬─┬─┐     
                 ├──▶(no ordering guarantees)│──▶││B│E│A│D│C│...───▶  Sort  ├───▶││A│B│C│D│E│... │
                 │  │                        │    └─┴─┴─┴─┴─┘      │        │     └─┴─┴─┴─┴─┘     
  ┌ ─ ─ ─ ─ ─ ┐  │  └────────────────────────┘   └ ─ ─ ─ ─ ─ ─ ┘   └────────┘    └ ─ ─ ─ ─ ─ ─ ─ ┘
   ┌─┬─┐         │                                 Partition                       Partition      
  ││E│C│ ...  ├──┘                                                                                
   └─┴─┘                                                                                          
  └ ─ ─ ─ ─ ─ ┘                                                                                   
   Partition 2                                                                                    
```                                                                                                 


The [`SortExec`] + [`SortPreservingMergeExec`] cascades
sorts each partition first, then merge partitions while retaining the sort:
```text
  ┌ ─ ─ ─ ─ ─ ┐   ┌────────┐   ┌ ─ ─ ─ ─ ─ ┐                                                 
   ┌─┬─┬─┐        │        │    ┌─┬─┬─┐                                                      
  ││B│A│D│... │──▶│  Sort  │──▶││A│B│D│... │──┐                                              
   └─┴─┴─┘        │        │    └─┴─┴─┘       │                                              
  └ ─ ─ ─ ─ ─ ┘   └────────┘   └ ─ ─ ─ ─ ─ ┘  │  ┌─────────────────────┐    ┌ ─ ─ ─ ─ ─ ─ ─ ┐
   Partition 1                  Partition 1   │  │                     │     ┌─┬─┬─┬─┬─┐     
                                              ├──▶ SortPreservingMerge ├───▶││A│B│C│D│E│... │
                                              │  │                     │     └─┴─┴─┴─┴─┘     
  ┌ ─ ─ ─ ─ ─ ┐   ┌────────┐   ┌ ─ ─ ─ ─ ─ ┐  │  └─────────────────────┘    └ ─ ─ ─ ─ ─ ─ ─ ┘
   ┌─┬─┐          │        │    ┌─┬─┐         │                               Partition      
  ││E│C│ ...  │──▶│  Sort  ├──▶││C│E│ ...  │──┘                                              
   └─┴─┘          │        │    └─┴─┘                                                        
  └ ─ ─ ─ ─ ─ ┘   └────────┘   └ ─ ─ ─ ─ ─ ┘                                                 
   Partition 2                  Partition 2                                                  
```

The latter [`SortExec`] + [`SortPreservingMergeExec`] cascade performs the
sort first on a per-partition basis, thereby parallelizing the sort.


The outcome is that plans of the form
```text
     "SortExec: expr=\[a@0 ASC\]",
     "  ...nodes..."
     "    CoalescePartitionsExec",
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```
are transformed into
```text
     "SortPreservingMergeExec: \[a@0 ASC\]",
     "  SortExec: expr=\[a@0 ASC\]",
     "    ...nodes..."
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```
by following connections from [`CoalescePartitionsExec`]s to [`SortExec`]s.
By performing sorting in parallel, we can increase performance in some scenarios.

This requires that there are no nodes between the [`SortExec`] and [`CoalescePartitionsExec`]
which require single partitioning. Do not parallelize when the following scenario occurs:
```text
     "SortExec: expr=\[a@0 ASC\]",
     "  ...nodes requiring single partitioning..."
     "    CoalescePartitionsExec",
     "      RepartitionExec: partitioning=RoundRobinBatch(8), input_partitions=1",
```

```rust
pub fn parallelize_sorts(requirements: PlanWithCorrespondingCoalescePartitions) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<PlanWithCorrespondingCoalescePartitions>> { /* ... */ }
```

#### Function `ensure_sorting`

This function enforces sorting requirements and makes optimizations without
violating these requirements whenever possible. Requires a bottom-up traversal.

```rust
pub fn ensure_sorting(requirements: PlanWithCorrespondingSort) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<PlanWithCorrespondingSort>> { /* ... */ }
```

## Module `join_selection`

The [`JoinSelection`] rule tries to modify a given plan so that it can
accommodate infinite sources and utilize statistical information (if there
is any) to obtain more performant plans. To achieve the first goal, it
tries to transform a non-runnable query (with the given infinite sources)
into a runnable query by replacing pipeline-breaking join operations with
pipeline-friendly ones. To achieve the second goal, it selects the proper
`PartitionMode` and the build side using the available statistics for hash joins.

```rust
pub mod join_selection { /* ... */ }
```

### Types

#### Struct `JoinSelection`

The [`JoinSelection`] rule tries to modify a given plan so that it can
accommodate infinite sources and optimize joins in the plan according to
available statistical information, if there is any.

```rust
pub struct JoinSelection {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> JoinSelection { /* ... */ }
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
- **Unpin**
- **Freeze**
- **IntoEither**
- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
#### Type Alias `PipelineFixerSubrule`

Pipeline-fixing join selection subrule.

```rust
pub type PipelineFixerSubrule = dyn Fn(std::sync::Arc<dyn ExecutionPlan>, &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>>;
```

### Functions

#### Function `swap_hash_join`

**Attributes:**

- `#[deprecated(since = "45.0.0", note =
"use HashJoinExec::swap_inputs instead")]`

**⚠️ Deprecated since 45.0.0**: use HashJoinExec::swap_inputs instead

This function swaps the inputs of the given join operator.
This function is public so other downstream projects can use it
to construct `HashJoinExec` with right side as the build side.

```rust
pub fn swap_hash_join(hash_join: &datafusion_physical_plan::joins::HashJoinExec, partition_mode: datafusion_physical_plan::joins::PartitionMode) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>> { /* ... */ }
```

#### Function `hash_join_swap_subrule`

This subrule will swap build/probe sides of a hash join depending on whether
one of its inputs may produce an infinite stream of records. The rule ensures
that the left (build) side of the hash join always operates on an input stream
that will produce a finite set of records. If the left side can not be chosen
to be "finite", the join sides stay the same as the original query.
```text
For example, this rule makes the following transformation:



          +--------------+              +--------------+
          |              |  unbounded   |              |
   Left   | Infinite     |    true      | Hash         |\true
          | Data source  |--------------| Repartition  | \   +--------------+       +--------------+
          |              |              |              |  \  |              |       |              |
          +--------------+              +--------------+   - |  Hash Join   |-------| Projection   |
                                                           - |              |       |              |
          +--------------+              +--------------+  /  +--------------+       +--------------+
          |              |  unbounded   |              | /
   Right  | Finite       |    false     | Hash         |/false
          | Data Source  |--------------| Repartition  |
          |              |              |              |
          +--------------+              +--------------+



          +--------------+              +--------------+
          |              |  unbounded   |              |
   Left   | Finite       |    false     | Hash         |\false
          | Data source  |--------------| Repartition  | \   +--------------+       +--------------+
          |              |              |              |  \  |              | true  |              | true
          +--------------+              +--------------+   - |  Hash Join   |-------| Projection   |-----
                                                           - |              |       |              |
          +--------------+              +--------------+  /  +--------------+       +--------------+
          |              |  unbounded   |              | /
   Right  | Infinite     |    true      | Hash         |/true
          | Data Source  |--------------| Repartition  |
          |              |              |              |
          +--------------+              +--------------+

```

```rust
pub fn hash_join_swap_subrule(input: std::sync::Arc<dyn ExecutionPlan>, _config_options: &datafusion_common::config::ConfigOptions) -> datafusion_common::error::Result<std::sync::Arc<dyn ExecutionPlan>> { /* ... */ }
```

## Module `limit_pushdown`

[`LimitPushdown`] pushes `LIMIT` down through `ExecutionPlan`s to reduce
data transfer as much as possible.

```rust
pub mod limit_pushdown { /* ... */ }
```

### Types

#### Struct `LimitPushdown`

This rule inspects [`ExecutionPlan`]'s and pushes down the fetch limit from
the parent to the child if applicable.

```rust
pub struct LimitPushdown {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

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

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **IntoEither**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> LimitPushdown { /* ... */ }
    ```

#### Struct `GlobalRequirements`

This is a "data class" we use within the [`LimitPushdown`] rule to push
down [`LimitExec`] in the plan. GlobalRequirements are hold as a rule-wide state
and holds the fetch and skip information. The struct also has a field named
satisfied which means if the "current" plan is valid in terms of limits or not.

For example: If the plan is satisfied with current fetch info, we decide to not add a LocalLimit

[`LimitPushdown`]: crate::limit_pushdown::LimitPushdown
[`LimitExec`]: crate::limit_pushdown::LimitExec

```rust
pub struct GlobalRequirements {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Allocation**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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
    fn clone(self: &Self) -> GlobalRequirements { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **UnwindSafe**
- **Unpin**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> GlobalRequirements { /* ... */ }
    ```

#### Enum `LimitExec`

This enumeration makes `skip` and `fetch` calculations easier by providing
a single API for both local and global limit operators.

```rust
pub enum LimitExec {
    Global(datafusion_physical_plan::limit::GlobalLimitExec),
    Local(datafusion_physical_plan::limit::LocalLimitExec),
}
```

##### Variants

###### `Global`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_physical_plan::limit::GlobalLimitExec` |  |

###### `Local`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_physical_plan::limit::LocalLimitExec` |  |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(limit_exec: LimitExec) -> Self { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Sync**
- **IntoEither**
- **Freeze**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Functions

#### Function `pushdown_limit_helper`

This function is the main helper function of the `LimitPushDown` rule.
The helper takes an `ExecutionPlan` and a global (algorithm) state which is
an instance of `GlobalRequirements` and modifies these parameters while
checking if the limits can be pushed down or not.

If a limit is encountered, a [`TreeNodeRecursion::Stop`] is returned. Otherwise,
return a [`TreeNodeRecursion::Continue`].

```rust
pub fn pushdown_limit_helper(pushdown_plan: std::sync::Arc<dyn ExecutionPlan>, global_state: GlobalRequirements) -> datafusion_common::error::Result<(datafusion_common::tree_node::Transformed<std::sync::Arc<dyn ExecutionPlan>>, GlobalRequirements)> { /* ... */ }
```

## Module `limited_distinct_aggregation`

A special-case optimizer rule that pushes limit into a grouped aggregation
which has no aggregate expressions or sorting requirements

```rust
pub mod limited_distinct_aggregation { /* ... */ }
```

### Types

#### Struct `LimitedDistinctAggregation`

An optimizer rule that passes a `limit` hint into grouped aggregations which don't require all
rows in the group to be processed for correctness. Example queries fitting this description are:
- `SELECT distinct l_orderkey FROM lineitem LIMIT 10;`
- `SELECT l_orderkey FROM lineitem GROUP BY l_orderkey LIMIT 10;`

```rust
pub struct LimitedDistinctAggregation {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new `LimitedDistinctAggregation`

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

## Module `optimizer`

Physical optimizer traits

```rust
pub mod optimizer { /* ... */ }
```

### Types

#### Struct `PhysicalOptimizer`

A rule-based physical optimizer.

```rust
pub struct PhysicalOptimizer {
    pub rules: Vec<std::sync::Arc<dyn PhysicalOptimizerRule + Send + Sync>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rules` | `Vec<std::sync::Arc<dyn PhysicalOptimizerRule + Send + Sync>>` | All rules to apply |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new optimizer using the recommended list of rules

- ```rust
  pub fn with_rules(rules: Vec<Arc<dyn PhysicalOptimizerRule + Send + Sync>>) -> Self { /* ... */ }
  ```
  Create a new optimizer with the given rules

###### Trait Implementations

- **Send**
- **ErasedDestructor**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PhysicalOptimizer { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `PhysicalOptimizerRule`

`PhysicalOptimizerRule` transforms one ['ExecutionPlan'] into another which
computes the same results, but in a potentially more efficient way.

Use [`SessionState::add_physical_optimizer_rule`] to register additional
`PhysicalOptimizerRule`s.

[`SessionState::add_physical_optimizer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_physical_optimizer_rule

```rust
pub trait PhysicalOptimizerRule: Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `optimize`: Rewrite `plan` to an optimized form
- `name`: A human readable name for this optimizer rule
- `schema_check`: A flag to indicate whether the physical planner should valid the rule will not

##### Implementations

This trait is implemented for the following types:

- `AggregateStatistics`
- `CoalesceBatches`
- `CombinePartialFinalAggregate`
- `EnforceDistribution`
- `EnforceSorting`
- `JoinSelection`
- `LimitPushdown`
- `LimitedDistinctAggregation`
- `OutputRequirements`
- `ProjectionPushdown`
- `SanityCheckPlan`
- `TopKAggregation`
- `OptimizeAggregateOrder`

## Module `output_requirements`

The GlobalOrderRequire optimizer rule either:
- Adds an auxiliary `OutputRequirementExec` operator to keep track of global
  ordering and distribution requirement across rules, or
- Removes the auxiliary `OutputRequirementExec` operator from the physical plan.
  Since the `OutputRequirementExec` operator is only a helper operator, it
  shouldn't occur in the final plan (i.e. the executed plan).

```rust
pub mod output_requirements { /* ... */ }
```

### Types

#### Struct `OutputRequirements`

This rule either adds or removes [`OutputRequirements`]s to/from the physical
plan according to its `mode` attribute, which is set by the constructors
`new_add_mode` and `new_remove_mode`. With this rule, we can keep track of
the global requirements (ordering and distribution) across rules.

The primary use case of this node and rule is to specify and preserve the desired output
ordering and distribution the entire plan. When sending to a single client, a single partition may
be desirable, but when sending to a multi-partitioned writer, keeping multiple partitions may be
better.

```rust
pub struct OutputRequirements {
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
  pub fn new_add_mode() -> Self { /* ... */ }
  ```
  Create a new rule which works in `Add` mode; i.e. it simply adds a

- ```rust
  pub fn new_remove_mode() -> Self { /* ... */ }
  ```
  Create a new rule which works in `Remove` mode; i.e. it simply removes

###### Trait Implementations

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Allocation**
- **IntoEither**
- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
#### Struct `OutputRequirementExec`

An ancillary, non-executable operator whose sole purpose is to track global
requirements during optimization. It imposes
- the ordering requirement in its `order_requirement` attribute.
- the distribution requirement in its `dist_requirement` attribute.

See [`OutputRequirements`] for more details

```rust
pub struct OutputRequirementExec {
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
  pub fn new(input: Arc<dyn ExecutionPlan>, requirements: Option<LexRequirement>, dist_requirement: Distribution) -> Self { /* ... */ }
  ```

- ```rust
  pub fn input(self: &Self) -> Arc<dyn ExecutionPlan> { /* ... */ }
  ```

###### Trait Implementations

- **DisplayAs**
  - ```rust
    fn fmt_as(self: &Self, t: DisplayFormatType, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **UnwindSafe**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ExecutionPlan**
  - ```rust
    fn name(self: &Self) -> &''static str { /* ... */ }
    ```

  - ```rust
    fn as_any(self: &Self) -> &dyn std::any::Any { /* ... */ }
    ```

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
    fn maintains_input_order(self: &Self) -> Vec<bool> { /* ... */ }
    ```

  - ```rust
    fn children(self: &Self) -> Vec<&Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn required_input_ordering(self: &Self) -> Vec<Option<LexRequirement>> { /* ... */ }
    ```

  - ```rust
    fn with_new_children(self: Arc<Self>, children: Vec<Arc<dyn ExecutionPlan>>) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn execute(self: &Self, _partition: usize, _context: Arc<TaskContext>) -> Result<SendableRecordBatchStream> { /* ... */ }
    ```

  - ```rust
    fn statistics(self: &Self) -> Result<Statistics> { /* ... */ }
    ```

  - ```rust
    fn try_swapping_with_projection(self: &Self, projection: &ProjectionExec) -> Result<Option<Arc<dyn ExecutionPlan>>> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
## Module `projection_pushdown`

This file implements the `ProjectionPushdown` physical optimization rule.
The function [`remove_unnecessary_projections`] tries to push down all
projections one by one if the operator below is amenable to this. If a
projection reaches a source, it can even disappear from the plan entirely.

```rust
pub mod projection_pushdown { /* ... */ }
```

### Types

#### Struct `ProjectionPushdown`

This rule inspects `ProjectionExec`'s in the given physical plan and tries to
remove or swap with its child.

```rust
pub struct ProjectionPushdown {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **ErasedDestructor**
- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **Sync**
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

- **Default**
  - ```rust
    fn default() -> ProjectionPushdown { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **IntoEither**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `pruning`

[`PruningPredicate`] to apply filter [`Expr`] to prune "containers"
based on statistics (e.g. Parquet Row Groups)

[`Expr`]: https://docs.rs/datafusion/latest/datafusion/logical_expr/enum.Expr.html

```rust
pub mod pruning { /* ... */ }
```

### Types

#### Struct `PruningPredicate`

Used to prove that arbitrary predicates (boolean expression) can not
possibly evaluate to `true` given information about a column provided by
[`PruningStatistics`].

# Introduction

`PruningPredicate` analyzes filter expressions using statistics such as
min/max values and null counts, attempting to prove a "container" (e.g.
Parquet Row Group) can be skipped without reading the actual data,
potentially leading to significant performance improvements.

For example, `PruningPredicate`s are used to prune Parquet Row Groups based
on the min/max values found in the Parquet metadata. If the
`PruningPredicate` can prove that the filter can never evaluate to `true`
for any row in the Row Group, the entire Row Group is skipped during query
execution.

The `PruningPredicate` API is general, and can be used for pruning other
types of containers (e.g. files) based on statistics that may be known from
external catalogs (e.g. Delta Lake) or other sources. How this works is a
subtle topic.  See the Background and Implementation section for details.

`PruningPredicate` supports:

1. Arbitrary expressions (including user defined functions)

2. Vectorized evaluation (provide more than one set of statistics at a time)
   so it is suitable for pruning 1000s of containers.

3. Any source of information that implements the [`PruningStatistics`] trait
   (not just Parquet metadata).

# Example

See the [`pruning.rs` example in the `datafusion-examples`] for a complete
example of how to use `PruningPredicate` to prune files based on min/max
values.

[`pruning.rs` example in the `datafusion-examples`]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/pruning.rs

Given an expression like `x = 5` and statistics for 3 containers (Row
Groups, files, etc) `A`, `B`, and `C`:

```text
  A: {x_min = 0, x_max = 4}
  B: {x_min = 2, x_max = 10}
  C: {x_min = 5, x_max = 8}
```

`PruningPredicate` will conclude that the rows in container `A` can never
be true (as the maximum value is only `4`), so it can be pruned:

```text
A: false (no rows could possibly match x = 5)
B: true  (rows might match x = 5)
C: true  (rows might match x = 5)
```

See [`PruningPredicate::try_new`] and [`PruningPredicate::prune`] for more information.

# Background

## Boolean Tri-state logic

To understand the details of the rest of this documentation, it is important
to understand how the tri-state boolean logic in SQL works. As this is
somewhat esoteric, we review it here.

SQL has a notion of `NULL` that represents the value is `“unknown”` and this
uncertainty propagates through expressions. SQL `NULL` behaves very
differently than the `NULL` in most other languages where it is a special,
sentinel value (e.g. `0` in `C/C++`). While representing uncertainty with
`NULL` is powerful and elegant, SQL `NULL`s are often deeply confusing when
first encountered as they behave differently than most programmers may
expect.

In most other programming languages,
* `a == NULL` evaluates to `true` if `a` also had the value `NULL`
* `a == NULL` evaluates to `false` if `a` has any other value

However, in SQL `a = NULL` **always** evaluates to `NULL` (never `true` or
`false`):

Expression    | Result
------------- | ---------
`1 = NULL`    | `NULL`
`NULL = NULL` | `NULL`

Also important is how `AND` and `OR` works with tri-state boolean logic as
(perhaps counterintuitively) the result is **not** always NULL. While
consistent with the notion of `NULL` representing “unknown”, this is again,
often deeply confusing 🤯 when first encountered.

Expression       | Result    | Intuition
---------------  | --------- | -----------
`NULL AND true`  |   `NULL`  | The `NULL` stands for “unknown” and if it were `true` or `false` the overall expression value could change
`NULL AND false` |  `false`  | If the `NULL` was either `true` or `false` the overall expression is still `false`
`NULL AND NULL`  | `NULL`    |

Expression      | Result    | Intuition
--------------- | --------- | ----------
`NULL OR true`  | `true`    |  If the `NULL` was either `true` or `false` the overall expression is still `true`
`NULL OR false` | `NULL`    |  The `NULL` stands for “unknown” and if it were `true` or `false` the overall expression value could change
`NULL OR NULL`  |  `NULL`   |

## SQL Filter Semantics

The SQL `WHERE` clause has a boolean expression, often called a filter or
predicate. The semantics of this predicate are that the query evaluates the
predicate for each row in the input tables and:

* Rows that evaluate to `true` are returned in the query results

* Rows that evaluate to `false` are not returned (“filtered out” or “pruned” or “skipped”).

* Rows that evaluate to `NULL` are **NOT** returned (also “filtered out”).
  Note: *this treatment of `NULL` is **DIFFERENT** than how `NULL` is treated
  in the rewritten predicate described below.*

# `PruningPredicate` Implementation

Armed with the information in the Background section, we can now understand
how the `PruningPredicate` logic works.

## Interface

**Inputs**
1. An input schema describing what columns exist

2. A predicate (expression that evaluates to a boolean)

3. [`PruningStatistics`] that provides information about columns in that
   schema, for multiple “containers”. For each column in each container, it
   provides optional information on contained values, min_values, max_values,
   null_counts counts, and row_counts counts.

**Outputs**:
A (non null) boolean value for each container:
* `true`: There MAY be rows that match the predicate

* `false`: There are no rows that could possibly match the predicate (the
  predicate can never possibly be true). The container can be pruned (skipped)
  entirely.

While `PruningPredicate` will never return a `NULL` value, the
rewritten predicate (as returned by `build_predicate_expression` and used internally
by `PruningPredicate`) may evaluate to `NULL` when some of the min/max values
or null / row counts are not known.

In order to be correct, `PruningPredicate` must return false
**only** if it can determine that for all rows in the container, the
predicate could never evaluate to `true` (always evaluates to either `NULL`
or `false`).

## Contains Analysis and Min/Max Rewrite

`PruningPredicate` works by first analyzing the predicate to see what
[`LiteralGuarantee`] must hold for the predicate to be true.

Then, the `PruningPredicate` rewrites the original predicate into an
expression that references the min/max values of each column in the original
predicate.

When the min/max values are actually substituted in to this expression and
evaluated, the result means

* `true`: there MAY be rows that pass the predicate, **KEEPS** the container

* `NULL`: there MAY be rows that pass the predicate, **KEEPS** the container
  Note that rewritten predicate can evaluate to NULL when some of
  the min/max values are not known. *Note that this is different than
  the SQL filter semantics where `NULL` means the row is filtered
  out.*

* `false`: there are no rows that could possibly match the predicate,
  **PRUNES** the container

For example, given a column `x`, the `x_min`, `x_max`, `x_null_count`, and
`x_row_count` represent the minimum and maximum values, the null count of
column `x`, and the row count of column `x`, provided by the `PruningStatistics`.
`x_null_count` and `x_row_count` are used to handle the case where the column `x`
is known to be all `NULL`s. Note this is different from knowing nothing about
the column `x`, which confusingly is encoded by returning `NULL` for the min/max
values from [`PruningStatistics::max_values`] and [`PruningStatistics::min_values`].

Here are some examples of the rewritten predicates:

Original Predicate | Rewritten Predicate
------------------ | --------------------
`x = 5` | `x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)`
`x < 5` | `x_null_count != x_row_count THEN false (x_max < 5)`
`x = 5 AND y = 10` | `x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max) AND y_null_count != y_row_count (y_min <= 10 AND 10 <= y_max)`
`x IS NULL`  | `x_null_count > 0`
`x IS NOT NULL`  | `x_null_count != row_count`
`CAST(x as int) = 5` | `x_null_count != x_row_count (CAST(x_min as int) <= 5 AND 5 <= CAST(x_max as int))`

## Predicate Evaluation
The PruningPredicate works in two passes

**First pass**:  For each `LiteralGuarantee` calls
[`PruningStatistics::contained`] and rules out containers where the
LiteralGuarantees are not satisfied

**Second Pass**: Evaluates the rewritten expression using the
min/max/null_counts/row_counts values for each column for each container. For any
container that this expression evaluates to `false`, it rules out those
containers.


### Example 1

Given the predicate, `x = 5 AND y = 10`, the rewritten predicate would look like:

```sql
x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)
AND
y_null_count != y_row_count AND (y_min <= 10 AND 10 <= y_max)
```

If we know that for a given container, `x` is between `1 and 100` and we know that
`y` is between `4` and `7`, we know nothing about the null count and row count of
`x` and `y`, the input statistics might look like:

Column   | Value
-------- | -----
`x_min`  | `1`
`x_max`  | `100`
`x_null_count` | `null`
`x_row_count`  | `null`
`y_min`  | `4`
`y_max`  | `7`
`y_null_count` | `null`
`y_row_count`  | `null`

When these statistics values are substituted in to the rewritten predicate and
simplified, the result is `false`:

* `null != null AND (1 <= 5 AND 5 <= 100) AND null != null AND (4 <= 10 AND 10 <= 7)`
* `null = null` is `null` which is not true, so the AND moves on to the next clause
* `null and (1 <= 5 AND 5 <= 100) AND null AND (4 <= 10 AND 10 <= 7)`
* evaluating the clauses further we get:
* `null and true and null and false`
* `null and false`
* `false`

Returning `false` means the container can be pruned, which matches the
intuition that  `x = 5 AND y = 10` can’t be true for any row if all values of `y`
are `7` or less.

Note that if we had ended up with `null AND true AND null AND true` the result
would have been `null`.
`null` is treated the same as`true`, because we can't prove that the predicate is `false.`

If, for some other container, we knew `y` was between the values `4` and
`15`, then the rewritten predicate evaluates to `true` (verifying this is
left as an exercise to the reader -- are you still here?), and the container
**could not** be pruned. The intuition is that there may be rows where the
predicate *might* evaluate to `true`, and the only way to find out is to do
more analysis, for example by actually reading the data and evaluating the
predicate row by row.

### Example 2

Given the same predicate, `x = 5 AND y = 10`, the rewritten predicate would
look like the same as example 1:

```sql
x_null_count != x_row_count AND (x_min <= 5 AND 5 <= x_max)
AND
y_null_count != y_row_count AND (y_min <= 10 AND 10 <= y_max)
```

If we know that for another given container, `x_min` is NULL and `x_max` is
NULL (the min/max values are unknown), `x_null_count` is `100` and `x_row_count`
 is `100`; we know that `y` is between `4` and `7`, but we know nothing about
the null count and row count of `y`. The input statistics might look like:

Column   | Value
-------- | -----
`x_min`  | `null`
`x_max`  | `null`
`x_null_count` | `100`
`x_row_count`  | `100`
`y_min`  | `4`
`y_max`  | `7`
`y_null_count` | `null`
`y_row_count`  | `null`

When these statistics values are substituted in to the rewritten predicate and
simplified, the result is `false`:

* `100 != 100 AND (null <= 5 AND 5 <= null) AND null = null AND (4 <= 10 AND 10 <= 7)`
* `false AND null AND null AND false`
* `false AND false`
* `false`

Returning `false` means the container can be pruned, which matches the
intuition that  `x = 5 AND y = 10` can’t be true because all values in `x`
are known to be NULL.

# Related Work

[`PruningPredicate`] implements the type of min/max pruning described in
Section `3.3.3` of the [`Snowflake SIGMOD Paper`]. The technique is
described by various research such as [small materialized aggregates], [zone
maps], and [data skipping].

[`Snowflake SIGMOD Paper`]: https://dl.acm.org/doi/10.1145/2882903.2903741
[small materialized aggregates]: https://www.vldb.org/conf/1998/p476.pdf
[zone maps]: https://dl.acm.org/doi/10.1007/978-3-642-03730-6_10
[data skipping]: https://dl.acm.org/doi/10.1145/2588555.2610515

```rust
pub struct PruningPredicate {
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
  pub fn try_new(expr: Arc<dyn PhysicalExpr>, schema: SchemaRef) -> Result<Self> { /* ... */ }
  ```
  Try to create a new instance of [`PruningPredicate`]

- ```rust
  pub fn prune<S: PruningStatistics>(self: &Self, statistics: &S) -> Result<Vec<bool>> { /* ... */ }
  ```
  For each set of statistics, evaluates the pruning predicate

- ```rust
  pub fn schema(self: &Self) -> &SchemaRef { /* ... */ }
  ```
  Return a reference to the input schema

- ```rust
  pub fn orig_expr(self: &Self) -> &Arc<dyn PhysicalExpr> { /* ... */ }
  ```
  Returns a reference to the physical expr used to construct this pruning predicate

- ```rust
  pub fn predicate_expr(self: &Self) -> &Arc<dyn PhysicalExpr> { /* ... */ }
  ```
  Returns a reference to the predicate expr

- ```rust
  pub fn literal_guarantees(self: &Self) -> &[LiteralGuarantee] { /* ... */ }
  ```
  Returns a reference to the literal guarantees

- ```rust
  pub fn always_true(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if this pruning predicate can not prune anything.

- ```rust
  pub fn required_columns(self: &Self) -> &RequiredColumns { /* ... */ }
  ```

- ```rust
  pub fn literal_columns(self: &Self) -> Vec<String> { /* ... */ }
  ```
  Names of the columns that are known to be / not be in a set

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

- **Send**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> PruningPredicate { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `RequiredColumns`

Describes which columns statistics are necessary to evaluate a
[`PruningPredicate`].

This structure permits reading and creating the minimum number statistics,
which is important since statistics may be non trivial to read (e.g. large
strings or when there are 1000s of columns).

Handles creating references to the min/max statistics
for columns as well as recording which statistics are needed

```rust
pub struct RequiredColumns {
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
  pub fn single_column(self: &Self) -> Option<&phys_expr::Column> { /* ... */ }
  ```
  Returns Some(column) if this is a single column predicate.

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(columns: Vec<(phys_expr::Column, StatisticsType, Field)>) -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> RequiredColumns { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RequiredColumns { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
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

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `PredicateRewriter`

Rewrite a predicate expression in terms of statistics (min/max/null_counts)
for use as a [`PruningPredicate`].

```rust
pub struct PredicateRewriter {
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
  Create a new `PredicateRewriter`

- ```rust
  pub fn with_unhandled_hook(self: Self, unhandled_hook: Arc<dyn UnhandledPredicateHook>) -> Self { /* ... */ }
  ```
  Set the unhandled hook to be used when a predicate can not be rewritten

- ```rust
  pub fn rewrite_predicate_to_statistics_predicate(self: &Self, expr: &Arc<dyn PhysicalExpr>, schema: &Schema) -> Arc<dyn PhysicalExpr> { /* ... */ }
  ```
  Translate logical filter expression into pruning predicate

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Traits

#### Trait `PruningStatistics`

A source of runtime statistical information to [`PruningPredicate`]s.

# Supported Information

1. Minimum and maximum values for columns

2. Null counts and row counts for columns

3. Whether the values in a column are contained in a set of literals

# Vectorized Interface

Information for containers / files are returned as Arrow [`ArrayRef`], so
the evaluation happens once on a single `RecordBatch`, which amortizes the
overhead of evaluating the predicate. This is important when pruning 1000s
of containers which often happens in analytic systems that have 1000s of
potential files to consider.

For example, for the following three files with a single column `a`:
```text
file1: column a: min=5, max=10
file2: column a: No stats
file2: column a: min=20, max=30
```

PruningStatistics would return:

```text
min_values("a") -> Some([5, Null, 20])
max_values("a") -> Some([10, Null, 30])
min_values("X") -> None
```

```rust
pub trait PruningStatistics {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `min_values`: Return the minimum values for the named column, if known.
- `max_values`: Return the maximum values for the named column, if known.
- `num_containers`: Return the number of containers (e.g. Row Groups) being pruned with
- `null_counts`: Return the number of null values for the named column as an
- `row_counts`: Return the number of rows for the named column in each container
- `contained`: Returns [`BooleanArray`] where each row represents information known

#### Trait `UnhandledPredicateHook`

Rewrites predicates that [`PredicateRewriter`] can not handle, e.g. certain
complex expressions or predicates that reference columns that are not in the
schema.

```rust
pub trait UnhandledPredicateHook {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `handle`: Called when a predicate can not be rewritten in terms of statistics or

## Module `sanity_checker`

The [SanityCheckPlan] rule ensures that a given plan can
accommodate its infinite sources, if there are any. It will reject
non-runnable query plans that use pipeline-breaking operators on
infinite input(s). In addition, it will check if all order and
distribution requirements of a plan are satisfied by its children.

```rust
pub mod sanity_checker { /* ... */ }
```

### Types

#### Struct `SanityCheckPlan`

The SanityCheckPlan rule rejects the following query plans:
1. Invalid plans containing nodes whose order and/or distribution requirements
   are not satisfied by their children.
2. Plans that use pipeline-breaking operators on infinite input(s),
   it is impossible to execute such queries (they will never generate output nor finish)

```rust
pub struct SanityCheckPlan {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SanityCheckPlan { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Functions

#### Function `check_finiteness_requirements`

This function propagates finiteness information and rejects any plan with
pipeline-breaking operators acting on infinite inputs.

```rust
pub fn check_finiteness_requirements(input: std::sync::Arc<dyn ExecutionPlan>, optimizer_options: &datafusion_common::config::OptimizerOptions) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn ExecutionPlan>>> { /* ... */ }
```

#### Function `check_plan_sanity`

Ensures that the plan is pipeline friendly and the order and
distribution requirements from its children are satisfied.

```rust
pub fn check_plan_sanity(plan: std::sync::Arc<dyn ExecutionPlan>, optimizer_options: &datafusion_common::config::OptimizerOptions) -> datafusion_common::Result<datafusion_common::tree_node::Transformed<std::sync::Arc<dyn ExecutionPlan>>> { /* ... */ }
```

## Module `topk_aggregation`

An optimizer rule that detects aggregate operations that could use a limited bucket count

```rust
pub mod topk_aggregation { /* ... */ }
```

### Types

#### Struct `TopKAggregation`

An optimizer rule that passes a `limit` hint to aggregations if the whole result is not needed

```rust
pub struct TopKAggregation {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new `LimitAggregation`

###### Trait Implementations

- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **MaybeSendSync**
- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Allocation**
## Module `update_aggr_exprs`

An optimizer rule that checks ordering requirements of aggregate expressions
and modifies the expressions to work more efficiently if possible.

```rust
pub mod update_aggr_exprs { /* ... */ }
```

### Types

#### Struct `OptimizeAggregateOrder`

This optimizer rule checks ordering requirements of aggregate expressions.

There are 3 kinds of aggregators in terms of ordering requirements:
- `AggregateOrderSensitivity::Insensitive`, meaning that ordering is not
  important.
- `AggregateOrderSensitivity::HardRequirement`, meaning that the aggregator
  requires a specific ordering.
- `AggregateOrderSensitivity::Beneficial`, meaning that the aggregator can
  handle unordered input, but can run more efficiently if its input conforms
  to a specific ordering.

This rule analyzes aggregate expressions of type `Beneficial` to see whether
their input ordering requirements are satisfied. If this is the case, the
aggregators are modified to run in a more efficient mode.

```rust
pub struct OptimizeAggregateOrder {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **RefUnwindSafe**
- **ErasedDestructor**
- **IntoEither**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> OptimizeAggregateOrder { /* ... */ }
    ```

- **PhysicalOptimizerRule**
  - ```rust
    fn optimize(self: &Self, plan: Arc<dyn ExecutionPlan>, _config: &ConfigOptions) -> Result<Arc<dyn ExecutionPlan>> { /* ... */ }
    ```
    Applies the `OptimizeAggregateOrder` rule to the provided execution plan.

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn schema_check(self: &Self) -> bool { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `utils`

```rust
pub mod utils { /* ... */ }
```

### Functions

#### Function `add_sort_above`

This utility function adds a `SortExec` above an operator according to the
given ordering requirements while preserving the original partitioning.

Note that this updates the plan in both the [`PlanContext.children`] and
the [`PlanContext.plan`]'s children. Therefore its not required to sync
the child plans with [`PlanContext::update_plan_from_children`].

```rust
pub fn add_sort_above<T: Clone + Default>(node: datafusion_physical_plan::tree_node::PlanContext<T>, sort_requirements: datafusion_physical_expr::LexRequirement, fetch: Option<usize>) -> datafusion_physical_plan::tree_node::PlanContext<T> { /* ... */ }
```

#### Function `add_sort_above_with_check`

This utility function adds a `SortExec` above an operator according to the
given ordering requirements while preserving the original partitioning. If
requirement is already satisfied no `SortExec` is added.

```rust
pub fn add_sort_above_with_check<T: Clone + Default>(node: datafusion_physical_plan::tree_node::PlanContext<T>, sort_requirements: datafusion_physical_expr::LexRequirement, fetch: Option<usize>) -> datafusion_physical_plan::tree_node::PlanContext<T> { /* ... */ }
```

#### Function `is_sort`

Checks whether the given operator is a [`SortExec`].

```rust
pub fn is_sort(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool { /* ... */ }
```

#### Function `is_window`

Checks whether the given operator is a window;
i.e. either a [`WindowAggExec`] or a [`BoundedWindowAggExec`].

```rust
pub fn is_window(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool { /* ... */ }
```

#### Function `is_union`

Checks whether the given operator is a [`UnionExec`].

```rust
pub fn is_union(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool { /* ... */ }
```

#### Function `is_sort_preserving_merge`

Checks whether the given operator is a [`SortPreservingMergeExec`].

```rust
pub fn is_sort_preserving_merge(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool { /* ... */ }
```

#### Function `is_coalesce_partitions`

Checks whether the given operator is a [`CoalescePartitionsExec`].

```rust
pub fn is_coalesce_partitions(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool { /* ... */ }
```

#### Function `is_repartition`

Checks whether the given operator is a [`RepartitionExec`].

```rust
pub fn is_repartition(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool { /* ... */ }
```

#### Function `is_limit`

Checks whether the given operator is a limit;
i.e. either a [`LocalLimitExec`] or a [`GlobalLimitExec`].

```rust
pub fn is_limit(plan: &std::sync::Arc<dyn ExecutionPlan>) -> bool { /* ... */ }
```

## Re-exports

### Re-export `PhysicalOptimizerRule`

```rust
pub use optimizer::PhysicalOptimizerRule;
```


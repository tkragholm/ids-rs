# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_optimizer`

# DataFusion Optimizer

Contains rules for rewriting [`LogicalPlan`]s

1. [`Analyzer`] applies [`AnalyzerRule`]s to transform `LogicalPlan`s
   to make the plan valid prior to the rest of the DataFusion optimization
   process (for example, [`TypeCoercion`]).

2. [`Optimizer`] applies [`OptimizerRule`]s to transform `LogicalPlan`s
   into equivalent, but more efficient plans.

[`LogicalPlan`]: datafusion_expr::LogicalPlan
[`TypeCoercion`]: analyzer::type_coercion::TypeCoercion

## Modules

## Module `analyzer`

[`Analyzer`] and [`AnalyzerRule`]

```rust
pub mod analyzer { /* ... */ }
```

### Modules

## Module `function_rewrite`

[`ApplyFunctionRewrites`] to replace `Expr`s with function calls (e.g `||` to array_concat`)

```rust
pub mod function_rewrite { /* ... */ }
```

### Types

#### Struct `ApplyFunctionRewrites`

Analyzer rule that invokes [`FunctionRewrite`]s on expressions

```rust
pub struct ApplyFunctionRewrites {
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
  pub fn new(function_rewrites: Vec<Arc<dyn FunctionRewrite + Send + Sync>>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> ApplyFunctionRewrites { /* ... */ }
    ```

- **Send**
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

- **Freeze**
- **MaybeSendSync**
- **ErasedDestructor**
- **Sync**
- **IntoEither**
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **AnalyzerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn analyze(self: &Self, plan: LogicalPlan, options: &ConfigOptions) -> Result<LogicalPlan> { /* ... */ }
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

## Module `resolve_grouping_function`

Analyzed rule to replace TableScan references
such as DataFrames and Views and inlines the LogicalPlan.

```rust
pub mod resolve_grouping_function { /* ... */ }
```

### Types

#### Struct `ResolveGroupingFunction`

Replaces grouping aggregation function with value derived from internal grouping id

```rust
pub struct ResolveGroupingFunction;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> ResolveGroupingFunction { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **MaybeSendSync**
- **ErasedDestructor**
- **AnalyzerRule**
  - ```rust
    fn analyze(self: &Self, plan: LogicalPlan, _: &ConfigOptions) -> Result<LogicalPlan> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Send**
- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `type_coercion`

Optimizer rule for type validation and coercion

```rust
pub mod type_coercion { /* ... */ }
```

### Types

#### Struct `TypeCoercion`

Performs type coercion by determining the schema
and performing the expression rewrites.

```rust
pub struct TypeCoercion {
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

- **Send**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> TypeCoercion { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
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

- **AnalyzerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn analyze(self: &Self, plan: LogicalPlan, config: &ConfigOptions) -> Result<LogicalPlan> { /* ... */ }
    ```

- **MaybeSendSync**
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

- **Freeze**
- **Sync**
#### Struct `TypeCoercionRewriter`

Rewrite expressions to apply type coercion.

```rust
pub struct TypeCoercionRewriter<''a> {
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
  pub fn new(schema: &''a DFSchema) -> Self { /* ... */ }
  ```
  Create a new [`TypeCoercionRewriter`] with a provided schema

- ```rust
  pub fn coerce_plan(self: &mut Self, plan: LogicalPlan) -> Result<LogicalPlan> { /* ... */ }
  ```
  Coerce the [`LogicalPlan`].

- ```rust
  pub fn coerce_join(self: &mut Self, join: Join) -> Result<LogicalPlan> { /* ... */ }
  ```
  Coerce join equality expressions and join filter

- ```rust
  pub fn coerce_union(union_plan: Union) -> Result<LogicalPlan> { /* ... */ }
  ```
  Coerce the union’s inputs to a common schema compatible with all inputs.

###### Trait Implementations

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **TreeNodeRewriter**
  - ```rust
    fn f_up(self: &mut Self, expr: Expr) -> Result<Transformed<Expr>> { /* ... */ }
    ```

- **Freeze**
- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **RefUnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Sync**
### Functions

#### Function `coerce_union_schema`

Get a common schema that is compatible with all inputs of UNION.

This method presumes that the wildcard expansion is unneeded, or has already
been applied.

```rust
pub fn coerce_union_schema(inputs: &[std::sync::Arc<datafusion_expr::LogicalPlan>]) -> datafusion_common::Result<datafusion_common::DFSchema> { /* ... */ }
```

## Module `subquery`

```rust
pub mod subquery { /* ... */ }
```

### Re-exports

#### Re-export `check_subquery_expr`

**Attributes:**

- `#[deprecated(since = "44.0.0", note =
"please use `datafusion_expr::check_subquery_expr` instead")]`

**⚠️ Deprecated since 44.0.0**: please use `datafusion_expr::check_subquery_expr` instead

```rust
pub use datafusion_expr::check_subquery_expr;
```

### Types

#### Struct `Analyzer`

Rule-based Analyzer.

Applies [`FunctionRewrite`]s and [`AnalyzerRule`]s to transform a
[`LogicalPlan`] in preparation for execution.

For example, the `Analyzer` applies type coercion to ensure the types of
operands match the types required by functions.

```rust
pub struct Analyzer {
    pub function_rewrites: Vec<std::sync::Arc<dyn FunctionRewrite + Send + Sync>>,
    pub rules: Vec<std::sync::Arc<dyn AnalyzerRule + Send + Sync>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `function_rewrites` | `Vec<std::sync::Arc<dyn FunctionRewrite + Send + Sync>>` | Expr --> Function writes to apply prior to analysis passes |
| `rules` | `Vec<std::sync::Arc<dyn AnalyzerRule + Send + Sync>>` | All rules to apply |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new analyzer using the recommended list of rules

- ```rust
  pub fn with_rules(rules: Vec<Arc<dyn AnalyzerRule + Send + Sync>>) -> Self { /* ... */ }
  ```
  Create a new analyzer with the given rules

- ```rust
  pub fn add_function_rewrite(self: &mut Self, rewrite: Arc<dyn FunctionRewrite + Send + Sync>) { /* ... */ }
  ```
  Add a function rewrite rule

- ```rust
  pub fn function_rewrites(self: &Self) -> &[Arc<dyn FunctionRewrite + Send + Sync>] { /* ... */ }
  ```
  return the list of function rewrites in this analyzer

- ```rust
  pub fn execute_and_check<F>(self: &Self, plan: LogicalPlan, config: &ConfigOptions, observer: F) -> Result<LogicalPlan>
where
    F: FnMut(&LogicalPlan, &dyn AnalyzerRule) { /* ... */ }
  ```
  Analyze the logical plan by applying analyzer rules, and

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Analyzer { /* ... */ }
    ```

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **Unpin**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Traits

#### Trait `AnalyzerRule`

[`AnalyzerRule`]s transform [`LogicalPlan`]s in some way to make
the plan valid prior to the rest of the DataFusion optimization process.

`AnalyzerRule`s are different than an [`OptimizerRule`](crate::OptimizerRule)s
which must preserve the semantics of the `LogicalPlan`, while computing
results in a more optimal way.

For example, an `AnalyzerRule` may resolve [`Expr`](datafusion_expr::Expr)s into more specific
forms such as a subquery reference, or do type coercion to ensure the types
of operands are correct.

Use [`SessionState::add_analyzer_rule`] to register additional
`AnalyzerRule`s.

[`SessionState::add_analyzer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_analyzer_rule

```rust
pub trait AnalyzerRule: Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `analyze`: Rewrite `plan`
- `name`: A human readable name for this analyzer rule

##### Implementations

This trait is implemented for the following types:

- `ApplyFunctionRewrites`
- `ResolveGroupingFunction`
- `TypeCoercion`

## Module `common_subexpr_eliminate`

[`CommonSubexprEliminate`] to avoid redundant computation of common sub-expressions

```rust
pub mod common_subexpr_eliminate { /* ... */ }
```

### Types

#### Struct `CommonSubexprEliminate`

Performs Common Sub-expression Elimination optimization.

This optimization improves query performance by computing expressions that
appear more than once and reusing those results rather than re-computing the
same value

Currently only common sub-expressions within a single `LogicalPlan` are
eliminated.

# Example

Given a projection that computes the same expensive expression
multiple times such as parsing as string as a date with `to_date` twice:

```text
ProjectionExec(expr=[extract (day from to_date(c1)), extract (year from to_date(c1))])
```

This optimization will rewrite the plan to compute the common expression once
using a new `ProjectionExec` and then rewrite the original expressions to
refer to that new column.

```text
ProjectionExec(exprs=[extract (day from new_col), extract (year from new_col)]) <-- reuse here
  ProjectionExec(exprs=[to_date(c1) as new_col]) <-- compute to_date once
```

```rust
pub struct CommonSubexprEliminate {
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Unpin**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Allocation**
- **RefUnwindSafe**
## Module `decorrelate`

[`PullUpCorrelatedExpr`] converts correlated subqueries to `Joins`

```rust
pub mod decorrelate { /* ... */ }
```

### Types

#### Struct `PullUpCorrelatedExpr`

This struct rewrite the sub query plan by pull up the correlated
expressions(contains outer reference columns) from the inner subquery's
'Filter'. It adds the inner reference columns to the 'Projection' or
'Aggregate' of the subquery if they are missing, so that they can be
evaluated by the parent operator as the join condition.

```rust
pub struct PullUpCorrelatedExpr {
    pub join_filters: Vec<datafusion_expr::Expr>,
    pub correlated_subquery_cols_map: datafusion_common::HashMap<datafusion_expr::LogicalPlan, std::collections::BTreeSet<datafusion_common::Column>>,
    pub in_predicate_opt: Option<datafusion_expr::Expr>,
    pub exists_sub_query: bool,
    pub can_pull_up: bool,
    pub need_handle_count_bug: bool,
    pub collected_count_expr_map: datafusion_common::HashMap<datafusion_expr::LogicalPlan, ExprResultMap>,
    pub pull_up_having_expr: Option<datafusion_expr::Expr>,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `join_filters` | `Vec<datafusion_expr::Expr>` |  |
| `correlated_subquery_cols_map` | `datafusion_common::HashMap<datafusion_expr::LogicalPlan, std::collections::BTreeSet<datafusion_common::Column>>` | mapping from the plan to its holding correlated columns |
| `in_predicate_opt` | `Option<datafusion_expr::Expr>` |  |
| `exists_sub_query` | `bool` | Is this an Exists(Not Exists) SubQuery. Defaults to **FALSE** |
| `can_pull_up` | `bool` | Can the correlated expressions be pulled up. Defaults to **TRUE** |
| `need_handle_count_bug` | `bool` | Do we need to handle [the count bug] during the pull up process.<br><br>The "count bug" was described in [Optimization of Nested SQL<br>Queries Revisited](https://dl.acm.org/doi/pdf/10.1145/38714.38723). This bug is<br>not specific to the COUNT function, and it can occur with any aggregate function,<br>such as SUM, AVG, etc. The anomaly arises because aggregates fail to distinguish<br>between an empty set and null values when optimizing a correlated query as a join.<br>Here, we use "the count bug" to refer to all such cases.<br><br>[the count bug]: https://github.com/apache/datafusion/issues/10553 |
| `collected_count_expr_map` | `datafusion_common::HashMap<datafusion_expr::LogicalPlan, ExprResultMap>` | mapping from the plan to its expressions' evaluation result on empty batch |
| `pull_up_having_expr` | `Option<datafusion_expr::Expr>` | pull up having expr, which must be evaluated after the Join |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_need_handle_count_bug(self: Self, need_handle_count_bug: bool) -> Self { /* ... */ }
  ```
  Set if we need to handle [the count bug] during the pull up process

- ```rust
  pub fn with_in_predicate_opt(self: Self, in_predicate_opt: Option<Expr>) -> Self { /* ... */ }
  ```
  Set the in_predicate_opt

- ```rust
  pub fn with_exists_sub_query(self: Self, exists_sub_query: bool) -> Self { /* ... */ }
  ```
  Set if this is an Exists(Not Exists) SubQuery

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
- **ErasedDestructor**
- **IntoEither**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TreeNodeRewriter**
  - ```rust
    fn f_down(self: &mut Self, plan: LogicalPlan) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn f_up(self: &mut Self, plan: LogicalPlan) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

#### Type Alias `ExprResultMap`

Mapping from expr display name to its evaluation result on empty record
batch (for example: 'count(*)' is 'ScalarValue(0)', 'count(*) + 2' is
'ScalarValue(2)')

```rust
pub type ExprResultMap = datafusion_common::HashMap<String, datafusion_expr::Expr>;
```

### Constants and Statics

#### Constant `UN_MATCHED_ROW_INDICATOR`

Used to indicate the unmatched rows from the inner(subquery) table after the left out Join
This is used to handle [the Count bug]

[the Count bug]: https://github.com/apache/datafusion/issues/10553

```rust
pub const UN_MATCHED_ROW_INDICATOR: &str = "__always_true";
```

## Module `decorrelate_predicate_subquery`

[`DecorrelatePredicateSubquery`] converts `IN`/`EXISTS` subquery predicates to `SEMI`/`ANTI` joins

```rust
pub mod decorrelate_predicate_subquery { /* ... */ }
```

### Types

#### Struct `DecorrelatePredicateSubquery`

Optimizer rule for rewriting predicate(IN/EXISTS) subquery to left semi/anti joins

```rust
pub struct DecorrelatePredicateSubquery {
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

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

- **Allocation**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> DecorrelatePredicateSubquery { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **MaybeSendSync**
- **Freeze**
## Module `eliminate_cross_join`

[`EliminateCrossJoin`] converts `CROSS JOIN` to `INNER JOIN` if join predicates are available.

```rust
pub mod eliminate_cross_join { /* ... */ }
```

### Types

#### Struct `EliminateCrossJoin`

```rust
pub struct EliminateCrossJoin;
```

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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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

- **Allocation**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> EliminateCrossJoin { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

## Module `eliminate_duplicated_expr`

[`EliminateDuplicatedExpr`] Removes redundant expressions

```rust
pub mod eliminate_duplicated_expr { /* ... */ }
```

### Types

#### Struct `EliminateDuplicatedExpr`

Optimization rule that eliminate duplicated expr.

```rust
pub struct EliminateDuplicatedExpr;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> EliminateDuplicatedExpr { /* ... */ }
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

- **RefUnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Send**
- **MaybeSendSync**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **OptimizerRule**
  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
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

## Module `eliminate_filter`

[`EliminateFilter`] replaces `where false` or `where null` with an empty relation.

```rust
pub mod eliminate_filter { /* ... */ }
```

### Types

#### Struct `EliminateFilter`

Optimization rule that eliminate the scalar value (true/false/null) filter
with an [LogicalPlan::EmptyRelation]

This saves time in planning and executing the query.
Note that this rule should be applied after simplify expressions optimizer rule.

```rust
pub struct EliminateFilter;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **Sync**
- **Default**
  - ```rust
    fn default() -> EliminateFilter { /* ... */ }
    ```

- **ErasedDestructor**
- **Allocation**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `eliminate_group_by_constant`

[`EliminateGroupByConstant`] removes constant expressions from `GROUP BY` clause

```rust
pub mod eliminate_group_by_constant { /* ... */ }
```

### Types

#### Struct `EliminateGroupByConstant`

Optimizer rule that removes constant expressions from `GROUP BY` clause
and places additional projection on top of aggregation, to preserve
original schema

```rust
pub struct EliminateGroupByConstant {
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
- **Default**
  - ```rust
    fn default() -> EliminateGroupByConstant { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Allocation**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
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
- **MaybeSendSync**
- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `eliminate_join`

[`EliminateJoin`] rewrites `INNER JOIN` with `true`/`null`

```rust
pub mod eliminate_join { /* ... */ }
```

### Types

#### Struct `EliminateJoin`

Eliminates joins when join condition is false.
Replaces joins when inner join condition is true with a cross join.

```rust
pub struct EliminateJoin;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EliminateJoin { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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

- **Allocation**
- **Sync**
- **Unpin**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

- **IntoEither**
## Module `eliminate_limit`

[`EliminateLimit`] eliminates `LIMIT` when possible

```rust
pub mod eliminate_limit { /* ... */ }
```

### Types

#### Struct `EliminateLimit`

Optimizer rule to replace `LIMIT 0` or `LIMIT` whose ancestor LIMIT's skip is
greater than or equal to current's fetch

It can cooperate with `propagate_empty_relation` and `limit_push_down`. on a
plan with an empty relation.

This rule also removes OFFSET 0 from the [LogicalPlan]

```rust
pub struct EliminateLimit;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EliminateLimit { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, datafusion_common::DataFusionError> { /* ... */ }
    ```

- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `eliminate_nested_union`

[`EliminateNestedUnion`]: flattens nested `Union` to a single `Union`

```rust
pub mod eliminate_nested_union { /* ... */ }
```

### Types

#### Struct `EliminateNestedUnion`

An optimization rule that replaces nested unions with a single union.

```rust
pub struct EliminateNestedUnion;
```

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

- **RefUnwindSafe**
- **Send**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> EliminateNestedUnion { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Allocation**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **UnwindSafe**
## Module `eliminate_one_union`

[`EliminateOneUnion`]  eliminates single element `Union`

```rust
pub mod eliminate_one_union { /* ... */ }
```

### Types

#### Struct `EliminateOneUnion`

An optimization rule that eliminates union with one element.

```rust
pub struct EliminateOneUnion;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Freeze**
- **ErasedDestructor**
- **Unpin**
- **Sync**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Allocation**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EliminateOneUnion { /* ... */ }
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `eliminate_outer_join`

[`EliminateOuterJoin`] converts `LEFT/RIGHT/FULL` joins to `INNER` joins

```rust
pub mod eliminate_outer_join { /* ... */ }
```

### Types

#### Struct `EliminateOuterJoin`


Attempt to replace outer joins with inner joins.

Outer joins are typically more expensive to compute at runtime
than inner joins and prevent various forms of predicate pushdown
and other optimizations, so removing them if possible is beneficial.

Inner joins filter out rows that do match. Outer joins pass rows
that do not match padded with nulls. If there is a filter in the
query that would filter any such null rows after the join the rows
introduced by the outer join are filtered.

For example, in the `select ... from a left join b on ... where b.xx = 100;`

For rows when `b.xx` is null (as it would be after an outer join),
the `b.xx = 100` predicate filters them out and there is no
need to produce null rows for output.

Generally, an outer join can be rewritten to inner join if the
filters from the WHERE clause return false while any inputs are
null and columns of those quals are come from nullable side of
outer join.

```rust
pub struct EliminateOuterJoin;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **RefUnwindSafe**
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

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> EliminateOuterJoin { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `eliminate_outer`

```rust
pub fn eliminate_outer(join_type: datafusion_expr::logical_plan::JoinType, left_non_nullable: bool, right_non_nullable: bool) -> datafusion_expr::logical_plan::JoinType { /* ... */ }
```

## Module `extract_equijoin_predicate`

[`ExtractEquijoinPredicate`] identifies equality join (equijoin) predicates

```rust
pub mod extract_equijoin_predicate { /* ... */ }
```

### Types

#### Struct `ExtractEquijoinPredicate`

Optimizer that splits conjunctive join predicates into equijoin
predicates and (other) filter predicates.

Join algorithms are often highly optimized for equality predicates such as `x = y`,
often called `equijoin` predicates, so it is important to locate such predicates
and treat them specially.

For example, `SELECT ... FROM A JOIN B ON (A.x = B.y AND B.z > 50)`
has one equijoin predicate (`A.x = B.y`) and one filter predicate (`B.z > 50`).
See [find_valid_equijoin_key_pair] for more information on what predicates
are considered equijoins.

```rust
pub struct ExtractEquijoinPredicate;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **ErasedDestructor**
- **MaybeSendSync**
- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
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

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> ExtractEquijoinPredicate { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **Unpin**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `filter_null_join_keys`

[`FilterNullJoinKeys`] adds filters to join inputs when input isn't nullable

```rust
pub mod filter_null_join_keys { /* ... */ }
```

### Types

#### Struct `FilterNullJoinKeys`

The FilterNullJoinKeys rule will identify joins with equi-join conditions
where the join key is nullable and then insert an `IsNotNull` filter on the nullable side since null values
can never match.

```rust
pub struct FilterNullJoinKeys {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Default**
  - ```rust
    fn default() -> FilterNullJoinKeys { /* ... */ }
    ```

- **Send**
- **Sync**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
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

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
## Module `optimize_projections`

[`OptimizeProjections`] identifies and eliminates unused columns

```rust
pub mod optimize_projections { /* ... */ }
```

### Types

#### Struct `OptimizeProjections`

Optimizer rule to prune unnecessary columns from intermediate schemas
inside the [`LogicalPlan`]. This rule:
- Removes unnecessary columns that do not appear at the output and/or are
  not used during any computation step.
- Adds projections to decrease table column size before operators that
  benefit from a smaller memory footprint at its input.
- Removes unnecessary [`LogicalPlan::Projection`]s from the [`LogicalPlan`].

`OptimizeProjections` is an optimizer rule that identifies and eliminates
columns from a logical plan that are not used by downstream operations.
This can improve query performance and reduce unnecessary data processing.

The rule analyzes the input logical plan, determines the necessary column
indices, and then removes any unnecessary columns. It also removes any
unnecessary projections from the plan tree.

```rust
pub struct OptimizeProjections {
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
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> OptimizeProjections { /* ... */ }
    ```

- **Unpin**
- **Allocation**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `optimizer`

[`Optimizer`] and [`OptimizerRule`]

```rust
pub mod optimizer { /* ... */ }
```

### Types

#### Struct `OptimizerContext`

A standalone [`OptimizerConfig`] that can be used independently
of DataFusion's config management

```rust
pub struct OptimizerContext {
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
  Create optimizer config

- ```rust
  pub fn filter_null_keys(self: Self, filter_null_keys: bool) -> Self { /* ... */ }
  ```
  Specify whether to enable the filter_null_keys rule

- ```rust
  pub fn with_query_execution_start_time(self: Self, query_execution_tart_time: DateTime<Utc>) -> Self { /* ... */ }
  ```
  Specify whether the optimizer should skip rules that produce

- ```rust
  pub fn with_skip_failing_rules(self: Self, b: bool) -> Self { /* ... */ }
  ```
  Specify whether the optimizer should skip rules that produce

- ```rust
  pub fn with_max_passes(self: Self, v: u8) -> Self { /* ... */ }
  ```
  Specify how many times to attempt to optimize the plan

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **UnwindSafe**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **OptimizerConfig**
  - ```rust
    fn query_execution_start_time(self: &Self) -> DateTime<Utc> { /* ... */ }
    ```

  - ```rust
    fn alias_generator(self: &Self) -> &Arc<AliasGenerator> { /* ... */ }
    ```

  - ```rust
    fn options(self: &Self) -> &ConfigOptions { /* ... */ }
    ```

- **Freeze**
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
    Create optimizer config

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
#### Struct `Optimizer`

A rule-based optimizer.

```rust
pub struct Optimizer {
    pub rules: Vec<std::sync::Arc<dyn OptimizerRule + Send + Sync>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `rules` | `Vec<std::sync::Arc<dyn OptimizerRule + Send + Sync>>` | All optimizer rules to apply |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Create a new optimizer using the recommended list of rules

- ```rust
  pub fn with_rules(rules: Vec<Arc<dyn OptimizerRule + Send + Sync>>) -> Self { /* ... */ }
  ```
  Create a new optimizer with the given rules

- ```rust
  pub fn optimize<F>(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig, observer: F) -> Result<LogicalPlan>
where
    F: FnMut(&LogicalPlan, &dyn OptimizerRule) { /* ... */ }
  ```
  Optimizes the logical plan by applying optimizer rules, and

###### Trait Implementations

- **Freeze**
- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Optimizer { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
#### Enum `ApplyOrder`

Specifies how recursion for an `OptimizerRule` should be handled.

* `Some(apply_order)`: The Optimizer will recursively apply the rule to the plan.
* `None`: the rule must handle any required recursion itself.

```rust
pub enum ApplyOrder {
    TopDown,
    BottomUp,
}
```

##### Variants

###### `TopDown`

Apply the rule to the node before its inputs

###### `BottomUp`

Apply the rule to the node after its inputs

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ApplyOrder { /* ... */ }
    ```

- **Copy**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ApplyOrder) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **StructuralPartialEq**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **MaybeSendSync**
- **Send**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Traits

#### Trait `OptimizerRule`

`OptimizerRule`s transforms one [`LogicalPlan`] into another which
computes the same results, but in a potentially more efficient
way. If there are no suitable transformations for the input plan,
the optimizer should simply return it unmodified.

To change the semantics of a `LogicalPlan`, see [`AnalyzerRule`]

Use [`SessionState::add_optimizer_rule`] to register additional
`OptimizerRule`s.

[`AnalyzerRule`]: crate::analyzer::AnalyzerRule
[`SessionState::add_optimizer_rule`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html#method.add_optimizer_rule

```rust
pub trait OptimizerRule: Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `name`: A human readable name for this optimizer rule

##### Provided Methods

- ```rust
  fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
  ```
  How should the rule be applied by the optimizer? See comments on

- ```rust
  fn supports_rewrite(self: &Self) -> bool { /* ... */ }
  ```
  Does this rule support rewriting owned plans (rather than by reference)?

- ```rust
  fn rewrite(self: &Self, _plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError> { /* ... */ }
  ```
  Try to rewrite `plan` to an optimized form, returning `Transformed::yes`

##### Implementations

This trait is implemented for the following types:

- `CommonSubexprEliminate`
- `DecorrelatePredicateSubquery`
- `EliminateCrossJoin`
- `EliminateDuplicatedExpr`
- `EliminateFilter`
- `EliminateGroupByConstant`
- `EliminateJoin`
- `EliminateLimit`
- `EliminateNestedUnion`
- `EliminateOneUnion`
- `EliminateOuterJoin`
- `ExtractEquijoinPredicate`
- `FilterNullJoinKeys`
- `OptimizeProjections`
- `PropagateEmptyRelation`
- `PushDownFilter`
- `PushDownLimit`
- `ReplaceDistinctWithAggregate`
- `ScalarSubqueryToJoin`
- `SimplifyExpressions`
- `SingleDistinctToGroupBy`

#### Trait `OptimizerConfig`

Options to control the DataFusion Optimizer.

```rust
pub trait OptimizerConfig {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `query_execution_start_time`: Return the time at which the query execution started. This
- `alias_generator`: Return alias generator used to generate unique aliases for subqueries
- `options`

##### Provided Methods

- ```rust
  fn function_registry(self: &Self) -> Option<&dyn FunctionRegistry> { /* ... */ }
  ```

##### Implementations

This trait is implemented for the following types:

- `OptimizerContext`

## Module `propagate_empty_relation`

[`PropagateEmptyRelation`] eliminates nodes fed by `EmptyRelation`

```rust
pub mod propagate_empty_relation { /* ... */ }
```

### Types

#### Struct `PropagateEmptyRelation`

Optimization rule that bottom-up to eliminate plan by propagating empty_relation.

```rust
pub struct PropagateEmptyRelation;
```

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **Default**
  - ```rust
    fn default() -> PropagateEmptyRelation { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `push_down_filter`

[`PushDownFilter`] applies filters as early as possible

```rust
pub mod push_down_filter { /* ... */ }
```

### Types

#### Struct `PushDownFilter`

Optimizer rule for pushing (moving) filter expressions down in a plan so
they are applied as early as possible.

# Introduction

The goal of this rule is to improve query performance by eliminating
redundant work.

For example, given a plan that sorts all values where `a > 10`:

```text
 Filter (a > 10)
   Sort (a, b)
```

A better plan is to  filter the data *before* the Sort, which sorts fewer
rows and therefore does less work overall:

```text
 Sort (a, b)
   Filter (a > 10)  <-- Filter is moved before the sort
```

However it is not always possible to push filters down. For example, given a
plan that finds the top 3 values and then keeps only those that are greater
than 10, if the filter is pushed below the limit it would produce a
different result.

```text
 Filter (a > 10)   <-- can not move this Filter before the limit
   Limit (fetch=3)
     Sort (a, b)
```


More formally, a filter-commutative operation is an operation `op` that
satisfies `filter(op(data)) = op(filter(data))`.

The filter-commutative property is plan and column-specific. A filter on `a`
can be pushed through a `Aggregate(group_by = [a], agg=[sum(b))`. However, a
filter on  `sum(b)` can not be pushed through the same aggregate.

# Handling Conjunctions

It is possible to only push down **part** of a filter expression if is
connected with `AND`s (more formally if it is a "conjunction").

For example, given the following plan:

```text
Filter(a > 10 AND sum(b) < 5)
  Aggregate(group_by = [a], agg = [sum(b))
```

The `a > 10` is commutative with the `Aggregate` but  `sum(b) < 5` is not.
Therefore it is possible to only push part of the expression, resulting in:

```text
Filter(sum(b) < 5)
  Aggregate(group_by = [a], agg = [sum(b))
    Filter(a > 10)
```

# Handling Column Aliases

This optimizer must sometimes handle re-writing filter expressions when they
pushed, for example if there is a projection that aliases `a+1` to `"b"`:

```text
Filter (b > 10)
    Projection: [a+1 AS "b"]  <-- changes the name of `a+1` to `b`
```

To apply the filter prior to the `Projection`, all references to `b` must be
rewritten to `a+1`:

```text
Projection: a AS "b"
    Filter: (a + 1 > 10)  <--- changed from b to a + 1
```
# Implementation Notes

This implementation performs a single pass through the plan, "pushing" down
filters. When it passes through a filter, it stores that filter, and when it
reaches a plan node that does not commute with that filter, it adds the
filter to that place. When it passes through a projection, it re-writes the
filter's expression taking into account that projection.

```rust
pub struct PushDownFilter {
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> PushDownFilter { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Unpin**
### Functions

#### Function `make_filter`

Creates a new LogicalPlan::Filter node.

```rust
pub fn make_filter(predicate: datafusion_expr::Expr, input: std::sync::Arc<datafusion_expr::logical_plan::LogicalPlan>) -> datafusion_common::Result<datafusion_expr::logical_plan::LogicalPlan> { /* ... */ }
```

#### Function `replace_cols_by_name`

replaces columns by its name on the projection.

```rust
pub fn replace_cols_by_name(e: datafusion_expr::Expr, replace_map: &std::collections::HashMap<String, datafusion_expr::Expr>) -> datafusion_common::Result<datafusion_expr::Expr> { /* ... */ }
```

## Module `push_down_limit`

[`PushDownLimit`] pushes `LIMIT` earlier in the query plan

```rust
pub mod push_down_limit { /* ... */ }
```

### Types

#### Struct `PushDownLimit`

Optimization rule that tries to push down `LIMIT`.


```rust
pub struct PushDownLimit {
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> PushDownLimit { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Unpin**
- **Sync**
- **Freeze**
- **IntoEither**
- **UnwindSafe**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `replace_distinct_aggregate`

[`ReplaceDistinctWithAggregate`] replaces `DISTINCT ...` with `GROUP BY ...`

```rust
pub mod replace_distinct_aggregate { /* ... */ }
```

### Types

#### Struct `ReplaceDistinctWithAggregate`

Optimizer that replaces logical [[Distinct]] with a logical [[Aggregate]]

```text
SELECT DISTINCT a, b FROM tab
```

Into
```text
SELECT a, b FROM tab GROUP BY a, b
```

On the other hand, for a `DISTINCT ON` query the replacement is
a bit more involved and effectively converts
```text
SELECT DISTINCT ON (a) b FROM tab ORDER BY a DESC, c
```

into
```text
SELECT b FROM (
    SELECT a, FIRST_VALUE(b ORDER BY a DESC, c) AS b
    FROM tab
    GROUP BY a
)
ORDER BY a DESC
```

```rust
pub struct ReplaceDistinctWithAggregate {
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

- **Freeze**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Allocation**
- **Sync**
- **Send**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> ReplaceDistinctWithAggregate { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
## Module `scalar_subquery_to_join`

[`ScalarSubqueryToJoin`] rewriting scalar subquery filters to `JOIN`s

```rust
pub mod scalar_subquery_to_join { /* ... */ }
```

### Types

#### Struct `ScalarSubqueryToJoin`

Optimizer rule for rewriting subquery filters to joins

```rust
pub struct ScalarSubqueryToJoin {
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

- **IntoEither**
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

- **Default**
  - ```rust
    fn default() -> ScalarSubqueryToJoin { /* ... */ }
    ```

- **Allocation**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **UnwindSafe**
- **OptimizerRule**
  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>> { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
## Module `simplify_expressions`

[`SimplifyExpressions`] simplifies expressions in the logical plan,
[`ExprSimplifier`] simplifies individual `Expr`s.

```rust
pub mod simplify_expressions { /* ... */ }
```

### Modules

## Module `expr_simplifier`

Expression simplification API

```rust
pub mod expr_simplifier { /* ... */ }
```

### Types

#### Struct `ExprSimplifier`

This structure handles API for expression simplification

Provides simplification information based on DFSchema and
[`ExecutionProps`]. This is the default implementation used by DataFusion

For example:
```
use arrow::datatypes::{Schema, Field, DataType};
use datafusion_expr::{col, lit};
use datafusion_common::{DataFusionError, ToDFSchema};
use datafusion_expr::execution_props::ExecutionProps;
use datafusion_expr::simplify::SimplifyContext;
use datafusion_optimizer::simplify_expressions::ExprSimplifier;

// Create the schema
let schema = Schema::new(vec![
    Field::new("i", DataType::Int64, false),
  ])
  .to_dfschema_ref().unwrap();

// Create the simplifier
let props = ExecutionProps::new();
let context = SimplifyContext::new(&props)
   .with_schema(schema);
let simplifier = ExprSimplifier::new(context);

// Use the simplifier

// b < 2 or (1 > 3)
let expr = col("b").lt(lit(2)).or(lit(1).gt(lit(3)));

// b < 2
let simplified = simplifier.simplify(expr).unwrap();
assert_eq!(simplified, col("b").lt(lit(2)));
```

```rust
pub struct ExprSimplifier<S> {
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
  pub fn new(info: S) -> Self { /* ... */ }
  ```
  Create a new `ExprSimplifier` with the given `info` such as an

- ```rust
  pub fn simplify(self: &Self, expr: Expr) -> Result<Expr> { /* ... */ }
  ```
  Simplifies this [`Expr`] as much as possible, evaluating

- ```rust
  pub fn simplify_with_cycle_count(self: &Self, expr: Expr) -> Result<(Expr, u32)> { /* ... */ }
  ```
  Like [Self::simplify], simplifies this [`Expr`] as much as possible, evaluating

- ```rust
  pub fn coerce(self: &Self, expr: Expr, schema: &DFSchema) -> Result<Expr> { /* ... */ }
  ```
  Apply type coercion to an [`Expr`] so that it can be

- ```rust
  pub fn with_guarantees(self: Self, guarantees: Vec<(Expr, NullableInterval)>) -> Self { /* ... */ }
  ```
  Input guarantees about the values of columns.

- ```rust
  pub fn with_canonicalize(self: Self, canonicalize: bool) -> Self { /* ... */ }
  ```
  Should `Canonicalizer` be applied before simplification?

- ```rust
  pub fn with_max_cycles(self: Self, max_simplifier_cycles: u32) -> Self { /* ... */ }
  ```
  Specifies the maximum number of simplification cycles to run.

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Freeze**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Unpin**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
### Constants and Statics

#### Constant `THRESHOLD_INLINE_INLIST`

```rust
pub const THRESHOLD_INLINE_INLIST: usize = 3;
```

#### Constant `DEFAULT_MAX_SIMPLIFIER_CYCLES`

```rust
pub const DEFAULT_MAX_SIMPLIFIER_CYCLES: u32 = 3;
```

## Module `simplify_exprs`

Simplify expressions optimizer rule and implementation

```rust
pub mod simplify_exprs { /* ... */ }
```

### Types

#### Struct `SimplifyExpressions`

Optimizer Pass that simplifies [`LogicalPlan`]s by rewriting
[`Expr`]`s evaluating constants and applying algebraic
simplifications

# Introduction
It uses boolean algebra laws to simplify or reduce the number of terms in expressions.

# Example:
`Filter: b > 2 AND b > 2`
is optimized to
`Filter: b > 2`

[`Expr`]: datafusion_expr::Expr

```rust
pub struct SimplifyExpressions {
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SimplifyExpressions { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Allocation**
- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
### Re-exports

#### Re-export `SimplifyContext`

```rust
pub use datafusion_expr::simplify::SimplifyContext;
```

#### Re-export `SimplifyInfo`

```rust
pub use datafusion_expr::simplify::SimplifyInfo;
```

#### Re-export `GuaranteeRewriter`

```rust
pub use guarantees::GuaranteeRewriter;
```

#### Re-export `expr_simplifier::*`

```rust
pub use expr_simplifier::*;
```

#### Re-export `simplify_exprs::*`

```rust
pub use simplify_exprs::*;
```

## Module `single_distinct_to_groupby`

[`SingleDistinctToGroupBy`] replaces `AGG(DISTINCT ..)` with `AGG(..) GROUP BY ..`

```rust
pub mod single_distinct_to_groupby { /* ... */ }
```

### Types

#### Struct `SingleDistinctToGroupBy`

single distinct to group by optimizer rule
 ```text
   Before:
   SELECT a, count(DISTINCT b), sum(c)
   FROM t
   GROUP BY a

   After:
   SELECT a, count(alias1), sum(alias2)
   FROM (
     SELECT a, b as alias1, sum(c) as alias2
     FROM t
     GROUP BY a, b
   )
   GROUP BY a
 ```

```rust
pub struct SingleDistinctToGroupBy {
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

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> SingleDistinctToGroupBy { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **OptimizerRule**
  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn apply_order(self: &Self) -> Option<ApplyOrder> { /* ... */ }
    ```

  - ```rust
    fn supports_rewrite(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn rewrite(self: &Self, plan: LogicalPlan, _config: &dyn OptimizerConfig) -> Result<Transformed<LogicalPlan>, DataFusionError> { /* ... */ }
    ```

- **Send**
- **Sync**
- **RefUnwindSafe**
- **MaybeSendSync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `utils`

Utility functions leveraged by the query optimizer rules

```rust
pub mod utils { /* ... */ }
```

### Functions

#### Function `log_plan`

Log the plan in debug/tracing mode after some part of the optimizer runs

```rust
pub fn log_plan(description: &str, plan: &datafusion_expr::logical_plan::LogicalPlan) { /* ... */ }
```

#### Function `is_restrict_null_predicate`

Determine whether a predicate can restrict NULLs. e.g.
`c0 > 8` return true;
`c0 IS NULL` return false.

```rust
pub fn is_restrict_null_predicate<''a, /* synthetic */ impl IntoIterator<Item = &'a Column>: IntoIterator<Item = &''a datafusion_common::Column>>(predicate: datafusion_expr::Expr, join_cols_of_predicate: impl IntoIterator<Item = &''a datafusion_common::Column>) -> datafusion_common::Result<bool> { /* ... */ }
```

#### Function `evaluates_to_null`

Determines if an expression will always evaluate to null.
`c0 + 8` return true
`c0 IS NULL` return false
`CASE WHEN c0 > 1 then 0 else 1` return false

```rust
pub fn evaluates_to_null<''a, /* synthetic */ impl IntoIterator<Item = &'a Column>: IntoIterator<Item = &''a datafusion_common::Column>>(predicate: datafusion_expr::Expr, null_columns: impl IntoIterator<Item = &''a datafusion_common::Column>) -> datafusion_common::Result<bool> { /* ... */ }
```

### Re-exports

#### Re-export `NamePreserver`

Re-export of `NamesPreserver` for backwards compatibility,
as it was initially placed here and then moved elsewhere.

```rust
pub use datafusion_expr::expr_rewriter::NamePreserver;
```

## Re-exports

### Re-export `Analyzer`

```rust
pub use analyzer::Analyzer;
```

### Re-export `AnalyzerRule`

```rust
pub use analyzer::AnalyzerRule;
```

### Re-export `ApplyOrder`

```rust
pub use optimizer::ApplyOrder;
```

### Re-export `Optimizer`

```rust
pub use optimizer::Optimizer;
```

### Re-export `OptimizerConfig`

```rust
pub use optimizer::OptimizerConfig;
```

### Re-export `OptimizerContext`

```rust
pub use optimizer::OptimizerContext;
```

### Re-export `OptimizerRule`

```rust
pub use optimizer::OptimizerRule;
```


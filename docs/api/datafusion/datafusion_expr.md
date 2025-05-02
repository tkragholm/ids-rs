# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_expr`

[DataFusion](https://github.com/apache/datafusion)
is an extensible query execution framework that uses
[Apache Arrow](https://arrow.apache.org) as its in-memory format.

This crate is a submodule of DataFusion that provides types representing
logical query plans ([LogicalPlan]) and logical expressions ([Expr]) as well as utilities for
working with these types.

The [expr_fn] module contains functions for creating expressions.

## Modules

## Module `conditional_expressions`

Conditional expressions

```rust
pub mod conditional_expressions { /* ... */ }
```

### Types

#### Struct `CaseBuilder`

Helper struct for building [Expr::Case]

```rust
pub struct CaseBuilder {
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
  pub fn new(expr: Option<Box<Expr>>, when_expr: Vec<Expr>, then_expr: Vec<Expr>, else_expr: Option<Box<Expr>>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn when(self: &mut Self, when: Expr, then: Expr) -> CaseBuilder { /* ... */ }
  ```

- ```rust
  pub fn otherwise(self: &mut Self, else_expr: Expr) -> Result<Expr> { /* ... */ }
  ```

- ```rust
  pub fn end(self: &Self) -> Result<Expr> { /* ... */ }
  ```

###### Trait Implementations

- **Send**
- **Freeze**
- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

## Module `execution_props`

```rust
pub mod execution_props { /* ... */ }
```

### Types

#### Struct `ExecutionProps`

Holds per-query execution properties and data (such as statement
starting timestamps).

An [`ExecutionProps`] is created each time a `LogicalPlan` is
prepared for execution (optimized). If the same plan is optimized
multiple times, a new `ExecutionProps` is created each time.

It is important that this structure be cheap to create as it is
done so during predicate pruning and expression simplification

```rust
pub struct ExecutionProps {
    pub query_execution_start_time: chrono::DateTime<chrono::Utc>,
    pub alias_generator: std::sync::Arc<datafusion_common::alias::AliasGenerator>,
    pub var_providers: Option<datafusion_common::HashMap<crate::var_provider::VarType, std::sync::Arc<dyn VarProvider + Send + Sync>>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `query_execution_start_time` | `chrono::DateTime<chrono::Utc>` |  |
| `alias_generator` | `std::sync::Arc<datafusion_common::alias::AliasGenerator>` | Alias generator used by subquery optimizer rules |
| `var_providers` | `Option<datafusion_common::HashMap<crate::var_provider::VarType, std::sync::Arc<dyn VarProvider + Send + Sync>>>` | Providers for scalar variables |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new execution props

- ```rust
  pub fn with_query_execution_start_time(self: Self, query_execution_start_time: DateTime<Utc>) -> Self { /* ... */ }
  ```
  Set the query execution start time to use

- ```rust
  pub fn start_execution(self: &mut Self) -> &Self { /* ... */ }
  ```
  Marks the execution of query started timestamp.

- ```rust
  pub fn add_var_provider(self: &mut Self, var_type: VarType, provider: Arc<dyn VarProvider + Send + Sync>) -> Option<Arc<dyn VarProvider + Send + Sync>> { /* ... */ }
  ```
  Registers a variable provider, returning the existing

- ```rust
  pub fn get_var_provider(self: &Self, var_type: VarType) -> Option<Arc<dyn VarProvider + Send + Sync>> { /* ... */ }
  ```
  Returns the provider for the `var_type`, if any

###### Trait Implementations

- **UnwindSafe**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **MaybeSendSync**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExecutionProps { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `expr`

Logical Expressions: [`Expr`]

```rust
pub mod expr { /* ... */ }
```

### Types

#### Enum `Expr`

 Represents logical expressions such as `A + 1`, or `CAST(c1 AS int)`.

 For example the expression `A + 1` will be represented as

```text
  BinaryExpr {
    left: Expr::Column("A"),
    op: Operator::Plus,
    right: Expr::Literal(ScalarValue::Int32(Some(1)))
 }
 ```

 # Creating Expressions

 `Expr`s can be created directly, but it is often easier and less verbose to
 use the fluent APIs in [`crate::expr_fn`] such as [`col`] and [`lit`], or
 methods such as [`Expr::alias`], [`Expr::cast_to`], and [`Expr::Like`]).

 See also [`ExprFunctionExt`] for creating aggregate and window functions.

 [`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

 # Printing Expressions

 You can print `Expr`s using the the `Debug` trait, `Display` trait, or
 [`Self::human_display`]. See the [examples](#examples-displaying-exprs) below.

 If you need  SQL to pass to other systems, consider using [`Unparser`].

 [`Unparser`]: https://docs.rs/datafusion/latest/datafusion/sql/unparser/struct.Unparser.html

 # Schema Access

 See [`ExprSchemable::get_type`] to access the [`DataType`] and nullability
 of an `Expr`.

 # Visiting and Rewriting `Expr`s

 The `Expr` struct implements the [`TreeNode`] trait for walking and
 rewriting expressions. For example [`TreeNode::apply`] recursively visits an
 `Expr` and [`TreeNode::transform`] can be used to rewrite an expression. See
 the examples below and [`TreeNode`] for more information.

 # Examples: Creating and Using `Expr`s

 ## Column References and Literals

 [`Expr::Column`] refer to the values of columns and are often created with
 the [`col`] function. For example to create an expression `c1` referring to
 column named "c1":

 [`col`]: crate::expr_fn::col

 ```
 # use datafusion_common::Column;
 # use datafusion_expr::{lit, col, Expr};
 let expr = col("c1");
 assert_eq!(expr, Expr::Column(Column::from_name("c1")));
 ```

 [`Expr::Literal`] refer to literal, or constant, values. These are created
 with the [`lit`] function. For example to create an expression `42`:

 [`lit`]: crate::lit

 ```
 # use datafusion_common::{Column, ScalarValue};
 # use datafusion_expr::{lit, col, Expr};
 // All literals are strongly typed in DataFusion. To make an `i64` 42:
 let expr = lit(42i64);
 assert_eq!(expr, Expr::Literal(ScalarValue::Int64(Some(42))));
 assert_eq!(expr, Expr::Literal(ScalarValue::Int64(Some(42))));
 // To make a (typed) NULL:
 let expr = Expr::Literal(ScalarValue::Int64(None));
 // to make an (untyped) NULL (the optimizer will coerce this to the correct type):
 let expr = lit(ScalarValue::Null);
 ```

 ## Binary Expressions

 Exprs implement traits that allow easy to understand construction of more
 complex expressions. For example, to create `c1 + c2` to add columns "c1" and
 "c2" together

 ```
 # use datafusion_expr::{lit, col, Operator, Expr};
 // Use the `+` operator to add two columns together
 let expr = col("c1") + col("c2");
 assert!(matches!(expr, Expr::BinaryExpr { ..} ));
 if let Expr::BinaryExpr(binary_expr) = expr {
   assert_eq!(*binary_expr.left, col("c1"));
   assert_eq!(*binary_expr.right, col("c2"));
   assert_eq!(binary_expr.op, Operator::Plus);
 }
 ```

 The expression `c1 = 42` to compares the value in column "c1" to the
 literal value `42`:

 ```
 # use datafusion_common::ScalarValue;
 # use datafusion_expr::{lit, col, Operator, Expr};
 let expr = col("c1").eq(lit(42_i32));
 assert!(matches!(expr, Expr::BinaryExpr { .. } ));
 if let Expr::BinaryExpr(binary_expr) = expr {
   assert_eq!(*binary_expr.left, col("c1"));
   let scalar = ScalarValue::Int32(Some(42));
   assert_eq!(*binary_expr.right, Expr::Literal(scalar));
   assert_eq!(binary_expr.op, Operator::Eq);
 }
 ```

 Here is how to implement the equivalent of `SELECT *` to select all
 [`Expr::Column`] from a [`DFSchema`]'s columns:

 ```
 # use arrow::datatypes::{DataType, Field, Schema};
 # use datafusion_common::{DFSchema, Column};
 # use datafusion_expr::Expr;
 // Create a schema c1(int, c2 float)
 let arrow_schema = Schema::new(vec![
    Field::new("c1", DataType::Int32, false),
    Field::new("c2", DataType::Float64, false),
 ]);
 // DFSchema is a an Arrow schema with optional relation name
 let df_schema = DFSchema::try_from_qualified_schema("t1", &arrow_schema)
   .unwrap();

 // Form Vec<Expr> with an expression for each column in the schema
 let exprs: Vec<_> = df_schema.iter()
   .map(Expr::from)
   .collect();

 assert_eq!(exprs, vec![
   Expr::from(Column::from_qualified_name("t1.c1")),
   Expr::from(Column::from_qualified_name("t1.c2")),
 ]);
 ```

 # Examples: Displaying `Exprs`

 There are three ways to print an `Expr` depending on the usecase.

 ## Use `Debug` trait

 Following Rust conventions, the `Debug` implementation prints out the
 internal structure of the expression, which is useful for debugging.

 ```
 # use datafusion_expr::{lit, col};
 let expr = col("c1") + lit(42);
 assert_eq!(format!("{expr:?}"), "BinaryExpr(BinaryExpr { left: Column(Column { relation: None, name: \"c1\" }), op: Plus, right: Literal(Int32(42)) })");
 ```

 ## Use the `Display` trait  (detailed expression)

 The `Display` implementation prints out the expression in a SQL-like form,
 but has additional details such as the data type of literals. This is useful
 for understanding the expression in more detail and is used for the low level
 [`ExplainFormat::Indent`] explain plan format.

 [`ExplainFormat::Indent`]: crate::logical_plan::ExplainFormat::Indent

 ```
 # use datafusion_expr::{lit, col};
 let expr = col("c1") + lit(42);
 assert_eq!(format!("{expr}"), "c1 + Int32(42)");
 ```

 ## Use [`Self::human_display`] (human readable)

 [`Self::human_display`]  prints out the expression in a SQL-like form, optimized
 for human consumption by end users. It is used for the
 [`ExplainFormat::Tree`] explain plan format.

 [`ExplainFormat::Tree`]: crate::logical_plan::ExplainFormat::Tree

```
 # use datafusion_expr::{lit, col};
 let expr = col("c1") + lit(42);
 assert_eq!(format!("{}", expr.human_display()), "c1 + 42");
 ```

 # Examples: Visiting and Rewriting `Expr`s

 Here is an example that finds all literals in an `Expr` tree:
 ```
 # use std::collections::{HashSet};
 use datafusion_common::ScalarValue;
 # use datafusion_expr::{col, Expr, lit};
 use datafusion_common::tree_node::{TreeNode, TreeNodeRecursion};
 // Expression a = 5 AND b = 6
 let expr = col("a").eq(lit(5)) & col("b").eq(lit(6));
 // find all literals in a HashMap
 let mut scalars = HashSet::new();
 // apply recursively visits all nodes in the expression tree
 expr.apply(|e| {
    if let Expr::Literal(scalar) = e {
       scalars.insert(scalar);
    }
    // The return value controls whether to continue visiting the tree
    Ok(TreeNodeRecursion::Continue)
 }).unwrap();
 // All subtrees have been visited and literals found
 assert_eq!(scalars.len(), 2);
 assert!(scalars.contains(&ScalarValue::Int32(Some(5))));
 assert!(scalars.contains(&ScalarValue::Int32(Some(6))));
 ```

 Rewrite an expression, replacing references to column "a" in an
 to the literal `42`:

  ```
 # use datafusion_common::tree_node::{Transformed, TreeNode};
 # use datafusion_expr::{col, Expr, lit};
 // expression a = 5 AND b = 6
 let expr = col("a").eq(lit(5)).and(col("b").eq(lit(6)));
 // rewrite all references to column "a" to the literal 42
 let rewritten = expr.transform(|e| {
   if let Expr::Column(c) = &e {
     if &c.name == "a" {
       // return Transformed::yes to indicate the node was changed
       return Ok(Transformed::yes(lit(42)))
     }
   }
   // return Transformed::no to indicate the node was not changed
   Ok(Transformed::no(e))
 }).unwrap();
 // The expression has been rewritten
 assert!(rewritten.transformed);
 // to 42 = 5 AND b = 6
 assert_eq!(rewritten.data, lit(42).eq(lit(5)).and(col("b").eq(lit(6))));

```rust
pub enum Expr {
    Alias(Alias),
    Column(datafusion_common::Column),
    ScalarVariable(arrow::datatypes::DataType, Vec<String>),
    Literal(datafusion_common::ScalarValue),
    BinaryExpr(BinaryExpr),
    Like(Like),
    SimilarTo(Like),
    Not(Box<Expr>),
    IsNotNull(Box<Expr>),
    IsNull(Box<Expr>),
    IsTrue(Box<Expr>),
    IsFalse(Box<Expr>),
    IsUnknown(Box<Expr>),
    IsNotTrue(Box<Expr>),
    IsNotFalse(Box<Expr>),
    IsNotUnknown(Box<Expr>),
    Negative(Box<Expr>),
    Between(Between),
    Case(Case),
    Cast(Cast),
    TryCast(TryCast),
    ScalarFunction(ScalarFunction),
    AggregateFunction(AggregateFunction),
    WindowFunction(WindowFunction),
    InList(InList),
    Exists(Exists),
    InSubquery(InSubquery),
    ScalarSubquery(crate::logical_plan::Subquery),
    Wildcard {
        qualifier: Option<datafusion_common::TableReference>,
        options: Box<WildcardOptions>,
    },
    GroupingSet(GroupingSet),
    Placeholder(Placeholder),
    OuterReferenceColumn(arrow::datatypes::DataType, datafusion_common::Column),
    Unnest(Unnest),
}
```

##### Variants

###### `Alias`

An expression with a specific name.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Alias` |  |

###### `Column`

A named reference to a qualified field in a schema.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_common::Column` |  |

###### `ScalarVariable`

A named reference to a variable in a registry.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `arrow::datatypes::DataType` |  |
| 1 | `Vec<String>` |  |

###### `Literal`

A constant value.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_common::ScalarValue` |  |

###### `BinaryExpr`

A binary expression such as "age > 21"

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `BinaryExpr` |  |

###### `Like`

LIKE expression

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Like` |  |

###### `SimilarTo`

LIKE expression that uses regular expressions

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Like` |  |

###### `Not`

Negation of an expression. The expression's type must be a boolean to make sense.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `IsNotNull`

True if argument is not NULL, false otherwise. This expression itself is never NULL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `IsNull`

True if argument is NULL, false otherwise. This expression itself is never NULL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `IsTrue`

True if argument is true, false otherwise. This expression itself is never NULL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `IsFalse`

True if argument is  false, false otherwise. This expression itself is never NULL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `IsUnknown`

True if argument is NULL, false otherwise. This expression itself is never NULL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `IsNotTrue`

True if argument is FALSE or NULL, false otherwise. This expression itself is never NULL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `IsNotFalse`

True if argument is TRUE OR NULL, false otherwise. This expression itself is never NULL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `IsNotUnknown`

True if argument is TRUE or FALSE, false otherwise. This expression itself is never NULL.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `Negative`

arithmetic negation of an expression, the operand must be of a signed numeric data type

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<Expr>` |  |

###### `Between`

Whether an expression is between a given range.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Between` |  |

###### `Case`

The CASE expression is similar to a series of nested if/else and there are two forms that
can be used. The first form consists of a series of boolean "when" expressions with
corresponding "then" expressions, and an optional "else" expression.

```text
CASE WHEN condition THEN result
     [WHEN ...]
     [ELSE result]
END
```

The second form uses a base expression and then a series of "when" clauses that match on a
literal value.

```text
CASE expression
    WHEN value THEN result
    [WHEN ...]
    [ELSE result]
END
```

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Case` |  |

###### `Cast`

Casts the expression to a given type and will return a runtime error if the expression cannot be cast.
This expression is guaranteed to have a fixed type.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Cast` |  |

###### `TryCast`

Casts the expression to a given type and will return a null value if the expression cannot be cast.
This expression is guaranteed to have a fixed type.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `TryCast` |  |

###### `ScalarFunction`

Represents the call of a scalar function with a set of arguments.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ScalarFunction` |  |

###### `AggregateFunction`

Calls an aggregate function with arguments, and optional
`ORDER BY`, `FILTER`, `DISTINCT` and `NULL TREATMENT`.

See also [`ExprFunctionExt`] to set these fields.

[`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `AggregateFunction` |  |

###### `WindowFunction`

Represents the call of a window function with arguments.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `WindowFunction` |  |

###### `InList`

Returns whether the list contains the expr value.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InList` |  |

###### `Exists`

EXISTS subquery

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Exists` |  |

###### `InSubquery`

IN subquery

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InSubquery` |  |

###### `ScalarSubquery`

Scalar subquery

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::logical_plan::Subquery` |  |

###### `Wildcard`

Represents a reference to all available fields in a specific schema,
with an optional (schema) qualifier.

This expr has to be resolved to a list of columns before translating logical
plan into physical plan.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `qualifier` | `Option<datafusion_common::TableReference>` |  |
| `options` | `Box<WildcardOptions>` |  |

###### `GroupingSet`

List of grouping set expressions. Only valid in the context of an aggregate
GROUP BY expression list

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `GroupingSet` |  |

###### `Placeholder`

A place holder for parameters in a prepared statement
(e.g. `$foo` or `$1`)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Placeholder` |  |

###### `OuterReferenceColumn`

A place holder which hold a reference to a qualified field
in the outer query, used for correlated sub queries.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `arrow::datatypes::DataType` |  |
| 1 | `datafusion_common::Column` |  |

###### `Unnest`

Unnest expression

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Unnest` |  |

##### Implementations

###### Methods

- ```rust
  pub fn schema_name(self: &Self) -> impl Display + ''_ { /* ... */ }
  ```
  The name of the column (field) that this `Expr` will produce.

- ```rust
  pub fn human_display(self: &Self) -> impl Display + ''_ { /* ... */ }
  ```
  Human readable display formatting for this expression.

- ```rust
  pub fn qualified_name(self: &Self) -> (Option<TableReference>, String) { /* ... */ }
  ```
  Returns the qualifier and the schema name of this expression.

- ```rust
  pub fn canonical_name(self: &Self) -> String { /* ... */ }
  ```
  Returns a full and complete string representation of this expression.

- ```rust
  pub fn variant_name(self: &Self) -> &str { /* ... */ }
  ```
  Return String representation of the variant represented by `self`

- ```rust
  pub fn eq(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self == other`

- ```rust
  pub fn not_eq(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self != other`

- ```rust
  pub fn gt(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self > other`

- ```rust
  pub fn gt_eq(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self >= other`

- ```rust
  pub fn lt(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self < other`

- ```rust
  pub fn lt_eq(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self <= other`

- ```rust
  pub fn and(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self && other`

- ```rust
  pub fn or(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self || other`

- ```rust
  pub fn like(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self LIKE other`

- ```rust
  pub fn not_like(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self NOT LIKE other`

- ```rust
  pub fn ilike(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self ILIKE other`

- ```rust
  pub fn not_ilike(self: Self, other: Expr) -> Expr { /* ... */ }
  ```
  Return `self NOT ILIKE other`

- ```rust
  pub fn name_for_alias(self: &Self) -> Result<String> { /* ... */ }
  ```
  Return the name to use for the specific Expr

- ```rust
  pub fn alias_if_changed(self: Self, original_name: String) -> Result<Expr> { /* ... */ }
  ```
  Ensure `expr` has the name as `original_name` by adding an

- ```rust
  pub fn alias</* synthetic */ impl Into<String>: Into<String>>(self: Self, name: impl Into<String>) -> Expr { /* ... */ }
  ```
  Return `self AS name` alias expression

- ```rust
  pub fn alias_with_metadata</* synthetic */ impl Into<String>: Into<String>>(self: Self, name: impl Into<String>, metadata: Option<std::collections::HashMap<String, String>>) -> Expr { /* ... */ }
  ```
  Return `self AS name` alias expression with metadata

- ```rust
  pub fn alias_qualified</* synthetic */ impl Into<TableReference>: Into<TableReference>, /* synthetic */ impl Into<String>: Into<String>>(self: Self, relation: Option<impl Into<TableReference>>, name: impl Into<String>) -> Expr { /* ... */ }
  ```
  Return `self AS name` alias expression with a specific qualifier

- ```rust
  pub fn alias_qualified_with_metadata</* synthetic */ impl Into<TableReference>: Into<TableReference>, /* synthetic */ impl Into<String>: Into<String>>(self: Self, relation: Option<impl Into<TableReference>>, name: impl Into<String>, metadata: Option<std::collections::HashMap<String, String>>) -> Expr { /* ... */ }
  ```
  Return `self AS name` alias expression with a specific qualifier and metadata

- ```rust
  pub fn unalias(self: Self) -> Expr { /* ... */ }
  ```
  Remove an alias from an expression if one exists.

- ```rust
  pub fn unalias_nested(self: Self) -> Transformed<Expr> { /* ... */ }
  ```
  Recursively removed potentially multiple aliases from an expression.

- ```rust
  pub fn in_list(self: Self, list: Vec<Expr>, negated: bool) -> Expr { /* ... */ }
  ```
  Return `self IN <list>` if `negated` is false, otherwise

- ```rust
  pub fn is_null(self: Self) -> Expr { /* ... */ }
  ```
  Return `IsNull(Box(self))

- ```rust
  pub fn is_not_null(self: Self) -> Expr { /* ... */ }
  ```
  Return `IsNotNull(Box(self))

- ```rust
  pub fn sort(self: Self, asc: bool, nulls_first: bool) -> Sort { /* ... */ }
  ```
  Create a sort configuration from an existing expression.

- ```rust
  pub fn is_true(self: Self) -> Expr { /* ... */ }
  ```
  Return `IsTrue(Box(self))`

- ```rust
  pub fn is_not_true(self: Self) -> Expr { /* ... */ }
  ```
  Return `IsNotTrue(Box(self))`

- ```rust
  pub fn is_false(self: Self) -> Expr { /* ... */ }
  ```
  Return `IsFalse(Box(self))`

- ```rust
  pub fn is_not_false(self: Self) -> Expr { /* ... */ }
  ```
  Return `IsNotFalse(Box(self))`

- ```rust
  pub fn is_unknown(self: Self) -> Expr { /* ... */ }
  ```
  Return `IsUnknown(Box(self))`

- ```rust
  pub fn is_not_unknown(self: Self) -> Expr { /* ... */ }
  ```
  Return `IsNotUnknown(Box(self))`

- ```rust
  pub fn between(self: Self, low: Expr, high: Expr) -> Expr { /* ... */ }
  ```
  return `self BETWEEN low AND high`

- ```rust
  pub fn not_between(self: Self, low: Expr, high: Expr) -> Expr { /* ... */ }
  ```
  Return `self NOT BETWEEN low AND high`

- ```rust
  pub fn try_as_col(self: &Self) -> Option<&Column> { /* ... */ }
  ```
  Return a reference to the inner `Column` if any

- ```rust
  pub fn get_as_join_column(self: &Self) -> Option<&Column> { /* ... */ }
  ```
  Returns the inner `Column` if any. This is a specialized version of

- ```rust
  pub fn column_refs(self: &Self) -> HashSet<&Column> { /* ... */ }
  ```
  Return all references to columns in this expression.

- ```rust
  pub fn add_column_refs<''a>(self: &''a Self, set: &mut HashSet<&''a Column>) { /* ... */ }
  ```
  Adds references to all columns in this expression to the set

- ```rust
  pub fn column_refs_counts(self: &Self) -> HashMap<&Column, usize> { /* ... */ }
  ```
  Return all references to columns and their occurrence counts in the expression.

- ```rust
  pub fn add_column_ref_counts<''a>(self: &''a Self, map: &mut HashMap<&''a Column, usize>) { /* ... */ }
  ```
  Adds references to all columns and their occurrence counts in the expression to

- ```rust
  pub fn any_column_refs(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if there are any column references in this Expr

- ```rust
  pub fn contains_outer(self: &Self) -> bool { /* ... */ }
  ```
  Return true if the expression contains out reference(correlated) expressions.

- ```rust
  pub fn is_volatile_node(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the expression node is volatile, i.e. whether it can return

- ```rust
  pub fn is_volatile(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if the expression is volatile, i.e. whether it can return different

- ```rust
  pub fn infer_placeholder_types(self: Self, schema: &DFSchema) -> Result<(Expr, bool)> { /* ... */ }
  ```
  Recursively find all [`Expr::Placeholder`] expressions, and

- ```rust
  pub fn short_circuits(self: &Self) -> bool { /* ... */ }
  ```
  Returns true if some of this `exprs` subexpressions may not be evaluated

- ```rust
  pub fn spans(self: &Self) -> Option<&Spans> { /* ... */ }
  ```
  Returns a reference to the set of locations in the SQL query where this

###### Trait Implementations

- **Not**
  - ```rust
    fn not(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
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
    fn clone(self: &Self) -> Expr { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BitOr**
  - ```rust
    fn bitor(self: Self, rhs: Self) -> Self { /* ... */ }
    ```

- **StructuralPartialEq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **BitAnd**
  - ```rust
    fn bitand(self: Self, rhs: Self) -> Self { /* ... */ }
    ```

- **Unpin**
- **Div**
  - ```rust
    fn div(self: Self, rhs: Self) -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **Rem**
  - ```rust
    fn rem(self: Self, rhs: Self) -> Self { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Expr) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **HashNode**
  - ```rust
    fn hash_node<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```
    As it is pretty easy to forget changing this method when `Expr` changes the

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **NumOps**
- **ErasedDestructor**
- **IntoEither**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Sub**
  - ```rust
    fn sub(self: Self, rhs: Self) -> Self { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Neg**
  - ```rust
    fn neg(self: Self) -> <Self as >::Output { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: Column) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: (Option<&''a TableReference>, &''a FieldRef)) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(expr: Expr) -> Self { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Shr**
  - ```rust
    fn shr(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Expr) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BitXor**
  - ```rust
    fn bitxor(self: Self, rhs: Self) -> Self { /* ... */ }
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

- **TreeNodeContainer**
  - ```rust
    fn apply_elements<F: FnMut(&''a Self) -> Result<TreeNodeRecursion>>(self: &''a Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
    ```

  - ```rust
    fn map_elements<F: FnMut(Self) -> Result<Transformed<Self>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
    ```

  - ```rust
    fn apply_elements<F: FnMut(&''a Expr) -> Result<TreeNodeRecursion>>(self: &''a Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
    ```

  - ```rust
    fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
    ```

  - ```rust
    fn apply_elements<F: FnMut(&''a Expr) -> Result<TreeNodeRecursion>>(self: &''a Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
    ```

  - ```rust
    fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
    ```

  - ```rust
    fn apply_elements<F: FnMut(&''a Expr) -> Result<TreeNodeRecursion>>(self: &''a Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
    ```

  - ```rust
    fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
    ```

- **NormalizeEq**
  - ```rust
    fn normalize_eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **ExprFunctionExt**
  - ```rust
    fn order_by(self: Self, order_by: Vec<Sort>) -> ExprFuncBuilder { /* ... */ }
    ```

  - ```rust
    fn filter(self: Self, filter: Expr) -> ExprFuncBuilder { /* ... */ }
    ```

  - ```rust
    fn distinct(self: Self) -> ExprFuncBuilder { /* ... */ }
    ```

  - ```rust
    fn null_treatment</* synthetic */ impl Into<Option<NullTreatment>>: Into<Option<NullTreatment>>>(self: Self, null_treatment: impl Into<Option<NullTreatment>>) -> ExprFuncBuilder { /* ... */ }
    ```

  - ```rust
    fn partition_by(self: Self, partition_by: Vec<Expr>) -> ExprFuncBuilder { /* ... */ }
    ```

  - ```rust
    fn window_frame(self: Self, window_frame: WindowFrame) -> ExprFuncBuilder { /* ... */ }
    ```

- **Normalizeable**
  - ```rust
    fn can_normalize(self: &Self) -> bool { /* ... */ }
    ```

- **ExprSchemable**
  - ```rust
    fn get_type(self: &Self, schema: &dyn ExprSchema) -> Result<DataType> { /* ... */ }
    ```
    Returns the [arrow::datatypes::DataType] of the expression

  - ```rust
    fn nullable(self: &Self, input_schema: &dyn ExprSchema) -> Result<bool> { /* ... */ }
    ```
    Returns the nullability of the expression based on [ExprSchema].

  - ```rust
    fn metadata(self: &Self, schema: &dyn ExprSchema) -> Result<HashMap<String, String>> { /* ... */ }
    ```

  - ```rust
    fn data_type_and_nullable(self: &Self, schema: &dyn ExprSchema) -> Result<(DataType, bool)> { /* ... */ }
    ```
    Returns the datatype and nullability of the expression based on [ExprSchema].

  - ```rust
    fn to_field(self: &Self, input_schema: &dyn ExprSchema) -> Result<(Option<TableReference>, Arc<Field>)> { /* ... */ }
    ```
    Returns a [arrow::datatypes::Field] compatible with this expression.

  - ```rust
    fn cast_to(self: Self, cast_to_type: &DataType, schema: &dyn ExprSchema) -> Result<Expr> { /* ... */ }
    ```
    Wraps this expression in a cast to a target [arrow::datatypes::DataType].

- **Add**
  - ```rust
    fn add(self: Self, rhs: Self) -> Self { /* ... */ }
    ```

- **Shl**
  - ```rust
    fn shl(self: Self, rhs: Self) -> <Self as >::Output { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Mul**
  - ```rust
    fn mul(self: Self, rhs: Self) -> Self { /* ... */ }
    ```

- **TreeNode**
  - ```rust
    fn apply_children<''n, F: FnMut(&''n Self) -> Result<TreeNodeRecursion>>(self: &''n Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
    ```
    Applies a function `f` to each child expression of `self`.

  - ```rust
    fn map_children<F: FnMut(Self) -> Result<Transformed<Self>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
    ```
    Maps each child of `self` using the provided closure `f`.

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

#### Struct `Unnest`

UNNEST expression.

```rust
pub struct Unnest {
    pub expr: Box<Expr>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Box<Expr>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Expr) -> Self { /* ... */ }
  ```
  Create a new Unnest expression.

- ```rust
  pub fn new_boxed(boxed: Box<Expr>) -> Self { /* ... */ }
  ```
  Create a new Unnest expression.

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Unnest { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Sync**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Unnest) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Unnest) -> bool { /* ... */ }
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
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `Alias`

Alias expression

```rust
pub struct Alias {
    pub expr: Box<Expr>,
    pub relation: Option<datafusion_common::TableReference>,
    pub name: String,
    pub metadata: Option<std::collections::HashMap<String, String>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Box<Expr>` |  |
| `relation` | `Option<datafusion_common::TableReference>` |  |
| `name` | `String` |  |
| `metadata` | `Option<std::collections::HashMap<String, String>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new</* synthetic */ impl Into<TableReference>: Into<TableReference>, /* synthetic */ impl Into<String>: Into<String>>(expr: Expr, relation: Option<impl Into<TableReference>>, name: impl Into<String>) -> Self { /* ... */ }
  ```
  Create an alias with an optional schema/field qualifier.

- ```rust
  pub fn with_metadata(self: Self, metadata: Option<std::collections::HashMap<String, String>>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Alias) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Alias { /* ... */ }
    ```

- **StructuralPartialEq**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<std::cmp::Ordering> { /* ... */ }
    ```

- **MaybeSendSync**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
- **ErasedDestructor**
- **IntoEither**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Struct `BinaryExpr`

Binary expression

```rust
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: crate::Operator,
    pub right: Box<Expr>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `left` | `Box<Expr>` | Left-hand side of the expression |
| `op` | `crate::Operator` | The comparison operator |
| `right` | `Box<Expr>` | Right-hand side of the expression |

##### Implementations

###### Methods

- ```rust
  pub fn new(left: Box<Expr>, op: Operator, right: Box<Expr>) -> Self { /* ... */ }
  ```
  Create a new binary expression

###### Trait Implementations

- **ErasedDestructor**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &BinaryExpr) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **StructuralPartialEq**
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Freeze**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &BinaryExpr) -> bool { /* ... */ }
    ```

- **Eq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> BinaryExpr { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

#### Struct `Case`

CASE expression

```rust
pub struct Case {
    pub expr: Option<Box<Expr>>,
    pub when_then_expr: Vec<(Box<Expr>, Box<Expr>)>,
    pub else_expr: Option<Box<Expr>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Option<Box<Expr>>` | Optional base expression that can be compared to literal values in the "when" expressions |
| `when_then_expr` | `Vec<(Box<Expr>, Box<Expr>)>` | One or more when/then expressions |
| `else_expr` | `Option<Box<Expr>>` | Optional "else" expression |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Option<Box<Expr>>, when_then_expr: Vec<(Box<Expr>, Box<Expr>)>, else_expr: Option<Box<Expr>>) -> Self { /* ... */ }
  ```
  Create a new Case expression

###### Trait Implementations

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
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

- **Unpin**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Case) -> bool { /* ... */ }
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

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Case { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Case) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `Like`

LIKE expression

```rust
pub struct Like {
    pub negated: bool,
    pub expr: Box<Expr>,
    pub pattern: Box<Expr>,
    pub escape_char: Option<char>,
    pub case_insensitive: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `negated` | `bool` |  |
| `expr` | `Box<Expr>` |  |
| `pattern` | `Box<Expr>` |  |
| `escape_char` | `Option<char>` |  |
| `case_insensitive` | `bool` | Whether to ignore case on comparing |

##### Implementations

###### Methods

- ```rust
  pub fn new(negated: bool, expr: Box<Expr>, pattern: Box<Expr>, escape_char: Option<char>, case_insensitive: bool) -> Self { /* ... */ }
  ```
  Create a new Like expression

###### Trait Implementations

- **Eq**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Like) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **UnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Like { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Like) -> bool { /* ... */ }
    ```

#### Struct `Between`

BETWEEN expression

```rust
pub struct Between {
    pub expr: Box<Expr>,
    pub negated: bool,
    pub low: Box<Expr>,
    pub high: Box<Expr>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Box<Expr>` | The value to compare |
| `negated` | `bool` | Whether the expression is negated |
| `low` | `Box<Expr>` | The low end of the range |
| `high` | `Box<Expr>` | The high end of the range |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Box<Expr>, negated: bool, low: Box<Expr>, high: Box<Expr>) -> Self { /* ... */ }
  ```
  Create a new Between expression

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Between) -> bool { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Between { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Between) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Eq**
#### Struct `ScalarFunction`

ScalarFunction expression invokes a built-in scalar function

```rust
pub struct ScalarFunction {
    pub func: std::sync::Arc<crate::ScalarUDF>,
    pub args: Vec<Expr>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `func` | `std::sync::Arc<crate::ScalarUDF>` | The function |
| `args` | `Vec<Expr>` | List of expressions to feed to the functions as arguments |

##### Implementations

###### Methods

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```

- ```rust
  pub fn new_udf(udf: Arc<crate::ScalarUDF>, args: Vec<Expr>) -> Self { /* ... */ }
  ```
  Create a new ScalarFunction expression with a user-defined function (UDF)

###### Trait Implementations

- **Eq**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &ScalarFunction) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ScalarFunction { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ScalarFunction) -> bool { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
#### Enum `GetFieldAccess`

Access a sub field of a nested type, such as `Field` or `List`

```rust
pub enum GetFieldAccess {
    NamedStructField {
        name: datafusion_common::ScalarValue,
    },
    ListIndex {
        key: Box<Expr>,
    },
    ListRange {
        start: Box<Expr>,
        stop: Box<Expr>,
        stride: Box<Expr>,
    },
}
```

##### Variants

###### `NamedStructField`

Named field, for example `struct["name"]`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `datafusion_common::ScalarValue` |  |

###### `ListIndex`

Single list index, for example: `list[i]`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `key` | `Box<Expr>` |  |

###### `ListRange`

List stride, for example `list[i:j:k]`

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `start` | `Box<Expr>` |  |
| `stop` | `Box<Expr>` |  |
| `stride` | `Box<Expr>` |  |

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GetFieldAccess { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **Send**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
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

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GetFieldAccess) -> bool { /* ... */ }
    ```

#### Struct `Cast`

Cast expression

```rust
pub struct Cast {
    pub expr: Box<Expr>,
    pub data_type: arrow::datatypes::DataType,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Box<Expr>` | The expression being cast |
| `data_type` | `arrow::datatypes::DataType` | The `DataType` the expression will yield |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Box<Expr>, data_type: DataType) -> Self { /* ... */ }
  ```
  Create a new Cast expression

###### Trait Implementations

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **MaybeSendSync**
- **Eq**
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
    fn partial_cmp(self: &Self, other: &Cast) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Cast { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Cast) -> bool { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **ErasedDestructor**
#### Struct `TryCast`

TryCast Expression

```rust
pub struct TryCast {
    pub expr: Box<Expr>,
    pub data_type: arrow::datatypes::DataType,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Box<Expr>` | The expression being cast |
| `data_type` | `arrow::datatypes::DataType` | The `DataType` the expression will yield |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Box<Expr>, data_type: DataType) -> Self { /* ... */ }
  ```
  Create a new TryCast expression

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

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> TryCast { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &TryCast) -> bool { /* ... */ }
    ```

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &TryCast) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Sync**
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

- **ErasedDestructor**
- **IntoEither**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Unpin**
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
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
#### Struct `Sort`

SORT expression

```rust
pub struct Sort {
    pub expr: Expr,
    pub asc: bool,
    pub nulls_first: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Expr` | The expression to sort on |
| `asc` | `bool` | The direction of the sort |
| `nulls_first` | `bool` | Whether to put Nulls before all other data values |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Expr, asc: bool, nulls_first: bool) -> Self { /* ... */ }
  ```
  Create a new Sort expression

- ```rust
  pub fn reverse(self: &Self) -> Self { /* ... */ }
  ```
  Create a new Sort expression with the opposite sort direction

- ```rust
  pub fn with_expr(self: &Self, expr: Expr) -> Self { /* ... */ }
  ```
  Replaces the Sort expressions with `expr`

###### Trait Implementations

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Sort) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Sort { /* ... */ }
    ```

- **Freeze**
- **Eq**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TreeNodeContainer**
  - ```rust
    fn apply_elements<F: FnMut(&''a Expr) -> Result<TreeNodeRecursion>>(self: &''a Self, f: F) -> Result<TreeNodeRecursion> { /* ... */ }
    ```

  - ```rust
    fn map_elements<F: FnMut(Expr) -> Result<Transformed<Expr>>>(self: Self, f: F) -> Result<Transformed<Self>> { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Sort) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **MaybeSendSync**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **StructuralPartialEq**
#### Struct `AggregateFunction`

Aggregate function

See also  [`ExprFunctionExt`] to set these fields on `Expr`

[`ExprFunctionExt`]: crate::expr_fn::ExprFunctionExt

```rust
pub struct AggregateFunction {
    pub func: std::sync::Arc<crate::AggregateUDF>,
    pub params: AggregateFunctionParams,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `func` | `std::sync::Arc<crate::AggregateUDF>` | Name of the function |
| `params` | `AggregateFunctionParams` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new_udf(func: Arc<crate::AggregateUDF>, args: Vec<Expr>, distinct: bool, filter: Option<Box<Expr>>, order_by: Option<Vec<Sort>>, null_treatment: Option<NullTreatment>) -> Self { /* ... */ }
  ```
  Create a new AggregateFunction expression with a user-defined function (UDF)

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AggregateFunction { /* ... */ }
    ```

- **StructuralPartialEq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &AggregateFunction) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AggregateFunction) -> bool { /* ... */ }
    ```

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **ErasedDestructor**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

#### Struct `AggregateFunctionParams`

```rust
pub struct AggregateFunctionParams {
    pub args: Vec<Expr>,
    pub distinct: bool,
    pub filter: Option<Box<Expr>>,
    pub order_by: Option<Vec<Sort>>,
    pub null_treatment: Option<sqlparser::ast::NullTreatment>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `args` | `Vec<Expr>` |  |
| `distinct` | `bool` | Whether this is a DISTINCT aggregation or not |
| `filter` | `Option<Box<Expr>>` | Optional filter |
| `order_by` | `Option<Vec<Sort>>` | Optional ordering |
| `null_treatment` | `Option<sqlparser::ast::NullTreatment>` |  |

##### Implementations

###### Trait Implementations

- **Eq**
- **Sync**
- **UnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &AggregateFunctionParams) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> AggregateFunctionParams { /* ... */ }
    ```

- **RefUnwindSafe**
- **StructuralPartialEq**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **IntoEither**
- **Send**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AggregateFunctionParams) -> bool { /* ... */ }
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

#### Enum `WindowFunctionDefinition`

A function used as a SQL window function

In SQL, you can use:
- Actual window functions ([`WindowUDF`])
- Normal aggregate functions ([`AggregateUDF`])

```rust
pub enum WindowFunctionDefinition {
    AggregateUDF(std::sync::Arc<crate::AggregateUDF>),
    WindowUDF(std::sync::Arc<crate::WindowUDF>),
}
```

##### Variants

###### `AggregateUDF`

A user defined aggregate function

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<crate::AggregateUDF>` |  |

###### `WindowUDF`

A user defined aggregate function

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<crate::WindowUDF>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn return_type(self: &Self, input_expr_types: &[DataType], _input_expr_nullable: &[bool], display_name: &str) -> Result<DataType> { /* ... */ }
  ```
  Returns the datatype of the window function

- ```rust
  pub fn signature(self: &Self) -> Signature { /* ... */ }
  ```
  The signatures supported by the function `fun`.

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Function's name for display

###### Trait Implementations

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: Arc<crate::AggregateUDF>) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Arc<WindowUDF>) -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WindowFunctionDefinition) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WindowFunctionDefinition) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Unpin**
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

- **StructuralPartialEq**
- **ErasedDestructor**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **IntoEither**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowFunctionDefinition { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `WindowFunction`

Window function

Holds the actual function to call [`WindowFunction`] as well as its
arguments (`args`) and the contents of the `OVER` clause:

1. `PARTITION BY`
2. `ORDER BY`
3. Window frame (e.g. `ROWS 1 PRECEDING AND 1 FOLLOWING`)

See [`ExprFunctionExt`] for examples of how to create a `WindowFunction`.

[`ExprFunctionExt`]: crate::ExprFunctionExt

```rust
pub struct WindowFunction {
    pub fun: WindowFunctionDefinition,
    pub params: WindowFunctionParams,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `fun` | `WindowFunctionDefinition` | Name of the function |
| `params` | `WindowFunctionParams` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new</* synthetic */ impl Into<WindowFunctionDefinition>: Into<WindowFunctionDefinition>>(fun: impl Into<WindowFunctionDefinition>, args: Vec<Expr>) -> Self { /* ... */ }
  ```
  Create a new Window expression with the specified argument an

###### Trait Implementations

- **Eq**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WindowFunction) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowFunction { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **Unpin**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WindowFunction) -> bool { /* ... */ }
    ```

#### Struct `WindowFunctionParams`

```rust
pub struct WindowFunctionParams {
    pub args: Vec<Expr>,
    pub partition_by: Vec<Expr>,
    pub order_by: Vec<Sort>,
    pub window_frame: crate::WindowFrame,
    pub null_treatment: Option<sqlparser::ast::NullTreatment>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `args` | `Vec<Expr>` | List of expressions to feed to the functions as arguments |
| `partition_by` | `Vec<Expr>` | List of partition by expressions |
| `order_by` | `Vec<Sort>` | List of order by expressions |
| `window_frame` | `crate::WindowFrame` | Window frame |
| `null_treatment` | `Option<sqlparser::ast::NullTreatment>` | Specifies how NULL value is treated: ignore or respect |

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Eq**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **StructuralPartialEq**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowFunctionParams { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WindowFunctionParams) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WindowFunctionParams) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
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

- **RefUnwindSafe**
#### Struct `Exists`

EXISTS expression

```rust
pub struct Exists {
    pub subquery: crate::logical_plan::Subquery,
    pub negated: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `subquery` | `crate::logical_plan::Subquery` | Subquery that will produce a single column of data |
| `negated` | `bool` | Whether the expression is negated |

##### Implementations

###### Methods

- ```rust
  pub fn new(subquery: Subquery, negated: bool) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Sync**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> Exists { /* ... */ }
    ```

- **RefUnwindSafe**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Exists) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
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

- **UnwindSafe**
- **IntoEither**
- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Exists) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `AggregateUDF`

User Defined Aggregate Function

See [`udaf::AggregateUDF`] for more information.

```rust
pub struct AggregateUDF {
    pub fun: std::sync::Arc<udaf::AggregateUDF>,
    pub args: Vec<Expr>,
    pub filter: Option<Box<Expr>>,
    pub order_by: Option<Vec<Expr>>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `fun` | `std::sync::Arc<udaf::AggregateUDF>` | The function |
| `args` | `Vec<Expr>` | List of expressions to feed to the functions as arguments |
| `filter` | `Option<Box<Expr>>` | Optional filter |
| `order_by` | `Option<Vec<Expr>>` | Optional ORDER BY applied prior to aggregating |

##### Implementations

###### Methods

- ```rust
  pub fn new(fun: Arc<udaf::AggregateUDF>, args: Vec<Expr>, filter: Option<Box<Expr>>, order_by: Option<Vec<Expr>>) -> Self { /* ... */ }
  ```
  Create a new AggregateUDF expression

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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Send**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> AggregateUDF { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &AggregateUDF) -> bool { /* ... */ }
    ```

- **Eq**
- **Freeze**
- **MaybeSendSync**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `InList`

InList expression

```rust
pub struct InList {
    pub expr: Box<Expr>,
    pub list: Vec<Expr>,
    pub negated: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Box<Expr>` | The expression to compare |
| `list` | `Vec<Expr>` | The list of values to compare against |
| `negated` | `bool` | Whether the expression is negated |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Box<Expr>, list: Vec<Expr>, negated: bool) -> Self { /* ... */ }
  ```
  Create a new InList expression

###### Trait Implementations

- **StructuralPartialEq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &InList) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **UnwindSafe**
- **ErasedDestructor**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InList) -> bool { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InList { /* ... */ }
    ```

#### Struct `InSubquery`

IN subquery

```rust
pub struct InSubquery {
    pub expr: Box<Expr>,
    pub subquery: crate::logical_plan::Subquery,
    pub negated: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `expr` | `Box<Expr>` | The expression to compare |
| `subquery` | `crate::logical_plan::Subquery` | Subquery that will produce a single column of data to compare against |
| `negated` | `bool` | Whether the expression is negated |

##### Implementations

###### Methods

- ```rust
  pub fn new(expr: Box<Expr>, subquery: Subquery, negated: bool) -> Self { /* ... */ }
  ```
  Create a new InSubquery expression

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **IntoEither**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InSubquery { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **StructuralPartialEq**
- **Unpin**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &InSubquery) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InSubquery) -> bool { /* ... */ }
    ```

#### Struct `Placeholder`

Placeholder, representing bind parameter values such as `$1` or `$name`.

The type of these parameters is inferred using [`Expr::infer_placeholder_types`]
or can be specified directly using `PREPARE` statements.

```rust
pub struct Placeholder {
    pub id: String,
    pub data_type: Option<arrow::datatypes::DataType>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `id` | `String` | The identifier of the parameter, including the leading `$` (e.g, `"$1"` or `"$foo"`) |
| `data_type` | `Option<arrow::datatypes::DataType>` | The type the parameter will be filled in with |

##### Implementations

###### Methods

- ```rust
  pub fn new(id: String, data_type: Option<DataType>) -> Self { /* ... */ }
  ```
  Create a new Placeholder expression

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Placeholder) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Placeholder { /* ... */ }
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

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Eq**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **IntoEither**
- **RefUnwindSafe**
- **Sync**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Placeholder) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Allocation**
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

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Enum `GroupingSet`

Grouping sets

See <https://www.postgresql.org/docs/current/queries-table-expressions.html#QUERIES-GROUPING-SETS>
for Postgres definition.
See <https://spark.apache.org/docs/latest/sql-ref-syntax-qry-select-groupby.html>
for Apache Spark definition.

```rust
pub enum GroupingSet {
    Rollup(Vec<Expr>),
    Cube(Vec<Expr>),
    GroupingSets(Vec<Vec<Expr>>),
}
```

##### Variants

###### `Rollup`

Rollup grouping sets

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Expr>` |  |

###### `Cube`

Cube grouping sets

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Expr>` |  |

###### `GroupingSets`

User-defined grouping sets

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<Vec<Expr>>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn distinct_expr(self: &Self) -> Vec<&Expr> { /* ... */ }
  ```
  Return all distinct exprs in the grouping set. For `CUBE` and `ROLLUP` this

###### Trait Implementations

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &GroupingSet) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **StructuralPartialEq**
- **IntoEither**
- **Freeze**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &GroupingSet) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
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

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> GroupingSet { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
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

- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `WildcardOptions`

Additional options for wildcards, e.g. Snowflake `EXCLUDE`/`RENAME` and Bigquery `EXCEPT`.

```rust
pub struct WildcardOptions {
    pub ilike: Option<sqlparser::ast::IlikeSelectItem>,
    pub exclude: Option<sqlparser::ast::ExcludeSelectItem>,
    pub except: Option<sqlparser::ast::ExceptSelectItem>,
    pub replace: Option<PlannedReplaceSelectItem>,
    pub rename: Option<sqlparser::ast::RenameSelectItem>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `ilike` | `Option<sqlparser::ast::IlikeSelectItem>` | `[ILIKE...]`.<br> Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters> |
| `exclude` | `Option<sqlparser::ast::ExcludeSelectItem>` | `[EXCLUDE...]`.<br> Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters> |
| `except` | `Option<sqlparser::ast::ExceptSelectItem>` | `[EXCEPT...]`.<br> BigQuery syntax: <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_except><br> Clickhouse syntax: <https://clickhouse.com/docs/en/sql-reference/statements/select#except> |
| `replace` | `Option<PlannedReplaceSelectItem>` | `[REPLACE]`<br> BigQuery syntax: <https://cloud.google.com/bigquery/docs/reference/standard-sql/query-syntax#select_replace><br> Clickhouse syntax: <https://clickhouse.com/docs/en/sql-reference/statements/select#replace><br> Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters> |
| `rename` | `Option<sqlparser::ast::RenameSelectItem>` | `[RENAME ...]`.<br> Snowflake syntax: <https://docs.snowflake.com/en/sql-reference/sql/select#parameters> |

##### Implementations

###### Methods

- ```rust
  pub fn with_replace(self: Self, replace: PlannedReplaceSelectItem) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WildcardOptions) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **UnwindSafe**
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

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WildcardOptions { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> WildcardOptions { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WildcardOptions) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **IntoEither**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
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

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
#### Struct `PlannedReplaceSelectItem`

The planned expressions for `REPLACE`

```rust
pub struct PlannedReplaceSelectItem {
    pub items: Vec<sqlparser::ast::ReplaceSelectElement>,
    pub planned_expressions: Vec<Expr>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `items` | `Vec<sqlparser::ast::ReplaceSelectElement>` | The original ast nodes |
| `planned_expressions` | `Vec<Expr>` | The expression planned from the ast nodes. They will be used when expanding the wildcard. |

##### Implementations

###### Methods

- ```rust
  pub fn items(self: &Self) -> &[ReplaceSelectElement] { /* ... */ }
  ```

- ```rust
  pub fn expressions(self: &Self) -> &[Expr] { /* ... */ }
  ```

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &PlannedReplaceSelectItem) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> PlannedReplaceSelectItem { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &PlannedReplaceSelectItem) -> bool { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PlannedReplaceSelectItem { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **IntoEither**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **UnwindSafe**
- **Send**
#### Struct `ExprListDisplay`

Formats a list of `&Expr` with a custom separator using SQL display format

```rust
pub struct ExprListDisplay<''a> {
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
  pub fn new(exprs: &''a [Expr], sep: &''a str) -> Self { /* ... */ }
  ```
  Create a new display struct with the given expressions and separator

- ```rust
  pub fn comma_separated(exprs: &''a [Expr]) -> Self { /* ... */ }
  ```
  Create a new display struct with comma-space separator

###### Trait Implementations

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **Sync**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
- **ErasedDestructor**
### Functions

#### Function `schema_name_from_exprs`

Get schema_name for Vector of expressions

```rust
pub fn schema_name_from_exprs(exprs: &[Expr]) -> datafusion_common::Result<String, fmt::Error> { /* ... */ }
```

#### Function `schema_name_from_sorts`

```rust
pub fn schema_name_from_sorts(sorts: &[Sort]) -> datafusion_common::Result<String, fmt::Error> { /* ... */ }
```

#### Function `physical_name`

The name of the column (field) that this `Expr` will produce in the physical plan.
The difference from [Expr::schema_name] is that top-level columns are unqualified.

```rust
pub fn physical_name(expr: &Expr) -> datafusion_common::Result<String> { /* ... */ }
```

### Constants and Statics

#### Constant `OUTER_REFERENCE_COLUMN_PREFIX`

```rust
pub const OUTER_REFERENCE_COLUMN_PREFIX: &str = "outer_ref";
```

#### Constant `UNNEST_COLUMN_PREFIX`

```rust
pub const UNNEST_COLUMN_PREFIX: &str = "UNNEST";
```

## Module `expr_fn`

Functions for creating logical expressions

```rust
pub mod expr_fn { /* ... */ }
```

### Types

#### Struct `SimpleScalarUDF`

Implements [`ScalarUDFImpl`] for functions that have a single signature and
return type.

```rust
pub struct SimpleScalarUDF {
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
  pub fn new</* synthetic */ impl Into<String>: Into<String>>(name: impl Into<String>, input_types: Vec<DataType>, return_type: DataType, volatility: Volatility, fun: ScalarFunctionImplementation) -> Self { /* ... */ }
  ```
  Create a new `SimpleScalarUDF` from a name, input types, return type and

- ```rust
  pub fn new_with_signature</* synthetic */ impl Into<String>: Into<String>>(name: impl Into<String>, signature: Signature, return_type: DataType, fun: ScalarFunctionImplementation) -> Self { /* ... */ }
  ```
  Create a new `SimpleScalarUDF` from a name, signature, return type and

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Send**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **ScalarUDFImpl**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **IntoEither**
- **Freeze**
#### Struct `SimpleAggregateUDF`

Implements [`AggregateUDFImpl`] for functions that have a single signature and
return type.

```rust
pub struct SimpleAggregateUDF {
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
  pub fn new</* synthetic */ impl Into<String>: Into<String>>(name: impl Into<String>, input_type: Vec<DataType>, return_type: DataType, volatility: Volatility, accumulator: AccumulatorFactoryFunction, state_fields: Vec<Field>) -> Self { /* ... */ }
  ```
  Create a new `SimpleAggregateUDF` from a name, input types, return type, state type and

- ```rust
  pub fn new_with_signature</* synthetic */ impl Into<String>: Into<String>>(name: impl Into<String>, signature: Signature, return_type: DataType, accumulator: AccumulatorFactoryFunction, state_fields: Vec<Field>) -> Self { /* ... */ }
  ```
  Create a new `SimpleAggregateUDF` from a name, signature, return type, state type and

###### Trait Implementations

- **ErasedDestructor**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn crate::Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, _args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Freeze**
- **Sync**
- **Send**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `SimpleWindowUDF`

Implements [`WindowUDFImpl`] for functions that have a single signature and
return type.

```rust
pub struct SimpleWindowUDF {
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
  pub fn new</* synthetic */ impl Into<String>: Into<String>>(name: impl Into<String>, input_type: DataType, return_type: DataType, volatility: Volatility, partition_evaluator_factory: PartitionEvaluatorFactory) -> Self { /* ... */ }
  ```
  Create a new `SimpleWindowUDF` from a name, input types, return type and

###### Trait Implementations

- **Sync**
- **Unpin**
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

- **Send**
- **Freeze**
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
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Enum `ExprFuncKind`

```rust
pub enum ExprFuncKind {
    Aggregate(crate::expr::AggregateFunction),
    Window(crate::expr::WindowFunction),
}
```

##### Variants

###### `Aggregate`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::expr::AggregateFunction` |  |

###### `Window`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::expr::WindowFunction` |  |

##### Implementations

###### Trait Implementations

- **IntoEither**
- **ErasedDestructor**
- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **UnwindSafe**
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

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExprFuncKind { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
#### Struct `ExprFuncBuilder`

Implementation of [`ExprFunctionExt`].

See [`ExprFunctionExt`] for usage and examples

```rust
pub struct ExprFuncBuilder {
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
  pub fn build(self: Self) -> Result<Expr> { /* ... */ }
  ```
  Updates and returns the in progress [`Expr::AggregateFunction`] or [`Expr::WindowFunction`]

###### Trait Implementations

- **RefUnwindSafe**
- **Freeze**
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

- **Sync**
- **IntoEither**
- **ExprFunctionExt**
  - ```rust
    fn order_by(self: Self, order_by: Vec<Sort>) -> ExprFuncBuilder { /* ... */ }
    ```
    Add `ORDER BY <order_by>`

  - ```rust
    fn filter(self: Self, filter: Expr) -> ExprFuncBuilder { /* ... */ }
    ```
    Add `FILTER <filter>`

  - ```rust
    fn distinct(self: Self) -> ExprFuncBuilder { /* ... */ }
    ```
    Add `DISTINCT`

  - ```rust
    fn null_treatment</* synthetic */ impl Into<Option<NullTreatment>>: Into<Option<NullTreatment>>>(self: Self, null_treatment: impl Into<Option<NullTreatment>>) -> ExprFuncBuilder { /* ... */ }
    ```
    Add `RESPECT NULLS` or `IGNORE NULLS`

  - ```rust
    fn partition_by(self: Self, partition_by: Vec<Expr>) -> ExprFuncBuilder { /* ... */ }
    ```

  - ```rust
    fn window_frame(self: Self, window_frame: WindowFrame) -> ExprFuncBuilder { /* ... */ }
    ```

- **Unpin**
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
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExprFuncBuilder { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

### Traits

#### Trait `ExprFunctionExt`

Extensions for configuring [`Expr::AggregateFunction`] or [`Expr::WindowFunction`]

Adds methods to [`Expr`] that make it easy to set optional options
such as `ORDER BY`, `FILTER` and `DISTINCT`

# Example
```no_run
# use datafusion_common::Result;
# use datafusion_expr::test::function_stub::count;
# use sqlparser::ast::NullTreatment;
# use datafusion_expr::{ExprFunctionExt, lit, Expr, col};
# // first_value is an aggregate function in another crate
# fn first_value(_arg: Expr) -> Expr {
unimplemented!() }
# fn main() -> Result<()> {
// Create an aggregate count, filtering on column y > 5
let agg = count(col("x")).filter(col("y").gt(lit(5))).build()?;

// Find the first value in an aggregate sorted by column y
// equivalent to:
// `FIRST_VALUE(x ORDER BY y ASC IGNORE NULLS)`
let sort_expr = col("y").sort(true, true);
let agg = first_value(col("x"))
    .order_by(vec![sort_expr])
    .null_treatment(NullTreatment::IgnoreNulls)
    .build()?;

// Create a window expression for percent rank partitioned on column a
// equivalent to:
// `PERCENT_RANK() OVER (PARTITION BY a ORDER BY b ASC NULLS LAST IGNORE NULLS)`
// percent_rank is an udwf function in another crate
# fn percent_rank() -> Expr {
unimplemented!() }
let window = percent_rank()
    .partition_by(vec![col("a")])
    .order_by(vec![col("b").sort(true, true)])
    .null_treatment(NullTreatment::IgnoreNulls)
    .build()?;
#     Ok(())
# }
```

```rust
pub trait ExprFunctionExt {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `order_by`: Add `ORDER BY <order_by>`
- `filter`: Add `FILTER <filter>`
- `distinct`: Add `DISTINCT`
- `null_treatment`: Add `RESPECT NULLS` or `IGNORE NULLS`
- `partition_by`: Add `PARTITION BY`
- `window_frame`: Add appropriate window frame conditions

##### Implementations

This trait is implemented for the following types:

- `ExprFuncBuilder`
- `crate::Expr`

### Functions

#### Function `col`

Create a column expression based on a qualified or unqualified column name. Will
normalize unquoted identifiers according to SQL rules (identifiers will become lowercase).

For example:

```rust
# use datafusion_expr::col;
let c1 = col("a");
let c2 = col("A");
assert_eq!(c1, c2);

// note how quoting with double quotes preserves the case
let c3 = col(r#""A""#);
assert_ne!(c1, c3);
```

```rust
pub fn col</* synthetic */ impl Into<Column>: Into<datafusion_common::Column>>(ident: impl Into<datafusion_common::Column>) -> crate::Expr { /* ... */ }
```

#### Function `out_ref_col`

Create an out reference column which hold a reference that has been resolved to a field
outside of the current plan.

```rust
pub fn out_ref_col</* synthetic */ impl Into<Column>: Into<datafusion_common::Column>>(dt: arrow::datatypes::DataType, ident: impl Into<datafusion_common::Column>) -> crate::Expr { /* ... */ }
```

#### Function `ident`

Create an unqualified column expression from the provided name, without normalizing
the column.

For example:

```rust
# use datafusion_expr::{col, ident};
let c1 = ident("A"); // not normalized staying as column 'A'
let c2 = col("A"); // normalized via SQL rules becoming column 'a'
assert_ne!(c1, c2);

let c3 = col(r#""A""#);
assert_eq!(c1, c3);

let c4 = col("t1.a"); // parses as relation 't1' column 'a'
let c5 = ident("t1.a"); // parses as column 't1.a'
assert_ne!(c4, c5);
```

```rust
pub fn ident</* synthetic */ impl Into<String>: Into<String>>(name: impl Into<String>) -> crate::Expr { /* ... */ }
```

#### Function `placeholder`

Create placeholder value that will be filled in (such as `$1`)

Note the parameter type can be inferred using [`Expr::infer_placeholder_types`]

# Example

```rust
# use datafusion_expr::{placeholder};
let p = placeholder("$0"); // $0, refers to parameter 1
assert_eq!(p.to_string(), "$0")
```

```rust
pub fn placeholder</* synthetic */ impl Into<String>: Into<String>>(id: impl Into<String>) -> crate::Expr { /* ... */ }
```

#### Function `wildcard`

Create an '*' [`Expr::Wildcard`] expression that matches all columns

# Example

```rust
# use datafusion_expr::{wildcard};
let p = wildcard();
assert_eq!(p.to_string(), "*")
```

```rust
pub fn wildcard() -> crate::select_expr::SelectExpr { /* ... */ }
```

#### Function `wildcard_with_options`

Create an '*' [`Expr::Wildcard`] expression with the wildcard options

```rust
pub fn wildcard_with_options(options: crate::expr::WildcardOptions) -> crate::select_expr::SelectExpr { /* ... */ }
```

#### Function `qualified_wildcard`

Create an 't.*' [`Expr::Wildcard`] expression that matches all columns from a specific table

# Example

```rust
# use datafusion_common::TableReference;
# use datafusion_expr::{qualified_wildcard};
let p = qualified_wildcard(TableReference::bare("t"));
assert_eq!(p.to_string(), "t.*")
```

```rust
pub fn qualified_wildcard</* synthetic */ impl Into<TableReference>: Into<datafusion_common::TableReference>>(qualifier: impl Into<datafusion_common::TableReference>) -> crate::select_expr::SelectExpr { /* ... */ }
```

#### Function `qualified_wildcard_with_options`

Create an 't.*' [`Expr::Wildcard`] expression with the wildcard options

```rust
pub fn qualified_wildcard_with_options</* synthetic */ impl Into<TableReference>: Into<datafusion_common::TableReference>>(qualifier: impl Into<datafusion_common::TableReference>, options: crate::expr::WildcardOptions) -> crate::select_expr::SelectExpr { /* ... */ }
```

#### Function `binary_expr`

Return a new expression `left <op> right`

```rust
pub fn binary_expr(left: crate::Expr, op: crate::Operator, right: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `and`

Return a new expression with a logical AND

```rust
pub fn and(left: crate::Expr, right: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `or`

Return a new expression with a logical OR

```rust
pub fn or(left: crate::Expr, right: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `not`

Return a new expression with a logical NOT

```rust
pub fn not(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `bitwise_and`

Return a new expression with bitwise AND

```rust
pub fn bitwise_and(left: crate::Expr, right: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `bitwise_or`

Return a new expression with bitwise OR

```rust
pub fn bitwise_or(left: crate::Expr, right: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `bitwise_xor`

Return a new expression with bitwise XOR

```rust
pub fn bitwise_xor(left: crate::Expr, right: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `bitwise_shift_right`

Return a new expression with bitwise SHIFT RIGHT

```rust
pub fn bitwise_shift_right(left: crate::Expr, right: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `bitwise_shift_left`

Return a new expression with bitwise SHIFT LEFT

```rust
pub fn bitwise_shift_left(left: crate::Expr, right: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `in_list`

Create an in_list expression

```rust
pub fn in_list(expr: crate::Expr, list: Vec<crate::Expr>, negated: bool) -> crate::Expr { /* ... */ }
```

#### Function `exists`

Create an EXISTS subquery expression

```rust
pub fn exists(subquery: std::sync::Arc<crate::LogicalPlan>) -> crate::Expr { /* ... */ }
```

#### Function `not_exists`

Create a NOT EXISTS subquery expression

```rust
pub fn not_exists(subquery: std::sync::Arc<crate::LogicalPlan>) -> crate::Expr { /* ... */ }
```

#### Function `in_subquery`

Create an IN subquery expression

```rust
pub fn in_subquery(expr: crate::Expr, subquery: std::sync::Arc<crate::LogicalPlan>) -> crate::Expr { /* ... */ }
```

#### Function `not_in_subquery`

Create a NOT IN subquery expression

```rust
pub fn not_in_subquery(expr: crate::Expr, subquery: std::sync::Arc<crate::LogicalPlan>) -> crate::Expr { /* ... */ }
```

#### Function `scalar_subquery`

Create a scalar subquery expression

```rust
pub fn scalar_subquery(subquery: std::sync::Arc<crate::LogicalPlan>) -> crate::Expr { /* ... */ }
```

#### Function `grouping_set`

Create a grouping set

```rust
pub fn grouping_set(exprs: Vec<Vec<crate::Expr>>) -> crate::Expr { /* ... */ }
```

#### Function `cube`

Create a grouping set for all combination of `exprs`

```rust
pub fn cube(exprs: Vec<crate::Expr>) -> crate::Expr { /* ... */ }
```

#### Function `rollup`

Create a grouping set for rollup

```rust
pub fn rollup(exprs: Vec<crate::Expr>) -> crate::Expr { /* ... */ }
```

#### Function `cast`

Create a cast expression

```rust
pub fn cast(expr: crate::Expr, data_type: arrow::datatypes::DataType) -> crate::Expr { /* ... */ }
```

#### Function `try_cast`

Create a try cast expression

```rust
pub fn try_cast(expr: crate::Expr, data_type: arrow::datatypes::DataType) -> crate::Expr { /* ... */ }
```

#### Function `is_null`

Create is null expression

```rust
pub fn is_null(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `is_true`

Create is true expression

```rust
pub fn is_true(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `is_not_true`

Create is not true expression

```rust
pub fn is_not_true(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `is_false`

Create is false expression

```rust
pub fn is_false(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `is_not_false`

Create is not false expression

```rust
pub fn is_not_false(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `is_unknown`

Create is unknown expression

```rust
pub fn is_unknown(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `is_not_unknown`

Create is not unknown expression

```rust
pub fn is_not_unknown(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `case`

Create a CASE WHEN statement with literal WHEN expressions for comparison to the base expression.

```rust
pub fn case(expr: crate::Expr) -> crate::conditional_expressions::CaseBuilder { /* ... */ }
```

#### Function `when`

Create a CASE WHEN statement with boolean WHEN expressions and no base expression.

```rust
pub fn when(when: crate::Expr, then: crate::Expr) -> crate::conditional_expressions::CaseBuilder { /* ... */ }
```

#### Function `unnest`

Create a Unnest expression

```rust
pub fn unnest(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `create_udf`

Convenience method to create a new user defined scalar function (UDF) with a
specific signature and specific return type.

Note this function does not expose all available features of [`ScalarUDF`],
such as

* computing return types based on input types
* multiple [`Signature`]s
* aliases

See [`ScalarUDF`] for details and examples on how to use the full
functionality.

```rust
pub fn create_udf(name: &str, input_types: Vec<arrow::datatypes::DataType>, return_type: arrow::datatypes::DataType, volatility: crate::Volatility, fun: crate::ScalarFunctionImplementation) -> crate::ScalarUDF { /* ... */ }
```

#### Function `create_udaf`

Creates a new UDAF with a specific signature, state type and return type.
The signature and state type must match the `Accumulator's implementation`.

```rust
pub fn create_udaf(name: &str, input_type: Vec<arrow::datatypes::DataType>, return_type: std::sync::Arc<arrow::datatypes::DataType>, volatility: crate::Volatility, accumulator: crate::function::AccumulatorFactoryFunction, state_type: std::sync::Arc<Vec<arrow::datatypes::DataType>>) -> crate::AggregateUDF { /* ... */ }
```

#### Function `create_udwf`

Creates a new UDWF with a specific signature, state type and return type.

The signature and state type must match the [`PartitionEvaluator`]'s implementation`.

[`PartitionEvaluator`]: crate::PartitionEvaluator

```rust
pub fn create_udwf(name: &str, input_type: arrow::datatypes::DataType, return_type: std::sync::Arc<arrow::datatypes::DataType>, volatility: crate::Volatility, partition_evaluator_factory: crate::function::PartitionEvaluatorFactory) -> crate::WindowUDF { /* ... */ }
```

#### Function `interval_year_month_lit`

```rust
pub fn interval_year_month_lit(value: &str) -> crate::Expr { /* ... */ }
```

#### Function `interval_datetime_lit`

```rust
pub fn interval_datetime_lit(value: &str) -> crate::Expr { /* ... */ }
```

#### Function `interval_month_day_nano_lit`

```rust
pub fn interval_month_day_nano_lit(value: &str) -> crate::Expr { /* ... */ }
```

## Module `expr_rewriter`

Expression rewriter

```rust
pub mod expr_rewriter { /* ... */ }
```

### Types

#### Struct `NamePreserver`

Handles ensuring the name of rewritten expressions is not changed.

This is important when optimizing plans to ensure the output
schema of plan nodes don't change after optimization.
For example, if an expression `1 + 2` is rewritten to `3`, the name of the
expression should be preserved: `3 as "1 + 2"`

See <https://github.com/apache/datafusion/issues/3555> for details

```rust
pub struct NamePreserver {
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
  pub fn new(plan: &LogicalPlan) -> Self { /* ... */ }
  ```
  Create a new NamePreserver for rewriting the `expr` that is part of the specified plan

- ```rust
  pub fn new_for_projection() -> Self { /* ... */ }
  ```
  Create a new NamePreserver for rewriting the `expr`s in `Projection`

- ```rust
  pub fn save(self: &Self, expr: &Expr) -> SavedName { /* ... */ }
  ```

###### Trait Implementations

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
- **MaybeSendSync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `SavedName`

If the qualified name of an expression is remembered, it will be preserved
when rewriting the expression

```rust
pub enum SavedName {
    Saved {
        relation: Option<datafusion_common::TableReference>,
        name: String,
    },
    None,
}
```

##### Variants

###### `Saved`

Saved qualified name to be preserved

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `relation` | `Option<datafusion_common::TableReference>` |  |
| `name` | `String` |  |

###### `None`

Name is not preserved

##### Implementations

###### Methods

- ```rust
  pub fn restore(self: Self, expr: Expr) -> Expr { /* ... */ }
  ```
  Ensures the qualified name of the rewritten expression is preserved

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Sync**
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **MaybeSendSync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `FunctionRewrite`

Trait for rewriting [`Expr`]s into function calls.

This trait is used with `FunctionRegistry::register_function_rewrite` to
to evaluating `Expr`s using functions that may not be built in to DataFusion

For example, concatenating arrays `a || b` is represented as
`Operator::ArrowAt`, but can be implemented by calling a function
`array_concat` from the `functions-nested` crate.

```rust
pub trait FunctionRewrite: Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `name`: Return a human readable name for this rewrite
- `rewrite`: Potentially rewrite `expr` to some other expression

### Functions

#### Function `normalize_col`

Recursively call `LogicalPlanBuilder::normalize` on all [`Column`] expressions
in the `expr` expression tree.

```rust
pub fn normalize_col(expr: crate::Expr, plan: &crate::LogicalPlan) -> datafusion_common::Result<crate::Expr> { /* ... */ }
```

#### Function `normalize_col_with_schemas_and_ambiguity_check`

See [`Column::normalize_with_schemas_and_ambiguity_check`] for usage

```rust
pub fn normalize_col_with_schemas_and_ambiguity_check(expr: crate::Expr, schemas: &[&[&datafusion_common::DFSchema]], using_columns: &[std::collections::HashSet<datafusion_common::Column>]) -> datafusion_common::Result<crate::Expr> { /* ... */ }
```

#### Function `normalize_cols`

Recursively normalize all [`Column`] expressions in a list of expression trees

```rust
pub fn normalize_cols</* synthetic */ impl Into<Expr>: Into<crate::Expr>, /* synthetic */ impl IntoIterator<Item = impl Into<Expr>>: IntoIterator<Item = impl Into<crate::Expr>>>(exprs: impl IntoIterator<Item = impl Into<crate::Expr>>, plan: &crate::LogicalPlan) -> datafusion_common::Result<Vec<crate::Expr>> { /* ... */ }
```

#### Function `normalize_sorts`

```rust
pub fn normalize_sorts</* synthetic */ impl Into<Sort>: Into<crate::expr::Sort>, /* synthetic */ impl IntoIterator<Item = impl Into<Sort>>: IntoIterator<Item = impl Into<crate::expr::Sort>>>(sorts: impl IntoIterator<Item = impl Into<crate::expr::Sort>>, plan: &crate::LogicalPlan) -> datafusion_common::Result<Vec<crate::expr::Sort>> { /* ... */ }
```

#### Function `replace_col`

Recursively replace all [`Column`] expressions in a given expression tree with
`Column` expressions provided by the hash map argument.

```rust
pub fn replace_col(expr: crate::Expr, replace_map: &std::collections::HashMap<&datafusion_common::Column, &datafusion_common::Column>) -> datafusion_common::Result<crate::Expr> { /* ... */ }
```

#### Function `unnormalize_col`

Recursively 'unnormalize' (remove all qualifiers) from an
expression tree.

For example, if there were expressions like `foo.bar` this would
rewrite it to just `bar`.

```rust
pub fn unnormalize_col(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `create_col_from_scalar_expr`

Create a Column from the Scalar Expr

```rust
pub fn create_col_from_scalar_expr(scalar_expr: &crate::Expr, subqry_alias: String) -> datafusion_common::Result<datafusion_common::Column> { /* ... */ }
```

#### Function `unnormalize_cols`

**Attributes:**

- `#[inline]`

Recursively un-normalize all [`Column`] expressions in a list of expression trees

```rust
pub fn unnormalize_cols</* synthetic */ impl IntoIterator<Item = Expr>: IntoIterator<Item = crate::Expr>>(exprs: impl IntoIterator<Item = crate::Expr>) -> Vec<crate::Expr> { /* ... */ }
```

#### Function `strip_outer_reference`

Recursively remove all the ['OuterReferenceColumn'] and return the inside Column
in the expression tree.

```rust
pub fn strip_outer_reference(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `coerce_plan_expr_for_schema`

Returns plan with expressions coerced to types compatible with
schema types

```rust
pub fn coerce_plan_expr_for_schema(plan: crate::LogicalPlan, schema: &datafusion_common::DFSchema) -> datafusion_common::Result<crate::LogicalPlan> { /* ... */ }
```

#### Function `unalias`

**Attributes:**

- `#[inline]`

Recursively un-alias an expressions

```rust
pub fn unalias(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

### Re-exports

#### Re-export `rewrite_sort_cols_by_aggs`

```rust
pub use order_by::rewrite_sort_cols_by_aggs;
```

## Module `expr_schema`

```rust
pub mod expr_schema { /* ... */ }
```

### Traits

#### Trait `ExprSchemable`

Trait to allow expr to typable with respect to a schema

```rust
pub trait ExprSchemable {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_type`: Given a schema, return the type of the expr
- `nullable`: Given a schema, return the nullability of the expr
- `metadata`: Given a schema, return the expr's optional metadata
- `to_field`: Convert to a field with respect to a schema
- `cast_to`: Cast to a type with respect to a schema
- `data_type_and_nullable`: Given a schema, return the type and nullability of the expr

##### Implementations

This trait is implemented for the following types:

- `super::Expr`

### Functions

#### Function `cast_subquery`

Cast subquery in InSubquery/ScalarSubquery to a given type.

1. **Projection plan**: If the subquery is a projection (i.e. a SELECT statement with specific
   columns), it casts the first expression in the projection to the target type and creates a
   new projection with the casted expression.
2. **Non-projection plan**: If the subquery isn't a projection, it adds a projection to the plan
   with the casted first column.


```rust
pub fn cast_subquery(subquery: crate::Subquery, cast_to_type: &arrow::datatypes::DataType) -> datafusion_common::Result<crate::Subquery> { /* ... */ }
```

## Module `function`

Function module contains typing and signature for built-in and user defined functions.

```rust
pub mod function { /* ... */ }
```

### Types

#### Enum `Hint`

```rust
pub enum Hint {
    Pad,
    AcceptsSingular,
}
```

##### Variants

###### `Pad`

Indicates the argument needs to be padded if it is scalar

###### `AcceptsSingular`

Indicates the argument can be converted to an array of length 1

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Freeze**
- **RefUnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Hint { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Copy**
#### Type Alias `ScalarFunctionImplementation`

Scalar function

The Fn param is the wrapped function but be aware that the function will
be passed with the slice / vec of columnar values (either scalar or array)
with the exception of zero param function, where a singular element vec
will be passed. In that case the single element is a null array to indicate
the batch's row count (so that the generative zero-argument function can know
the result array size).

```rust
pub type ScalarFunctionImplementation = std::sync::Arc<dyn Fn(&[crate::ColumnarValue]) -> datafusion_common::Result<crate::ColumnarValue> + Send + Sync>;
```

#### Type Alias `ReturnTypeFunction`

Factory that returns the functions's return type given the input argument types

```rust
pub type ReturnTypeFunction = std::sync::Arc<dyn Fn(&[arrow::datatypes::DataType]) -> datafusion_common::Result<std::sync::Arc<arrow::datatypes::DataType>> + Send + Sync>;
```

#### Type Alias `PartitionEvaluatorFactory`

Factory that creates a PartitionEvaluator for the given window
function

```rust
pub type PartitionEvaluatorFactory = std::sync::Arc<dyn Fn() -> datafusion_common::Result<Box<dyn PartitionEvaluator>> + Send + Sync>;
```

#### Type Alias `StateTypeFunction`

Factory that returns the types used by an aggregator to serialize
its state, given its return datatype.

```rust
pub type StateTypeFunction = std::sync::Arc<dyn Fn(&arrow::datatypes::DataType) -> datafusion_common::Result<std::sync::Arc<Vec<arrow::datatypes::DataType>>> + Send + Sync>;
```

#### Type Alias `AggregateFunctionSimplification`

[crate::udaf::AggregateUDFImpl::simplify] simplifier closure
A closure with two arguments:
* 'aggregate_function': [crate::expr::AggregateFunction] for which simplified has been invoked
* 'info': [crate::simplify::SimplifyInfo]

Closure returns simplified [Expr] or an error.

```rust
pub type AggregateFunctionSimplification = Box<dyn Fn(crate::expr::AggregateFunction, &dyn crate::simplify::SimplifyInfo) -> datafusion_common::Result<crate::Expr>>;
```

#### Type Alias `WindowFunctionSimplification`

[crate::udwf::WindowUDFImpl::simplify] simplifier closure
A closure with two arguments:
* 'window_function': [crate::expr::WindowFunction] for which simplified has been invoked
* 'info': [crate::simplify::SimplifyInfo]

Closure returns simplified [Expr] or an error.

```rust
pub type WindowFunctionSimplification = Box<dyn Fn(crate::expr::WindowFunction, &dyn crate::simplify::SimplifyInfo) -> datafusion_common::Result<crate::Expr>>;
```

### Re-exports

#### Re-export `AccumulatorArgs`

```rust
pub use datafusion_functions_aggregate_common::accumulator::AccumulatorArgs;
```

#### Re-export `AccumulatorFactoryFunction`

```rust
pub use datafusion_functions_aggregate_common::accumulator::AccumulatorFactoryFunction;
```

#### Re-export `StateFieldsArgs`

```rust
pub use datafusion_functions_aggregate_common::accumulator::StateFieldsArgs;
```

#### Re-export `ExpressionArgs`

```rust
pub use datafusion_functions_window_common::expr::ExpressionArgs;
```

#### Re-export `WindowUDFFieldArgs`

```rust
pub use datafusion_functions_window_common::field::WindowUDFFieldArgs;
```

#### Re-export `PartitionEvaluatorArgs`

```rust
pub use datafusion_functions_window_common::partition::PartitionEvaluatorArgs;
```

## Module `select_expr`

```rust
pub mod select_expr { /* ... */ }
```

### Types

#### Enum `SelectExpr`

Represents a SELECT expression in a SQL query.

`SelectExpr` supports three types of expressions commonly found in the SELECT clause:

* Wildcard (`*`) - Selects all columns
* Qualified wildcard (`table.*`) - Selects all columns from a specific table
* Regular expression - Any other expression like columns, functions, literals etc.

This enum is typically used when you need to handle wildcards. After expanding `*` in the query,
you can use `Expr` for all other expressions.

# Examples

```
use datafusion_expr::col;
use datafusion_expr::expr::WildcardOptions;
use datafusion_expr::select_expr::SelectExpr;

// SELECT *
let wildcard = SelectExpr::Wildcard(WildcardOptions::default());

// SELECT mytable.*
let qualified = SelectExpr::QualifiedWildcard(
    "mytable".into(),
    WildcardOptions::default()
);

// SELECT col1
let expr = SelectExpr::Expression(col("col1").into());
```

```rust
pub enum SelectExpr {
    Wildcard(crate::expr::WildcardOptions),
    QualifiedWildcard(datafusion_common::TableReference, crate::expr::WildcardOptions),
    Expression(crate::Expr),
}
```

##### Variants

###### `Wildcard`

Represents a wildcard (`*`) that selects all columns from all tables.
The `WildcardOptions` control additional behavior like exclusions.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::expr::WildcardOptions` |  |

###### `QualifiedWildcard`

Represents a qualified wildcard (`table.*`) that selects all columns from a specific table.
The `TableReference` specifies the table and `WildcardOptions` control additional behavior.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_common::TableReference` |  |
| 1 | `crate::expr::WildcardOptions` |  |

###### `Expression`

Represents any other valid SELECT expression like column references,
function calls, literals, etc.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::Expr` |  |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Sync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(expr: Expr) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: Column) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(value: (Option<&''a TableReference>, &''a FieldRef)) -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SelectExpr { /* ... */ }
    ```

## Module `groups_accumulator`

```rust
pub mod groups_accumulator { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_expr_common::groups_accumulator::*`

```rust
pub use datafusion_expr_common::groups_accumulator::*;
```

## Module `interval_arithmetic`

```rust
pub mod interval_arithmetic { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_expr_common::interval_arithmetic::*`

```rust
pub use datafusion_expr_common::interval_arithmetic::*;
```

## Module `logical_plan`

```rust
pub mod logical_plan { /* ... */ }
```

### Modules

## Module `builder`

This module provides a builder for creating LogicalPlans

```rust
pub mod builder { /* ... */ }
```

### Types

#### Struct `LogicalPlanBuilderOptions`

Options for [`LogicalPlanBuilder`]

```rust
pub struct LogicalPlanBuilderOptions {
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
  pub fn with_add_implicit_group_by_exprs(self: Self, add: bool) -> Self { /* ... */ }
  ```
  Should the builder add functionally dependent expressions as additional aggregation groupings.

###### Trait Implementations

- **Unpin**
- **Send**
- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> LogicalPlanBuilderOptions { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LogicalPlanBuilderOptions { /* ... */ }
    ```

#### Struct `LogicalPlanBuilder`

Builder for logical plans

# Example building a simple plan
```
# use datafusion_expr::{lit, col, LogicalPlanBuilder, logical_plan::table_scan};
# use datafusion_common::Result;
# use arrow::datatypes::{Schema, DataType, Field};
#
# fn main() -> Result<()> {
#
# fn employee_schema() -> Schema {
#    Schema::new(vec![
#           Field::new("id", DataType::Int32, false),
#           Field::new("first_name", DataType::Utf8, false),
#           Field::new("last_name", DataType::Utf8, false),
#           Field::new("state", DataType::Utf8, false),
#           Field::new("salary", DataType::Int32, false),
#       ])
#   }
#
// Create a plan similar to
// SELECT last_name
// FROM employees
// WHERE salary < 1000
let plan = table_scan(Some("employee"), &employee_schema(), None)?
 // Keep only rows where salary < 1000
 .filter(col("salary").lt(lit(1000)))?
 // only show "last_name" in the final results
 .project(vec![col("last_name")])?
 .build()?;

// Convert from plan back to builder
let builder = LogicalPlanBuilder::from(plan);

# Ok(())
# }
```

```rust
pub struct LogicalPlanBuilder {
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
  pub fn new(plan: LogicalPlan) -> Self { /* ... */ }
  ```
  Create a builder from an existing plan

- ```rust
  pub fn new_from_arc(plan: Arc<LogicalPlan>) -> Self { /* ... */ }
  ```
  Create a builder from an existing plan

- ```rust
  pub fn with_options(self: Self, options: LogicalPlanBuilderOptions) -> Self { /* ... */ }
  ```

- ```rust
  pub fn schema(self: &Self) -> &DFSchemaRef { /* ... */ }
  ```
  Return the output schema of the plan build so far

- ```rust
  pub fn plan(self: &Self) -> &LogicalPlan { /* ... */ }
  ```
  Return the LogicalPlan of the plan build so far

- ```rust
  pub fn empty(produce_one_row: bool) -> Self { /* ... */ }
  ```
  Create an empty relation.

- ```rust
  pub fn to_recursive_query(self: Self, name: String, recursive_term: LogicalPlan, is_distinct: bool) -> Result<Self> { /* ... */ }
  ```
  Convert a regular plan into a recursive query.

- ```rust
  pub fn values(values: Vec<Vec<Expr>>) -> Result<Self> { /* ... */ }
  ```
  Create a values list based relation, and the schema is inferred from data, consuming

- ```rust
  pub fn values_with_schema(values: Vec<Vec<Expr>>, schema: &DFSchemaRef) -> Result<Self> { /* ... */ }
  ```
  Create a values list based relation, and the schema is inferred from data itself or table schema if provided, consuming

- ```rust
  pub fn scan</* synthetic */ impl Into<TableReference>: Into<TableReference>>(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>) -> Result<Self> { /* ... */ }
  ```
  Convert a table provider into a builder with a TableScan

- ```rust
  pub fn copy_to(input: LogicalPlan, output_url: String, file_type: Arc<dyn FileType>, options: HashMap<String, String>, partition_by: Vec<String>) -> Result<Self> { /* ... */ }
  ```
  Create a [CopyTo] for copying the contents of this builder to the specified file(s)

- ```rust
  pub fn insert_into</* synthetic */ impl Into<TableReference>: Into<TableReference>>(input: LogicalPlan, table_name: impl Into<TableReference>, target: Arc<dyn TableSource>, insert_op: InsertOp) -> Result<Self> { /* ... */ }
  ```
  Create a [`DmlStatement`] for inserting the contents of this builder into the named table.

- ```rust
  pub fn scan_with_filters</* synthetic */ impl Into<TableReference>: Into<TableReference>>(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>, filters: Vec<Expr>) -> Result<Self> { /* ... */ }
  ```
  Convert a table provider into a builder with a TableScan

- ```rust
  pub fn scan_with_filters_fetch</* synthetic */ impl Into<TableReference>: Into<TableReference>>(table_name: impl Into<TableReference>, table_source: Arc<dyn TableSource>, projection: Option<Vec<usize>>, filters: Vec<Expr>, fetch: Option<usize>) -> Result<Self> { /* ... */ }
  ```
  Convert a table provider into a builder with a TableScan with filter and fetch

- ```rust
  pub fn window_plan</* synthetic */ impl IntoIterator<Item = Expr>: IntoIterator<Item = Expr>>(input: LogicalPlan, window_exprs: impl IntoIterator<Item = Expr>) -> Result<LogicalPlan> { /* ... */ }
  ```
  Wrap a plan in a window

- ```rust
  pub fn project</* synthetic */ impl Into<SelectExpr>: Into<SelectExpr>, /* synthetic */ impl IntoIterator<Item = impl Into<SelectExpr>>: IntoIterator<Item = impl Into<SelectExpr>>>(self: Self, expr: impl IntoIterator<Item = impl Into<SelectExpr>>) -> Result<Self> { /* ... */ }
  ```
  Apply a projection without alias.

- ```rust
  pub fn project_with_validation</* synthetic */ impl Into<SelectExpr>: Into<SelectExpr>>(self: Self, expr: Vec<(impl Into<SelectExpr>, bool)>) -> Result<Self> { /* ... */ }
  ```
  Apply a projection without alias with optional validation

- ```rust
  pub fn select</* synthetic */ impl IntoIterator<Item = usize>: IntoIterator<Item = usize>>(self: Self, indices: impl IntoIterator<Item = usize>) -> Result<Self> { /* ... */ }
  ```
  Select the given column indices

- ```rust
  pub fn filter</* synthetic */ impl Into<Expr>: Into<Expr>>(self: Self, expr: impl Into<Expr>) -> Result<Self> { /* ... */ }
  ```
  Apply a filter

- ```rust
  pub fn having</* synthetic */ impl Into<Expr>: Into<Expr>>(self: Self, expr: impl Into<Expr>) -> Result<Self> { /* ... */ }
  ```
  Apply a filter which is used for a having clause

- ```rust
  pub fn prepare(self: Self, name: String, data_types: Vec<DataType>) -> Result<Self> { /* ... */ }
  ```
  Make a builder for a prepare logical plan from the builder's plan

- ```rust
  pub fn limit(self: Self, skip: usize, fetch: Option<usize>) -> Result<Self> { /* ... */ }
  ```
  Limit the number of rows returned

- ```rust
  pub fn limit_by_expr(self: Self, skip: Option<Expr>, fetch: Option<Expr>) -> Result<Self> { /* ... */ }
  ```
  Limit the number of rows returned

- ```rust
  pub fn alias</* synthetic */ impl Into<TableReference>: Into<TableReference>>(self: Self, alias: impl Into<TableReference>) -> Result<Self> { /* ... */ }
  ```
  Apply an alias

- ```rust
  pub fn sort_by</* synthetic */ impl Into<Expr>: Into<Expr>, /* synthetic */ impl IntoIterator<Item = impl Into<Expr>> + Clone: IntoIterator<Item = impl Into<Expr>> + Clone>(self: Self, expr: impl IntoIterator<Item = impl Into<Expr>> + Clone) -> Result<Self> { /* ... */ }
  ```
  Apply a sort by provided expressions with default direction

- ```rust
  pub fn sort</* synthetic */ impl Into<SortExpr>: Into<SortExpr>, /* synthetic */ impl IntoIterator<Item = impl Into<SortExpr>> + Clone: IntoIterator<Item = impl Into<SortExpr>> + Clone>(self: Self, sorts: impl IntoIterator<Item = impl Into<SortExpr>> + Clone) -> Result<Self> { /* ... */ }
  ```

- ```rust
  pub fn sort_with_limit</* synthetic */ impl Into<SortExpr>: Into<SortExpr>, /* synthetic */ impl IntoIterator<Item = impl Into<SortExpr>> + Clone: IntoIterator<Item = impl Into<SortExpr>> + Clone>(self: Self, sorts: impl IntoIterator<Item = impl Into<SortExpr>> + Clone, fetch: Option<usize>) -> Result<Self> { /* ... */ }
  ```
  Apply a sort

- ```rust
  pub fn union(self: Self, plan: LogicalPlan) -> Result<Self> { /* ... */ }
  ```
  Apply a union, preserving duplicate rows

- ```rust
  pub fn union_by_name(self: Self, plan: LogicalPlan) -> Result<Self> { /* ... */ }
  ```
  Apply a union by name, preserving duplicate rows

- ```rust
  pub fn union_by_name_distinct(self: Self, plan: LogicalPlan) -> Result<Self> { /* ... */ }
  ```
  Apply a union by name, removing duplicate rows

- ```rust
  pub fn union_distinct(self: Self, plan: LogicalPlan) -> Result<Self> { /* ... */ }
  ```
  Apply a union, removing duplicate rows

- ```rust
  pub fn distinct(self: Self) -> Result<Self> { /* ... */ }
  ```
  Apply deduplication: Only distinct (different) values are returned)

- ```rust
  pub fn distinct_on(self: Self, on_expr: Vec<Expr>, select_expr: Vec<Expr>, sort_expr: Option<Vec<SortExpr>>) -> Result<Self> { /* ... */ }
  ```
  Project first values of the specified expression list according to the provided

- ```rust
  pub fn join</* synthetic */ impl Into<Column>: Into<Column>, /* synthetic */ impl Into<Column>: Into<Column>>(self: Self, right: LogicalPlan, join_type: JoinType, join_keys: (Vec<impl Into<Column>>, Vec<impl Into<Column>>), filter: Option<Expr>) -> Result<Self> { /* ... */ }
  ```
  Apply a join to `right` using explicitly specified columns and an

- ```rust
  pub fn join_on</* synthetic */ impl IntoIterator<Item = Expr>: IntoIterator<Item = Expr>>(self: Self, right: LogicalPlan, join_type: JoinType, on_exprs: impl IntoIterator<Item = Expr>) -> Result<Self> { /* ... */ }
  ```
  Apply a join using the specified expressions.

- ```rust
  pub fn join_detailed</* synthetic */ impl Into<Column>: Into<Column>, /* synthetic */ impl Into<Column>: Into<Column>>(self: Self, right: LogicalPlan, join_type: JoinType, join_keys: (Vec<impl Into<Column>>, Vec<impl Into<Column>>), filter: Option<Expr>, null_equals_null: bool) -> Result<Self> { /* ... */ }
  ```
  Apply a join with on constraint and specified null equality.

- ```rust
  pub fn join_using</* synthetic */ impl Into<Column> + Clone: Into<Column> + Clone>(self: Self, right: LogicalPlan, join_type: JoinType, using_keys: Vec<impl Into<Column> + Clone>) -> Result<Self> { /* ... */ }
  ```
  Apply a join with using constraint, which duplicates all join columns in output schema.

- ```rust
  pub fn cross_join(self: Self, right: LogicalPlan) -> Result<Self> { /* ... */ }
  ```
  Apply a cross join

- ```rust
  pub fn repartition(self: Self, partitioning_scheme: Partitioning) -> Result<Self> { /* ... */ }
  ```
  Repartition

- ```rust
  pub fn window</* synthetic */ impl Into<Expr>: Into<Expr>, /* synthetic */ impl IntoIterator<Item = impl Into<Expr>>: IntoIterator<Item = impl Into<Expr>>>(self: Self, window_expr: impl IntoIterator<Item = impl Into<Expr>>) -> Result<Self> { /* ... */ }
  ```
  Apply a window functions to extend the schema

- ```rust
  pub fn aggregate</* synthetic */ impl Into<Expr>: Into<Expr>, /* synthetic */ impl IntoIterator<Item = impl Into<Expr>>: IntoIterator<Item = impl Into<Expr>>, /* synthetic */ impl Into<Expr>: Into<Expr>, /* synthetic */ impl IntoIterator<Item = impl Into<Expr>>: IntoIterator<Item = impl Into<Expr>>>(self: Self, group_expr: impl IntoIterator<Item = impl Into<Expr>>, aggr_expr: impl IntoIterator<Item = impl Into<Expr>>) -> Result<Self> { /* ... */ }
  ```
  Apply an aggregate: grouping on the `group_expr` expressions

- ```rust
  pub fn explain(self: Self, verbose: bool, analyze: bool) -> Result<Self> { /* ... */ }
  ```
  Create an expression to represent the explanation of the plan

- ```rust
  pub fn intersect(left_plan: LogicalPlan, right_plan: LogicalPlan, is_all: bool) -> Result<LogicalPlan> { /* ... */ }
  ```
  Process intersect set operator

- ```rust
  pub fn except(left_plan: LogicalPlan, right_plan: LogicalPlan, is_all: bool) -> Result<LogicalPlan> { /* ... */ }
  ```
  Process except set operator

- ```rust
  pub fn build(self: Self) -> Result<LogicalPlan> { /* ... */ }
  ```
  Build the plan

- ```rust
  pub fn join_with_expr_keys</* synthetic */ impl Into<Expr>: Into<Expr>, /* synthetic */ impl Into<Expr>: Into<Expr>>(self: Self, right: LogicalPlan, join_type: JoinType, equi_exprs: (Vec<impl Into<Expr>>, Vec<impl Into<Expr>>), filter: Option<Expr>) -> Result<Self> { /* ... */ }
  ```
  Apply a join with both explicit equijoin and non equijoin predicates.

- ```rust
  pub fn unnest_column</* synthetic */ impl Into<Column>: Into<Column>>(self: Self, column: impl Into<Column>) -> Result<Self> { /* ... */ }
  ```
  Unnest the given column.

- ```rust
  pub fn unnest_column_with_options</* synthetic */ impl Into<Column>: Into<Column>>(self: Self, column: impl Into<Column>, options: UnnestOptions) -> Result<Self> { /* ... */ }
  ```
  Unnest the given column given [`UnnestOptions`]

- ```rust
  pub fn unnest_columns_with_options(self: Self, columns: Vec<Column>, options: UnnestOptions) -> Result<Self> { /* ... */ }
  ```
  Unnest the given columns with the given [`UnnestOptions`]

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> LogicalPlanBuilder { /* ... */ }
    ```

- **IntoEither**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Unpin**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(plan: LogicalPlan) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(plan: Arc<LogicalPlan>) -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
#### Struct `LogicalTableSource`

Basic TableSource implementation intended for use in tests and documentation. It is expected
that users will provide their own TableSource implementations or use DataFusion's
DefaultTableSource.

```rust
pub struct LogicalTableSource {
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
  pub fn new(table_schema: SchemaRef) -> Self { /* ... */ }
  ```
  Create a new LogicalTableSource

- ```rust
  pub fn with_constraints(self: Self, constraints: Constraints) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **TableSource**
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
    fn supports_filters_pushdown(self: &Self, filters: &[&Expr]) -> Result<Vec<TableProviderFilterPushDown>> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Allocation**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **ErasedDestructor**
### Functions

#### Function `change_redundant_column`

```rust
pub fn change_redundant_column(fields: &arrow::datatypes::Fields) -> Vec<arrow::datatypes::Field> { /* ... */ }
```

#### Function `build_join_schema`

Creates a schema for a join operation.
The fields from the left side are first

```rust
pub fn build_join_schema(left: &datafusion_common::DFSchema, right: &datafusion_common::DFSchema, join_type: &crate::logical_plan::JoinType) -> datafusion_common::Result<datafusion_common::DFSchema> { /* ... */ }
```

#### Function `add_group_by_exprs_from_dependencies`

Add additional "synthetic" group by expressions based on functional
dependencies.

For example, if we are grouping on `[c1]`, and we know from
functional dependencies that column `c1` determines `c2`, this function
adds `c2` to the group by list.

This allows MySQL style selects like
`SELECT col FROM t WHERE pk = 5` if col is unique

```rust
pub fn add_group_by_exprs_from_dependencies(group_expr: Vec<crate::Expr>, schema: &datafusion_common::DFSchemaRef) -> datafusion_common::Result<Vec<crate::Expr>> { /* ... */ }
```

#### Function `validate_unique_names`

Errors if one or more expressions have equal names.

```rust
pub fn validate_unique_names<''a, /* synthetic */ impl IntoIterator<Item = &'a Expr>: IntoIterator<Item = &''a crate::Expr>>(node_name: &str, expressions: impl IntoIterator<Item = &''a crate::Expr>) -> datafusion_common::Result<()> { /* ... */ }
```

#### Function `union`

Union two [`LogicalPlan`]s.

Constructs the UNION plan, but does not perform type-coercion. Therefore the
subtree expressions will not be properly typed until the optimizer pass.

If a properly typed UNION plan is needed, refer to [`TypeCoercionRewriter::coerce_union`]
or alternatively, merge the union input schema using [`coerce_union_schema`] and
apply the expression rewrite with [`coerce_plan_expr_for_schema`].

[`TypeCoercionRewriter::coerce_union`]: https://docs.rs/datafusion-optimizer/latest/datafusion_optimizer/analyzer/type_coercion/struct.TypeCoercionRewriter.html#method.coerce_union
[`coerce_union_schema`]: https://docs.rs/datafusion-optimizer/latest/datafusion_optimizer/analyzer/type_coercion/fn.coerce_union_schema.html

```rust
pub fn union(left_plan: crate::logical_plan::LogicalPlan, right_plan: crate::logical_plan::LogicalPlan) -> datafusion_common::Result<crate::logical_plan::LogicalPlan> { /* ... */ }
```

#### Function `union_by_name`

Like [`union`], but combine rows from different tables by name, rather than
by position.

```rust
pub fn union_by_name(left_plan: crate::logical_plan::LogicalPlan, right_plan: crate::logical_plan::LogicalPlan) -> datafusion_common::Result<crate::logical_plan::LogicalPlan> { /* ... */ }
```

#### Function `project`

Create Projection
# Errors
This function errors under any of the following conditions:
* Two or more expressions have the same name
* An invalid expression is used (e.g. a `sort` expression)

```rust
pub fn project</* synthetic */ impl Into<SelectExpr>: Into<crate::select_expr::SelectExpr>, /* synthetic */ impl IntoIterator<Item = impl Into<SelectExpr>>: IntoIterator<Item = impl Into<crate::select_expr::SelectExpr>>>(plan: crate::logical_plan::LogicalPlan, expr: impl IntoIterator<Item = impl Into<crate::select_expr::SelectExpr>>) -> datafusion_common::Result<crate::logical_plan::LogicalPlan> { /* ... */ }
```

#### Function `subquery_alias`

Create a SubqueryAlias to wrap a LogicalPlan.

```rust
pub fn subquery_alias</* synthetic */ impl Into<TableReference>: Into<datafusion_common::TableReference>>(plan: crate::logical_plan::LogicalPlan, alias: impl Into<datafusion_common::TableReference>) -> datafusion_common::Result<crate::logical_plan::LogicalPlan> { /* ... */ }
```

#### Function `table_scan`

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema.
This is mostly used for testing and documentation.

```rust
pub fn table_scan</* synthetic */ impl Into<TableReference>: Into<datafusion_common::TableReference>>(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>) -> datafusion_common::Result<LogicalPlanBuilder> { /* ... */ }
```

#### Function `table_scan_with_filters`

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema,
and inlined filters.
This is mostly used for testing and documentation.

```rust
pub fn table_scan_with_filters</* synthetic */ impl Into<TableReference>: Into<datafusion_common::TableReference>>(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, filters: Vec<crate::Expr>) -> datafusion_common::Result<LogicalPlanBuilder> { /* ... */ }
```

#### Function `table_scan_with_filter_and_fetch`

Create a LogicalPlanBuilder representing a scan of a table with the provided name and schema,
filters, and inlined fetch.
This is mostly used for testing and documentation.

```rust
pub fn table_scan_with_filter_and_fetch</* synthetic */ impl Into<TableReference>: Into<datafusion_common::TableReference>>(name: Option<impl Into<datafusion_common::TableReference>>, table_schema: &arrow::datatypes::Schema, projection: Option<Vec<usize>>, filters: Vec<crate::Expr>, fetch: Option<usize>) -> datafusion_common::Result<LogicalPlanBuilder> { /* ... */ }
```

#### Function `table_source`

```rust
pub fn table_source(table_schema: &arrow::datatypes::Schema) -> std::sync::Arc<dyn TableSource> { /* ... */ }
```

#### Function `table_source_with_constraints`

```rust
pub fn table_source_with_constraints(table_schema: &arrow::datatypes::Schema, constraints: datafusion_common::Constraints) -> std::sync::Arc<dyn TableSource> { /* ... */ }
```

#### Function `wrap_projection_for_join_if_necessary`

Wrap projection for a plan, if the join keys contains normal expression.

```rust
pub fn wrap_projection_for_join_if_necessary(join_keys: &[crate::Expr], input: crate::logical_plan::LogicalPlan) -> datafusion_common::Result<(crate::logical_plan::LogicalPlan, Vec<datafusion_common::Column>, bool)> { /* ... */ }
```

#### Function `unnest`

Create a [`LogicalPlan::Unnest`] plan

```rust
pub fn unnest(input: crate::logical_plan::LogicalPlan, columns: Vec<datafusion_common::Column>) -> datafusion_common::Result<crate::logical_plan::LogicalPlan> { /* ... */ }
```

#### Function `get_struct_unnested_columns`

```rust
pub fn get_struct_unnested_columns(col_name: &String, inner_fields: &arrow::datatypes::Fields) -> Vec<datafusion_common::Column> { /* ... */ }
```

#### Function `get_unnested_columns`

```rust
pub fn get_unnested_columns(col_name: &String, data_type: &arrow::datatypes::DataType, depth: usize) -> datafusion_common::Result<Vec<(datafusion_common::Column, std::sync::Arc<arrow::datatypes::Field>)>> { /* ... */ }
```

#### Function `unnest_with_options`

Create a [`LogicalPlan::Unnest`] plan with options
This function receive a list of columns to be unnested
because multiple unnest can be performed on the same column (e.g unnest with different depth)
The new schema will contains post-unnest fields replacing the original field

For example:
Input schema as
```text
+---------------------+-------------------+
| col1                | col2              |
+---------------------+-------------------+
| Struct(INT64,INT32) | List(List(Int64)) |
+---------------------+-------------------+
```



Then unnesting columns with:
- (col1,Struct)
- (col2,List(\[depth=1,depth=2\]))

will generate a new schema as
```text
+---------+---------+---------------------+---------------------+
| col1.c0 | col1.c1 | unnest_col2_depth_1 | unnest_col2_depth_2 |
+---------+---------+---------------------+---------------------+
| Int64   | Int32   | List(Int64)         |  Int64              |
+---------+---------+---------------------+---------------------+
```

```rust
pub fn unnest_with_options(input: crate::logical_plan::LogicalPlan, columns_to_unnest: Vec<datafusion_common::Column>, options: datafusion_common::UnnestOptions) -> datafusion_common::Result<crate::logical_plan::LogicalPlan> { /* ... */ }
```

### Constants and Statics

#### Constant `UNNAMED_TABLE`

Default table name for unnamed table

```rust
pub const UNNAMED_TABLE: &str = "?table?";
```

## Module `display`

This module provides logic for displaying LogicalPlans in various styles

```rust
pub mod display { /* ... */ }
```

### Types

#### Struct `IndentVisitor`

Formats plans with a single line per node. For example:

Projection: id
   Filter: state Eq Utf8(\"CO\")\
      CsvScan: employee.csv projection=Some([0, 3])";

```rust
pub struct IndentVisitor<''a, ''b> {
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
  pub fn new(f: &''a mut fmt::Formatter<''b>, with_schema: bool) -> Self { /* ... */ }
  ```
  Create a visitor that will write a formatted LogicalPlan to f. If `with_schema` is

###### Trait Implementations

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TreeNodeVisitor**
  - ```rust
    fn f_down(self: &mut Self, plan: &''n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion> { /* ... */ }
    ```

  - ```rust
    fn f_up(self: &mut Self, _plan: &''n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Struct `GraphvizVisitor`

Formats plans for graphical display using the `DOT` language. This
format can be visualized using software from
[`graphviz`](https://graphviz.org/)

```rust
pub struct GraphvizVisitor<''a, ''b> {
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
  pub fn new(f: &''a mut fmt::Formatter<''b>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn set_with_schema(self: &mut Self, with_schema: bool) { /* ... */ }
  ```
  Sets a flag which controls if the output schema is displayed

- ```rust
  pub fn pre_visit_plan(self: &mut Self, label: &str) -> fmt::Result { /* ... */ }
  ```

- ```rust
  pub fn post_visit_plan(self: &mut Self) -> fmt::Result { /* ... */ }
  ```

- ```rust
  pub fn start_graph(self: &mut Self) -> fmt::Result { /* ... */ }
  ```

- ```rust
  pub fn end_graph(self: &mut Self) -> fmt::Result { /* ... */ }
  ```

###### Trait Implementations

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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **TreeNodeVisitor**
  - ```rust
    fn f_down(self: &mut Self, plan: &''n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion> { /* ... */ }
    ```

  - ```rust
    fn f_up(self: &mut Self, _plan: &LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion> { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **IntoEither**
#### Struct `PgJsonVisitor`

Formats plans to display as postgresql plan json format.

There are already many existing visualizer for this format, for example [dalibo](https://explain.dalibo.com/).
Unfortunately, there is no formal spec for this format, but it is widely used in the PostgreSQL community.

Here is an example of the format:

```json
[
    {
        "Plan": {
            "Node Type": "Sort",
            "Output": [
                "question_1.id",
                "question_1.title",
                "question_1.text",
                "question_1.file",
                "question_1.type",
                "question_1.source",
                "question_1.exam_id"
            ],
            "Sort Key": [
                "question_1.id"
            ],
            "Plans": [
                {
                    "Node Type": "Seq Scan",
                    "Parent Relationship": "Left",
                    "Relation Name": "question",
                    "Schema": "public",
                    "Alias": "question_1",
                    "Output": [
                       "question_1.id",
                        "question_1.title",
                       "question_1.text",
                        "question_1.file",
                        "question_1.type",
                        "question_1.source",
                        "question_1.exam_id"
                    ],
                    "Filter": "(question_1.exam_id = 1)"
                }
            ]
        }
    }
]
```

```rust
pub struct PgJsonVisitor<''a, ''b> {
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
  pub fn new(f: &''a mut fmt::Formatter<''b>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_schema(self: &mut Self, with_schema: bool) { /* ... */ }
  ```
  Sets a flag which controls if the output schema is displayed

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **UnwindSafe**
- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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

- **Unpin**
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

- **TreeNodeVisitor**
  - ```rust
    fn f_down(self: &mut Self, node: &''n LogicalPlan) -> datafusion_common::Result<TreeNodeRecursion> { /* ... */ }
    ```

  - ```rust
    fn f_up(self: &mut Self, _node: &<Self as >::Node) -> datafusion_common::Result<TreeNodeRecursion> { /* ... */ }
    ```

### Functions

#### Function `display_schema`

Print the schema in a compact representation to `buf`

For example: `foo:Utf8` if `foo` can not be null, and
`foo:Utf8;N` if `foo` is nullable.

```
use arrow::datatypes::{Field, Schema, DataType};
# use datafusion_expr::logical_plan::display_schema;
let schema = Schema::new(vec![
    Field::new("id", DataType::Int32, false),
    Field::new("first_name", DataType::Utf8, true),
 ]);

 assert_eq!(
     "[id:Int32, first_name:Utf8;N]",
     format!("{}", display_schema(&schema))
 );
```

```rust
pub fn display_schema(schema: &arrow::datatypes::Schema) -> impl fmt::Display + ''_ { /* ... */ }
```

## Module `dml`

```rust
pub mod dml { /* ... */ }
```

### Types

#### Struct `CopyTo`

Operator that copies the contents of a database to file(s)

```rust
pub struct CopyTo {
    pub input: std::sync::Arc<crate::LogicalPlan>,
    pub output_url: String,
    pub partition_by: Vec<String>,
    pub file_type: std::sync::Arc<dyn FileType>,
    pub options: std::collections::HashMap<String, String>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `input` | `std::sync::Arc<crate::LogicalPlan>` | The relation that determines the tuples to write to the output file(s) |
| `output_url` | `String` | The location to write the file(s) |
| `partition_by` | `Vec<String>` | Determines which, if any, columns should be used for hive-style partitioned writes |
| `file_type` | `std::sync::Arc<dyn FileType>` | File type trait |
| `options` | `std::collections::HashMap<String, String>` | SQL Options that can affect the formats |

##### Implementations

###### Trait Implementations

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CopyTo { /* ... */ }
    ```

- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Freeze**
- **IntoEither**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

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
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

#### Struct `DmlStatement`

The operator that modifies the content of a database (adapted from
substrait WriteRel)

```rust
pub struct DmlStatement {
    pub table_name: datafusion_common::TableReference,
    pub target: std::sync::Arc<dyn TableSource>,
    pub op: WriteOp,
    pub input: std::sync::Arc<crate::LogicalPlan>,
    pub output_schema: datafusion_common::DFSchemaRef,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `table_name` | `datafusion_common::TableReference` | The table name |
| `target` | `std::sync::Arc<dyn TableSource>` | this is target table to insert into |
| `op` | `WriteOp` | The type of operation to perform |
| `input` | `std::sync::Arc<crate::LogicalPlan>` | The relation that determines the tuples to add/remove/modify the schema must match with table_schema |
| `output_schema` | `datafusion_common::DFSchemaRef` | The schema of the output relation |

##### Implementations

###### Methods

- ```rust
  pub fn new(table_name: TableReference, target: Arc<dyn TableSource>, op: WriteOp, input: Arc<LogicalPlan>) -> Self { /* ... */ }
  ```
  Creates a new DML statement with the output schema set to a single `count` column.

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Return a descriptive name of this [`DmlStatement`]

###### Trait Implementations

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Self) -> bool { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
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

- **RefUnwindSafe**
- **Hash**
  - ```rust
    fn hash<H: Hasher>(self: &Self, state: &mut H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &Self) -> Option<Ordering> { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
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

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Send**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DmlStatement { /* ... */ }
    ```

- **Eq**
#### Enum `WriteOp`

```rust
pub enum WriteOp {
    Insert(InsertOp),
    Delete,
    Update,
    Ctas,
}
```

##### Variants

###### `Insert`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `InsertOp` |  |

###### `Delete`

###### `Update`

###### `Ctas`

##### Implementations

###### Methods

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Return a descriptive name of this [`WriteOp`]

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **UnwindSafe**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

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
- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WriteOp { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WriteOp) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WriteOp) -> bool { /* ... */ }
    ```

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Sync**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `InsertOp`

```rust
pub enum InsertOp {
    Append,
    Overwrite,
    Replace,
}
```

##### Variants

###### `Append`

Appends new rows to the existing table without modifying any
existing rows. This corresponds to the SQL `INSERT INTO` query.

###### `Overwrite`

Overwrites all existing rows in the table with the new rows.
This corresponds to the SQL `INSERT OVERWRITE` query.

###### `Replace`

If any existing rows collides with the inserted rows (typically based
on a unique key or primary key), those existing rows are replaced.
This corresponds to the SQL `REPLACE INTO` query and its equivalents.

##### Implementations

###### Methods

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Return a descriptive name of this [`InsertOp`]

###### Trait Implementations

- **UnwindSafe**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &InsertOp) -> bool { /* ... */ }
    ```

- **Copy**
- **Freeze**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> InsertOp { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **Eq**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &InsertOp) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
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

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
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

- **RefUnwindSafe**
- **Unpin**
- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `tree_node`

 [`TreeNode`] based visiting and rewriting for [`LogicalPlan`]s

Visiting (read only) APIs
* [`LogicalPlan::visit`]: recursively visit the node and all of its inputs
* [`LogicalPlan::visit_with_subqueries`]: recursively visit the node and all of its inputs, including subqueries
* [`LogicalPlan::apply_children`]: recursively visit all inputs of this node
* [`LogicalPlan::apply_expressions`]: (non recursively) visit all expressions of this node
* [`LogicalPlan::apply_subqueries`]: (non recursively) visit all subqueries of this node
* [`LogicalPlan::apply_with_subqueries`]: recursively visit all inputs and embedded subqueries.

Rewriting (update) APIs:
* [`LogicalPlan::exists`]: search for an expression in a plan
* [`LogicalPlan::rewrite`]: recursively rewrite the node and all of its inputs
* [`LogicalPlan::map_children`]: recursively rewrite all inputs of this node
* [`LogicalPlan::map_expressions`]: (non recursively) visit all expressions of this node
* [`LogicalPlan::map_subqueries`]: (non recursively) rewrite all subqueries of this node
* [`LogicalPlan::rewrite_with_subqueries`]: recursively rewrite the node and all of its inputs, including subqueries

(Re)creation APIs (these require substantial cloning and thus are slow):
* [`LogicalPlan::with_new_exprs`]: Create a new plan with different expressions
* [`LogicalPlan::expressions`]: Return a copy of the plan's expressions

```rust
pub mod tree_node { /* ... */ }
```

### Re-exports

#### Re-export `assert_expected_schema`

```rust
pub use invariants::assert_expected_schema;
```

#### Re-export `check_subquery_expr`

```rust
pub use invariants::check_subquery_expr;
```

#### Re-export `InvariantLevel`

```rust
pub use invariants::InvariantLevel;
```

#### Re-export `build_join_schema`

```rust
pub use builder::build_join_schema;
```

#### Re-export `table_scan`

```rust
pub use builder::table_scan;
```

#### Re-export `union`

```rust
pub use builder::union;
```

#### Re-export `wrap_projection_for_join_if_necessary`

```rust
pub use builder::wrap_projection_for_join_if_necessary;
```

#### Re-export `LogicalPlanBuilder`

```rust
pub use builder::LogicalPlanBuilder;
```

#### Re-export `LogicalPlanBuilderOptions`

```rust
pub use builder::LogicalPlanBuilderOptions;
```

#### Re-export `LogicalTableSource`

```rust
pub use builder::LogicalTableSource;
```

#### Re-export `UNNAMED_TABLE`

```rust
pub use builder::UNNAMED_TABLE;
```

#### Re-export `CreateCatalog`

```rust
pub use ddl::CreateCatalog;
```

#### Re-export `CreateCatalogSchema`

```rust
pub use ddl::CreateCatalogSchema;
```

#### Re-export `CreateExternalTable`

```rust
pub use ddl::CreateExternalTable;
```

#### Re-export `CreateFunction`

```rust
pub use ddl::CreateFunction;
```

#### Re-export `CreateFunctionBody`

```rust
pub use ddl::CreateFunctionBody;
```

#### Re-export `CreateIndex`

```rust
pub use ddl::CreateIndex;
```

#### Re-export `CreateMemoryTable`

```rust
pub use ddl::CreateMemoryTable;
```

#### Re-export `CreateView`

```rust
pub use ddl::CreateView;
```

#### Re-export `DdlStatement`

```rust
pub use ddl::DdlStatement;
```

#### Re-export `DropCatalogSchema`

```rust
pub use ddl::DropCatalogSchema;
```

#### Re-export `DropFunction`

```rust
pub use ddl::DropFunction;
```

#### Re-export `DropTable`

```rust
pub use ddl::DropTable;
```

#### Re-export `DropView`

```rust
pub use ddl::DropView;
```

#### Re-export `OperateFunctionArg`

```rust
pub use ddl::OperateFunctionArg;
```

#### Re-export `DmlStatement`

```rust
pub use dml::DmlStatement;
```

#### Re-export `WriteOp`

```rust
pub use dml::WriteOp;
```

#### Re-export `projection_schema`

```rust
pub use plan::projection_schema;
```

#### Re-export `Aggregate`

```rust
pub use plan::Aggregate;
```

#### Re-export `Analyze`

```rust
pub use plan::Analyze;
```

#### Re-export `ColumnUnnestList`

```rust
pub use plan::ColumnUnnestList;
```

#### Re-export `DescribeTable`

```rust
pub use plan::DescribeTable;
```

#### Re-export `Distinct`

```rust
pub use plan::Distinct;
```

#### Re-export `DistinctOn`

```rust
pub use plan::DistinctOn;
```

#### Re-export `EmptyRelation`

```rust
pub use plan::EmptyRelation;
```

#### Re-export `Explain`

```rust
pub use plan::Explain;
```

#### Re-export `ExplainFormat`

```rust
pub use plan::ExplainFormat;
```

#### Re-export `Extension`

```rust
pub use plan::Extension;
```

#### Re-export `FetchType`

```rust
pub use plan::FetchType;
```

#### Re-export `Filter`

```rust
pub use plan::Filter;
```

#### Re-export `Join`

```rust
pub use plan::Join;
```

#### Re-export `JoinConstraint`

```rust
pub use plan::JoinConstraint;
```

#### Re-export `JoinType`

```rust
pub use plan::JoinType;
```

#### Re-export `Limit`

```rust
pub use plan::Limit;
```

#### Re-export `LogicalPlan`

```rust
pub use plan::LogicalPlan;
```

#### Re-export `Partitioning`

```rust
pub use plan::Partitioning;
```

#### Re-export `PlanType`

```rust
pub use plan::PlanType;
```

#### Re-export `Projection`

```rust
pub use plan::Projection;
```

#### Re-export `RecursiveQuery`

```rust
pub use plan::RecursiveQuery;
```

#### Re-export `Repartition`

```rust
pub use plan::Repartition;
```

#### Re-export `SkipType`

```rust
pub use plan::SkipType;
```

#### Re-export `Sort`

```rust
pub use plan::Sort;
```

#### Re-export `StringifiedPlan`

```rust
pub use plan::StringifiedPlan;
```

#### Re-export `Subquery`

```rust
pub use plan::Subquery;
```

#### Re-export `SubqueryAlias`

```rust
pub use plan::SubqueryAlias;
```

#### Re-export `TableScan`

```rust
pub use plan::TableScan;
```

#### Re-export `ToStringifiedPlan`

```rust
pub use plan::ToStringifiedPlan;
```

#### Re-export `Union`

```rust
pub use plan::Union;
```

#### Re-export `Unnest`

```rust
pub use plan::Unnest;
```

#### Re-export `Values`

```rust
pub use plan::Values;
```

#### Re-export `Window`

```rust
pub use plan::Window;
```

#### Re-export `Deallocate`

```rust
pub use statement::Deallocate;
```

#### Re-export `Execute`

```rust
pub use statement::Execute;
```

#### Re-export `Prepare`

```rust
pub use statement::Prepare;
```

#### Re-export `SetVariable`

```rust
pub use statement::SetVariable;
```

#### Re-export `Statement`

```rust
pub use statement::Statement;
```

#### Re-export `TransactionAccessMode`

```rust
pub use statement::TransactionAccessMode;
```

#### Re-export `TransactionConclusion`

```rust
pub use statement::TransactionConclusion;
```

#### Re-export `TransactionEnd`

```rust
pub use statement::TransactionEnd;
```

#### Re-export `TransactionIsolationLevel`

```rust
pub use statement::TransactionIsolationLevel;
```

#### Re-export `TransactionStart`

```rust
pub use statement::TransactionStart;
```

#### Re-export `display_schema`

```rust
pub use display::display_schema;
```

#### Re-export `UserDefinedLogicalNode`

```rust
pub use extension::UserDefinedLogicalNode;
```

#### Re-export `UserDefinedLogicalNodeCore`

```rust
pub use extension::UserDefinedLogicalNodeCore;
```

## Module `planner`

[`ContextProvider`] and [`ExprPlanner`] APIs to customize SQL query planning

```rust
pub mod planner { /* ... */ }
```

### Types

#### Struct `RawBinaryExpr`

An operator with two arguments to plan

Note `left` and `right` are DataFusion [`Expr`]s but the `op` is the SQL AST
operator.

This structure is used by [`ExprPlanner`] to plan operators with
custom expressions.

```rust
pub struct RawBinaryExpr {
    pub op: ast::BinaryOperator,
    pub left: crate::Expr,
    pub right: crate::Expr,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `op` | `ast::BinaryOperator` |  |
| `left` | `crate::Expr` |  |
| `right` | `crate::Expr` |  |

##### Implementations

###### Trait Implementations

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RawBinaryExpr { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Send**
- **UnwindSafe**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
#### Struct `RawFieldAccessExpr`

An expression with GetFieldAccess to plan

This structure is used by [`ExprPlanner`] to plan operators with
custom expressions.

```rust
pub struct RawFieldAccessExpr {
    pub field_access: crate::GetFieldAccess,
    pub expr: crate::Expr,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `field_access` | `crate::GetFieldAccess` |  |
| `expr` | `crate::Expr` |  |

##### Implementations

###### Trait Implementations

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RawFieldAccessExpr { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Unpin**
#### Struct `RawDictionaryExpr`

A Dictionary literal expression `{ key: value, ...}`

This structure is used by [`ExprPlanner`] to plan operators with
custom expressions.

```rust
pub struct RawDictionaryExpr {
    pub keys: Vec<crate::Expr>,
    pub values: Vec<crate::Expr>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `keys` | `Vec<crate::Expr>` |  |
| `values` | `Vec<crate::Expr>` |  |

##### Implementations

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RawDictionaryExpr { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **IntoEither**
- **RefUnwindSafe**
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
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

#### Struct `RawAggregateExpr`

This structure is used by `AggregateFunctionPlanner` to plan operators with
custom expressions.

```rust
pub struct RawAggregateExpr {
    pub func: std::sync::Arc<crate::AggregateUDF>,
    pub args: Vec<crate::Expr>,
    pub distinct: bool,
    pub filter: Option<Box<crate::Expr>>,
    pub order_by: Option<Vec<crate::SortExpr>>,
    pub null_treatment: Option<sqlparser::ast::NullTreatment>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `func` | `std::sync::Arc<crate::AggregateUDF>` |  |
| `args` | `Vec<crate::Expr>` |  |
| `distinct` | `bool` |  |
| `filter` | `Option<Box<crate::Expr>>` |  |
| `order_by` | `Option<Vec<crate::SortExpr>>` |  |
| `null_treatment` | `Option<sqlparser::ast::NullTreatment>` |  |

##### Implementations

###### Trait Implementations

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RawAggregateExpr { /* ... */ }
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

- **MaybeSendSync**
- **IntoEither**
- **Send**
- **ErasedDestructor**
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

- **UnwindSafe**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `RawWindowExpr`

This structure is used by `WindowFunctionPlanner` to plan operators with
custom expressions.

```rust
pub struct RawWindowExpr {
    pub func_def: crate::WindowFunctionDefinition,
    pub args: Vec<crate::Expr>,
    pub partition_by: Vec<crate::Expr>,
    pub order_by: Vec<crate::SortExpr>,
    pub window_frame: crate::WindowFrame,
    pub null_treatment: Option<sqlparser::ast::NullTreatment>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `func_def` | `crate::WindowFunctionDefinition` |  |
| `args` | `Vec<crate::Expr>` |  |
| `partition_by` | `Vec<crate::Expr>` |  |
| `order_by` | `Vec<crate::SortExpr>` |  |
| `window_frame` | `crate::WindowFrame` |  |
| `null_treatment` | `Option<sqlparser::ast::NullTreatment>` |  |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> RawWindowExpr { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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
- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Enum `PlannerResult`

Result of planning a raw expr with [`ExprPlanner`]

```rust
pub enum PlannerResult<T> {
    Planned(crate::Expr),
    Original(T),
}
```

##### Variants

###### `Planned`

The raw expression was successfully planned as a new [`Expr`]

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::Expr` |  |

###### `Original`

The raw expression could not be planned, and is returned unmodified

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `T` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PlannerResult<T> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Traits

#### Trait `ContextProvider`

Provides the `SQL` query planner meta-data about tables and
functions referenced in SQL statements, without a direct dependency on the
`datafusion` Catalog structures such as [`TableProvider`]

[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

```rust
pub trait ContextProvider {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_table_source`: Returns a table by reference, if it exists
- `get_function_meta`: Return the scalar function with a given name, if any
- `get_aggregate_meta`: Return the aggregate function with a given name, if any
- `get_window_meta`: Return the window function with a given name, if any
- `get_variable_type`: Return the system/user-defined variable type, if any
- `options`: Return overall configuration options
- `udf_names`: Return all scalar function names
- `udaf_names`: Return all aggregate function names
- `udwf_names`: Return all window function names

##### Provided Methods

- ```rust
  fn get_file_type(self: &Self, _ext: &str) -> Result<Arc<dyn FileType>> { /* ... */ }
  ```
  Return the type of a file based on its extension (e.g. `.parquet`)

- ```rust
  fn get_table_function_source(self: &Self, _name: &str, _args: Vec<Expr>) -> Result<Arc<dyn TableSource>> { /* ... */ }
  ```
  Getter for a table function

- ```rust
  fn create_cte_work_table(self: &Self, _name: &str, _schema: SchemaRef) -> Result<Arc<dyn TableSource>> { /* ... */ }
  ```
  Provides an intermediate table that is used to store the results of a CTE during execution

- ```rust
  fn get_expr_planners(self: &Self) -> &[Arc<dyn ExprPlanner>] { /* ... */ }
  ```
  Return [`ExprPlanner`] extensions for planning expressions

- ```rust
  fn get_type_planner(self: &Self) -> Option<Arc<dyn TypePlanner>> { /* ... */ }
  ```
  Return [`TypePlanner`] extensions for planning data types

#### Trait `ExprPlanner`

Customize planning of SQL AST expressions to [`Expr`]s

```rust
pub trait ExprPlanner: Debug + Send + Sync {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn plan_binary_op(self: &Self, expr: RawBinaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawBinaryExpr>> { /* ... */ }
  ```
  Plan the binary operation between two expressions, returns original

- ```rust
  fn plan_field_access(self: &Self, expr: RawFieldAccessExpr, _schema: &DFSchema) -> Result<PlannerResult<RawFieldAccessExpr>> { /* ... */ }
  ```
  Plan the field access expression, such as `foo.bar`

- ```rust
  fn plan_array_literal(self: &Self, exprs: Vec<Expr>, _schema: &DFSchema) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
  ```
  Plan an array literal, such as `[1, 2, 3]`

- ```rust
  fn plan_position(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
  ```
  Plan a `POSITION` expression, such as `POSITION(<expr> in <expr>)`

- ```rust
  fn plan_dictionary_literal(self: &Self, expr: RawDictionaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawDictionaryExpr>> { /* ... */ }
  ```
  Plan a dictionary literal, such as `{ key: value, ...}`

- ```rust
  fn plan_extract(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
  ```
  Plan an extract expression, such as`EXTRACT(month FROM foo)`

- ```rust
  fn plan_substring(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
  ```
  Plan an substring expression, such as `SUBSTRING(<expr> [FROM <expr>] [FOR <expr>])`

- ```rust
  fn plan_struct_literal(self: &Self, args: Vec<Expr>, _is_named_struct: bool) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
  ```
  Plans a struct literal, such as  `{'field1' : expr1, 'field2' : expr2, ...}`

- ```rust
  fn plan_overlay(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
  ```
  Plans an overlay expression, such as `overlay(str PLACING substr FROM pos [FOR count])`

- ```rust
  fn plan_make_map(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
  ```
  Plans a `make_map` expression, such as `make_map(key1, value1, key2, value2, ...)`

- ```rust
  fn plan_compound_identifier(self: &Self, _field: &Field, _qualifier: Option<&TableReference>, _nested_names: &[String]) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
  ```
  Plans compound identifier such as `db.schema.table` for non-empty nested names

- ```rust
  fn plan_any(self: &Self, expr: RawBinaryExpr) -> Result<PlannerResult<RawBinaryExpr>> { /* ... */ }
  ```
  Plans `ANY` expression, such as `expr = ANY(array_expr)`

- ```rust
  fn plan_aggregate(self: &Self, expr: RawAggregateExpr) -> Result<PlannerResult<RawAggregateExpr>> { /* ... */ }
  ```
  Plans aggregate functions, such as `COUNT(<expr>)`

- ```rust
  fn plan_window(self: &Self, expr: RawWindowExpr) -> Result<PlannerResult<RawWindowExpr>> { /* ... */ }
  ```
  Plans window functions, such as `COUNT(<expr>)`

#### Trait `TypePlanner`

Customize planning SQL types to DataFusion (Arrow) types.

```rust
pub trait TypePlanner: Debug + Send + Sync {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn plan_type(self: &Self, _sql_type: &ast::DataType) -> Result<Option<DataType>> { /* ... */ }
  ```
  Plan SQL [`ast::DataType`] to DataFusion [`DataType`]

## Module `registry`

FunctionRegistry trait

```rust
pub mod registry { /* ... */ }
```

### Types

#### Struct `MemoryFunctionRegistry`

A  [`FunctionRegistry`] that uses in memory [`HashMap`]s

```rust
pub struct MemoryFunctionRegistry {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> MemoryFunctionRegistry { /* ... */ }
    ```

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

- **UnwindSafe**
- **FunctionRegistry**
  - ```rust
    fn udfs(self: &Self) -> HashSet<String> { /* ... */ }
    ```

  - ```rust
    fn udf(self: &Self, name: &str) -> Result<Arc<ScalarUDF>> { /* ... */ }
    ```

  - ```rust
    fn udaf(self: &Self, name: &str) -> Result<Arc<AggregateUDF>> { /* ... */ }
    ```

  - ```rust
    fn udwf(self: &Self, name: &str) -> Result<Arc<WindowUDF>> { /* ... */ }
    ```

  - ```rust
    fn register_udf(self: &mut Self, udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>> { /* ... */ }
    ```

  - ```rust
    fn register_udaf(self: &mut Self, udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>> { /* ... */ }
    ```

  - ```rust
    fn register_udwf(self: &mut Self, udaf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>> { /* ... */ }
    ```

  - ```rust
    fn expr_planners(self: &Self) -> Vec<Arc<dyn ExprPlanner>> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Traits

#### Trait `FunctionRegistry`

A registry knows how to build logical expressions out of user-defined function' names

```rust
pub trait FunctionRegistry {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `udfs`: Set of all available udfs.
- `udf`: Returns a reference to the user defined scalar function (udf) named
- `udaf`: Returns a reference to the user defined aggregate function (udaf) named
- `udwf`: Returns a reference to the user defined window function (udwf) named
- `expr_planners`: Set of all registered [`ExprPlanner`]s

##### Provided Methods

- ```rust
  fn register_udf(self: &mut Self, _udf: Arc<ScalarUDF>) -> Result<Option<Arc<ScalarUDF>>> { /* ... */ }
  ```
  Registers a new [`ScalarUDF`], returning any previously registered

- ```rust
  fn register_udaf(self: &mut Self, _udaf: Arc<AggregateUDF>) -> Result<Option<Arc<AggregateUDF>>> { /* ... */ }
  ```
  Registers a new [`AggregateUDF`], returning any previously registered

- ```rust
  fn register_udwf(self: &mut Self, _udaf: Arc<WindowUDF>) -> Result<Option<Arc<WindowUDF>>> { /* ... */ }
  ```
  Registers a new [`WindowUDF`], returning any previously registered

- ```rust
  fn deregister_udf(self: &mut Self, _name: &str) -> Result<Option<Arc<ScalarUDF>>> { /* ... */ }
  ```
  Deregisters a [`ScalarUDF`], returning the implementation that was

- ```rust
  fn deregister_udaf(self: &mut Self, _name: &str) -> Result<Option<Arc<AggregateUDF>>> { /* ... */ }
  ```
  Deregisters a [`AggregateUDF`], returning the implementation that was

- ```rust
  fn deregister_udwf(self: &mut Self, _name: &str) -> Result<Option<Arc<WindowUDF>>> { /* ... */ }
  ```
  Deregisters a [`WindowUDF`], returning the implementation that was

- ```rust
  fn register_function_rewrite(self: &mut Self, _rewrite: Arc<dyn FunctionRewrite + Send + Sync>) -> Result<()> { /* ... */ }
  ```
  Registers a new [`FunctionRewrite`] with the registry.

- ```rust
  fn register_expr_planner(self: &mut Self, _expr_planner: Arc<dyn ExprPlanner>) -> Result<()> { /* ... */ }
  ```
  Registers a new [`ExprPlanner`] with the registry.

##### Implementations

This trait is implemented for the following types:

- `MemoryFunctionRegistry`

#### Trait `SerializerRegistry`

Serializer and deserializer registry for extensions like [UserDefinedLogicalNode].

```rust
pub trait SerializerRegistry: Debug + Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `serialize_logical_plan`: Serialize this node to a byte array. This serialization should not include
- `deserialize_logical_plan`: Deserialize user defined logical plan node ([UserDefinedLogicalNode]) from

## Module `simplify`

Structs and traits to provide the information needed for expression simplification.

```rust
pub mod simplify { /* ... */ }
```

### Types

#### Struct `SimplifyContext`

Provides simplification information based on DFSchema and
[`ExecutionProps`]. This is the default implementation used by DataFusion

# Example
See the `simplify_demo` in the [`expr_api` example]

[`expr_api` example]: https://github.com/apache/datafusion/blob/main/datafusion-examples/examples/expr_api.rs

```rust
pub struct SimplifyContext<''a> {
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
  pub fn new(props: &''a ExecutionProps) -> Self { /* ... */ }
  ```
  Create a new SimplifyContext

- ```rust
  pub fn with_schema(self: Self, schema: DFSchemaRef) -> Self { /* ... */ }
  ```
  Register a [`DFSchemaRef`] with this context

###### Trait Implementations

- **RefUnwindSafe**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **SimplifyInfo**
  - ```rust
    fn is_boolean_type(self: &Self, expr: &Expr) -> Result<bool> { /* ... */ }
    ```
    Returns true if this Expr has boolean type

  - ```rust
    fn nullable(self: &Self, expr: &Expr) -> Result<bool> { /* ... */ }
    ```
    Returns true if expr is nullable

  - ```rust
    fn get_data_type(self: &Self, expr: &Expr) -> Result<DataType> { /* ... */ }
    ```
    Returns data type of this expr needed for determining optimized int type of a value

  - ```rust
    fn execution_props(self: &Self) -> &ExecutionProps { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SimplifyContext<''a> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

#### Enum `ExprSimplifyResult`

Was the expression simplified?

```rust
pub enum ExprSimplifyResult {
    Simplified(crate::Expr),
    Original(Vec<crate::Expr>),
}
```

##### Variants

###### `Simplified`

The function call was simplified to an entirely new Expr

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `crate::Expr` |  |

###### `Original`

The function call could not be simplified, and the arguments
are return unmodified.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Vec<crate::Expr>` |  |

##### Implementations

###### Trait Implementations

- **Send**
- **ErasedDestructor**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Traits

#### Trait `SimplifyInfo`

Provides the information necessary to apply algebraic simplification to an
[Expr]. See [SimplifyContext] for one concrete implementation.

This trait exists so that other systems can plug schema
information in without having to create `DFSchema` objects. If you
have a [`DFSchemaRef`] you can use [`SimplifyContext`]

```rust
pub trait SimplifyInfo {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `is_boolean_type`: Returns true if this Expr has boolean type
- `nullable`: Returns true of this expr is nullable (could possibly be NULL)
- `execution_props`: Returns details needed for partial expression evaluation
- `get_data_type`: Returns data type of this expr needed for determining optimized int type of a value

##### Implementations

This trait is implemented for the following types:

- `SimplifyContext<''_>`

## Module `sort_properties`

```rust
pub mod sort_properties { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_expr_common::sort_properties::*`

```rust
pub use datafusion_expr_common::sort_properties::*;
```

## Module `statistics`

```rust
pub mod statistics { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_expr_common::statistics::*`

```rust
pub use datafusion_expr_common::statistics::*;
```

## Module `test`

```rust
pub mod test { /* ... */ }
```

### Modules

## Module `function_stub`

Aggregate function stubs for test in expr / optimizer.

These are used to avoid a dependence on `datafusion-functions-aggregate` which live in a different crate

```rust
pub mod function_stub { /* ... */ }
```

### Types

#### Struct `Sum`

Stub `sum` used for optimizer testing

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

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, _args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn groups_accumulator_supported(self: &Self, _args: AccumulatorArgs<''_>) -> bool { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn order_sensitivity(self: &Self) -> AggregateOrderSensitivity { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `Count`

Testing stub implementation of COUNT aggregate

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
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
    fn is_nullable(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, _args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Unpin**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **UnwindSafe**
#### Struct `Min`

Testing stub implementation of Min aggregate

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **MaybeSendSync**
- **Sync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, _args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn is_descending(self: &Self) -> Option<bool> { /* ... */ }
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
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **IntoEither**
- **RefUnwindSafe**
#### Struct `Max`

Testing stub implementation of MAX aggregate

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

- **UnwindSafe**
- **RefUnwindSafe**
- **Sync**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, _args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn create_groups_accumulator(self: &Self, _args: AccumulatorArgs<''_>) -> Result<Box<dyn GroupsAccumulator>> { /* ... */ }
    ```

  - ```rust
    fn reverse_expr(self: &Self) -> ReversedUDAF { /* ... */ }
    ```

  - ```rust
    fn is_descending(self: &Self) -> Option<bool> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<''_>) -> std::fmt::Result { /* ... */ }
    ```

- **Freeze**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
#### Struct `Avg`

Testing stub implementation of avg aggregate

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

- **RefUnwindSafe**
- **ErasedDestructor**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Unpin**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn accumulator(self: &Self, _acc_args: AccumulatorArgs<''_>) -> Result<Box<dyn Accumulator>> { /* ... */ }
    ```

  - ```rust
    fn state_fields(self: &Self, _args: StateFieldsArgs<''_>) -> Result<Vec<Field>> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **Sync**
### Functions

#### Function `sum_udaf`

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Sum`]

```rust
pub fn sum_udaf() -> std::sync::Arc<crate::AggregateUDF> { /* ... */ }
```

#### Function `sum`

```rust
pub fn sum(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `count_udaf`

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Count`]

```rust
pub fn count_udaf() -> std::sync::Arc<crate::AggregateUDF> { /* ... */ }
```

#### Function `count`

```rust
pub fn count(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `avg_udaf`

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Avg`]

```rust
pub fn avg_udaf() -> std::sync::Arc<crate::AggregateUDF> { /* ... */ }
```

#### Function `avg`

```rust
pub fn avg(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `min_udaf`

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Min`]

```rust
pub fn min_udaf() -> std::sync::Arc<crate::AggregateUDF> { /* ... */ }
```

#### Function `min`

```rust
pub fn min(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

#### Function `max_udaf`

AggregateFunction that returns a [AggregateUDF](crate::AggregateUDF) for [`Max`]

```rust
pub fn max_udaf() -> std::sync::Arc<crate::AggregateUDF> { /* ... */ }
```

#### Function `max`

```rust
pub fn max(expr: crate::Expr) -> crate::Expr { /* ... */ }
```

## Module `tree_node`

Tree node implementation for Logical Expressions

```rust
pub mod tree_node { /* ... */ }
```

## Module `type_coercion`

Type coercion rules for DataFusion

Coercion is performed automatically by DataFusion when the types
of arguments passed to a function or needed by operators do not
exactly match the types required by that function / operator. In
this case, DataFusion will attempt to *coerce* the arguments to
types accepted by the function by inserting CAST operations.

CAST operations added by coercion are lossless and never discard
information.

For example coercion from i32 -> i64 might be
performed because all valid i32 values can be represented using an
i64. However, i64 -> i32 is never performed as there are i64
values which can not be represented by i32 values.

```rust
pub mod type_coercion { /* ... */ }
```

### Modules

## Module `aggregates`

```rust
pub mod aggregates { /* ... */ }
```

### Re-exports

#### Re-export `datafusion_expr_common::type_coercion::aggregates::*`

```rust
pub use datafusion_expr_common::type_coercion::aggregates::*;
```

## Module `functions`

```rust
pub mod functions { /* ... */ }
```

### Functions

#### Function `data_types_with_scalar_udf`

Performs type coercion for scalar function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

```rust
pub fn data_types_with_scalar_udf(current_types: &[arrow::datatypes::DataType], func: &crate::ScalarUDF) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>> { /* ... */ }
```

#### Function `data_types_with_aggregate_udf`

Performs type coercion for aggregate function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

```rust
pub fn data_types_with_aggregate_udf(current_types: &[arrow::datatypes::DataType], func: &crate::AggregateUDF) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>> { /* ... */ }
```

#### Function `data_types_with_window_udf`

Performs type coercion for window function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

```rust
pub fn data_types_with_window_udf(current_types: &[arrow::datatypes::DataType], func: &crate::WindowUDF) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>> { /* ... */ }
```

#### Function `data_types`

Performs type coercion for function arguments.

Returns the data types to which each argument must be coerced to
match `signature`.

For more details on coercion in general, please see the
[`type_coercion`](crate::type_coercion) module.

```rust
pub fn data_types</* synthetic */ impl AsRef<str>: AsRef<str>>(function_name: impl AsRef<str>, current_types: &[arrow::datatypes::DataType], signature: &crate::Signature) -> datafusion_common::Result<Vec<arrow::datatypes::DataType>> { /* ... */ }
```

#### Function `can_coerce_from`

Return true if a value of type `type_from` can be coerced
(losslessly converted) into a value of `type_to`

See the module level documentation for more detail on coercion.

```rust
pub fn can_coerce_from(type_into: &arrow::datatypes::DataType, type_from: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

## Module `other`

```rust
pub mod other { /* ... */ }
```

### Functions

#### Function `get_coerce_type_for_list`

Attempts to coerce the types of `list_types` to be comparable with the
`expr_type`.
Returns the common data type for `expr_type` and `list_types`

```rust
pub fn get_coerce_type_for_list(expr_type: &arrow::datatypes::DataType, list_types: &[arrow::datatypes::DataType]) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `get_coerce_type_for_case_expression`

Find a common coerceable type for all `when_or_then_types` as well
and the `case_or_else_type`, if specified.
Returns the common data type for `when_or_then_types` and `case_or_else_type`

```rust
pub fn get_coerce_type_for_case_expression(when_or_then_types: &[arrow::datatypes::DataType], case_or_else_type: Option<&arrow::datatypes::DataType>) -> Option<arrow::datatypes::DataType> { /* ... */ }
```

### Functions

#### Function `is_signed_numeric`

Determine whether the given data type `dt` represents signed numeric values.

```rust
pub fn is_signed_numeric(dt: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_null`

Determine whether the given data type `dt` is `Null`.

```rust
pub fn is_null(dt: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_timestamp`

Determine whether the given data type `dt` is a `Timestamp`.

```rust
pub fn is_timestamp(dt: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_interval`

Determine whether the given data type 'dt' is a `Interval`.

```rust
pub fn is_interval(dt: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_datetime`

Determine whether the given data type `dt` is a `Date` or `Timestamp`.

```rust
pub fn is_datetime(dt: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_utf8_or_utf8view_or_large_utf8`

Determine whether the given data type `dt` is a `Utf8` or `Utf8View` or `LargeUtf8`.

```rust
pub fn is_utf8_or_utf8view_or_large_utf8(dt: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `is_decimal`

Determine whether the given data type `dt` is a `Decimal`.

```rust
pub fn is_decimal(dt: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

### Re-exports

#### Re-export `binary`

```rust
pub use datafusion_expr_common::type_coercion::binary;
```

## Module `utils`

Expression utilities

```rust
pub mod utils { /* ... */ }
```

### Functions

#### Function `grouping_set_expr_count`

Count the number of distinct exprs in a list of group by expressions. If the
first element is a `GroupingSet` expression then it must be the only expr.

```rust
pub fn grouping_set_expr_count(group_expr: &[crate::Expr]) -> datafusion_common::Result<usize> { /* ... */ }
```

#### Function `enumerate_grouping_sets`

Convert multiple grouping expressions into one [`GroupingSet::GroupingSets`],\
if the grouping expression does not contain [`Expr::GroupingSet`] or only has one expression,\
no conversion will be performed.

e.g.

person.id,\
GROUPING SETS ((person.age, person.salary),(person.age)),\
ROLLUP(person.state, person.birth_date)

=>

GROUPING SETS (\
  (person.id, person.age, person.salary),\
  (person.id, person.age, person.salary, person.state),\
  (person.id, person.age, person.salary, person.state, person.birth_date),\
  (person.id, person.age),\
  (person.id, person.age, person.state),\
  (person.id, person.age, person.state, person.birth_date)\
)

```rust
pub fn enumerate_grouping_sets(group_expr: Vec<crate::Expr>) -> datafusion_common::Result<Vec<crate::Expr>> { /* ... */ }
```

#### Function `grouping_set_to_exprlist`

Find all distinct exprs in a list of group by expressions. If the
first element is a `GroupingSet` expression then it must be the only expr.

```rust
pub fn grouping_set_to_exprlist(group_expr: &[crate::Expr]) -> datafusion_common::Result<Vec<&crate::Expr>> { /* ... */ }
```

#### Function `expr_to_columns`

Recursively walk an expression tree, collecting the unique set of columns
referenced in the expression

```rust
pub fn expr_to_columns(expr: &crate::Expr, accum: &mut std::collections::HashSet<datafusion_common::Column>) -> datafusion_common::Result<()> { /* ... */ }
```

#### Function `expand_wildcard`

Resolves an `Expr::Wildcard` to a collection of `Expr::Column`'s.

```rust
pub fn expand_wildcard(schema: &datafusion_common::DFSchema, plan: &crate::LogicalPlan, wildcard_options: Option<&crate::expr::WildcardOptions>) -> datafusion_common::Result<Vec<crate::Expr>> { /* ... */ }
```

#### Function `expand_qualified_wildcard`

Resolves an `Expr::Wildcard` to a collection of qualified `Expr::Column`'s.

```rust
pub fn expand_qualified_wildcard(qualifier: &datafusion_common::TableReference, schema: &datafusion_common::DFSchema, wildcard_options: Option<&crate::expr::WildcardOptions>) -> datafusion_common::Result<Vec<crate::Expr>> { /* ... */ }
```

#### Function `generate_sort_key`

Generate a sort key for a given window expr's partition_by and order_by expr

```rust
pub fn generate_sort_key(partition_by: &[crate::Expr], order_by: &[crate::expr::Sort]) -> datafusion_common::Result<Vec<(crate::expr::Sort, bool)>> { /* ... */ }
```

#### Function `compare_sort_expr`

Compare the sort expr as PostgreSQL's common_prefix_cmp():
<https://github.com/postgres/postgres/blob/master/src/backend/optimizer/plan/planner.c>

```rust
pub fn compare_sort_expr(sort_expr_a: &crate::expr::Sort, sort_expr_b: &crate::expr::Sort, schema: &datafusion_common::DFSchemaRef) -> std::cmp::Ordering { /* ... */ }
```

#### Function `group_window_expr_by_sort_keys`

Group a slice of window expression expr by their order by expressions

```rust
pub fn group_window_expr_by_sort_keys</* synthetic */ impl IntoIterator<Item = Expr>: IntoIterator<Item = crate::Expr>>(window_expr: impl IntoIterator<Item = crate::Expr>) -> datafusion_common::Result<Vec<(Vec<(crate::expr::Sort, bool)>, Vec<crate::Expr>)>> { /* ... */ }
```

#### Function `find_aggregate_exprs`

Collect all deeply nested `Expr::AggregateFunction`.
They are returned in order of occurrence (depth
first), with duplicates omitted.

```rust
pub fn find_aggregate_exprs<''a, /* synthetic */ impl IntoIterator<Item = &'a Expr>: IntoIterator<Item = &''a crate::Expr>>(exprs: impl IntoIterator<Item = &''a crate::Expr>) -> Vec<crate::Expr> { /* ... */ }
```

#### Function `find_window_exprs`

Collect all deeply nested `Expr::WindowFunction`. They are returned in order of occurrence
(depth first), with duplicates omitted.

```rust
pub fn find_window_exprs(exprs: &[crate::Expr]) -> Vec<crate::Expr> { /* ... */ }
```

#### Function `find_out_reference_exprs`

Collect all deeply nested `Expr::OuterReferenceColumn`. They are returned in order of occurrence
(depth first), with duplicates omitted.

```rust
pub fn find_out_reference_exprs(expr: &crate::Expr) -> Vec<crate::Expr> { /* ... */ }
```

#### Function `inspect_expr_pre`

Recursively inspect an [`Expr`] and all its children.

```rust
pub fn inspect_expr_pre<F, E>(expr: &crate::Expr, f: F) -> datafusion_common::Result<(), E>
where
    F: FnMut(&crate::Expr) -> datafusion_common::Result<(), E> { /* ... */ }
```

#### Function `exprlist_to_fields`

Create field meta-data from an expression, for use in a result set schema

```rust
pub fn exprlist_to_fields<''a, /* synthetic */ impl IntoIterator<Item = &'a Expr>: IntoIterator<Item = &''a crate::Expr>>(exprs: impl IntoIterator<Item = &''a crate::Expr>, plan: &crate::LogicalPlan) -> datafusion_common::Result<Vec<(Option<datafusion_common::TableReference>, std::sync::Arc<arrow::datatypes::Field>)>> { /* ... */ }
```

#### Function `columnize_expr`

Convert an expression into Column expression if it's already provided as input plan.

For example, it rewrites:

```text
.aggregate(vec![col("c1")], vec![sum(col("c2"))])?
.project(vec![col("c1"), sum(col("c2"))?
```

Into:

```text
.aggregate(vec![col("c1")], vec![sum(col("c2"))])?
.project(vec![col("c1"), col("SUM(c2)")?
```

```rust
pub fn columnize_expr(e: crate::Expr, input: &crate::LogicalPlan) -> datafusion_common::Result<crate::Expr> { /* ... */ }
```

#### Function `find_column_exprs`

Collect all deeply nested `Expr::Column`'s. They are returned in order of
appearance (depth first), and may contain duplicates.

```rust
pub fn find_column_exprs(exprs: &[crate::Expr]) -> Vec<crate::Expr> { /* ... */ }
```

#### Function `expr_as_column_expr`

Convert any `Expr` to an `Expr::Column`.

```rust
pub fn expr_as_column_expr(expr: &crate::Expr, plan: &crate::LogicalPlan) -> datafusion_common::Result<crate::Expr> { /* ... */ }
```

#### Function `can_hash`

Can this data type be used in hash join equal conditions??
Data types here come from function 'equal_rows', if more data types are supported
in create_hashes, add those data types here to generate join logical plan.

```rust
pub fn can_hash(data_type: &arrow::datatypes::DataType) -> bool { /* ... */ }
```

#### Function `check_all_columns_from_schema`

Check whether all columns are from the schema.

```rust
pub fn check_all_columns_from_schema(columns: &std::collections::HashSet<&datafusion_common::Column>, schema: &datafusion_common::DFSchema) -> datafusion_common::Result<bool> { /* ... */ }
```

#### Function `find_valid_equijoin_key_pair`

Give two sides of the equijoin predicate, return a valid join key pair.
If there is no valid join key pair, return None.

A valid join means:
1. All referenced column of the left side is from the left schema, and
   all referenced column of the right side is from the right schema.
2. Or opposite. All referenced column of the left side is from the right schema,
   and the right side is from the left schema.


```rust
pub fn find_valid_equijoin_key_pair(left_key: &crate::Expr, right_key: &crate::Expr, left_schema: &datafusion_common::DFSchema, right_schema: &datafusion_common::DFSchema) -> datafusion_common::Result<Option<(crate::Expr, crate::Expr)>> { /* ... */ }
```

#### Function `generate_signature_error_msg`

Creates a detailed error message for a function with wrong signature.

For example, a query like `select round(3.14, 1.1);` would yield:
```text
Error during planning: No function matches 'round(Float64, Float64)'. You might need to add explicit type casts.
    Candidate functions:
    round(Float64, Int64)
    round(Float32, Int64)
    round(Float64)
    round(Float32)
```

```rust
pub fn generate_signature_error_msg(func_name: &str, func_signature: datafusion_expr_common::signature::Signature, input_expr_types: &[arrow::datatypes::DataType]) -> String { /* ... */ }
```

#### Function `split_conjunction`

Splits a conjunctive [`Expr`] such as `A AND B AND C` => `[A, B, C]`

See [`split_conjunction_owned`] for more details and an example.

```rust
pub fn split_conjunction(expr: &crate::Expr) -> Vec<&crate::Expr> { /* ... */ }
```

#### Function `iter_conjunction`

Iterate parts in a conjunctive [`Expr`] such as `A AND B AND C` => `[A, B, C]`

See [`split_conjunction_owned`] for more details and an example.

```rust
pub fn iter_conjunction(expr: &crate::Expr) -> impl Iterator<Item = &crate::Expr> { /* ... */ }
```

#### Function `iter_conjunction_owned`

Iterate parts in a conjunctive [`Expr`] such as `A AND B AND C` => `[A, B, C]`

See [`split_conjunction_owned`] for more details and an example.

```rust
pub fn iter_conjunction_owned(expr: crate::Expr) -> impl Iterator<Item = crate::Expr> { /* ... */ }
```

#### Function `split_conjunction_owned`

Splits an owned conjunctive [`Expr`] such as `A AND B AND C` => `[A, B, C]`

This is often used to "split" filter expressions such as `col1 = 5
AND col2 = 10` into [`col1 = 5`, `col2 = 10`];

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::split_conjunction_owned;
// a=1 AND b=2
let expr = col("a").eq(lit(1)).and(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![
  col("a").eq(lit(1)),
  col("b").eq(lit(2)),
];

// use split_conjunction_owned to split them
assert_eq!(split_conjunction_owned(expr), split);
```

```rust
pub fn split_conjunction_owned(expr: crate::Expr) -> Vec<crate::Expr> { /* ... */ }
```

#### Function `split_binary_owned`

Splits an owned binary operator tree [`Expr`] such as `A <OP> B <OP> C` => `[A, B, C]`

This is often used to "split" expressions such as `col1 = 5
AND col2 = 10` into [`col1 = 5`, `col2 = 10`];

# Example
```
# use datafusion_expr::{col, lit, Operator};
# use datafusion_expr::utils::split_binary_owned;
# use std::ops::Add;
// a=1 + b=2
let expr = col("a").eq(lit(1)).add(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![
  col("a").eq(lit(1)),
  col("b").eq(lit(2)),
];

// use split_binary_owned to split them
assert_eq!(split_binary_owned(expr, Operator::Plus), split);
```

```rust
pub fn split_binary_owned(expr: crate::Expr, op: crate::Operator) -> Vec<crate::Expr> { /* ... */ }
```

#### Function `split_binary`

Splits an binary operator tree [`Expr`] such as `A <OP> B <OP> C` => `[A, B, C]`

See [`split_binary_owned`] for more details and an example.

```rust
pub fn split_binary(expr: &crate::Expr, op: crate::Operator) -> Vec<&crate::Expr> { /* ... */ }
```

#### Function `conjunction`

Combines an array of filter expressions into a single filter
expression consisting of the input filter expressions joined with
logical AND.

Returns None if the filters array is empty.

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::conjunction;
// a=1 AND b=2
let expr = col("a").eq(lit(1)).and(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![
  col("a").eq(lit(1)),
  col("b").eq(lit(2)),
];

// use conjunction to join them together with `AND`
assert_eq!(conjunction(split), Some(expr));
```

```rust
pub fn conjunction</* synthetic */ impl IntoIterator<Item = Expr>: IntoIterator<Item = crate::Expr>>(filters: impl IntoIterator<Item = crate::Expr>) -> Option<crate::Expr> { /* ... */ }
```

#### Function `disjunction`

Combines an array of filter expressions into a single filter
expression consisting of the input filter expressions joined with
logical OR.

Returns None if the filters array is empty.

# Example
```
# use datafusion_expr::{col, lit};
# use datafusion_expr::utils::disjunction;
// a=1 OR b=2
let expr = col("a").eq(lit(1)).or(col("b").eq(lit(2)));

// [a=1, b=2]
let split = vec![
  col("a").eq(lit(1)),
  col("b").eq(lit(2)),
];

// use disjunction to join them together with `OR`
assert_eq!(disjunction(split), Some(expr));
```

```rust
pub fn disjunction</* synthetic */ impl IntoIterator<Item = Expr>: IntoIterator<Item = crate::Expr>>(filters: impl IntoIterator<Item = crate::Expr>) -> Option<crate::Expr> { /* ... */ }
```

#### Function `add_filter`

Returns a new [LogicalPlan] that filters the output of  `plan` with a
[LogicalPlan::Filter] with all `predicates` ANDed.

# Example
Before:
```text
plan
```

After:
```text
Filter(predicate)
  plan
```

```rust
pub fn add_filter(plan: crate::LogicalPlan, predicates: &[&crate::Expr]) -> datafusion_common::Result<crate::LogicalPlan> { /* ... */ }
```

#### Function `find_join_exprs`

Looks for correlating expressions: for example, a binary expression with one field from the subquery, and
one not in the subquery (closed upon from outer scope)

# Arguments

* `exprs` - List of expressions that may or may not be joins

# Return value

Tuple of (expressions containing joins, remaining non-join expressions)

```rust
pub fn find_join_exprs(exprs: Vec<&crate::Expr>) -> datafusion_common::Result<(Vec<crate::Expr>, Vec<crate::Expr>)> { /* ... */ }
```

#### Function `only_or_err`

Returns the first (and only) element in a slice, or an error

# Arguments

* `slice` - The slice to extract from

# Return value

The first element, or an error

```rust
pub fn only_or_err<T>(slice: &[T]) -> datafusion_common::Result<&T> { /* ... */ }
```

#### Function `merge_schema`

merge inputs schema into a single schema.

```rust
pub fn merge_schema(inputs: &[&crate::LogicalPlan]) -> datafusion_common::DFSchema { /* ... */ }
```

#### Function `format_state_name`

Build state name. State is the intermediate state of the aggregate function.

```rust
pub fn format_state_name(name: &str, state_name: &str) -> String { /* ... */ }
```

#### Function `collect_subquery_cols`

Determine the set of [`Column`]s produced by the subquery.

```rust
pub fn collect_subquery_cols(exprs: &[crate::Expr], subquery_schema: &datafusion_common::DFSchema) -> datafusion_common::Result<std::collections::BTreeSet<datafusion_common::Column>> { /* ... */ }
```

### Re-exports

#### Re-export `AggregateOrderSensitivity`

```rust
pub use datafusion_functions_aggregate_common::order::AggregateOrderSensitivity;
```

#### Re-export `COUNT_STAR_EXPANSION`

The value to which `COUNT(*)` is expanded to in
`COUNT(<constant>)` expressions

```rust
pub use datafusion_common::utils::expr::COUNT_STAR_EXPANSION;
```

## Module `var_provider`

Variable provider

```rust
pub mod var_provider { /* ... */ }
```

### Types

#### Enum `VarType`

Variable type, system/user defined

```rust
pub enum VarType {
    System,
    UserDefined,
}
```

##### Variants

###### `System`

System variable, like @@version

###### `UserDefined`

User defined variable, like @name

##### Implementations

###### Trait Implementations

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Send**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Allocation**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **StructuralPartialEq**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &VarType) -> bool { /* ... */ }
    ```

- **Eq**
- **ErasedDestructor**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> VarType { /* ... */ }
    ```

- **Sync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

### Traits

#### Trait `VarProvider`

A var provider for `@variable` and `@@variable` runtime values.

```rust
pub trait VarProvider: std::fmt::Debug {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `get_value`: Get variable value
- `get_type`: Return the type of the given variable

### Functions

#### Function `is_system_variables`

Returns true if the specified string is a "system" variable such as
`@@version`

See [`SessionContext::register_variable`] for more details

[`SessionContext::register_variable`]: https://docs.rs/datafusion/latest/datafusion/execution/context/struct.SessionContext.html#method.register_variable

```rust
pub fn is_system_variables(variable_names: &[String]) -> bool { /* ... */ }
```

## Module `window_frame`

Window frame module

The frame-spec determines which output rows are read by an aggregate window function. The frame-spec consists of four parts:
- A frame type - either ROWS, RANGE or GROUPS,
- A starting frame boundary,
- An ending frame boundary,
- An EXCLUDE clause.

```rust
pub mod window_frame { /* ... */ }
```

### Types

#### Struct `WindowFrame`

The frame specification determines which output rows are read by an aggregate
window function. The ending frame boundary can be omitted if the `BETWEEN`
and `AND` keywords that surround the starting frame boundary are also omitted,
in which case the ending frame boundary defaults to `CURRENT ROW`.

```rust
pub struct WindowFrame {
    pub units: WindowFrameUnits,
    pub start_bound: WindowFrameBound,
    pub end_bound: WindowFrameBound,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `units` | `WindowFrameUnits` | Frame type - either `ROWS`, `RANGE` or `GROUPS` |
| `start_bound` | `WindowFrameBound` | Starting frame boundary |
| `end_bound` | `WindowFrameBound` | Ending frame boundary |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn new(order_by: Option<bool>) -> Self { /* ... */ }
  ```
  Creates a new, default window frame (with the meaning of default

- ```rust
  pub fn reverse(self: &Self) -> Self { /* ... */ }
  ```
  Get reversed window frame. For example

- ```rust
  pub fn is_causal(self: &Self) -> bool { /* ... */ }
  ```
  Get whether window frame is causal

- ```rust
  pub fn new_bounds(units: WindowFrameUnits, start_bound: WindowFrameBound, end_bound: WindowFrameBound) -> Self { /* ... */ }
  ```
  Initializes window frame from units (type), start bound and end bound.

- ```rust
  pub fn regularize_order_bys(self: &Self, order_by: &mut Vec<Sort>) -> Result<()> { /* ... */ }
  ```
  Regularizes the ORDER BY clause of the window frame.

- ```rust
  pub fn can_accept_multi_orderby(self: &Self) -> bool { /* ... */ }
  ```
  Returns whether the window frame can accept multiple ORDER BY expressions.

- ```rust
  pub fn is_ever_expanding(self: &Self) -> bool { /* ... */ }
  ```
  Is the window frame ever-expanding (it always grows in the superset sense).

###### Trait Implementations

- **UnwindSafe**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WindowFrame) -> bool { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowFrame { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WindowFrame) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **StructuralPartialEq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Eq**
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

- **RefUnwindSafe**
- **ErasedDestructor**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

  - ```rust
    fn try_from(value: ast::WindowFrame) -> Result<Self> { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

#### Enum `WindowFrameBound`

There are five ways to describe starting and ending frame boundaries:

1. UNBOUNDED PRECEDING
2. `<expr>` PRECEDING
3. CURRENT ROW
4. `<expr>` FOLLOWING
5. UNBOUNDED FOLLOWING


```rust
pub enum WindowFrameBound {
    Preceding(datafusion_common::ScalarValue),
    CurrentRow,
    Following(datafusion_common::ScalarValue),
}
```

##### Variants

###### `Preceding`

1. UNBOUNDED PRECEDING
   The frame boundary is the first row in the partition.

2. `<expr>` PRECEDING
   `<expr>` must be a non-negative constant numeric expression. The boundary is a row that
   is `<expr>` "units" prior to the current row.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_common::ScalarValue` |  |

###### `CurrentRow`

3. The current row.

For RANGE and GROUPS frame types, peers of the current row are also
included in the frame, unless specifically excluded by the EXCLUDE clause.
This is true regardless of whether CURRENT ROW is used as the starting or ending frame
boundary.

###### `Following`

4. This is the same as "`<expr>` PRECEDING" except that the boundary is `<expr>` units after the
   current rather than before the current row.

5. UNBOUNDED FOLLOWING
   The frame boundary is the last row in the partition.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `datafusion_common::ScalarValue` |  |

##### Implementations

###### Methods

- ```rust
  pub fn is_unbounded(self: &Self) -> bool { /* ... */ }
  ```

###### Trait Implementations

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **Unpin**
- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WindowFrameBound) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **StructuralPartialEq**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Eq**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WindowFrameBound) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **ErasedDestructor**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Freeze**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowFrameBound { /* ... */ }
    ```

#### Enum `WindowFrameUnits`

There are three frame types: ROWS, GROUPS, and RANGE. The frame type determines how the
starting and ending boundaries of the frame are measured.

```rust
pub enum WindowFrameUnits {
    Rows,
    Range,
    Groups,
}
```

##### Variants

###### `Rows`

The ROWS frame type means that the starting and ending boundaries for the frame are
determined by counting individual rows relative to the current row.

###### `Range`

The RANGE frame type requires that the ORDER BY clause of the window have exactly one
term. Call that term "X". With the RANGE frame type, the elements of the frame are
determined by computing the value of expression X for all rows in the partition and framing
those rows for which the value of X is within a certain range of the value of X for the
current row.

###### `Groups`

The GROUPS frame type means that the starting and ending boundaries are determine
by counting "groups" relative to the current group. A "group" is a set of rows that all have
equivalent values for all all terms of the window ORDER BY clause.

##### Implementations

###### Trait Implementations

- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **PartialOrd**
  - ```rust
    fn partial_cmp(self: &Self, other: &WindowFrameUnits) -> $crate::option::Option<$crate::cmp::Ordering> { /* ... */ }
    ```

- **Unpin**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> WindowFrameUnits { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Copy**
- **ErasedDestructor**
- **RefUnwindSafe**
- **IntoEither**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &WindowFrameUnits) -> bool { /* ... */ }
    ```

- **MaybeSendSync**
- **Allocation**
- **Hash**
  - ```rust
    fn hash<__H: $crate::hash::Hasher>(self: &Self, state: &mut __H) { /* ... */ }
    ```

- **Eq**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **DynHash**
  - ```rust
    fn dyn_hash(self: &Self, state: &mut dyn Hasher) { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(value: ast::WindowFrameUnits) -> Self { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
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

- **StructuralPartialEq**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `window_state`

Structures used to hold window function state (for implementing WindowUDFs)

```rust
pub mod window_state { /* ... */ }
```

### Types

#### Struct `WindowAggState`

Holds the state of evaluating a window function

```rust
pub struct WindowAggState {
    pub window_frame_range: std::ops::Range<usize>,
    pub window_frame_ctx: Option<WindowFrameContext>,
    pub last_calculated_index: usize,
    pub offset_pruned_rows: usize,
    pub out_col: arrow::array::ArrayRef,
    pub n_row_result_missing: usize,
    pub is_end: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `window_frame_range` | `std::ops::Range<usize>` | The range that we calculate the window function |
| `window_frame_ctx` | `Option<WindowFrameContext>` |  |
| `last_calculated_index` | `usize` | The index of the last row that its result is calculated inside the partition record batch buffer. |
| `offset_pruned_rows` | `usize` | The offset of the deleted row number |
| `out_col` | `arrow::array::ArrayRef` | Stores the results calculated by window frame |
| `n_row_result_missing` | `usize` | Keeps track of how many rows should be generated to be in sync with input record_batch. |
| `is_end` | `bool` | Flag indicating whether we have received all data for this partition |

##### Implementations

###### Methods

- ```rust
  pub fn prune_state(self: &mut Self, n_prune: usize) { /* ... */ }
  ```

- ```rust
  pub fn update(self: &mut Self, out_col: &ArrayRef, partition_batch_state: &PartitionBatchState) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn new(out_type: &DataType) -> Result<Self> { /* ... */ }
  ```

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **IntoEither**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **UnwindSafe**
- **ErasedDestructor**
#### Enum `WindowFrameContext`

This object stores the window frame state for use in incremental calculations.

```rust
pub enum WindowFrameContext {
    Rows(std::sync::Arc<crate::WindowFrame>),
    Range {
        window_frame: std::sync::Arc<crate::WindowFrame>,
        state: WindowFrameStateRange,
    },
    Groups {
        window_frame: std::sync::Arc<crate::WindowFrame>,
        state: WindowFrameStateGroups,
    },
}
```

##### Variants

###### `Rows`

ROWS frames are inherently stateless.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `std::sync::Arc<crate::WindowFrame>` |  |

###### `Range`

RANGE frames are stateful, they store indices specifying where the
previous search left off. This amortizes the overall cost to O(n)
where n denotes the row count.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `window_frame` | `std::sync::Arc<crate::WindowFrame>` |  |
| `state` | `WindowFrameStateRange` |  |

###### `Groups`

GROUPS frames are stateful, they store group boundaries and indices
specifying where the previous search left off. This amortizes the
overall cost to O(n) where n denotes the row count.

Fields:

| Name | Type | Documentation |
|------|------|---------------|
| `window_frame` | `std::sync::Arc<crate::WindowFrame>` |  |
| `state` | `WindowFrameStateGroups` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(window_frame: Arc<WindowFrame>, sort_options: Vec<SortOptions>) -> Self { /* ... */ }
  ```
  Create a new state object for the given window frame.

- ```rust
  pub fn calculate_range(self: &mut Self, range_columns: &[ArrayRef], last_range: &Range<usize>, length: usize, idx: usize) -> Result<Range<usize>> { /* ... */ }
  ```
  This function calculates beginning/ending indices for the frame of the current row.

###### Trait Implementations

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
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

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **Unpin**
#### Struct `PartitionBatchState`

State for each unique partition determined according to PARTITION BY column(s)

```rust
pub struct PartitionBatchState {
    pub record_batch: arrow::record_batch::RecordBatch,
    pub most_recent_row: Option<arrow::record_batch::RecordBatch>,
    pub is_end: bool,
    pub n_out_row: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `record_batch` | `arrow::record_batch::RecordBatch` | The record batch belonging to current partition |
| `most_recent_row` | `Option<arrow::record_batch::RecordBatch>` | The record batch that contains the most recent row at the input.<br>Please note that this batch doesn't necessarily have the same partitioning<br>with `record_batch`. Keeping track of this batch enables us to prune<br>`record_batch` when cardinality of the partition is sparse. |
| `is_end` | `bool` | Flag indicating whether we have received all data for this partition |
| `n_out_row` | `usize` | Number of rows emitted for each partition |

##### Implementations

###### Methods

- ```rust
  pub fn new(schema: SchemaRef) -> Self { /* ... */ }
  ```

- ```rust
  pub fn extend(self: &mut Self, batch: &RecordBatch) -> Result<()> { /* ... */ }
  ```

- ```rust
  pub fn set_most_recent_row(self: &mut Self, batch: RecordBatch) { /* ... */ }
  ```

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
#### Struct `WindowFrameStateRange`

This structure encapsulates all the state information we require as we scan
ranges of data while processing RANGE frames.
Attribute `sort_options` stores the column ordering specified by the ORDER
BY clause. This information is used to calculate the range.

```rust
pub struct WindowFrameStateRange {
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Unpin**
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

- **Send**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> WindowFrameStateRange { /* ... */ }
    ```

- **Sync**
#### Struct `WindowFrameStateGroups`

This structure encapsulates all the state information we require as we
scan groups of data while processing window frames.

```rust
pub struct WindowFrameStateGroups {
    pub group_end_indices: std::collections::VecDeque<(Vec<datafusion_common::ScalarValue>, usize)>,
    pub current_group_idx: usize,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `group_end_indices` | `std::collections::VecDeque<(Vec<datafusion_common::ScalarValue>, usize)>` | A tuple containing group values and the row index where the group ends.<br>Example: [[1, 1], [1, 1], [2, 1], [2, 1], ...] would correspond to<br>         [([1, 1], 2), ([2, 1], 4), ...]. |
| `current_group_idx` | `usize` | The group index to which the row index belongs. |

##### Implementations

###### Trait Implementations

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
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

- **Default**
  - ```rust
    fn default() -> WindowFrameStateGroups { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Macros

### Macro `expr_vec_fmt`

**Attributes:**

- `#[macro_export]`

```rust
pub macro_rules! expr_vec_fmt {
    /* macro_rules! expr_vec_fmt {
    ( $ARRAY:expr ) => { ... };
} */
}
```

## Re-exports

### Re-export `DocSection`

```rust
pub use datafusion_doc::DocSection;
```

### Re-export `Documentation`

```rust
pub use datafusion_doc::Documentation;
```

### Re-export `DocumentationBuilder`

```rust
pub use datafusion_doc::DocumentationBuilder;
```

### Re-export `Accumulator`

```rust
pub use datafusion_expr_common::accumulator::Accumulator;
```

### Re-export `ColumnarValue`

```rust
pub use datafusion_expr_common::columnar_value::ColumnarValue;
```

### Re-export `EmitTo`

```rust
pub use datafusion_expr_common::groups_accumulator::EmitTo;
```

### Re-export `GroupsAccumulator`

```rust
pub use datafusion_expr_common::groups_accumulator::GroupsAccumulator;
```

### Re-export `Operator`

```rust
pub use datafusion_expr_common::operator::Operator;
```

### Re-export `ArrayFunctionArgument`

```rust
pub use datafusion_expr_common::signature::ArrayFunctionArgument;
```

### Re-export `ArrayFunctionSignature`

```rust
pub use datafusion_expr_common::signature::ArrayFunctionSignature;
```

### Re-export `Coercion`

```rust
pub use datafusion_expr_common::signature::Coercion;
```

### Re-export `Signature`

```rust
pub use datafusion_expr_common::signature::Signature;
```

### Re-export `TypeSignature`

```rust
pub use datafusion_expr_common::signature::TypeSignature;
```

### Re-export `TypeSignatureClass`

```rust
pub use datafusion_expr_common::signature::TypeSignatureClass;
```

### Re-export `Volatility`

```rust
pub use datafusion_expr_common::signature::Volatility;
```

### Re-export `TIMEZONE_WILDCARD`

```rust
pub use datafusion_expr_common::signature::TIMEZONE_WILDCARD;
```

### Re-export `binary`

```rust
pub use datafusion_expr_common::type_coercion::binary;
```

### Re-export `Between`

```rust
pub use expr::Between;
```

### Re-export `BinaryExpr`

```rust
pub use expr::BinaryExpr;
```

### Re-export `Case`

```rust
pub use expr::Case;
```

### Re-export `Cast`

```rust
pub use expr::Cast;
```

### Re-export `Expr`

```rust
pub use expr::Expr;
```

### Re-export `GetFieldAccess`

```rust
pub use expr::GetFieldAccess;
```

### Re-export `GroupingSet`

```rust
pub use expr::GroupingSet;
```

### Re-export `Like`

```rust
pub use expr::Like;
```

### Re-export `Sort`

```rust
pub use expr::Sort as SortExpr;
```

### Re-export `TryCast`

```rust
pub use expr::TryCast;
```

### Re-export `WindowFunctionDefinition`

```rust
pub use expr::WindowFunctionDefinition;
```

### Re-export `ExprSchemable`

```rust
pub use expr_schema::ExprSchemable;
```

### Re-export `AccumulatorFactoryFunction`

```rust
pub use function::AccumulatorFactoryFunction;
```

### Re-export `PartitionEvaluatorFactory`

```rust
pub use function::PartitionEvaluatorFactory;
```

### Re-export `ReturnTypeFunction`

```rust
pub use function::ReturnTypeFunction;
```

### Re-export `ScalarFunctionImplementation`

```rust
pub use function::ScalarFunctionImplementation;
```

### Re-export `StateTypeFunction`

```rust
pub use function::StateTypeFunction;
```

### Re-export `lit`

```rust
pub use literal::lit;
```

### Re-export `lit_timestamp_nano`

```rust
pub use literal::lit_timestamp_nano;
```

### Re-export `Literal`

```rust
pub use literal::Literal;
```

### Re-export `TimestampLiteral`

```rust
pub use literal::TimestampLiteral;
```

### Re-export `PartitionEvaluator`

```rust
pub use partition_evaluator::PartitionEvaluator;
```

### Re-export `sqlparser`

```rust
pub use sqlparser;
```

### Re-export `TableProviderFilterPushDown`

```rust
pub use table_source::TableProviderFilterPushDown;
```

### Re-export `TableSource`

```rust
pub use table_source::TableSource;
```

### Re-export `TableType`

```rust
pub use table_source::TableType;
```

### Re-export `aggregate_doc_sections`

```rust
pub use udaf::aggregate_doc_sections;
```

### Re-export `AggregateUDF`

```rust
pub use udaf::AggregateUDF;
```

### Re-export `AggregateUDFImpl`

```rust
pub use udaf::AggregateUDFImpl;
```

### Re-export `ReversedUDAF`

```rust
pub use udaf::ReversedUDAF;
```

### Re-export `SetMonotonicity`

```rust
pub use udaf::SetMonotonicity;
```

### Re-export `StatisticsArgs`

```rust
pub use udaf::StatisticsArgs;
```

### Re-export `scalar_doc_sections`

```rust
pub use udf::scalar_doc_sections;
```

### Re-export `ReturnInfo`

```rust
pub use udf::ReturnInfo;
```

### Re-export `ReturnTypeArgs`

```rust
pub use udf::ReturnTypeArgs;
```

### Re-export `ScalarFunctionArgs`

```rust
pub use udf::ScalarFunctionArgs;
```

### Re-export `ScalarUDF`

```rust
pub use udf::ScalarUDF;
```

### Re-export `ScalarUDFImpl`

```rust
pub use udf::ScalarUDFImpl;
```

### Re-export `window_doc_sections`

```rust
pub use udwf::window_doc_sections;
```

### Re-export `ReversedUDWF`

```rust
pub use udwf::ReversedUDWF;
```

### Re-export `WindowUDF`

```rust
pub use udwf::WindowUDF;
```

### Re-export `WindowUDFImpl`

```rust
pub use udwf::WindowUDFImpl;
```

### Re-export `WindowFrame`

```rust
pub use window_frame::WindowFrame;
```

### Re-export `WindowFrameBound`

```rust
pub use window_frame::WindowFrameBound;
```

### Re-export `WindowFrameUnits`

```rust
pub use window_frame::WindowFrameUnits;
```

### Re-export `expr_fn::*`

```rust
pub use expr_fn::*;
```

### Re-export `logical_plan::*`

```rust
pub use logical_plan::*;
```


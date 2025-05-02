# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_sql`

This crate provides:

1. A SQL parser, [`DFParser`], that translates SQL query text into
   an abstract syntax tree (AST), [`Statement`].

2. A SQL query planner [`SqlToRel`] that creates [`LogicalPlan`]s
   from [`Statement`]s.

3. A SQL [`unparser`] that converts [`Expr`]s and [`LogicalPlan`]s
   into SQL query text.

[`DFParser`]: parser::DFParser
[`Statement`]: parser::Statement
[`SqlToRel`]: planner::SqlToRel
[`LogicalPlan`]: datafusion_expr::logical_plan::LogicalPlan
[`Expr`]: datafusion_expr::expr::Expr

## Modules

## Module `parser`

[`DFParser`]: DataFusion SQL Parser based on [`sqlparser`]

This parser implements DataFusion specific statements such as
`CREATE EXTERNAL TABLE`

```rust
pub mod parser { /* ... */ }
```

### Types

#### Struct `ExplainStatement`

 DataFusion specific `EXPLAIN`

 Syntax:
 ```sql
 EXPLAIN <ANALYZE> <VERBOSE> [FORMAT format] statement
```

```rust
pub struct ExplainStatement {
    pub analyze: bool,
    pub verbose: bool,
    pub format: Option<String>,
    pub statement: Box<Statement>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `analyze` | `bool` | `EXPLAIN ANALYZE ..` |
| `verbose` | `bool` | `EXPLAIN .. VERBOSE ..` |
| `format` | `Option<String>` | `EXPLAIN .. FORMAT ` |
| `statement` | `Box<Statement>` | The statement to analyze. Note this is a DataFusion [`Statement`] (not a<br>[`sqlparser::ast::Statement`] so that we can use `EXPLAIN`, `COPY`, and other<br>DataFusion specific statements |

##### Implementations

###### Trait Implementations

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **Unpin**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &ExplainStatement) -> bool { /* ... */ }
    ```

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ExplainStatement { /* ... */ }
    ```

- **UnwindSafe**
- **Eq**
#### Struct `CopyToStatement`

DataFusion extension DDL for `COPY`

# Syntax:

```text
COPY <table_name | (<query>)>
TO
<destination_url>
(key_value_list)
```

# Examples

```sql
COPY lineitem  TO 'lineitem'
STORED AS PARQUET (
  partitions 16,
  row_group_limit_rows 100000,
  row_group_limit_bytes 200000
)

COPY (SELECT l_orderkey from lineitem) to 'lineitem.parquet';
```

```rust
pub struct CopyToStatement {
    pub source: CopyToSource,
    pub target: String,
    pub partitioned_by: Vec<String>,
    pub stored_as: Option<String>,
    pub options: Vec<(String, sqlparser::ast::Value)>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `source` | `CopyToSource` | From where the data comes from |
| `target` | `String` | The URL to where the data is heading |
| `partitioned_by` | `Vec<String>` | Partition keys |
| `stored_as` | `Option<String>` | File type (Parquet, NDJSON, CSV etc.) |
| `options` | `Vec<(String, sqlparser::ast::Value)>` | Target specific options |

##### Implementations

###### Trait Implementations

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CopyToStatement) -> bool { /* ... */ }
    ```

- **Allocation**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **IntoEither**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CopyToStatement { /* ... */ }
    ```

- **StructuralPartialEq**
- **ErasedDestructor**
- **Eq**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Enum `CopyToSource`

```rust
pub enum CopyToSource {
    Relation(sqlparser::ast::ObjectName),
    Query(Box<sqlparser::ast::Query>),
}
```

##### Variants

###### `Relation`

`COPY <table> TO ...`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `sqlparser::ast::ObjectName` |  |

###### `Query`

COPY (...query...) TO ...

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<sqlparser::ast::Query>` |  |

##### Implementations

###### Trait Implementations

- **Sync**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CopyToSource { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CopyToSource) -> bool { /* ... */ }
    ```

- **Eq**
- **Unpin**
- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **StructuralPartialEq**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
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

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

#### Struct `CreateExternalTable`

DataFusion extension DDL for `CREATE EXTERNAL TABLE`

Syntax:

```text
CREATE EXTERNAL TABLE
[ IF NOT EXISTS ]
<TABLE_NAME>[ (<column_definition>) ]
STORED AS <file_type>
[ PARTITIONED BY (<column_definition list> | <column list>) ]
[ WITH ORDER (<ordered column list>)
[ OPTIONS (<key_value_list>) ]
LOCATION <literal>

<column_definition> := (<column_name> <data_type>, ...)

<column_list> := (<column_name>, ...)

<ordered_column_list> := (<column_name> <sort_clause>, ...)

<key_value_list> := (<literal> <literal, <literal> <literal>, ...)
```

```rust
pub struct CreateExternalTable {
    pub name: sqlparser::ast::ObjectName,
    pub columns: Vec<sqlparser::ast::ColumnDef>,
    pub file_type: String,
    pub location: String,
    pub table_partition_cols: Vec<String>,
    pub order_exprs: Vec<Vec<sqlparser::ast::OrderByExpr>>,
    pub if_not_exists: bool,
    pub temporary: bool,
    pub unbounded: bool,
    pub options: Vec<(String, sqlparser::ast::Value)>,
    pub constraints: Vec<sqlparser::ast::TableConstraint>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `name` | `sqlparser::ast::ObjectName` | Table name |
| `columns` | `Vec<sqlparser::ast::ColumnDef>` | Optional schema |
| `file_type` | `String` | File type (Parquet, NDJSON, CSV, etc) |
| `location` | `String` | Path to file |
| `table_partition_cols` | `Vec<String>` | Partition Columns |
| `order_exprs` | `Vec<Vec<sqlparser::ast::OrderByExpr>>` | Ordered expressions |
| `if_not_exists` | `bool` | Option to not error if table already exists |
| `temporary` | `bool` | Whether the table is a temporary table |
| `unbounded` | `bool` | Infinite streams? |
| `options` | `Vec<(String, sqlparser::ast::Value)>` | Table(provider) specific options |
| `constraints` | `Vec<sqlparser::ast::TableConstraint>` | A table-level constraint |

##### Implementations

###### Trait Implementations

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CreateExternalTable { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CreateExternalTable) -> bool { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **IntoEither**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Eq**
- **RefUnwindSafe**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **StructuralPartialEq**
- **Sync**
- **Freeze**
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

- **Allocation**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
#### Enum `Statement`

DataFusion SQL Statement.

This can either be a [`Statement`] from [`sqlparser`] from a
standard SQL dialect, or a DataFusion extension such as `CREATE
EXTERNAL TABLE`. See [`DFParser`] for more information.

[`Statement`]: sqlparser::ast::Statement

```rust
pub enum Statement {
    Statement(Box<sqlparser::ast::Statement>),
    CreateExternalTable(CreateExternalTable),
    CopyTo(CopyToStatement),
    Explain(ExplainStatement),
}
```

##### Variants

###### `Statement`

ANSI SQL AST node (from sqlparser-rs)

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `Box<sqlparser::ast::Statement>` |  |

###### `CreateExternalTable`

Extension: `CREATE EXTERNAL TABLE`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CreateExternalTable` |  |

###### `CopyTo`

Extension: `COPY TO`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `CopyToStatement` |  |

###### `Explain`

EXPLAIN for extensions

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `ExplainStatement` |  |

##### Implementations

###### Trait Implementations

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
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

- **RefUnwindSafe**
- **ErasedDestructor**
- **DynEq**
  - ```rust
    fn dyn_eq(self: &Self, other: &dyn Any + ''static) -> bool { /* ... */ }
    ```

- **Unpin**
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Statement { /* ... */ }
    ```

- **StructuralPartialEq**
- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &Statement) -> bool { /* ... */ }
    ```

- **Eq**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
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

- **IntoEither**
- **Freeze**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `DFParser`

DataFusion SQL Parser based on [`sqlparser`]

Parses DataFusion's SQL dialect, often delegating to [`sqlparser`]'s [`Parser`].

DataFusion mostly follows existing SQL dialects via
`sqlparser`. However, certain statements such as `COPY` and
`CREATE EXTERNAL TABLE` have special syntax in DataFusion. See
[`Statement`] for a list of this special syntax

```rust
pub struct DFParser<''a> {
    pub parser: sqlparser::parser::Parser<''a>,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `parser` | `sqlparser::parser::Parser<''a>` |  |

##### Implementations

###### Methods

- ```rust
  pub fn new(sql: &''a str) -> Result<Self, ParserError> { /* ... */ }
  ```

- ```rust
  pub fn new_with_dialect(sql: &''a str, dialect: &''a dyn Dialect) -> Result<Self, ParserError> { /* ... */ }
  ```

- ```rust
  pub fn parse_sql(sql: &''a str) -> Result<VecDeque<Statement>, ParserError> { /* ... */ }
  ```
  Parse a sql string into one or [`Statement`]s using the

- ```rust
  pub fn parse_sql_with_dialect(sql: &str, dialect: &dyn Dialect) -> Result<VecDeque<Statement>, ParserError> { /* ... */ }
  ```
  Parse a SQL string and produce one or more [`Statement`]s with

- ```rust
  pub fn parse_sql_into_expr_with_dialect(sql: &str, dialect: &dyn Dialect) -> Result<ExprWithAlias, ParserError> { /* ... */ }
  ```

- ```rust
  pub fn parse_statements(self: &mut Self) -> Result<VecDeque<Statement>, ParserError> { /* ... */ }
  ```
  Parse a sql string into one or [`Statement`]s

- ```rust
  pub fn parse_statement(self: &mut Self) -> Result<Statement, ParserError> { /* ... */ }
  ```
  Parse a new expression

- ```rust
  pub fn parse_expr(self: &mut Self) -> Result<ExprWithAlias, ParserError> { /* ... */ }
  ```

- ```rust
  pub fn parse_copy(self: &mut Self) -> Result<Statement, ParserError> { /* ... */ }
  ```
  Parse a SQL `COPY TO` statement

- ```rust
  pub fn parse_option_key(self: &mut Self) -> Result<String, ParserError> { /* ... */ }
  ```
  Parse the next token as a key name for an option list

- ```rust
  pub fn parse_option_value(self: &mut Self) -> Result<Value, ParserError> { /* ... */ }
  ```
  Parse the next token as a value for an option list

- ```rust
  pub fn parse_explain(self: &mut Self) -> Result<Statement, ParserError> { /* ... */ }
  ```
  Parse a SQL `EXPLAIN`

- ```rust
  pub fn parse_explain_format(self: &mut Self) -> Result<Option<String>, ParserError> { /* ... */ }
  ```

- ```rust
  pub fn parse_create(self: &mut Self) -> Result<Statement, ParserError> { /* ... */ }
  ```
  Parse a SQL `CREATE` statement handling `CREATE EXTERNAL TABLE`

- ```rust
  pub fn parse_order_by_exprs(self: &mut Self) -> Result<Vec<OrderByExpr>, ParserError> { /* ... */ }
  ```
  Parse the ordering clause of a `CREATE EXTERNAL TABLE` SQL statement

- ```rust
  pub fn parse_order_by_expr(self: &mut Self) -> Result<OrderByExpr, ParserError> { /* ... */ }
  ```
  Parse an ORDER BY sub-expression optionally followed by ASC or DESC.

###### Trait Implementations

- **IntoEither**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

#### Struct `DFParserBuilder`

Builder for [`DFParser`]

# Example: Create and Parse SQL statements
```
# use datafusion_sql::parser::DFParserBuilder;
# use datafusion_common::Result;
# fn test() -> Result<()> {
let mut parser = DFParserBuilder::new("SELECT * FROM foo; SELECT 1 + 2")
  .build()?;
// parse the SQL into DFStatements
let statements = parser.parse_statements()?;
assert_eq!(statements.len(), 2);
# Ok(())
# }
```

# Example: Create and Parse expression with a different dialect
```
# use datafusion_sql::parser::DFParserBuilder;
# use datafusion_common::Result;
# use datafusion_sql::sqlparser::dialect::MySqlDialect;
# use datafusion_sql::sqlparser::ast::Expr;
# fn test() -> Result<()> {
let dialect = MySqlDialect{}; // Parse using MySQL dialect
let mut parser = DFParserBuilder::new("1 + 2")
  .with_dialect(&dialect)
  .build()?;
// parse 1+2 into an sqlparser::ast::Expr
let res = parser.parse_expr()?;
assert!(matches!(res.expr, Expr::BinaryOp {..}));
# Ok(())
# }
```

```rust
pub struct DFParserBuilder<''a> {
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
  pub fn new(sql: &''a str) -> Self { /* ... */ }
  ```
  Create a new parser builder for the specified tokens using the

- ```rust
  pub fn with_dialect(self: Self, dialect: &''a dyn Dialect) -> Self { /* ... */ }
  ```
  Adjust the parser builder's dialect. Defaults to [`GenericDialect`]

- ```rust
  pub fn with_recursion_limit(self: Self, recursion_limit: usize) -> Self { /* ... */ }
  ```
  Adjust the recursion limit of sql parsing.  Defaults to 50

- ```rust
  pub fn build(self: Self) -> Result<DFParser<''a>, ParserError> { /* ... */ }
  ```

###### Trait Implementations

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
- **Send**
- **MaybeSendSync**
- **Unpin**
- **Sync**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **ErasedDestructor**
## Module `planner`

[`SqlToRel`]: SQL Query Planner (produces [`LogicalPlan`] from SQL AST)

```rust
pub mod planner { /* ... */ }
```

### Types

#### Struct `ParserOptions`

SQL parser options

```rust
pub struct ParserOptions {
    pub parse_float_as_decimal: bool,
    pub enable_ident_normalization: bool,
    pub support_varchar_with_length: bool,
    pub enable_options_value_normalization: bool,
    pub collect_spans: bool,
    pub map_varchar_to_utf8view: bool,
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `parse_float_as_decimal` | `bool` | Whether to parse float as decimal. |
| `enable_ident_normalization` | `bool` | Whether to normalize identifiers. |
| `support_varchar_with_length` | `bool` | Whether to support varchar with length. |
| `enable_options_value_normalization` | `bool` | Whether to normalize options value. |
| `collect_spans` | `bool` | Whether to collect spans |
| `map_varchar_to_utf8view` | `bool` | Whether `VARCHAR` is mapped to `Utf8View` during SQL planning. |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```
  Creates a new `ParserOptions` instance with default values.

- ```rust
  pub fn with_parse_float_as_decimal(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Sets the `parse_float_as_decimal` option.

- ```rust
  pub fn with_enable_ident_normalization(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Sets the `enable_ident_normalization` option.

- ```rust
  pub fn with_support_varchar_with_length(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Sets the `support_varchar_with_length` option.

- ```rust
  pub fn with_map_varchar_to_utf8view(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Sets the `map_varchar_to_utf8view` option.

- ```rust
  pub fn with_enable_options_value_normalization(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Sets the `enable_options_value_normalization` option.

- ```rust
  pub fn with_collect_spans(self: Self, value: bool) -> Self { /* ... */ }
  ```
  Sets the `collect_spans` option.

###### Trait Implementations

- **ErasedDestructor**
- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Allocation**
- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **Clone**
  - ```rust
    fn clone(self: &Self) -> ParserOptions { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(options: &SqlParserOptions) -> Self { /* ... */ }
    ```

#### Struct `IdentNormalizer`

Ident Normalizer

```rust
pub struct IdentNormalizer {
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
  pub fn new(normalize: bool) -> Self { /* ... */ }
  ```

- ```rust
  pub fn normalize(self: &Self, ident: Ident) -> String { /* ... */ }
  ```

###### Trait Implementations

- **IntoEither**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Allocation**
- **RefUnwindSafe**
#### Struct `PlannerContext`

Struct to store the states used by the Planner. The Planner will leverage the states
to resolve CTEs, Views, subqueries and PREPARE statements. The states include
Common Table Expression (CTE) provided with WITH clause and
Parameter Data Types provided with PREPARE statement and the query schema of the
outer query plan.

# Cloning

Only the `ctes` are truly cloned when the `PlannerContext` is cloned.
This helps resolve scoping issues of CTEs.
By using cloning, a subquery can inherit CTEs from the outer query
and can also define its own private CTEs without affecting the outer query.


```rust
pub struct PlannerContext {
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
  Create an empty PlannerContext

- ```rust
  pub fn with_prepare_param_data_types(self: Self, prepare_param_data_types: Vec<DataType>) -> Self { /* ... */ }
  ```
  Update the PlannerContext with provided prepare_param_data_types

- ```rust
  pub fn outer_query_schema(self: &Self) -> Option<&DFSchema> { /* ... */ }
  ```

- ```rust
  pub fn set_outer_query_schema(self: &mut Self, schema: Option<DFSchemaRef>) -> Option<DFSchemaRef> { /* ... */ }
  ```
  Sets the outer query schema, returning the existing one, if

- ```rust
  pub fn set_table_schema(self: &mut Self, schema: Option<DFSchemaRef>) -> Option<DFSchemaRef> { /* ... */ }
  ```

- ```rust
  pub fn table_schema(self: &Self) -> Option<DFSchemaRef> { /* ... */ }
  ```

- ```rust
  pub fn outer_from_schema(self: &Self) -> Option<Arc<DFSchema>> { /* ... */ }
  ```

- ```rust
  pub fn set_outer_from_schema(self: &mut Self, schema: Option<DFSchemaRef>) -> Option<DFSchemaRef> { /* ... */ }
  ```
  Sets the outer FROM schema, returning the existing one, if any

- ```rust
  pub fn extend_outer_from_schema(self: &mut Self, schema: &DFSchemaRef) -> Result<()> { /* ... */ }
  ```
  Extends the FROM schema, returning the existing one, if any

- ```rust
  pub fn prepare_param_data_types(self: &Self) -> &[DataType] { /* ... */ }
  ```
  Return the types of parameters (`$1`, `$2`, etc) if known

- ```rust
  pub fn contains_cte(self: &Self, cte_name: &str) -> bool { /* ... */ }
  ```
  Returns true if there is a Common Table Expression (CTE) /

- ```rust
  pub fn insert_cte</* synthetic */ impl Into<String>: Into<String>>(self: &mut Self, cte_name: impl Into<String>, plan: LogicalPlan) { /* ... */ }
  ```
  Inserts a LogicalPlan for the Common Table Expression (CTE) /

- ```rust
  pub fn get_cte(self: &Self, cte_name: &str) -> Option<&LogicalPlan> { /* ... */ }
  ```
  Return a plan for the Common Table Expression (CTE) / Subquery for the

###### Trait Implementations

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

- **Send**
- **UnwindSafe**
- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> PlannerContext { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

#### Struct `SqlToRel`

SQL query planner and binder

This struct is used to convert a SQL AST into a [`LogicalPlan`].

You can control the behavior of the planner by providing [`ParserOptions`].

It performs the following tasks:

1. Name and type resolution (called "binding" in other systems). This
   phase looks up table and column names using the [`ContextProvider`].
2. Mechanical translation of the AST into a [`LogicalPlan`].

It does not perform type coercion, or perform optimization, which are done
by subsequent passes.

Key interfaces are:
* [`Self::sql_statement_to_plan`]: Convert a statement
  (e.g. `SELECT ...`) into a [`LogicalPlan`]
* [`Self::sql_to_expr`]: Convert an expression (e.g. `1 + 2`) into an [`Expr`]

```rust
pub struct SqlToRel<''a, S: ContextProvider> {
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
  pub fn sql_to_expr_with_alias(self: &Self, sql: SQLExprWithAlias, schema: &DFSchema, planner_context: &mut PlannerContext) -> Result<Expr> { /* ... */ }
  ```

- ```rust
  pub fn sql_to_expr(self: &Self, sql: SQLExpr, schema: &DFSchema, planner_context: &mut PlannerContext) -> Result<Expr> { /* ... */ }
  ```
  Generate a relational expression from a SQL expression

- ```rust
  pub fn new(context_provider: &''a S) -> Self { /* ... */ }
  ```
  Create a new query planner.

- ```rust
  pub fn new_with_options(context_provider: &''a S, options: ParserOptions) -> Self { /* ... */ }
  ```
  Create a new query planner with the given parser options.

- ```rust
  pub fn build_schema(self: &Self, columns: Vec<SQLColumnDef>) -> Result<Schema> { /* ... */ }
  ```

- ```rust
  pub fn statement_to_plan(self: &Self, statement: DFStatement) -> Result<LogicalPlan> { /* ... */ }
  ```
  Generate a logical plan from an DataFusion SQL statement

- ```rust
  pub fn sql_statement_to_plan(self: &Self, statement: Statement) -> Result<LogicalPlan> { /* ... */ }
  ```
  Generate a logical plan from an SQL statement

- ```rust
  pub fn sql_statement_to_plan_with_context(self: &Self, statement: Statement, planner_context: &mut PlannerContext) -> Result<LogicalPlan> { /* ... */ }
  ```
  Generate a logical plan from an SQL statement

- ```rust
  pub fn new_constraint_from_table_constraints(self: &Self, constraints: &[TableConstraint], df_schema: &DFSchemaRef) -> Result<Constraints> { /* ... */ }
  ```
  Convert each [TableConstraint] to corresponding [Constraint]

###### Trait Implementations

- **UnwindSafe**
- **RefUnwindSafe**
- **Allocation**
- **IntoEither**
- **Freeze**
- **Unpin**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
### Functions

#### Function `object_name_to_table_reference`

Create a [`TableReference`] after normalizing the specified ObjectName

Examples
```text
['foo']          -> Bare { table: "foo" }
['"foo.bar"]]    -> Bare { table: "foo.bar" }
['foo', 'Bar']   -> Partial { schema: "foo", table: "bar" } <-- note lower case "bar"
['foo', 'bar']   -> Partial { schema: "foo", table: "bar" }
['foo', '"Bar"'] -> Partial { schema: "foo", table: "Bar" }
```

```rust
pub fn object_name_to_table_reference(object_name: sqlparser::ast::ObjectName, enable_normalization: bool) -> datafusion_common::Result<datafusion_common::TableReference> { /* ... */ }
```

#### Function `object_name_to_qualifier`

Construct a WHERE qualifier suitable for e.g. information_schema filtering
from the provided object identifiers (catalog, schema and table names).

```rust
pub fn object_name_to_qualifier(sql_table_name: &sqlparser::ast::ObjectName, enable_normalization: bool) -> datafusion_common::Result<String> { /* ... */ }
```

### Re-exports

#### Re-export `ContextProvider`

```rust
pub use datafusion_expr::planner::ContextProvider;
```

## Module `resolve`

```rust
pub mod resolve { /* ... */ }
```

### Functions

#### Function `resolve_table_references`

Collects all tables and views referenced in the SQL statement. CTEs are collected separately.
This can be used to determine which tables need to be in the catalog for a query to be planned.

# Returns

A `(table_refs, ctes)` tuple, the first element contains table and view references and the second
element contains any CTE aliases that were defined and possibly referenced.

## Example

```
# use datafusion_sql::parser::DFParser;
# use datafusion_sql::resolve::resolve_table_references;
let query = "SELECT a FROM foo where x IN (SELECT y FROM bar)";
let statement = DFParser::parse_sql(query).unwrap().pop_back().unwrap();
let (table_refs, ctes) = resolve_table_references(&statement, true).unwrap();
assert_eq!(table_refs.len(), 2);
assert_eq!(table_refs[0].to_string(), "bar");
assert_eq!(table_refs[1].to_string(), "foo");
assert_eq!(ctes.len(), 0);
```

## Example with CTEs  
  
```  
# use datafusion_sql::parser::DFParser;  
# use datafusion_sql::resolve::resolve_table_references;
let query = "with my_cte as (values (1), (2)) SELECT * from my_cte;";  
let statement = DFParser::parse_sql(query).unwrap().pop_back().unwrap();  
let (table_refs, ctes) = resolve_table_references(&statement, true).unwrap();  
assert_eq!(table_refs.len(), 0);
assert_eq!(ctes.len(), 1);  
assert_eq!(ctes[0].to_string(), "my_cte");  
```

```rust
pub fn resolve_table_references(statement: &crate::parser::Statement, enable_ident_normalization: bool) -> datafusion_common::Result<(Vec<crate::TableReference>, Vec<crate::TableReference>)> { /* ... */ }
```

## Module `unparser`

**Attributes:**

- `#[cfg(feature = "unparser")]`

[`Unparser`] for converting `Expr` to SQL text

```rust
pub mod unparser { /* ... */ }
```

### Modules

## Module `ast`

```rust
pub mod ast { /* ... */ }
```

### Types

#### Struct `QueryBuilder`

```rust
pub struct QueryBuilder {
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
  pub fn with(self: &mut Self, value: Option<ast::With>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn body(self: &mut Self, value: Box<ast::SetExpr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn take_body(self: &mut Self) -> Option<Box<ast::SetExpr>> { /* ... */ }
  ```

- ```rust
  pub fn order_by(self: &mut Self, value: OrderByKind) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn limit(self: &mut Self, value: Option<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn limit_by(self: &mut Self, value: Vec<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn offset(self: &mut Self, value: Option<ast::Offset>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn fetch(self: &mut Self, value: Option<ast::Fetch>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn locks(self: &mut Self, value: Vec<ast::LockClause>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn for_clause(self: &mut Self, value: Option<ast::ForClause>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: &Self) -> Result<ast::Query, BuilderError> { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Allocation**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
    fn clone(self: &Self) -> QueryBuilder { /* ... */ }
    ```

- **Sync**
#### Struct `SelectBuilder`

```rust
pub struct SelectBuilder {
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
  pub fn distinct(self: &mut Self, value: Option<ast::Distinct>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn top(self: &mut Self, value: Option<ast::Top>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn projection(self: &mut Self, value: Vec<ast::SelectItem>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn pop_projections(self: &mut Self) -> Vec<ast::SelectItem> { /* ... */ }
  ```

- ```rust
  pub fn already_projected(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn into(self: &mut Self, value: Option<ast::SelectInto>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn from(self: &mut Self, value: Vec<TableWithJoinsBuilder>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn push_from(self: &mut Self, value: TableWithJoinsBuilder) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn pop_from(self: &mut Self) -> Option<TableWithJoinsBuilder> { /* ... */ }
  ```

- ```rust
  pub fn lateral_views(self: &mut Self, value: Vec<ast::LateralView>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn replace_mark(self: &mut Self, existing_expr: &ast::Expr, value: &ast::Expr) -> &mut Self { /* ... */ }
  ```
  Replaces the selection with a new value.

- ```rust
  pub fn selection(self: &mut Self, value: Option<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn group_by(self: &mut Self, value: ast::GroupByExpr) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn cluster_by(self: &mut Self, value: Vec<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn distribute_by(self: &mut Self, value: Vec<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn sort_by(self: &mut Self, value: Vec<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn having(self: &mut Self, value: Option<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn named_window(self: &mut Self, value: Vec<ast::NamedWindowDefinition>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn qualify(self: &mut Self, value: Option<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn value_table_mode(self: &mut Self, value: Option<ast::ValueTableMode>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: &Self) -> Result<ast::Select, BuilderError> { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **Allocation**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> SelectBuilder { /* ... */ }
    ```

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

#### Struct `TableWithJoinsBuilder`

```rust
pub struct TableWithJoinsBuilder {
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
  pub fn relation(self: &mut Self, value: RelationBuilder) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn joins(self: &mut Self, value: Vec<ast::Join>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn push_join(self: &mut Self, value: ast::Join) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: &Self) -> Result<Option<ast::TableWithJoins>, BuilderError> { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Sync**
- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TableWithJoinsBuilder { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

#### Struct `RelationBuilder`

```rust
pub struct RelationBuilder {
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
  pub fn has_relation(self: &Self) -> bool { /* ... */ }
  ```

- ```rust
  pub fn table(self: &mut Self, value: TableRelationBuilder) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn derived(self: &mut Self, value: DerivedRelationBuilder) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn unnest(self: &mut Self, value: UnnestRelationBuilder) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn empty(self: &mut Self) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn alias(self: &mut Self, value: Option<ast::TableAlias>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: &Self) -> Result<Option<ast::TableFactor>, BuilderError> { /* ... */ }
  ```

###### Trait Implementations

- **MaybeSendSync**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> RelationBuilder { /* ... */ }
    ```

- **Allocation**
- **ErasedDestructor**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

#### Struct `TableRelationBuilder`

```rust
pub struct TableRelationBuilder {
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
  pub fn name(self: &mut Self, value: ast::ObjectName) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn alias(self: &mut Self, value: Option<ast::TableAlias>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn args(self: &mut Self, value: Option<Vec<ast::FunctionArg>>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn with_hints(self: &mut Self, value: Vec<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn version(self: &mut Self, value: Option<ast::TableVersion>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn partitions(self: &mut Self, value: Vec<ast::Ident>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn index_hints(self: &mut Self, value: Vec<ast::TableIndexHints>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: &Self) -> Result<ast::TableFactor, BuilderError> { /* ... */ }
  ```

###### Trait Implementations

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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
- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> TableRelationBuilder { /* ... */ }
    ```

#### Struct `DerivedRelationBuilder`

```rust
pub struct DerivedRelationBuilder {
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
  pub fn lateral(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn subquery(self: &mut Self, value: Box<ast::Query>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn alias(self: &mut Self, value: Option<ast::TableAlias>) -> &mut Self { /* ... */ }
  ```

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

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Clone**
  - ```rust
    fn clone(self: &Self) -> DerivedRelationBuilder { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **Allocation**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
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

- **RefUnwindSafe**
#### Struct `UnnestRelationBuilder`

```rust
pub struct UnnestRelationBuilder {
    pub alias: Option<ast::TableAlias>,
    pub array_exprs: Vec<ast::Expr>,
    // Some fields omitted
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `alias` | `Option<ast::TableAlias>` |  |
| `array_exprs` | `Vec<ast::Expr>` |  |
| *private fields* | ... | *Some fields have been omitted* |

##### Implementations

###### Methods

- ```rust
  pub fn alias(self: &mut Self, value: Option<ast::TableAlias>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn array_exprs(self: &mut Self, value: Vec<ast::Expr>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn with_offset(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn with_offset_alias(self: &mut Self, value: Option<ast::Ident>) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn with_ordinality(self: &mut Self, value: bool) -> &mut Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: &Self) -> Result<ast::TableFactor, BuilderError> { /* ... */ }
  ```

###### Trait Implementations

- **RefUnwindSafe**
- **MaybeSendSync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> UnnestRelationBuilder { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Allocation**
- **IntoEither**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
#### Struct `UninitializedFieldError`

Runtime error when a `build()` method is called and one or more required fields
do not have a value.

```rust
pub struct UninitializedFieldError(/* private field */);
```

##### Fields

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `private` | *Private field* |

##### Implementations

###### Methods

- ```rust
  pub fn new(field_name: &''static str) -> Self { /* ... */ }
  ```
  Create a new `UninitializedFieldError` for the specified field name.

- ```rust
  pub fn field_name(self: &Self) -> &''static str { /* ... */ }
  ```
  Get the name of the first-declared field that wasn't initialized

###### Trait Implementations

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Allocation**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Error**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> UninitializedFieldError { /* ... */ }
    ```

- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(field_name: &''static str) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: UninitializedFieldError) -> Self { /* ... */ }
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

- **Sync**
#### Enum `BuilderError`

```rust
pub enum BuilderError {
    UninitializedField(&''static str),
    ValidationError(String),
}
```

##### Variants

###### `UninitializedField`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''static str` |  |

###### `ValidationError`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `String` |  |

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **ErasedDestructor**
- **RefUnwindSafe**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

  - ```rust
    fn from(s: UninitializedFieldError) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(s: String) -> Self { /* ... */ }
    ```

  - ```rust
    fn from(e: BuilderError) -> Self { /* ... */ }
    ```

- **Error**
- **UnwindSafe**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `dialect`

```rust
pub mod dialect { /* ... */ }
```

### Types

#### Type Alias `ScalarFnToSqlHandler`

```rust
pub type ScalarFnToSqlHandler = Box<dyn Fn(&super::Unparser<''_>, &[datafusion_expr::Expr]) -> datafusion_common::Result<Option<ast::Expr>> + Send + Sync>;
```

#### Enum `IntervalStyle`

`IntervalStyle` to use for unparsing

<https://www.postgresql.org/docs/current/datatype-datetime.html#DATATYPE-INTERVAL-INPUT>
different DBMS follows different standards, popular ones are:
postgres_verbose: '2 years 15 months 100 weeks 99 hours 123456789 milliseconds' which is
compatible with arrow display format, as well as duckdb
sql standard format is '1-2' for year-month, or '1 10:10:10.123456' for day-time
<https://www.contrib.andrew.cmu.edu/~shadow/sql/sql1992.txt>

```rust
pub enum IntervalStyle {
    PostgresVerbose,
    SQLStandard,
    MySQL,
}
```

##### Variants

###### `PostgresVerbose`

###### `SQLStandard`

###### `MySQL`

##### Implementations

###### Trait Implementations

- **IntoEither**
- **UnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Allocation**
- **RefUnwindSafe**
- **Sync**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> IntervalStyle { /* ... */ }
    ```

- **MaybeSendSync**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Copy**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

#### Enum `DateFieldExtractStyle`

Datetime subfield extraction style for unparsing

`<https://www.postgresql.org/docs/current/functions-datetime.html#FUNCTIONS-DATETIME-EXTRACT>`
Different DBMSs follow different standards; popular ones are:
date_part('YEAR', date '2001-02-16')
EXTRACT(YEAR from date '2001-02-16')
Some DBMSs, like Postgres, support both, whereas others like MySQL require EXTRACT.

```rust
pub enum DateFieldExtractStyle {
    DatePart,
    Extract,
    Strftime,
}
```

##### Variants

###### `DatePart`

###### `Extract`

###### `Strftime`

##### Implementations

###### Trait Implementations

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DateFieldExtractStyle { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DateFieldExtractStyle) -> bool { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Sync**
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

- **Unpin**
- **MaybeSendSync**
- **StructuralPartialEq**
- **Allocation**
- **Copy**
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
- **RefUnwindSafe**
#### Enum `CharacterLengthStyle`

`CharacterLengthStyle` to use for unparsing

Different DBMSs uses different names for function calculating the number of characters in the string
`Length` style uses length(x)
`SQLStandard` style uses character_length(x)

```rust
pub enum CharacterLengthStyle {
    Length,
    CharacterLength,
}
```

##### Variants

###### `Length`

###### `CharacterLength`

##### Implementations

###### Trait Implementations

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
- **StructuralPartialEq**
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
- **Send**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Allocation**
- **ErasedDestructor**
- **Copy**
- **UnwindSafe**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Clone**
  - ```rust
    fn clone(self: &Self) -> CharacterLengthStyle { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &CharacterLengthStyle) -> bool { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `DefaultDialect`

```rust
pub struct DefaultDialect {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Dialect**
  - ```rust
    fn identifier_quote_style(self: &Self, identifier: &str) -> Option<char> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
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

- **Allocation**
#### Struct `PostgreSqlDialect`

```rust
pub struct PostgreSqlDialect {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **RefUnwindSafe**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Dialect**
  - ```rust
    fn identifier_quote_style(self: &Self, _: &str) -> Option<char> { /* ... */ }
    ```

  - ```rust
    fn interval_style(self: &Self) -> IntervalStyle { /* ... */ }
    ```

  - ```rust
    fn float64_ast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn scalar_function_to_sql_overrides(self: &Self, unparser: &Unparser<''_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **IntoEither**
- **Allocation**
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

#### Struct `DuckDBDialect`

```rust
pub struct DuckDBDialect {
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

- **Unpin**
- **Default**
  - ```rust
    fn default() -> DuckDBDialect { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **RefUnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Dialect**
  - ```rust
    fn identifier_quote_style(self: &Self, _: &str) -> Option<char> { /* ... */ }
    ```

  - ```rust
    fn character_length_style(self: &Self) -> CharacterLengthStyle { /* ... */ }
    ```

  - ```rust
    fn division_operator(self: &Self) -> BinaryOperator { /* ... */ }
    ```

  - ```rust
    fn with_custom_scalar_overrides(self: Self, handlers: Vec<(&str, ScalarFnToSqlHandler)>) -> Self { /* ... */ }
    ```

  - ```rust
    fn scalar_function_to_sql_overrides(self: &Self, unparser: &Unparser<''_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>> { /* ... */ }
    ```

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

#### Struct `MySqlDialect`

```rust
pub struct MySqlDialect {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **UnwindSafe**
- **Sync**
- **IntoEither**
- **Dialect**
  - ```rust
    fn identifier_quote_style(self: &Self, _: &str) -> Option<char> { /* ... */ }
    ```

  - ```rust
    fn supports_nulls_first_in_sort(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn interval_style(self: &Self) -> IntervalStyle { /* ... */ }
    ```

  - ```rust
    fn utf8_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn large_utf8_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn date_field_extract_style(self: &Self) -> DateFieldExtractStyle { /* ... */ }
    ```

  - ```rust
    fn int64_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn int32_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn timestamp_cast_dtype(self: &Self, _time_unit: &TimeUnit, _tz: &Option<Arc<str>>) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn requires_derived_table_alias(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn scalar_function_to_sql_overrides(self: &Self, unparser: &Unparser<''_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>> { /* ... */ }
    ```

- **ErasedDestructor**
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

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

#### Struct `SqliteDialect`

```rust
pub struct SqliteDialect {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Dialect**
  - ```rust
    fn identifier_quote_style(self: &Self, _: &str) -> Option<char> { /* ... */ }
    ```

  - ```rust
    fn date_field_extract_style(self: &Self) -> DateFieldExtractStyle { /* ... */ }
    ```

  - ```rust
    fn date32_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn character_length_style(self: &Self) -> CharacterLengthStyle { /* ... */ }
    ```

  - ```rust
    fn supports_column_alias_in_table_alias(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn scalar_function_to_sql_overrides(self: &Self, unparser: &Unparser<''_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **IntoEither**
- **Unpin**
#### Struct `CustomDialect`

```rust
pub struct CustomDialect {
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
  pub fn new(identifier_quote_style: Option<char>) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Allocation**
- **MaybeSendSync**
- **Dialect**
  - ```rust
    fn identifier_quote_style(self: &Self, _: &str) -> Option<char> { /* ... */ }
    ```

  - ```rust
    fn supports_nulls_first_in_sort(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn use_timestamp_for_date64(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn interval_style(self: &Self) -> IntervalStyle { /* ... */ }
    ```

  - ```rust
    fn float64_ast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn utf8_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn large_utf8_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn date_field_extract_style(self: &Self) -> DateFieldExtractStyle { /* ... */ }
    ```

  - ```rust
    fn character_length_style(self: &Self) -> CharacterLengthStyle { /* ... */ }
    ```

  - ```rust
    fn int64_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn int32_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn timestamp_cast_dtype(self: &Self, _time_unit: &TimeUnit, tz: &Option<Arc<str>>) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn date32_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
    ```

  - ```rust
    fn supports_column_alias_in_table_alias(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn scalar_function_to_sql_overrides(self: &Self, unparser: &Unparser<''_>, func_name: &str, args: &[Expr]) -> Result<Option<ast::Expr>> { /* ... */ }
    ```

  - ```rust
    fn requires_derived_table_alias(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn division_operator(self: &Self) -> BinaryOperator { /* ... */ }
    ```

  - ```rust
    fn window_func_support_window_frame(self: &Self, _func_name: &str, _start_bound: &WindowFrameBound, _end_bound: &WindowFrameBound) -> bool { /* ... */ }
    ```

  - ```rust
    fn full_qualified_col(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn unnest_as_table_factor(self: &Self) -> bool { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **ErasedDestructor**
- **Send**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Unpin**
#### Struct `CustomDialectBuilder`

`CustomDialectBuilder` to build `CustomDialect` using builder pattern


# Examples

Building a custom dialect with all default options set in CustomDialectBuilder::new()
but with `use_timestamp_for_date64` overridden to `true`

```
use datafusion_sql::unparser::dialect::CustomDialectBuilder;
let dialect = CustomDialectBuilder::new()
    .with_use_timestamp_for_date64(true)
    .build();
```

```rust
pub struct CustomDialectBuilder {
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
  pub fn build(self: Self) -> CustomDialect { /* ... */ }
  ```

- ```rust
  pub fn with_identifier_quote_style(self: Self, identifier_quote_style: char) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific identifier quote style, e.g. '`', '"'

- ```rust
  pub fn with_supports_nulls_first_in_sort(self: Self, supports_nulls_first_in_sort: bool) -> Self { /* ... */ }
  ```
  Customize the dialect to support `NULLS FIRST` in `ORDER BY` clauses

- ```rust
  pub fn with_use_timestamp_for_date64(self: Self, use_timestamp_for_date64: bool) -> Self { /* ... */ }
  ```
  Customize the dialect to uses TIMESTAMP when casting Date64 rather than DATETIME

- ```rust
  pub fn with_interval_style(self: Self, interval_style: IntervalStyle) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific interval style listed in `IntervalStyle`

- ```rust
  pub fn with_character_length_style(self: Self, character_length_style: CharacterLengthStyle) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific character_length_style listed in `CharacterLengthStyle`

- ```rust
  pub fn with_float64_ast_dtype(self: Self, float64_ast_dtype: ast::DataType) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific SQL type for Float64 casting: DOUBLE, DOUBLE PRECISION, etc.

- ```rust
  pub fn with_utf8_cast_dtype(self: Self, utf8_cast_dtype: ast::DataType) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific SQL type for Utf8 casting: VARCHAR, CHAR, etc.

- ```rust
  pub fn with_large_utf8_cast_dtype(self: Self, large_utf8_cast_dtype: ast::DataType) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific SQL type for LargeUtf8 casting: TEXT, CHAR, etc.

- ```rust
  pub fn with_date_field_extract_style(self: Self, date_field_extract_style: DateFieldExtractStyle) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific date field extract style listed in `DateFieldExtractStyle`

- ```rust
  pub fn with_int64_cast_dtype(self: Self, int64_cast_dtype: ast::DataType) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific SQL type for Int64 casting: BigInt, SIGNED, etc.

- ```rust
  pub fn with_int32_cast_dtype(self: Self, int32_cast_dtype: ast::DataType) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific SQL type for Int32 casting: Integer, SIGNED, etc.

- ```rust
  pub fn with_timestamp_cast_dtype(self: Self, timestamp_cast_dtype: ast::DataType, timestamp_tz_cast_dtype: ast::DataType) -> Self { /* ... */ }
  ```
  Customize the dialect with a specific SQL type for Timestamp casting: Timestamp, Datetime, etc.

- ```rust
  pub fn with_date32_cast_dtype(self: Self, date32_cast_dtype: ast::DataType) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_supports_column_alias_in_table_alias(self: Self, supports_column_alias_in_table_alias: bool) -> Self { /* ... */ }
  ```
  Customize the dialect to support column aliases as part of alias table definition

- ```rust
  pub fn with_requires_derived_table_alias(self: Self, requires_derived_table_alias: bool) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_division_operator(self: Self, division_operator: BinaryOperator) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_window_func_support_window_frame(self: Self, window_func_support_window_frame: bool) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_full_qualified_col(self: Self, full_qualified_col: bool) -> Self { /* ... */ }
  ```
  Customize the dialect to allow full qualified column names

- ```rust
  pub fn with_unnest_as_table_factor(self: Self, unnest_as_table_factor: bool) -> Self { /* ... */ }
  ```

###### Trait Implementations

- **Send**
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
- **UnwindSafe**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **ErasedDestructor**
- **RefUnwindSafe**
- **Sync**
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

- **Allocation**
### Traits

#### Trait `Dialect`

`Dialect` to use for Unparsing

The default dialect tries to avoid quoting identifiers unless necessary (e.g. `a` instead of `"a"`)
but this behavior can be overridden as needed

**Note**: This trait will eventually be replaced by the Dialect in the SQLparser package

See <https://github.com/sqlparser-rs/sqlparser-rs/pull/1170>
See also the discussion in <https://github.com/apache/datafusion/pull/10625>

```rust
pub trait Dialect: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `identifier_quote_style`: Return the character used to quote identifiers.

##### Provided Methods

- ```rust
  fn supports_nulls_first_in_sort(self: &Self) -> bool { /* ... */ }
  ```
  Does the dialect support specifying `NULLS FIRST/LAST` in `ORDER BY` clauses?

- ```rust
  fn use_timestamp_for_date64(self: &Self) -> bool { /* ... */ }
  ```
  Does the dialect use TIMESTAMP to represent Date64 rather than DATETIME?

- ```rust
  fn interval_style(self: &Self) -> IntervalStyle { /* ... */ }
  ```

- ```rust
  fn float64_ast_dtype(self: &Self) -> ast::DataType { /* ... */ }
  ```
  Does the dialect use DOUBLE PRECISION to represent Float64 rather than DOUBLE?

- ```rust
  fn utf8_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
  ```
  The SQL type to use for Arrow Utf8 unparsing

- ```rust
  fn large_utf8_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
  ```
  The SQL type to use for Arrow LargeUtf8 unparsing

- ```rust
  fn date_field_extract_style(self: &Self) -> DateFieldExtractStyle { /* ... */ }
  ```
  The date field extract style to use: `DateFieldExtractStyle`

- ```rust
  fn character_length_style(self: &Self) -> CharacterLengthStyle { /* ... */ }
  ```
  The character length extraction style to use: `CharacterLengthStyle`

- ```rust
  fn int64_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
  ```
  The SQL type to use for Arrow Int64 unparsing

- ```rust
  fn int32_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
  ```
  The SQL type to use for Arrow Int32 unparsing

- ```rust
  fn timestamp_cast_dtype(self: &Self, _time_unit: &TimeUnit, tz: &Option<Arc<str>>) -> ast::DataType { /* ... */ }
  ```
  The SQL type to use for Timestamp unparsing

- ```rust
  fn date32_cast_dtype(self: &Self) -> ast::DataType { /* ... */ }
  ```
  The SQL type to use for Arrow Date32 unparsing

- ```rust
  fn supports_column_alias_in_table_alias(self: &Self) -> bool { /* ... */ }
  ```
  Does the dialect support specifying column aliases as part of alias table definition?

- ```rust
  fn requires_derived_table_alias(self: &Self) -> bool { /* ... */ }
  ```
  Whether the dialect requires a table alias for any subquery in the FROM clause

- ```rust
  fn division_operator(self: &Self) -> BinaryOperator { /* ... */ }
  ```
  The division operator for the dialect

- ```rust
  fn scalar_function_to_sql_overrides(self: &Self, _unparser: &Unparser<''_>, _func_name: &str, _args: &[Expr]) -> Result<Option<ast::Expr>> { /* ... */ }
  ```
  Allows the dialect to override scalar function unparsing if the dialect has specific rules.

- ```rust
  fn window_func_support_window_frame(self: &Self, _func_name: &str, _start_bound: &WindowFrameBound, _end_bound: &WindowFrameBound) -> bool { /* ... */ }
  ```
  Allows the dialect to choose to omit window frame in unparsing

- ```rust
  fn with_custom_scalar_overrides(self: Self, _handlers: Vec<(&str, ScalarFnToSqlHandler)>) -> Self
where
    Self: Sized { /* ... */ }
  ```
  Extends the dialect's default rules for unparsing scalar functions.

- ```rust
  fn full_qualified_col(self: &Self) -> bool { /* ... */ }
  ```
  Allow to unparse a qualified column with a full qualified name

- ```rust
  fn unnest_as_table_factor(self: &Self) -> bool { /* ... */ }
  ```
  Allow to unparse the unnest plan as [ast::TableFactor::UNNEST].

##### Implementations

This trait is implemented for the following types:

- `DefaultDialect`
- `PostgreSqlDialect`
- `DuckDBDialect`
- `MySqlDialect`
- `SqliteDialect`
- `CustomDialect`

## Module `extension_unparser`

```rust
pub mod extension_unparser { /* ... */ }
```

### Types

#### Enum `UnparseWithinStatementResult`

The result of unparsing a custom logical node within a statement.

```rust
pub enum UnparseWithinStatementResult {
    Modified,
    Unmodified,
}
```

##### Variants

###### `Modified`

If the custom logical node was successfully unparsed within a statement.

###### `Unmodified`

If the custom logical node wasn't unparsed.

##### Implementations

###### Trait Implementations

- **MaybeSendSync**
- **IntoEither**
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

- **Allocation**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **RefUnwindSafe**
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

#### Enum `UnparseToStatementResult`

The result of unparsing a custom logical node to a statement.

```rust
pub enum UnparseToStatementResult {
    Modified(sqlparser::ast::Statement),
    Unmodified,
}
```

##### Variants

###### `Modified`

If the custom logical node was successfully unparsed to a statement.

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `sqlparser::ast::Statement` |  |

###### `Unmodified`

If the custom logical node wasn't unparsed.

##### Implementations

###### Trait Implementations

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **MaybeSendSync**
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

- **Send**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
### Traits

#### Trait `UserDefinedLogicalNodeUnparser`

This trait allows users to define custom unparser logic for their custom logical nodes.

```rust
pub trait UserDefinedLogicalNodeUnparser {
    /* Associated items */
}
```

##### Provided Methods

- ```rust
  fn unparse(self: &Self, _node: &dyn UserDefinedLogicalNode, _unparser: &Unparser<''_>, _query: &mut Option<&mut QueryBuilder>, _select: &mut Option<&mut SelectBuilder>, _relation: &mut Option<&mut RelationBuilder>) -> datafusion_common::Result<UnparseWithinStatementResult> { /* ... */ }
  ```
  Unparse the custom logical node to SQL within a statement.

- ```rust
  fn unparse_to_statement(self: &Self, _node: &dyn UserDefinedLogicalNode, _unparser: &Unparser<''_>) -> datafusion_common::Result<UnparseToStatementResult> { /* ... */ }
  ```
  Unparse the custom logical node to a statement.

### Types

#### Struct `Unparser`

Convert a DataFusion [`Expr`] to [`sqlparser::ast::Expr`]

See [`expr_to_sql`] for background. `Unparser` allows greater control of
the conversion, but with a more complicated API.

To get more human-readable output, see [`Self::with_pretty`]

# Example
```
use datafusion_expr::{col, lit};
use datafusion_sql::unparser::Unparser;
let expr = col("a").gt(lit(4)); // form an expression `a > 4`
let unparser = Unparser::default();
let sql = unparser.expr_to_sql(&expr).unwrap();// convert to AST
// use the Display impl to convert to SQL text
assert_eq!(sql.to_string(), "(a > 4)");
// now convert to pretty sql
let unparser = unparser.with_pretty(true);
let sql = unparser.expr_to_sql(&expr).unwrap();
assert_eq!(sql.to_string(), "a > 4"); // note lack of parenthesis
```

[`Expr`]: datafusion_expr::Expr

```rust
pub struct Unparser<''a> {
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
  pub fn expr_to_sql(self: &Self, expr: &Expr) -> Result<ast::Expr> { /* ... */ }
  ```

- ```rust
  pub fn scalar_function_to_sql(self: &Self, func_name: &str, args: &[Expr]) -> Result<ast::Expr> { /* ... */ }
  ```

- ```rust
  pub fn sort_to_sql(self: &Self, sort: &Sort) -> Result<ast::OrderByExpr> { /* ... */ }
  ```

- ```rust
  pub fn col_to_sql(self: &Self, col: &Column) -> Result<ast::Expr> { /* ... */ }
  ```

- ```rust
  pub fn plan_to_sql(self: &Self, plan: &LogicalPlan) -> Result<ast::Statement> { /* ... */ }
  ```

- ```rust
  pub fn new(dialect: &''a dyn Dialect) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_pretty(self: Self, pretty: bool) -> Self { /* ... */ }
  ```
  Create pretty SQL output, better suited for human consumption

- ```rust
  pub fn with_extension_unparsers(self: Self, extension_unparsers: Vec<Arc<dyn UserDefinedLogicalNodeUnparser>>) -> Self { /* ... */ }
  ```
  Add a custom unparser for user defined logical nodes

###### Trait Implementations

- **MaybeSendSync**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
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
- **ErasedDestructor**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Re-exports

#### Re-export `expr_to_sql`

```rust
pub use expr::expr_to_sql;
```

#### Re-export `plan_to_sql`

```rust
pub use plan::plan_to_sql;
```

## Module `utils`

SQL Utility Functions

```rust
pub mod utils { /* ... */ }
```

### Functions

#### Function `window_expr_common_partition_keys`

Given a slice of window expressions sharing the same sort key, find their common partition
keys.

```rust
pub fn window_expr_common_partition_keys(window_exprs: &[datafusion_expr::Expr]) -> datafusion_common::Result<&[datafusion_expr::Expr]> { /* ... */ }
```

### Constants and Statics

#### Constant `UNNEST_PLACEHOLDER`

```rust
pub const UNNEST_PLACEHOLDER: &str = "__unnest_placeholder";
```

## Re-exports

### Re-export `ResolvedTableReference`

**Attributes:**

- `#[deprecated(since = "46.0.0", note =
"use datafusion_common::{ResolvedTableReference, TableReference}")]`

**⚠️ Deprecated since 46.0.0**: use datafusion_common::{ResolvedTableReference, TableReference}

```rust
pub use datafusion_common::ResolvedTableReference;
```

### Re-export `TableReference`

**Attributes:**

- `#[deprecated(since = "46.0.0", note =
"use datafusion_common::{ResolvedTableReference, TableReference}")]`

**⚠️ Deprecated since 46.0.0**: use datafusion_common::{ResolvedTableReference, TableReference}

```rust
pub use datafusion_common::TableReference;
```

### Re-export `sqlparser`

```rust
pub use sqlparser;
```


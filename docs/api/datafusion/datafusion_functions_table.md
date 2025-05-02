# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_functions_table`

## Modules

## Module `generate_series`

```rust
pub mod generate_series { /* ... */ }
```

### Types

#### Struct `GenerateSeriesFunc`

```rust
pub struct GenerateSeriesFunc {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Allocation**
- **Unpin**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
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

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TableFunctionImpl**
  - ```rust
    fn call(self: &Self, exprs: &[Expr]) -> Result<Arc<dyn TableProvider>> { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
#### Struct `RangeFunc`

```rust
pub struct RangeFunc {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **TableFunctionImpl**
  - ```rust
    fn call(self: &Self, exprs: &[Expr]) -> Result<Arc<dyn TableProvider>> { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Freeze**
- **Send**
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

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Allocation**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Functions

### Function `all_default_table_functions`

Returns all default table functions

```rust
pub fn all_default_table_functions() -> Vec<std::sync::Arc<datafusion_catalog::TableFunction>> { /* ... */ }
```

### Function `generate_series`

```rust
pub fn generate_series() -> Arc<TableFunction> { /* ... */ }
```

### Function `range`

```rust
pub fn range() -> Arc<TableFunction> { /* ... */ }
```

## Macros

### Macro `create_udtf_function`

**Attributes:**

- `#[macro_export]`

Creates a singleton instance of a table function
- `$module`: A struct implementing `TableFunctionImpl` to create the function from
- `$name`: The name to give to the created function

This is used to ensure creating the list of `TableFunction` only happens once.

```rust
pub macro_rules! create_udtf_function {
    /* macro_rules! create_udtf_function {
    ($module:path, $name:expr) => { ... };
} */
}
```


# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_functions_window_common`

Common user-defined window functionality for [DataFusion]

[DataFusion]: <https://crates.io/crates/datafusion>

## Modules

## Module `expr`

```rust
pub mod expr { /* ... */ }
```

### Types

#### Struct `ExpressionArgs`

Arguments passed to user-defined window function

```rust
pub struct ExpressionArgs<''a> {
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
  pub fn new(input_exprs: &''a [Arc<dyn PhysicalExpr>], input_types: &''a [DataType]) -> Self { /* ... */ }
  ```
  Create an instance of [`ExpressionArgs`].

- ```rust
  pub fn input_exprs(self: &Self) -> &''a [Arc<dyn PhysicalExpr>] { /* ... */ }
  ```
  Returns the expressions passed as arguments to the user-defined

- ```rust
  pub fn input_types(self: &Self) -> &''a [DataType] { /* ... */ }
  ```
  Returns the [`DataType`]s corresponding to the input expressions

###### Trait Implementations

- **Unpin**
- **Default**
  - ```rust
    fn default() -> ExpressionArgs<''a> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **IntoEither**
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

- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
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

- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `field`

```rust
pub mod field { /* ... */ }
```

### Types

#### Struct `WindowUDFFieldArgs`

Metadata for defining the result field from evaluating a
user-defined window function.

```rust
pub struct WindowUDFFieldArgs<''a> {
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
  pub fn new(input_types: &''a [DataType], display_name: &''a str) -> Self { /* ... */ }
  ```
  Create an instance of [`WindowUDFFieldArgs`].

- ```rust
  pub fn input_types(self: &Self) -> &[DataType] { /* ... */ }
  ```
  Returns the data type of input expressions passed as arguments

- ```rust
  pub fn name(self: &Self) -> &str { /* ... */ }
  ```
  Returns the name for the field of the final result of evaluating

- ```rust
  pub fn get_input_type(self: &Self, index: usize) -> Option<DataType> { /* ... */ }
  ```
  Returns `Some(DataType)` of input expression at index, otherwise

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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Allocation**
- **ErasedDestructor**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **UnwindSafe**
- **IntoEither**
## Module `partition`

```rust
pub mod partition { /* ... */ }
```

### Types

#### Struct `PartitionEvaluatorArgs`

Arguments passed to created user-defined window function state
during physical execution.

```rust
pub struct PartitionEvaluatorArgs<''a> {
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
  pub fn new(input_exprs: &''a [Arc<dyn PhysicalExpr>], input_types: &''a [DataType], is_reversed: bool, ignore_nulls: bool) -> Self { /* ... */ }
  ```
  Create an instance of [`PartitionEvaluatorArgs`].

- ```rust
  pub fn input_exprs(self: &Self) -> &''a [Arc<dyn PhysicalExpr>] { /* ... */ }
  ```
  Returns the expressions passed as arguments to the user-defined

- ```rust
  pub fn input_types(self: &Self) -> &''a [DataType] { /* ... */ }
  ```
  Returns the [`DataType`]s corresponding to the input expressions

- ```rust
  pub fn is_reversed(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` when the user-defined window function is

- ```rust
  pub fn ignore_nulls(self: &Self) -> bool { /* ... */ }
  ```
  Returns `true` when `IGNORE NULLS` is specified, otherwise

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **IntoEither**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> PartitionEvaluatorArgs<''a> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Sync**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**

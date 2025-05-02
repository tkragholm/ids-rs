# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_functions_nested`

Nested type Functions for [DataFusion].

This crate contains a collection of nested type functions implemented using the
extension API.

[DataFusion]: https://crates.io/crates/datafusion

You can register the functions in this crate using the [`register_all`] function.


## Modules

## Module `macros`

**Attributes:**

- `#[macro_use]`

```rust
pub mod macros { /* ... */ }
```

## Module `array_has`

[`ScalarUDFImpl`] definitions for array_has, array_has_all and array_has_any functions.

```rust
pub mod array_has { /* ... */ }
```

### Types

#### Struct `ArrayHas`

```rust
pub struct ArrayHas {
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
    fn return_type(self: &Self, _: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn simplify(self: &Self, args: Vec<Expr>, _info: &dyn datafusion_expr::simplify::SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
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
- **UnwindSafe**
- **Sync**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **IntoEither**
- **Same**
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
#### Struct `ArrayHasAll`

```rust
pub struct ArrayHasAll {
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **UnwindSafe**
- **Unpin**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
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
    fn return_type(self: &Self, _: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **ErasedDestructor**
- **RefUnwindSafe**
- **Same**
- **Freeze**
- **IntoEither**
#### Struct `ArrayHasAny`

```rust
pub struct ArrayHasAny {
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

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **UnwindSafe**
- **RefUnwindSafe**
- **Sync**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
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
    fn return_type(self: &Self, _: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

### Functions

#### Function `array_has`

returns true, if the element appears in the first array, otherwise false.

```rust
pub fn array_has(haystack_array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_has_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayHas

```rust
pub fn array_has_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_has_all`

returns true if each element of the second array appears in the first array; otherwise, it returns false.

```rust
pub fn array_has_all(haystack_array: datafusion_expr::Expr, needle_array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_has_all_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayHasAll

```rust
pub fn array_has_all_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_has_any`

returns true if at least one element of the second array appears in the first array; otherwise, it returns false.

```rust
pub fn array_has_any(haystack_array: datafusion_expr::Expr, needle_array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_has_any_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayHasAny

```rust
pub fn array_has_any_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `cardinality`

[`ScalarUDFImpl`] definitions for cardinality function.

```rust
pub mod cardinality { /* ... */ }
```

### Types

#### Struct `Cardinality`

```rust
pub struct Cardinality {
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Same**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **MaybeSendSync**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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

- **RefUnwindSafe**
### Functions

#### Function `cardinality`

returns the total number of elements in the array or map.

```rust
pub fn cardinality(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `cardinality_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
Cardinality

```rust
pub fn cardinality_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `cardinality_inner`

Cardinality SQL function

```rust
pub fn cardinality_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `concat`

[`ScalarUDFImpl`] definitions for `array_append`, `array_prepend` and `array_concat` functions.

```rust
pub mod concat { /* ... */ }
```

### Types

#### Struct `ArrayAppend`

```rust
pub struct ArrayAppend {
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

- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Sync**
- **ErasedDestructor**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

#### Struct `ArrayPrepend`

```rust
pub struct ArrayPrepend {
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
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

- **UnwindSafe**
- **Freeze**
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

- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **Same**
- **Sync**
#### Struct `ArrayConcat`

```rust
pub struct ArrayConcat {
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
- **RefUnwindSafe**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `array_append`

appends an element to the end of an array.

```rust
pub fn array_append(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_append_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayAppend

```rust
pub fn array_append_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_prepend`

Prepends an element to the beginning of an array.

```rust
pub fn array_prepend(element: datafusion_expr::Expr, array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_prepend_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPrepend

```rust
pub fn array_prepend_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_concat`

Concatenates arrays.

```rust
pub fn array_concat(arg: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_concat_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayConcat

```rust
pub fn array_concat_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `dimension`

[`ScalarUDFImpl`] definitions for array_dims and array_ndims functions.

```rust
pub mod dimension { /* ... */ }
```

### Types

#### Struct `ArrayDims`

```rust
pub struct ArrayDims {
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

- **Same**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

### Functions

#### Function `array_dims`

returns an array of the array's dimensions.

```rust
pub fn array_dims(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_dims_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayDims

```rust
pub fn array_dims_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_ndims`

returns the number of dimensions of the array.

```rust
pub fn array_ndims(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_ndims_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayNdims

```rust
pub fn array_ndims_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_dims_inner`

Array_dims SQL function

```rust
pub fn array_dims_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `array_ndims_inner`

Array_ndims SQL function

```rust
pub fn array_ndims_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `distance`

[ScalarUDFImpl] definitions for array_distance function.

```rust
pub mod distance { /* ... */ }
```

### Types

#### Struct `ArrayDistance`

```rust
pub struct ArrayDistance {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Send**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **IntoEither**
### Functions

#### Function `array_distance`

returns the Euclidean distance between two numeric arrays.

```rust
pub fn array_distance(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_distance_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayDistance

```rust
pub fn array_distance_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_distance_inner`

```rust
pub fn array_distance_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `empty`

[`ScalarUDFImpl`] definitions for array_empty function.

```rust
pub mod empty { /* ... */ }
```

### Types

#### Struct `ArrayEmpty`

```rust
pub struct ArrayEmpty {
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **Same**
### Functions

#### Function `array_empty`

returns true for an empty array or false for a non-empty array.

```rust
pub fn array_empty(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_empty_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayEmpty

```rust
pub fn array_empty_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_empty_inner`

Array_empty SQL function

```rust
pub fn array_empty_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `except`

[`ScalarUDFImpl`] definitions for array_except function.

```rust
pub mod except { /* ... */ }
```

### Types

#### Struct `ArrayExcept`

```rust
pub struct ArrayExcept {
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
- **Freeze**
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

- **MaybeSendSync**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **Sync**
- **Same**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **UnwindSafe**
### Functions

#### Function `array_except`

returns an array of the elements that appear in the first array but not in the second.

```rust
pub fn array_except(first_array: datafusion_expr::Expr, second_array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_except_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayExcept

```rust
pub fn array_except_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_except_inner`

Array_except SQL function

```rust
pub fn array_except_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `expr_ext`

Extension methods for Expr.

```rust
pub mod expr_ext { /* ... */ }
```

### Traits

#### Trait `IndexAccessor`

Return access to the element field. Example `expr["name"]`

## Example Access element 2 from column "c1"

For example if column "c1" holds documents like this

```json
[10, 20, 30, 40]
```

You can access the value "30" with

```
# use datafusion_expr::{lit, col, Expr};
# use datafusion_functions_nested::expr_ext::IndexAccessor;
let expr = col("c1")
   .index(lit(3));
assert_eq!(expr.schema_name().to_string(), "c1[Int32(3)]");
```

```rust
pub trait IndexAccessor {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `index`

##### Implementations

This trait is implemented for the following types:

- `datafusion_expr::Expr`

#### Trait `SliceAccessor`

Return elements between `1` based `start` and `stop`, for
example `expr[1:3]`

## Example: Access element 2, 3, 4 from column "c1"

For example if column "c1" holds documents like this

```json
[10, 20, 30, 40]
```

You can access the value `[20, 30, 40]` with

```
# use datafusion_expr::{lit, col};
# use datafusion_functions_nested::expr_ext::SliceAccessor;
let expr = col("c1")
   .range(lit(2), lit(4));
assert_eq!(expr.schema_name().to_string(), "c1[Int32(2):Int32(4)]");
```

```rust
pub trait SliceAccessor {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `range`

##### Implementations

This trait is implemented for the following types:

- `datafusion_expr::Expr`

## Module `extract`

[`ScalarUDFImpl`] definitions for array_element, array_slice, array_pop_front, array_pop_back, and array_any_value functions.

```rust
pub mod extract { /* ... */ }
```

### Types

#### Struct `ArrayElement`

```rust
pub struct ArrayElement {
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Send**
- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **ScalarUDFImpl**
  - ```rust
    fn as_any(self: &Self) -> &dyn Any { /* ... */ }
    ```

  - ```rust
    fn name(self: &Self) -> &str { /* ... */ }
    ```

  - ```rust
    fn display_name(self: &Self, args: &[Expr]) -> Result<String> { /* ... */ }
    ```

  - ```rust
    fn schema_name(self: &Self, args: &[Expr]) -> Result<String> { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `array_element`

extracts the element with the index n from the array.

```rust
pub fn array_element(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_element_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayElement

```rust
pub fn array_element_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_slice_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArraySlice

```rust
pub fn array_slice_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_pop_front`

returns the array without the first element.

```rust
pub fn array_pop_front(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_pop_front_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPopFront

```rust
pub fn array_pop_front_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_pop_back`

returns the array without the last element.

```rust
pub fn array_pop_back(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_pop_back_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPopBack

```rust
pub fn array_pop_back_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_any_value`

returns the first non-null element in the array.

```rust
pub fn array_any_value(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_any_value_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayAnyValue

```rust
pub fn array_any_value_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_slice`

returns a slice of the array.

```rust
pub fn array_slice(array: datafusion_expr::Expr, begin: datafusion_expr::Expr, end: datafusion_expr::Expr, stride: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

## Module `flatten`

[`ScalarUDFImpl`] definitions for flatten function.

```rust
pub mod flatten { /* ... */ }
```

### Types

#### Struct `Flatten`

```rust
pub struct Flatten {
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **RefUnwindSafe**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Same**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

### Functions

#### Function `flatten`

flattens an array of arrays into a single array.

```rust
pub fn flatten(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `flatten_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
Flatten

```rust
pub fn flatten_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `flatten_inner`

Flatten SQL function

```rust
pub fn flatten_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `length`

[`ScalarUDFImpl`] definitions for array_length function.

```rust
pub mod length { /* ... */ }
```

### Types

#### Struct `ArrayLength`

```rust
pub struct ArrayLength {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
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

- **MaybeSendSync**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Unpin**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **RefUnwindSafe**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Functions

#### Function `array_length`

returns the length of the array dimension.

```rust
pub fn array_length(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_length_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayLength

```rust
pub fn array_length_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_length_inner`

Array_length SQL function

```rust
pub fn array_length_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `make_array`

[`ScalarUDFImpl`] definitions for `make_array` function.

```rust
pub mod make_array { /* ... */ }
```

### Types

#### Struct `MakeArray`

```rust
pub struct MakeArray {
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **Send**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Same**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
### Functions

#### Function `make_array`

Returns an Arrow array using the specified input expressions.

```rust
pub fn make_array(arg: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `make_array_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MakeArray

```rust
pub fn make_array_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `map`

```rust
pub mod map { /* ... */ }
```

### Types

#### Struct `MapFunc`

```rust
pub struct MapFunc {
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

- **Sync**
- **IntoEither**
- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

### Functions

#### Function `map`

Returns a map created from a key list and a value list

```rust
pub fn map(keys: Vec<datafusion_expr::Expr>, values: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `map_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapFunc

```rust
pub fn map_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `map_extract`

[`ScalarUDFImpl`] definitions for map_extract functions.

```rust
pub mod map_extract { /* ... */ }
```

### Types

#### Struct `MapExtract`

```rust
pub struct MapExtract {
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

- **ErasedDestructor**
- **Same**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Send**
- **UnwindSafe**
- **RefUnwindSafe**
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

### Functions

#### Function `map_extract`

Return a list containing the value for a given key or an empty list if the key is not contained in the map.

```rust
pub fn map_extract(map: datafusion_expr::Expr, key: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `map_extract_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapExtract

```rust
pub fn map_extract_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `map_keys`

[`ScalarUDFImpl`] definitions for map_keys function.

```rust
pub mod map_keys { /* ... */ }
```

### Types

#### Struct `MapKeysFunc`

```rust
pub struct MapKeysFunc {
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
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
- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Sync**
- **IntoEither**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Functions

#### Function `map_keys`

Return a list of all keys in the map.

```rust
pub fn map_keys(map: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `map_keys_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapKeysFunc

```rust
pub fn map_keys_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `map_values`

[`ScalarUDFImpl`] definitions for map_values function.

```rust
pub mod map_values { /* ... */ }
```

### Functions

#### Function `map_values`

Return a list of all values in the map.

```rust
pub fn map_values(map: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `map_values_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
MapValuesFunc

```rust
pub fn map_values_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `max`

[`ScalarUDFImpl`] definitions for array_max function.

```rust
pub mod max { /* ... */ }
```

### Types

#### Struct `ArrayMax`

```rust
pub struct ArrayMax {
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

- **Same**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> datafusion_common::Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> datafusion_common::Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **UnwindSafe**
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

- **ErasedDestructor**
- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **MaybeSendSync**
- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `array_max`

returns the maximum value in the array.

```rust
pub fn array_max(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_max_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayMax

```rust
pub fn array_max_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_max_inner`

array_max SQL function

There is one argument for array_max as the array.
`array_max(array)`

For example:
> array_max(\[1, 3, 2]) -> 3

```rust
pub fn array_max_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `planner`

SQL planning extensions like [`NestedFunctionPlanner`] and [`FieldAccessPlanner`]

```rust
pub mod planner { /* ... */ }
```

### Types

#### Struct `NestedFunctionPlanner`

```rust
pub struct NestedFunctionPlanner;
```

##### Implementations

###### Trait Implementations

- **IntoEither**
- **Freeze**
- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ExprPlanner**
  - ```rust
    fn plan_binary_op(self: &Self, expr: RawBinaryExpr, schema: &DFSchema) -> Result<PlannerResult<RawBinaryExpr>> { /* ... */ }
    ```

  - ```rust
    fn plan_array_literal(self: &Self, exprs: Vec<Expr>, _schema: &DFSchema) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
    ```

  - ```rust
    fn plan_make_map(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
    ```

  - ```rust
    fn plan_any(self: &Self, expr: RawBinaryExpr) -> Result<PlannerResult<RawBinaryExpr>> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Same**
- **UnwindSafe**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Unpin**
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

- **MaybeSendSync**
#### Struct `FieldAccessPlanner`

```rust
pub struct FieldAccessPlanner;
```

##### Implementations

###### Trait Implementations

- **Unpin**
- **UnwindSafe**
- **RefUnwindSafe**
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

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Allocation**
- **Send**
- **ErasedDestructor**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ExprPlanner**
  - ```rust
    fn plan_field_access(self: &Self, expr: RawFieldAccessExpr, schema: &DFSchema) -> Result<PlannerResult<RawFieldAccessExpr>> { /* ... */ }
    ```

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

- **Sync**
- **Same**
## Module `position`

[`ScalarUDFImpl`] definitions for array_position and array_positions functions.

```rust
pub mod position { /* ... */ }
```

### Types

#### Struct `ArrayPosition`

```rust
pub struct ArrayPosition {
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
- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
- **IntoEither**
- **Sync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `array_position`

searches for an element in the array, returns first occurrence.

```rust
pub fn array_position(array: datafusion_expr::Expr, element: datafusion_expr::Expr, index: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_position_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPosition

```rust
pub fn array_position_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_position_inner`

Array_position SQL function

```rust
pub fn array_position_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `array_positions`

searches for an element in the array, returns all occurrences.

```rust
pub fn array_positions(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_positions_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayPositions

```rust
pub fn array_positions_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_positions_inner`

Array_positions SQL function

```rust
pub fn array_positions_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `range`

[`ScalarUDFImpl`] definitions for range and gen_series functions.

```rust
pub mod range { /* ... */ }
```

### Types

#### Struct `Range`

```rust
pub struct Range {
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

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **IntoEither**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **Freeze**
- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

### Functions

#### Function `range`

create a list of values in the range between start and stop

```rust
pub fn range(start: datafusion_expr::Expr, stop: datafusion_expr::Expr, step: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `range_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
Range

```rust
pub fn range_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `gen_series`

create a list of values in the range between start and stop, include upper bound

```rust
pub fn gen_series(start: datafusion_expr::Expr, stop: datafusion_expr::Expr, step: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `gen_series_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
GenSeries

```rust
pub fn gen_series_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `remove`

[`ScalarUDFImpl`] definitions for array_remove, array_remove_n, array_remove_all functions.

```rust
pub mod remove { /* ... */ }
```

### Types

#### Struct `ArrayRemove`

```rust
pub struct ArrayRemove {
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Same**
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
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

- **Freeze**
### Functions

#### Function `array_remove`

removes the first element from the array equal to the given value.

```rust
pub fn array_remove(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_remove_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayRemove

```rust
pub fn array_remove_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_remove_n`

removes the first `max` elements from the array equal to the given value.

```rust
pub fn array_remove_n(array: datafusion_expr::Expr, element: datafusion_expr::Expr, max: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_remove_n_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayRemoveN

```rust
pub fn array_remove_n_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_remove_all`

removes all elements from the array equal to the given value.

```rust
pub fn array_remove_all(array: datafusion_expr::Expr, element: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_remove_all_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayRemoveAll

```rust
pub fn array_remove_all_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_remove_inner`

Array_remove SQL function

```rust
pub fn array_remove_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `array_remove_n_inner`

Array_remove_n SQL function

```rust
pub fn array_remove_n_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `array_remove_all_inner`

Array_remove_all SQL function

```rust
pub fn array_remove_all_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `repeat`

[`ScalarUDFImpl`] definitions for array_repeat function.

```rust
pub mod repeat { /* ... */ }
```

### Types

#### Struct `ArrayRepeat`

```rust
pub struct ArrayRepeat {
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

- **Send**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Same**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Unpin**
- **ErasedDestructor**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Functions

#### Function `array_repeat`

returns an array containing element `count` times.

```rust
pub fn array_repeat(element: datafusion_expr::Expr, count: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_repeat_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayRepeat

```rust
pub fn array_repeat_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_repeat_inner`

Array_repeat SQL function

```rust
pub fn array_repeat_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `replace`

[`ScalarUDFImpl`] definitions for array_replace, array_replace_n and array_replace_all functions.

```rust
pub mod replace { /* ... */ }
```

### Types

#### Struct `ArrayReplace`

```rust
pub struct ArrayReplace {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

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
    fn return_type(self: &Self, args: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Send**
- **IntoEither**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Functions

#### Function `array_replace`

replaces the first occurrence of the specified element with another specified element.

```rust
pub fn array_replace(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_replace_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayReplace

```rust
pub fn array_replace_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_replace_n`

replaces the first `max` occurrences of the specified element with another specified element.

```rust
pub fn array_replace_n(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr, max: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_replace_n_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayReplaceN

```rust
pub fn array_replace_n_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_replace_all`

replaces all occurrences of the specified element with another specified element.

```rust
pub fn array_replace_all(array: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_replace_all_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayReplaceAll

```rust
pub fn array_replace_all_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `resize`

[`ScalarUDFImpl`] definitions for array_resize function.

```rust
pub mod resize { /* ... */ }
```

### Types

#### Struct `ArrayResize`

```rust
pub struct ArrayResize {
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
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
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

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
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
- **IntoEither**
- **UnwindSafe**
- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `array_resize`

returns an array with the specified size filled with the given value.

```rust
pub fn array_resize(array: datafusion_expr::Expr, size: datafusion_expr::Expr, value: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_resize_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayResize

```rust
pub fn array_resize_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `reverse`

[`ScalarUDFImpl`] definitions for array_reverse function.

```rust
pub mod reverse { /* ... */ }
```

### Types

#### Struct `ArrayReverse`

```rust
pub struct ArrayReverse {
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

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Send**
- **Same**
### Functions

#### Function `array_reverse`

reverses the order of elements in the array.

```rust
pub fn array_reverse(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_reverse_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayReverse

```rust
pub fn array_reverse_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_reverse_inner`

array_reverse SQL function

```rust
pub fn array_reverse_inner(arg: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `set_ops`

[`ScalarUDFImpl`] definitions for array_union, array_intersect and array_distinct functions.

```rust
pub mod set_ops { /* ... */ }
```

### Types

#### Struct `ArrayUnion`

```rust
pub struct ArrayUnion {
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
- **Unpin**
- **Sync**
- **ErasedDestructor**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Same**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
### Functions

#### Function `array_union`

returns an array of the elements in the union of array1 and array2 without duplicates.

```rust
pub fn array_union(array1: datafusion_expr::Expr, array2: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_union_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayUnion

```rust
pub fn array_union_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_intersect`

returns an array of the elements in the intersection of array1 and array2.

```rust
pub fn array_intersect(first_array: datafusion_expr::Expr, second_array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_intersect_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayIntersect

```rust
pub fn array_intersect_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_distinct`

returns distinct values from the array after removing duplicates.

```rust
pub fn array_distinct(array: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_distinct_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayDistinct

```rust
pub fn array_distinct_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `sort`

[`ScalarUDFImpl`] definitions for array_sort function.

```rust
pub mod sort { /* ... */ }
```

### Types

#### Struct `ArraySort`

Implementation of `array_sort` function

`array_sort` sorts the elements of an array

# Example

`array_sort([3, 1, 2])` returns `[1, 2, 3]`

```rust
pub struct ArraySort {
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **IntoEither**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
### Functions

#### Function `array_sort`

returns sorted array.

```rust
pub fn array_sort(array: datafusion_expr::Expr, desc: datafusion_expr::Expr, null_first: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_sort_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArraySort

```rust
pub fn array_sort_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `array_sort_inner`

Array_sort SQL function

```rust
pub fn array_sort_inner(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `string`

[`ScalarUDFImpl`] definitions for array_to_string and string_to_array functions.

```rust
pub mod string { /* ... */ }
```

### Types

#### Struct `ArrayToString`

```rust
pub struct ArrayToString {
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

- **Sync**
- **Same**
- **MaybeSendSync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

### Functions

#### Function `array_to_string`

converts each element to its text representation.

```rust
pub fn array_to_string(array: datafusion_expr::Expr, delimiter: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `array_to_string_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
ArrayToString

```rust
pub fn array_to_string_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `string_to_array`

splits a `string` based on a `delimiter` and returns an array of parts. Any parts matching the optional `null_string` will be replaced with `NULL`

```rust
pub fn string_to_array(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, null_string: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `string_to_array_udf`

ScalarFunction that returns a [`ScalarUDF`](datafusion_expr::ScalarUDF) for 
StringToArray

```rust
pub fn string_to_array_udf() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

## Module `utils`

array function utils

```rust
pub mod utils { /* ... */ }
```

## Module `expr_fn`

Fluent-style API for creating `Expr`s

```rust
pub mod expr_fn { /* ... */ }
```

### Re-exports

#### Re-export `array_has`

```rust
pub use super::array_has::array_has;
```

#### Re-export `array_has_all`

```rust
pub use super::array_has::array_has_all;
```

#### Re-export `array_has_any`

```rust
pub use super::array_has::array_has_any;
```

#### Re-export `cardinality`

```rust
pub use super::cardinality::cardinality;
```

#### Re-export `array_append`

```rust
pub use super::concat::array_append;
```

#### Re-export `array_concat`

```rust
pub use super::concat::array_concat;
```

#### Re-export `array_prepend`

```rust
pub use super::concat::array_prepend;
```

#### Re-export `array_dims`

```rust
pub use super::dimension::array_dims;
```

#### Re-export `array_ndims`

```rust
pub use super::dimension::array_ndims;
```

#### Re-export `array_distance`

```rust
pub use super::distance::array_distance;
```

#### Re-export `array_empty`

```rust
pub use super::empty::array_empty;
```

#### Re-export `array_except`

```rust
pub use super::except::array_except;
```

#### Re-export `array_any_value`

```rust
pub use super::extract::array_any_value;
```

#### Re-export `array_element`

```rust
pub use super::extract::array_element;
```

#### Re-export `array_pop_back`

```rust
pub use super::extract::array_pop_back;
```

#### Re-export `array_pop_front`

```rust
pub use super::extract::array_pop_front;
```

#### Re-export `array_slice`

```rust
pub use super::extract::array_slice;
```

#### Re-export `flatten`

```rust
pub use super::flatten::flatten;
```

#### Re-export `array_length`

```rust
pub use super::length::array_length;
```

#### Re-export `make_array`

```rust
pub use super::make_array::make_array;
```

#### Re-export `map_extract`

```rust
pub use super::map_extract::map_extract;
```

#### Re-export `map_keys`

```rust
pub use super::map_keys::map_keys;
```

#### Re-export `map_values`

```rust
pub use super::map_values::map_values;
```

#### Re-export `array_position`

```rust
pub use super::position::array_position;
```

#### Re-export `array_positions`

```rust
pub use super::position::array_positions;
```

#### Re-export `gen_series`

```rust
pub use super::range::gen_series;
```

#### Re-export `range`

```rust
pub use super::range::range;
```

#### Re-export `array_remove`

```rust
pub use super::remove::array_remove;
```

#### Re-export `array_remove_all`

```rust
pub use super::remove::array_remove_all;
```

#### Re-export `array_remove_n`

```rust
pub use super::remove::array_remove_n;
```

#### Re-export `array_repeat`

```rust
pub use super::repeat::array_repeat;
```

#### Re-export `array_replace`

```rust
pub use super::replace::array_replace;
```

#### Re-export `array_replace_all`

```rust
pub use super::replace::array_replace_all;
```

#### Re-export `array_replace_n`

```rust
pub use super::replace::array_replace_n;
```

#### Re-export `array_resize`

```rust
pub use super::resize::array_resize;
```

#### Re-export `array_reverse`

```rust
pub use super::reverse::array_reverse;
```

#### Re-export `array_distinct`

```rust
pub use super::set_ops::array_distinct;
```

#### Re-export `array_intersect`

```rust
pub use super::set_ops::array_intersect;
```

#### Re-export `array_union`

```rust
pub use super::set_ops::array_union;
```

#### Re-export `array_sort`

```rust
pub use super::sort::array_sort;
```

#### Re-export `array_to_string`

```rust
pub use super::string::array_to_string;
```

#### Re-export `string_to_array`

```rust
pub use super::string::string_to_array;
```

## Functions

### Function `all_default_nested_functions`

Return all default nested type functions

```rust
pub fn all_default_nested_functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

### Function `register_all`

Registers all enabled packages with a [`FunctionRegistry`]

```rust
pub fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()> { /* ... */ }
```


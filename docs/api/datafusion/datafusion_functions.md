# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_functions`

 Function packages for [DataFusion].

 This crate contains a collection of various function packages for DataFusion,
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

 To access and use only the functions in a certain package, use the
 `functions()` method in each module.

 ```
 # fn main() -> datafusion_common::Result<()> {
 # let mut registry = datafusion_execution::registry::MemoryFunctionRegistry::new();
 # use datafusion_execution::FunctionRegistry;
 // get the encoding functions
 use datafusion_functions::encoding;
 for udf in encoding::functions() {
   registry.register_udf(udf)?;
 }
 # Ok(())
 # }
 ```

 Each package also exports an `expr_fn` submodule to help create [`Expr`]s that invoke
 functions using a fluent style. For example:

 ```
 // create an Expr that will invoke the encode function
 use datafusion_expr::{col, lit};
 use datafusion_functions::expr_fn;
 // Equivalent to "encode(my_data, 'hex')" in SQL:
 let expr = expr_fn::encode(col("my_data"), lit("hex"));
 ```

[`Expr`]: datafusion_expr::Expr

 # Implementing A New Package

 To add a new package to this crate, you should follow the model of existing
 packages. The high level steps are:

 1. Create a new module with the appropriate [`ScalarUDF`] implementations.

 2. Use the macros in [`macros`] to create standard entry points.

 3. Add a new feature to `Cargo.toml`, with any optional dependencies

 4. Use the `make_package!` macro to expose the module when the
    feature is enabled.

 [`ScalarUDF`]: datafusion_expr::ScalarUDF

## Modules

## Module `macros`

**Attributes:**

- `#[macro_use]`

```rust
pub mod macros { /* ... */ }
```

## Module `string`

**Attributes:**

- `#[cfg(feature = "string_expressions")]`

"string" DataFusion functions

```rust
pub mod string { /* ... */ }
```

### Modules

## Module `ascii`

```rust
pub mod ascii { /* ... */ }
```

### Types

#### Struct `AsciiFunc`

```rust
pub struct AsciiFunc {
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

- **Sync**
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **MaybeSendSync**
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
- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **Freeze**
### Functions

#### Function `ascii`

Returns the numeric code of the first character of the argument.

```rust
pub fn ascii(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `bit_length`

```rust
pub mod bit_length { /* ... */ }
```

### Types

#### Struct `BitLengthFunc`

```rust
pub struct BitLengthFunc {
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **ErasedDestructor**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Same**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `btrim`

```rust
pub mod btrim { /* ... */ }
```

### Types

#### Struct `BTrimFunc`

```rust
pub struct BTrimFunc {
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
- **Same**
- **IntoEither**
- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

## Module `chr`

```rust
pub mod chr { /* ... */ }
```

### Types

#### Struct `ChrFunc`

```rust
pub struct ChrFunc {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Unpin**
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

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **IntoEither**
### Functions

#### Function `chr`

Returns the character with the given code. chr(0) is disallowed because text data types cannot store that character.
chr(65) = 'A'

```rust
pub fn chr(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `common`

Common utilities for implementing string functions

```rust
pub mod common { /* ... */ }
```

## Module `concat`

```rust
pub mod concat { /* ... */ }
```

### Types

#### Struct `ConcatFunc`

```rust
pub struct ConcatFunc {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **IntoEither**
- **RefUnwindSafe**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```
    Concatenates the text representations of all the arguments. NULL arguments are ignored.

  - ```rust
    fn simplify(self: &Self, args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```
    Simplify the `concat` function by

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn preserves_lex_ordering(self: &Self, _inputs: &[ExprProperties]) -> Result<bool> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Functions

#### Function `simplify_concat`

```rust
pub fn simplify_concat(args: Vec<datafusion_expr::Expr>) -> datafusion_common::Result<datafusion_expr::simplify::ExprSimplifyResult> { /* ... */ }
```

## Module `concat_ws`

```rust
pub mod concat_ws { /* ... */ }
```

### Types

#### Struct `ConcatWsFunc`

```rust
pub struct ConcatWsFunc {
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```
    Concatenates all but the first argument, with separators. The first argument is used as the separator string, and should not be NULL. Other NULL arguments are ignored.

  - ```rust
    fn simplify(self: &Self, args: Vec<Expr>, _info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```
    Simply the `concat_ws` function by

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

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

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
- **Unpin**
- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `contains`

```rust
pub mod contains { /* ... */ }
```

### Types

#### Struct `ContainsFunc`

```rust
pub struct ContainsFunc {
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Same**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `ends_with`

```rust
pub mod ends_with { /* ... */ }
```

### Types

#### Struct `EndsWithFunc`

```rust
pub struct EndsWithFunc {
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **Send**
- **RefUnwindSafe**
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Same**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
## Module `levenshtein`

```rust
pub mod levenshtein { /* ... */ }
```

### Types

#### Struct `LevenshteinFunc`

```rust
pub struct LevenshteinFunc {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
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

- **Send**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Same**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
## Module `lower`

```rust
pub mod lower { /* ... */ }
```

### Types

#### Struct `LowerFunc`

```rust
pub struct LowerFunc {
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
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **Freeze**
- **MaybeSendSync**
- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Unpin**
## Module `ltrim`

```rust
pub mod ltrim { /* ... */ }
```

### Types

#### Struct `LtrimFunc`

```rust
pub struct LtrimFunc {
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

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

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
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

- **RefUnwindSafe**
- **ErasedDestructor**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `octet_length`

```rust
pub mod octet_length { /* ... */ }
```

### Types

#### Struct `OctetLengthFunc`

```rust
pub struct OctetLengthFunc {
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
- **Same**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
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

- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `overlay`

```rust
pub mod overlay { /* ... */ }
```

### Re-exports

#### Re-export `crate::core::overlay::*`

**Attributes:**

- `#[deprecated(note =
"overlay has been moved to core. Update imports to use core::overlay.")]`

**⚠️ Deprecated**: overlay has been moved to core. Update imports to use core::overlay.

```rust
pub use crate::core::overlay::*;
```

## Module `repeat`

```rust
pub mod repeat { /* ... */ }
```

### Types

#### Struct `RepeatFunc`

```rust
pub struct RepeatFunc {
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
- **Unpin**
- **Freeze**
- **ErasedDestructor**
- **UnwindSafe**
## Module `replace`

```rust
pub mod replace { /* ... */ }
```

### Types

#### Struct `ReplaceFunc`

```rust
pub struct ReplaceFunc {
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

- **Unpin**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Same**
- **UnwindSafe**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `rtrim`

```rust
pub mod rtrim { /* ... */ }
```

### Types

#### Struct `RtrimFunc`

```rust
pub struct RtrimFunc {
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
- **Same**
- **ErasedDestructor**
- **RefUnwindSafe**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
## Module `split_part`

```rust
pub mod split_part { /* ... */ }
```

### Types

#### Struct `SplitPartFunc`

```rust
pub struct SplitPartFunc {
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Same**
- **ErasedDestructor**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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

- **Sync**
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
### Functions

#### Function `split_part_impl`

impl

```rust
pub fn split_part_impl<''a, StringArrType, DelimiterArrType, StringArrayLen>(string_array: StringArrType, delimiter_array: DelimiterArrType, n_array: &arrow::array::Int64Array) -> datafusion_common::Result<arrow::array::ArrayRef>
where
    StringArrType: StringArrayType<''a>,
    DelimiterArrType: StringArrayType<''a>,
    StringArrayLen: OffsetSizeTrait { /* ... */ }
```

## Module `starts_with`

```rust
pub mod starts_with { /* ... */ }
```

### Types

#### Struct `StartsWithFunc`

```rust
pub struct StartsWithFunc {
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

- **Send**
- **IntoEither**
- **RefUnwindSafe**
- **MaybeSendSync**
- **Same**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn simplify(self: &Self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `to_hex`

```rust
pub mod to_hex { /* ... */ }
```

### Types

#### Struct `ToHexFunc`

```rust
pub struct ToHexFunc {
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Same**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **Freeze**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
### Functions

#### Function `to_hex`

Converts the number to its equivalent hexadecimal representation.
to_hex(2147483647) = '7fffffff'

```rust
pub fn to_hex<T: ArrowPrimitiveType>(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef>
where
    <T as >::Native: OffsetSizeTrait { /* ... */ }
```

## Module `upper`

```rust
pub mod upper { /* ... */ }
```

### Types

#### Struct `UpperFunc`

```rust
pub struct UpperFunc {
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
- **MaybeSendSync**
- **UnwindSafe**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Same**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `uuid`

```rust
pub mod uuid { /* ... */ }
```

### Types

#### Struct `UuidFunc`

```rust
pub struct UuidFunc {
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **Unpin**
- **Send**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```
    Prints random (v4) uuid values per row

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
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
- **UnwindSafe**
## Module `expr_fn`

```rust
pub mod expr_fn { /* ... */ }
```

### Functions

#### Function `ascii`

Returns the numeric code of the first character of the argument.

```rust
pub fn ascii(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `bit_length`

Returns the number of bits in the `string`

```rust
pub fn bit_length(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `btrim`

Removes all characters, spaces by default, from both sides of a string

```rust
pub fn btrim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `chr`

Converts the Unicode code point to a UTF8 character

```rust
pub fn chr(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `concat`

Concatenates the text representations of all the arguments. NULL arguments are ignored

```rust
pub fn concat(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `ends_with`

Returns true if the `string` ends with the `suffix`, false otherwise.

```rust
pub fn ends_with(string: datafusion_expr::Expr, suffix: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `levenshtein`

Returns the Levenshtein distance between the two given strings

```rust
pub fn levenshtein(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `lower`

Converts a string to lowercase.

```rust
pub fn lower(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `ltrim`

Removes all characters, spaces by default, from the beginning of a string

```rust
pub fn ltrim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `octet_length`

returns the number of bytes of a string

```rust
pub fn octet_length(args: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `repeat`

Repeats the `string` to `n` times

```rust
pub fn repeat(string: datafusion_expr::Expr, n: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `replace`

Replaces all occurrences of `from` with `to` in the `string`

```rust
pub fn replace(string: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `rtrim`

Removes all characters, spaces by default, from the end of a string

```rust
pub fn rtrim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `split_part`

Splits a string based on a delimiter and picks out the desired field based on the index.

```rust
pub fn split_part(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, index: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `starts_with`

Returns true if string starts with prefix.

```rust
pub fn starts_with(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_hex`

Converts an integer to a hexadecimal string.

```rust
pub fn to_hex(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `upper`

Converts a string to uppercase.

```rust
pub fn upper(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `uuid`

returns uuid v4 as a string value

```rust
pub fn uuid() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `contains`

Return true if search_string is found within string.

```rust
pub fn contains() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `trim`

Removes all characters, spaces by default, from both sides of a string

```rust
pub fn trim(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `concat_ws`

Concatenates all but the first argument, with separators. The first argument is used as the separator string, and should not be NULL. Other NULL arguments are ignored.

```rust
pub fn concat_ws(delimiter: datafusion_expr::Expr, args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

### Functions

#### Function `ascii`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of ascii

```rust
pub fn ascii() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `bit_length`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of bit_length

```rust
pub fn bit_length() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `btrim`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of btrim

```rust
pub fn btrim() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `chr`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of chr

```rust
pub fn chr() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `concat`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of concat

```rust
pub fn concat() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `concat_ws`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of concat_ws

```rust
pub fn concat_ws() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `ends_with`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of ends_with

```rust
pub fn ends_with() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `levenshtein`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of levenshtein

```rust
pub fn levenshtein() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `ltrim`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of ltrim

```rust
pub fn ltrim() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `lower`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of lower

```rust
pub fn lower() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `octet_length`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of octet_length

```rust
pub fn octet_length() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `repeat`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of repeat

```rust
pub fn repeat() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `replace`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of replace

```rust
pub fn replace() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `rtrim`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of rtrim

```rust
pub fn rtrim() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `starts_with`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of starts_with

```rust
pub fn starts_with() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `split_part`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of split_part

```rust
pub fn split_part() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_hex`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_hex

```rust
pub fn to_hex() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `upper`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of upper

```rust
pub fn upper() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `uuid`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of uuid

```rust
pub fn uuid() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `contains`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of contains

```rust
pub fn contains() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `functions`

Returns all DataFusion functions defined in this package

```rust
pub fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

## Module `core`

Core datafusion expressions
These are always available and not controlled by a feature flag
"core" DataFusion functions

```rust
pub mod core { /* ... */ }
```

### Modules

## Module `arrow_cast`

[`ArrowCastFunc`]: Implementation of the `arrow_cast`

```rust
pub mod arrow_cast { /* ... */ }
```

### Types

#### Struct `ArrowCastFunc`

Implements casting to arbitrary arrow types (rather than SQL types)

Note that the `arrow_cast` function is somewhat special in that its
return depends only on the *value* of its second argument (not its type)

It is implemented by calling the same underlying arrow `cast` kernel as
normal SQL casts.

For example to cast to `int` using SQL  (which is then mapped to the arrow
type `Int32`)

```sql
select cast(column_x as int) ...
```

Use the `arrow_cast` function to cast to a specific arrow type

For example
```sql
select arrow_cast(column_x, 'Float64')
```

```rust
pub struct ArrowCastFunc {
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
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn return_type_from_args(self: &Self, args: ReturnTypeArgs<''_>) -> Result<ReturnInfo> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, _args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn simplify(self: &Self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `arrowtypeof`

```rust
pub mod arrowtypeof { /* ... */ }
```

### Types

#### Struct `ArrowTypeOfFunc`

```rust
pub struct ArrowTypeOfFunc {
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
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Unpin**
- **MaybeSendSync**
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `coalesce`

```rust
pub mod coalesce { /* ... */ }
```

### Types

#### Struct `CoalesceFunc`

```rust
pub struct CoalesceFunc {
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
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn return_type_from_args(self: &Self, args: ReturnTypeArgs<''_>) -> Result<ReturnInfo> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```
    coalesce evaluates to the first value which is not NULL

  - ```rust
    fn short_circuits(self: &Self) -> bool { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Send**
- **ErasedDestructor**
- **Unpin**
- **UnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `expr_ext`

Extension methods for Expr.

```rust
pub mod expr_ext { /* ... */ }
```

### Traits

#### Trait `FieldAccessor`

Return access to the named field. Example `expr["name"]`

## Access field "my_field" from column "c1"

For example if column "c1" holds documents like this

```json
{
  "my_field": 123.34,
  "other_field": "Boston",
}
```

You can access column "my_field" with

```
# use datafusion_expr::{col};
# use datafusion_functions::core::expr_ext::FieldAccessor;
let expr = col("c1")
   .field("my_field");
assert_eq!(expr.schema_name().to_string(), "c1[my_field]");
```

```rust
pub trait FieldAccessor {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `field`

##### Implementations

This trait is implemented for the following types:

- `datafusion_expr::Expr`

## Module `getfield`

```rust
pub mod getfield { /* ... */ }
```

### Types

#### Struct `GetFieldFunc`

```rust
pub struct GetFieldFunc {
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
- **Send**
- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Same**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **MaybeSendSync**
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

- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
    fn display_name(self: &Self, args: &[Expr]) -> Result<String> { /* ... */ }
    ```

  - ```rust
    fn schema_name(self: &Self, args: &[Expr]) -> Result<String> { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, _: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn return_type_from_args(self: &Self, args: ReturnTypeArgs<''_>) -> Result<ReturnInfo> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `greatest`

```rust
pub mod greatest { /* ... */ }
```

### Types

#### Struct `GreatestFunc`

```rust
pub struct GreatestFunc {
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

- **Unpin**
- **Same**
- **RefUnwindSafe**
- **UnwindSafe**
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

- **Freeze**
- **Sync**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
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
- **Send**
## Module `least`

```rust
pub mod least { /* ... */ }
```

### Types

#### Struct `LeastFunc`

```rust
pub struct LeastFunc {
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

- **Send**
- **Freeze**
- **Same**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `named_struct`

```rust
pub mod named_struct { /* ... */ }
```

### Types

#### Struct `NamedStructFunc`

```rust
pub struct NamedStructFunc {
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
- **UnwindSafe**
- **RefUnwindSafe**
- **Send**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn return_type_from_args(self: &Self, args: ReturnTypeArgs<''_>) -> Result<ReturnInfo> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **MaybeSendSync**
- **IntoEither**
## Module `nullif`

```rust
pub mod nullif { /* ... */ }
```

### Types

#### Struct `NullIfFunc`

```rust
pub struct NullIfFunc {
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

- **UnwindSafe**
- **Sync**
- **Same**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
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

- **Unpin**
- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

## Module `nvl`

```rust
pub mod nvl { /* ... */ }
```

### Types

#### Struct `NVLFunc`

```rust
pub struct NVLFunc {
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Same**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Sync**
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ScalarUDFImpl**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **IntoEither**
## Module `nvl2`

```rust
pub mod nvl2 { /* ... */ }
```

### Types

#### Struct `NVL2Func`

```rust
pub struct NVL2Func {
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **ErasedDestructor**
- **UnwindSafe**
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

- **Same**
- **Sync**
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **MaybeSendSync**
- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ScalarUDFImpl**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `overlay`

```rust
pub mod overlay { /* ... */ }
```

### Types

#### Struct `OverlayFunc`

```rust
pub struct OverlayFunc {
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

- **Unpin**
- **Freeze**
- **RefUnwindSafe**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **IntoEither**
- **UnwindSafe**
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
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Send**
### Functions

#### Function `string_overlay`

```rust
pub fn string_overlay<T: OffsetSizeTrait>(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `string_view_overlay`

```rust
pub fn string_view_overlay<T: OffsetSizeTrait>(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `planner`

```rust
pub mod planner { /* ... */ }
```

### Types

#### Struct `CoreFunctionPlanner`

```rust
pub struct CoreFunctionPlanner {
}
```

##### Fields

| Name | Type | Documentation |
|------|------|---------------|

##### Implementations

###### Trait Implementations

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ExprPlanner**
  - ```rust
    fn plan_dictionary_literal(self: &Self, expr: RawDictionaryExpr, _schema: &DFSchema) -> Result<PlannerResult<RawDictionaryExpr>> { /* ... */ }
    ```

  - ```rust
    fn plan_struct_literal(self: &Self, args: Vec<Expr>, is_named_struct: bool) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
    ```

  - ```rust
    fn plan_overlay(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
    ```

  - ```rust
    fn plan_compound_identifier(self: &Self, field: &Field, qualifier: Option<&TableReference>, nested_names: &[String]) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
    ```

- **ErasedDestructor**
- **UnwindSafe**
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

- **Allocation**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **IntoEither**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Default**
  - ```rust
    fn default() -> CoreFunctionPlanner { /* ... */ }
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

- **Same**
## Module `struct`

```rust
pub mod struct { /* ... */ }
```

### Types

#### Struct `StructFunc`

```rust
pub struct StructFunc {
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

- **Send**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn signature(self: &Self) -> &Signature { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **ErasedDestructor**
- **UnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

## Module `union_extract`

```rust
pub mod union_extract { /* ... */ }
```

### Types

#### Struct `UnionExtractFun`

```rust
pub struct UnionExtractFun {
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ScalarUDFImpl**
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
    fn return_type(self: &Self, _: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn return_type_from_args(self: &Self, args: ReturnTypeArgs<''_>) -> Result<ReturnInfo> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **IntoEither**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **UnwindSafe**
- **RefUnwindSafe**
- **ErasedDestructor**
- **Sync**
## Module `version`

[`VersionFunc`]: Implementation of the `version` function.

```rust
pub mod version { /* ... */ }
```

### Types

#### Struct `VersionFunc`

```rust
pub struct VersionFunc {
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **IntoEither**
- **RefUnwindSafe**
- **UnwindSafe**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Freeze**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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
    fn return_type(self: &Self, args: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

## Module `expr_fn`

```rust
pub mod expr_fn { /* ... */ }
```

### Functions

#### Function `nullif`

Returns NULL if value1 equals value2; otherwise it returns value1. This can be used to perform the inverse operation of the COALESCE expression

```rust
pub fn nullif(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `arrow_cast`

Returns value2 if value1 is NULL; otherwise it returns value1

```rust
pub fn arrow_cast(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `nvl`

Returns value2 if value1 is NULL; otherwise it returns value1

```rust
pub fn nvl(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `nvl2`

Returns value2 if value1 is not NULL; otherwise, it returns value3.

```rust
pub fn nvl2(arg1: datafusion_expr::Expr, arg2: datafusion_expr::Expr, arg3: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `overlay`

replace the substring of string that starts at the start'th character and extends for count characters with new substring

```rust
pub fn overlay(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `arrow_typeof`

Returns the Arrow type of the input expression.

```rust
pub fn arrow_typeof(arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `struct`

Returns a struct with the given arguments

```rust
pub fn struct(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `named_struct`

Returns a struct with the given names and arguments pairs

```rust
pub fn named_struct(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `coalesce`

Returns `coalesce(args...)`, which evaluates to the value of the first expr which is not NULL

```rust
pub fn coalesce(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `greatest`

Returns `greatest(args...)`, which evaluates to the greatest value in the list of expressions or NULL if all the expressions are NULL

```rust
pub fn greatest(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `least`

Returns `least(args...)`, which evaluates to the smallest value in the list of expressions or NULL if all the expressions are NULL

```rust
pub fn least(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `get_field`

Returns the value of the field with the given name from the struct

```rust
pub fn get_field</* synthetic */ impl Literal: Literal>(arg1: datafusion_expr::Expr, arg2: impl Literal) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `union_extract`

Returns the value of the field with the given name from the union when it's selected, or NULL otherwise

```rust
pub fn union_extract</* synthetic */ impl Literal: Literal>(arg1: datafusion_expr::Expr, arg2: impl Literal) -> datafusion_expr::Expr { /* ... */ }
```

### Functions

#### Function `arrow_cast`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of arrow_cast

```rust
pub fn arrow_cast() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `nullif`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of nullif

```rust
pub fn nullif() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `nvl`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of nvl

```rust
pub fn nvl() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `nvl2`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of nvl2

```rust
pub fn nvl2() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `overlay`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of overlay

```rust
pub fn overlay() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `arrow_typeof`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of arrow_typeof

```rust
pub fn arrow_typeof() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `struct`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of r#struct

```rust
pub fn struct() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `named_struct`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of named_struct

```rust
pub fn named_struct() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `get_field`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of get_field

```rust
pub fn get_field() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `coalesce`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of coalesce

```rust
pub fn coalesce() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `greatest`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of greatest

```rust
pub fn greatest() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `least`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of least

```rust
pub fn least() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `union_extract`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of union_extract

```rust
pub fn union_extract() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `version`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of version

```rust
pub fn version() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `functions`

Returns all DataFusion functions defined in this package

```rust
pub fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

## Module `datetime`

**Attributes:**

- `#[cfg(feature = "datetime_expressions")]`

Date and time expressions.
Contains functions such as to_timestamp
Enabled via feature flag `datetime_expressions`
date & time DataFusion functions

```rust
pub mod datetime { /* ... */ }
```

### Modules

## Module `common`

```rust
pub mod common { /* ... */ }
```

## Module `current_date`

```rust
pub mod current_date { /* ... */ }
```

### Types

#### Struct `CurrentDateFunc`

```rust
pub struct CurrentDateFunc {
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
    fn invoke_with_args(self: &Self, _args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn simplify(self: &Self, _args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Send**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **IntoEither**
- **Freeze**
## Module `current_time`

```rust
pub mod current_time { /* ... */ }
```

### Types

#### Struct `CurrentTimeFunc`

```rust
pub struct CurrentTimeFunc {
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **MaybeSendSync**
- **IntoEither**
- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
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
    fn invoke_with_args(self: &Self, _args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn simplify(self: &Self, _args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Sync**
## Module `date_bin`

```rust
pub mod date_bin { /* ... */ }
```

### Types

#### Struct `DateBinFunc`

```rust
pub struct DateBinFunc {
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
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
    fn output_ordering(self: &Self, input: &[ExprProperties]) -> Result<SortProperties> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Send**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

## Module `date_part`

```rust
pub mod date_part { /* ... */ }
```

### Types

#### Struct `DatePartFunc`

```rust
pub struct DatePartFunc {
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
- **Same**
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
    fn return_type_from_args(self: &Self, args: ReturnTypeArgs<''_>) -> Result<ReturnInfo> { /* ... */ }
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **IntoEither**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **UnwindSafe**
## Module `date_trunc`

```rust
pub mod date_trunc { /* ... */ }
```

### Types

#### Struct `DateTruncFunc`

```rust
pub struct DateTruncFunc {
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
    fn output_ordering(self: &Self, input: &[ExprProperties]) -> Result<SortProperties> { /* ... */ }
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

- **UnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
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

- **MaybeSendSync**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
## Module `from_unixtime`

```rust
pub mod from_unixtime { /* ... */ }
```

### Types

#### Struct `FromUnixtimeFunc`

```rust
pub struct FromUnixtimeFunc {
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

- **Same**
- **Unpin**
- **ErasedDestructor**
- **RefUnwindSafe**
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

- **Freeze**
- **Sync**
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
    fn return_type_from_args(self: &Self, args: ReturnTypeArgs<''_>) -> Result<ReturnInfo> { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

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

- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **MaybeSendSync**
## Module `make_date`

```rust
pub mod make_date { /* ... */ }
```

### Types

#### Struct `MakeDateFunc`

```rust
pub struct MakeDateFunc {
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **ErasedDestructor**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Same**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **UnwindSafe**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

## Module `now`

```rust
pub mod now { /* ... */ }
```

### Types

#### Struct `NowFunc`

```rust
pub struct NowFunc {
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn return_type_from_args(self: &Self, _args: ReturnTypeArgs<''_>) -> Result<ReturnInfo> { /* ... */ }
    ```

  - ```rust
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, _args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn simplify(self: &Self, _args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Unpin**
- **MaybeSendSync**
- **Same**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **IntoEither**
## Module `to_char`

```rust
pub mod to_char { /* ... */ }
```

### Types

#### Struct `ToCharFunc`

```rust
pub struct ToCharFunc {
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

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
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
- **RefUnwindSafe**
- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **Freeze**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

## Module `to_date`

```rust
pub mod to_date { /* ... */ }
```

### Types

#### Struct `ToDateFunc`

```rust
pub struct ToDateFunc {
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

- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **ErasedDestructor**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **Same**
## Module `to_local_time`

```rust
pub mod to_local_time { /* ... */ }
```

### Types

#### Struct `ToLocalTimeFunc`

A UDF function that converts a timezone-aware timestamp to local time (with no offset or
timezone information). In other words, this function strips off the timezone from the timestamp,
while keep the display value of the timestamp the same.

```rust
pub struct ToLocalTimeFunc {
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
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **RefUnwindSafe**
- **Freeze**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `to_timestamp`

```rust
pub mod to_timestamp { /* ... */ }
```

### Types

#### Struct `ToTimestampFunc`

```rust
pub struct ToTimestampFunc {
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

- **Send**
- **Freeze**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **RefUnwindSafe**
- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `ToTimestampSecondsFunc`

```rust
pub struct ToTimestampSecondsFunc {
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
- **Sync**
- **UnwindSafe**
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

- **MaybeSendSync**
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **Same**
#### Struct `ToTimestampMillisFunc`

```rust
pub struct ToTimestampMillisFunc {
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Send**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **UnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
#### Struct `ToTimestampMicrosFunc`

```rust
pub struct ToTimestampMicrosFunc {
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **Same**
- **ErasedDestructor**
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

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

#### Struct `ToTimestampNanosFunc`

```rust
pub struct ToTimestampNanosFunc {
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
- **UnwindSafe**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **MaybeSendSync**
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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Unpin**
## Module `to_unixtime`

```rust
pub mod to_unixtime { /* ... */ }
```

### Types

#### Struct `ToUnixtimeFunc`

```rust
pub struct ToUnixtimeFunc {
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **ErasedDestructor**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **RefUnwindSafe**
- **Sync**
## Module `expr_fn`

```rust
pub mod expr_fn { /* ... */ }
```

### Functions

#### Function `current_date`

returns current UTC date as a Date32 value

```rust
pub fn current_date() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `current_time`

returns current UTC time as a Time64 value

```rust
pub fn current_time() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `from_unixtime`

converts an integer to RFC3339 timestamp format string

```rust
pub fn from_unixtime(unixtime: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `date_bin`

coerces an arbitrary timestamp to the start of the nearest specified interval

```rust
pub fn date_bin(stride: datafusion_expr::Expr, source: datafusion_expr::Expr, origin: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `date_part`

extracts a subfield from the date

```rust
pub fn date_part(part: datafusion_expr::Expr, date: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `date_trunc`

truncates the date to a specified level of precision

```rust
pub fn date_trunc(part: datafusion_expr::Expr, date: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `make_date`

make a date from year, month and day component parts

```rust
pub fn make_date(year: datafusion_expr::Expr, month: datafusion_expr::Expr, day: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `now`

returns the current timestamp in nanoseconds, using the same value for all instances of now() in same statement

```rust
pub fn now() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_local_time`

converts a timezone-aware timestamp to local time (with no offset or timezone information), i.e. strips off the timezone from the timestamp

```rust
pub fn to_local_time(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_unixtime`

converts a string and optional formats to a Unixtime

```rust
pub fn to_unixtime(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_timestamp`

converts a string and optional formats to a `Timestamp(Nanoseconds, None)`

```rust
pub fn to_timestamp(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_timestamp_seconds`

converts a string and optional formats to a `Timestamp(Seconds, None)`

```rust
pub fn to_timestamp_seconds(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_timestamp_millis`

converts a string and optional formats to a `Timestamp(Milliseconds, None)`

```rust
pub fn to_timestamp_millis(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_timestamp_micros`

converts a string and optional formats to a `Timestamp(Microseconds, None)`

```rust
pub fn to_timestamp_micros(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_timestamp_nanos`

converts a string and optional formats to a `Timestamp(Nanoseconds, None)`

```rust
pub fn to_timestamp_nanos(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_char`

Returns a string representation of a date, time, timestamp or duration based
on a Chrono pattern.

The syntax for the patterns can be found at
<https://docs.rs/chrono/latest/chrono/format/strftime/index.html>

# Examples

```ignore
# use chrono::prelude::*;
# use datafusion::prelude::*;
# use datafusion::error::Result;
# use datafusion_common::ScalarValue::TimestampNanosecond;
# use std::sync::Arc;
# use arrow::array::{Date32Array, RecordBatch, StringArray};
# use arrow::datatypes::{DataType, Field, Schema};
# #[tokio::main]
# async fn main() -> Result<()> {
let schema = Arc::new(Schema::new(vec![
    Field::new("values", DataType::Date32, false),
    Field::new("patterns", DataType::Utf8, false),
]));

let batch = RecordBatch::try_new(
    schema,
    vec![
        Arc::new(Date32Array::from(vec![
            18506,
            18507,
            18508,
            18509,
        ])),
        Arc::new(StringArray::from(vec![
            "%Y-%m-%d",
            "%Y:%m:%d",
            "%Y%m%d",
            "%d-%m-%Y",
        ])),
    ],
)?;

let ctx = SessionContext::new();
ctx.register_batch("t", batch)?;
let df = ctx.table("t").await?;

// use the to_char function to convert col 'values',
// to strings using patterns in col 'patterns'
let df = df.with_column(
    "date_str",
    to_char(col("values"), col("patterns"))
)?;
// Note that providing a scalar value for the pattern
// is more performant
let df = df.with_column(
    "date_str2",
    to_char(col("values"), lit("%d-%m-%Y"))
)?;
// literals can be used as well with dataframe calls
let timestamp = "2026-07-08T09:10:11"
    .parse::<NaiveDateTime>()
    .unwrap()
    .with_nanosecond(56789)
    .unwrap()
    .timestamp_nanos_opt()
    .unwrap();
let df = df.with_column(
    "timestamp_str",
    to_char(lit(TimestampNanosecond(Some(timestamp), None)), lit("%d-%m-%Y %H:%M:%S"))
)?;

df.show().await?;

# Ok(())
# }
```

```rust
pub fn to_char(datetime: datafusion_expr::Expr, format: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `to_date`

```ignore
# use std::sync::Arc;

# use datafusion_common::Result;

# #[tokio::main]
# async fn main() -> Result<()> {
#  use arrow::array::StringArray;
#  use arrow::datatypes::{DataType, Field, Schema};
#  use arrow::record_batch::RecordBatch;
#  use datafusion_expr::col;
#  use datafusion::prelude::*;
#  use datafusion_functions::expr_fn::to_date;

    // define a schema.
    let schema = Arc::new(Schema::new(vec![Field::new("a", DataType::Utf8, false)]));

    // define data.
    let batch = RecordBatch::try_new(
        schema,
        vec![Arc::new(StringArray::from(vec![
            "2020-09-08T13:42:29Z",
            "2020-09-08T13:42:29.190855-05:00",
            "2020-08-09 12:13:29",
            "2020-01-02",
        ]))],
    )?;

    // declare a new context. In spark API, this corresponds to a new spark SQLsession
    let ctx = SessionContext::new();

    // declare a table in memory. In spark API, this corresponds to createDataFrame(...).
    ctx.register_batch("t", batch)?;
    let df = ctx.table("t").await?;

    // use to_date function to convert col 'a' to timestamp type using the default parsing
    let df = df.with_column("a", to_date(vec![col("a")]))?;

    let df = df.select_columns(&["a"])?;

    // print the results
    df.show().await?;

    # Ok(())
# }
```

```rust
pub fn to_date(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

### Functions

#### Function `current_date`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of current_date

```rust
pub fn current_date() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `current_time`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of current_time

```rust
pub fn current_time() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `date_bin`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of date_bin

```rust
pub fn date_bin() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `date_part`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of date_part

```rust
pub fn date_part() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `date_trunc`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of date_trunc

```rust
pub fn date_trunc() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `make_date`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of make_date

```rust
pub fn make_date() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `from_unixtime`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of from_unixtime

```rust
pub fn from_unixtime() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `now`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of now

```rust
pub fn now() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_char`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_char

```rust
pub fn to_char() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_date`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_date

```rust
pub fn to_date() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_local_time`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_local_time

```rust
pub fn to_local_time() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_unixtime`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_unixtime

```rust
pub fn to_unixtime() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_timestamp`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp

```rust
pub fn to_timestamp() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_timestamp_seconds`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp_seconds

```rust
pub fn to_timestamp_seconds() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_timestamp_millis`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp_millis

```rust
pub fn to_timestamp_millis() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_timestamp_micros`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp_micros

```rust
pub fn to_timestamp_micros() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `to_timestamp_nanos`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of to_timestamp_nanos

```rust
pub fn to_timestamp_nanos() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `functions`

Returns all DataFusion functions defined in this package

```rust
pub fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

## Module `encoding`

**Attributes:**

- `#[cfg(feature = "encoding_expressions")]`

Encoding expressions.
Contains Hex and binary `encode` and `decode` functions.
Enabled via feature flag `encoding_expressions`

```rust
pub mod encoding { /* ... */ }
```

### Modules

## Module `inner`

Encoding expressions

```rust
pub mod inner { /* ... */ }
```

### Types

#### Struct `EncodeFunc`

```rust
pub struct EncodeFunc {
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
- **UnwindSafe**
- **RefUnwindSafe**
- **ErasedDestructor**
- **Same**
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
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Sync**
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
- **Unpin**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Struct `DecodeFunc`

```rust
pub struct DecodeFunc {
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
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Freeze**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **MaybeSendSync**
- **Same**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Unpin**
- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
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
    fn coerce_types(self: &Self, arg_types: &[DataType]) -> Result<Vec<DataType>> { /* ... */ }
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

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
## Module `expr_fn`

```rust
pub mod expr_fn { /* ... */ }
```

### Functions

#### Function `encode`

encode the `input`, using the `encoding`. encoding can be base64 or hex

```rust
pub fn encode(input: datafusion_expr::Expr, encoding: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `decode`

decode the `input`, using the `encoding`. encoding can be base64 or hex

```rust
pub fn decode(input: datafusion_expr::Expr, encoding: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

### Functions

#### Function `encode`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of encode

```rust
pub fn encode() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `decode`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of decode

```rust
pub fn decode() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `functions`

Returns all DataFusion functions defined in this package

```rust
pub fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

## Module `math`

**Attributes:**

- `#[cfg(feature = "math_expressions")]`

Mathematical functions.
Enabled via feature flag `math_expressions`
"math" DataFusion functions

```rust
pub mod math { /* ... */ }
```

### Modules

## Module `abs`

math expressions

```rust
pub mod abs { /* ... */ }
```

### Types

#### Struct `AbsFunc`

```rust
pub struct AbsFunc {
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
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn output_ordering(self: &Self, input: &[ExprProperties]) -> Result<SortProperties> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **RefUnwindSafe**
- **Sync**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `bounds`

```rust
pub mod bounds { /* ... */ }
```

## Module `cot`

```rust
pub mod cot { /* ... */ }
```

### Types

#### Struct `CotFunc`

```rust
pub struct CotFunc {
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

- **Send**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
- **IntoEither**
## Module `factorial`

```rust
pub mod factorial { /* ... */ }
```

### Types

#### Struct `FactorialFunc`

```rust
pub struct FactorialFunc {
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **Send**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **MaybeSendSync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `gcd`

```rust
pub mod gcd { /* ... */ }
```

### Types

#### Struct `GcdFunc`

```rust
pub struct GcdFunc {
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
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **UnwindSafe**
- **Sync**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Same**
- **Send**
- **Unpin**
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

### Functions

#### Function `compute_gcd`

Computes greatest common divisor using Binary GCD algorithm.

```rust
pub fn compute_gcd(x: i64, y: i64) -> datafusion_common::Result<i64, arrow::error::ArrowError> { /* ... */ }
```

## Module `iszero`

```rust
pub mod iszero { /* ... */ }
```

### Types

#### Struct `IsZeroFunc`

```rust
pub struct IsZeroFunc {
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
- **Sync**
- **Send**
- **RefUnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Same**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **MaybeSendSync**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

### Functions

#### Function `iszero`

Iszero SQL function

```rust
pub fn iszero(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `lcm`

```rust
pub mod lcm { /* ... */ }
```

### Types

#### Struct `LcmFunc`

```rust
pub struct LcmFunc {
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Unpin**
- **IntoEither**
- **UnwindSafe**
- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **MaybeSendSync**
- **Send**
- **Same**
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

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

## Module `log`

Math function: `log()`.

```rust
pub mod log { /* ... */ }
```

### Types

#### Struct `LogFunc`

```rust
pub struct LogFunc {
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **UnwindSafe**
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

- **Same**
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

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn output_ordering(self: &Self, input: &[ExprProperties]) -> Result<SortProperties> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

  - ```rust
    fn simplify(self: &Self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```
    Simplify the `log` function by the relevant rules:

- **Unpin**
- **Send**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

## Module `monotonicity`

```rust
pub mod monotonicity { /* ... */ }
```

### Functions

#### Function `acos_order`

Non-increasing on the interval \[−1, 1\], undefined otherwise.

```rust
pub fn acos_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_acos_doc`

```rust
pub fn get_acos_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `acosh_order`

Non-decreasing for x ≥ 1, undefined otherwise.

```rust
pub fn acosh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_acosh_doc`

```rust
pub fn get_acosh_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `asin_order`

Non-decreasing on the interval \[−1, 1\], undefined otherwise.

```rust
pub fn asin_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_asin_doc`

```rust
pub fn get_asin_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `asinh_order`

Non-decreasing for all real numbers.

```rust
pub fn asinh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_asinh_doc`

```rust
pub fn get_asinh_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `atan_order`

Non-decreasing for all real numbers.

```rust
pub fn atan_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_atan_doc`

```rust
pub fn get_atan_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `atanh_order`

Non-decreasing on the interval \[−1, 1\], undefined otherwise.

```rust
pub fn atanh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_atanh_doc`

```rust
pub fn get_atanh_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `atan2_order`

Order depends on the quadrant.

```rust
pub fn atan2_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_atan2_doc`

```rust
pub fn get_atan2_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `cbrt_order`

Non-decreasing for all real numbers.

```rust
pub fn cbrt_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_cbrt_doc`

```rust
pub fn get_cbrt_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `ceil_order`

Non-decreasing for all real numbers.

```rust
pub fn ceil_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_ceil_doc`

```rust
pub fn get_ceil_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `cos_order`

Non-increasing on \[0, π\] and then non-decreasing on \[π, 2π\].
This pattern repeats periodically with a period of 2π.

```rust
pub fn cos_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_cos_doc`

```rust
pub fn get_cos_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `cosh_order`

Non-decreasing for x ≥ 0 and symmetrically non-increasing for x ≤ 0.

```rust
pub fn cosh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_cosh_doc`

```rust
pub fn get_cosh_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `degrees_order`

Non-decreasing function that converts radians to degrees.

```rust
pub fn degrees_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_degrees_doc`

```rust
pub fn get_degrees_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `exp_order`

Non-decreasing for all real numbers.

```rust
pub fn exp_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_exp_doc`

```rust
pub fn get_exp_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `floor_order`

Non-decreasing for all real numbers.

```rust
pub fn floor_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_floor_doc`

```rust
pub fn get_floor_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `ln_order`

Non-decreasing for x ≥ 0, undefined otherwise.

```rust
pub fn ln_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_ln_doc`

```rust
pub fn get_ln_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `log2_order`

Non-decreasing for x ≥ 0, undefined otherwise.

```rust
pub fn log2_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_log2_doc`

```rust
pub fn get_log2_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `log10_order`

Non-decreasing for x ≥ 0, undefined otherwise.

```rust
pub fn log10_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_log10_doc`

```rust
pub fn get_log10_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `radians_order`

Non-decreasing for all real numbers x.

```rust
pub fn radians_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_radians_doc`

```rust
pub fn get_radians_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `sin_order`

Non-decreasing on \[0, π\] and then non-increasing on \[π, 2π\].
This pattern repeats periodically with a period of 2π.

```rust
pub fn sin_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_sin_doc`

```rust
pub fn get_sin_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `sinh_order`

Non-decreasing for all real numbers.

```rust
pub fn sinh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_sinh_doc`

```rust
pub fn get_sinh_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `sqrt_order`

Non-decreasing for x ≥ 0, undefined otherwise.

```rust
pub fn sqrt_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_sqrt_doc`

```rust
pub fn get_sqrt_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `tan_order`

Non-decreasing between vertical asymptotes at x = k * π ± π / 2 for any
integer k.

```rust
pub fn tan_order(_input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_tan_doc`

```rust
pub fn get_tan_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

#### Function `tanh_order`

Non-decreasing for all real numbers.

```rust
pub fn tanh_order(input: &[datafusion_expr::sort_properties::ExprProperties]) -> datafusion_common::Result<datafusion_expr::sort_properties::SortProperties> { /* ... */ }
```

#### Function `get_tanh_doc`

```rust
pub fn get_tanh_doc() -> &''static datafusion_expr::Documentation { /* ... */ }
```

## Module `nans`

Math function: `isnan()`.

```rust
pub mod nans { /* ... */ }
```

### Types

#### Struct `IsNanFunc`

```rust
pub struct IsNanFunc {
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

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

- **Unpin**
- **MaybeSendSync**
- **ErasedDestructor**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Same**
- **Freeze**
- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `nanvl`

```rust
pub mod nanvl { /* ... */ }
```

### Types

#### Struct `NanvlFunc`

```rust
pub struct NanvlFunc {
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

- **Same**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **RefUnwindSafe**
- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **UnwindSafe**
## Module `pi`

```rust
pub mod pi { /* ... */ }
```

### Types

#### Struct `PiFunc`

```rust
pub struct PiFunc {
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

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **IntoEither**
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

- **Send**
- **UnwindSafe**
- **Same**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
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

  - ```rust
    fn output_ordering(self: &Self, _input: &[ExprProperties]) -> Result<SortProperties> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Sync**
## Module `power`

Math function: `power()`.

```rust
pub mod power { /* ... */ }
```

### Types

#### Struct `PowerFunc`

```rust
pub struct PowerFunc {
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

- **Same**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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
    fn aliases(self: &Self) -> &[String] { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn simplify(self: &Self, args: Vec<Expr>, info: &dyn SimplifyInfo) -> Result<ExprSimplifyResult> { /* ... */ }
    ```
    Simplify the `power` function by the relevant rules:

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Unpin**
- **MaybeSendSync**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

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

## Module `random`

```rust
pub mod random { /* ... */ }
```

### Types

#### Struct `RandomFunc`

```rust
pub struct RandomFunc {
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
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

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **IntoEither**
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

- **Same**
- **Sync**
- **UnwindSafe**
- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `round`

```rust
pub mod round { /* ... */ }
```

### Types

#### Struct `RoundFunc`

```rust
pub struct RoundFunc {
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn output_ordering(self: &Self, input: &[ExprProperties]) -> Result<SortProperties> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
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

- **Unpin**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **IntoEither**
- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

### Functions

#### Function `round`

Round SQL function

```rust
pub fn round(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `signum`

```rust
pub mod signum { /* ... */ }
```

### Types

#### Struct `SignumFunc`

```rust
pub struct SignumFunc {
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **UnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn output_ordering(self: &Self, input: &[ExprProperties]) -> Result<SortProperties> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
- **Unpin**
- **Same**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ErasedDestructor**
### Functions

#### Function `signum`

signum SQL function

```rust
pub fn signum(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `trunc`

```rust
pub mod trunc { /* ... */ }
```

### Types

#### Struct `TruncFunc`

```rust
pub struct TruncFunc {
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
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Same**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn output_ordering(self: &Self, input: &[ExprProperties]) -> Result<SortProperties> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Unpin**
- **Freeze**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

## Module `expr_fn`

```rust
pub mod expr_fn { /* ... */ }
```

### Functions

#### Function `abs`

returns the absolute value of a given number

```rust
pub fn abs(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `acos`

returns the arc cosine or inverse cosine of a number

```rust
pub fn acos(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `acosh`

returns inverse hyperbolic cosine

```rust
pub fn acosh(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `asin`

returns the arc sine or inverse sine of a number

```rust
pub fn asin(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `asinh`

returns inverse hyperbolic sine

```rust
pub fn asinh(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `atan`

returns inverse tangent

```rust
pub fn atan(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `atan2`

returns inverse tangent of a division given in the argument

```rust
pub fn atan2(y: datafusion_expr::Expr, x: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `atanh`

returns inverse hyperbolic tangent

```rust
pub fn atanh(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `cbrt`

cube root of a number

```rust
pub fn cbrt(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `ceil`

nearest integer greater than or equal to argument

```rust
pub fn ceil(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `cos`

cosine

```rust
pub fn cos(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `cosh`

hyperbolic cosine

```rust
pub fn cosh(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `cot`

cotangent of a number

```rust
pub fn cot(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `degrees`

converts radians to degrees

```rust
pub fn degrees(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `exp`

exponential

```rust
pub fn exp(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `factorial`

factorial

```rust
pub fn factorial(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `floor`

nearest integer less than or equal to argument

```rust
pub fn floor(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `gcd`

greatest common divisor

```rust
pub fn gcd(x: datafusion_expr::Expr, y: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `isnan`

returns true if a given number is +NaN or -NaN otherwise returns false

```rust
pub fn isnan(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `iszero`

returns true if a given number is +0.0 or -0.0 otherwise returns false

```rust
pub fn iszero(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `lcm`

least common multiple

```rust
pub fn lcm(x: datafusion_expr::Expr, y: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `ln`

natural logarithm (base e) of a number

```rust
pub fn ln(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `log`

logarithm of a number for a particular `base`

```rust
pub fn log(base: datafusion_expr::Expr, num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `log2`

base 2 logarithm of a number

```rust
pub fn log2(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `log10`

base 10 logarithm of a number

```rust
pub fn log10(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `nanvl`

returns x if x is not NaN otherwise returns y

```rust
pub fn nanvl(x: datafusion_expr::Expr, y: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `pi`

Returns an approximate value of π

```rust
pub fn pi() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `power`

`base` raised to the power of `exponent`

```rust
pub fn power(base: datafusion_expr::Expr, exponent: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `radians`

converts degrees to radians

```rust
pub fn radians(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `random`

Returns a random value in the range 0.0 <= x < 1.0

```rust
pub fn random() -> datafusion_expr::Expr { /* ... */ }
```

#### Function `signum`

sign of the argument (-1, 0, +1)

```rust
pub fn signum(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `sin`

sine

```rust
pub fn sin(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `sinh`

hyperbolic sine

```rust
pub fn sinh(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `sqrt`

square root of a number

```rust
pub fn sqrt(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `tan`

returns the tangent of a number

```rust
pub fn tan(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `tanh`

returns the hyperbolic tangent of a number

```rust
pub fn tanh(num: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `round`

round to nearest integer

```rust
pub fn round(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `trunc`

truncate toward zero, with optional precision

```rust
pub fn trunc(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

### Functions

#### Function `abs`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of abs

```rust
pub fn abs() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `acos`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of acos

```rust
pub fn acos() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `acosh`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of acosh

```rust
pub fn acosh() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `asin`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of asin

```rust
pub fn asin() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `asinh`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of asinh

```rust
pub fn asinh() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `atan`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of atan

```rust
pub fn atan() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `atanh`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of atanh

```rust
pub fn atanh() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `atan2`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of atan2

```rust
pub fn atan2() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `cbrt`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of cbrt

```rust
pub fn cbrt() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `ceil`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of ceil

```rust
pub fn ceil() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `cos`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of cos

```rust
pub fn cos() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `cosh`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of cosh

```rust
pub fn cosh() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `cot`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of cot

```rust
pub fn cot() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `degrees`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of degrees

```rust
pub fn degrees() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `exp`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of exp

```rust
pub fn exp() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `factorial`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of factorial

```rust
pub fn factorial() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `floor`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of floor

```rust
pub fn floor() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `log`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of log

```rust
pub fn log() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `gcd`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of gcd

```rust
pub fn gcd() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `isnan`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of isnan

```rust
pub fn isnan() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `iszero`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of iszero

```rust
pub fn iszero() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `lcm`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of lcm

```rust
pub fn lcm() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `ln`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of ln

```rust
pub fn ln() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `log2`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of log2

```rust
pub fn log2() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `log10`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of log10

```rust
pub fn log10() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `nanvl`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of nanvl

```rust
pub fn nanvl() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `pi`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of pi

```rust
pub fn pi() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `power`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of power

```rust
pub fn power() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `radians`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of radians

```rust
pub fn radians() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `random`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of random

```rust
pub fn random() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `round`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of round

```rust
pub fn round() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `signum`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of signum

```rust
pub fn signum() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `sin`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of sin

```rust
pub fn sin() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `sinh`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of sinh

```rust
pub fn sinh() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `sqrt`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of sqrt

```rust
pub fn sqrt() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `tan`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of tan

```rust
pub fn tan() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `tanh`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of tanh

```rust
pub fn tanh() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `trunc`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of trunc

```rust
pub fn trunc() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `functions`

Returns all DataFusion functions defined in this package

```rust
pub fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

## Module `regex`

**Attributes:**

- `#[cfg(feature = "regex_expressions")]`

Regular expression functions.
Enabled via feature flag `regex_expressions`
"regex" DataFusion functions

```rust
pub mod regex { /* ... */ }
```

### Modules

## Module `regexpcount`

```rust
pub mod regexpcount { /* ... */ }
```

### Types

#### Struct `RegexpCountFunc`

```rust
pub struct RegexpCountFunc {
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
- **Unpin**
- **UnwindSafe**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **RefUnwindSafe**
- **IntoEither**
- **ScalarUDFImpl**
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Send**
- **Same**
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Functions

#### Function `regexp_count_func`

```rust
pub fn regexp_count_func(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `regexp_count`

`arrow-rs` style implementation of `regexp_count` function.
This function `regexp_count` is responsible for counting the occurrences of a regular expression pattern
within a string array. It supports optional start positions and flags for case insensitivity.

The function accepts a variable number of arguments:
- `values`: The array of strings to search within.
- `regex_array`: The array of regular expression patterns to search for.
- `start_array` (optional): The array of start positions for the search.
- `flags_array` (optional): The array of flags to modify the search behavior (e.g., case insensitivity).

The function handles different combinations of scalar and array inputs for the regex patterns, start positions,
and flags. It uses a cache to store compiled regular expressions for efficiency.

# Errors
Returns an error if the input arrays have mismatched lengths or if the regular expression fails to compile.

```rust
pub fn regexp_count(values: &dyn Array, regex_array: &dyn Datum, start_array: Option<&dyn Datum>, flags_array: Option<&dyn Datum>) -> datafusion_common::Result<arrow::array::ArrayRef, arrow::error::ArrowError> { /* ... */ }
```

#### Function `regexp_count_inner`

```rust
pub fn regexp_count_inner<''a, S>(values: S, regex_array: S, is_regex_scalar: bool, start_array: Option<&arrow::array::Int64Array>, is_start_scalar: bool, flags_array: Option<S>, is_flags_scalar: bool) -> datafusion_common::Result<arrow::array::ArrayRef, arrow::error::ArrowError>
where
    S: StringArrayType<''a> { /* ... */ }
```

## Module `regexplike`

Regex expressions

```rust
pub mod regexplike { /* ... */ }
```

### Types

#### Struct `RegexpLikeFunc`

```rust
pub struct RegexpLikeFunc {
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
- **Unpin**
- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Same**
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

- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Sync**
- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

### Functions

#### Function `regexp_like`

Tests a string using a regular expression returning true if at
least one match, false otherwise.

The full list of supported features and syntax can be found at
<https://docs.rs/regex/latest/regex/#syntax>

Supported flags can be found at
<https://docs.rs/regex/latest/regex/#grouping-and-flags>

# Examples

```ignore
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/regex.csv", CsvReadOptions::new()).await?;

// use the regexp_like function to test col 'values',
// against patterns in col 'patterns' without flags
let df = df.with_column(
    "a",
    regexp_like(vec![col("values"), col("patterns")])
)?;
// use the regexp_like function to test col 'values',
// against patterns in col 'patterns' with flags
let df = df.with_column(
    "b",
    regexp_like(vec![col("values"), col("patterns"), col("flags")])
)?;
// literals can be used as well with dataframe calls
let df = df.with_column(
    "c",
    regexp_like(vec![lit("foobarbequebaz"), lit("(bar)(beque)")])
)?;

df.show().await?;

# Ok(())
# }
```

```rust
pub fn regexp_like(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `regexpmatch`

Regex expressions

```rust
pub mod regexpmatch { /* ... */ }
```

### Types

#### Struct `RegexpMatchFunc`

```rust
pub struct RegexpMatchFunc {
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Sync**
- **RefUnwindSafe**
- **MaybeSendSync**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Freeze**
- **Same**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Unpin**
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

- **UnwindSafe**
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

### Functions

#### Function `regexp_match`

```rust
pub fn regexp_match(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `regexpreplace`

Regex expressions

```rust
pub mod regexpreplace { /* ... */ }
```

### Types

#### Struct `RegexpReplaceFunc`

```rust
pub struct RegexpReplaceFunc {
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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Same**
- **IntoEither**
- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Send**
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

- **MaybeSendSync**
### Functions

#### Function `regexp_replace`

Replaces substring(s) matching a PCRE-like regular expression.

The full list of supported features and syntax can be found at
<https://docs.rs/regex/latest/regex/#syntax>

Supported flags with the addition of 'g' can be found at
<https://docs.rs/regex/latest/regex/#grouping-and-flags>

# Examples

```ignore
# use datafusion::prelude::*;
# use datafusion::error::Result;
# #[tokio::main]
# async fn main() -> Result<()> {
let ctx = SessionContext::new();
let df = ctx.read_csv("tests/data/regex.csv", CsvReadOptions::new()).await?;

// use the regexp_replace function to replace substring(s) without flags
let df = df.with_column(
    "a",
    regexp_replace(vec![col("values"), col("patterns"), col("replacement")])
)?;
// use the regexp_replace function to replace substring(s) with flags
let df = df.with_column(
    "b",
    regexp_replace(vec![col("values"), col("patterns"), col("replacement"), col("flags")]),
)?;

// literals can be used as well
let df = df.with_column(
    "c",
    regexp_replace(vec![lit("foobarbequebaz"), lit("(bar)(beque)"), lit(r"\2")]),
)?;

df.show().await?;

# Ok(())
# }
```

```rust
pub fn regexp_replace<''a, T: OffsetSizeTrait, V, B>(string_array: V, pattern_array: B, replacement_array: B, flags: Option<&arrow::array::ArrayRef>) -> datafusion_common::Result<arrow::array::ArrayRef>
where
    V: ArrayAccessor<Item = &''a str>,
    B: ArrayAccessor<Item = &''a str> { /* ... */ }
```

#### Function `specialize_regexp_replace`

Determine which implementation of the regexp_replace to use based
on the given set of arguments.

```rust
pub fn specialize_regexp_replace<T: OffsetSizeTrait>(args: &[datafusion_expr::ColumnarValue]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `expr_fn`

```rust
pub mod expr_fn { /* ... */ }
```

### Functions

#### Function `regexp_count`

Returns the number of consecutive occurrences of a regular expression in a string.

```rust
pub fn regexp_count(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, start: Option<datafusion_expr::Expr>, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regexp_match`

Returns a list of regular expression matches in a string.

```rust
pub fn regexp_match(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regexp_like`

Returns true if a has at least one match in a string, false otherwise.

```rust
pub fn regexp_like(values: datafusion_expr::Expr, regex: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `regexp_replace`

Replaces substrings in a string that match.

```rust
pub fn regexp_replace(string: datafusion_expr::Expr, pattern: datafusion_expr::Expr, replacement: datafusion_expr::Expr, flags: Option<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

### Functions

#### Function `regexp_count`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_count

```rust
pub fn regexp_count() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `regexp_match`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_match

```rust
pub fn regexp_match() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `regexp_like`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_like

```rust
pub fn regexp_like() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `regexp_replace`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of regexp_replace

```rust
pub fn regexp_replace() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `functions`

Returns all DataFusion functions defined in this package

```rust
pub fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

## Module `crypto`

**Attributes:**

- `#[cfg(feature = "crypto_expressions")]`

"crypto" DataFusion functions

```rust
pub mod crypto { /* ... */ }
```

### Modules

## Module `basic`

"crypto" DataFusion functions

```rust
pub mod basic { /* ... */ }
```

### Types

#### Enum `DigestAlgorithm`

```rust
pub enum DigestAlgorithm {
    Md5,
    Sha224,
    Sha256,
    Sha384,
    Sha512,
    Blake2s,
    Blake2b,
    Blake3,
}
```

##### Variants

###### `Md5`

###### `Sha224`

###### `Sha256`

###### `Sha384`

###### `Sha512`

###### `Blake2s`

###### `Blake2b`

###### `Blake3`

##### Implementations

###### Methods

- ```rust
  pub fn digest_scalar(self: Self, value: Option<&[u8]>) -> ColumnarValue { /* ... */ }
  ```
  digest an optional string to its hash value, null values are returned as is

- ```rust
  pub fn digest_binary_array<T>(self: Self, value: &dyn Array) -> Result<ColumnarValue>
where
    T: OffsetSizeTrait { /* ... */ }
  ```
  digest a binary array to their hash values

- ```rust
  pub fn digest_utf8_array<T>(self: Self, value: &dyn Array) -> Result<ColumnarValue>
where
    T: OffsetSizeTrait { /* ... */ }
  ```
  digest a string array to their hash values

- ```rust
  pub fn digest_utf8_array_impl<''a, StringArrType>(self: Self, input_value: StringArrType) -> ArrayRef
where
    StringArrType: StringArrayType<''a> { /* ... */ }
  ```

- ```rust
  pub fn digest_binary_array_impl<''a, BinaryArrType>(self: Self, input_value: BinaryArrType) -> ArrayRef
where
    BinaryArrType: BinaryArrayType<''a> { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **Display**
  - ```rust
    fn fmt(self: &Self, f: &mut fmt::Formatter<''_>) -> fmt::Result { /* ... */ }
    ```

- **Copy**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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
- **MaybeSendSync**
- **UnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **FromStr**
  - ```rust
    fn from_str(name: &str) -> Result<DigestAlgorithm> { /* ... */ }
    ```

- **ErasedDestructor**
- **Freeze**
- **ToOwned**
  - ```rust
    fn to_owned(self: &Self) -> T { /* ... */ }
    ```

  - ```rust
    fn clone_into(self: &Self, target: &mut T) { /* ... */ }
    ```

- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Send**
- **ToString**
  - ```rust
    fn to_string(self: &Self) -> String { /* ... */ }
    ```

- **Same**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DigestAlgorithm { /* ... */ }
    ```

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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Functions

#### Function `sha224`

computes sha224 hash digest of the given input

```rust
pub fn sha224(args: &[ColumnarValue]) -> Result<ColumnarValue> { /* ... */ }
```

#### Function `sha256`

computes sha256 hash digest of the given input

```rust
pub fn sha256(args: &[ColumnarValue]) -> Result<ColumnarValue> { /* ... */ }
```

#### Function `sha384`

computes sha384 hash digest of the given input

```rust
pub fn sha384(args: &[ColumnarValue]) -> Result<ColumnarValue> { /* ... */ }
```

#### Function `sha512`

computes sha512 hash digest of the given input

```rust
pub fn sha512(args: &[ColumnarValue]) -> Result<ColumnarValue> { /* ... */ }
```

#### Function `blake2b`

computes blake2b hash digest of the given input

```rust
pub fn blake2b(args: &[ColumnarValue]) -> Result<ColumnarValue> { /* ... */ }
```

#### Function `blake2s`

computes blake2s hash digest of the given input

```rust
pub fn blake2s(args: &[ColumnarValue]) -> Result<ColumnarValue> { /* ... */ }
```

#### Function `blake3`

computes blake3 hash digest of the given input

```rust
pub fn blake3(args: &[ColumnarValue]) -> Result<ColumnarValue> { /* ... */ }
```

#### Function `digest`

Digest computes a binary hash of the given data, accepts Utf8 or LargeUtf8 and returns a [`ColumnarValue`].
Second argument is the algorithm to use.
Standard algorithms are md5, sha1, sha224, sha256, sha384 and sha512.

```rust
pub fn digest(args: &[datafusion_expr::ColumnarValue]) -> datafusion_common::Result<datafusion_expr::ColumnarValue> { /* ... */ }
```

#### Function `md5`

computes md5 hash digest of the given input

```rust
pub fn md5(args: &[datafusion_expr::ColumnarValue]) -> datafusion_common::Result<datafusion_expr::ColumnarValue> { /* ... */ }
```

#### Function `utf8_or_binary_to_binary_type`

```rust
pub fn utf8_or_binary_to_binary_type(arg_type: &arrow::datatypes::DataType, name: &str) -> datafusion_common::Result<arrow::datatypes::DataType> { /* ... */ }
```

#### Function `digest_process`

```rust
pub fn digest_process(value: &datafusion_expr::ColumnarValue, digest_algorithm: DigestAlgorithm) -> datafusion_common::Result<datafusion_expr::ColumnarValue> { /* ... */ }
```

## Module `digest`

"crypto" DataFusion functions

```rust
pub mod digest { /* ... */ }
```

### Types

#### Struct `DigestFunc`

```rust
pub struct DigestFunc {
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
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

- **Send**
- **Sync**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **ErasedDestructor**
- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **Freeze**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Module `md5`

"crypto" DataFusion functions

```rust
pub mod md5 { /* ... */ }
```

### Types

#### Struct `Md5Func`

```rust
pub struct Md5Func {
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

- **Same**
- **Sync**
- **MaybeSendSync**
- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
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
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **RefUnwindSafe**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `sha224`

"crypto" DataFusion functions

```rust
pub mod sha224 { /* ... */ }
```

### Types

#### Struct `SHA224Func`

```rust
pub struct SHA224Func {
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
- **Same**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **Sync**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Unpin**
- **IntoEither**
- **Freeze**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

## Module `sha256`

"crypto" DataFusion functions

```rust
pub mod sha256 { /* ... */ }
```

### Types

#### Struct `SHA256Func`

```rust
pub struct SHA256Func {
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
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **Sync**
- **RefUnwindSafe**
- **UnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

## Module `sha384`

"crypto" DataFusion functions

```rust
pub mod sha384 { /* ... */ }
```

### Types

#### Struct `SHA384Func`

```rust
pub struct SHA384Func {
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

- **Sync**
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

- **MaybeSendSync**
- **IntoEither**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Same**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
- **ErasedDestructor**
- **Unpin**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

## Module `sha512`

"crypto" DataFusion functions

```rust
pub mod sha512 { /* ... */ }
```

### Types

#### Struct `SHA512Func`

```rust
pub struct SHA512Func {
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
- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **MaybeSendSync**
- **IntoEither**
- **Unpin**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
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

- **UnwindSafe**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

## Module `expr_fn`

```rust
pub mod expr_fn { /* ... */ }
```

### Functions

#### Function `digest`

Computes the binary hash of an expression using the specified algorithm.

```rust
pub fn digest(input_arg1: datafusion_expr::Expr, input_arg2: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `md5`

Computes an MD5 128-bit checksum for a string expression.

```rust
pub fn md5(input_arg: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `sha224`

Computes the SHA-224 hash of a binary string.

```rust
pub fn sha224(input_arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `sha256`

Computes the SHA-256 hash of a binary string.

```rust
pub fn sha256(input_arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `sha384`

Computes the SHA-384 hash of a binary string.

```rust
pub fn sha384(input_arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `sha512`

Computes the SHA-512 hash of a binary string.

```rust
pub fn sha512(input_arg1: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

### Functions

#### Function `digest`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of digest

```rust
pub fn digest() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `md5`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of md5

```rust
pub fn md5() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `sha224`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of sha224

```rust
pub fn sha224() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `sha256`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of sha256

```rust
pub fn sha256() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `sha384`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of sha384

```rust
pub fn sha384() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `sha512`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of sha512

```rust
pub fn sha512() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `functions`

Returns all DataFusion functions defined in this package

```rust
pub fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

## Module `unicode`

**Attributes:**

- `#[cfg(feature = "unicode_expressions")]`

"unicode" DataFusion functions

```rust
pub mod unicode { /* ... */ }
```

### Modules

## Module `character_length`

```rust
pub mod character_length { /* ... */ }
```

### Types

#### Struct `CharacterLengthFunc`

```rust
pub struct CharacterLengthFunc {
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Freeze**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **RefUnwindSafe**
- **Send**
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

- **ErasedDestructor**
- **Unpin**
## Module `find_in_set`

```rust
pub mod find_in_set { /* ... */ }
```

### Types

#### Struct `FindInSetFunc`

```rust
pub struct FindInSetFunc {
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

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Sync**
- **Same**
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
    fn invoke_with_args(self: &Self, args: ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

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

- **Freeze**
- **Unpin**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

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

- **ErasedDestructor**
- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **MaybeSendSync**
### Functions

#### Function `find_in_set_general`

```rust
pub fn find_in_set_general<''a, T, V>(string_array: V, str_list_array: V) -> datafusion_common::Result<arrow::array::ArrayRef>
where
    T: ArrowPrimitiveType,
    <T as >::Native: OffsetSizeTrait,
    V: ArrayAccessor<Item = &''a str> { /* ... */ }
```

## Module `initcap`

```rust
pub mod initcap { /* ... */ }
```

### Types

#### Struct `InitcapFunc`

```rust
pub struct InitcapFunc {
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

- **Freeze**
- **ErasedDestructor**
- **IntoEither**
- **MaybeSendSync**
- **Same**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Sync**
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

- **RefUnwindSafe**
- **UnwindSafe**
## Module `left`

```rust
pub mod left { /* ... */ }
```

### Types

#### Struct `LeftFunc`

```rust
pub struct LeftFunc {
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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Send**
- **Freeze**
- **MaybeSendSync**
- **Sync**
- **Unpin**
- **RefUnwindSafe**
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

- **ErasedDestructor**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Same**
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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

### Functions

#### Function `left`

Returns first n characters in the string, or when n is negative, returns all but last |n| characters.
left('abcde', 2) = 'ab'
The implementation uses UTF-8 code points as characters

```rust
pub fn left<T: OffsetSizeTrait>(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `lpad`

```rust
pub mod lpad { /* ... */ }
```

### Types

#### Struct `LPadFunc`

```rust
pub struct LPadFunc {
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
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
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

- **MaybeSendSync**
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
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
    ```

  - ```rust
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Freeze**
- **ErasedDestructor**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Unpin**
- **UnwindSafe**
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

- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

### Functions

#### Function `lpad`

Extends the string to length 'length' by prepending the characters fill (a space by default).
If the string is already longer than length then it is truncated (on the right).
lpad('hi', 5, 'xy') = 'xyxhi'

```rust
pub fn lpad<T: OffsetSizeTrait>(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `reverse`

```rust
pub mod reverse { /* ... */ }
```

### Types

#### Struct `ReverseFunc`

```rust
pub struct ReverseFunc {
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
    fn return_type(self: &Self, arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn invoke_with_args(self: &Self, args: datafusion_expr::ScalarFunctionArgs<''_>) -> Result<ColumnarValue> { /* ... */ }
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

- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **MaybeSendSync**
- **IntoEither**
- **Same**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **UnwindSafe**
- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **ErasedDestructor**
- **RefUnwindSafe**
### Functions

#### Function `reverse`

Reverses the order of the characters in the string `reverse('abcde') = 'edcba'`.
The implementation uses UTF-8 code points as characters

```rust
pub fn reverse<T: OffsetSizeTrait>(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `right`

```rust
pub mod right { /* ... */ }
```

### Types

#### Struct `RightFunc`

```rust
pub struct RightFunc {
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
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Unpin**
- **Send**
- **Sync**
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

- **Freeze**
- **UnwindSafe**
- **ErasedDestructor**
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
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

### Functions

#### Function `right`

Returns last n characters in the string, or when n is negative, returns all but first |n| characters.
right('abcde', 2) = 'de'
The implementation uses UTF-8 code points as characters

```rust
pub fn right<T: OffsetSizeTrait>(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `rpad`

```rust
pub mod rpad { /* ... */ }
```

### Types

#### Struct `RPadFunc`

```rust
pub struct RPadFunc {
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
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **RefUnwindSafe**
- **ErasedDestructor**
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **UnwindSafe**
- **Freeze**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **IntoEither**
- **Same**
- **MaybeSendSync**
### Functions

#### Function `rpad`

```rust
pub fn rpad<StringArrayLen: OffsetSizeTrait, FillArrayLen: OffsetSizeTrait>(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

#### Function `rpad_impl`

Extends the string to length 'length' by appending the characters fill (a space by default). If the string is already longer than length then it is truncated.
rpad('hi', 5, 'xy') = 'hixyx'

```rust
pub fn rpad_impl<''a, StringArrType, FillArrType, StringArrayLen>(string_array: StringArrType, length_array: &arrow::array::Int64Array, fill_array: Option<FillArrType>) -> datafusion_common::Result<arrow::array::ArrayRef>
where
    StringArrType: StringArrayType<''a>,
    FillArrType: StringArrayType<''a>,
    StringArrayLen: OffsetSizeTrait { /* ... */ }
```

## Module `strpos`

```rust
pub mod strpos { /* ... */ }
```

### Types

#### Struct `StrposFunc`

```rust
pub struct StrposFunc {
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

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Freeze**
- **Send**
- **UnwindSafe**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **IntoEither**
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
    ```

  - ```rust
    fn return_type_from_args(self: &Self, args: datafusion_expr::ReturnTypeArgs<''_>) -> Result<datafusion_expr::ReturnInfo> { /* ... */ }
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

- **RefUnwindSafe**
- **Unpin**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Same**
- **MaybeSendSync**
- **Sync**
## Module `substr`

```rust
pub mod substr { /* ... */ }
```

### Types

#### Struct `SubstrFunc`

```rust
pub struct SubstrFunc {
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

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
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

- **Sync**
- **Freeze**
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
    fn return_type(self: &Self, _arg_types: &[DataType]) -> Result<DataType> { /* ... */ }
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

- **UnwindSafe**
- **MaybeSendSync**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **IntoEither**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Same**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Unpin**
### Functions

#### Function `substr`

Extracts the substring of string starting at the start'th character, and extending for count characters if that is specified. (Same as substring(string from start for count).)
substr('alphabet', 3) = 'phabet'
substr('alphabet', 3, 2) = 'ph'
The implementation uses UTF-8 code points as characters

```rust
pub fn substr(args: &[arrow::array::ArrayRef]) -> datafusion_common::Result<arrow::array::ArrayRef> { /* ... */ }
```

## Module `substrindex`

```rust
pub mod substrindex { /* ... */ }
```

### Types

#### Struct `SubstrIndexFunc`

```rust
pub struct SubstrIndexFunc {
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
- **RefUnwindSafe**
- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

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
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Same**
- **Unpin**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Send**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **IntoEither**
### Functions

#### Function `substr_index_general`

```rust
pub fn substr_index_general<''a, T: ArrowPrimitiveType, V: ArrayAccessor<Item = &''a str>, P: ArrayAccessor<Item = i64>>(string_array: V, delimiter_array: V, count_array: P) -> datafusion_common::Result<arrow::array::ArrayRef>
where
    <T as >::Native: OffsetSizeTrait { /* ... */ }
```

## Module `translate`

```rust
pub mod translate { /* ... */ }
```

### Types

#### Struct `TranslateFunc`

```rust
pub struct TranslateFunc {
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
    fn documentation(self: &Self) -> Option<&Documentation> { /* ... */ }
    ```

- **Unpin**
- **Send**
- **IntoEither**
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

- **RefUnwindSafe**
- **ErasedDestructor**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Freeze**
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
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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
- **Same**
## Module `expr_fn`

```rust
pub mod expr_fn { /* ... */ }
```

### Functions

#### Function `character_length`

the number of characters in the `string`

```rust
pub fn character_length(string: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `lpad`

fill up a string to the length by prepending the characters

```rust
pub fn lpad(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `rpad`

fill up a string to the length by appending the characters

```rust
pub fn rpad(args: Vec<datafusion_expr::Expr>) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `reverse`

reverses the `string`

```rust
pub fn reverse(string: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `substr`

substring from the `position` to the end

```rust
pub fn substr(string: datafusion_expr::Expr, position: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `substr_index`

Returns the substring from str before count occurrences of the delimiter

```rust
pub fn substr_index(string: datafusion_expr::Expr, delimiter: datafusion_expr::Expr, count: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `strpos`

finds the position from where the `substring` matches the `string`

```rust
pub fn strpos(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `substring`

substring from the `position` with `length` characters

```rust
pub fn substring(string: datafusion_expr::Expr, position: datafusion_expr::Expr, length: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `translate`

replaces the characters in `from` with the counterpart in `to`

```rust
pub fn translate(string: datafusion_expr::Expr, from: datafusion_expr::Expr, to: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `right`

returns the last `n` characters in the `string`

```rust
pub fn right(string: datafusion_expr::Expr, n: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `left`

returns the first `n` characters in the `string`

```rust
pub fn left(string: datafusion_expr::Expr, n: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `initcap`

converts the first letter of each word in `string` in uppercase and the remaining characters in lowercase

```rust
pub fn initcap(string: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `find_in_set`

Returns a value in the range of 1 to N if the string `str` is in the string list `strlist` consisting of N substrings

```rust
pub fn find_in_set(string: datafusion_expr::Expr, strlist: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `char_length`

the number of characters in the `string`

```rust
pub fn char_length(string: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `instr`

finds the position from where the `substring` matches the `string`

```rust
pub fn instr(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `length`

the number of characters in the `string`

```rust
pub fn length(string: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

#### Function `position`

finds the position from where the `substring` matches the `string`

```rust
pub fn position(string: datafusion_expr::Expr, substring: datafusion_expr::Expr) -> datafusion_expr::Expr { /* ... */ }
```

### Functions

#### Function `character_length`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of character_length

```rust
pub fn character_length() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `find_in_set`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of find_in_set

```rust
pub fn find_in_set() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `initcap`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of initcap

```rust
pub fn initcap() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `left`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of left

```rust
pub fn left() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `lpad`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of lpad

```rust
pub fn lpad() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `right`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of right

```rust
pub fn right() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `reverse`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of reverse

```rust
pub fn reverse() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `rpad`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of rpad

```rust
pub fn rpad() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `strpos`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of strpos

```rust
pub fn strpos() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `substr`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of substr

```rust
pub fn substr() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `substring`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of substring

```rust
pub fn substring() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `substr_index`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of substr_index

```rust
pub fn substr_index() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `translate`

Return a [`ScalarUDF`](datafusion_expr::ScalarUDF) implementation of translate

```rust
pub fn translate() -> std::sync::Arc<datafusion_expr::ScalarUDF> { /* ... */ }
```

#### Function `functions`

Returns all DataFusion functions defined in this package

```rust
pub fn functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

## Module `planner`

**Attributes:**

- `#[cfg(any(feature = "datetime_expressions", feature = "unicode_expressions"))]`

SQL planning extensions like [`UserDefinedFunctionPlanner`]

```rust
pub mod planner { /* ... */ }
```

### Types

#### Struct `UserDefinedFunctionPlanner`

```rust
pub struct UserDefinedFunctionPlanner;
```

##### Implementations

###### Trait Implementations

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **Unpin**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Allocation**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **ErasedDestructor**
- **Default**
  - ```rust
    fn default() -> UserDefinedFunctionPlanner { /* ... */ }
    ```

- **Sync**
- **Send**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **UnwindSafe**
- **Same**
- **IntoEither**
- **MaybeSendSync**
- **ExprPlanner**
  - ```rust
    fn plan_extract(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
    ```

  - ```rust
    fn plan_position(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
    ```

  - ```rust
    fn plan_substring(self: &Self, args: Vec<Expr>) -> Result<PlannerResult<Vec<Expr>>> { /* ... */ }
    ```

- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

## Module `strings`

```rust
pub mod strings { /* ... */ }
```

### Types

#### Struct `StringArrayBuilder`

Optimized version of the StringBuilder in Arrow that:
1. Precalculating the expected length of the result, avoiding reallocations.
2. Avoids creating / incrementally creating a `NullBufferBuilder`

```rust
pub struct StringArrayBuilder {
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
  pub fn with_capacity(item_capacity: usize, data_capacity: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn write<const CHECK_VALID: bool>(self: &mut Self, column: &ColumnarValueRef<''_>, i: usize) { /* ... */ }
  ```

- ```rust
  pub fn append_offset(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn finish(self: Self, null_buffer: Option<NullBuffer>) -> StringArray { /* ... */ }
  ```
  Finalize the builder into a concrete [`StringArray`].

###### Trait Implementations

- **IntoEither**
- **Allocation**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
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

- **Unpin**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
- **Same**
- **UnwindSafe**
- **RefUnwindSafe**
- **Sync**
- **Freeze**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **ErasedDestructor**
#### Struct `StringViewArrayBuilder`

```rust
pub struct StringViewArrayBuilder {
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
  pub fn with_capacity(_item_capacity: usize, data_capacity: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn write<const CHECK_VALID: bool>(self: &mut Self, column: &ColumnarValueRef<''_>, i: usize) { /* ... */ }
  ```

- ```rust
  pub fn append_offset(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn finish(self: Self) -> StringViewArray { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
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

- **Same**
- **IntoEither**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **Send**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **ErasedDestructor**
- **Unpin**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **MaybeSendSync**
- **Sync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **UnwindSafe**
#### Struct `LargeStringArrayBuilder`

```rust
pub struct LargeStringArrayBuilder {
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
  pub fn with_capacity(item_capacity: usize, data_capacity: usize) -> Self { /* ... */ }
  ```

- ```rust
  pub fn write<const CHECK_VALID: bool>(self: &mut Self, column: &ColumnarValueRef<''_>, i: usize) { /* ... */ }
  ```

- ```rust
  pub fn append_offset(self: &mut Self) { /* ... */ }
  ```

- ```rust
  pub fn finish(self: Self, null_buffer: Option<NullBuffer>) -> LargeStringArray { /* ... */ }
  ```
  Finalize the builder into a concrete [`LargeStringArray`].

###### Trait Implementations

- **ErasedDestructor**
- **Unpin**
- **Freeze**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Same**
- **IntoEither**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **Allocation**
- **RefUnwindSafe**
- **MaybeSendSync**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Send**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

#### Enum `ColumnarValueRef`

```rust
pub enum ColumnarValueRef<''a> {
    Scalar(&''a [u8]),
    NullableArray(&''a arrow::array::StringArray),
    NonNullableArray(&''a arrow::array::StringArray),
    NullableLargeStringArray(&''a arrow::array::LargeStringArray),
    NonNullableLargeStringArray(&''a arrow::array::LargeStringArray),
    NullableStringViewArray(&''a arrow::array::StringViewArray),
    NonNullableStringViewArray(&''a arrow::array::StringViewArray),
}
```

##### Variants

###### `Scalar`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a [u8]` |  |

###### `NullableArray`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a arrow::array::StringArray` |  |

###### `NonNullableArray`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a arrow::array::StringArray` |  |

###### `NullableLargeStringArray`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a arrow::array::LargeStringArray` |  |

###### `NonNullableLargeStringArray`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a arrow::array::LargeStringArray` |  |

###### `NullableStringViewArray`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a arrow::array::StringViewArray` |  |

###### `NonNullableStringViewArray`

Fields:

| Index | Type | Documentation |
|-------|------|---------------|
| 0 | `&''a arrow::array::StringViewArray` |  |

##### Implementations

###### Methods

- ```rust
  pub fn is_valid(self: &Self, i: usize) -> bool { /* ... */ }
  ```

- ```rust
  pub fn nulls(self: &Self) -> Option<NullBuffer> { /* ... */ }
  ```

###### Trait Implementations

- **Allocation**
- **VZip**
  - ```rust
    fn vzip(self: Self) -> V { /* ... */ }
    ```

- **RefUnwindSafe**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **MaybeSendSync**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **ErasedDestructor**
- **IntoEither**
- **Same**
- **Sync**
- **Freeze**
- **Unpin**
- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

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

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

### Traits

#### Trait `StringArrayType`

**Attributes:**

- `#[deprecated(since = "45.0.0", note =
"Use arrow::array::StringArrayType instead")]`

**⚠️ Deprecated since 45.0.0**: Use arrow::array::StringArrayType instead

Abstracts iteration over different types of string arrays.

```rust
pub trait StringArrayType<''a>: ArrayAccessor<Item = &''a str> + Sized {
    /* Associated items */
}
```

> This trait is not object-safe and cannot be used in dynamic trait objects.

##### Required Items

###### Required Methods

- `iter`: Return an [`ArrayIter`]  over the values of the array.
- `is_ascii`: Check if the array is ASCII only.

##### Implementations

This trait is implemented for the following types:

- `&''a arrow::array::GenericStringArray<T>` with <''a, T: OffsetSizeTrait>
- `&''a arrow::array::StringViewArray` with <''a>

### Functions

#### Function `make_and_append_view`

Append a new view to the views buffer with the given substr

# Safety

original_view must be a valid view (the format described on
[`GenericByteViewArray`](arrow::array::GenericByteViewArray).

# Arguments
- views_buffer: The buffer to append the new view to
- null_builder: The buffer to append the null value to
- original_view: The original view value
- substr: The substring to append. Must be a valid substring of the original view
- start_offset: The start offset of the substring in the view

```rust
pub fn make_and_append_view(views_buffer: &mut Vec<u128>, null_builder: &mut arrow::array::NullBufferBuilder, original_view: &u128, substr: &str, start_offset: u32) { /* ... */ }
```

## Module `utils`

```rust
pub mod utils { /* ... */ }
```

## Module `expr_fn`

Fluent-style API for creating `Expr`s

```rust
pub mod expr_fn { /* ... */ }
```

### Re-exports

#### Re-export `super::core::expr_fn::*`

```rust
pub use super::core::expr_fn::*;
```

#### Re-export `super::crypto::expr_fn::*`

**Attributes:**

- `#[cfg(feature = "crypto_expressions")]`

```rust
pub use super::crypto::expr_fn::*;
```

#### Re-export `super::datetime::expr_fn::*`

**Attributes:**

- `#[cfg(feature = "datetime_expressions")]`

```rust
pub use super::datetime::expr_fn::*;
```

#### Re-export `super::encoding::expr_fn::*`

**Attributes:**

- `#[cfg(feature = "encoding_expressions")]`

```rust
pub use super::encoding::expr_fn::*;
```

#### Re-export `super::math::expr_fn::*`

**Attributes:**

- `#[cfg(feature = "math_expressions")]`

```rust
pub use super::math::expr_fn::*;
```

#### Re-export `super::regex::expr_fn::*`

**Attributes:**

- `#[cfg(feature = "regex_expressions")]`

```rust
pub use super::regex::expr_fn::*;
```

#### Re-export `super::string::expr_fn::*`

**Attributes:**

- `#[cfg(feature = "string_expressions")]`

```rust
pub use super::string::expr_fn::*;
```

#### Re-export `super::unicode::expr_fn::*`

**Attributes:**

- `#[cfg(feature = "unicode_expressions")]`

```rust
pub use super::unicode::expr_fn::*;
```

## Functions

### Function `all_default_functions`

Return all default functions

```rust
pub fn all_default_functions() -> Vec<std::sync::Arc<datafusion_expr::ScalarUDF>> { /* ... */ }
```

### Function `register_all`

Registers all enabled packages with a [`FunctionRegistry`]

```rust
pub fn register_all(registry: &mut dyn FunctionRegistry) -> datafusion_common::Result<()> { /* ... */ }
```

## Macros

### Macro `downcast_named_arg`

**Attributes:**

- `#[macro_export]`

Downcast a named argument to a specific array type, returning an internal error
if the cast fails

$ARG: ArrayRef
$NAME: name of the argument (for error messages)
$ARRAY_TYPE: the type of array to cast the argument to

```rust
pub macro_rules! downcast_named_arg {
    /* macro_rules! downcast_named_arg {
    ($ARG:expr, $NAME:expr, $ARRAY_TYPE:ident) => { ... };
} */
}
```

### Macro `downcast_arg`

**Attributes:**

- `#[macro_export]`

Downcast an argument to a specific array type, returning an internal error
if the cast fails

$ARG: ArrayRef
$ARRAY_TYPE: the type of array to cast the argument to

```rust
pub macro_rules! downcast_arg {
    /* macro_rules! downcast_arg {
    ($ARG:expr, $ARRAY_TYPE:ident) => { ... };
} */
}
```


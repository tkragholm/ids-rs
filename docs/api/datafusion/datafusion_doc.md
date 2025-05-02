# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_doc`

## Types

### Struct `Documentation`

**Attributes:**

- `#[allow(rustdoc::broken_intra_doc_links)]`

Documentation for use by [`ScalarUDFImpl`](ScalarUDFImpl),
[`AggregateUDFImpl`](AggregateUDFImpl) and [`WindowUDFImpl`](WindowUDFImpl) functions.

See the [`DocumentationBuilder`] to create a new [`Documentation`] struct.

The DataFusion [SQL function documentation] is automatically  generated from these structs.
The name of the udf will be pulled from the [`ScalarUDFImpl::name`](ScalarUDFImpl::name),
[`AggregateUDFImpl::name`](AggregateUDFImpl::name) or [`WindowUDFImpl::name`](WindowUDFImpl::name)
function as appropriate.

All strings in the documentation are required to be
in [markdown format](https://www.markdownguide.org/basic-syntax/).

Currently, documentation only supports a single language
thus all text should be in English.

[SQL function documentation]: https://datafusion.apache.org/user-guide/sql/index.html

```rust
pub struct Documentation {
    pub doc_section: DocSection,
    pub description: String,
    pub syntax_example: String,
    pub sql_example: Option<String>,
    pub arguments: Option<Vec<(String, String)>>,
    pub alternative_syntax: Option<Vec<String>>,
    pub related_udfs: Option<Vec<String>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `doc_section` | `DocSection` | The section in the documentation where the UDF will be documented |
| `description` | `String` | The description for the UDF |
| `syntax_example` | `String` | A brief example of the syntax. For example "ascii(str)" |
| `sql_example` | `Option<String>` | A sql example for the UDF, usually in the form of a sql prompt<br>query and output. It is strongly recommended to provide an<br>example for anything but the most basic UDF's |
| `arguments` | `Option<Vec<(String, String)>>` | Arguments for the UDF which will be displayed in array order.<br>Left member of a pair is the argument name, right is a<br>description for the argument |
| `alternative_syntax` | `Option<Vec<String>>` | A list of alternative syntax examples for a function |
| `related_udfs` | `Option<Vec<String>>` | Related functions if any. Values should match the related<br>udf's name exactly. Related udf's must be of the same<br>UDF type (scalar, aggregate or window) for proper linking to<br>occur |

#### Implementations

##### Methods

- ```rust
  pub fn builder</* synthetic */ impl Into<String>: Into<String>, /* synthetic */ impl Into<String>: Into<String>>(doc_section: DocSection, description: impl Into<String>, syntax_example: impl Into<String>) -> DocumentationBuilder { /* ... */ }
  ```
  Returns a new [`DocumentationBuilder`] with no options set.

- ```rust
  pub fn to_doc_attribute(self: &Self) -> String { /* ... */ }
  ```
  Output the `Documentation` struct in form of custom Rust documentation attributes

##### Trait Implementations

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
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
    ```

- **RefUnwindSafe**
- **UnwindSafe**
- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **Freeze**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> Documentation { /* ... */ }
    ```

- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

### Struct `DocSection`

```rust
pub struct DocSection {
    pub include: bool,
    pub label: &''static str,
    pub description: Option<&''static str>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `include` | `bool` | True to include this doc section in the public<br>documentation, false otherwise |
| `label` | `&''static str` | A display label for the doc section. For example: "Math Expressions" |
| `description` | `Option<&''static str>` | An optional description for the doc section |

#### Implementations

##### Trait Implementations

- **UnwindSafe**
- **Clone**
  - ```rust
    fn clone(self: &Self) -> DocSection { /* ... */ }
    ```

- **StructuralPartialEq**
- **RefUnwindSafe**
- **CloneToUninit**
  - ```rust
    unsafe fn clone_to_uninit(self: &Self, dst: *mut u8) { /* ... */ }
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

- **Send**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **BorrowMut**
  - ```rust
    fn borrow_mut(self: &mut Self) -> &mut T { /* ... */ }
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

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

- **PartialEq**
  - ```rust
    fn eq(self: &Self, other: &DocSection) -> bool { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Default**
  - ```rust
    fn default() -> Self { /* ... */ }
    ```
    Returns a "default" Doc section.

- **Sync**
- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **Freeze**
### Struct `DocumentationBuilder`

A builder for [`Documentation`]'s.

Example:

```rust

# fn main() {
    use datafusion_doc::{DocSection, Documentation};
    let doc_section = DocSection {
        include: true,
        label: "Display Label",
        description: None,
    };

    let documentation = Documentation::builder(doc_section, "Add one to an int32".to_owned(), "add_one(2)".to_owned())
          .with_argument("arg_1", "The int32 number to add one to")
          .build();
# }

```rust
pub struct DocumentationBuilder {
    pub doc_section: DocSection,
    pub description: String,
    pub syntax_example: String,
    pub sql_example: Option<String>,
    pub arguments: Option<Vec<(String, String)>>,
    pub alternative_syntax: Option<Vec<String>>,
    pub related_udfs: Option<Vec<String>>,
}
```

#### Fields

| Name | Type | Documentation |
|------|------|---------------|
| `doc_section` | `DocSection` |  |
| `description` | `String` |  |
| `syntax_example` | `String` |  |
| `sql_example` | `Option<String>` |  |
| `arguments` | `Option<Vec<(String, String)>>` |  |
| `alternative_syntax` | `Option<Vec<String>>` |  |
| `related_udfs` | `Option<Vec<String>>` |  |

#### Implementations

##### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

- ```rust
  pub fn new_with_details</* synthetic */ impl Into<String>: Into<String>, /* synthetic */ impl Into<String>: Into<String>>(doc_section: DocSection, description: impl Into<String>, syntax_example: impl Into<String>) -> Self { /* ... */ }
  ```
  Creates a new [`DocumentationBuilder`] with all required fields

- ```rust
  pub fn with_doc_section(self: Self, doc_section: DocSection) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_description</* synthetic */ impl Into<String>: Into<String>>(self: Self, description: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_syntax_example</* synthetic */ impl Into<String>: Into<String>>(self: Self, syntax_example: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_sql_example</* synthetic */ impl Into<String>: Into<String>>(self: Self, sql_example: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_argument</* synthetic */ impl Into<String>: Into<String>, /* synthetic */ impl Into<String>: Into<String>>(self: Self, arg_name: impl Into<String>, arg_description: impl Into<String>) -> Self { /* ... */ }
  ```
  Adds documentation for a specific argument to the documentation.

- ```rust
  pub fn with_standard_argument</* synthetic */ impl Into<String>: Into<String>>(self: Self, arg_name: impl Into<String>, expression_type: Option<&str>) -> Self { /* ... */ }
  ```
  Add a standard "expression" argument to the documentation

- ```rust
  pub fn with_alternative_syntax</* synthetic */ impl Into<String>: Into<String>>(self: Self, syntax_name: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn with_related_udf</* synthetic */ impl Into<String>: Into<String>>(self: Self, related_udf: impl Into<String>) -> Self { /* ... */ }
  ```

- ```rust
  pub fn build(self: Self) -> Documentation { /* ... */ }
  ```
  Build the documentation from provided components

##### Trait Implementations

- **RefUnwindSafe**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **Send**
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

- **Sync**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **Freeze**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Unpin**
- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```


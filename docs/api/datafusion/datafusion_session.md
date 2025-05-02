# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_session`

Session management for DataFusion query execution environment

This module provides the core session management functionality for DataFusion,
handling both Catalog (Table) and Datasource (File) configurations. It defines
the fundamental interfaces and implementations for maintaining query execution
state and configurations.

Key components:

- [`Session`] - Manages query execution context, including configurations,
  catalogs, and runtime state
- [`SessionStore`] - Handles session persistence and retrieval

The session system enables:

- Configuration management for query execution
- Catalog and schema management
- Function registry access
- Runtime environment configuration
- Query state persistence

## Modules

## Module `session`

```rust
pub mod session { /* ... */ }
```

### Types

#### Struct `SessionStore`

The state store that stores the reference of the runtime session state.

```rust
pub struct SessionStore {
    // Some fields omitted
}
```

##### Fields

| Name             | Type | Documentation                   |
| ---------------- | ---- | ------------------------------- |
| _private fields_ | ...  | _Some fields have been omitted_ |

##### Implementations

###### Methods

- ```rust
  pub fn new() -> Self { /* ... */ }
  ```

  Create a new [SessionStore]

- ```rust
  pub fn with_state(self: &Self, state: Weak<RwLock<dyn Session>>) { /* ... */ }
  ```

  Set the session state of the store

- ```rust
  pub fn get_session(self: &Self) -> Weak<RwLock<dyn Session>> { /* ... */ }
  ```
  Get the current session of the store

###### Trait Implementations

- **From**

  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **Freeze**
- **MaybeSendSync**
- **TryFrom**

  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
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

- **Send**
- **Borrow**

  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

- **RefUnwindSafe**
- **ErasedDestructor**
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
- **Sync**
- **UnwindSafe**
- **TryInto**

  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
    ```

- **Debug**
  - ```rust
    fn fmt(self: &Self, f: &mut $crate::fmt::Formatter<''_>) -> $crate::fmt::Result { /* ... */ }
    ```

### Traits

#### Trait `Session`

Interface for accessing [`SessionState`] from the catalog and data source.

This trait provides access to the information needed to plan and execute
queries, such as configuration, functions, and runtime environment. See the
documentation on [`SessionState`] for more information.

Historically, the `SessionState` struct was passed directly to catalog
traits such as [`TableProvider`], which required a direct dependency on the
DataFusion core. The interface required is now defined by this trait. See
[#10782] for more details.

[#10782]: https://github.com/apache/datafusion/issues/10782

# Migration from `SessionState`

Using trait methods is preferred, as the implementation may change in future
versions. However, you can downcast a `Session` to a `SessionState` as shown
in the example below. If you find yourself needing to do this, please open
an issue on the DataFusion repository so we can extend the trait to provide
the required information.

```
# use datafusion_session::Session;
# use datafusion_common::{Result, exec_datafusion_err};
# struct SessionState {}
// Given a `Session` reference, get the concrete `SessionState` reference
// Note: this may stop working in future versions,
fn session_state_from_session(session: &dyn Session) -> Result<&SessionState> {
   session.as_any()
    .downcast_ref::<SessionState>()
    .ok_or_else(|| exec_datafusion_err!("Failed to downcast Session to SessionState"))
}
```

[`SessionState`]: https://docs.rs/datafusion/latest/datafusion/execution/session_state/struct.SessionState.html
[`TableProvider`]: https://docs.rs/datafusion/latest/datafusion/catalog/trait.TableProvider.html

```rust
pub trait Session: Send + Sync {
    /* Associated items */
}
```

##### Required Items

###### Required Methods

- `session_id`: Return the session ID
- `config`: Return the [`SessionConfig`]
- `create_physical_plan`: Creates a physical [`ExecutionPlan`] plan from a [`LogicalPlan`].
- `create_physical_expr`: Create a [`PhysicalExpr`] from an [`Expr`] after applying type
- `scalar_functions`: Return reference to scalar_functions
- `aggregate_functions`: Return reference to aggregate_functions
- `window_functions`: Return reference to window functions
- `runtime_env`: Return the runtime env
- `execution_props`: Return the execution properties
- `as_any`
- `table_options`: Return the table options
- `table_options_mut`: Returns a mutable reference to [`TableOptions`]
- `task_ctx`: Get a new TaskContext to run in this session

##### Provided Methods

- ```rust
  fn config_options(self: &Self) -> &ConfigOptions { /* ... */ }
  ```

  return the [`ConfigOptions`]

- ```rust
  fn default_table_options(self: &Self) -> TableOptions { /* ... */ }
  ```
  return the TableOptions options with its extensions

## Re-exports

### Re-export `Session`

```rust
pub use crate::session::Session;
```

### Re-export `SessionStore`

```rust
pub use crate::session::SessionStore;
```

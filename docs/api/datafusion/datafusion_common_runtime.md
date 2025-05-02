# Crate Documentation

**Version:** 47.0.0

**Format Version:** 39

# Module `datafusion_common_runtime`

## Modules

## Module `common`

```rust
pub mod common { /* ... */ }
```

### Types

#### Struct `SpawnedTask`

Helper that  provides a simple API to spawn a single task and join it.
Provides guarantees of aborting on `Drop` to keep it cancel-safe.
Note that if the task was spawned with `spawn_blocking`, it will only be
aborted if it hasn't started yet.

Technically, it's just a wrapper of a `JoinHandle` overriding drop.

```rust
pub struct SpawnedTask<R> {
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
  pub fn spawn<T>(task: T) -> Self
where
    T: Future<Output = R> + Send + ''static,
    R: Send { /* ... */ }
  ```

- ```rust
  pub fn spawn_blocking<T>(task: T) -> Self
where
    T: FnOnce() -> R + Send + ''static,
    R: Send { /* ... */ }
  ```

- ```rust
  pub async fn join(self: Self) -> Result<R, JoinError> { /* ... */ }
  ```
  Joins the task, returning the result of join (`Result<R, JoinError>`).

- ```rust
  pub async fn join_unwind(self: Self) -> Result<R, JoinError> { /* ... */ }
  ```
  Joins the task and unwinds the panic if it happens.

###### Trait Implementations

- **Sync**
- **TryInto**
  - ```rust
    fn try_into(self: Self) -> Result<U, <U as TryFrom<T>>::Error> { /* ... */ }
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

- **IntoFuture**
  - ```rust
    fn into_future(self: Self) -> <F as IntoFuture>::IntoFuture { /* ... */ }
    ```

- **TryFuture**
  - ```rust
    fn try_poll(self: Pin<&mut F>, cx: &mut Context<''_>) -> Poll<<F as Future>::Output> { /* ... */ }
    ```

- **FutureExt**
- **Any**
  - ```rust
    fn type_id(self: &Self) -> TypeId { /* ... */ }
    ```

- **TryFrom**
  - ```rust
    fn try_from(value: U) -> Result<T, <T as TryFrom<U>>::Error> { /* ... */ }
    ```

- **Unpin**
- **Future**
  - ```rust
    fn poll(self: Pin<&mut Self>, cx: &mut Context<''_>) -> Poll<<Self as >::Output> { /* ... */ }
    ```

- **Into**
  - ```rust
    fn into(self: Self) -> U { /* ... */ }
    ```
    Calls `U::from(self)`.

- **TryFutureExt**
- **From**
  - ```rust
    fn from(t: T) -> T { /* ... */ }
    ```
    Returns the argument unchanged.

- **UnwindSafe**
- **Drop**
  - ```rust
    fn drop(self: &mut Self) { /* ... */ }
    ```

- **Send**
- **Freeze**
- **Borrow**
  - ```rust
    fn borrow(self: &Self) -> &T { /* ... */ }
    ```

## Re-exports

### Re-export `SpawnedTask`

```rust
pub use common::SpawnedTask;
```

### Re-export `JoinSet`

```rust
pub use join_set::JoinSet;
```

### Re-export `set_join_set_tracer`

```rust
pub use trace_utils::set_join_set_tracer;
```

### Re-export `JoinSetTracer`

```rust
pub use trace_utils::JoinSetTracer;
```


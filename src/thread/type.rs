use crate::*;
use std::{any::Any, sync::Arc};
use tokio::task::JoinError;

/// SpawnError
pub type SpawnError = Box<dyn Any + Send>;

/// Type alias for the result type returned by spawnable functions.
///
/// - `Ok(())`: Indicates successful execution of the function.
/// - `Err(JoinError)`: Contains a error value in case of a panic or failure.
pub type AsyncSpawnResult = Result<(), JoinError>;

/// Type alias for the result type returned by spawnable functions.
///
/// - `Ok(())`: Indicates successful execution of the function.
/// - `Err(SpawnError)`: Contains a boxed error value in case of a panic or failure.
pub type SyncSpawnResult = Result<(), SpawnError>;

/// Alias for an `Arc`-wrapped recoverable function.
///
/// - This type represents an `Arc`-wrapped version of any function implementing the `AsyncRecoverableFunction` trait.
/// - Enables shared ownership and thread-safe usage of recoverable functions in concurrent environments.
pub type ArcAsyncRecoverableFunction<O, F> =
    Arc<dyn AsyncRecoverableFunction<Output = O, Future = F>>;

/// Alias for an `Arc`-wrapped error handler function.
///
/// - This type represents an `Arc`-wrapped version of any function implementing the `AsyncErrorHandlerFunction` trait.
/// - Allows shared ownership and thread-safe handling of errors with custom logic across multiple threads.
pub type ArcAsyncErrorHandlerFunction<O> = Arc<dyn AsyncErrorHandlerFunction<Future = O>>;

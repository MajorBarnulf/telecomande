/// Autotrait that constraints the commands that can be sent to a [`crate::Processor`].
/// not ment to be implemented manually
pub trait Command: Send + Sync + 'static {}
impl<T> Command for T where T: Send + Sync + 'static {}

/// Autotrait that constraints the kind of errors that can be returned by a [`crate::Processor`].
/// not ment to be implemented manually
pub trait Error: Send + 'static {}
impl<T> Error for T where T: Send + 'static {}

/// Represents the handling of [`crate::Command`] in a task.
/// consumed by a [`crate::Executor`].
#[async_trait::async_trait]
pub trait Processor: Send + Sync + 'static {
    /// The type of [`crate::Command`] to handle, sent by a [`crate::Remote`].
    type Command: crate::Command;

    /// The [`Error`] that may be thrown during the handling of a [`crate::Command`].
    type Error: crate::Error;

    /// Method called on each received [`crate::Command`].
    /// Values may be returned through the use of [`tokio::sync::oneshot`] channels.
    /// May need the `#[telecomande::async_trait]` macro over the implementation block in order to satisfy the compiler.
    async fn handle(&mut self, command: Self::Command) -> Result<(), Self::Error>;
}

pub use crate::executor::Executor;

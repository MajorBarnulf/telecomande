/// Autotrait that constraints the signals that can be sent to a [`crate::traits::Manager`].
/// not ment to be implemented manually
pub trait Signal: Send + 'static {}
impl<T> Signal for T where T: Send + 'static {}

/// Represents the handling of [`crate::traits::Signal`] in a task.
/// consumed by the [`crate::utils::spawn`] function.
#[async_trait::async_trait]
pub trait Manager: Send + Sync + 'static {
    type Signal: Signal;

    /// Method called on each received [`crate::traits::Signal`].
    /// Values may be returned through the use of [`tokio::sync::oneshot`] channels.
    /// May need the `#[telecomande::async_trait]` macro over the implementation block in order to satisfy the compiler.
    async fn handle(&mut self, signal: Self::Signal);
}

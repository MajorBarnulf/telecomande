use tokio::sync::mpsc;

use crate::{inner::Inner, Handle, Manager};

/// Spawns a task using the provided [`crate::Manager`] and returns a [`crate::Handle`] to that task.
/// [`crate::Signal`] can be sent through a [`crate::Remote`] that can be provided by the [`crate::Handle`].
pub fn spawn<M>(manager: M) -> Handle<M>
where
    M: Manager,
{
    let (sender, receiver) = mpsc::unbounded_channel();
    let task = tokio::spawn(async move {
        let mut inner = Inner::new(manager);
        inner.listen(receiver).await;
    });
    Handle::new(task, sender)
}

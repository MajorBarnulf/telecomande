use tokio::{
    sync::mpsc,
    task::{JoinError, JoinHandle},
};

use crate::{Manager, Remote};

/// A handle to a task running a [`crate::traits::Manager`].
/// - Warning: the manager stops operating if this is dropped.
/// Permit the creation of [`crate::remote::Remote`].
pub struct Handle<M>
where
    M: Manager,
{
    task: JoinHandle<()>,
    sender: mpsc::UnboundedSender<M::Signal>,
}

impl<M> Handle<M>
where
    M: Manager,
{
    pub(crate) fn new(task: JoinHandle<()>, sender: mpsc::UnboundedSender<M::Signal>) -> Self {
        Self { task, sender }
    }

    /// Will wait -possibly endlessly- for the task to finish.
    /// Prevents the task from being dropped before the current thread.
    pub async fn join(self) -> Result<(), JoinError> {
        let Self { task, sender: _ } = self;
        task.await
    }

    /// Returns a [`crate::remote::Remote`] able to send [`crate::traits::Signal`] to the [`crate::traits::Manager`] of the running task.
    pub fn remote(&self) -> Remote<M> {
        let sender = self.sender.clone();
        Remote::new(sender)
    }
}

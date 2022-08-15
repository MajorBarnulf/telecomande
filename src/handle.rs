use tokio::{
    sync::mpsc,
    task::{JoinError, JoinHandle},
};

use crate::{remote::Remote, traits::Processor};

/// A handle to a task running a [`crate::Processor`].
/// - Warning: the manager stops operating if this is dropped.
/// Permit the creation of [`crate::Remote`].
pub struct Handle<P>
where
    P: Processor,
{
    task: JoinHandle<P::Error>,
    sender: mpsc::UnboundedSender<P::Command>,
}

impl<P> Handle<P>
where
    P: Processor,
{
    pub fn new(task: JoinHandle<P::Error>, sender: mpsc::UnboundedSender<P::Command>) -> Self {
        Self { task, sender }
    }

    /// Will wait -possibly endlessly- for the task to finish.
    /// Prevents the task from being dropped before the current thread.
    pub async fn join(self) -> Result<P::Error, JoinError> {
        let Self { task, sender: _ } = self;
        task.await
    }

    pub fn abort(self) {
        let Self { task, sender: _ } = self;
        task.abort();
    }

    /// Returns a [`crate::Remote`] able to send [`crate::Command`] to the [`crate::Processor`] of the running task.
    pub fn remote(&self) -> Remote<P> {
        let sender = self.sender.clone();
        Remote::new(sender)
    }
}

use tokio::sync::mpsc;

use crate::Manager;

/// Represents a shareable/clonable remote to a task running a [`crate::traits::Manager`].
/// Used to send [`crate::traits::Signal`] to that task.
/// Initially constructed by the [`crate::handle::Handle`] returned on the [`crate::utils::spawn`] of the task.
#[derive(Clone)]
pub struct Remote<M>
where
    M: Manager,
{
    sender: mpsc::UnboundedSender<M::Signal>,
}

impl<M> Remote<M>
where
    M: Manager,
{
    pub(crate) fn new(sender: mpsc::UnboundedSender<M::Signal>) -> Self {
        Self { sender }
    }

    /// Main method, sends a [`crate::traits::Signal`] to the running [`crate::traits::Manager`].
    pub fn send(&self, signal: M::Signal) -> Result<(), mpsc::error::SendError<M::Signal>> {
        self.sender.send(signal)
    }
}

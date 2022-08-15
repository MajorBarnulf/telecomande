use tokio::sync::mpsc::{self, error::SendError};

use crate::traits::Processor;

/// Represents a shareable/clonable remote to a task running a [`crate::Processor`].
/// Used to send [`crate::Command`] to that task.
/// Initially constructed by the [`crate::Handle`] returned on the [`crate::Executor::spawn`] of the task.
#[derive(Clone)]
pub struct Remote<P>
where
    P: Processor,
{
    sender: mpsc::UnboundedSender<P::Command>,
}

impl<P> Remote<P>
where
    P: Processor,
{
    pub fn new(sender: mpsc::UnboundedSender<P::Command>) -> Self {
        Self { sender }
    }

    /// Sends a [`crate::Command`] to the running [`crate::Processor`] for it to handle.
    pub fn send(&self, command: P::Command) -> Result<(), SendError<P::Command>> {
        self.sender.send(command)
    }
}

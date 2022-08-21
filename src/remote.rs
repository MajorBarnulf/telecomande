use std::fmt::Debug;

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

    /// Returns a [`crate::Remote`] able to send [`crate::Command`] to the [`crate::Processor`] of the running task.
    pub fn remote(&self) -> Self {
        Self::new(self.sender.clone())
    }
}

impl<P> Debug for Remote<P>
where
    P: Processor,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Remote").field("sender", &"...").finish()
    }
}

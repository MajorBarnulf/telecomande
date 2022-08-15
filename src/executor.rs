use tokio::sync::mpsc;

use crate::{handle::Handle, Processor};

pub mod simple;

/// Internal data of a task running and handling incomming [`crate::Command`] for a [`crate::Processor`].
#[async_trait::async_trait]
pub trait Executor<P>: Send + 'static
where
    P: Processor,
{
    /// Returns if all [`crate::Remote`] and the [`crate::Handle`] have been dropped.
    async fn run(&mut self, receiver: mpsc::UnboundedReceiver<P::Command>) -> Result<(), P::Error>;

    /// Creates a tokio task in which an [`crate::Executor`] listen for incoming [`crate::Command`].
    fn spawn(self) -> Handle<P>
    where
        Self: Sized,
    {
        let (sender, receiver) = mpsc::unbounded_channel();
        let task = tokio::spawn(async move {
            let mut executor = self;
            executor.run(receiver).await.err().unwrap()
        });
        Handle::new(task, sender)
    }
}

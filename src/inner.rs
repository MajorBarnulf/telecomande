use tokio::sync::mpsc;

use crate::Manager;

/// Internal data of a task running a [`crate::traits::Manager`].
pub(crate) struct Inner<M>
where
    M: Manager,
{
    manager: M,
}

impl<M> Inner<M>
where
    M: Manager,
{
    pub(crate) fn new(manager: M) -> Self {
        Self { manager }
    }

    /// Returns if all [`crate::remote::Remote`] and the [`crate::handle::Handle`] have been dropped.
    pub(crate) async fn listen(
        &mut self,
        mut receiver: mpsc::UnboundedReceiver<M::Signal>,
    ) -> Option<()> {
        loop {
            let signal = receiver.recv().await?;
            self.manager.handle(signal).await;
        }
    }
}

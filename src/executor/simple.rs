use tokio::sync::mpsc;

use crate::traits::Processor;

use super::Executor;

/// Simple implementation of [`crate::Executor`], process incomming [`crate::Command`]s one by one.
pub struct SimpleExecutor<P>
where
    P: Processor,
{
    processor: P,
}

impl<P> SimpleExecutor<P>
where
    P: Processor,
{
    /// Constructor, takes a [`crate::Processor`] that will be used to handle incomming [`crate::Command`].
    pub fn new(processor: P) -> Self {
        Self { processor }
    }
}

#[async_trait::async_trait]
impl<P> Executor<P> for SimpleExecutor<P>
where
    P: Processor,
{
    async fn run(&mut self, receiver: mpsc::UnboundedReceiver<P::Command>) -> Result<(), P::Error> {
        let mut receiver = receiver;
        loop {
            let command = receiver.recv().await.unwrap();
            self.processor.handle(command).await?;
        }
    }
}

impl<P> From<P> for SimpleExecutor<P>
where
    P: Processor,
{
    fn from(input: P) -> Self {
        Self::new(input)
    }
}

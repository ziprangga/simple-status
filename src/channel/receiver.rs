use crate::status::Status;
use std::sync::Arc;

use super::BoxFuture;
use super::BoxStream;

use futures::stream;

// trait for Receiver
pub trait ReceiverHandler: Send + Sync {
    fn try_recv(&self) -> Option<Status>;
    fn recv(&self) -> BoxFuture<'_, Option<Status>>;
    fn stream(&self) -> BoxStream<'_, Status>;
}

#[derive(Clone)]
pub struct Receiver {
    receiver: Arc<dyn ReceiverHandler>,
}

impl Receiver {
    pub fn new(receiver: Arc<dyn ReceiverHandler>) -> Self {
        Self { receiver }
    }

    pub fn sync_recv(&self) -> Option<Status> {
        self.receiver.try_recv()
    }

    pub async fn async_recv(&self) -> Option<Status> {
        self.receiver.recv().await
    }

    pub fn stream(&self) -> Option<BoxStream<'static, Status>> {
        let this = self.clone();
        let s = stream::unfold(this, |res| async move {
            let status = res.async_recv().await?;
            Some((status, res))
        });

        Some(Box::pin(s) as BoxStream<'static, Status>)
    }
}

impl<T: ReceiverHandler + 'static> From<T> for Receiver {
    fn from(handler: T) -> Self {
        Self {
            receiver: Arc::new(handler),
        }
    }
}

impl std::fmt::Debug for Receiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusReceiver")
            .field("receiver", &"<dyn StatusReceiverHandler>")
            .finish()
    }
}

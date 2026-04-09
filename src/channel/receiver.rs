use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::status::Status;

// trait for Receiver
pub trait ReceiverHandler: Send + Sync {
    fn try_recv(&self) -> Option<Status>;
    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<Status>> + Send + '_>>;
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
}

impl std::fmt::Debug for Receiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusReceiver")
            .field("receiver", &"<dyn StatusReceiverHandler>")
            .finish()
    }
}

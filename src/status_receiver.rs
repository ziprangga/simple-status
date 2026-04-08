use crate::StatusEvent;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// trait for Receiver
pub trait StatusReceiverHandler: Send + Sync {
    fn try_recv(&self) -> Option<StatusEvent>;
    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<StatusEvent>> + Send + '_>>;
}

#[derive(Clone)]
pub struct StatusReceiver {
    receiver: Arc<dyn StatusReceiverHandler>,
}

impl StatusReceiver {
    pub fn new(receiver: Arc<dyn StatusReceiverHandler>) -> Self {
        Self { receiver }
    }

    pub fn sync_recv(&self) -> Option<StatusEvent> {
        self.receiver.try_recv()
    }

    pub async fn async_recv(&self) -> Option<StatusEvent> {
        self.receiver.recv().await
    }
}

impl std::fmt::Debug for StatusReceiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusReceiver")
            .field("receiver", &"<dyn StatusReceiverHandler>")
            .finish()
    }
}

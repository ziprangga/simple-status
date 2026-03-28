use crate::Status;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// trait for Emitter
pub trait StatusEmitterHandler: Send + Sync {
    fn try_emit(&self, status: Status);
    fn emit(&self, status: Status) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
}

// trait for Receiver
pub trait StatusReceiverHandler: Send + Sync {
    fn try_recv(&self) -> Option<Status>;
    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<Status>> + Send + '_>>;
}

#[derive(Clone)]
pub struct StatusEmitter {
    emitter: Arc<dyn StatusEmitterHandler>,
}

impl StatusEmitter {
    pub fn new(emitter: Arc<dyn StatusEmitterHandler>) -> Self {
        Self { emitter }
    }

    pub fn try_emit(&self, status: Status) {
        self.emitter.try_emit(status);
    }

    pub async fn emit(&self, status: Status) {
        self.emitter.emit(status).await;
    }
}

#[derive(Clone)]
pub struct StatusReceiver {
    receiver: Arc<dyn StatusReceiverHandler>,
}

impl StatusReceiver {
    pub fn new(receiver: Arc<dyn StatusReceiverHandler>) -> Self {
        Self { receiver }
    }

    pub fn try_recv(&self) -> Option<Status> {
        self.receiver.try_recv()
    }

    pub async fn recv(&self) -> Option<Status> {
        self.receiver.recv().await
    }
}

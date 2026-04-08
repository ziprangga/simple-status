use crate::StatusEvent;
use crate::StatusReceiver;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// trait for Emitter
pub trait StatusEmitterHandler: Send + Sync {
    fn try_emit(&self, status: StatusEvent);
    fn emit(&self, status: StatusEvent) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
    fn subscribe(&self) -> Option<Arc<StatusReceiver>>;
}

#[derive(Clone)]
pub struct StatusEmitter {
    emitter: Arc<dyn StatusEmitterHandler>,
}

impl StatusEmitter {
    pub fn new(emitter: Arc<dyn StatusEmitterHandler>) -> Self {
        Self { emitter }
    }

    pub fn sync_emit(&self, status: StatusEvent) {
        self.emitter.try_emit(status);
    }

    pub async fn async_emit(&self, status: StatusEvent) {
        self.emitter.emit(status).await;
    }

    pub fn subscribe(&self) -> Option<Arc<StatusReceiver>> {
        self.emitter.subscribe()
    }
}

impl std::fmt::Debug for StatusEmitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusEmitter")
            .field("emitter", &"<dyn StatusEmitterHandler>")
            .finish()
    }
}

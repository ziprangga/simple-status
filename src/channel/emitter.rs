use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use super::Receiver;
use crate::status::Status;

// trait for Emitter
pub trait EmitterHandler: Send + Sync {
    fn try_emit(&self, status: Status);
    fn emit(&self, status: Status) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
    fn subscribe(&self) -> Option<Arc<Receiver>>;
}

#[derive(Clone)]
pub struct Emitter {
    emitter: Arc<dyn EmitterHandler>,
}

impl Emitter {
    pub fn new(emitter: Arc<dyn EmitterHandler>) -> Self {
        Self { emitter }
    }

    pub fn sync_emit(&self, status: Status) {
        self.emitter.try_emit(status);
    }

    pub async fn async_emit(&self, status: Status) {
        self.emitter.emit(status).await;
    }

    pub fn subscribe(&self) -> Option<Arc<Receiver>> {
        self.emitter.subscribe()
    }
}

impl std::fmt::Debug for Emitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusEmitter")
            .field("emitter", &"<dyn StatusEmitterHandler>")
            .finish()
    }
}

use std::sync::Arc;

use super::Receiver;
use crate::status::Status;

use super::BoxFuture;

// trait for Emitter
pub trait EmitterHandler: Send + Sync {
    fn try_emit(&self, status: Status);
    fn emit(&self, status: Status) -> BoxFuture<'_, ()>;
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

impl<T: EmitterHandler + 'static> From<T> for Emitter {
    fn from(handler: T) -> Self {
        Self {
            emitter: Arc::new(handler),
        }
    }
}

impl std::fmt::Debug for Emitter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusEmitter")
            .field("emitter", &"<dyn StatusEmitterHandler>")
            .finish()
    }
}

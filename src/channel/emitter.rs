use crate::Event;
use crate::Receiver;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

// trait for Emitter
pub trait EmitterHandler: Send + Sync {
    fn try_emit(&self, event: Event);
    fn emit(&self, event: Event) -> Pin<Box<dyn Future<Output = ()> + Send + '_>>;
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

    pub fn sync_emit(&self, event: Event) {
        self.emitter.try_emit(event);
    }

    pub async fn async_emit(&self, event: Event) {
        self.emitter.emit(event).await;
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

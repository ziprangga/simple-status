use std::sync::Arc;

use crate::channel::BoxFuture;
use crate::channel::Receiver;
use crate::status::Status;

/// Trait implemented by emitter backends.
///
/// Doc:
/// Defines the low-level interface used by `Emitter`.
///
/// Note:
/// Library users typically interact with `Emitter` instead of implementing this
/// trait unless they are creating a custom transport.
pub trait EmitterHandler: Send + Sync {
    /// Emits a status synchronously.
    ///
    /// Note:
    /// This method should complete the emission before returning.
    fn try_emit(&self, status: Status);

    /// Emits a status asynchronously.
    ///
    /// Note:
    /// The returned future is driven by the caller and does not begin
    /// execution until it is polled.
    fn emit(&self, status: Status) -> BoxFuture<'_, ()>;

    /// Creates a new receiver from this emitter, if supported.
    ///
    /// Note:
    /// Implementations that do not support creating additional receivers
    /// should return `None`.
    fn subscribe(&self) -> Option<Arc<Receiver>>;
}

/// Type-erased status emitter.
///
/// Doc:
/// Provides a uniform API over any type implementing
/// `EmitterHandler`.
///
/// Note:
/// Uses dynamic dispatch (`Arc<dyn EmitterHandler>`) so different emitter
/// implementations share the same public interface.
#[derive(Clone)]
pub struct Emitter {
    emitter: Arc<dyn EmitterHandler>,
}

impl Emitter {
    pub fn new(emitter: Arc<dyn EmitterHandler>) -> Self {
        Self { emitter }
    }

    /// Emits a status synchronously.
    pub fn emit_sync(&self, status: Status) {
        self.emitter.try_emit(status);
    }

    /// Emits a status asynchronously.
    pub async fn emit_async(&self, status: Status) {
        self.emitter.emit(status).await;
    }

    /// Creates a new receiver from this emitter, if supported.
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

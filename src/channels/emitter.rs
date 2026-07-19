use std::sync::Arc;

use crate::channels::BoxFuture;
use crate::channels::Receiver;

/// Trait implemented by emitter backends.
///
/// Doc:
/// Defines the low-level interface used by `Emitter`.
///
/// Note:
/// Library users typically interact with `Emitter` instead of implementing this
/// trait unless they are creating a custom transport.
pub trait EmitterHandler<T>: Send + Sync {
    /// Emits a value synchronously.
    ///
    /// Note:
    /// This method should complete the emission before returning.
    fn try_emit(&self, se: T);

    /// Emits a valueasynchronously.
    ///
    /// Note:
    /// The returned future is driven by the caller and does not begin
    /// execution until it is polled.
    fn emit(&self, se: T) -> BoxFuture<'_, ()>;

    /// Creates a new receiver from this emitter, if supported.
    ///
    /// Note:
    /// Implementations that do not support creating additional receivers
    /// should return `None`.
    fn subscribe(&self) -> Option<Arc<Receiver<T>>>;
}

/// Type-erased value emitter.
///
/// Doc:
/// Provides a uniform API over any type implementing
/// `EmitterHandler`.
///
/// Note:
/// Uses dynamic dispatch (`Arc<dyn EmitterHandler>`) so different emitter
/// implementations share the same public interface.
#[derive(Clone)]
pub struct Emitter<T> {
    emitter: Arc<dyn EmitterHandler<T>>,
}

impl<T> Emitter<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub fn new(emitter: Arc<dyn EmitterHandler<T>>) -> Self {
        Self { emitter }
    }

    /// Emits a value synchronously.
    pub fn emit_sync(&self, se: T) {
        self.emitter.try_emit(se);
    }

    /// Emits a value asynchronously.
    pub async fn emit_async(&self, se: T) {
        self.emitter.emit(se).await;
    }

    /// Creates a new receiver from this emitter, if supported.
    pub fn subscribe(&self) -> Option<Arc<Receiver<T>>> {
        self.emitter.subscribe()
    }

    pub fn from_handler<H>(handler: H) -> Self
    where
        H: EmitterHandler<T> + 'static,
    {
        Self {
            emitter: Arc::new(handler),
        }
    }
}

impl<T> std::fmt::Debug for Emitter<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusEmitter")
            .field("emitter", &"<dyn valueEmitterHandler>")
            .finish()
    }
}

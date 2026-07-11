use std::sync::Arc;

use crate::channel::BoxFuture;
use crate::channel::Receiver;
use crate::status::StatusEvent;

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
    fn try_emit(&self, status: StatusEvent);

    /// Emits a status asynchronously.
    ///
    /// Note:
    /// The returned future is driven by the caller and does not begin
    /// execution until it is polled.
    fn emit(&self, status: StatusEvent) -> BoxFuture<'_, ()>;

    /// Creates a new receiver from this emitter, if supported.
    ///
    /// Note:
    /// Implementations that do not support creating additional receivers
    /// should return `None`.
    fn subscribe(&self) -> Option<Arc<Receiver>>;
}

/// Conversion into an optional emitter reference.
///
/// Doc:
/// Provides a uniform way to accept either an `&Emitter` or an
/// `Option<&Emitter>` and normalize them into `Option<&Emitter>`.
///
/// Note:
/// This trait is primarily intended for API ergonomics
pub trait IntoEmitter<'a> {
    /// Converts this value into an optional emitter reference.
    ///
    /// Note:
    /// Implementations may return `None` when no emitter is available.
    /// The conversion consumes `self`, though implementors are generally
    /// lightweight reference-based types.
    fn into_emitter(self) -> Option<&'a Emitter>;
}

impl<'a> IntoEmitter<'a> for Option<&'a Emitter> {
    /// Returns the emitter unchanged.
    ///
    /// Note:
    /// This implementation allows APIs accepting `IntoEmitter` to receive an
    /// optional emitter directly.
    fn into_emitter(self) -> Option<&'a Emitter> {
        self
    }
}

impl<'a> IntoEmitter<'a> for &'a Emitter {
    /// Wraps the emitter in `Some`.
    ///
    /// Note:
    /// This implementation allows APIs accepting `IntoEmitter` to receive a
    /// concrete emitter reference without requiring callers to construct
    /// `Some(...)` explicitly.
    fn into_emitter(self) -> Option<&'a Emitter> {
        Some(self)
    }
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
    pub fn emit_sync(&self, se: StatusEvent) {
        self.emitter.try_emit(se);
    }

    /// Emits a status asynchronously.
    pub async fn emit_async(&self, se: StatusEvent) {
        self.emitter.emit(se).await;
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

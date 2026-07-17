use std::sync::Arc;

use crate::channels::BoxFuture;
use crate::channels::Receiver;
use crate::status_event::NoId;
use crate::status_event::StatusEvent;

/// Trait implemented by emitter backends.
///
/// Doc:
/// Defines the low-level interface used by `Emitter`.
///
/// Note:
/// Library users typically interact with `Emitter` instead of implementing this
/// trait unless they are creating a custom transport.
pub trait EmitterHandler<I>: Send + Sync {
    /// Emits a status synchronously.
    ///
    /// Note:
    /// This method should complete the emission before returning.
    fn try_emit(&self, se: StatusEvent<I>);

    /// Emits a status asynchronously.
    ///
    /// Note:
    /// The returned future is driven by the caller and does not begin
    /// execution until it is polled.
    fn emit(&self, se: StatusEvent<I>) -> BoxFuture<'_, ()>;

    /// Creates a new receiver from this emitter, if supported.
    ///
    /// Note:
    /// Implementations that do not support creating additional receivers
    /// should return `None`.
    fn subscribe(&self) -> Option<Arc<Receiver<I>>>;
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
pub struct Emitter<I = NoId> {
    emitter: Arc<dyn EmitterHandler<I>>,
}

impl<I> Emitter<I>
where
    I: Send + Sync + Clone + 'static,
{
    // pub fn new(emitter: Arc<dyn EmitterHandler<I>>) -> Self {
    //     Self { emitter }
    // }

    /// Emits a status synchronously.
    pub fn emit_sync(&self, se: StatusEvent<I>) {
        self.emitter.try_emit(se);
    }

    /// Emits a status asynchronously.
    pub async fn emit_async(&self, se: StatusEvent<I>) {
        self.emitter.emit(se).await;
    }

    /// Creates a new receiver from this emitter, if supported.
    pub fn subscribe(&self) -> Option<Arc<Receiver<I>>> {
        self.emitter.subscribe()
    }

    pub fn from_handler<H>(handler: H) -> Self
    where
        H: EmitterHandler<I> + 'static,
    {
        Self {
            emitter: Arc::new(handler),
        }
    }
}

// impl<T: EmitterHandler + 'static> From<T> for Emitter {
//     fn from(handler: T) -> Self {
//         Self {
//             emitter: Arc::new(handler),
//         }
//     }
// }

impl<I> std::fmt::Debug for Emitter<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusEmitter")
            .field("emitter", &"<dyn StatusEmitterHandler>")
            .finish()
    }
}

use futures::stream;
use std::sync::Arc;

use crate::channels::BoxFuture;
use crate::channels::BoxStream;

/// Trait implemented by receiver backends.
///
/// Doc:
/// Defines the low-level interface used by `Receiver`.
///
/// Note:
/// Custom channel implementations implement this trait to integrate with the
/// library.
pub trait ReceiverHandler<T>: Send + Sync {
    /// Receives a value synchronously.
    ///
    /// Note:
    /// This method should return immediately. If no value is available,
    /// it should return `None` instead of waiting.
    fn try_recv(&self) -> Option<T>;

    /// Receives a value asynchronously.
    ///
    /// Note:
    /// The returned future is driven by the caller and does not begin
    /// execution until it is polled.
    fn recv(&self) -> BoxFuture<'_, Option<T>>;

    /// Creates a stream of received values.
    ///
    /// Note:
    /// The returned stream is driven by the caller and yields values
    /// until the receiver is exhausted or the underlying channel is closed.
    fn stream(&self) -> BoxStream<'_, T>;
}

/// Type-erased value receiver.
///
/// Doc:
/// Provides synchronous, asynchronous, and streaming access to received
/// values.
///
/// Note:
/// Like `Emitter`, this type hides the concrete receiver implementation behind
/// `Arc<dyn ReceiverHandler<T>>`.
#[derive(Clone)]
pub struct Receiver<T> {
    receiver: Arc<dyn ReceiverHandler<T>>,
}

impl<T> Receiver<T>
where
    T: Send + Sync + Clone + 'static,
{
    /// Creates a receiver from a type-erased handler.
    pub fn new(receiver: Arc<dyn ReceiverHandler<T>>) -> Self {
        Self { receiver }
    }

    /// Attempts to receive a value synchronously.
    pub fn sync_recv(&self) -> Option<T> {
        self.receiver.try_recv()
    }

    /// Receives the next value asynchronously.
    pub async fn async_recv(&self) -> Option<T> {
        self.receiver.recv().await
    }

    /// Converts this receiver into an asynchronous stream.
    ///
    /// Doc:
    /// The stream repeatedly calls `async_recv()` until the receiver returns
    /// `None`.
    ///
    /// Note:
    /// This is a convenience wrapper built using `futures::stream::unfold`.
    pub fn stream(&self) -> Option<BoxStream<'static, T>> {
        let this = self.clone();
        let s = stream::unfold(this, |res| async move {
            let se = res.async_recv().await?;
            Some((se, res))
        });

        Some(Box::pin(s) as BoxStream<'static, T>)
    }

    /// Creates a receiver from a concrete handler implementation.
    ///
    /// Note:
    /// This allows custom receiver backends to be wrapped by `Receiver`.
    pub fn from_handler<H>(handler: H) -> Self
    where
        H: ReceiverHandler<T> + 'static,
    {
        Self {
            receiver: Arc::new(handler),
        }
    }
}

impl<I> std::fmt::Debug for Receiver<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusReceiver")
            .field("receiver", &"<dyn ReceiverHandler>")
            .finish()
    }
}

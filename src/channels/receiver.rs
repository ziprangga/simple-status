use futures::stream;
use std::sync::Arc;

use crate::channels::BoxFuture;
use crate::channels::BoxStream;
use crate::status_event::NoId;
use crate::status_event::StatusEvent;

/// Trait implemented by receiver backends.
///
/// Doc:
/// Defines the low-level interface used by `Receiver`.
///
/// Note:
/// Custom channel implementations implement this trait to integrate with the
/// library.
pub trait ReceiverHandler<I>: Send + Sync {
    /// Receives a status synchronously.
    ///
    /// Note:
    /// This method should return immediately. If no status is available,
    /// it should return `None` instead of waiting.
    fn try_recv(&self) -> Option<StatusEvent<I>>;

    /// Receives a status asynchronously.
    ///
    /// Note:
    /// The returned future is driven by the caller and does not begin
    /// execution until it is polled.
    fn recv(&self) -> BoxFuture<'_, Option<StatusEvent<I>>>;

    /// Creates a stream of received statuses.
    ///
    /// Note:
    /// The returned stream is driven by the caller and yields statuses
    /// until the receiver is exhausted or the underlying channel is closed.
    fn stream(&self) -> BoxStream<'_, StatusEvent<I>>;
}

/// Type-erased status receiver.
///
/// Doc:
/// Provides synchronous, asynchronous, and streaming access to received
/// statuses.
///
/// Note:
/// Like `Emitter`, this type hides the concrete receiver implementation behind
/// dynamic dispatch.
#[derive(Clone)]
pub struct Receiver<I = NoId> {
    receiver: Arc<dyn ReceiverHandler<I>>,
}

impl<I> Receiver<I>
where
    I: Send + Sync + Clone + 'static,
{
    pub fn new(receiver: Arc<dyn ReceiverHandler<I>>) -> Self {
        Self { receiver }
    }

    /// Attempts to receive a status synchronously.
    pub fn sync_recv(&self) -> Option<StatusEvent<I>> {
        self.receiver.try_recv()
    }

    /// Receives the next status asynchronously.
    pub async fn async_recv(&self) -> Option<StatusEvent<I>> {
        self.receiver.recv().await
    }

    /// Converts this receiver into an asynchronous stream.
    ///
    /// Doc:
    /// The stream repeatedly calls `async_recv()` until no more statuses are
    /// available.
    ///
    /// Note:
    /// This is a convenience wrapper built using `futures::stream::unfold`.
    pub fn stream(&self) -> Option<BoxStream<'static, StatusEvent<I>>> {
        let this = self.clone();
        let s = stream::unfold(this, |res| async move {
            let se = res.async_recv().await?;
            Some((se, res))
        });

        Some(Box::pin(s) as BoxStream<'static, StatusEvent<I>>)
    }

    pub fn from_handler<H>(handler: H) -> Self
    where
        H: ReceiverHandler<I> + 'static,
    {
        Self {
            receiver: Arc::new(handler),
        }
    }
}

// impl<T: ReceiverHandler + 'static> From<T> for Receiver {
//     fn from(handler: T) -> Self {
//         Self {
//             receiver: Arc::new(handler),
//         }
//     }
// }

impl<I> std::fmt::Debug for Receiver<I> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusReceiver")
            .field("receiver", &"<dyn StatusReceiverHandler>")
            .finish()
    }
}

use futures::stream;
use std::sync::Arc;

use crate::channel::BoxFuture;
use crate::channel::BoxStream;
use crate::status::Status;

/// Trait implemented by receiver backends.
///
/// Doc:
/// Defines the low-level interface used by `Receiver`.
///
/// Note:
/// Custom channel implementations implement this trait to integrate with the
/// library.
pub trait ReceiverHandler: Send + Sync {
    fn try_recv(&self) -> Option<Status>;
    fn recv(&self) -> BoxFuture<'_, Option<Status>>;
    fn stream(&self) -> BoxStream<'_, Status>;
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
pub struct Receiver {
    receiver: Arc<dyn ReceiverHandler>,
}

impl Receiver {
    pub fn new(receiver: Arc<dyn ReceiverHandler>) -> Self {
        Self { receiver }
    }

    /// Attempts to receive a status synchronously.
    pub fn sync_recv(&self) -> Option<Status> {
        self.receiver.try_recv()
    }

    /// Receives the next status asynchronously.
    pub async fn async_recv(&self) -> Option<Status> {
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
    pub fn stream(&self) -> Option<BoxStream<'static, Status>> {
        let this = self.clone();
        let s = stream::unfold(this, |res| async move {
            let status = res.async_recv().await?;
            Some((status, res))
        });

        Some(Box::pin(s) as BoxStream<'static, Status>)
    }
}

impl<T: ReceiverHandler + 'static> From<T> for Receiver {
    fn from(handler: T) -> Self {
        Self {
            receiver: Arc::new(handler),
        }
    }
}

impl std::fmt::Debug for Receiver {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("StatusReceiver")
            .field("receiver", &"<dyn StatusReceiverHandler>")
            .finish()
    }
}

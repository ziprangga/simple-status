mod channel_emitter;
mod channel_receiver;
mod emitter;
mod receiver;

pub use emitter::{Emitter, EmitterHandler};
pub use receiver::{Receiver, ReceiverHandler};

pub use channel_emitter::{BroadcastEmitter, MpscEmitter};
pub use channel_receiver::{BroadcastReceiver, MpscReceiver};

use crate::status::Status;
use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub use async_stream::stream;
pub use futures::StreamExt;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;

#[derive(Debug, Clone)]
pub enum ChannelKind {
    Mpsc,
    Broadcast,
}

impl std::str::FromStr for ChannelKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mpsc" => Ok(Self::Mpsc),
            "broadcast" => Ok(Self::Broadcast),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct Channels {
    emitter: Option<Arc<Emitter>>,
    receiver: Option<Arc<Receiver>>,
}

impl Channels {
    pub fn new(emitter: Option<impl Into<Emitter>>, receiver: Option<impl Into<Receiver>>) -> Self {
        Self {
            emitter: emitter.map(|e| Arc::new(e.into())),
            receiver: receiver.map(|r| Arc::new(r.into())),
        }
    }

    pub fn set_emitter(&mut self, emitter: impl Into<Emitter>) {
        self.emitter = Some(Arc::new(emitter.into()));
    }

    pub fn set_receiver(&mut self, receiver: impl Into<Receiver>) {
        self.receiver = Some(Arc::new(receiver.into()));
    }

    pub fn get_emitter(&self) -> Option<Arc<Emitter>> {
        self.emitter.clone()
    }

    pub fn get_receiver(&self) -> Option<Arc<Receiver>> {
        self.receiver.clone()
    }

    /// Synchronous emission (Non-blocking)
    pub fn emit_sync(&self, status: Status) {
        if let Some(e) = &self.emitter {
            e.sync_emit(status);
        }
    }

    /// Asynchronous emission (Awaitable)
    pub async fn emit_async(&self, status: Status) {
        if let Some(e) = &self.emitter {
            e.async_emit(status).await;
        }
    }

    /// Synchronous receiver
    pub fn recv_sync(&self) -> Option<Status> {
        self.receiver.as_ref()?.sync_recv()
    }

    /// Asynchronous receiver
    pub async fn recv_async(&self) -> Option<Status> {
        if let Some(r) = &self.receiver {
            r.async_recv().await
        } else {
            None
        }
    }

    /// sync stream
    pub fn stream_sync(&self) -> Option<BoxStream<'_, Status>> {
        let receiver = self.receiver.as_ref()?;
        receiver.stream()
    }

    /// async stream
    pub fn stream_async(&self) -> Option<BoxStream<'static, Status>> {
        let receiver = self.receiver.as_ref()?.clone();

        Some(Box::pin(stream! {
            if let Some(mut s) = receiver.stream() {
                while let Some(status) = s.next().await {
                    yield status;
                }
            }
        }))
    }

    /// Create a new subscriber from the existing emitter
    pub fn subscribe(&self) -> Option<Arc<Receiver>> {
        self.emitter.as_ref()?.subscribe()
    }
}

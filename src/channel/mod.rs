mod channel_emitter;
mod channel_receiver;
mod emitter;
mod receiver;

pub use emitter::{Emitter, EmitterHandler};
pub use receiver::{Receiver, ReceiverHandler};

pub use channel_emitter::ChannelEmitter;
pub use channel_receiver::ChannelReceiver;

use crate::status::Status;
use std::sync::Arc;

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
    pub fn new(emitter: Option<Arc<Emitter>>, receiver: Option<Arc<Receiver>>) -> Self {
        Self { emitter, receiver }
    }

    pub async fn recv_async(&self) -> Option<Status> {
        if let Some(receiver) = &self.receiver {
            receiver.async_recv().await
        } else {
            None
        }
    }

    pub fn recv_sync(&self) -> Option<Status> {
        self.receiver.as_ref().and_then(|r| r.sync_recv())
    }

    pub fn new_subscriber(&self) -> Option<Arc<Receiver>> {
        self.emitter.as_ref()?.subscribe()
    }

    pub fn emitter(&self) -> Option<Arc<Emitter>> {
        self.emitter.clone()
    }

    pub fn receiver(&self) -> Option<Arc<Receiver>> {
        self.receiver.clone()
    }

    pub fn set_emitter(&mut self, emitter: Option<Arc<Emitter>>) {
        self.emitter = emitter;
    }

    pub fn set_receiver(&mut self, receiver: Option<Arc<Receiver>>) {
        self.receiver = receiver;
    }
}

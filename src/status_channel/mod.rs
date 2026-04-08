mod channel_receiver;
mod channel_sender;

pub use channel_receiver::{ChannelReceiver, ChannelReceiverBroadcast};
pub use channel_sender::{ChannelSender, ChannelSenderBroadcast};

use std::sync::Arc;

use crate::StatusEmitter;
use crate::StatusReceiver;

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

#[derive(Debug, Clone)]
pub enum ChannelHandler {
    Mpsc,
    Broadcast(Arc<StatusEmitter>),
}

impl ChannelHandler {
    pub fn subscribe(&self) -> Option<Arc<StatusReceiver>> {
        match self {
            ChannelHandler::Broadcast(emitter) => emitter.subscribe(),
            _ => None,
        }
    }

    pub fn is_mpsc(&self) -> bool {
        matches!(self, ChannelHandler::Mpsc)
    }

    pub fn is_broadcast(&self) -> bool {
        matches!(self, ChannelHandler::Broadcast(_))
    }
}

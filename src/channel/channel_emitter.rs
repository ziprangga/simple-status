use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use super::ChannelKind;
use super::ChannelReceiver;
use super::EmitterHandler;
use super::Receiver;
use crate::status::Status;

#[derive(Debug, Clone)]
pub struct ChannelEmitter {
    kind: ChannelKind,
    mpsc_sender: Option<mpsc::Sender<Status>>,
    broadcast_sender: Option<broadcast::Sender<Status>>,
}

impl ChannelEmitter {
    pub fn new_mpsc(sender: mpsc::Sender<Status>) -> Self {
        Self {
            kind: ChannelKind::Mpsc,
            mpsc_sender: Some(sender),
            broadcast_sender: None,
        }
    }

    pub fn new_broadcast(sender: broadcast::Sender<Status>) -> Self {
        Self {
            kind: ChannelKind::Broadcast,
            mpsc_sender: None,
            broadcast_sender: Some(sender),
        }
    }

    fn send_status(&self, status: Status) {
        match self.kind {
            ChannelKind::Mpsc => {
                if let Some(sender) = &self.mpsc_sender {
                    let _ = sender.try_send(status);
                }
            }
            ChannelKind::Broadcast => {
                if let Some(sender) = &self.broadcast_sender {
                    let _ = sender.send(status);
                }
            }
        }
    }
}

impl EmitterHandler for ChannelEmitter {
    fn try_emit(&self, status: Status) {
        self.send_status(status);
    }

    fn emit(&self, status: Status) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            match self.kind {
                ChannelKind::Mpsc => {
                    if let Some(sender) = &self.mpsc_sender {
                        let _ = sender.send(status).await;
                    }
                }
                ChannelKind::Broadcast => {
                    if let Some(sender) = &self.broadcast_sender {
                        let _ = sender.send(status);
                    }
                }
            }
        })
    }

    fn subscribe(&self) -> Option<Arc<Receiver>> {
        match self.kind {
            ChannelKind::Mpsc => None,
            ChannelKind::Broadcast => {
                if let Some(sender) = &self.broadcast_sender {
                    // Subscribe to broadcast channel
                    let rx = sender.subscribe();
                    // Wrap in unified ChannelReceiver
                    let receiver = ChannelReceiver::new_broadcast(rx);
                    Some(Arc::new(Receiver::new(Arc::new(receiver))))
                } else {
                    None
                }
            }
        }
    }
}

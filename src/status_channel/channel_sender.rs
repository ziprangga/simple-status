use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use super::ChannelKind;
use super::ChannelReceiver;
use crate::emitter_receiver::StatusEmitterHandler;
use crate::emitter_receiver::StatusReceiver;
use crate::status_event::StatusEvent;

#[derive(Debug, Clone)]
pub struct ChannelSender {
    kind: ChannelKind,
    mpsc_sender: Option<mpsc::Sender<StatusEvent>>,
    broadcast_sender: Option<broadcast::Sender<StatusEvent>>,
}

impl ChannelSender {
    pub fn new_mpsc(sender: mpsc::Sender<StatusEvent>) -> Self {
        Self {
            kind: ChannelKind::Mpsc,
            mpsc_sender: Some(sender),
            broadcast_sender: None,
        }
    }

    pub fn new_broadcast(sender: broadcast::Sender<StatusEvent>) -> Self {
        Self {
            kind: ChannelKind::Broadcast,
            mpsc_sender: None,
            broadcast_sender: Some(sender),
        }
    }

    fn send_event(&self, event: StatusEvent) {
        match self.kind {
            ChannelKind::Mpsc => {
                if let Some(sender) = &self.mpsc_sender {
                    let _ = sender.try_send(event);
                }
            }
            ChannelKind::Broadcast => {
                if let Some(sender) = &self.broadcast_sender {
                    let _ = sender.send(event);
                }
            }
        }
    }
}

impl StatusEmitterHandler for ChannelSender {
    fn try_emit(&self, event: StatusEvent) {
        self.send_event(event);
    }

    fn emit(&self, status: StatusEvent) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
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

    fn subscribe(&self) -> Option<Arc<StatusReceiver>> {
        match self.kind {
            ChannelKind::Mpsc => None,
            ChannelKind::Broadcast => {
                if let Some(sender) = &self.broadcast_sender {
                    // Subscribe to broadcast channel
                    let rx = sender.subscribe();
                    // Wrap in unified ChannelReceiver
                    let receiver = ChannelReceiver::new_broadcast(rx);
                    Some(Arc::new(StatusReceiver::new(Arc::new(receiver))))
                } else {
                    None
                }
            }
        }
    }
}

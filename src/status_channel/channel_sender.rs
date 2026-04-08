use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::ChannelReceiverBroadcast;
use crate::StatusEmitterHandler;
use crate::StatusEvent;
use crate::StatusReceiver;

#[derive(Debug, Clone)]
pub struct ChannelSender {
    channel_sender: mpsc::Sender<StatusEvent>,
}

impl ChannelSender {
    pub fn new(channel_sender: mpsc::Sender<StatusEvent>) -> Self {
        Self { channel_sender }
    }

    fn send_event(&self, event: StatusEvent) {
        let _ = self.channel_sender.try_send(event);
    }
}

impl StatusEmitterHandler for ChannelSender {
    fn try_emit(&self, event: StatusEvent) {
        self.send_event(event);
    }

    fn emit(&self, status: StatusEvent) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let _ = self.channel_sender.send(status).await;
        })
    }

    fn subscribe(&self) -> Option<Arc<StatusReceiver>> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ChannelSenderBroadcast {
    channel_sender: broadcast::Sender<StatusEvent>,
}

impl ChannelSenderBroadcast {
    pub fn new(channel_sender: broadcast::Sender<StatusEvent>) -> Self {
        Self { channel_sender }
    }
}

impl StatusEmitterHandler for ChannelSenderBroadcast {
    fn try_emit(&self, status: StatusEvent) {
        let _ = self.channel_sender.send(status);
    }

    fn emit(&self, status: StatusEvent) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let _ = self.channel_sender.send(status);
        })
    }

    fn subscribe(&self) -> Option<Arc<StatusReceiver>> {
        let rx = self.channel_sender.subscribe();
        Some(Arc::new(StatusReceiver::new(Arc::new(
            ChannelReceiverBroadcast::new(rx),
        ))))
    }
}

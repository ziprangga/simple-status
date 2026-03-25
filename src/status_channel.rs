use std::future::Future;
use std::pin::Pin;
use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::Status;
use crate::StatusEmitterHandler;
use crate::StatusReceiverHandler;

pub struct ChannelSender {
    channel_sender: Sender<Status>,
}

impl ChannelSender {
    pub fn new(channel_sender: Sender<Status>) -> Self {
        Self { channel_sender }
    }

    fn send_event(&self, event: Status) {
        let _ = self.channel_sender.try_send(event);
    }
}

impl StatusEmitterHandler for ChannelSender {
    fn try_emit(&self, event: Status) {
        self.send_event(event);
    }

    fn emit(&self, status: Status) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let _ = self.channel_sender.send(status).await;
        })
    }
}

pub struct ChannelReceiver {
    receiver: Mutex<mpsc::Receiver<Status>>,
}

impl ChannelReceiver {
    pub fn new(rx: mpsc::Receiver<Status>) -> Self {
        Self {
            receiver: Mutex::new(rx),
        }
    }

    fn recv_event(&self) -> Option<Status> {
        if let Ok(mut guard) = self.receiver.try_lock() {
            guard.try_recv().ok()
        } else {
            None
        }
    }
}

impl StatusReceiverHandler for ChannelReceiver {
    fn try_recv(&self) -> Option<Status> {
        self.recv_event()
    }

    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<Status>> + Send + '_>> {
        Box::pin(async move {
            let mut guard = self.receiver.lock().await;
            guard.recv().await
        })
    }
}

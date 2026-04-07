use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::Status;
use crate::StatusEmitterHandler;
use crate::StatusReceiver;
use crate::StatusReceiverHandler;

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
pub struct ChannelSender {
    channel_sender: mpsc::Sender<Status>,
}

impl ChannelSender {
    pub fn new(channel_sender: mpsc::Sender<Status>) -> Self {
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

    fn subscribe(&self) -> Option<Arc<StatusReceiver>> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct ChannelSenderBroadcast {
    channel_sender: broadcast::Sender<Status>,
}

impl ChannelSenderBroadcast {
    pub fn new(channel_sender: broadcast::Sender<Status>) -> Self {
        Self { channel_sender }
    }
}

impl StatusEmitterHandler for ChannelSenderBroadcast {
    fn try_emit(&self, status: Status) {
        let _ = self.channel_sender.send(status);
    }

    fn emit(&self, status: Status) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
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

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ChannelReceiverBroadcast {
    receiver: Mutex<broadcast::Receiver<Status>>,
}

impl ChannelReceiverBroadcast {
    pub fn new(receiver: broadcast::Receiver<Status>) -> Self {
        Self {
            receiver: Mutex::new(receiver),
        }
    }
}

impl StatusReceiverHandler for ChannelReceiverBroadcast {
    fn try_recv(&self) -> Option<Status> {
        if let Ok(mut guard) = self.receiver.try_lock() {
            guard.try_recv().ok()
        } else {
            None
        }
    }

    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<Status>> + Send + '_>> {
        Box::pin(async move {
            let mut guard = self.receiver.lock().await;
            loop {
                match guard.recv().await {
                    Ok(v) => return Some(v),
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => return None,
                }
            }
        })
    }
}

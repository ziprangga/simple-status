use std::future::Future;
use std::pin::Pin;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::StatusEvent;
use crate::StatusReceiverHandler;

#[derive(Debug)]
pub struct ChannelReceiver {
    receiver: Mutex<mpsc::Receiver<StatusEvent>>,
}

impl ChannelReceiver {
    pub fn new(rx: mpsc::Receiver<StatusEvent>) -> Self {
        Self {
            receiver: Mutex::new(rx),
        }
    }

    fn recv_event(&self) -> Option<StatusEvent> {
        if let Ok(mut guard) = self.receiver.try_lock() {
            guard.try_recv().ok()
        } else {
            None
        }
    }
}

impl StatusReceiverHandler for ChannelReceiver {
    fn try_recv(&self) -> Option<StatusEvent> {
        self.recv_event()
    }

    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<StatusEvent>> + Send + '_>> {
        Box::pin(async move {
            let mut guard = self.receiver.lock().await;
            guard.recv().await
        })
    }
}

#[derive(Debug)]
pub struct ChannelReceiverBroadcast {
    receiver: Mutex<broadcast::Receiver<StatusEvent>>,
}

impl ChannelReceiverBroadcast {
    pub fn new(receiver: broadcast::Receiver<StatusEvent>) -> Self {
        Self {
            receiver: Mutex::new(receiver),
        }
    }
}

impl StatusReceiverHandler for ChannelReceiverBroadcast {
    fn try_recv(&self) -> Option<StatusEvent> {
        if let Ok(mut guard) = self.receiver.try_lock() {
            guard.try_recv().ok()
        } else {
            None
        }
    }

    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<StatusEvent>> + Send + '_>> {
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

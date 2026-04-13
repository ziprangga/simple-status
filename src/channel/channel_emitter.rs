use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use super::BroadcastReceiver;
use super::EmitterHandler;
use super::Receiver;
use crate::status::Status;

use super::BoxFuture;

#[derive(Debug, Clone)]
pub struct MpscEmitter {
    sender: mpsc::Sender<Status>,
}

impl MpscEmitter {
    pub fn new(sender: mpsc::Sender<Status>) -> Self {
        Self { sender }
    }
}

impl EmitterHandler for MpscEmitter {
    fn try_emit(&self, status: Status) {
        let _ = self.sender.try_send(status);
    }

    fn emit(&self, status: Status) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            let _ = self.sender.send(status).await;
        })
    }

    fn subscribe(&self) -> Option<Arc<Receiver>> {
        None
    }
}

#[derive(Debug, Clone)]
pub struct BroadcastEmitter {
    sender: broadcast::Sender<Status>,
}

impl BroadcastEmitter {
    pub fn new(sender: broadcast::Sender<Status>) -> Self {
        Self { sender }
    }
}

impl EmitterHandler for BroadcastEmitter {
    fn try_emit(&self, status: Status) {
        let _ = self.sender.send(status);
    }

    fn emit(&self, status: Status) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            self.try_emit(status);
        })
    }

    fn subscribe(&self) -> Option<Arc<Receiver>> {
        let rx = self.sender.subscribe();
        let receiver = BroadcastReceiver::new(rx);
        Some(Arc::new(Receiver::new(Arc::new(receiver))))
    }
}

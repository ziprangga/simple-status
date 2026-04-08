use std::future::Future;
use std::pin::Pin;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use super::ChannelKind;
use crate::emitter_receiver::StatusReceiverHandler;
use crate::status_event::StatusEvent;

#[derive(Debug)]
pub struct ChannelReceiver {
    kind: ChannelKind,
    mpsc_receiver: Option<Mutex<mpsc::Receiver<StatusEvent>>>,
    broadcast_receiver: Option<Mutex<broadcast::Receiver<StatusEvent>>>,
}

impl ChannelReceiver {
    pub fn new_mpsc(rx: mpsc::Receiver<StatusEvent>) -> Self {
        Self {
            kind: ChannelKind::Mpsc,
            mpsc_receiver: Some(Mutex::new(rx)),
            broadcast_receiver: None,
        }
    }

    pub fn new_broadcast(rx: broadcast::Receiver<StatusEvent>) -> Self {
        Self {
            kind: ChannelKind::Broadcast,
            mpsc_receiver: None,
            broadcast_receiver: Some(Mutex::new(rx)),
        }
    }

    fn try_recv_event(&self) -> Option<StatusEvent> {
        match self.kind {
            ChannelKind::Mpsc => {
                if let Some(rx) = &self.mpsc_receiver {
                    if let Ok(mut guard) = rx.try_lock() {
                        return guard.try_recv().ok();
                    }
                }
                None
            }
            ChannelKind::Broadcast => {
                if let Some(rx) = &self.broadcast_receiver {
                    if let Ok(mut guard) = rx.try_lock() {
                        return guard.try_recv().ok();
                    }
                }
                None
            }
        }
    }
}

impl StatusReceiverHandler for ChannelReceiver {
    fn try_recv(&self) -> Option<StatusEvent> {
        self.try_recv_event()
    }

    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<StatusEvent>> + Send + '_>> {
        Box::pin(async move {
            match self.kind {
                ChannelKind::Mpsc => {
                    if let Some(rx) = &self.mpsc_receiver {
                        let mut guard = rx.lock().await;
                        guard.recv().await
                    } else {
                        None
                    }
                }
                ChannelKind::Broadcast => {
                    if let Some(rx) = &self.broadcast_receiver {
                        let mut guard = rx.lock().await;
                        loop {
                            match guard.recv().await {
                                Ok(v) => return Some(v),
                                Err(broadcast::error::RecvError::Lagged(_)) => continue,
                                Err(_) => return None,
                            }
                        }
                    } else {
                        None
                    }
                }
            }
        })
    }
}

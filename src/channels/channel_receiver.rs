//! Doc:
//! Provides the built-in channel implementations for the communication layer.
//!
//! These implementations adapt Tokio channel types to the generic
//! `EmitterHandler` and `ReceiverHandler` interfaces.
//!
//! Built-in implementations:
//! - `MpscReceiver`
//! - `BroadcastReceiver`
//!
//! Note:
//! These implementations are provided as the crate's default channel adapters.
//! They are conveniences, not requirements. Applications may implement
//! `EmitterHandler` and `ReceiverHandler` for their own transports if different
//! communication mechanisms are needed.
//!
//! The built-in implementations intentionally remain thin adapters over Tokio
//! channels. They do not add buffering, retries, filtering, or other behavior.
//! Their sole responsibility is to translate Tokio channel operations into the
//! crate's generic channel interfaces.
//!..
use futures::stream;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::channels::BoxFuture;
use crate::channels::BoxStream;
use crate::channels::ReceiverHandler;
use crate::status_event::StatusEvent;

/// Receives `Status` values from a Tokio MPSC channel.
///
/// Doc:
/// Wraps `tokio::sync::mpsc::Receiver` as a `ReceiverHandler`.
#[derive(Debug)]
pub struct MpscReceiver {
    inner: Mutex<mpsc::Receiver<StatusEvent>>,
}

impl MpscReceiver {
    pub fn new(rx: mpsc::Receiver<StatusEvent>) -> Self {
        Self {
            inner: Mutex::new(rx),
        }
    }
}

impl ReceiverHandler for MpscReceiver {
    fn try_recv(&self) -> Option<StatusEvent> {
        self.inner.try_lock().ok()?.try_recv().ok()
    }

    fn recv(&self) -> BoxFuture<'_, Option<StatusEvent>> {
        Box::pin(async move { self.inner.lock().await.recv().await })
    }

    fn stream(&self) -> BoxStream<'_, StatusEvent> {
        Box::pin(stream::unfold(self, |this| async move {
            this.recv().await.map(|status| (status, this))
        }))
    }
}

/// Receives `Status` values from a Tokio broadcast channel.
///
/// Doc:
/// Wraps `tokio::sync::broadcast::Receiver` as a `ReceiverHandler`.
///
/// Note:
/// Lagged messages are skipped automatically until the next available status is
/// received.
#[derive(Debug)]
pub struct BroadcastReceiver {
    inner: Mutex<broadcast::Receiver<StatusEvent>>,
}

impl BroadcastReceiver {
    pub fn new(rx: broadcast::Receiver<StatusEvent>) -> Self {
        Self {
            inner: Mutex::new(rx),
        }
    }
}

impl ReceiverHandler for BroadcastReceiver {
    fn try_recv(&self) -> Option<StatusEvent> {
        let mut guard = self.inner.try_lock().ok()?;
        loop {
            match guard.try_recv() {
                Ok(status) => return Some(status),
                Err(broadcast::error::TryRecvError::Lagged(_)) => continue,
                _ => return None,
            }
        }
    }

    fn recv(&self) -> BoxFuture<'_, Option<StatusEvent>> {
        Box::pin(async move {
            let mut guard = self.inner.lock().await;
            loop {
                match guard.recv().await {
                    Ok(status) => return Some(status),
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    _ => return None,
                }
            }
        })
    }

    fn stream(&self) -> BoxStream<'_, StatusEvent> {
        Box::pin(stream::unfold(self, |this| async move {
            this.recv().await.map(|status| (status, this))
        }))
    }
}

//! Doc:
//! Provides the built-in channel implementations for the communication layer.
//!
//! These implementations adapt Tokio channel types to the generic
//! `EmitterHandler` and `ReceiverHandler` interfaces.
//!
//! Built-in implementations:
//! - `MpscEmitter`
//! - `BroadcastEmitter`
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

use std::sync::Arc;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use crate::channel::BoxFuture;
use crate::channel::BroadcastReceiver;
use crate::channel::EmitterHandler;
use crate::channel::Receiver;
use crate::status::StatusEvent;

/// Emits `Status` values through a Tokio MPSC channel.
///
/// Doc:
/// Wraps `tokio::sync::mpsc::Sender` as an `EmitterHandler`.
///
/// Note:
/// MPSC channels support a single receiver. Calling `subscribe()` always
/// returns `None`.
#[derive(Debug, Clone)]
pub struct MpscEmitter {
    sender: mpsc::Sender<StatusEvent>,
}

impl MpscEmitter {
    pub fn new(sender: mpsc::Sender<StatusEvent>) -> Self {
        Self { sender }
    }
}

impl EmitterHandler for MpscEmitter {
    fn try_emit(&self, status: StatusEvent) {
        let _ = self.sender.try_send(status);
    }

    fn emit(&self, status: StatusEvent) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            let _ = self.sender.send(status).await;
        })
    }

    fn subscribe(&self) -> Option<Arc<Receiver>> {
        None
    }
}

/// Emits `Status` values through a Tokio broadcast channel.
///
/// Doc:
/// Wraps `tokio::sync::broadcast::Sender` as an `EmitterHandler`.
///
/// Note:
/// Supports multiple subscribers. Each call to `subscribe()` creates a new
/// independent receiver.
#[derive(Debug, Clone)]
pub struct BroadcastEmitter {
    sender: broadcast::Sender<StatusEvent>,
}

impl BroadcastEmitter {
    pub fn new(sender: broadcast::Sender<StatusEvent>) -> Self {
        Self { sender }
    }
}

impl EmitterHandler for BroadcastEmitter {
    fn try_emit(&self, status: StatusEvent) {
        let _ = self.sender.send(status);
    }

    fn emit(&self, status: StatusEvent) -> BoxFuture<'_, ()> {
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

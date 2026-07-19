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

use crate::channels::BoxFuture;
use crate::channels::BroadcastReceiver;
use crate::channels::EmitterHandler;
use crate::channels::Receiver;

/// Emits values through a Tokio MPSC channel.
///
/// Doc:
/// Wraps `tokio::sync::mpsc::Sender` as an `EmitterHandler`.
///
/// Note:
/// MPSC channels support a single receiver. Calling `subscribe()` always
/// returns `None`.
#[derive(Debug, Clone)]
pub struct MpscEmitter<T> {
    sender: mpsc::Sender<T>,
}

impl<T> MpscEmitter<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub fn new(sender: mpsc::Sender<T>) -> Self {
        Self { sender }
    }
}

impl<T> EmitterHandler<T> for MpscEmitter<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn try_emit(&self, value: T) {
        let _ = self.sender.try_send(value);
    }

    fn emit(&self, value: T) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            let _ = self.sender.send(value).await;
        })
    }

    fn subscribe(&self) -> Option<Arc<Receiver<T>>> {
        None
    }
}

/// Emits  values through a Tokio broadcast channel.
///
/// Doc:
/// Wraps `tokio::sync::broadcast::Sender` as an `EmitterHandler`.
///
/// Note:
/// Supports multiple subscribers. Each call to `subscribe()` creates a new
/// independent receiver.
#[derive(Debug, Clone)]
pub struct BroadcastEmitter<T> {
    sender: broadcast::Sender<T>,
}

impl<T> BroadcastEmitter<T>
where
    T: Send + Sync + Clone + 'static,
{
    pub fn new(sender: broadcast::Sender<T>) -> Self {
        Self { sender }
    }
}

impl<T> EmitterHandler<T> for BroadcastEmitter<T>
where
    T: Send + Sync + Clone + 'static,
{
    fn try_emit(&self, value: T) {
        let _ = self.sender.send(value);
    }

    fn emit(&self, value: T) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            self.try_emit(value);
        })
    }

    fn subscribe(&self) -> Option<Arc<Receiver<T>>> {
        let rx = self.sender.subscribe();
        let receiver = BroadcastReceiver::new(rx);
        Some(Arc::new(Receiver::new(Arc::new(receiver))))
    }
}

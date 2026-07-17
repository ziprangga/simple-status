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
use crate::status_event::StatusEvent;

/// Emits `Status Event` values through a Tokio MPSC channel.
///
/// Doc:
/// Wraps `tokio::sync::mpsc::Sender` as an `EmitterHandler`.
///
/// Note:
/// MPSC channels support a single receiver. Calling `subscribe()` always
/// returns `None`.
#[derive(Debug, Clone)]
pub struct MpscEmitter<I> {
    sender: mpsc::Sender<StatusEvent<I>>,
}

impl<I> MpscEmitter<I>
where
    I: Send + Sync + Clone + 'static,
{
    pub fn new(sender: mpsc::Sender<StatusEvent<I>>) -> Self {
        Self { sender }
    }
}

impl<I> EmitterHandler<I> for MpscEmitter<I>
where
    I: Send + Sync + Clone + 'static,
{
    fn try_emit(&self, se: StatusEvent<I>) {
        let _ = self.sender.try_send(se);
    }

    fn emit(&self, se: StatusEvent<I>) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            let _ = self.sender.send(se).await;
        })
    }

    fn subscribe(&self) -> Option<Arc<Receiver<I>>> {
        None
    }
}

/// Emits `Status Event` values through a Tokio broadcast channel.
///
/// Doc:
/// Wraps `tokio::sync::broadcast::Sender` as an `EmitterHandler`.
///
/// Note:
/// Supports multiple subscribers. Each call to `subscribe()` creates a new
/// independent receiver.
#[derive(Debug, Clone)]
pub struct BroadcastEmitter<I> {
    sender: broadcast::Sender<StatusEvent<I>>,
}

impl<I> BroadcastEmitter<I>
where
    I: Send + Sync + Clone + 'static,
{
    pub fn new(sender: broadcast::Sender<StatusEvent<I>>) -> Self {
        Self { sender }
    }
}

impl<I> EmitterHandler<I> for BroadcastEmitter<I>
where
    I: Send + Sync + Clone + 'static,
{
    fn try_emit(&self, se: StatusEvent<I>) {
        let _ = self.sender.send(se);
    }

    fn emit(&self, se: StatusEvent<I>) -> BoxFuture<'_, ()> {
        Box::pin(async move {
            self.try_emit(se);
        })
    }

    fn subscribe(&self) -> Option<Arc<Receiver<I>>> {
        let rx = self.sender.subscribe();
        let receiver = BroadcastReceiver::new(rx);
        Some(Arc::new(Receiver::new(Arc::new(receiver))))
    }
}

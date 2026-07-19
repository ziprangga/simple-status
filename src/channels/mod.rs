//! Doc:
//! Provides the communication layer for transmitting values between
//! producers and consumers.
//!
//! This module defines common abstractions over different channel
//! implementations. Users interact with [`Emitter`] and [`Receiver`] instead of
//! concrete channel types, allowing channel implementations to be exchanged
//! without changing application code.
//!
//! Built-in implementations include:
//! - MPSC channels
//! - Broadcast channels
//!
//! Note:
//! `Emitter` and `Receiver` are intentionally thin wrappers around trait
//! objects (`EmitterHandler` and `ReceiverHandler`). This separates the public
//! API from concrete channel implementations, allowing different transports
//! (such as MPSC, Broadcast, or future implementations) to share the same
//! interface without changing user code.
//!..

mod channel_emitter;
mod channel_receiver;
mod emitter;
mod receiver;

pub use emitter::{Emitter, EmitterHandler};
pub use receiver::{Receiver, ReceiverHandler};

pub use channel_emitter::{BroadcastEmitter, MpscEmitter};
pub use channel_receiver::{BroadcastReceiver, MpscReceiver};

use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::status_event::StatusEvent;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;
pub type BoxStream<'a, T> = Pin<Box<dyn Stream<Item = T> + Send + 'a>>;

/// Selects a channel implementation.
///
/// Doc:
/// Used when a channel type needs to be selected dynamically.
///
/// Note:
/// This enum only identifies built-in channel implementations. It does not
/// restrict custom implementations created through `EmitterHandler` and
/// `ReceiverHandler`.
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

/// Owns an optional emitter and receiver.
///
/// Doc:
/// `Channels` provides a convenient wrapper for working with both sides of a
/// communication channel through a single object.
///
/// Either side may be absent depending on the application's requirements.
///
/// Note:
/// Both components are stored inside `Arc` so they can be cheaply cloned and
/// shared between threads.
#[derive(Debug, Clone)]
pub struct Channels<T = StatusEvent> {
    emitter: Arc<Emitter<T>>,
    receiver: Arc<Receiver<T>>,
}

impl<T> Channels<T>
where
    T: Send + Sync + Clone + 'static,
{
    /// Creates a new channel container.
    ///
    /// Doc:
    /// Both the emitter and receiver are optional.
    pub fn new(emitter: impl Into<Emitter<T>>, receiver: impl Into<Receiver<T>>) -> Self {
        Self {
            emitter: Arc::new(emitter.into()),
            receiver: Arc::new(receiver.into()),
        }
    }

    /// Replaces the current emitter.
    pub fn set_emitter(&mut self, emitter: impl Into<Emitter<T>>) {
        self.emitter = Arc::new(emitter.into());
    }

    /// Replaces the current receiver.
    pub fn set_receiver(&mut self, receiver: impl Into<Receiver<T>>) {
        self.receiver = Arc::new(receiver.into());
    }

    /// Returns a shared emitter, if one exists.
    pub fn get_emitter(&self) -> Arc<Emitter<T>> {
        self.emitter.clone()
    }

    /// Returns a shared receiver, if one exists.
    pub fn get_receiver(&self) -> Arc<Receiver<T>> {
        self.receiver.clone()
    }

    /// Emits a value synchronously.
    ///
    /// Doc:
    /// Performs an immediate, non-async emission.
    ///
    /// If no emitter exists, this method does nothing.
    pub fn emit_sync(&self, se: T) {
        self.emitter.emit_sync(se);
    }

    /// Emits a value asynchronously.
    ///
    /// Doc:
    /// Awaits the underlying channel implementation.
    ///
    /// If no emitter exists, this method completes immediately.
    pub async fn emit_async(&self, se: T) {
        self.emitter.emit_async(se).await;
    }

    /// Attempts to receive a value synchronously.
    ///
    /// Returns `None` if no receiver exists or no value is available.
    pub fn recv_sync(&self) -> Option<T> {
        self.receiver.sync_recv()
    }

    /// Receives the next value asynchronously.
    ///
    /// Returns `None` if no receiver exists.
    pub async fn recv_async(&self) -> Option<T> {
        self.receiver.async_recv().await
    }

    /// Creates a stream of received values.
    ///
    /// Doc:
    /// Each item is produced by repeatedly awaiting `recv_async()`.
    ///
    /// The returned stream implements `Stream`.
    ///
    /// Note:
    /// Stream from existing receiver need StreamExt to map and use next(),
    /// `StreamExt` is re-exported by this crate for convenience.
    /// can be use from simple_status::StreamExt
    pub fn stream(&self) -> Option<BoxStream<'static, T>> {
        self.receiver.stream()
    }

    /// Creates a new receiver subscribed to the current emitter.
    ///
    /// Doc:
    /// Only emitters that support multiple subscribers return a receiver.
    ///
    /// Returns `None` if the emitter does not support subscriptions.
    pub fn subscribe(&self) -> Option<Arc<Receiver<T>>> {
        self.emitter.subscribe()
    }
}

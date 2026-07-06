//! Doc:
//! Provides the communication layer for transmitting [`Status`] values between
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

use crate::status::Status;
use futures::Stream;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub use futures::StreamExt;

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
#[derive(Debug, Clone, Default)]
pub struct Channels {
    emitter: Option<Arc<Emitter>>,
    receiver: Option<Arc<Receiver>>,
}

impl Channels {
    /// Creates a new channel container.
    ///
    /// Doc:
    /// Both the emitter and receiver are optional.
    pub fn new(emitter: Option<impl Into<Emitter>>, receiver: Option<impl Into<Receiver>>) -> Self {
        Self {
            emitter: emitter.map(|e| Arc::new(e.into())),
            receiver: receiver.map(|r| Arc::new(r.into())),
        }
    }

    /// Replaces the current emitter.
    pub fn set_emitter(&mut self, emitter: impl Into<Emitter>) {
        self.emitter = Some(Arc::new(emitter.into()));
    }

    /// Replaces the current receiver.
    pub fn set_receiver(&mut self, receiver: impl Into<Receiver>) {
        self.receiver = Some(Arc::new(receiver.into()));
    }

    /// Returns a shared emitter, if one exists.
    pub fn get_emitter(&self) -> Option<Arc<Emitter>> {
        self.emitter.clone()
    }

    /// Returns a shared receiver, if one exists.
    pub fn get_receiver(&self) -> Option<Arc<Receiver>> {
        self.receiver.clone()
    }

    /// Emits a status synchronously.
    ///
    /// Doc:
    /// Performs an immediate, non-async emission.
    ///
    /// If no emitter exists, this method does nothing.
    pub fn emit_sync(&self, status: Status) {
        if let Some(e) = &self.emitter {
            e.emit_sync(status);
        }
    }

    /// Emits a status asynchronously.
    ///
    /// Doc:
    /// Awaits the underlying channel implementation.
    ///
    /// If no emitter exists, this method completes immediately.
    pub async fn emit_async(&self, status: Status) {
        if let Some(e) = &self.emitter {
            e.emit_async(status).await;
        }
    }

    /// Attempts to receive a status synchronously.
    ///
    /// Returns `None` if no receiver exists or no status is available.
    pub fn recv_sync(&self) -> Option<Status> {
        self.receiver.as_ref()?.sync_recv()
    }

    /// Receives the next status asynchronously.
    ///
    /// Returns `None` if no receiver exists.
    pub async fn recv_async(&self) -> Option<Status> {
        if let Some(r) = &self.receiver {
            r.async_recv().await
        } else {
            None
        }
    }

    /// Creates a stream of received statuses.
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
    pub fn stream(&self) -> Option<BoxStream<'static, Status>> {
        self.receiver.as_ref()?.stream()
    }

    /// Creates a new receiver subscribed to the current emitter.
    ///
    /// Doc:
    /// Only emitters that support multiple subscribers return a receiver.
    ///
    /// Returns `None` if the emitter does not support subscriptions.
    pub fn subscribe(&self) -> Option<Arc<Receiver>> {
        self.emitter.as_ref()?.subscribe()
    }
}

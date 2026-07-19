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
//! `Emitter` and `Receiver` are generic over the transmitted value type,
//! intentionally thin wrappers around trait objects (`EmitterHandler` and `ReceiverHandler`).
//! This separates the public API from concrete channel implementations,
//! allowing different transports (such as MPSC, Broadcast, or future implementations)
//! to share the same interface without changing user code.
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
use std::sync::OnceLock;

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

impl ChannelKind {
    /// Constructs a built-in channel implementation.
    ///
    /// Creates the emitter and receiver pair corresponding to the selected
    /// [`ChannelKind`].
    ///
    /// # Note
    ///
    /// This function centralizes channel construction to ensure that
    /// all crate APIs create equivalent channel configurations.
    pub fn build_channels<T>(&self, buffer: usize) -> Channels<T>
    where
        T: Send + Sync + Clone + 'static,
    {
        match self {
            Self::Mpsc => {
                let (tx, rx) = tokio::sync::mpsc::channel(buffer);

                let emitter = Emitter::<T>::from_handler(MpscEmitter::<T>::new(tx));
                let receiver = Receiver::<T>::from_handler(MpscReceiver::<T>::new(rx));

                Channels::<T>::new(emitter, receiver)
            }

            Self::Broadcast => {
                let (tx, _rx) = tokio::sync::broadcast::channel(buffer);

                let persistent_rx = tx.subscribe();

                let emitter = Emitter::<T>::from_handler(BroadcastEmitter::<T>::new(tx));
                let receiver =
                    Receiver::<T>::from_handler(BroadcastReceiver::<T>::new(persistent_rx));

                Channels::<T>::new(emitter, receiver)
            }
        }
    }
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

/// Owns an emitter and receiver pair.
///
/// Doc:
/// `Channels` provides a convenient wrapper for working with both sides of a
/// communication channel through a single object.
///
/// `T` represents the transmitted value type. The default type is
/// [`StatusEvent`].
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
    /// Creates a new channel container from an emitter and receiver.
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

    /// Returns a shared emitter.
    pub fn get_emitter(&self) -> Arc<Emitter<T>> {
        self.emitter.clone()
    }

    /// Returns a shared receiver.
    pub fn get_receiver(&self) -> Arc<Receiver<T>> {
        self.receiver.clone()
    }

    /// Emits a value synchronously.
    ///
    /// Doc:
    /// Performs an immediate, non-async emission.
    pub fn emit_sync(&self, se: T) {
        self.emitter.emit_sync(se);
    }

    /// Emits a value asynchronously.
    ///
    /// Doc:
    /// Awaits the underlying channel implementation.
    pub async fn emit_async(&self, se: T) {
        self.emitter.emit_async(se).await;
    }

    /// Attempts to receive a value synchronously.
    ///
    /// Returns `None` if no value is available.
    pub fn recv_sync(&self) -> Option<T> {
        self.receiver.sync_recv()
    }

    /// Receives the next value asynchronously.
    ///
    /// Returns `None` if no value is available.
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

/// Stores a lazily initialized channel set.
///
/// `ChannelsBus<T>` is intended to be declared as a global static and
/// initialized during application startup with [`init_channels_bus()`]
/// or [`ChannelsBus::set_channels()`].
///
/// The bus defaults to [`StatusEvent`] but can store channels for any
/// value type satisfying the channel trait bounds.
///
/// It is commonly declared as a global static and initialized during
/// application startup.
///
/// # Example
///
/// ```rust
/// use simple_status::{
///     ChannelKind,
///     ChannelsBus,
///     init_channels_bus,
/// };
///
/// static STATUS_BUS: ChannelsBus = ChannelsBus::new();
///
/// fn main() {
///     init_channels_bus(&STATUS_BUS, 32, ChannelKind::Broadcast);
/// }
/// ```
///
/// # Note
///
/// Internally uses [`OnceLock`] to ensure channels are initialized at most
/// once and can be safely shared across threads.
pub struct ChannelsBus<T = StatusEvent> {
    channels: OnceLock<Channels<T>>,
}

impl<T> ChannelsBus<T>
where
    T: Send + Sync + Clone + 'static,
{
    /// Creates an uninitialized channel bus.
    ///
    /// The bus contains no channels until [`set_channels`] is called.
    ///
    /// ```rust
    /// use simple_status::ChannelsBus;
    ///
    /// static STATUS_BUS: ChannelsBus = ChannelsBus::new();
    /// ```
    ///
    /// The bus must later be initialized with [`init_channels_bus()`]
    /// or [`ChannelsBus::set_channels()`] before use.
    pub const fn new() -> Self {
        Self {
            channels: OnceLock::new(),
        }
    }

    /// Creates a built-in [`Channels<T>`] instance using the selected
    /// [`ChannelKind`] and stores it in the bus.
    ///
    /// The resulting channels become available through the bus APIs.
    ///
    /// # Example
    ///
    /// ```rust
    /// use simple_status::{
    ///     ChannelKind,
    ///     ChannelsBus,
    ///     init_channels_bus,
    /// };
    ///
    /// static STATUS_BUS: ChannelsBus = ChannelsBus::new();
    ///
    /// fn main() {
    ///     init_channels_bus(&STATUS_BUS, 32, ChannelKind::Broadcast);
    /// }
    /// ```
    ///
    /// Equivalent:
    ///
    /// ```rust
    /// use simple_status::{
    ///     ChannelKind,
    ///     ChannelsBus,
    /// };
    ///
    /// static STATUS_BUS: ChannelsBus = ChannelsBus::new();
    ///
    /// fn main() {
    ///     STATUS_BUS.set_channels(32, ChannelKind::Broadcast);
    /// }
    /// ```
    ///
    /// # Note
    ///
    /// Initialization is backed by [`OnceLock`]. Only the first call
    /// stores channels in the bus. Subsequent calls have no effect.
    pub fn set_channels(&self, buffer: usize, kind: ChannelKind) {
        let channels = kind.build_channels(buffer);
        let _ = self.channels.set(channels);
    }

    /// Returns a reference to the stored [`Channels<T>`].
    ///
    /// # Panics
    ///
    /// Panics if the bus has not been initialized with
    /// [`init_channels_bus()`] or [`ChannelsBus::set_channels()`].
    pub fn channels(&self) -> &Channels<T> {
        self.channels
            .get()
            .expect("ChannelsBus has not been initialized")
    }

    // ==========================
    // ChannelsBus API
    // ==========================
    //
    // These functions operate on the stored [`Channels<T>`].
    //
    // A bus is commonly declared as a global static and initialized once
    // through [`init_channels_bus()`] or [`ChannelsBus::set_channels()`].
    //
    // # Panics
    //
    // Panics if the provided bus has not been initialized with
    // [`init_channels_bus()`].
    //

    /// Creates a stream of values from the stored channels.
    ///
    /// Each item is produced by repeatedly receiving values from the
    /// underlying receiver.
    ///
    /// Returns `None` if streaming is not supported by the configured
    /// channel implementation.
    ///
    /// # Panics
    ///
    /// Panics if `bus` has not been initialized.
    pub fn stream(&self) -> Option<BoxStream<'static, T>> {
        self.channels().stream()
    }

    /// Emits a value synchronously through the stored channels.
    ///
    /// # Panics
    ///
    /// Panics if `bus` has not been initialized.
    pub fn emit_sync(&self, value: T) {
        self.channels().emit_sync(value);
    }

    /// Emits a value asynchronously through the stored channels.
    ///
    /// # Panics
    ///
    /// Panics if `bus` has not been initialized.
    pub async fn emit_async(&self, value: T) {
        self.channels().emit_async(value).await;
    }

    /// Attempts to receive a value synchronously from the stored channels.
    ///
    /// Returns `None` if no value is available.
    ///
    /// # Panics
    ///
    /// Panics if `bus` has not been initialized.
    pub fn recv_sync(&self) -> Option<T> {
        self.channels().recv_sync()
    }

    /// Receives the next value asynchronously from the stored channels.
    ///
    /// Returns `None` if no value is available.
    ///
    /// # Panics
    ///
    /// Panics if `bus` has not been initialized.
    pub async fn recv_async(&self) -> Option<T> {
        self.channels().recv_async().await
    }

    /// Creates an additional receiver subscribed to the stored channels.
    ///
    /// For channel implementations that support multiple receivers,
    /// the returned receiver can be used independently of the primary
    /// receiver.
    ///
    /// Returns `None` if subscriptions are not supported.
    ///
    /// # Panics
    ///
    /// Panics if `bus` has not been initialized.
    pub fn subscribe(&self) -> Option<Arc<Receiver<T>>> {
        self.channels().subscribe()
    }
}

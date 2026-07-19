// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Doc:
//! `simple-status` provides a lightweight status reporting system for
//! applications.
//!
//! The crate is built around three core concepts:
//!
//! - `Status` represents a single status update.
//! - `Emitter` sends status updates.
//! - `Receiver` receives status updates.
//!
//! The crate supports both:
//!
//! - Independent channel instances created with `create_channels()`.
//! - A global channel initialized with `init_channels()` for use with the
//!   provided macros.
//!
//! Built-in channel implementations include:
//! - MPSC
//! - Broadcast
//!
//! Note:
//! Most users only need the public API re-exported from this crate root.
//! Internal modules exist to organize the implementation and are not intended
//! to be used directly.
//!
//! The crate intentionally separates three responsibilities:
//!
//! - `Status` describes state.
//! - Channels transport state.
//! - Macros provide a concise interface for constructing and emitting state.
//!
//! This separation keeps the data model, transport layer, and ergonomics
//! independent, allowing applications to use only the components they need.
//!
//! The crate is designed around abstractions (`Emitter`, `Receiver`, and
//! `Status`) so that different communication mechanisms can share a common
//! interface.
//!..

mod channels;
mod status_event;

#[macro_use]
mod macros;

use channels::BroadcastEmitter;
use channels::BroadcastReceiver;
use channels::Channels;
use channels::Emitter;
use channels::MpscEmitter;
use channels::MpscReceiver;
use channels::Receiver;

pub use channels::BoxFuture;
pub use channels::BoxStream;
pub use channels::ChannelKind;
pub use channels::EmitterHandler;
pub use channels::ReceiverHandler;
pub use status_event::Event;
pub use status_event::Id;
pub use status_event::IntoId;
pub use status_event::StatusEvent;
pub use status_event::StatusEventRenderer;

// // Re-export commonly used stream extension traits for convenience.
pub use futures::StreamExt;

#[doc(hidden)]
pub mod __private_helper;

use std::sync::{Arc, OnceLock};

/// Type alias for an emitter that sends [`StatusEvent`] values.
///
/// This is the primary sending handle used throughout the crate.
pub type StatusEmitter = Emitter<StatusEvent>;

/// Type alias for a receiver that receives [`StatusEvent`] values.
///
/// This is the primary receiving handle used throughout the crate.
pub type StatusReceiver = Receiver<StatusEvent>;

/// Type alias for a paired emitter/receiver channel abstraction.
///
/// A `StatusChannels` instance owns both communication endpoints and
/// provides convenience methods for sending, receiving, subscribing,
/// and streaming events.
pub type StatusChannels = Channels<StatusEvent>;

/// Stores a lazily initialized channel set.
///
/// `ChannelsBus` is intended to be declared as a global static and
/// initialized during application startup with [`init_channels()`].
///
/// # Example
///
/// ```rust
/// use simple_status::{
///     ChannelKind,
///     ChannelsBus,
///     init_channels,
/// };
///
/// static STATUS_BUS: ChannelsBus = ChannelsBus::new();
///
/// fn main() {
///     init_channels(&STATUS_BUS, 32, ChannelKind::Broadcast);
/// }
/// ```
///
/// # Note
///
/// Internally, channel storage is backed by [`OnceLock`], allowing the
/// bus to be safely initialized once and shared throughout the application.
pub struct ChannelsBus {
    channels: OnceLock<StatusChannels>,
}

impl ChannelsBus {
    /// Creates an uninitialized channel bus.
    ///
    /// This function is typically used when defining a global static:
    ///
    /// ```rust
    /// use simple_status::ChannelsBus;
    ///
    /// static STATUS_BUS: ChannelsBus = ChannelsBus::new();
    /// ```
    ///
    /// The bus must later be initialized with [`init_channels()`]
    /// before use.
    pub const fn new() -> Self {
        Self {
            channels: OnceLock::new(),
        }
    }

    /// Returns the initialized channel set.
    ///
    /// # Panics
    ///
    /// Panics if the bus has not been initialized with
    /// [`init_channels()`].
    pub fn channels(&self) -> &StatusChannels {
        self.channels
            .get()
            .expect("simple_status::init_channels_bus() has not been called")
    }
}

/// Initializes a [`ChannelsBus`] with a channel implementation.
///
/// The created channels are stored in the provided bus and can later
/// be accessed through the crate APIs or directly through the bus.
///
/// # Example
///
/// ```rust
/// use simple_status::{
///     ChannelKind,
///     ChannelsBus,
///     init_channels,
/// };
///
/// static STATUS_BUS: ChannelsBus = ChannelsBus::new();
///
/// fn main() {
///     init_channels(&STATUS_BUS, 32, ChannelKind::Broadcast);
/// }
/// ```
///
/// # Note
///
/// Initialization is backed by [`OnceLock`]. Only the first call
/// stores channels in the bus. Subsequent calls have no effect.
pub fn init_channels(bus: &ChannelsBus, buffer: usize, kind: ChannelKind) {
    let (emitter, receiver) = build_channels(buffer, kind);

    let channels = StatusChannels::new(emitter, receiver);

    let _ = bus.channels.set(channels);
}

//// Creates an independent channel set.
///
/// Returns a new [`StatusChannels`] instance containing an emitter
/// and receiver pair.
///
/// Unlike [`init_channels()`], the returned channels are not stored
/// in a [`ChannelsBus`] and are managed entirely by the caller.
///
/// # Common Uses
///
/// - Isolated communication between components.
/// - Multiple independent event pipelines.
/// - Testing without shared state.
///
/// # Example
///
/// ```rust
/// use simple_status::{create_channels, ChannelKind};
///
/// let channels = create_channels(32, ChannelKind::Broadcast);
///
/// channels.emit_sync("hello");
/// ```
///
/// # Note
/// - Isolated communication between components.
/// - Multiple independent event pipelines.
/// - Testing without shared state.
/// This function creates a standalone [`StatusChannels`] value.
/// No global or shared state is modified.

pub fn create_channels(buffer: usize, kind: ChannelKind) -> StatusChannels {
    let (emitter, receiver) = build_channels(buffer, kind);
    StatusChannels::new(emitter, receiver)
}

// ==========================
// ChannelsBus API
// ==========================
//
// These functions operate on the channel set stored in a
// [`ChannelsBus`].
//
// A bus is typically declared as a global static, initialized once
// with [`init_channels()`], and then passed to these functions to
// send, receive, subscribe, or stream events.
//
// # Panics
//
// Panics if the provided bus has not been initialized with
// [`init_channels()`].
//
/// Creates a stream of events from the channels stored in `bus`.
///
/// Each item is produced by repeatedly receiving events from the
/// underlying receiver.
///
/// Returns `None` if streaming is not supported by the configured
/// channel implementation.
///
/// # Panics
///
/// Panics if `bus` has not been initialized.
pub fn stream(bus: &ChannelsBus) -> Option<BoxStream<'static, StatusEvent>> {
    bus.channels().stream()
}

/// Emits an status event synchronously through the channels stored in `bus`.
///
/// # Panics
///
/// Panics if `bus` has not been initialized.
pub fn emit_sync(bus: &ChannelsBus, se: StatusEvent) {
    bus.channels().emit_sync(se);
}

/// Emits an status event asynchronously through the channels stored in `bus`.
///
/// # Panics
///
/// Panics if `bus` has not been initialized.
pub async fn emit_async(bus: &ChannelsBus, se: StatusEvent) {
    bus.channels().emit_async(se).await;
}

/// Attempts to receive an status event synchronously from the channels stored
/// in `bus`.
///
/// Returns `None` if no status event is available.
///
/// # Panics
///
/// Panics if `bus` has not been initialized.
pub fn recv_sync(bus: &ChannelsBus) -> Option<StatusEvent> {
    bus.channels().recv_sync()
}

/// Receives the next status event asynchronously from the channels stored in
/// `bus`.
///
/// Returns `None` if receiving is unavailable.
///
/// # Panics
///
/// Panics if `bus` has not been initialized.
pub async fn recv_async(bus: &ChannelsBus) -> Option<StatusEvent> {
    bus.channels().recv_async().await
}

/// Creates a subscription to the channels stored in `bus`.
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
pub fn subscribe(bus: &ChannelsBus) -> Option<Arc<StatusReceiver>> {
    bus.channels().subscribe()
}

// ==========================
// Direct emitter helpers
// ==========================
//
// Internal helpers used by crate macros.
//
// These functions centralize event emission logic so the macros can
// operate on any compatible emitter implementation.
pub fn status_emit_sync(emitter: &StatusEmitter, se: StatusEvent) {
    emitter.emit_sync(se);
}

pub async fn status_emit_async(emitter: &StatusEmitter, se: StatusEvent) {
    emitter.emit_async(se).await;
}

// ==========================
// Channels builder
// ==========================
/// Constructs a built-in channel implementation.
///
/// Creates the emitter and receiver pair corresponding to the selected
/// [`ChannelKind`].
///
/// # Note
///
/// This function centralizes channel construction to ensure that
/// all crate APIs create equivalent channel configurations.
fn build_channels(buffer: usize, kind: ChannelKind) -> (StatusEmitter, StatusReceiver) {
    match kind {
        ChannelKind::Mpsc => {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer);

            let emitter = StatusEmitter::from_handler(MpscEmitter::<StatusEvent>::new(tx));
            let receiver = StatusReceiver::from_handler(MpscReceiver::<StatusEvent>::new(rx));

            (emitter, receiver)
        }

        ChannelKind::Broadcast => {
            let (tx, _rx) = tokio::sync::broadcast::channel(buffer);

            let persistent_rx = tx.subscribe();

            let emitter = StatusEmitter::from_handler(BroadcastEmitter::<StatusEvent>::new(tx));
            let receiver =
                StatusReceiver::from_handler(BroadcastReceiver::<StatusEvent>::new(persistent_rx));
            (emitter, receiver)
        }
    }
}

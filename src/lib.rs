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

mod channel;
mod status;

#[macro_use]
mod macros;

pub use channel::BoxFuture;
pub use channel::BoxStream;
pub use channel::BroadcastEmitter;
pub use channel::BroadcastReceiver;
pub use channel::ChannelKind;
pub use channel::Channels;
pub use channel::Emitter;
pub use channel::EmitterHandler;
pub use channel::IntoEmitter;
pub use channel::MpscEmitter;
pub use channel::MpscReceiver;
pub use channel::Receiver;
pub use channel::ReceiverHandler;
pub use status::Event;
pub use status::StatusEvent;
pub use status::StatusFormatter;

#[doc(hidden)]
pub mod __private_helper;

use std::sync::{Arc, OnceLock};

static CHANNELS_BUS: OnceLock<Channels> = OnceLock::new();

/// Initializes the global channel.
///
/// Doc:
/// Creates the global channel used by the crate-level functions and macros,
/// such as `status_emit!()`.
///
/// This function should be called once during application initialization.
///
/// Note:
/// Subsequent calls have no effect because the global channel is stored in a
/// `OnceLock`.
pub fn init_channels(buffer: usize, kind: ChannelKind) {
    let (emitter, receiver) = build_channels(buffer, kind);
    let channel_handler = Channels::new(Some(emitter), Some(receiver));
    let _ = CHANNELS_BUS.set(channel_handler);
}

/// Creates an independent channel pair.
///
/// Doc:
/// Returns a new `Channels` instance containing an emitter and receiver.
///
/// Unlike `init_channels()`, this function does not modify the global state.
///
/// Note:
/// Use this when an application requires isolated communication channels or
/// multiple independent status streams.
pub fn create_channels(buffer: usize, kind: ChannelKind) -> Channels {
    let (emitter, receiver) = build_channels(buffer, kind);
    let channel_handler = Channels::new(Some(emitter), Some(receiver));
    channel_handler
}

// =====================================
// Global API
// =====================================
//
// Doc:
// These functions operate on the channel initialized by
// `init_channels()`.
//
// Note:
// Calling any of these functions before `init_channels()` will panic because
// no global channel exists.
fn channels_bus() -> &'static Channels {
    CHANNELS_BUS
        .get()
        .expect("simple_status::init_channels() has not been called")
}

pub fn stream() -> Option<BoxStream<'static, StatusEvent>> {
    channels_bus().stream()
}

pub fn emit_sync(status: StatusEvent) {
    channels_bus().emit_sync(status);
}

pub async fn emit_async(status: StatusEvent) {
    channels_bus().emit_async(status).await;
}

pub fn recv_sync() -> Option<StatusEvent> {
    channels_bus().recv_sync()
}

pub async fn recv_async() -> Option<StatusEvent> {
    channels_bus().recv_async().await
}

pub fn subscribe() -> Option<Arc<Receiver>> {
    channels_bus().subscribe()
}

// ==========================
// Direct emitter helpers
// ==========================
//
// Doc:
// Internal helpers used by the public macros.
//
// Note:
// These functions allow the macros to emit through either an explicit emitter
// or no emitter (`None`) without duplicating logic.
pub fn status_emit_sync(emitter: Option<&Emitter>, status: StatusEvent) {
    if let Some(e) = emitter {
        e.emit_sync(status);
    }
}

pub async fn status_emit_async(emitter: Option<&Emitter>, status: StatusEvent) {
    if let Some(e) = emitter {
        e.emit_async(status).await;
    }
}

// ==========================
// Channels builder
// ==========================
/// Creates the built-in channel implementation.
///
/// Note:
/// This function is the single location responsible for constructing the
/// crate's default channel adapters from a `ChannelKind`.
///
/// Keeping construction centralized ensures both `init_channels()` and
/// `create_channels()` always produce identical channel configurations.
fn build_channels(buffer: usize, kind: ChannelKind) -> (Emitter, Receiver) {
    match kind {
        ChannelKind::Mpsc => {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer);

            let emitter: Emitter = MpscEmitter::new(tx).into();
            let receiver: Receiver = MpscReceiver::new(rx).into();

            (emitter, receiver)
        }

        ChannelKind::Broadcast => {
            let (tx, _rx) = tokio::sync::broadcast::channel(buffer);

            let persistent_rx = tx.subscribe();

            let emitter: Emitter = BroadcastEmitter::new(tx).into();
            let receiver: Receiver = BroadcastReceiver::new(persistent_rx).into();
            (emitter, receiver)
        }
    }
}

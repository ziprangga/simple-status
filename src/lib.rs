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
pub use channel::MpscEmitter;
pub use channel::MpscReceiver;
pub use channel::Receiver;
pub use channel::ReceiverHandler;
pub use status::Event;
pub use status::Status;
pub use status::StatusFormatter;

#[doc(hidden)]
pub mod __private_helper;

use std::sync::{Arc, OnceLock};

static CHANNELS_BUS: OnceLock<Channels> = OnceLock::new();

/// Initialize the global status channel.
///
/// Call once if want to use the global API/macros.
pub fn init_channels(buffer: usize, kind: ChannelKind) {
    let (emitter, receiver) = build_channels(buffer, kind);
    let channel_handler = Channels::new(Some(emitter), Some(receiver));
    let _ = CHANNELS_BUS.set(channel_handler);
}

/// Initializes a new independent status channel.
///
/// Typically called once when creating your application state, although it may
/// be called multiple times if need multiple independent `Channels`
/// instances. This does not initialize or affect the global channel.
pub fn create_channels(buffer: usize, kind: ChannelKind) -> Channels {
    let (emitter, receiver) = build_channels(buffer, kind);
    let channel_handler = Channels::new(Some(emitter), Some(receiver));
    channel_handler
}

// =====================================
// Global
// =====================================

/// Returns the global channel.
///
/// Panics if `init()` has not been called.
fn channels_bus() -> &'static Channels {
    CHANNELS_BUS
        .get()
        .expect("simple_status::init_channels() has not been called")
}

pub fn stream() -> Option<BoxStream<'static, Status>> {
    channels_bus().stream()
}

pub fn emit_sync(status: Status) {
    channels_bus().emit_sync(status);
}

pub async fn emit_async(status: Status) {
    channels_bus().emit_async(status).await;
}

pub fn recv_sync() -> Option<Status> {
    channels_bus().recv_sync()
}

pub async fn recv_async() -> Option<Status> {
    channels_bus().recv_async().await
}

pub fn subscribe() -> Option<Arc<Receiver>> {
    channels_bus().subscribe()
}

// ==========================
// Instant
// ==========================

pub async fn emit_status_async(emitter: Option<&Emitter>, status: Status) {
    if let Some(e) = emitter {
        e.async_emit(status).await;
    }
}

pub fn emit_status_sync(emitter: Option<&Emitter>, status: Status) {
    if let Some(e) = emitter {
        e.sync_emit(status);
    }
}

// ==========================
// Channels builder
// ==========================

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

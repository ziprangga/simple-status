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
//! - `StatusEvent` represents a status update.
//! - `Emitter<T>` sends values.
//! - `Receiver<T>` receives values.
//!
//! The crate supports both:
//!
//! - Independent channel instances created with [`create_channels()`].
//! - Shared channel instances stored in [`ChannelsBus<T>`] and initialized
//!   with [`init_channels_bus()`].
//!
//! Built-in channel implementations include:
//! - MPSC
//! - Broadcast
//!
//! Note:
//! The crate consists of two layers:
//!
//! - A generic channel layer capable of transporting any value type.
//! - A status-reporting layer built around [`StatusEvent`].
//!
//! Most applications only need the public API re-exported from the crate root.
//! Internal modules exist only to organize the implementation.
//!
//! The crate intentionally separates three responsibilities:
//!
//! - `StatusEvent` describes status state.
//! - Channels transport values.
//! - Macros provide a concise interface for constructing and emitting status events.
//!
//! This separation keeps the data model, transport layer, and ergonomics
//! independent, allowing applications to use only the components they need.
//!
//! The crate is designed around generic communication abstractions
//! (`Emitter<T>` and `Receiver<T>`) while providing `StatusEvent`
//! as the default value type used by the status system.
//!
//! The channel layer (`Emitter<T>`, `Receiver<T>`, `Channels<T>`, and
//! `ChannelsBus<T>`) is fully generic and can transport any value type
//! satisfying:
//!
//! `T: Send + Sync + Clone + 'static`
//!
//! The status-reporting APIs use [`StatusEvent`] as the default value type.
//!..

mod channels;
mod renderer;
mod status_event;

#[macro_use]
mod macros;

pub use channels::BoxFuture;
pub use channels::BoxStream;
pub use channels::ChannelKind;
pub use channels::Channels;
pub use channels::ChannelsBus;
pub use channels::Emitter;
pub use channels::EmitterHandler;
pub use channels::Receiver;
pub use channels::ReceiverHandler;
pub use status_event::Event;
pub use status_event::Id;
pub use status_event::IntoId;
pub use status_event::StatusEvent;
// pub use status_event::StatusEventRenderer;
pub use renderer::Renderable;
pub use renderer::Renderer;

// // Re-export commonly used stream extension traits for convenience.
pub use futures::StreamExt;

#[doc(hidden)]
pub mod __private_helper;

/// Convenience alias for `Emitter<StatusEvent>`.
///
/// This is the default emitter type used by the status-reporting APIs.
pub type StatusEmitter = Emitter<StatusEvent>;

/// Convenience alias for `Receiver<StatusEvent>`.
///
/// This is the default receiver type used by the status-reporting APIs.
pub type StatusReceiver = Receiver<StatusEvent>;

/// Convenience alias for `Channels<StatusEvent>`.
///
/// This is the default channel container used by the status-reporting APIs.
pub type StatusChannels = Channels<StatusEvent>;

/// Initializes a [`ChannelsBus<T>`] using the selected channel implementation.
///
/// This is a convenience wrapper around [`ChannelsBus::set_channels`].
///
/// The created channels are stored inside the provided [`ChannelsBus`] and become
/// available through the bus APIs.
///
/// This function exists primarily for ergonomic initialization of
/// global static buses.
///
/// Equivalent to:
///
/// ```rust
/// STATUS_BUS.set_channels(buffer, kind);
/// ```
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
/// Initialization is backed by [`OnceLock`]. Only the first call stores
/// channels in the bus. Subsequent calls have no effect.
pub fn init_channels_bus<T>(bus: &ChannelsBus<T>, buffer: usize, kind: ChannelKind)
where
    T: Send + Sync + Clone + 'static,
{
    bus.set_channels(buffer, kind);
}

/// Creates an independent [`Channels<T>`] instance.
///
/// Returns a standalone emitter/receiver pair using the selected
/// channel implementation.
///
/// Unlike [`init_channels_bus()`], the returned channels are not stored
/// in a [`ChannelsBus<T>`] and are managed entirely by the caller.
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
/// use simple_status::{
///     create_channels,
///     ChannelKind,
///     StatusEvent,
/// };
///
/// let channels = create_channels::<StatusEvent>(
///     32,
///     ChannelKind::Broadcast,
/// );
/// ```
/// # Note
///
/// This function creates a standalone [`Channels<T>`] value.
/// No global or shared state is modified.
pub fn create_channels<T>(buffer: usize, kind: ChannelKind) -> Channels<T>
where
    T: Send + Sync + Clone + 'static,
{
    kind.build_channels(buffer)
}

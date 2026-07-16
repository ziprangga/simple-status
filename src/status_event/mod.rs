//! Core status event model.
//!
//! This module defines the data structures used to describe a single status event
//! update.
//!
//! A status event consists of a [`StatusEvent`], which stores the information to
//! display, such as the current action, progress, message, optional filesystem
//! path, and an optional typed identifier.
//!
//! A status event may exist in one of two forms:
//!
//! - [`StatusEvent<NoId>`] — no identifier is attached.
//! - [`StatusEvent<I>`] — an identifier of type `I` is attached.
//!
//! The builder supports transitioning from a status event without an ID to one
//! with a strongly typed ID.
//!
//! [`StatusEvent`] provides:
//!
//! - Access to the stored event data.
//! - Optional message and filesystem path storage.
//! - Optional strongly typed identifiers.
//! - Custom rendering through [`StatusEventRenderer`].
//!
//! Most users will construct a status event through the builder.
//!
//! ```rust
//! use simple_status::{Event, StatusEvent};
//!
//! let status = StatusEvent::builder()
//!     .message("Compiling")
//!     .event(
//!         Event::builder()
//!             .action("Build")
//!             .current(2)
//!             .total(5)
//!             .build(),
//!     )
//!     .build();
//! ```
//!
//! A typed identifier can also be attached:
//!
//! ```rust
//! use simple_status::{Event, StatusEvent};
//!
//! let status = StatusEvent::builder()
//!     .id(42u64)
//!     .message("Compiling")
//!     .event(Event::default())
//!     .build();
//!
//! assert_eq!(*status.id(), 42);
//! ```
//!
//! Module summary.
//!
//! Doc:
//! - Explains the public API.
//! - Describes how status events are created and rendered.
//! - Documents the typed-ID state transition model.
//!
//! Note:
//! - `StatusEvent` owns all stored data.
//! - The ID type is generic to allow applications to use domain-specific
//!   identifiers without allocations or trait objects.
//! - `NoId` is used as the default state so IDs remain completely optional.
//! - The builder uses type transitions to prevent accidental construction of
//!   invalid ID states.
//!..

mod event;
mod renderer;
pub use event::Event;
use std::borrow::Cow;
use std::path::PathBuf;

/// Marker type representing the absence of an ID.
///
/// This type is used as the default generic parameter for [`StatusEvent`].
///
/// Doc:
/// - Indicates that no identifier is attached.
/// - Serves as the builder's initial state.
///
/// Note:
/// - Zero-sized type.
/// - Introduces no runtime overhead.
/// - Allows ID support without requiring `Option<I>`.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoId;

/// A renderer for converting a [`StatusEvent`] into another representation.
///
/// This trait allows applications to define custom output formats without
/// changing the underlying status data.
///
/// A renderer may produce any output type, such as:
///
/// - `String` for text formatting.
/// - Structured data for serialization.
/// - UI-specific view models.
///
/// Any closure or function matching
/// `Fn(&StatusEvent<I>) -> O` automatically implements this trait.
///
/// # Example
///
/// ```rust
/// use simple_status::StatusEvent;
///
/// let status = StatusEvent::builder()
///     .message("Finished")
///     .build();
///
/// let text = status.render_with(|s| {
///     format!("Status: {:?}", s.message())
/// });
/// ```
///
/// Doc:
/// - Defines how a status event is transformed into another value.
/// - Allows rendering without modifying the underlying status data.
/// - Supports arbitrary output types through the associated `Output` type.
///
/// Note:
/// - The renderer is generic over the status event's ID type.
/// - Closures and functions automatically implement this trait.
/// - Rendering is intentionally separated from storage to keep
///   `StatusEvent` focused on data ownership.
pub trait StatusEventRenderer<I = NoId> {
    type Output;

    fn render(&self, se: &StatusEvent<I>) -> Self::Output;
}

/// Represents a status update.
///
/// A `StatusEvent` owns a single [`Event`] describing the current state of an
/// operation, along with optional metadata such as a message, filesystem path,
/// and typed identifier.
///
/// The identifier type is determined by the generic parameter `I`.
///
/// Doc:
/// - A `StatusEvent` owns exactly one `Event`.
/// - Additional metadata can be attached through the message and path fields.
/// - An optional strongly typed ID may be associated with the event.
/// - It is the primary object passed around the library.
///
/// Note:
/// - `StatusEvent` intentionally owns its data instead of borrowing it.
/// - Ownership avoids lifetime propagation throughout the public API.
/// - The generic ID parameter allows applications to use their own identifier
///   types without runtime overhead.
/// - `NoId` represents the absence of an identifier.
#[derive(Debug, Default, Clone)]
pub struct StatusEvent<I = NoId> {
    id: I,
    message: Option<Cow<'static, str>>,
    event: Event,
    path: Option<PathBuf>,
}

impl StatusEvent<NoId> {
    /// Creates a builder in the "no id" state.
    pub fn builder() -> StatusEventBuilder<NoId> {
        StatusEventBuilder::new()
    }

    /// Attaches an identifier to this status event.
    ///
    /// This converts a `StatusEvent<NoId>` into a `StatusEvent<I>`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use simple_status::StatusEvent;
    ///
    /// let status = StatusEvent::builder()
    ///     .message("Running")
    ///     .build()
    ///     .with_id(42u64);
    ///
    /// assert_eq!(*status.id(), 42);
    /// ```
    ///
    /// Doc:
    /// - Associates a strongly typed identifier with the status event.
    /// - Changes the status event's ID type parameter.
    ///
    /// Note:
    /// - Consumes the original value.
    /// - Performs a type-state transition without cloning stored data.
    pub fn with_id<I>(self, id: I) -> StatusEvent<I> {
        StatusEvent {
            id,
            message: self.message,
            event: self.event,
            path: self.path,
        }
    }
}

impl<I> StatusEvent<I> {
    /// Returns the identifier associated with this status event.
    ///
    /// Doc:
    /// - Provides read-only access to the stored identifier.
    ///
    /// Note:
    /// - The returned type matches the status event's generic ID type.
    /// - For `StatusEvent<NoId>`, this returns a reference to the `NoId` marker.
    pub fn id(&self) -> &I {
        &self.id
    }

    /// Returns the status message, if present.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    /// Returns a shared reference to the current event.
    pub fn event(&self) -> &Event {
        &self.event
    }

    /// Returns the associated filesystem path, if present.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    /// Renders this status event using a custom renderer.
    ///
    /// The renderer may be any type implementing
    /// [`StatusEventRenderer`], including closures and functions.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use simple_status::StatusEvent;
    /// let status = StatusEvent::builder()
    ///     .message("Done")
    ///     .build();
    ///
    /// let text = status.render_with(|status| {
    ///     format!("> {:?}", status.message())
    /// });
    /// ```
    ///
    /// Doc:
    /// - Delegates rendering to a user-provided renderer.
    /// - Supports arbitrary output types.
    ///
    /// Note:
    /// - The returned value is determined by the renderer.
    /// - No allocation is performed unless required by the renderer.
    pub fn render_with<R>(&self, renderer: R) -> R::Output
    where
        R: StatusEventRenderer<I>,
    {
        renderer.render(self)
    }
}

/// Automatically implements [`StatusEventRenderer`] for compatible closures
/// and functions.
///
/// Doc:
/// - Enables using closures directly with
///   [`StatusEvent::render_with`].
///
/// Note:
/// - This implementation is primarily an ergonomics feature.
/// - Most users will never need to implement
///   [`StatusEventRenderer`] manually.
impl<F, O, I> StatusEventRenderer<I> for F
where
    F: Fn(&StatusEvent<I>) -> O,
{
    type Output = O;

    fn render(&self, se: &StatusEvent<I>) -> Self::Output {
        (self)(se)
    }
}

/// Builder for constructing [`StatusEvent`] values.
///
/// The builder starts in the [`NoId`] state and may optionally transition to a
/// typed-ID state through [`StatusEventBuilder::id`].
///
/// Doc:
/// - Supports incremental construction of status events.
/// - Allows attaching an optional typed identifier.
/// - Produces a fully owned [`StatusEvent`].
///
/// Note:
/// - Uses type-state transitions instead of runtime validation.
/// - The builder type changes when an ID is attached.
#[derive(Debug, Default, Clone)]
pub struct StatusEventBuilder<I = NoId> {
    status_event: StatusEvent<I>,
}

impl StatusEventBuilder<NoId> {
    pub fn new() -> Self {
        Self {
            status_event: StatusEvent::default(),
        }
    }

    /// Transition from "no id" to a typed ID.
    ///
    /// ```rust
    /// let builder = StatusEvent::builder().id(42);
    /// // StatusEventBuilder<u64>
    /// ```
    pub fn id<I>(self, id: I) -> StatusEventBuilder<I> {
        StatusEventBuilder {
            status_event: self.status_event.with_id(id),
        }
    }
}

impl<I> StatusEventBuilder<I> {
    pub fn message(mut self, msg: impl Into<Cow<'static, str>>) -> Self {
        self.status_event.message = Some(msg.into());
        self
    }

    pub fn event(mut self, event: Event) -> Self {
        self.status_event.event = event;
        self
    }

    pub fn path(mut self, path: PathBuf) -> Self {
        self.status_event.path = Some(path);
        self
    }

    /// Builds the [`StatusEvent`].
    ///
    /// Doc:
    /// - Consumes the builder.
    /// - Returns the constructed status event.
    /// - Preserves the builder's ID type parameter.
    ///
    /// Note:
    /// - No validation is performed.
    /// - Message, event, and path are optional.
    /// - A status event may be constructed with or without an ID.
    pub fn build(self) -> StatusEvent<I> {
        self.status_event
    }
}

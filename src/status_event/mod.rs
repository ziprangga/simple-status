//! Core status event model.
//!
//! This module defines the data structures used to describe a single status event
//! update.
//!
//! A status event consists of a [`StatusEvent`], which stores information about
//! the current state of an operation, such as the current action, progress,
//! message, optional filesystem path, and an optional typed identifier.
//!
//! A status event may contain an optional identifier represented by [`Id`].
//!
//! The identifier can store built-in types such as `u64` and `String`, or
//! application-specific types through [`Id::Custom`].
//!
//! [`StatusEvent`] provides:
//!
//! - Access to the stored event data.
//! - Optional message and filesystem path storage.
//! - Optional identifiers through [`Id`].
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
//! assert_eq!(status.id().as_u64(), Some(42));
//! ```
//!
//! Module summary.
//!
//! Doc:
//! - Explains the public API.
//! - Defines the core status event data model.
//! - Documents status event construction through the builder API.
//! - Documents identifier storage and retrieval through [`Id`].
//!
//! Note:
//! - [`StatusEvent`] owns all stored data.
//! - [`Event`] describes the state associated with a status event.
//! - [`Id`] allows optional identifiers without requiring generic parameters.
//! - Custom identifiers can be stored and retrieved through [`Id::Custom`].
//!..
mod event;
pub use event::Event;
use std::any::Any;
use std::borrow::Cow;
use std::path::PathBuf;
use std::sync::Arc;

/// Represents an optional identifier attached to a status event.
///
/// `Id` supports common identifier types and allows applications to store
/// custom identifier values.
///
/// Doc:
/// - `None` represents the absence of an identifier.
/// - Other variants store available identifier values.
///
/// Note:
/// - Supports both built-in and application-defined identifier types.
/// - Allows optional identifiers without requiring generic parameters on [`StatusEvent`].
/// - Custom identifiers can be stored through [`Id::Custom`].
#[derive(Debug, Default, Clone)]
pub enum Id {
    #[default]
    None,
    U64(u64),
    String(String),
    Custom(Arc<dyn Any + Send + Sync>),
}

/// Conversion into an [`Id`].
///
/// This trait allows common identifier types to be passed directly to
/// [`StatusEventBuilder::id`] without requiring manual construction of
/// [`Id`] values.
pub trait IntoId {
    /// Converts this value into an [`Id`].
    fn into_id(self) -> Id;
}

/// Represents a status update.
///
/// A `StatusEvent` owns a single [`Event`] describing the current state of an
/// operation, along with optional metadata such as a message, filesystem path,
/// and typed identifier.
///
/// The identifier is stored as an [`Id`] value.
///
/// Doc:
/// - A `StatusEvent` owns exactly one `Event`.
/// - Additional metadata can be attached through the message and path fields.
/// - An optional identifier may be associated with the event.
/// - It is the primary object passed around the library.
///
/// Note:
/// - `StatusEvent` intentionally owns its data instead of borrowing it.
/// - Ownership avoids lifetime propagation throughout the public API.
/// - [`Id::Custom`] allows applications to store domain-specific identifiers.
#[derive(Debug, Default, Clone)]
pub struct StatusEvent {
    id: Id,
    message: Option<Cow<'static, str>>,
    event: Event,
    path: Option<PathBuf>,
}

impl StatusEvent {
    /// Creates a builder in the "no id" state.
    pub fn builder() -> StatusEventBuilder {
        StatusEventBuilder::new()
    }

    /// Returns the identifier associated with this status event.
    ///
    /// Doc:
    /// - Provides read-only access to the stored identifier.
    ///
    /// Note:
    /// - Returns the stored [`Id`] representation.
    /// - Use helper methods such as [`Id::as_u64`], [`Id::as_string`], or
    ///   [`Id::downcast_ref`] to access the stored value.
    pub fn id(&self) -> &Id {
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
}

/// Builder for constructing [`StatusEvent`] values.
///
/// The builder starts without an identifier and may attach an [`Id`] value
/// through [`StatusEventBuilder::id`].
///
/// Doc:
/// - Supports incremental construction of status events.
/// - Allows attaching an optional identifier.
/// - Produces a fully owned [`StatusEvent`].
///
/// Note:
/// - The builder stores the identifier internally as [`Id`].
#[derive(Debug, Default, Clone)]
pub struct StatusEventBuilder {
    status_event: StatusEvent,
}

impl StatusEventBuilder {
    pub fn new() -> Self {
        Self {
            status_event: StatusEvent::default(),
        }
    }

    /// Transition from "no id" to a typed ID.
    ///
    /// ```rust
    /// let status = StatusEvent::builder()
    ///     .id(42)
    ///     .build();
    ///
    /// assert_eq!(status.id().as_u64(), Some(42));
    /// ```
    pub fn id<T>(mut self, id: T) -> Self
    where
        T: IntoId,
    {
        self.status_event.id = id.into_id();
        self
    }

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
    /// - Preserves the stored [`Id`] value.
    ///
    /// Note:
    /// - No validation is performed.
    /// - Message, event, and path are optional.
    /// - A status event may be constructed with or without an ID.
    pub fn build(self) -> StatusEvent {
        self.status_event
    }
}

impl Id {
    pub fn custom<T>(value: T) -> Self
    where
        T: Any + Send + Sync + 'static,
    {
        Id::Custom(Arc::new(value))
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any + Send + Sync + 'static,
    {
        match self {
            Id::Custom(v) => v.downcast_ref::<T>(),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&str> {
        match self {
            Id::String(id) => Some(id),
            _ => None,
        }
    }

    pub fn as_u64(&self) -> Option<u64> {
        match self {
            Id::U64(id) => Some(*id),
            _ => None,
        }
    }
}

impl IntoId for Id {
    fn into_id(self) -> Id {
        self
    }
}

impl IntoId for u64 {
    fn into_id(self) -> Id {
        Id::U64(self)
    }
}

impl IntoId for String {
    fn into_id(self) -> Id {
        Id::String(self)
    }
}

//! Core status model.
//!
//! This module defines the data structures used to describe a single status
//! update.
//!
//! A status consists of an [`Event`], which stores the information to display,
//! such as the current action, progress, message, and optional filesystem path.
//!
//! [`Status`] acts as the container for the current event and provides:
//!
//! - Access to the stored event.
//! - Event replacement and resetting.
//! - Default text formatting through [`Display`].
//! - Custom formatting through [`StatusFormatter`].
//!
//! Most users will construct an [`Event`] using its builder and then create a
//! [`Status`] from it.
//!
//! ```rust
//! use simple_status::{Event, Status};
//!
//! let event = Event::builder()
//!     .action("Build")
//!     .current(2)
//!     .total(5)
//!     .message("Compiling")
//!     .build();
//!
//! let status = Status::new(event);
//!
//! println!("{status}");
//! ```
//!
//! Module summary.
//!
//! Doc:
//! - Explains the public API.
//! - Describes how users should use this module.
//!
//! Note:
//! - Internal implementation notes.
//! - Design decisions.
//! - Things to remember when modifying this module.
//!...

mod event;
mod renderer;
pub use event::Event;
use std::borrow::Cow;
use std::path::PathBuf;

/// A formatter for converting a [`Status`] into a string.
///
/// This trait allows applications to define custom output formats without
/// changing the underlying status data.
///
/// Any closure or function matching `Fn(&Status) -> String` automatically
/// implements this trait.
///
/// # Example
///
/// ```rust
/// use simple_status::{Event, Status};
///
/// let status = Status::new(
///     Event::builder()
///         .message("Finished")
///         .build(),
/// );
///
/// let text = status.format_with(|s| {
///     format!("Status: {}", s)
/// });
/// ```
// pub trait StatusFormatter {
//     fn format(&self, status: &StatusEvent) -> String;
// }

// impl<F> StatusFormatter for F
// where
//     F: Fn(&StatusEvent) -> String,
// {
//     fn format(&self, status: &StatusEvent) -> String {
//         (self)(status)
//     }
// }

pub trait StatusEventRenderer {
    type Output;

    fn render(&self, status: &StatusEvent) -> Self::Output;
}

impl<F, O> StatusEventRenderer for F
where
    F: Fn(&StatusEvent) -> O,
{
    type Output = O;

    fn render(&self, status: &StatusEvent) -> Self::Output {
        (self)(status)
    }
}

/// Represents the current status.
///
/// A `Status` owns a single [`Event`] describing the current state of an
/// operation.
///
/// The stored event can be replaced, reset, or formatted for display.
///
/// By default, formatting uses the implementation of [`Display`], while
/// [`Status::format_with`] allows custom formatting.
/// Represents the current status.
///
/// Doc:
/// - A `Status` owns exactly one `Event`.
/// - It is the primary object passed around the library.
/// - Default formatting is provided through `Display`.
///
/// Note:
/// - `Status` intentionally owns `Event` instead of borrowing it.
/// - Keeping ownership simplifies the API and avoids lifetime propagation.
#[derive(Debug, Default, Clone)]
pub struct StatusEvent {
    message: Option<Cow<'static, str>>,
    event: Event,
    path: Option<PathBuf>,
}

impl StatusEvent {
    // /// Creates a new status from an event.
    // ///
    // /// Doc:
    // /// - Constructs a `Status` from an existing `Event`.
    // ///
    // /// Note:
    // /// - No validation is performed.
    // /// - The provided event becomes the current status.
    // pub fn new(message: Option<String>, event: Event, path: Option<PathBuf>) -> Self {
    //     Self {
    //         message,
    //         event,
    //         path,
    //     }
    // }

    pub fn builder() -> StatusEventBuilder {
        StatusEventBuilder::new()
    }

    /// Returns the status message, if present.
    pub fn message(&self) -> Option<&str> {
        self.message.as_deref()
    }

    // /// Sets the event message.
    // pub fn message_mut(&mut self, message: impl Into<String>) {
    //     self.message = Some(message.into())
    // }

    // /// Replaces the current event with an empty event.
    // ///
    // /// After calling this method, all event fields are cleared.
    // pub fn reset_event(&mut self) {
    //     self.event = Event::default();
    // }

    /// Returns a shared reference to the current event.
    pub fn event(&self) -> &Event {
        &self.event
    }

    // /// Returns a mutable reference to the current event.
    // ///
    // /// This allows modifying the stored event without replacing it.
    // pub fn event_mut(&mut self) -> &mut Event {
    //     &mut self.event
    // }

    // /// Replaces the current event.
    // pub fn set_event(&mut self, event: Event) {
    //     self.event = event;
    // }

    /// Returns the associated filesystem path, if present.
    pub fn path(&self) -> Option<&PathBuf> {
        self.path.as_ref()
    }

    // /// Sets the associated filesystem path.
    // pub fn path_mut(&mut self, path: PathBuf) {
    //     self.path = Some(path);
    // }

    /// Formats this status using a custom formatter.
    ///
    /// The formatter may be any type implementing [`StatusFormatter`], including
    /// closures and functions.
    ///
    /// # Example
    ///
    /// ```rust
    /// # use simple_status::{Event, Status};
    /// let status = Status::new(Event::builder().message("Done").build());
    ///
    /// let text = status.format_with(|status| {
    ///     format!("> {}", status)
    /// });
    /// ```
    // pub fn format_with<F>(&self, f: F) -> String
    // where
    //     F: StatusFormatter,
    // {
    //     f.format(self)
    // }
    pub fn render_with<R>(&self, renderer: R) -> R::Output
    where
        R: StatusEventRenderer,
    {
        renderer.render(self)
    }
}

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

    pub fn build(self) -> StatusEvent {
        self.status_event
    }
}

//! Core status model.
//!
//! This module defines the data structures used to describe a single status
//! update.
//!
//! A status consists of an [`Event`], which stores the information to display,
//! such as the current stage, progress, message, and optional filesystem path.
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
//!     .stage("Build")
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
pub use event::Event;

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
pub trait StatusFormatter {
    fn format(&self, status: &Status) -> String;
}

impl<F> StatusFormatter for F
where
    F: Fn(&Status) -> String,
{
    fn format(&self, status: &Status) -> String {
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
pub struct Status {
    event: Event,
}

impl Status {
    /// Creates a new status from an event.
    ///
    /// Doc:
    /// - Constructs a `Status` from an existing `Event`.
    ///
    /// Note:
    /// - No validation is performed.
    /// - The provided event becomes the current status.
    pub fn new(event: Event) -> Self {
        Self { event }
    }

    /// Replaces the current event with an empty event.
    ///
    /// After calling this method, all event fields are cleared.
    pub fn reset_event(&mut self) {
        self.event = Event::default();
    }

    /// Returns a shared reference to the current event.
    pub fn event(&self) -> &Event {
        &self.event
    }

    /// Returns a mutable reference to the current event.
    ///
    /// This allows modifying the stored event without replacing it.
    pub fn event_mut(&mut self) -> &mut Event {
        &mut self.event
    }

    /// Replaces the current event.
    pub fn set_event(&mut self, event: Event) {
        self.event = event;
    }

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
    pub fn format_with<F>(&self, f: F) -> String
    where
        F: StatusFormatter,
    {
        f.format(self)
    }
}

impl std::fmt::Display for Status {
    // Note:
    // Formatting order is fixed:
    //
    //     [stage] current/total message path
    //
    // Missing fields are skipped completely.
    //
    // Keep this implementation allocation-free. Avoid building intermediate
    // strings unless necessary.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = &self.event();

        if let Some(stage) = &e.stage() {
            write!(f, "[{}] ", stage)?;
        }

        if let (Some(c), Some(t)) = (e.current(), e.total()) {
            write!(f, "{}/{} ", c, t)?;
        } else if let Some(c) = e.current() {
            write!(f, "{} ", c)?;
        } else if let Some(t) = e.total() {
            write!(f, "{} ", t)?;
        }

        if let Some(msg) = &e.message() {
            write!(f, "{}", msg)?;
        }

        if let Some(path) = &e.path() {
            write!(f, " {}", path.display())?;
        }

        Ok(())
    }
}

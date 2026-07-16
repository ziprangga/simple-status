//! Event model.
//!
//! Defines the event data stored by [`StatusEvent`].
//!
//! An [`Event`] represents the state of an operation, such as the current
//! action being performed and optional progress information.
//!
//! Each field is optional, allowing callers to provide only the information
//! relevant to the current operation.
//!
//! [`EventBuilder`] provides a fluent API for constructing an [`Event`].
//!
//! Doc:
//! - Defines the core event data model.
//! - Describes the operation currently being performed.
//! - Supports optional progress reporting.
//!
//! Note:
//! - `Event` intentionally stores only data.
//! - Rendering and presentation are handled separately by
//!   [`StatusEventRenderer`] implementations.
//! - The same event may be rendered differently depending on the application
//!   or output target.
//!..

use std::borrow::Cow;

/// A single operation event.
///
/// An `Event` describes the state of an operation, such as the current action
/// and optional progress information.
///
/// Doc:
/// - Stores operation-specific status information.
/// - May contain an action label.
/// - May contain progress information through `current` and `total`.
/// - All fields are optional.
///
/// Note:
/// - `Event` is a pure data type.
/// - It contains no rendering, formatting, or presentation logic.
/// - `Event` is typically stored within a [`StatusEvent`].
/// - Instances are generally constructed through [`EventBuilder`].
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Event {
    action: Option<Cow<'static, str>>,
    current: Option<usize>,
    total: Option<usize>,
}

impl Event {
    /// Creates a new [`EventBuilder`].
    ///
    /// Doc:
    /// - Recommended way to construct an [`Event`].
    /// - Starts with all fields unset.
    ///
    /// Note:
    /// - Builder methods may be chained fluently.
    /// - No allocation occurs unless required by supplied values.
    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }

    /// Returns the action label, if present.
    ///
    /// Doc:
    /// - Identifies the operation being performed.
    /// - Common examples include `"Build"`, `"Download"`, or `"Upload"`.
    ///
    /// Note:
    /// - Action names are application-defined.
    pub fn action(&self) -> Option<&str> {
        self.action.as_deref()
    }

    /// Returns the current progress value, if present.
    ///
    /// Doc:
    /// - Represents the current position within an operation.
    ///
    /// Note:
    /// - Meaning is application-defined.
    /// - Often used together with [`Event::total`].
    pub fn current(&self) -> Option<usize> {
        self.current
    }

    /// Returns the total progress value, if present.
    ///
    /// Doc:
    /// - Represents the expected completion target.
    ///
    /// Note:
    /// - Often paired with [`Event::current`] to calculate progress.
    pub fn total(&self) -> Option<usize> {
        self.total
    }
}

/// Builder for constructing [`Event`] values.
///
/// Doc:
/// - Supports incremental construction of an event.
/// - Allows optional fields to be configured fluently.
/// - Produces a fully owned [`Event`].
///
/// Note:
/// - The builder owns the event being constructed.
/// - Methods consume and return `Self` for ergonomic chaining.
/// - No validation occurs during construction.
#[derive(Debug, Clone)]
pub struct EventBuilder {
    event: Event,
}

impl EventBuilder {
    /// Creates an empty event builder.
    ///
    /// Doc:
    /// - Initializes a builder with all event fields unset.
    ///
    /// Note:
    /// - Equivalent to starting from `Event::default()`.
    pub fn new() -> Self {
        Self {
            event: Event::default(),
        }
    }

    /// Sets the event action.
    pub fn action(mut self, action: impl Into<Cow<'static, str>>) -> Self {
        self.event.action = Some(action.into());
        self
    }

    /// Sets the current progress value.
    pub fn current(mut self, current: usize) -> Self {
        self.event.current = Some(current);
        self
    }

    /// Sets the total progress value.
    pub fn total(mut self, total: usize) -> Self {
        self.event.total = Some(total);
        self
    }

    //// Builds the [`Event`].
    ///
    /// Doc:
    /// - Consumes the builder.
    /// - Returns the constructed event.
    ///
    /// Note:
    /// - No validation is performed.
    /// - All fields are optional.
    /// - An entirely empty `Event` is considered valid.
    pub fn build(self) -> Event {
        self.event
    }
}

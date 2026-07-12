//! Doc:
//! Defines the event model used by `Status`.
//!
//! An `Event` represents a single status update. Each field is optional,
//! allowing callers to provide only the information relevant to the current
//! operation.
//!
//! `EventBuilder` provides a fluent API for constructing an `Event`.
//!
//! Note:
//! `Event` intentionally stores only data and contains no formatting or
//! rendering logic. Display formatting is handled by `Status`, allowing the
//! same event to be rendered differently when needed.
//!..

use std::borrow::Cow;

/// A single status event.
///
/// Doc:
/// Stores the information describing the current state of an operation.
///
/// Every field is optional. Fields that are not provided simply represent
/// unavailable information.
///
/// Note:
/// `Event` is intentionally immutable after construction. Modification is
/// performed by creating a new event or replacing the event stored inside
/// `Status`.
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
    /// The recommended way to construct an [`Event`].
    pub fn builder() -> EventBuilder {
        EventBuilder::new()
    }

    /// Returns the action name, if present.
    ///
    /// Doc:
    /// actions are free-form labels such as `"Build"` or `"Download"`.
    pub fn action(&self) -> Option<&str> {
        self.action.as_deref()
    }

    /// Returns the current progress value, if present.
    pub fn current(&self) -> Option<usize> {
        self.current
    }

    /// Returns the total progress value, if present.
    pub fn total(&self) -> Option<usize> {
        self.total
    }
}

/// Builder for constructing an [`Event`].
///
/// Doc:
/// Allows optional event fields to be configured using a fluent builder API.
///
/// Note:
/// The builder owns the event being constructed. Each builder method consumes
/// and returns `Self`, making it convenient to chain method calls.
#[derive(Debug, Clone)]
pub struct EventBuilder {
    event: Event,
}

impl EventBuilder {
    /// Creates an empty event builder.
    ///
    /// Doc:
    /// All event fields are initially unset.
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

    /// Builds the [`Event`].
    ///
    /// Doc:
    /// Consumes the builder and returns the constructed event.
    ///
    /// Note:
    /// No validation is performed. All fields are optional, so an empty `Event` is
    /// considered valid.
    pub fn build(self) -> Event {
        self.event
    }
}

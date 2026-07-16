//! Default rendering implementation for `simple_status`.
//!
//! This module provides the default textual representation of a
//! [`StatusEvent`].
//!
//! [`DefaultRenderer`] converts a status event into a `String` using a stable
//! formatting scheme. The [`Display`] implementation for [`StatusEvent`]
//! delegates to this renderer.
//!
//! Doc:
//! - Defines the default text renderer for [`StatusEvent`].
//! - Provides the implementation used by [`Display`].
//! - Centralizes formatting behavior.
//!
//! Note:
//! - Rendering is intentionally separated from status-event storage.
//! - Alternative renderers can be implemented through
//!   [`StatusEventRenderer`].
//! - Formatting behavior should remain consistent between
//!   [`DefaultRenderer`] and [`Display`].
//!..

use crate::status_event::StatusEvent;
use crate::status_event::StatusEventRenderer;

/// Default text renderer for [`StatusEvent`].
///
/// Produces the default string representation for a status event.
///
/// Doc:
/// - Converts a [`StatusEvent`] into a `String`.
/// - Includes available event metadata.
/// - Omits fields that are not present.
///
/// Note:
/// - Rendering order is intentionally stable.
/// - Formatting behavior is kept separate from status-event storage.
/// - Applications may define alternative renderers by implementing
///   [`StatusEventRenderer`].
pub struct DefaultRenderer;

impl StatusEventRenderer for DefaultRenderer {
    type Output = String;

    /// Renders a [`StatusEvent`] into its default textual form.
    ///
    /// Doc:
    /// - Includes the message when present.
    /// - Includes the event action when present.
    /// - Includes progress information when available.
    /// - Includes the associated filesystem path when present.
    ///
    /// Note:
    /// - Missing fields are omitted entirely.
    /// - The exact formatting is considered part of the default renderer's
    ///   behavior and may differ from custom renderers.
    fn render(&self, se: &StatusEvent) -> String {
        let mut out = String::new();

        if let Some(msg) = se.message() {
            out.push('[');
            out.push_str(msg);
            out.push_str("] ");
        }

        if let Some(action) = se.event().action() {
            out.push('[');
            out.push_str(action);
            out.push_str("] ");
        }

        match (se.event().current(), se.event().total()) {
            (Some(current), Some(total)) => {
                out.push_str(&format!("[{current}/{total}] "));
            }
            (Some(current), None) => {
                out.push_str(&format!("[{current}] "));
            }
            (None, Some(total)) => {
                out.push_str(&format!("[/{total}] "));
            }
            (None, None) => {}
        }

        if let Some(path) = se.path() {
            out.push_str(" | path=");
            out.push_str(&path.display().to_string());
        }

        out
    }
}

/// Formats a [`StatusEvent`] using [`DefaultRenderer`].
///
/// Doc:
/// - Provides the default textual representation of a status event.
/// - Delegates formatting to [`DefaultRenderer`].
///
/// Note:
/// - Formatting logic is intentionally centralized in
///   [`DefaultRenderer`].
/// - This implementation should remain behaviorally equivalent to
///   calling `DefaultRenderer.render(...)`.
impl std::fmt::Display for StatusEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let text = DefaultRenderer.render(self);
        f.write_str(&text)
    }
}

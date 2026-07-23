//! Default rendering implementation for `simple_status`.
//!
//! This module provides the default textual representation of a
//! [`StatusEvent`].
//!
//! [`DefaultDisplayRenderer`] converts a status event into a `String` using a
//! stable formatting scheme. The [`Display`] implementation for
//! [`StatusEvent`] delegates to this renderer.
//!
//! Doc:
//! - Defines the default text renderer for [`StatusEvent`].
//! - Provides the formatting implementation used by [`Display`].
//! - Centralizes default textual formatting behavior.
//!
//! Note:
//! - Rendering is intentionally separated from status-event storage.
//! - Alternative renderers can be implemented through
//!   [`Renderer<StatusEvent>`].
//! - Formatting behavior should remain consistent between
//!   [`DefaultDisplayRenderer`] and [`Display`].
//!..

use crate::renderer::Renderer;
use crate::status_event::StatusEvent;

/// Default text renderer for [`StatusEvent`].
///
/// Produces the default string representation of a status event.
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
///   [`Renderer<StatusEvent>`].
pub struct DefaultDisplayRenderer;

impl Renderer<StatusEvent> for DefaultDisplayRenderer {
    type Output = String;

    /// Renders a [`StatusEvent`] into its default textual representation.
    ///
    /// Doc:
    /// - Includes the status message when present.
    /// - Includes the event action when present.
    /// - Includes progress information when available.
    /// - Includes the associated filesystem path when present.
    ///
    /// Note:
    /// - Missing fields are omitted entirely.
    /// - Output ordering is intentionally stable.
    /// - Formatting behavior may differ from application-defined renderers.
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

/// Formats a [`StatusEvent`] using [`DefaultDisplayRenderer`].
///
/// Doc:
/// - Provides the default textual representation of a status event.
/// - Delegates formatting to [`DefaultDisplayRenderer`].
/// - Enables use with standard formatting APIs.
///
/// Note:
/// - Formatting logic is intentionally centralized in
///   [`DefaultDisplayRenderer`].
/// - This implementation should remain behaviorally equivalent to
///   calling `DefaultDisplayRenderer.render(...)`.
/// - Applications requiring alternative formatting should use a custom
///   renderer through the rendering framework.
impl std::fmt::Display for StatusEvent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&DefaultDisplayRenderer.render(self))
    }
}

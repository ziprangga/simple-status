// Doc:
// Internal helpers used exclusively by the crate's procedural interface
// (macros).
//
// Note:
// These items are not part of the public API and may change without notice.
#[doc(hidden)]
mod __private {
    use crate::Emitter;
    use crate::Event;
    use crate::StatusEvent;
    use crate::emit_async;
    use crate::emit_sync;
    use crate::status_emit_async;
    use crate::status_emit_sync;
    use std::borrow::Cow;
    use std::path::PathBuf;

    fn int_event_build(
        action: Option<Cow<'static, str>>,
        current: Option<usize>,
        total: Option<usize>,
    ) -> Event {
        let mut builder = Event::builder();

        if let Some(act) = action {
            builder = builder.action(act);
        }
        if let Some(current) = current {
            builder = builder.current(current);
        }
        if let Some(total) = total {
            builder = builder.total(total);
        }

        builder.build()
    }

    fn int_status_event_build(
        action: Option<Cow<'static, str>>,
        current: Option<usize>,
        total: Option<usize>,
        message: Option<Cow<'static, str>>,
        path: Option<PathBuf>,
    ) -> StatusEvent {
        let event = int_event_build(action, current, total);
        let mut status = StatusEvent::builder();
        if let Some(m) = message {
            status = status.message(m)
        }

        status = status.event(event);

        if let Some(p) = path {
            status = status.path(p)
        }

        status.build()
    }

    // =====================================================

    pub fn global_emit_sync(se: StatusEvent) {
        emit_sync(se);
    }

    pub async fn global_emit_async(se: StatusEvent) {
        emit_async(se).await;
    }

    pub fn ind_status_emit_sync(emitter: &Emitter, se: StatusEvent) {
        status_emit_sync(emitter, se);
    }

    pub async fn ind_status_emit_async(emitter: &Emitter, se: StatusEvent) {
        status_emit_async(emitter, se).await;
    }

    pub fn opt_cow(value: Option<impl Into<Cow<'static, str>>>) -> Option<Cow<'static, str>> {
        value.map(Into::into)
    }

    pub fn format_message(args: std::fmt::Arguments<'_>) -> Option<Cow<'static, str>> {
        Some(Cow::Owned(args.to_string()))
    }

    /// Constructs a `StatusEvent` object from optional fields passed by macros.
    pub fn build_status_event(
        action: Option<Cow<'static, str>>,
        current: Option<usize>,
        total: Option<usize>,
        message: Option<Cow<'static, str>>,
        path: Option<PathBuf>,
    ) -> StatusEvent {
        int_status_event_build(action, current, total, message, path)
    }
}

pub use self::__private::build_status_event;
pub use self::__private::format_message;
pub use self::__private::global_emit_async;
pub use self::__private::global_emit_sync;
pub use self::__private::ind_status_emit_async;
pub use self::__private::ind_status_emit_sync;
pub use self::__private::opt_cow;

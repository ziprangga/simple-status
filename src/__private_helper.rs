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
    use std::path::PathBuf;

    fn int_event_build(
        stage: Option<String>,
        current: Option<usize>,
        total: Option<usize>,
        message: Option<String>,
        path: Option<PathBuf>,
    ) -> Event {
        let mut builder = Event::builder();

        if let Some(stage) = stage {
            builder = builder.stage(stage);
        }
        if let Some(current) = current {
            builder = builder.current(current);
        }
        if let Some(total) = total {
            builder = builder.total(total);
        }
        if let Some(message) = message {
            builder = builder.message(message);
        }
        if let Some(path) = path {
            builder = builder.path(path);
        }

        builder.build()
    }

    fn int_status_event_build(
        stage: Option<String>,
        current: Option<usize>,
        total: Option<usize>,
        message: Option<String>,
        path: Option<PathBuf>,
    ) -> StatusEvent {
        let event = int_event_build(stage, current, total, message, path);
        StatusEvent::new(event)
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

    /// Constructs a `StatusEvent` object from optional fields passed by macros.
    pub fn build_status_event(
        stage: Option<String>,
        current: Option<usize>,
        total: Option<usize>,
        message: Option<String>,
        path: Option<PathBuf>,
    ) -> StatusEvent {
        int_status_event_build(stage, current, total, message, path)
    }
}

pub use self::__private::build_status_event;
pub use self::__private::global_emit_async;
pub use self::__private::global_emit_sync;
pub use self::__private::ind_status_emit_async;
pub use self::__private::ind_status_emit_sync;

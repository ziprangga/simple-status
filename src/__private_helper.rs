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
    use crate::IntoEmitter;
    use crate::StatusEvent;
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

    fn int_status_build(
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

    pub fn into_emitter_opt<'a, E>(emitter: E) -> Option<&'a Emitter>
    where
        E: IntoEmitter<'a>,
    {
        emitter.into_emitter()
    }

    /// Constructs a `StatusEvent` object from optional fields passed by macros.
    pub fn build_status(
        stage: Option<String>,
        current: Option<usize>,
        total: Option<usize>,
        message: Option<String>,
        path: Option<PathBuf>,
    ) -> StatusEvent {
        int_status_build(stage, current, total, message, path)
    }
}

pub use self::__private::build_status;
pub use self::__private::into_emitter_opt;

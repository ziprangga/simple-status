// Doc:
// Internal helpers used exclusively by the crate's procedural interface
// (macros).
//
// Note:
// These items are not part of the public API and may change without notice.
#[doc(hidden)]
mod __private {
    // use crate::Emitter;
    use crate::Event;
    use crate::StatusEvent;
    use std::path::PathBuf;

    // /// Conversion into an optional emitter reference.
    // ///
    // /// Doc:
    // /// Provides a uniform way to accept either an `&Emitter` or an
    // /// `Option<&Emitter>` and normalize them into `Option<&Emitter>`.
    // ///
    // /// Note:
    // /// This trait is primarily intended for API ergonomics
    // pub trait IntoEmitter<'a> {
    //     /// Converts this value into an optional emitter reference.
    //     ///
    //     /// Note:
    //     /// Implementations may return `None` when no emitter is available.
    //     /// The conversion consumes `self`, though implementors are generally
    //     /// lightweight reference-based types.
    //     fn into_emitter(self) -> Option<&'a Emitter>;
    // }

    // impl<'a> IntoEmitter<'a> for Option<&'a Emitter> {
    //     /// Returns the emitter unchanged.
    //     ///
    //     /// Note:
    //     /// This implementation allows APIs accepting `IntoEmitter` to receive an
    //     /// optional emitter directly.
    //     fn into_emitter(self) -> Option<&'a Emitter> {
    //         self
    //     }
    // }

    // impl<'a> IntoEmitter<'a> for &'a Emitter {
    //     /// Wraps the emitter in `Some`.
    //     ///
    //     /// Note:
    //     /// This implementation allows APIs accepting `IntoEmitter` to receive a
    //     /// concrete emitter reference without requiring callers to construct
    //     /// `Some(...)` explicitly.
    //     fn into_emitter(self) -> Option<&'a Emitter> {
    //         Some(self)
    //     }
    // }

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

    // pub fn into_emitter_opt<'a, E>(emitter: E) -> Option<&'a Emitter>
    // where
    //     E: IntoEmitter<'a>,
    // {
    //     emitter.into_emitter()
    // }

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
// pub use self::__private::into_emitter_opt;

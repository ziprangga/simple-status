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
    use crate::NoId;
    use crate::StatusEvent;
    use crate::emit_async;
    use crate::emit_sync;
    use crate::status_emit_async;
    use crate::status_emit_sync;
    use std::borrow::Cow;
    use std::path::PathBuf;

    pub trait IntoCowOpt {
        fn into_cow_opt(self) -> Option<Cow<'static, str>>;
    }

    impl IntoCowOpt for &'static str {
        fn into_cow_opt(self) -> Option<Cow<'static, str>> {
            Some(Cow::Borrowed(self))
        }
    }

    impl IntoCowOpt for String {
        fn into_cow_opt(self) -> Option<Cow<'static, str>> {
            Some(Cow::Owned(self))
        }
    }

    impl<T: IntoCowOpt> IntoCowOpt for Option<T> {
        fn into_cow_opt(self) -> Option<Cow<'static, str>> {
            self.and_then(|x| x.into_cow_opt())
        }
    }

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

    fn int_into_cow_opt(m: impl IntoCowOpt) -> Option<Cow<'static, str>> {
        m.into_cow_opt()
    }

    fn int_status_event_build_id<I>(
        id: I,
        action: impl IntoCowOpt,
        current: Option<usize>,
        total: Option<usize>,
        message: impl IntoCowOpt,
        path: Option<PathBuf>,
    ) -> StatusEvent<I> {
        let action_opt = int_into_cow_opt(action);
        let event = int_event_build(action_opt, current, total);

        let mut status_event = StatusEvent::builder().id(id);

        let message_opt = int_into_cow_opt(message);
        if let Some(m) = message_opt {
            status_event = status_event.message(m)
        }

        status_event = status_event.event(event);

        if let Some(p) = path {
            status_event = status_event.path(p)
        }

        status_event.build()
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

    /// Constructs a `StatusEvent` object from optional fields passed by macros without id.
    pub fn build_status_event_no_id(
        action: impl IntoCowOpt,
        current: Option<usize>,
        total: Option<usize>,
        message: impl IntoCowOpt,
        path: Option<PathBuf>,
    ) -> StatusEvent<NoId> {
        int_status_event_build_id(NoId, action, current, total, message, path)
    }

    /// Constructs a `StatusEvent` object from optional fields passed by macros with id.
    pub fn build_status_event_id<I>(
        id: I,
        action: impl IntoCowOpt,
        current: Option<usize>,
        total: Option<usize>,
        message: impl IntoCowOpt,
        path: Option<PathBuf>,
    ) -> StatusEvent<I> {
        int_status_event_build_id(id, action, current, total, message, path)
    }
}

#[doc(hidden)]
pub use self::__private::build_status_event_id;

#[doc(hidden)]
pub use self::__private::build_status_event_no_id;

#[doc(hidden)]
pub use self::__private::global_emit_async;

#[doc(hidden)]
pub use self::__private::global_emit_sync;

#[doc(hidden)]
pub use self::__private::ind_status_emit_async;

#[doc(hidden)]
pub use self::__private::ind_status_emit_sync;

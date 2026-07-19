//! Doc:
//! Internal helper utilities used by crate macros.
//!
//! Provides:
//! - StatusEvent construction helpers.
//! - Emitter conversion helpers.
//! - Global and emitter-based emission helpers.
//!
//! Note:
//! - Not part of the public API.
//! - Intended exclusively for macro expansion.
//! - Public code should use the crate's macros and APIs instead.
//! - Function signatures may change without notice.
//!
//! These helpers are tightly coupled to `StatusEvent`
//! and are not intended to support arbitrary channel
//! value types.
//!...

#[doc(hidden)]
mod __private {
    use crate::ChannelsBus;
    use crate::Event;
    use crate::Id;
    use crate::IntoId;
    use crate::StatusEmitter;
    use crate::StatusEvent;
    use std::borrow::Cow;
    use std::path::PathBuf;
    use std::sync::Arc;

    /// Converts various emitter references into a common optional form.
    ///
    /// Doc:
    /// Normalizes emitter arguments accepted by crate macros.
    ///
    /// Note:
    /// - Used internally by macro expansion.
    /// - Supports direct and optional emitter references.
    /// - Not intended for public use.
    pub trait IntoEmitter<'a> {
        fn into_emitter(self) -> Option<&'a StatusEmitter>;
    }

    impl<'a> IntoEmitter<'a> for Option<&'a StatusEmitter> {
        fn into_emitter(self) -> Option<&'a StatusEmitter> {
            self
        }
    }

    impl<'a> IntoEmitter<'a> for &'a StatusEmitter {
        fn into_emitter(self) -> Option<&'a StatusEmitter> {
            Some(self)
        }
    }

    impl<'a> IntoEmitter<'a> for Option<&'a Arc<StatusEmitter>> {
        fn into_emitter(self) -> Option<&'a StatusEmitter> {
            self.map(Arc::as_ref)
        }
    }

    impl<'a> IntoEmitter<'a> for &'a Arc<StatusEmitter> {
        fn into_emitter(self) -> Option<&'a StatusEmitter> {
            Some(self.as_ref())
        }
    }

    /// Converts supported message inputs into an optional `Cow<str>`.
    ///
    /// Doc:
    /// Normalizes macro string arguments before status construction.
    ///
    /// Note:
    /// - Used internally by macro expansion.
    /// - Supports borrowed, owned, and optional string values.
    /// - Not part of the public API.
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

    // These functions centralize event emission logic so the macros can
    // operate on any compatible emitter implementation.
    fn bus_emit_sync(bus: &ChannelsBus, se: StatusEvent) {
        bus.channels().emit_sync(se);
    }

    async fn bus_emit_async(bus: &ChannelsBus, se: StatusEvent) {
        bus.channels().emit_async(se).await;
    }

    fn emitter_emit_sync(emitter: &StatusEmitter, se: StatusEvent) {
        emitter.emit_sync(se);
    }

    async fn emitter_emit_async(emitter: &StatusEmitter, se: StatusEvent) {
        emitter.emit_async(se).await;
    }

    /// Constructs an `Event` from normalized optional fields.
    ///
    /// Note:
    /// Internal helper used during macro-generated status construction.
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

    /// Constructs a `StatusEvent` from normalized macro inputs.
    ///
    /// Doc:
    /// Centralizes status construction logic shared by all macro forms.
    ///
    /// Note:
    /// - Accepts any identifier implementing `IntoId`.
    /// - Used exclusively by internal macro helpers.
    /// - Not part of the public API.
    fn int_status_event_build_id(
        id: impl IntoId,
        action: impl IntoCowOpt,
        current: Option<usize>,
        total: Option<usize>,
        message: impl IntoCowOpt,
        path: Option<PathBuf>,
    ) -> StatusEvent {
        let action_opt = int_into_cow_opt(action);
        let event = int_event_build(action_opt, current, total);

        let mut status_event = StatusEvent::builder();

        status_event = status_event.id(id);

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

    /// Converts a macro emitter argument into an optional emitter reference.
    ///
    /// Note:
    /// Internal helper used by `status_emit!` macro expansion.
    pub fn into_opt_emitter<'a>(emitter: impl IntoEmitter<'a>) -> Option<&'a StatusEmitter> {
        emitter.into_emitter()
    }

    /// Emits a status event through a `ChannelsBus`.
    ///
    /// Note:
    /// Internal helper used by macro expansion.
    pub fn global_emit_sync(bus: &'static ChannelsBus, se: StatusEvent) {
        bus_emit_sync(bus, se);
    }

    pub async fn global_emit_async(bus: &'static ChannelsBus, se: StatusEvent) {
        bus_emit_async(bus, se).await;
    }

    /// Emits a status event through a provided emitter when available.
    ///
    /// Note:
    /// Internal helper used by macro expansion.
    /// A missing emitter results in a no-op.
    pub fn ind_status_emit_sync(emitter: Option<&StatusEmitter>, se: StatusEvent) {
        if let Some(emit) = emitter {
            emitter_emit_sync(emit, se);
        }
    }

    pub async fn ind_status_emit_async(emitter: Option<&StatusEmitter>, se: StatusEvent) {
        if let Some(emit) = emitter {
            emitter_emit_async(emit, se).await;
        }
    }

    /// Constructs a `StatusEvent` with the default identifier.
    ///
    /// Doc:
    /// Used by macro forms that do not provide an `id` field.
    ///
    /// Note:
    /// - Sets the identifier to `Id::None`.
    /// - Equivalent to constructing a status event without an explicit ID.
    /// - Internal helper for macro expansion.
    pub fn build_status_event_no_id(
        action: impl IntoCowOpt,
        current: Option<usize>,
        total: Option<usize>,
        message: impl IntoCowOpt,
        path: Option<PathBuf>,
    ) -> StatusEvent {
        int_status_event_build_id(Id::None, action, current, total, message, path)
    }

    /// Constructs a `StatusEvent` with an identifier.
    ///
    /// Doc:
    /// Entry point used by macro forms that provide an `id` field.
    ///
    /// Note:
    /// - Accepts any type implementing `IntoId`.
    /// - Internal helper for macro expansion.
    pub fn build_status_event_id(
        id: impl IntoId,
        action: impl IntoCowOpt,
        current: Option<usize>,
        total: Option<usize>,
        message: impl IntoCowOpt,
        path: Option<PathBuf>,
    ) -> StatusEvent {
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

#[doc(hidden)]
pub use self::__private::into_opt_emitter;

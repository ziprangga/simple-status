#[doc(hidden)]
#[macro_export]
#[clippy::format_args]
macro_rules! __into_emitter {
    // Standardizes the provided emitter expression by converting it
    // into the required emitter type via the internal `IntoEmitter` trait.
    ($emitter:expr) => {{ $crate::__private_helper::IntoEmitter::into_emitter($emitter) }};
}

#[macro_export]
#[clippy::format_args]
macro_rules! status {
    (
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr $(,)?)?
    ) => {{

        $crate::__private_helper::build_status(
            $crate::status!(@opt_str $($stage)?),
            $crate::status!(@opt_usize $($current)?),
            $crate::status!(@opt_usize $($total)?),
            $crate::status!(@opt_str $($message)?),
            $crate::status!(@opt_path $($path)?),
        )
    }};

    (@opt_str $value:expr) => { Some($value.into()) };
    (@opt_str) => { None };
    (@opt_usize $value:expr) => { Some($value) };
    (@opt_usize) => { None };
    (@opt_path $value:expr) => { Some($value) };
    (@opt_path) => { None };

    ($($arg:tt)+) => {{
        $crate::__private_helper::build_status(None, None, None, Some(format!($($arg)+)), None)
    }};

}

#[macro_export]
#[clippy::format_args]
macro_rules! status_emit {
    // ==================================
    // ASYNC MODE
    // ==================================

    // Instance Async (with key-value pairs)
    (async, $emitter:expr, $(stage: $stage:expr,)? $(current: $current:expr,)? $(total: $total:expr,)? $(message: $message:expr,)? $(path: $path:expr $(,)?)?) => {{
        $crate::emit_status_async(
            $crate::__into_emitter!($emitter),
            $crate::status!($(stage: $stage,)? $(current: $current,)? $(total: $total,)? $(message: $message,)? $(path: $path,)?)
        ).await;
    }};

    // Instance Async (with string format / raw arguments)
    // Triggered when the second argument is an expression but the remaining tokens are format strings
    (async, $emitter:expr, $fmt:expr, $($arg:tt)+) => {{
        $crate::emit_status_async(
            $crate::__into_emitter!($emitter),
            $crate::status!($fmt, $($arg)+)
        ).await;
    }};

    // Global Async (with key-value pairs)
    (async, stage: $stage:expr, $($rest:tt)*) => {{
        $crate::emit_async($crate::status!(stage: $stage, $($rest)*)).await;
    }};
    (async, current: $current:expr, $($rest:tt)*) => {{
        $crate::emit_async($crate::status!(current: $current, $($rest)*)).await;
    }};
    (async, total: $total:expr, $($rest:tt)*) => {{
        $crate::emit_async($crate::status!(total: $total, $($rest)*)).await;
    }};
    (async, message: $message:expr, $($rest:tt)*) => {{
        $crate::emit_async($crate::status!(message: $message, $($rest)*)).await;
    }};
    (async, path: $path:expr, $($rest:tt)*) => {{
        $crate::emit_async($crate::status!(path: $path, $($rest)*)).await;
    }};

    // Global Async (fallback for arbitrary format strings / single text)
    (async, $($arg:tt)+) => {{
        $crate::emit_async($crate::status!($($arg)+)).await;
    }};

    // ==================================
    // SYNC MODE
    // ==================================

    // Instance Sync (with key-value pairs)
    ($emitter:expr, $(stage: $stage:expr,)? $(current: $current:expr,)? $(total: $total:expr,)? $(message: $message:expr,)? $(path: $path:expr $(,)?)?) => {{
        $crate::emit_status_sync(
            $crate::__into_emitter!($emitter),
            $crate::status!($(stage: $stage,)? $(current: $current,)? $(total: $total,)? $(message: $message,)? $(path: $path,)?)
        );
    }};

    // Instance Sync (with string format / raw arguments)
    ($emitter:expr, $fmt:expr, $($arg:tt)+) => {{
        $crate::emit_status_sync(
            $crate::__into_emitter!($emitter),
            $crate::status!($fmt, $($arg)+)
        );
    }};

    // Global Sync (with key-value pairs)
    (stage: $stage:expr, $($rest:tt)*) => {{
        $crate::emit_sync($crate::status!(stage: $stage, $($rest)*));
    }};
    (current: $current:expr, $($rest:tt)*) => {{
        $crate::emit_sync($crate::status!(current: $current, $($rest)*));
    }};
    (total: $total:expr, $($rest:tt)*) => {{
        $crate::emit_sync($crate::status!(total: $total, $($rest)*));
    }};
    (message: $message:expr, $($rest:tt)*) => {{
        $crate::emit_sync($crate::status!(message: $message, $($rest)*));
    }};
    (path: $path:expr, $($rest:tt)*) => {{
        $crate::emit_sync($crate::status!(path: $path, $($rest)*));
    }};

    // Global Sync (fallback for arbitrary format strings / single text)
    ($($arg:tt)+) => {{
        $crate::emit_sync($crate::status!($($arg)+));
    }};
}

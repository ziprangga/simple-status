use crate::Event;
use crate::Status;
use std::path::PathBuf;

pub fn build_status(
    stage: Option<String>,
    current: Option<usize>,
    total: Option<usize>,
    message: Option<String>,
    path: Option<PathBuf>,
) -> Status {
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

    Status::new(builder.build())
}

#[macro_export]
macro_rules! status {
    (
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr $(,)?)?
    ) => {{

        $crate::build_status(
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
        $crate::build_status(None, None, None, Some(format!($($arg)+)), None)
    }};

}

#[macro_export]
macro_rules! status_emit {
    // ==================================
    // async mode
    // ==================================
    // GLOBAL ASYNC
    (global, async,
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr $(,)?)?
    ) => {{
        $crate::emit_async(
            $crate::status!(
                $(stage: $stage,)?
                $(current: $current,)?
                $(total: $total,)?
                $(message: $message,)?
                $(path: $path,)?
            )
        ).await;
    }};

    (global, async, $($arg:tt)+) => {{
        $crate::emit_async(
            $crate::status!($($arg)+)
        ).await;
    }};

    // ================================
    // INSTANCE ASYNC
    (ins, async, $emitter:expr,
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr $(,)?)?
    ) => {{
        $crate::emit_status_async(
            $crate::status_emit!(@opt_emitter $emitter),
            $crate::status!(
                $(stage: $stage,)?
                $(current: $current,)?
                $(total: $total,)?
                $(message: $message,)?
                $(path: $path,)?
            )
        ).await

    }};

    (ins, async, $emitter:expr, $($arg:tt)+) => {{
        $crate::emit_status_async(
            $crate::status_emit!(@opt_emitter $emitter),
            $crate::status!($($arg)+)
        ).await;
    }};

    // =================================
    // Sync mode
    // =================================
    // GLOBAL SYNC
    (global,
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr $(,)?)?
    ) => {{
        $crate::emit_sync(
            $crate::status!(
                $(stage: $stage,)?
                $(current: $current,)?
                $(total: $total,)?
                $(message: $message,)?
                $(path: $path,)?
            )
        );
    }};

    (global, $($arg:tt)+) => {{
        $crate::emit_sync(
            $crate::status!($($arg)+)
        );
    }};

    // ================================
    // INSTANCE SYNC
    (ins, $emitter:expr,
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr $(,)?)?
    ) => {{
       $crate::emit_status_sync(
           $crate::status_emit!(@opt_emitter $emitter),
           $crate::status!(
            $(stage: $stage,)?
            $(current: $current,)?
            $(total: $total,)?
            $(message: $message,)?
            $(path: $path,)?
        ))
    }};

    (ins, $emitter:expr, $($arg:tt)+) => {{
        $crate::emit_status_sync(
            $crate::status_emit!(@opt_emitter $emitter),
            $crate::status!($($arg)+)
        );
    }};

    (@opt_emitter $emitter:expr) => { $crate::IntoEmitter::into_emitter($emitter) };
    (@opt_emitter) => { None };
}

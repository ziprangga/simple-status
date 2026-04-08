use crate::Event;
use std::path::PathBuf;

pub fn build_event(
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

#[macro_export]
macro_rules! event {
    (
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{

        $crate::build_event(
            $crate::event!(@opt_str $($stage)?),
            $crate::event!(@opt_usize $($current)?),
            $crate::event!(@opt_usize $($total)?),
            $crate::event!(@opt_str $($message)?),
            $crate::event!(@opt_path $($path)?),
        )
    }};

    (@opt_str $value:expr) => { Some($value.into()) };
    (@opt_str) => { None };
    (@opt_usize $value:expr) => { Some($value) };
    (@opt_usize) => { None };
    (@opt_path $value:expr) => { Some($value) };
    (@opt_path) => { None };

    ($($arg:tt)+) => {{
        $crate::build_event(None, None, None, Some(format!($($arg)+)), None)
    }};

}

#[macro_export]
macro_rules! status {
    (
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{
        $crate::event!(
            $(stage: $stage,)?
            $(current: $current,)?
            $(total: $total,)?
            $(message: $message,)?
            $(path: $path,)?
        )
    }};

    ($($arg:tt)+) => {{ $crate::Status::new($crate::event!($($arg)+))
    }};
}

#[macro_export]
macro_rules! event_emit {
    // async mode
    (async, Some($emitter:expr),
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{
        match $emitter {
            Some(emitter) => emitter.async_emit($crate::event!(
                $(stage: $stage,)?
                $(current: $current,)?
                $(total: $total,)?
                $(message: $message,)?
                $(path: $path,)?
            )).await,
            None => {}
        }
    }};

    (async, $emitter:expr,
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{
       $emitter.async_emit($crate::event!(
            $(stage: $stage,)?
            $(current: $current,)?
            $(total: $total,)?
            $(message: $message,)?
            $(path: $path,)?
        )).await
    }};

    (async, Some($emitter:expr), $($arg:tt)+) => {{
        match $emitter {
            Some(emitter) => emitter.async_emit($crate::event!($($arg)+)).await,
            None => {},
         }
    }};

    (async, $emitter:expr, $($arg:tt)+) => {{
        $emitter.async_emit(
                $crate::event!($($arg)+)
        ).await;
    }};

    // sync mode (default)
    (Some($emitter:expr),
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{
        match $emitter {
            Some(emitter) => emitter.sync_emit($crate::event!(
                $(stage: $stage,)?
                $(current: $current,)?
                $(total: $total,)?
                $(message: $message,)?
                $(path: $path,)?
            )),
            None => {}
        }
    }};

    ($emitter:expr,
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{
       $emitter.sync_emit($crate::event!(
            $(stage: $stage,)?
            $(current: $current,)?
            $(total: $total,)?
            $(message: $message,)?
            $(path: $path,)?
        ))
    }};

    (Some($emitter:expr), $($arg:tt)+) => {{
        match $emitter {
            Some(emitter) => emitter.sync_emit($crate::event!($($arg)+)),
            None => {},
        }
    }};

    ($emitter:expr, $($arg:tt)+) => {{
        $emitter.sync_emit(
                $crate::event!($($arg)+)
        );
    }};
}

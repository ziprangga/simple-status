mod status_channel;
mod status_emrec;
mod status_event;
mod status_format;

pub use status_channel::{ChannelReceiver, ChannelSender};
pub use status_emrec::{
    StatusEmitter, StatusEmitterHandler, StatusReceiver, StatusReceiverHandler,
};
pub use status_event::StatusEvent;
pub use status_format::{StatusFormatConfig, StatusFormatter};

use std::sync::Arc;

pub fn setup_status(buffer: usize) -> (Arc<StatusEmitter>, Arc<StatusReceiver>) {
    let (tx, rx) = tokio::sync::mpsc::channel(buffer);
    let handler = Arc::new(ChannelSender::new(tx));
    let emitter = Arc::new(StatusEmitter::new(handler));

    let channel_receiver = Arc::new(ChannelReceiver::new(rx));
    let receiver = Arc::new(StatusReceiver::new(channel_receiver));
    (emitter, receiver)
}

#[derive(Debug, Default, Clone)]
pub struct Status {
    event: StatusEvent,
}

impl Status {
    pub fn new(event: StatusEvent) -> Self {
        Self { event }
    }

    pub fn event(&self) -> &StatusEvent {
        &self.event
    }

    pub fn format<F>(&self, f: F) -> String
    where
        F: StatusFormatter,
    {
        f.format(&self.event)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cfg = StatusFormatConfig::default();
        write!(f, "{}", self.format(cfg))
    }
}

// =================
// macros
// =================
use std::path::PathBuf;

pub fn build_status_event(
    stage: Option<String>,
    current: Option<usize>,
    total: Option<usize>,
    message: Option<String>,
    path: Option<PathBuf>,
) -> StatusEvent {
    let mut builder = StatusEvent::builder();

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
macro_rules! status_event {
    (
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{

        $crate::build_status_event(
            $crate::status_event!(@opt_str $($stage)?),
            $crate::status_event!(@opt_usize $($current)?),
            $crate::status_event!(@opt_usize $($total)?),
            $crate::status_event!(@opt_str $($message)?),
            $crate::status_event!(@opt_path $($path)?),
        )
    }};

    (@opt_str $value:expr) => { Some($value.into()) };
    (@opt_str) => { None };
    (@opt_usize $value:expr) => { Some($value) };
    (@opt_usize) => { None };
    (@opt_path $value:expr) => { Some($value) };
    (@opt_path) => { None };

    ($($arg:tt)+) => {{
        $crate::build_status_event(None, None, None, Some(format!($($arg)+)), None)
    }};

}

#[macro_export]
macro_rules! status {
    ($($arg:tt)+) => {{
        $crate::Status::new(
            $crate::status_event!($($arg)+)
        )
    }};
}

#[macro_export]
macro_rules! status_emit {
    // async mode
    (async, Some($emitter:expr),
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{
        match $emitter {
            Some(emitter) => emitter.emit($crate::Status::new($crate::status_event!(
                $(stage: $stage,)?
                $(current: $current,)?
                $(total: $total,)?
                $(message: $message,)?
                $(path: $path,)?
            ))).await,
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
       $emitter.emit($crate::Status::new($crate::status_event!(
            $(stage: $stage,)?
            $(current: $current,)?
            $(total: $total,)?
            $(message: $message,)?
            $(path: $path,)?
        ))).await
    }};

    (async, Some($emitter:expr), $($arg:tt)+) => {{
        match $emitter {
            Some(emitter) => emitter.emit($crate::Status::new($crate::status_event!($($arg)+))).await,
            None => {},
         }
    }};

    (async, $emitter:expr, $($arg:tt)+) => {{
        $emitter.emit(
            $crate::Status::new(
                $crate::status_event!($($arg)+)
            )
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
            Some(emitter) => emitter.try_emit($crate::Status::new($crate::status_event!(
                $(stage: $stage,)?
                $(current: $current,)?
                $(total: $total,)?
                $(message: $message,)?
                $(path: $path,)?
            ))),
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
       $emitter.try_emit($crate::Status::new($crate::status_event!(
            $(stage: $stage,)?
            $(current: $current,)?
            $(total: $total,)?
            $(message: $message,)?
            $(path: $path,)?
        )))
    }};

    (Some($emitter:expr), $($arg:tt)+) => {{
        match $emitter {
            Some(emitter) => emitter.try_emit($crate::Status::new($crate::status_event!($($arg)+))),
            None => {},
        }
    }};

    ($emitter:expr, $($arg:tt)+) => {{
        $emitter.try_emit(
            $crate::Status::new(
                $crate::status_event!($($arg)+)
            )
        );
    }};
}

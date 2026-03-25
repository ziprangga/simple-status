mod status_channel;
mod status_emre;
mod status_event;
mod status_format;

pub use status_channel::{ChannelReceiver, ChannelSender};
pub use status_emre::{StatusEmitter, StatusEmitterHandler, StatusReceiver, StatusReceiverHandler};
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

#[macro_export]
macro_rules! status_event {
    (
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
    ) => {{
        let mut builder = $crate::StatusEvent::builder();
        $(builder = builder.stage($stage);)?
        $(builder = builder.current($current);)?
        $(builder = builder.total($total);)?
        $(builder = builder.message($message);)?
        $(builder = builder.path($path);)?
        builder.build()
    }};

    ($($arg:tt)+) => {{
        $crate::StatusEvent::builder()
            .message(format!($($arg)+))
            .build()
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
    (async, $emitter:expr, $($arg:tt)+) => {{
        $emitter.emit(
            $crate::Status::new(
                $crate::status_event!($($arg)+)
            )
        ).await;
    }};

    // sync mode (default)
    ($emitter:expr, $($arg:tt)+) => {{
        $emitter.try_emit(
            $crate::Status::new(
                $crate::status_event!($($arg)+)
            )
        );
    }};
}

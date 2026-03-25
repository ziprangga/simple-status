mod status_channel;
mod status_event;

pub use status_channel::{ChannelReceiver, ChannelSender};
pub use status_event::StatusEmitter;
pub use status_event::StatusEvent;
pub use status_event::StatusReceiver;
use std::sync::Arc;

pub fn setup_status(buffer: usize) -> (Arc<StatusEmitter>, StatusReceiver) {
    let (tx, rx) = tokio::sync::mpsc::channel(buffer);
    let handler = Arc::new(ChannelSender::new(tx));
    let emitter = Arc::new(StatusEmitter::new(handler));

    let channel_receiver = Arc::new(ChannelReceiver::new(rx));
    let receiver = StatusReceiver::new(channel_receiver);
    (emitter, receiver)
}

// =================
// macros
// =================

#[macro_export]
macro_rules! status {
    (
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
        $(separator: $sep:expr,)?
    ) => {{
        $crate::StatusEvent::new()
            $(.with_stage($stage))?
            $(.with_current($current))?
            $(.with_total($total))?
            $(.with_message($message))?
            $(.with_path($path))?
            $(.with_separator($sep))?
    }};

    ($($arg:tt)+) => {{
        $crate::StatusEvent::new().with_message(format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! status_emit {
    ($emitter:expr,
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
        $(separator: $sep:expr,)?
    ) => {{
        $emitter.emit(
            $crate::StatusEvent::new()
                $(.with_stage($stage))?
                $(.with_current($current))?
                $(.with_total($total))?
                $(.with_message($message))?
                $(.with_path($path))?
                $(.with_separator($sep))?
        );
    }};

    ($emitter:expr, $($arg:tt)+) => {{
        $emitter.emit(
            $crate::StatusEvent::new().with_message(format!($($arg)+))
        );
    }};
}

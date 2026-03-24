mod status_channel;
mod status_event;

pub use status_channel::StatusChannel;
pub use status_event::StatusEmitter;
pub use status_event::StatusEvent;
use std::sync::{Arc, OnceLock};
use tokio::sync::mpsc;

pub static STATUS: OnceLock<Arc<StatusEmitter>> = OnceLock::new();

pub fn setup_status(buffer: usize) -> mpsc::Receiver<StatusEvent> {
    let (tx, rx) = mpsc::channel::<StatusEvent>(buffer);
    let handler = Arc::new(StatusChannel::new(tx));
    let emitter = Arc::new(StatusEmitter::new(handler));
    let _ = STATUS.set(emitter);
    rx
}

// =================
// macros
// =================

#[macro_export]
macro_rules! status {
    // full builder
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

    // simple message
    ($($arg:tt)+) => {{
        $crate::StatusEvent::new().with_message(format!($($arg)+))
    }};
}

#[macro_export]
macro_rules! status_emit {
    (
        $(stage: $stage:expr,)?
        $(current: $current:expr,)?
        $(total: $total:expr,)?
        $(message: $message:expr,)?
        $(path: $path:expr,)?
        $(separator: $sep:expr,)?
    ) => {{
        if let Some(emitter) = $crate::STATUS.get() {
            emitter.emit(
                $crate::StatusEvent::new()
                    $(.with_stage($stage))?
                    $(.with_current($current))?
                    $(.with_total($total))?
                    $(.with_message($message))?
                    $(.with_path($path))?
                    $(.with_separator($sep))?
            );
        }
    }};

    ($($arg:tt)+) => {{
        if let Some(emitter) = $crate::STATUS.get() {
            emitter.emit(
                $crate::StatusEvent::new().with_message(format!($($arg)+))
            );
        }
    }};
}

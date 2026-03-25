mod status_channel;
mod status_event;
mod status_format;

pub use status_channel::{ChannelReceiver, ChannelSender};
pub use status_event::StatusEvent;
pub use status_format::StatusFormatConfig;

use std::sync::Arc;

pub fn setup_status(buffer: usize) -> (Arc<StatusEmitter>, StatusReceiver) {
    let (tx, rx) = tokio::sync::mpsc::channel(buffer);
    let handler = Arc::new(ChannelSender::new(tx));
    let emitter = Arc::new(StatusEmitter::new(handler));

    let channel_receiver = Arc::new(ChannelReceiver::new(rx));
    let receiver = StatusReceiver::new(channel_receiver);
    (emitter, receiver)
}

pub trait StatusEmitterHandler: Send + Sync {
    fn emit_event(&self, status: Status);
}

pub trait StatusReceiverHandler: Send + Sync {
    fn recv_event(&self) -> Option<Status>;
}

#[derive(Clone)]
pub struct StatusEmitter {
    emitter: Arc<dyn StatusEmitterHandler>,
}

impl StatusEmitter {
    pub fn new(emitter: Arc<dyn StatusEmitterHandler>) -> Self {
        Self { emitter }
    }

    pub fn emit(&self, status: Status) {
        self.emitter.emit_event(status);
    }
}

#[derive(Clone)]
pub struct StatusReceiver {
    receiver: Arc<dyn StatusReceiverHandler>,
}

impl StatusReceiver {
    pub fn new(receiver: Arc<dyn StatusReceiverHandler>) -> Self {
        Self { receiver }
    }
    pub fn try_recv(&self) -> Option<Status> {
        self.receiver.recv_event()
    }
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

    pub fn format(&self, cfg: &StatusFormatConfig) -> String {
        cfg.write(&self.event)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cfg = StatusFormatConfig::default();
        write!(f, "{}", self.format(&cfg))
    }
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
    ) => {{
        let mut builder = $crate::StatusEvent::builder();
        $(builder = builder.stage($stage);)?
        $(builder = builder.current($current);)?
        $(builder = builder.total($total);)?
        $(builder = builder.message($message);)?
        $(builder = builder.path($path);)?
        $crate::Status::new(builder.build())
    }};

    ($($arg:tt)+) => {{
        $crate::Status::new(
            $crate::StatusEvent::builder()
                .message(format!($($arg)+))
                .build()
        )
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
    ) => {{
        let status = $crate::Status::new(
            $crate::StatusEvent::builder()
                $(.stage($stage))?
                $(.current($current))?
                $(.total($total))?
                $(.message($message))?
                $(.path($path))?
                .build()
        );
        $emitter.emit(status);
    }};

    ($emitter:expr, $($arg:tt)+) => {{
        $emitter.emit(
            $crate::status!( $($arg)+ )
        );
    }};
}

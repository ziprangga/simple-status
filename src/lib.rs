mod status_channel;
mod status_emitter;
mod status_event;
mod status_format;
mod status_receiver;

#[macro_use]
mod status_macro;
pub use status_macro::*;

pub use status_channel::*;
pub use status_emitter::{StatusEmitter, StatusEmitterHandler};
pub use status_event::StatusEvent;
pub use status_format::{StatusFormatConfig, StatusFormatter};
pub use status_receiver::{StatusReceiver, StatusReceiverHandler};

use std::sync::Arc;

fn create_channel(
    buffer: usize,
    kind: ChannelKind,
) -> (
    Arc<StatusEmitter>,
    Arc<StatusReceiver>,
    Option<ChannelHandler>,
) {
    match kind {
        ChannelKind::Mpsc => {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer);

            let emitter = Arc::new(StatusEmitter::new(Arc::new(ChannelSender::new(tx))));
            let receiver = Arc::new(StatusReceiver::new(Arc::new(ChannelReceiver::new(rx))));

            (emitter, receiver, None)
        }

        ChannelKind::Broadcast => {
            let (tx, _rx) = tokio::sync::broadcast::channel(buffer);

            let persistent_rx = tx.subscribe();
            let receiver = Arc::new(StatusReceiver::new(Arc::new(
                ChannelReceiverBroadcast::new(persistent_rx),
            )));

            let emitter = Arc::new(StatusEmitter::new(Arc::new(ChannelSenderBroadcast::new(
                tx.clone(),
            ))));

            (
                emitter.clone(),
                receiver,
                Some(ChannelHandler::Broadcast(emitter)),
            )
        }
    }
}

pub fn setup_handler(buffer: usize, kind: ChannelKind) -> StatusHandler {
    let (emitter, receiver, handler) = create_channel(buffer, kind);

    let status_handler = StatusHandler {
        emitter: Some(emitter),
        receiver: Some(receiver),
        handler,
    };
    status_handler
}

#[derive(Debug, Clone, Default)]
pub struct StatusHandler {
    emitter: Option<Arc<StatusEmitter>>,
    receiver: Option<Arc<StatusReceiver>>,
    handler: Option<ChannelHandler>,
}

impl StatusHandler {
    pub async fn recv_async(&self) -> StatusEvent {
        if let Some(receiver) = &self.receiver {
            receiver.async_recv().await.unwrap_or_default()
        } else {
            StatusEvent::default()
        }
    }

    pub fn recv_sync(&self) -> StatusEvent {
        self.receiver
            .as_ref()
            .and_then(|r| r.sync_recv())
            .unwrap_or_default()
    }

    pub fn new_subscriber(&self) -> Option<Arc<StatusReceiver>> {
        self.handler.as_ref()?.subscribe()
    }

    pub fn emitter(&self) -> Option<Arc<StatusEmitter>> {
        self.emitter.clone()
    }

    pub fn receiver(&self) -> Option<Arc<StatusReceiver>> {
        self.receiver.clone()
    }

    pub fn handler(&self) -> Option<ChannelHandler> {
        self.handler.clone()
    }

    pub fn set_emitter(&mut self, emitter: Option<Arc<StatusEmitter>>) {
        self.emitter = emitter;
    }

    pub fn set_receiver(&mut self, receiver: Option<Arc<StatusReceiver>>) {
        self.receiver = receiver;
    }

    pub fn set_handler(&mut self, handler: Option<ChannelHandler>) {
        self.handler = handler;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Status {
    status_event: StatusEvent,
    status_handler: StatusHandler,
}

impl Status {
    pub fn new(event: StatusEvent, handler: StatusHandler) -> Self {
        Self {
            status_event: event,
            status_handler: handler,
        }
    }

    pub fn new_handler(handler: StatusHandler) -> Self {
        Self {
            status_event: StatusEvent::default(),
            status_handler: handler,
        }
    }

    pub fn reset_status_event(&mut self) {
        self.status_event = StatusEvent::default();
    }

    pub fn status_handler(&self) -> &StatusHandler {
        &self.status_handler
    }

    pub fn status_handler_mut(&mut self) -> &mut StatusHandler {
        &mut self.status_handler
    }

    pub fn status_event(&self) -> &StatusEvent {
        &self.status_event
    }

    pub fn status_event_mut(&mut self) -> &mut StatusEvent {
        &mut self.status_event
    }

    pub fn set_status_event(&mut self, event: StatusEvent) {
        self.status_event = event;
    }

    pub fn set_status_handler(
        &mut self,
        emitter: Option<Arc<StatusEmitter>>,
        receiver: Option<Arc<StatusReceiver>>,
        handler: Option<ChannelHandler>,
    ) {
        if let Some(emitter) = emitter {
            self.status_handler.set_emitter(Some(emitter));
        }
        if let Some(receiver) = receiver {
            self.status_handler.set_receiver(Some(receiver));
        }
        if let Some(handler) = handler {
            self.status_handler.set_handler(Some(handler));
        }
    }

    pub fn format<F>(&self, f: F) -> String
    where
        F: StatusFormatter,
    {
        f.format(&self.status_event)
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cfg = StatusFormatConfig::default();
        write!(f, "{}", self.format(cfg))
    }
}

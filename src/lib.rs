mod emitter_receiver;
mod status_channel;
mod status_event;
mod status_format;

#[macro_use]
mod status_macro;
pub use status_macro::*;

pub use emitter_receiver::*;
pub use status_channel::*;
pub use status_event::StatusEvent;
pub use status_format::{StatusFormatConfig, StatusFormatter};

use std::sync::Arc;

fn create_channel(buffer: usize, kind: ChannelKind) -> (Arc<StatusEmitter>, Arc<StatusReceiver>) {
    match kind {
        ChannelKind::Mpsc => {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer);

            let emitter = Arc::new(StatusEmitter::new(Arc::new(ChannelSender::new_mpsc(tx))));
            let receiver = Arc::new(StatusReceiver::new(Arc::new(ChannelReceiver::new_mpsc(rx))));

            (emitter, receiver)
        }

        ChannelKind::Broadcast => {
            let (tx, _rx) = tokio::sync::broadcast::channel(buffer);

            let persistent_rx = tx.subscribe();
            let receiver = Arc::new(StatusReceiver::new(Arc::new(
                ChannelReceiver::new_broadcast(persistent_rx),
            )));

            let emitter = Arc::new(StatusEmitter::new(Arc::new(ChannelSender::new_broadcast(
                tx.clone(),
            ))));

            (emitter, receiver)
        }
    }
}

pub fn setup_handler(buffer: usize, kind: ChannelKind) -> ChannelHandler {
    let (emitter, receiver) = create_channel(buffer, kind);

    let channel_handler = ChannelHandler {
        emitter: Some(emitter),
        receiver: Some(receiver),
    };
    channel_handler
}

#[derive(Debug, Clone, Default)]
pub struct ChannelHandler {
    emitter: Option<Arc<StatusEmitter>>,
    receiver: Option<Arc<StatusReceiver>>,
}

impl ChannelHandler {
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
        self.emitter.as_ref()?.subscribe()
    }

    pub fn emitter(&self) -> Option<Arc<StatusEmitter>> {
        self.emitter.clone()
    }

    pub fn receiver(&self) -> Option<Arc<StatusReceiver>> {
        self.receiver.clone()
    }

    pub fn set_emitter(&mut self, emitter: Option<Arc<StatusEmitter>>) {
        self.emitter = emitter;
    }

    pub fn set_receiver(&mut self, receiver: Option<Arc<StatusReceiver>>) {
        self.receiver = receiver;
    }
}

#[derive(Debug, Default, Clone)]
pub struct Status {
    status_event: StatusEvent,
    channel_handler: ChannelHandler,
}

impl Status {
    pub fn new(event: StatusEvent, handler: ChannelHandler) -> Self {
        Self {
            status_event: event,
            channel_handler: handler,
        }
    }

    pub fn new_handler(handler: ChannelHandler) -> Self {
        Self {
            status_event: StatusEvent::default(),
            channel_handler: handler,
        }
    }

    pub fn reset_status_event(&mut self) {
        self.status_event = StatusEvent::default();
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

    pub fn channel_handler(&self) -> &ChannelHandler {
        &self.channel_handler
    }

    pub fn channel_handler_mut(&mut self) -> &mut ChannelHandler {
        &mut self.channel_handler
    }

    pub fn set_channel_handler(
        &mut self,
        emitter: Option<Arc<StatusEmitter>>,
        receiver: Option<Arc<StatusReceiver>>,
    ) {
        if let Some(emitter) = emitter {
            self.channel_handler.set_emitter(Some(emitter));
        }
        if let Some(receiver) = receiver {
            self.channel_handler.set_receiver(Some(receiver));
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

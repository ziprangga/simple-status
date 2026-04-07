use simple_status::{
    ChannelHandler, ChannelKind, Status, StatusEmitter, StatusReceiver, setup_status,
};
use std::sync::Arc;

#[derive(Debug, Clone, Copy, Default)]
pub enum StatusSource {
    EmitAsync,
    Emit,
    NonEmit,
    #[default]
    Direct,
    OptionNonEmit,
    OptionEmitAsync,
}

#[derive(Debug, Clone, Default)]
pub struct StatusReport {
    pub status_event: Status,
    pub source: StatusSource,

    pub emitter: Option<Arc<StatusEmitter>>,
    pub receiver: Option<Arc<StatusReceiver>>,
    pub handle: Option<ChannelHandler>,
}

impl StatusReport {
    pub fn new(buffer: usize, kind: ChannelKind) -> Self {
        let (emitter, receiver, handle) = setup_status(buffer, kind);

        Self {
            status_event: Status::default(),
            source: StatusSource::default(),
            emitter: Some(emitter),
            receiver: Some(receiver),
            handle,
        }
    }

    pub fn update_status(&self, status_event: Status, source: StatusSource) -> Self {
        Self {
            status_event,
            source,
            emitter: self.emitter.clone(),
            receiver: self.receiver.clone(),
            handle: self.handle.clone(),
        }
    }

    pub async fn recv_async(&self) -> Status {
        self.receiver
            .as_ref()
            .unwrap()
            .async_recv()
            .await
            .unwrap_or_default()
    }

    pub fn recv_sync(&self) -> Status {
        self.receiver
            .as_ref()
            .unwrap()
            .sync_recv()
            .unwrap_or_default()
    }

    pub fn new_subscriber(&self) -> Option<Arc<StatusReceiver>> {
        self.handle.as_ref()?.subscribe()
    }

    pub fn status_message(&self) -> String {
        self.status_event.to_string()
    }

    pub fn reset(&mut self) {
        self.status_event = Status::default();
        self.source = StatusSource::default();
    }
}

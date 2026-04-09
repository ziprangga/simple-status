use simple_status::{ChannelKind, Channels, Status, init_channels};

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

#[derive(Debug, Clone)]
pub enum AppMessage {
    ShowStatus(Status),
    ButtonEmitAsync,
    ButtonEmit,
    ButtonNonEmit,
    ButtonDirect,
    ButtonOptionNonEmit,
    ButtonOptionEmitAsync,
    NoOperations,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub show_status: Status,
    pub source: StatusSource,
    pub channel: Channels,
}

impl AppState {
    pub fn new(buffer: usize, kind: ChannelKind) -> Self {
        let channel = init_channels(buffer, kind);

        Self {
            show_status: Status::default(),
            source: StatusSource::default(),
            channel: channel,
        }
    }

    pub fn reset(&mut self) {
        self.show_status.reset_event();
        self.source = StatusSource::default();
    }
}

use simple_status::{ChannelKind, Status, setup_handler};

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
}

impl AppState {
    pub fn new(buffer: usize, kind: ChannelKind) -> Self {
        let status_handler = setup_handler(buffer, kind);

        Self {
            show_status: Status::new_handler(status_handler),
            source: StatusSource::default(),
        }
    }

    pub fn reset(&mut self) {
        self.show_status.reset_status_event();
        self.source = StatusSource::default();
    }
}

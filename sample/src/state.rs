use simple_status::{ChannelKind, Status, init_channels};

#[derive(Debug, Clone, Copy, Default)]
pub enum StatusSource {
    #[default]
    Direct,

    EmitSync,
    EmitAsync,

    GlobalEmitSync,
    GlobalEmitAsync,

    IndependentEmitSyncWithProgress,
    IndependentEmitAsyncWithProgress,

    GlobalEmitSyncWithProgress,
    GlobalEmitAsyncWithProgress,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ShowStatus(Status),

    ButtonDirect,

    ButtonEmitSync,
    ButtonEmitAsync,

    ButtonGlobalEmitSync,
    ButtonGlobalEmitAsync,

    ButtonIndependentEmitSyncWithProgress,
    ButtonIndependentEmitAsyncWithProgress,

    ButtonGlobalEmitSyncWithProgress,
    ButtonGlobalEmitAsyncWithProgress,

    NoOperations,
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub show_status: Status,
    pub source: StatusSource,
}

impl AppState {
    pub fn new(buffer: usize, kind: ChannelKind) -> Self {
        let _ = init_channels(buffer, kind);

        Self {
            show_status: Status::default(),
            source: StatusSource::default(),
        }
    }

    pub fn reset(&mut self) {
        self.show_status.reset_event();
        self.source = StatusSource::default();
    }
}

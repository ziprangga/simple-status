use simple_status::Status;

#[derive(Clone, Copy)]
pub enum StatusSource {
    EmitAsync,
    Emit,
    NonEmit,
    Direct,
}

#[derive(Debug, Clone)]
pub enum AppMessage {
    ShowStatus(Status),
    ButtonEmitAsync,
    ButtonEmit,
    ButtonNonEmit,
    ButtonDirect,
    NoOperations,
}

#[derive(Clone)]
pub struct AppState {
    pub show_status: Status,
    pub source: StatusSource,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            show_status: Status::default(),
            source: StatusSource::Direct,
        }
    }

    pub fn reset(&mut self) {
        self.show_status = Status::default();
    }
}

use simple_status::Status;

#[derive(Debug, Clone)]
pub enum AppMessage {
    ShowMessage,
    StatusEmit(Status),
    StatusNonEmit(Status),
    NoOperations,
}

#[derive(Clone)]
pub struct AppState {
    pub status_emit: Status,
    pub status_non_emit: Status,
    pub status_direct: Status,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            status_emit: Status::default(),
            status_non_emit: Status::default(),
            status_direct: Status::default(),
        }
    }

    pub fn reset(&mut self) {
        self.status_emit = Status::default();
        self.status_non_emit = Status::default();
        self.status_direct = Status::default();
    }
}

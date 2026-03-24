use status_msg::StatusEvent;

#[derive(Debug, Clone)]
pub enum AppMessage {
    ShowMessage,
    StatusEmit(StatusEvent),
    StatusNonEmit(StatusEvent),
    NoOperations,
}

#[derive(Clone)]
pub struct AppState {
    pub status_emit: StatusEvent,
    pub status_non_emit: StatusEvent,
    pub status_direct: StatusEvent,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            status_emit: StatusEvent::new(),
            status_non_emit: StatusEvent::new(),
            status_direct: StatusEvent::new(),
        }
    }

    pub fn reset(&mut self) {
        self.status_emit = StatusEvent::new();
        self.status_non_emit = StatusEvent::new();
        self.status_direct = StatusEvent::new();
    }
}

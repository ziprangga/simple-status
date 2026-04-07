use crate::status_report::StatusReport;
use simple_status::ChannelKind;

#[derive(Debug, Clone)]
pub enum AppMessage {
    ShowStatus(StatusReport),
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
    pub show_status: StatusReport,
}

impl AppState {
    pub fn new(buffer: usize, kind: ChannelKind) -> Self {
        Self {
            show_status: StatusReport::new(buffer, kind),
        }
    }

    pub fn reset(&mut self) {
        self.show_status.reset();
    }
}

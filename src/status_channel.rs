use tokio::sync::mpsc::Sender;

use crate::status_event::StatusEvent;
use crate::status_event::StatusHandler;

pub struct StatusChannel {
    sender: Sender<StatusEvent>,
}

impl StatusChannel {
    pub fn new(sender: Sender<StatusEvent>) -> Self {
        Self { sender }
    }

    fn send_event(&self, event: StatusEvent) {
        let _ = self.sender.try_send(event);
    }
}

impl StatusHandler for StatusChannel {
    fn handle_event(&self, event: StatusEvent) {
        self.send_event(event);
    }
}

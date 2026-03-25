use std::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::status_event::StatusEmitterHandler;
use crate::status_event::StatusEvent;
use crate::status_event::StatusReceiverHandler;

pub struct ChannelSender {
    channel_sender: Sender<StatusEvent>,
}

impl ChannelSender {
    pub fn new(channel_sender: Sender<StatusEvent>) -> Self {
        Self { channel_sender }
    }

    fn send_event(&self, event: StatusEvent) {
        let _ = self.channel_sender.try_send(event);
    }
}

impl StatusEmitterHandler for ChannelSender {
    fn emit_event(&self, event: StatusEvent) {
        self.send_event(event);
    }
}

pub struct ChannelReceiver {
    receiver: Mutex<mpsc::Receiver<StatusEvent>>,
}

impl ChannelReceiver {
    pub fn new(rx: mpsc::Receiver<StatusEvent>) -> Self {
        Self {
            receiver: Mutex::new(rx),
        }
    }
}

impl StatusReceiverHandler for ChannelReceiver {
    fn recv_event(&self) -> Option<StatusEvent> {
        self.receiver.lock().unwrap().try_recv().ok()
    }
}

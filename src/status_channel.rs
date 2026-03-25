use tokio::sync::Mutex;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;

use crate::Status;
use crate::StatusEmitterHandler;
use crate::StatusReceiverHandler;

pub struct ChannelSender {
    channel_sender: Sender<Status>,
}

impl ChannelSender {
    pub fn new(channel_sender: Sender<Status>) -> Self {
        Self { channel_sender }
    }

    fn send_event(&self, event: Status) {
        let _ = self.channel_sender.try_send(event);
    }
}

impl StatusEmitterHandler for ChannelSender {
    fn emit_event(&self, event: Status) {
        self.send_event(event);
    }
}

pub struct ChannelReceiver {
    receiver: Mutex<mpsc::Receiver<Status>>,
}

impl ChannelReceiver {
    pub fn new(rx: mpsc::Receiver<Status>) -> Self {
        Self {
            receiver: Mutex::new(rx),
        }
    }

    fn recv_event(&self) -> Option<Status> {
        if let Ok(mut guard) = self.receiver.try_lock() {
            guard.try_recv().ok()
        } else {
            None
        }
    }
}

impl StatusReceiverHandler for ChannelReceiver {
    fn recv_event(&self) -> Option<Status> {
        self.recv_event()
    }
}

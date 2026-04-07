// update.rs

use iced::Task;
use simple_status::*;

use crate::state::{AppMessage, AppState};
use crate::status_report::{StatusReport, StatusSource};
use crate::task::{
    message_emit_async_task, message_emit_task, message_emit_with_option_task,
    message_non_emit_task, message_non_emit_with_option_task,
};

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::ButtonEmitAsync => {
            let emitter = state.show_status.emitter.clone().unwrap();
            let handle = state.show_status.handle.clone().unwrap();

            // new subscriber (broadcast)
            let receiver = handle.subscribe();

            Task::perform(
                async move {
                    message_emit_async_task(&emitter).await;

                    StatusReport {
                        status_event: receiver.async_recv().await.unwrap_or_default(),
                        source: StatusSource::EmitAsync,
                        emitter: Some(emitter),
                        receiver: Some(receiver),
                        handle: Some(handle),
                    }
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonEmit => {
            let emitter = state.show_status.emitter.clone().unwrap();
            let handle = state.show_status.handle.clone().unwrap();

            let receiver = handle.subscribe();

            Task::perform(
                async move {
                    message_emit_task(&emitter).await;

                    StatusReport {
                        status_event: receiver.sync_recv().unwrap_or_default(),
                        source: StatusSource::Emit,
                        emitter: Some(emitter),
                        receiver: Some(receiver),
                        handle: Some(handle),
                    }
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonNonEmit => Task::perform(async { message_non_emit_task().await }, |se| {
            AppMessage::ShowStatus(StatusReport {
                status_event: se,
                source: StatusSource::NonEmit,
                emitter: None,
                receiver: None,
                handle: None,
            })
        }),

        AppMessage::ButtonDirect => {
            state.show_status = StatusReport {
                status_event: status!("this is direct message"),
                source: StatusSource::Direct,
                emitter: None,
                receiver: None,
                handle: None,
            };
            Task::none()
        }

        AppMessage::ButtonOptionNonEmit => Task::perform(
            async { message_non_emit_with_option_task().await },
            |se| match se {
                Some(status) => AppMessage::ShowStatus(StatusReport {
                    status_event: status,
                    source: StatusSource::OptionNonEmit,
                    emitter: None,
                    receiver: None,
                    handle: None,
                }),
                None => AppMessage::NoOperations,
            },
        ),

        AppMessage::ButtonOptionEmitAsync => {
            let emitter = state.show_status.emitter.clone().unwrap();
            let handle = state.show_status.handle.clone().unwrap();

            let receiver = handle.subscribe();

            Task::perform(
                async move {
                    message_emit_with_option_task(Some(&emitter)).await;

                    StatusReport {
                        status_event: receiver.async_recv().await.unwrap_or_default(),
                        source: StatusSource::OptionEmitAsync,
                        emitter: Some(emitter),
                        receiver: Some(receiver),
                        handle: Some(handle),
                    }
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ShowStatus(se) => {
            state.show_status = se;
            Task::none()
        }

        AppMessage::NoOperations => Task::none(),
    }
}

pub fn setup_status(
    buffer: usize,
    kind: ChannelKind,
) -> (
    Arc<StatusEmitter>,
    Arc<StatusReceiver>,
    Option<ChannelHandle>,
) {
    match kind {
        ChannelKind::Mpsc => {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer);

            let emitter = Arc::new(StatusEmitter::new(Arc::new(ChannelSender::new(tx))));
            let receiver = Arc::new(StatusReceiver::new(Arc::new(ChannelReceiver::new(rx))));

            (emitter, receiver, None)
        }

        ChannelKind::Broadcast => {
            let (tx, _rx) = tokio::sync::broadcast::channel(buffer);

            // Create a persistent subscriber
            let persistent_sub = tx.subscribe();

            let emitter = Arc::new(StatusEmitter::new(Arc::new(ChannelSenderBroadcast::new(
                tx.clone(),
            ))));

            let receiver = Arc::new(StatusReceiver::new(Arc::new(
                ChannelReceiverBroadcast::new(persistent_sub),
            )));

            (emitter, receiver, Some(ChannelHandle::Broadcast(tx)))
        }
    }
}

use simple_status::{
    ChannelHandle, ChannelKind, Status, StatusEmitter, StatusReceiver, setup_status,
};
use std::sync::Arc;

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

#[derive(Debug, Clone, Default)]
pub struct StatusReport {
    pub status_event: Status,
    pub source: StatusSource,

    /// Persistent emitter and receiver
    pub emitter: Option<Arc<StatusEmitter>>,
    pub receiver: Option<Arc<StatusReceiver>>,

    /// Optional broadcast handle to create additional subscribers
    pub handle: Option<ChannelHandle>,
}

impl StatusReport {
    pub fn new(buffer: usize, kind: ChannelKind) -> Self {
        let (emitter, receiver, handle) = setup_status(buffer, kind);

        Self {
            status_event: Status::default(),
            source: StatusSource::default(),
            emitter: Some(emitter),
            receiver: Some(receiver), // persistent subscriber
            handle,                   // keep handle for extra subscribers
        }
    }

    /// Update status but keep the same emitter/receiver/handle
    pub fn update_status(&self, status_event: Status, source: StatusSource) -> Self {
        Self {
            status_event,
            source,
            emitter: self.emitter.clone(),
            receiver: self.receiver.clone(),
            handle: self.handle.clone(),
        }
    }

    /// Async receive using the persistent subscriber
    pub async fn recv_async(&self) -> Status {
        self.receiver
            .as_ref()
            .unwrap()
            .async_recv()
            .await
            .unwrap_or_default()
    }

    /// Sync receive using the persistent subscriber
    pub fn recv_sync(&self) -> Status {
        self.receiver
            .as_ref()
            .unwrap()
            .sync_recv()
            .unwrap_or_default()
    }

    /// Create a **new subscriber** for broadcast if needed
    pub fn new_subscriber(&self) -> Option<Arc<StatusReceiver>> {
        self.handle.as_ref()?.subscribe()
    }

    pub fn status_message(&self) -> String {
        self.status_event.to_string()
    }

    pub fn reset(&mut self) {
        self.status_event = Status::default();
        self.source = StatusSource::default();
    }
}

/// Setup function to return emitter, receiver, and optional broadcast handle
pub fn setup_status(
    buffer: usize,
    kind: ChannelKind,
) -> (
    Arc<StatusEmitter>,
    Arc<StatusReceiver>,
    Option<ChannelHandle>,
) {
    match kind {
        ChannelKind::Mpsc => {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer);

            let emitter = Arc::new(StatusEmitter::new(Arc::new(ChannelSender::new(tx))));
            let receiver = Arc::new(StatusReceiver::new(Arc::new(ChannelReceiver::new(rx))));

            (emitter, receiver, None)
        }

        ChannelKind::Broadcast => {
            let (tx, rx) = tokio::sync::broadcast::channel(buffer);

            // Persistent subscriber
            let persistent_rx = tx.subscribe();
            let receiver = Arc::new(StatusReceiver::new(Arc::new(
                ChannelReceiverBroadcast::new(persistent_rx),
            )));

            let emitter = Arc::new(StatusEmitter::new(Arc::new(ChannelSenderBroadcast::new(
                tx.clone(),
            ))));

            (emitter, receiver, Some(ChannelHandle::Broadcast(tx)))
        }
    }
}

use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use tokio::sync::{Mutex, broadcast, mpsc};

use crate::{Status, StatusEmitterHandler, StatusReceiver, StatusReceiverHandler};

// ---------------------------
// Channel Kinds
// ---------------------------

#[derive(Debug, Clone)]
pub enum ChannelKind {
    Mpsc,
    Broadcast,
}

impl std::str::FromStr for ChannelKind {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "mpsc" => Ok(Self::Mpsc),
            "broadcast" => Ok(Self::Broadcast),
            _ => Err(()),
        }
    }
}

// ---------------------------
// MPSC Sender / Receiver
// ---------------------------

#[derive(Debug, Clone)]
pub struct ChannelSender {
    channel_sender: mpsc::Sender<Status>,
}

impl ChannelSender {
    pub fn new(channel_sender: mpsc::Sender<Status>) -> Self {
        Self { channel_sender }
    }
}

impl StatusEmitterHandler for ChannelSender {
    fn try_emit(&self, event: Status) {
        let _ = self.channel_sender.try_send(event);
    }

    fn emit(&self, status: Status) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let _ = self.channel_sender.send(status).await;
        })
    }

    fn subscribe(&self) -> Option<Arc<StatusReceiver>> {
        None // MPSC does not support multiple subscribers
    }
}

#[derive(Debug)]
pub struct ChannelReceiver {
    receiver: Mutex<mpsc::Receiver<Status>>,
}

impl ChannelReceiver {
    pub fn new(rx: mpsc::Receiver<Status>) -> Self {
        Self {
            receiver: Mutex::new(rx),
        }
    }
}

impl StatusReceiverHandler for ChannelReceiver {
    fn try_recv(&self) -> Option<Status> {
        if let Ok(mut guard) = self.receiver.try_lock() {
            guard.try_recv().ok()
        } else {
            None
        }
    }

    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<Status>> + Send + '_>> {
        Box::pin(async move {
            let mut guard = self.receiver.lock().await;
            guard.recv().await
        })
    }
}

// ---------------------------
// Broadcast Sender / Receiver
// ---------------------------

#[derive(Debug, Clone)]
pub struct ChannelSenderBroadcast {
    channel_sender: broadcast::Sender<Status>,
}

impl ChannelSenderBroadcast {
    pub fn new(channel_sender: broadcast::Sender<Status>) -> Self {
        Self { channel_sender }
    }
}

impl StatusEmitterHandler for ChannelSenderBroadcast {
    fn try_emit(&self, status: Status) {
        let _ = self.channel_sender.send(status);
    }

    fn emit(&self, status: Status) -> Pin<Box<dyn Future<Output = ()> + Send + '_>> {
        Box::pin(async move {
            let _ = self.channel_sender.send(status);
        })
    }

    fn subscribe(&self) -> Option<Arc<StatusReceiver>> {
        let rx = self.channel_sender.subscribe();
        Some(Arc::new(StatusReceiver::new(Arc::new(
            ChannelReceiverBroadcast::new(rx),
        ))))
    }
}

#[derive(Debug)]
pub struct ChannelReceiverBroadcast {
    receiver: Mutex<broadcast::Receiver<Status>>,
}

impl ChannelReceiverBroadcast {
    pub fn new(rx: broadcast::Receiver<Status>) -> Self {
        Self {
            receiver: Mutex::new(rx),
        }
    }
}

impl StatusReceiverHandler for ChannelReceiverBroadcast {
    fn try_recv(&self) -> Option<Status> {
        if let Ok(mut guard) = self.receiver.try_lock() {
            guard.try_recv().ok()
        } else {
            None
        }
    }

    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<Status>> + Send + '_>> {
        Box::pin(async move {
            let mut guard = self.receiver.lock().await;
            loop {
                match guard.recv().await {
                    Ok(status) => return Some(status),
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(_) => return None,
                }
            }
        })
    }
}

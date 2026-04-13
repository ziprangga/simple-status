use std::future::Future;
use std::pin::Pin;
use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use super::ChannelKind;
use super::ReceiverHandler;
use crate::status::Status;

use async_stream::stream;
use futures::Stream;

#[derive(Debug)]
pub struct ChannelReceiver {
    kind: ChannelKind,
    mpsc_receiver: Option<Mutex<mpsc::Receiver<Status>>>,
    broadcast_receiver: Option<Mutex<broadcast::Receiver<Status>>>,
}

impl ChannelReceiver {
    pub fn new_mpsc(rx: mpsc::Receiver<Status>) -> Self {
        Self {
            kind: ChannelKind::Mpsc,
            mpsc_receiver: Some(Mutex::new(rx)),
            broadcast_receiver: None,
        }
    }

    pub fn new_broadcast(rx: broadcast::Receiver<Status>) -> Self {
        Self {
            kind: ChannelKind::Broadcast,
            mpsc_receiver: None,
            broadcast_receiver: Some(Mutex::new(rx)),
        }
    }

    fn try_recv_status(&self) -> Option<Status> {
        match self.kind {
            ChannelKind::Mpsc => {
                let mut guard = self.mpsc_receiver.as_ref()?.try_lock().ok()?;
                guard.try_recv().ok()
            }
            ChannelKind::Broadcast => {
                let mut guard = self.broadcast_receiver.as_ref()?.try_lock().ok()?;
                guard.try_recv().ok()
            }
        }
    }
}

impl ReceiverHandler for ChannelReceiver {
    fn try_recv(&self) -> Option<Status> {
        self.try_recv_status()
    }

    fn recv(&self) -> Pin<Box<dyn Future<Output = Option<Status>> + Send + '_>> {
        Box::pin(async move {
            match self.kind {
                ChannelKind::Mpsc => {
                    let rx = self.mpsc_receiver.as_ref()?;
                    let mut guard = rx.lock().await;
                    guard.recv().await
                }
                ChannelKind::Broadcast => {
                    let rx = self.broadcast_receiver.as_ref()?;
                    let mut guard = rx.lock().await;
                    loop {
                        match guard.recv().await {
                            Ok(v) => return Some(v),
                            Err(broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(_) => return None,
                        }
                    }
                }
            }
        })
    }

    fn stream(&self) -> Pin<Box<dyn Stream<Item = Status> + Send + '_>> {
        match self.kind {
            ChannelKind::Mpsc => {
                let rx = self.mpsc_receiver.as_ref().unwrap();

                Box::pin(stream! {
                    let mut guard = rx.lock().await;

                    while let Some(v) = guard.recv().await {
                        yield v;
                    }
                })
            }

            ChannelKind::Broadcast => {
                let rx = self.broadcast_receiver.as_ref().unwrap();

                Box::pin(stream! {
                    let mut guard = rx.lock().await;

                    loop {
                        match guard.recv().await {
                            Ok(v) => yield v,
                            Err(broadcast::error::RecvError::Lagged(_)) => continue,
                            Err(_) => break,
                        }
                    }
                })
            }
        }
    }
}

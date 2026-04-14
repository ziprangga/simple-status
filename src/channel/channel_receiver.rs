use tokio::sync::Mutex;
use tokio::sync::broadcast;
use tokio::sync::mpsc;

use super::ReceiverHandler;
use crate::status::Status;
use futures::stream;

use super::BoxFuture;
use super::BoxStream;

#[derive(Debug)]
pub struct MpscReceiver {
    inner: Mutex<mpsc::Receiver<Status>>,
}

impl MpscReceiver {
    pub fn new(rx: mpsc::Receiver<Status>) -> Self {
        Self {
            inner: Mutex::new(rx),
        }
    }
}

impl ReceiverHandler for MpscReceiver {
    fn try_recv(&self) -> Option<Status> {
        self.inner.try_lock().ok()?.try_recv().ok()
    }

    fn recv(&self) -> BoxFuture<'_, Option<Status>> {
        Box::pin(async move { self.inner.lock().await.recv().await })
    }

    fn stream(&self) -> BoxStream<'_, Status> {
        Box::pin(stream::unfold(self, |this| async move {
            this.recv().await.map(|status| (status, this))
        }))
    }
}

#[derive(Debug)]
pub struct BroadcastReceiver {
    inner: Mutex<broadcast::Receiver<Status>>,
}

impl BroadcastReceiver {
    pub fn new(rx: broadcast::Receiver<Status>) -> Self {
        Self {
            inner: Mutex::new(rx),
        }
    }
}

impl ReceiverHandler for BroadcastReceiver {
    fn try_recv(&self) -> Option<Status> {
        let mut guard = self.inner.try_lock().ok()?;
        loop {
            match guard.try_recv() {
                Ok(status) => return Some(status),
                Err(broadcast::error::TryRecvError::Lagged(_)) => continue,
                _ => return None,
            }
        }
    }

    fn recv(&self) -> BoxFuture<'_, Option<Status>> {
        Box::pin(async move {
            let mut guard = self.inner.lock().await;
            loop {
                match guard.recv().await {
                    Ok(status) => return Some(status),
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    _ => return None,
                }
            }
        })
    }

    fn stream(&self) -> BoxStream<'_, Status> {
        Box::pin(stream::unfold(self, |this| async move {
            this.recv().await.map(|status| (status, this))
        }))
    }
}

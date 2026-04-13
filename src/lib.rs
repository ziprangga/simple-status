mod channel;
// mod format;
mod status;

#[macro_use]
mod macros;
pub use macros::*;

pub use channel::*;
// pub use format::*;
pub use status::*;

use std::sync::Arc;

fn create_channels(buffer: usize, kind: ChannelKind) -> (Arc<Emitter>, Arc<Receiver>) {
    match kind {
        ChannelKind::Mpsc => {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer);

            let emitter = Arc::new(Emitter::new(Arc::new(ChannelEmitter::new_mpsc(tx))));
            let receiver = Arc::new(Receiver::new(Arc::new(ChannelReceiver::new_mpsc(rx))));

            (emitter, receiver)
        }

        ChannelKind::Broadcast => {
            let (tx, _rx) = tokio::sync::broadcast::channel(buffer);

            let persistent_rx = tx.subscribe();
            let receiver = Arc::new(Receiver::new(Arc::new(ChannelReceiver::new_broadcast(
                persistent_rx,
            ))));

            let emitter = Arc::new(Emitter::new(Arc::new(ChannelEmitter::new_broadcast(
                tx.clone(),
            ))));

            (emitter, receiver)
        }
    }
}

pub fn init_channels(buffer: usize, kind: ChannelKind) -> Channels {
    let (emitter, receiver) = create_channels(buffer, kind);
    let channel_handler = Channels::new(Some(emitter), Some(receiver));
    channel_handler
}

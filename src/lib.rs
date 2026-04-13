mod channel;
mod status;

#[macro_use]
mod macros;
pub use macros::*;

pub use channel::*;
pub use status::*;

fn create_channels(buffer: usize, kind: ChannelKind) -> (Emitter, Receiver) {
    match kind {
        ChannelKind::Mpsc => {
            let (tx, rx) = tokio::sync::mpsc::channel(buffer);

            let emitter: Emitter = MpscEmitter::new(tx).into();
            let receiver: Receiver = MpscReceiver::new(rx).into();

            (emitter, receiver)
        }

        ChannelKind::Broadcast => {
            let (tx, _rx) = tokio::sync::broadcast::channel(buffer);

            let persistent_rx = tx.subscribe();

            let emitter: Emitter = BroadcastEmitter::new(tx).into();
            let receiver: Receiver = BroadcastReceiver::new(persistent_rx).into();
            (emitter, receiver)
        }
    }
}

pub fn init_channels(buffer: usize, kind: ChannelKind) -> Channels {
    let (emitter, receiver) = create_channels(buffer, kind);
    let channel_handler = Channels::new(Some(emitter), Some(receiver));
    channel_handler
}

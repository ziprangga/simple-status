use simple_status::{ChannelKind, init_channels, status, status_emit};

#[test]
fn test_status_macro() {
    let s = status!(
        stage: "stage",
        current: 1,
        total: 10,
        message: "hello",
    );

    assert_eq!(s.event().stage().as_deref(), Some("stage"));
    assert_eq!(s.event().current(), Some(1));
    assert_eq!(s.event().total(), Some(10));
    assert_eq!(s.event().message().as_deref(), Some("hello"));
}

#[test]
fn test_status_format_message() {
    let s = status!("value {}", 42);
    assert_eq!(s.event().message().as_deref(), Some("value 42"));
}

#[test]
fn test_mpsc_sync_emit_recv() {
    let channels = init_channels(10, ChannelKind::Mpsc);
    let emitter = channels.emitter().unwrap();

    status_emit!(
        Some(&*emitter),
        message: "sync mpsc",
    );

    let received = channels.recv_sync().unwrap();
    assert_eq!(received.event().message().as_deref(), Some("sync mpsc"));
}

#[test]
fn test_broadcast_sync_emit_recv() {
    let channels = init_channels(10, ChannelKind::Broadcast);
    let emitter = channels.emitter().unwrap();

    status_emit!(
        Some(&*emitter),
        message: "sync broadcast",
    );

    let received = channels.recv_sync().unwrap();
    assert_eq!(
        received.event().message().as_deref(),
        Some("sync broadcast")
    );
}

#[tokio::test]
async fn test_async_emit_recv() {
    let channels = init_channels(10, ChannelKind::Mpsc);
    let emitter = channels.emitter().unwrap();

    status_emit!(
        async,
        Some(&*emitter),
        message: "async test",
    );

    let received = channels.recv_async().await.unwrap();
    assert_eq!(received.event().message().as_deref(), Some("async test"));
}

#[tokio::test]
async fn test_emit_with_none_emitter() {
    let s = status!("no emitter");

    simple_status::emit_status_sync(None, s.clone());
    simple_status::emit_status_async(None, s).await;
}

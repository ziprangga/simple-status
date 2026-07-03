use simple_status::{ChannelKind, Emitter, init_channels, status, status_emit};

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
    let emitter = channels.get_emitter().unwrap();

    status_emit!(ins,
        Some(&*emitter),
        message: "sync mpsc",
    );

    let received = channels.recv_sync().unwrap();
    assert_eq!(received.event().message().as_deref(), Some("sync mpsc"));
}

#[test]
fn test_broadcast_sync_emit_recv() {
    let channels = init_channels(10, ChannelKind::Broadcast);
    let emitter = channels.get_emitter().unwrap();

    status_emit!(ins,
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
    let emitter = channels.get_emitter().unwrap();

    status_emit!(ins,
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

#[test]
fn test_macro_with_raw_reference_argument() {
    let channels = init_channels(10, ChannelKind::Mpsc);
    let emitter = channels.get_emitter().unwrap();

    // Create a direct reference &Emitter type to mimic a function argument
    let raw_ref: &Emitter = &*emitter;

    // The macro should automatically wrap this reference into Some() using the trait
    status_emit!(ins,
        raw_ref,
        message: "raw reference test",
    );

    let received = channels.recv_sync().unwrap();
    assert_eq!(
        received.event().message().as_deref(),
        Some("raw reference test")
    );
}

#[test]
fn test_macro_with_option_variable_argument() {
    let channels = init_channels(10, ChannelKind::Mpsc);
    let emitter = channels.get_emitter().unwrap();

    // Create an Option<&Emitter> variable to mimic a function argument
    let option_var: Option<&Emitter> = Some(&*emitter);

    // The macro should forward this Option straight through without double-wrapping
    status_emit!(ins,
        option_var,
        message: "option variable test",
    );

    let received = channels.recv_sync().unwrap();
    assert_eq!(
        received.event().message().as_deref(),
        Some("option variable test")
    );
}

#[test]
fn test_macro_with_raw_reference_format_fallback() {
    let channels = init_channels(10, ChannelKind::Mpsc);
    let emitter = channels.get_emitter().unwrap();
    let raw_ref: &Emitter = &*emitter;

    // Tests that the printf fallback rule also correctly uses the IntoEmitter trait
    status_emit!(ins, raw_ref, "fallback formatting with raw ref: {}", 100);

    let received = channels.recv_sync().unwrap();
    assert_eq!(
        received.event().message().as_deref(),
        Some("fallback formatting with raw ref: 100")
    );
}

#[tokio::test]
async fn test_async_macro_with_option_and_raw_ref() {
    let channels = init_channels(10, ChannelKind::Mpsc);
    let emitter = channels.get_emitter().unwrap();

    let raw_ref: &Emitter = &*emitter;
    let option_var: Option<&Emitter> = Some(&*emitter);

    // Test async path with raw reference
    status_emit!(ins,
        async,
        raw_ref,
        message: "async raw ref",
    );
    let received1 = channels.recv_async().await.unwrap();
    assert_eq!(
        received1.event().message().as_deref(),
        Some("async raw ref")
    );

    // Test async path with option variable
    status_emit!(ins,
        async,
        option_var,
        message: "async option var",
    );
    let received2 = channels.recv_async().await.unwrap();
    assert_eq!(
        received2.event().message().as_deref(),
        Some("async option var")
    );
}

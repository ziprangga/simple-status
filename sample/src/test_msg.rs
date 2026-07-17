use simple_status::{ChannelsBus, Emitter};
use simple_status::{StatusEvent, status, status_emit};
use tokio::time::Duration;

pub static TEST_BUS: ChannelsBus = ChannelsBus::new();

pub fn direct_message() -> StatusEvent {
    status!("this is DIRECT")
}

pub fn emit_sync_message(emitter: &Emitter) {
    status_emit!(emitter, "{}", "this is EMIT SYNC INDEPENDENT".to_string());
}

pub async fn emit_async_message(emitter: &Emitter) {
    status_emit!(
        async,
        emitter,
        "{}",
        "this is EMIT ASYNC INDEPENDENT".to_string()
    );
}

pub fn global_emit_sync_message() {
    status_emit!(bus TEST_BUS, "this is EMIT SYNC GLOBAL");
}

pub async fn global_emit_async_message() {
    status_emit!(async, bus TEST_BUS, "this is EMIT ASYNC GLOBAL");
}

/// Note:
/// This example emits all status updates synchronously in a tight loop.
///
/// Because the loop does not yield control between emissions, receivers may
/// process multiple queued updates together. Applications that display only the
/// most recent status may therefore show only the final progress value.
///
/// If each progress update should be observed individually, use the async
/// example or introduce a delay between emissions.
pub fn independent_emit_sync_with_progress(emitter: &Emitter) {
    let total = 20;

    for current in 0..=total {
        status_emit!(
            emitter,
            action: "INDEPENDENT EMIT SYNC",
            current: current,
            total: total,
            message: "build style",
        );
    }
}

pub async fn independent_emit_async_with_progress(emitter: &Emitter) {
    let total = 20;

    for current in 0..=total {
        status_emit!(
            async,
            emitter,
            action: "INDEPENDENT EMIT ASYNC",
            current: current,
            total: total,
            message: "build style",
        );

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

/// Note:
/// This example emits all status updates synchronously in a tight loop.
///
/// Because the loop does not yield control between emissions, receivers may
/// process multiple queued updates together. Applications that display only the
/// most recent status may therefore show only the final progress value.
///
/// If each progress update should be observed individually, use the async
/// example or introduce a delay between emissions.
pub fn global_emit_sync_with_progress() {
    let total = 20;

    for current in 0..=total {
        status_emit!(
            bus TEST_BUS,
            action: "GLOBAL EMIT SYNC",
            current: current,
            total: total,
            message: "build style",
        );
    }
}

pub async fn global_emit_async_with_progress() {
    let total = 20;

    for current in 0..=total {
        status_emit!(
            async,
            bus TEST_BUS,
            action: "GLOBAL EMIT ASYNC",
            current: current,
            total: total,
            message: "build style",
        );

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

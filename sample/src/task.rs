use simple_status::{Emitter, StatusEvent};

use crate::test_msg::*;

pub async fn direct_message_task() -> StatusEvent {
    direct_message()
}

pub async fn emit_sync_message_task(emitter: Option<&Emitter>) {
    emit_sync_message(emitter);
}

pub async fn emit_async_message_task(emitter: Option<&Emitter>) {
    emit_async_message(emitter).await;
}

pub async fn global_emit_sync_message_task() {
    global_emit_sync_message();
}

pub async fn global_emit_async_message_task() {
    global_emit_async_message().await;
}

pub async fn independent_emit_sync_with_progress_task(emitter: Option<&Emitter>) {
    independent_emit_sync_with_progress(emitter);
}

pub async fn independent_emit_async_with_progress_task(emitter: Option<&Emitter>) {
    independent_emit_async_with_progress(emitter).await;
}

pub async fn global_emit_sync_with_progress_task() {
    global_emit_sync_with_progress();
}

pub async fn global_emit_async_with_progress_task() {
    global_emit_async_with_progress().await;
}

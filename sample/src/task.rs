use simple_status::{StatusEmitter, StatusEvent};

use crate::test_msg::*;

pub async fn direct_message_task() -> StatusEvent {
    direct_message()
}

pub async fn emit_sync_message_task(emitter: &StatusEmitter) {
    emit_sync_message(emitter);
}

pub async fn emit_async_message_task(emitter: &StatusEmitter) {
    emit_async_message(emitter).await;
}

pub async fn global_emit_sync_message_task() {
    global_emit_sync_message();
}

pub async fn global_emit_async_message_task() {
    global_emit_async_message().await;
}

pub async fn independent_emit_sync_with_id_task(emitter: &StatusEmitter, id: Option<String>) {
    independent_emit_sync_with_id(emitter, id);
}

pub async fn independent_emit_async_with_progress_task(emitter: &StatusEmitter) {
    independent_emit_async_with_progress(emitter).await;
}

pub async fn global_emit_sync_with_id_task(id: Option<String>) {
    global_emit_sync_with_id(id);
}

pub async fn global_emit_async_with_progress_task() {
    global_emit_async_with_progress().await;
}

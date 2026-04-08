use simple_status::{StatusEmitter, StatusEvent};

use crate::test_msg::*;
pub async fn message_non_emit_task() -> StatusEvent {
    message_non_emit()
}

pub async fn message_emit_task(status: &StatusEmitter) {
    message_emit(status)
}

pub async fn message_emit_async_task(status: &StatusEmitter) {
    message_emit_async(status).await;
}

pub async fn message_non_emit_with_option_task() -> Option<StatusEvent> {
    message_non_emit_with_option()
}

pub async fn message_emit_with_option_task(status: Option<&StatusEmitter>) {
    message_emit_with_option(status).await;
}

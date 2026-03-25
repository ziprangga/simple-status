use simple_status::{Status, StatusEmitter};

use crate::test_msg::*;
pub async fn message_non_emit_task() -> Status {
    message_non_emit()
}

pub async fn message_emit_task(status: &StatusEmitter) {
    message_emit(status)
}

pub async fn message_emit_async_task(status: &StatusEmitter) {
    message_emit_async(status).await;
}

use simple_status::{StatusEmitter, StatusEvent};

use crate::test_msg::*;
pub async fn message_non_emit_async() -> StatusEvent {
    message_non_emit()
}

pub async fn message_emit_async(status: &StatusEmitter) {
    message_emit(status)
}

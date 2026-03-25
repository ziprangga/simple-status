use simple_status::{Status, StatusEmitter};

use crate::test_msg::*;
pub async fn message_non_emit_async() -> Status {
    message_non_emit()
}

pub async fn message_emit_async(status: &StatusEmitter) {
    message_emit(status)
}

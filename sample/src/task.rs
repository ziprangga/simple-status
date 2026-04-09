use simple_status::{Emitter, Status};

use crate::test_msg::*;

pub async fn message_non_emit_task() -> Status {
    message_non_emit()
}

pub async fn message_emit_task(emitter: &Emitter) {
    message_emit(emitter)
}

pub async fn message_emit_async_task(emitter: &Emitter) {
    message_emit_async(emitter).await;
}

pub async fn message_non_emit_with_option_task() -> Option<Status> {
    message_non_emit_with_option()
}

pub async fn message_emit_with_option_task(emitter: Option<&Emitter>) {
    message_emit_with_option(emitter).await;
}

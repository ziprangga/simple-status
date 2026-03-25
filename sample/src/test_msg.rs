use simple_status::{Status, StatusEmitter, status, status_emit};

pub fn message_non_emit() -> Status {
    status!("this is non emit/return status")
}

pub fn message_emit(status: &StatusEmitter) {
    status_emit!(status, "this is emit");
}

pub async fn message_emit_async(status: &StatusEmitter) {
    status_emit!(async, status, "this is async emit");
}

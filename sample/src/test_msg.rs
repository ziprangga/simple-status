use simple_status::{StatusEmitter, StatusEvent, status, status_emit};

pub fn message_non_emit() -> StatusEvent {
    status!("this is non emit/return status")
}

pub fn message_emit(status: &StatusEmitter) {
    status_emit!(status, "this is emit");
}

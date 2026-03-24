use status_msg::{StatusEvent, status, status_emit};

pub fn message_non_emit() -> StatusEvent {
    status!("this is non emit")
}

pub fn message_emit() {
    status_emit!("this is emit");
}

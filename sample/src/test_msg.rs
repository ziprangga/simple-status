use simple_status::{Emitter, Status, event_emit, status};

pub fn message_non_emit() -> Status {
    status!("this is non emit/return status")
}

pub fn message_emit(emitter: &Emitter) {
    event_emit!(emitter, "this is emit");
}

pub async fn message_emit_async(emitter: &Emitter) {
    event_emit!(async, emitter, "this is async emit");
}

pub fn message_non_emit_with_option() -> Option<Status> {
    Some(status!("this is option non emit/return status"))
}

pub async fn message_emit_with_option(emitter: Option<&Emitter>) {
    event_emit!(
        async,
        Some(emitter),
        message: "this is option async emit",
    );
}

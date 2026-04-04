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

pub fn message_non_emit_with_option() -> Option<Status> {
    Some(status!("this is option non emit/return status"))
}

pub async fn message_emit_with_option(status: Option<&StatusEmitter>) {
    // status_emit!(async, Some(status), "this is option async emit");
    status_emit!(
        async,
        Some(status),
        message: "this is option async emit",
    );
}

use simple_status::{Emitter, Status, status, status_emit};

pub fn message_non_emit() -> Status {
    status!("this is non emit/return status")
}

pub fn message_emit(emitter: &Emitter) {
    status_emit!(Some(emitter), "this is emit");
}

pub async fn message_emit_async(emitter: &Emitter) {
    status_emit!(async, Some(emitter), "this is async emit");
}

pub fn message_non_emit_with_option() -> Option<Status> {
    Some(status!("this is option non emit/return status"))
}

pub async fn message_emit_with_option(emitter: Option<&Emitter>) {
    // status_emit!(
    //     async,
    //     emitter,
    //     message: "this is option async emit",
    // );

    status_emit!(
        async,
        emitter,
        stage: "test",
        current: 12,
        total: 20,
        message: "build style",
    );
}

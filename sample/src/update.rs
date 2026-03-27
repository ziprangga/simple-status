use iced::Task;
use simple_status::*;

use crate::state::{AppMessage, AppState, StatusSource};
use crate::task::{message_emit_async_task, message_emit_task, message_non_emit_task};

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::ButtonEmitAsync => {
            state.source = StatusSource::EmitAsync;
            // Setup channel for emitted statuses
            let (emitter, receiver) = setup_status(10);
            Task::perform(
                async move {
                    message_emit_async_task(&emitter).await;
                    receiver.try_recv()
                },
                |maybe_emit| match maybe_emit {
                    Some(se) => AppMessage::ShowStatus(se),
                    None => AppMessage::NoOperations,
                },
            )
        }

        AppMessage::ButtonEmit => {
            state.source = StatusSource::Emit;
            let (emitter, receiver) = setup_status(10);

            Task::perform(
                async move {
                    message_emit_task(&emitter).await;
                    receiver.try_recv()
                },
                |maybe_emit| match maybe_emit {
                    Some(se) => AppMessage::ShowStatus(se),
                    None => AppMessage::NoOperations,
                },
            )
        }

        AppMessage::ButtonNonEmit => {
            state.source = StatusSource::NonEmit;
            Task::perform(async { message_non_emit_task().await }, |se| {
                AppMessage::ShowStatus(se)
            })
        }

        AppMessage::ButtonDirect => {
            state.source = StatusSource::Direct;
            let status_direct = status!("this is direct message");
            state.show_status = status_direct;
            Task::none()
        }

        AppMessage::ShowStatus(se) => {
            state.show_status = se;
            Task::none()
        }

        AppMessage::NoOperations => Task::none(),
    }
}

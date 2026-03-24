use iced::Task;
use simple_status::*;

use crate::state::{AppMessage, AppState};
use crate::task::{message_emit_async, message_non_emit_async};

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::ShowMessage => {
            // Set status_direct immediately
            let status_direct = status!("this is direct message");
            state.status_direct = status_direct;

            // Setup channel for emitted statuses
            let mut rx = setup_status(10);

            // Task for non-emit status
            let non_emit_task = Task::perform(async { message_non_emit_async().await }, |se| {
                AppMessage::StatusNonEmit(se)
            });

            // Task for emitted status
            let emit_task = Task::perform(
                async move {
                    message_emit_async().await;
                    rx.recv().await
                },
                |maybe_emit| match maybe_emit {
                    Some(se) => AppMessage::StatusEmit(se),
                    None => AppMessage::NoOperations,
                },
            );

            Task::batch(vec![non_emit_task, emit_task])
        }

        AppMessage::StatusEmit(se) => {
            state.status_emit = se;
            Task::none()
        }

        AppMessage::StatusNonEmit(se) => {
            state.status_non_emit = se;
            Task::none()
        }

        AppMessage::NoOperations => Task::none(),
    }
}

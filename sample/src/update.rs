use iced::Task;
use simple_status::*;

use crate::state::{AppMessage, AppState};
use crate::task::{message_emit_async_task, message_emit_task, message_non_emit_task};

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::ShowMessage => {
            // Set status_direct immediately
            let status_direct = status!("this is direct message");
            state.status_direct = status_direct;

            // Setup channel for emitted statuses
            let (emitter, receiver) = setup_status(10);

            // Task for non-emit status
            let non_emit_task = Task::perform(async { message_non_emit_task().await }, |se| {
                AppMessage::StatusNonEmit(se)
            });

            // Task for emitted status
            let emit_task = {
                let emitter = emitter.clone();
                let receiver = receiver.clone();
                Task::perform(
                    async move {
                        message_emit_task(&emitter).await;
                        receiver.try_recv()
                    },
                    |maybe_emit| match maybe_emit {
                        Some(se) => AppMessage::StatusEmit(se),
                        None => AppMessage::NoOperations,
                    },
                )
            };

            let emit_task_async = {
                let emitter = emitter.clone();
                let receiver = receiver.clone();
                Task::perform(
                    async move {
                        message_emit_async_task(&emitter).await;
                        receiver.try_recv()
                    },
                    |maybe_emit| match maybe_emit {
                        Some(se) => AppMessage::StatusEmitAsync(se),
                        None => AppMessage::NoOperations,
                    },
                )
            };

            Task::batch(vec![non_emit_task, emit_task, emit_task_async])
        }

        AppMessage::StatusEmitAsync(se) => {
            state.status_emit_async = se;
            Task::none()
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

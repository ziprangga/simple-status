use iced::Task;
use simple_status::*;

use crate::state::{AppMessage, AppState, StatusSource};
use crate::task::{
    message_emit_async_task, message_emit_task, message_emit_with_option_task,
    message_non_emit_task, message_non_emit_with_option_task,
};

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::ButtonEmitAsync => {
            state.source = StatusSource::EmitAsync;
            let channel = state.channel.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &channel.emitter() {
                        message_emit_async_task(emitter).await;
                        if let Some(status) = channel.recv_async().await {
                            return AppMessage::ShowStatus(status);
                        }
                    }
                    AppMessage::NoOperations
                },
                |msg| msg,
            )
        }

        AppMessage::ButtonEmit => {
            state.source = StatusSource::Emit;
            let channel = state.channel.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &channel.emitter() {
                        message_emit_task(emitter).await;
                        if let Some(status) = channel.recv_async().await {
                            return AppMessage::ShowStatus(status);
                        }
                    }
                    AppMessage::NoOperations
                },
                |msg| msg,
            )
        }

        AppMessage::ButtonNonEmit => {
            state.source = StatusSource::NonEmit;
            Task::perform(async { message_non_emit_task().await }, move |status| {
                AppMessage::ShowStatus(status)
            })
        }

        AppMessage::ButtonDirect => {
            state.source = StatusSource::Direct;
            state.show_status = status!("this is direct message");
            Task::none()
        }

        AppMessage::ButtonOptionNonEmit => {
            state.source = StatusSource::OptionNonEmit;
            Task::perform(
                async { message_non_emit_with_option_task().await },
                move |maybe_status| match maybe_status {
                    Some(status) => AppMessage::ShowStatus(status),
                    None => AppMessage::NoOperations,
                },
            )
        }

        AppMessage::ButtonOptionEmitAsync => {
            state.source = StatusSource::OptionEmitAsync;
            let channel = state.channel.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &channel.emitter() {
                        message_emit_with_option_task(Some(&emitter)).await;
                        if let Some(status) = channel.recv_async().await {
                            return AppMessage::ShowStatus(status);
                        }
                    }
                    AppMessage::NoOperations
                },
                |msg| msg,
            )
        }

        AppMessage::ShowStatus(se) => {
            state.show_status = se;
            Task::none()
        }

        AppMessage::NoOperations => Task::none(),
    }
}

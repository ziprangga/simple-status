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
            let mut status = state.show_status.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &channel.emitter() {
                        message_emit_async_task(emitter).await;
                        if let Some(event) = channel.recv_async().await {
                            status.set_event(event);
                        }
                    }
                    status
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonEmit => {
            state.source = StatusSource::Emit;

            let channel = state.channel.clone();
            let mut status = state.show_status.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &channel.emitter() {
                        message_emit_task(emitter).await;
                        if let Some(event) = channel.recv_async().await {
                            status.set_event(event);
                        }
                    }
                    status
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonNonEmit => {
            state.source = StatusSource::NonEmit;
            Task::perform(async { message_non_emit_task().await }, move |event| {
                AppMessage::ShowStatus(event)
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
                move |maybe_event| match maybe_event {
                    Some(event) => AppMessage::ShowStatus(event),
                    None => AppMessage::NoOperations,
                },
            )
        }

        AppMessage::ButtonOptionEmitAsync => {
            state.source = StatusSource::OptionEmitAsync;

            let channel = state.channel.clone();
            let mut status = state.show_status.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &channel.emitter() {
                        message_emit_with_option_task(Some(&emitter)).await;
                        if let Some(event) = channel.recv_async().await {
                            status.set_event(event);
                        }
                    }
                    status
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ShowStatus(se) => {
            state.show_status = se;
            Task::none()
        }

        AppMessage::NoOperations => Task::none(),
    }
}

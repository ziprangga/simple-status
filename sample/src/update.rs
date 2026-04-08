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
            let mut report_status = state.show_status.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &report_status.status_handler().emitter() {
                        message_emit_async_task(emitter).await;
                        let status = report_status.status_handler().recv_async().await;
                        report_status.set_status_event(status)
                    } else {
                        report_status.set_status_event(StatusEvent::default());
                    }
                    report_status
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonEmit => {
            state.source = StatusSource::Emit;
            let mut report_status = state.show_status.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &report_status.status_handler().emitter() {
                        message_emit_task(emitter).await;
                        let status = report_status.status_handler().recv_async().await;
                        report_status.set_status_event(status)
                    } else {
                        report_status.set_status_event(StatusEvent::default());
                    }
                    report_status
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonNonEmit => {
            state.source = StatusSource::NonEmit;
            let mut report_status = state.show_status.clone();
            Task::perform(
                async { message_non_emit_task().await },
                move |status_event| {
                    report_status.set_status_event(status_event);
                    AppMessage::ShowStatus(report_status)
                },
            )
        }

        AppMessage::ButtonDirect => {
            state.source = StatusSource::Direct;
            let mut report_status = state.show_status.clone();
            report_status.set_status_event(status!("this is direct message"));
            Task::none()
        }

        AppMessage::ButtonOptionNonEmit => {
            state.source = StatusSource::OptionNonEmit;
            let mut current_status = state.show_status.clone();
            Task::perform(
                async { message_non_emit_with_option_task().await },
                move |maybe_status| match maybe_status {
                    Some(status) => {
                        current_status.set_status_event(status);
                        AppMessage::ShowStatus(current_status)
                    }
                    None => AppMessage::NoOperations,
                },
            )
        }

        AppMessage::ButtonOptionEmitAsync => {
            state.source = StatusSource::OptionEmitAsync;
            let mut report_status = state.show_status.clone();

            Task::perform(
                async move {
                    if let Some(emitter) = &report_status.status_handler().emitter() {
                        message_emit_with_option_task(Some(&emitter)).await;
                        let status = report_status.status_handler().recv_async().await;
                        report_status.set_status_event(status)
                    } else {
                        report_status.set_status_event(StatusEvent::default());
                    }
                    report_status
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

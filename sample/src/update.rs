use iced::Task;
use simple_status::*;

use crate::state::{AppMessage, AppState};
use crate::status_report::{StatusReport, StatusSource};
use crate::task::{
    message_emit_async_task, message_emit_task, message_emit_with_option_task,
    message_non_emit_task, message_non_emit_with_option_task,
};

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::ButtonEmitAsync => {
            let report_status = state.show_status.clone();

            Task::perform(
                async move {
                    let emitter = report_status.emitter.clone().unwrap();
                    message_emit_async_task(&emitter).await;
                    let status = report_status.recv_async().await;
                    report_status.update_status(status, StatusSource::EmitAsync)
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonEmit => {
            let report_status = state.show_status.clone();

            Task::perform(
                async move {
                    let emitter = report_status.emitter.clone().unwrap();
                    message_emit_task(&emitter).await;
                    let status = report_status.recv_sync();
                    report_status.update_status(status, StatusSource::Emit)
                },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonNonEmit => {
            Task::perform(async { message_non_emit_task().await }, |status_event| {
                let report =
                    StatusReport::default().update_status(status_event, StatusSource::NonEmit);
                AppMessage::ShowStatus(report)
            })
        }

        AppMessage::ButtonDirect => {
            state.show_status = state
                .show_status
                .update_status(status!("this is direct message"), StatusSource::Direct);
            Task::none()
        }

        AppMessage::ButtonOptionNonEmit => {
            let current_status = state.show_status.clone();
            Task::perform(
                async { message_non_emit_with_option_task().await },
                move |maybe_status| match maybe_status {
                    Some(status) => {
                        // Use update_status to preserve emitter/receiver/handle
                        AppMessage::ShowStatus(
                            current_status.update_status(status, StatusSource::OptionNonEmit),
                        )
                    }
                    None => AppMessage::NoOperations,
                },
            )
        }

        AppMessage::ButtonOptionEmitAsync => {
            let report_status = state.show_status.clone();

            Task::perform(
                async move {
                    let emitter = report_status.emitter.clone().unwrap();
                    message_emit_with_option_task(Some(&emitter)).await;
                    let status = report_status.recv_async().await;
                    report_status.update_status(status, StatusSource::OptionEmitAsync)
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

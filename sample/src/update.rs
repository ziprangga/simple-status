use crate::test_msg::TEST_BUS;
use iced::Task;
use iced::{Subscription, futures::StreamExt};
use simple_status::{ChannelKind, create_channels};

use crate::state::{AppMessage, AppState, StatusSource};
use crate::task::{
    direct_message_task, emit_async_message_task, emit_sync_message_task,
    global_emit_async_message_task, global_emit_async_with_progress_task,
    global_emit_sync_message_task, global_emit_sync_with_progress_task,
    independent_emit_async_with_progress_task, independent_emit_sync_with_progress_task,
};

pub fn update(state: &mut AppState, message: AppMessage) -> Task<AppMessage> {
    match message {
        AppMessage::ButtonDirect => {
            state.source = StatusSource::Direct;

            Task::perform(
                async { direct_message_task().await },
                AppMessage::ShowStatus,
            )
        }

        AppMessage::ButtonEmitSync => {
            state.source = StatusSource::EmitSync;
            let channels = create_channels(100, ChannelKind::Mpsc);
            let emitter = channels.get_emitter();
            let emit_task = Task::perform(
                async move {
                    emit_sync_message_task(&emitter).await;
                    AppMessage::NoOperations
                },
                |msg| msg,
            );

            let status_task = channels
                .stream()
                .map(|s| Task::stream(s.map(AppMessage::ShowStatus)))
                .unwrap_or_else(Task::none);

            Task::batch(vec![emit_task, status_task])
        }

        AppMessage::ButtonEmitAsync => {
            state.source = StatusSource::EmitAsync;
            let channels = create_channels(100, ChannelKind::Mpsc);
            let emitter = channels.get_emitter();
            let emit_task = Task::perform(
                async move {
                    emit_async_message_task(&emitter).await;
                    AppMessage::NoOperations
                },
                |msg| msg,
            );

            let stream_task = channels
                .stream()
                .map(|s| Task::stream(s.map(AppMessage::ShowStatus)))
                .unwrap_or_else(Task::none);

            Task::batch(vec![emit_task, stream_task])
        }

        AppMessage::ButtonGlobalEmitSync => {
            state.source = StatusSource::GlobalEmitSync;

            Task::perform(
                async {
                    global_emit_sync_message_task().await;
                    AppMessage::NoOperations
                },
                |msg| msg,
            )
        }

        AppMessage::ButtonGlobalEmitAsync => {
            state.source = StatusSource::GlobalEmitAsync;

            Task::perform(
                async {
                    global_emit_async_message_task().await;
                    AppMessage::NoOperations
                },
                |msg| msg,
            )
        }

        AppMessage::ButtonIndependentEmitSyncWithProgress => {
            state.source = StatusSource::IndependentEmitSyncWithProgress;
            let channels = create_channels(100, ChannelKind::Mpsc);
            let emitter = channels.get_emitter();

            let emit_task = Task::perform(
                async move {
                    independent_emit_sync_with_progress_task(&emitter).await;
                    AppMessage::NoOperations
                },
                |msg| msg,
            );

            let stream_task = channels
                .stream()
                .map(|s| Task::stream(s.map(AppMessage::ShowStatus)))
                .unwrap_or_else(Task::none);

            Task::batch(vec![emit_task, stream_task])
        }

        AppMessage::ButtonIndependentEmitAsyncWithProgress => {
            state.source = StatusSource::IndependentEmitAsyncWithProgress;
            let channels = create_channels(100, ChannelKind::Mpsc);
            let emitter = channels.get_emitter();

            let emit_task = Task::perform(
                async move {
                    independent_emit_async_with_progress_task(&emitter).await;
                    AppMessage::NoOperations
                },
                |msg| msg,
            );

            let stream_task = channels
                .stream()
                .map(|s| Task::stream(s.map(AppMessage::ShowStatus)))
                .unwrap_or_else(Task::none);

            Task::batch(vec![emit_task, stream_task])
        }

        AppMessage::ButtonGlobalEmitSyncWithProgress => {
            state.source = StatusSource::GlobalEmitSyncWithProgress;

            Task::perform(
                async {
                    global_emit_sync_with_progress_task().await;
                    AppMessage::NoOperations
                },
                |msg| msg,
            )
        }

        AppMessage::ButtonGlobalEmitAsyncWithProgress => {
            state.source = StatusSource::GlobalEmitAsyncWithProgress;

            Task::perform(
                async {
                    global_emit_async_with_progress_task().await;
                    AppMessage::NoOperations
                },
                |msg| msg,
            )
        }

        AppMessage::ShowStatus(status) => {
            state.show_status = status;
            Task::none()
        }

        AppMessage::NoOperations => Task::none(),
    }
}

pub fn subscription(_: &AppState) -> Subscription<AppMessage> {
    Subscription::run(|| match simple_status::stream(&TEST_BUS) {
        Some(stream) => stream.map(AppMessage::ShowStatus).boxed(),
        None => iced::futures::stream::empty()
            .map(AppMessage::ShowStatus)
            .boxed(),
    })
}

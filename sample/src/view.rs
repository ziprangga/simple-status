use iced::widget::{Column, Row};
use iced::{
    Color, alignment,
    widget::{Container, button, text},
};
use iced::{Element, Length};

use crate::button_style::{ButtonThemeStyle, CustomStyle};
use crate::state::{AppMessage, AppState, StatusSource};

pub fn view(state: &AppState) -> Element<'_, AppMessage> {
    let color = match state.source {
        StatusSource::Direct => Color::from_rgb8(255, 255, 0),

        StatusSource::EmitSync => Color::from_rgb8(0, 255, 0),
        StatusSource::EmitAsync => Color::from_rgb8(255, 0, 0),

        StatusSource::GlobalEmitSync => Color::from_rgb8(0, 128, 255),
        StatusSource::GlobalEmitAsync => Color::from_rgb8(128, 0, 255),

        StatusSource::IndependentEmitSyncWithProgress => Color::from_rgb8(0, 200, 100),
        StatusSource::IndependentEmitAsyncWithProgress => Color::from_rgb8(100, 200, 0),

        StatusSource::GlobalEmitSyncWithProgress => Color::from_rgb8(0, 200, 200),
        StatusSource::GlobalEmitAsyncWithProgress => Color::from_rgb8(200, 0, 200),
    };

    let show_status_message = Container::new(
        text(state.show_status.to_string())
            .size(12)
            .width(Length::Fill)
            .center()
            .style(move |_| iced::widget::text::Style { color: Some(color) }),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_direct = Container::new(
        button(text("Direct").size(12))
            .custom_style(ButtonThemeStyle::Default)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonDirect),
    );

    let button_emit_sync = Container::new(
        button(text("Independent Sync").size(12))
            .custom_style(ButtonThemeStyle::CustomRounded)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonEmitSync),
    );

    let button_emit_async = Container::new(
        button(text("Independent Async").size(12))
            .custom_style(ButtonThemeStyle::BlankBorder)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonEmitAsync),
    );

    let button_global_emit_sync = Container::new(
        button(text("Global Sync").size(12))
            .custom_style(ButtonThemeStyle::Custom)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonGlobalEmitSync),
    );

    let button_global_emit_async = Container::new(
        button(text("Global Async").size(12))
            .custom_style(ButtonThemeStyle::Default)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonGlobalEmitAsync),
    );

    let button_independent_sync_progress = Container::new(
        button(text("Independent Sync Progress").size(12))
            .custom_style(ButtonThemeStyle::BlankBorder)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonIndependentEmitSyncWithProgress),
    );

    let button_independent_async_progress = Container::new(
        button(text("Independent Async Progress").size(12))
            .custom_style(ButtonThemeStyle::Danger)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonIndependentEmitAsyncWithProgress),
    );

    let button_global_sync_progress = Container::new(
        button(text("Global Sync Progress").size(12))
            .custom_style(ButtonThemeStyle::Default)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonGlobalEmitSyncWithProgress),
    );

    let button_global_async_progress = Container::new(
        button(text("Global Async Progress").size(12))
            .custom_style(ButtonThemeStyle::CustomRounded)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonGlobalEmitAsyncWithProgress),
    );

    let row_button = Container::new(
        Row::new()
            .push(button_direct)
            .push(button_emit_sync)
            .push(button_emit_async)
            .push(button_global_emit_sync)
            .push(button_global_emit_async)
            .push(button_independent_sync_progress)
            .push(button_independent_async_progress)
            .push(button_global_sync_progress)
            .push(button_global_async_progress)
            .spacing(10)
            .width(Length::Fill)
            .align_y(alignment::Vertical::Center),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    Column::new()
        .push(show_status_message)
        .push(row_button)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(20)
        .padding(10)
        .into()
}

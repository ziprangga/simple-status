use iced::widget::{Column, Row};
use iced::{
    Color, alignment,
    widget::{Container, text},
};
use iced::{Element, Length};

use crate::button_style::*;
use crate::state::{AppMessage, AppState};
use crate::status_report::StatusSource;

pub fn view(state: &AppState) -> Element<'_, AppMessage> {
    let color = match state.show_status.source {
        StatusSource::EmitAsync => Color::from_rgb8(255, 0, 0),
        StatusSource::Emit => Color::from_rgb8(0, 255, 0),
        StatusSource::NonEmit => Color::from_rgb8(0, 0, 255),
        StatusSource::Direct => Color::from_rgb8(255, 255, 0),
        StatusSource::OptionNonEmit => Color::from_rgb8(0, 255, 255),
        StatusSource::OptionEmitAsync => Color::from_rgb8(255, 0, 255),
    };

    let show_status_message = Container::new(
        text(state.show_status.status_event.to_string())
            .size(12)
            .width(Length::Fill)
            .center()
            .style(move |_| iced::widget::text::Style { color: Some(color) }),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_emit_async = Container::new(
        CustomButton::new("Click me (async emit)")
            .text_align_x(alignment::Horizontal::Center)
            .text_align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonEmitAsync)
            .style(danger_style)
            .view(),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_emit = Container::new(
        CustomButton::new("Click me (emit)")
            .text_align_x(alignment::Horizontal::Center)
            .text_align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonEmit)
            .style(custom_btn_style)
            .view(),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_non_emit = Container::new(
        CustomButton::new("Click me (non emit)")
            .text_align_x(alignment::Horizontal::Center)
            .text_align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonNonEmit)
            .style(danger_style)
            .view(),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_direct = Container::new(
        CustomButton::new("Click me (direct)")
            .text_align_x(alignment::Horizontal::Center)
            .text_align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonDirect)
            .style(custom_btn_style)
            .view(),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_option_non_emit = Container::new(
        CustomButton::new("Click me (option non emit)")
            .text_align_x(alignment::Horizontal::Center)
            .text_align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonOptionNonEmit)
            .style(custom_btn_style)
            .view(),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_option_async_emit = Container::new(
        CustomButton::new("Click me (option async emit)")
            .text_align_x(alignment::Horizontal::Center)
            .text_align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonOptionEmitAsync)
            .style(custom_btn_style)
            .view(),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let row_button = Container::new(
        Row::new()
            .push(button_emit_async)
            .push(button_emit)
            .push(button_non_emit)
            .push(button_direct)
            .push(button_option_non_emit)
            .push(button_option_async_emit)
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

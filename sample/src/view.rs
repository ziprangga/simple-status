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
        StatusSource::EmitAsync => Color::from_rgb8(255, 0, 0),
        StatusSource::Emit => Color::from_rgb8(0, 255, 0),
        StatusSource::NonEmit => Color::from_rgb8(0, 0, 255),
        StatusSource::Direct => Color::from_rgb8(255, 255, 0),
        StatusSource::OptionNonEmit => Color::from_rgb8(0, 255, 255),
        StatusSource::OptionEmitAsync => Color::from_rgb8(255, 0, 255),
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

    let button_emit_async = Container::new(
        button(text("Click me (async emit)").size(12))
            .custom_style(ButtonThemeStyle::Default)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonEmitAsync),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_emit = Container::new(
        button(text("Click me (emit)").size(12))
            .custom_style(ButtonThemeStyle::CustomRounded)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonEmit),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_non_emit = Container::new(
        button(text("Click me (non emit)").size(12))
            .custom_style(ButtonThemeStyle::BlankBorder)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonNonEmit),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_direct = Container::new(
        button(text("Click me (direct)").size(12))
            .custom_style(ButtonThemeStyle::Danger)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonDirect),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_option_non_emit = Container::new(
        button(text("Click me (option non emit)").size(12))
            .custom_style(ButtonThemeStyle::Custom)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonOptionNonEmit),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_option_async_emit = Container::new(
        button(text("Click me (option async emit)").size(12))
            .custom_style(ButtonThemeStyle::Default)
            .width(Length::Fill)
            .on_press(AppMessage::ButtonOptionEmitAsync),
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

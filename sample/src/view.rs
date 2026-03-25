use iced::widget::Column;
use iced::{
    Color, alignment,
    widget::{Container, text},
};
use iced::{Element, Length};

use crate::button_style::*;
use crate::state::{AppMessage, AppState};

pub fn view(state: &AppState) -> Element<'_, AppMessage> {
    let status_msg_emit_async = Container::new(
        text(state.status_emit_async.to_string())
            .size(12)
            .width(Length::Fill)
            .center()
            .style(|_: &iced::Theme| iced::widget::text::Style {
                color: Some(Color::from_rgb8(200, 200, 200)),
            }),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let status_msg_emit = Container::new(
        text(state.status_emit.to_string())
            .size(12)
            .width(Length::Fill)
            .center()
            .style(|_: &iced::Theme| iced::widget::text::Style {
                color: Some(Color::from_rgb8(200, 200, 200)),
            }),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let status_msg_non_emit = Container::new(
        text(state.status_non_emit.to_string())
            .size(12)
            .width(Length::Fill)
            .center()
            .style(|_: &iced::Theme| iced::widget::text::Style {
                color: Some(Color::from_rgb8(150, 255, 150)),
            }),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let status_msg_direct = Container::new(
        text(state.status_direct.to_string())
            .size(12)
            .width(Length::Fill)
            .center()
            .style(|_: &iced::Theme| iced::widget::text::Style {
                color: Some(Color::from_rgb8(255, 150, 150)),
            }),
    )
    .width(Length::Fill)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    let button_show_message = Container::new(
        CustomButton::new("Click me")
            .text_align_x(alignment::Horizontal::Center)
            .text_align_y(alignment::Vertical::Center)
            .width(Length::Fill)
            .on_press(AppMessage::ShowMessage)
            .style(danger_style)
            .view(),
    )
    .width(Length::Shrink)
    .align_x(alignment::Horizontal::Center)
    .align_y(alignment::Vertical::Center);

    Column::new()
        .push(status_msg_emit_async)
        .push(status_msg_emit)
        .push(status_msg_non_emit)
        .push(status_msg_direct)
        .push(button_show_message)
        .width(Length::Fill)
        .height(Length::Fill)
        .spacing(10)
        .padding(10)
        .into()
}

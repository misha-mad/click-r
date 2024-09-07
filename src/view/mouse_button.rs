use crate::message::Message;
use crate::AutoClicker;
use enigo::Button as MouseButton;
use iced::alignment;
use iced::theme::Button;
use iced::widget::{button, row, text};
use iced::{Alignment, Length};

pub fn view(auto_clicker: &AutoClicker) -> iced::Element<Message> {
    row![
        row![
            text("Choose mouse button:").width(Length::FillPortion(1)),
            text(format!(
                "{:?}",
                auto_clicker.selected_mouse_button.lock().unwrap()
            ))
            .width(Length::FillPortion(1)),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(1)),
        row![
            row![
                button(text("Left").horizontal_alignment(alignment::Horizontal::Center))
                    .on_press(Message::SelectMouseButton(MouseButton::Left))
                    .style(
                        if *auto_clicker.selected_mouse_button.lock().unwrap() == MouseButton::Left
                        {
                            Button::Primary
                        } else {
                            Button::Secondary
                        },
                    )
                    .width(Length::FillPortion(1)),
                button(text("Middle").horizontal_alignment(alignment::Horizontal::Center))
                    .on_press(Message::SelectMouseButton(MouseButton::Middle))
                    .style(
                        if *auto_clicker.selected_mouse_button.lock().unwrap()
                            == MouseButton::Middle
                        {
                            Button::Primary
                        } else {
                            Button::Secondary
                        },
                    )
                    .width(Length::FillPortion(1)),
                button(text("Right").horizontal_alignment(alignment::Horizontal::Center))
                    .on_press(Message::SelectMouseButton(MouseButton::Right))
                    .style(
                        if *auto_clicker.selected_mouse_button.lock().unwrap() == MouseButton::Right
                        {
                            Button::Primary
                        } else {
                            Button::Secondary
                        },
                    )
                    .width(Length::FillPortion(1))
            ]
            .spacing(10)
            .align_items(Alignment::Center)
            .width(Length::FillPortion(1)),
            row![].width(Length::FillPortion(1)),
        ]
        .width(Length::FillPortion(2)),
    ]
    .align_items(Alignment::Center)
    .spacing(10)
    .into()
}

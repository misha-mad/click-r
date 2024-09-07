use crate::message::Message;
use crate::AutoClicker;
use iced::theme::Button;
use iced::widget::{button, column, horizontal_rule, row, text, Column};
use iced::Alignment;
use iced::{alignment, Length};

pub fn view(auto_clicker: &AutoClicker) -> Column<Message> {
    column![
        horizontal_rule(20),
        row![
            text(format!("Delay Timer: {}s", auto_clicker.delay_timer)),
            text(format!("Time Running: {}s", auto_clicker.time_running)),
            text(format!(
                "Total Clicks: {}",
                *auto_clicker.total_clicks.lock().unwrap()
            ))
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .height(Length::FillPortion(1)),
        horizontal_rule(20),
        row![
            row![
                button(text("Start").horizontal_alignment(alignment::Horizontal::Center))
                    .on_press_maybe(if *auto_clicker.is_running.lock().unwrap() {
                        None
                    } else {
                        Some(Message::Start)
                    })
                    .width(Length::FillPortion(1)),
                button(text("Stop").horizontal_alignment(alignment::Horizontal::Center))
                    .on_press_maybe(if *auto_clicker.is_running.lock().unwrap() {
                        Some(Message::Stop)
                    } else {
                        None
                    })
                    .width(Length::FillPortion(1)),
            ]
            .spacing(10),
            row![].width(Length::Fill),
            row![
                button(text("Save Settings").horizontal_alignment(alignment::Horizontal::Center))
                    .on_press(Message::SaveSettings)
                    .style(Button::Primary)
                    .on_press(Message::SaveSettings)
                    .style(Button::Positive)
                    .width(Length::FillPortion(1)),
                button(
                    text("Reset to Defaults").horizontal_alignment(alignment::Horizontal::Center)
                )
                .on_press(Message::ResetToDefaults)
                .style(Button::Destructive)
                .on_press(Message::ResetToDefaults)
                .width(Length::FillPortion(1)),
            ]
            .spacing(10),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .height(Length::FillPortion(1)),
    ]
    .spacing(10)
    .align_items(Alignment::Center)
    .into()
}

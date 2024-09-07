use crate::message::Message;
use crate::AutoClicker;
use iced::widget::{row, text, text_input};
use iced::{Alignment, Length};

pub fn view(auto_clicker: &AutoClicker) -> iced::Element<Message> {
    row![
        row![
            text("Delay before start:").width(Length::FillPortion(1)),
            text(format!(
                "{}s",
                auto_clicker.delay_seconds
                    + auto_clicker.delay_minutes * 60
                    + auto_clicker.delay_hours * 3600
            ))
            .width(Length::FillPortion(1)),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(1)),
        row![
            text("Hours:").width(Length::FillPortion(1)),
            text_input("Hours", &auto_clicker.delay_hours.to_string())
                .on_input(|s| {
                    if let Ok(value) = s.parse::<u64>() {
                        Message::DelayHoursChanged(value.min(23))
                    } else {
                        Message::DelayHoursChanged(0)
                    }
                })
                .width(Length::FillPortion(2)),
            text("Minutes:").width(Length::FillPortion(1)),
            text_input("Minutes", &auto_clicker.delay_minutes.to_string())
                .on_input(|s| {
                    if let Ok(value) = s.parse::<u64>() {
                        Message::DelayMinutesChanged(value.min(59))
                    } else {
                        Message::DelayMinutesChanged(0)
                    }
                })
                .width(Length::FillPortion(2)),
            text("Seconds:").width(Length::FillPortion(1)),
            text_input("Seconds", &auto_clicker.delay_seconds.to_string())
                .on_input(|s| {
                    if let Ok(value) = s.parse::<u64>() {
                        Message::DelaySecondsChanged(value.min(59))
                    } else {
                        Message::DelaySecondsChanged(0)
                    }
                })
                .width(Length::FillPortion(2)),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(2)),
    ]
    .into()
}

use crate::message::Message;
use crate::AutoClicker;
use iced::widget::{row, text, text_input};
use iced::{Alignment, Length};

pub fn view(auto_clicker: &AutoClicker) -> iced::Element<Message> {
    row![
        row![
            text("Duration:").width(Length::FillPortion(1)),
            text(
                if auto_clicker.duration_seconds == 0
                    && auto_clicker.duration_minutes == 0
                    && auto_clicker.duration_hours == 0
                {
                    "∞".to_string()
                } else {
                    format!(
                        "{}s",
                        (auto_clicker.duration_seconds
                            + auto_clicker.duration_minutes * 60
                            + auto_clicker.duration_hours * 3600)
                            .to_string()
                    )
                },
            )
            .width(Length::FillPortion(1)),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(1)),
        row![
            text("Hours:").width(Length::FillPortion(1)),
            text_input("Hours", &auto_clicker.duration_hours.to_string())
                .on_input(|s| {
                    if let Ok(value) = s.parse::<u64>() {
                        Message::DurationHoursChanged(value.min(23))
                    } else {
                        Message::DurationHoursChanged(0)
                    }
                })
                .width(Length::FillPortion(2)),
            text("Minutes:").width(Length::FillPortion(1)),
            text_input("Minutes", &auto_clicker.duration_minutes.to_string())
                .on_input(|s| {
                    if let Ok(value) = s.parse::<u64>() {
                        Message::DurationMinutesChanged(value.min(59))
                    } else {
                        Message::DurationMinutesChanged(0)
                    }
                })
                .width(Length::FillPortion(2)),
            text("Seconds:").width(Length::FillPortion(1)),
            text_input("Seconds", &auto_clicker.duration_seconds.to_string())
                .on_input(|s| {
                    if let Ok(value) = s.parse::<u64>() {
                        Message::DurationSecondsChanged(value.min(59))
                    } else {
                        Message::DurationSecondsChanged(0)
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

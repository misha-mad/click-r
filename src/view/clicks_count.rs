use crate::message::Message;
use crate::AutoClicker;
use iced::widget::{row, slider, text};
use iced::{Alignment, Length};

pub fn view(auto_clicker: &AutoClicker) -> iced::Element<Message> {
    row![
        row![
            text("Click counts between intervals:").width(Length::FillPortion(1)),
            text(format!(
                "{:?} {}",
                auto_clicker.clicks_count_slider_value,
                if auto_clicker.clicks_count_slider_value == 1 {
                    "click"
                } else {
                    "clicks"
                }
            ))
            .width(Length::FillPortion(1)),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(1)),
        slider(
            1..=100,
            auto_clicker.clicks_count_slider_value,
            Message::ClickCountSliderChanged,
        )
        .width(Length::FillPortion(2))
    ]
    .align_items(Alignment::Center)
    .spacing(10)
    .into()
}

use crate::message::Message;
use crate::AutoClicker;
use iced::widget::{row, slider, text};
use iced::{Alignment, Length};

pub fn view(auto_clicker: &AutoClicker) -> iced::Element<Message> {
    row![
        row![
            text("Interval:").width(Length::FillPortion(1)),
            text(format!("{:?}s", auto_clicker.click_interval_slider_value))
                .width(Length::FillPortion(1)),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(1)),
        slider(
            1..=100,
            auto_clicker.click_interval_slider_value,
            Message::IntervalSliderChanged,
        )
        .width(Length::FillPortion(2))
    ]
    .align_items(Alignment::Center)
    .spacing(10)
    .into()
}

use crate::message::Message;
use crate::AutoClicker;
use iced::widget::{pick_list, row, text};
use iced::{Alignment, Length, Theme};

pub fn view(auto_clicker: &AutoClicker) -> iced::Element<Message> {
    row![
        row![
            text("Theme:").width(Length::FillPortion(1)),
            text(format!("{:?}", auto_clicker.theme)).width(Length::FillPortion(1)),
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(1)),
        pick_list(Theme::ALL, Some(&auto_clicker.theme), Message::ThemeChanged)
            .width(Length::FillPortion(2))
    ]
    .align_items(Alignment::Center)
    .spacing(10)
    .into()
}

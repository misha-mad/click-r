use crate::message::Message;
use crate::view::{
    clicks_count, delay_before_start, duration, footer, interval, mouse_button, parameter_name,
    theme,
};
use crate::AutoClicker;
use iced::widget::column;
use iced::{Alignment, Length};

pub fn view(auto_clicker: &AutoClicker) -> iced::Element<Message> {
    column![
        column![
            parameter_name::view(),
            theme::view(auto_clicker),
            interval::view(auto_clicker),
            clicks_count::view(auto_clicker),
            delay_before_start::view(auto_clicker),
            duration::view(auto_clicker),
            mouse_button::view(auto_clicker),
        ]
        .spacing(20)
        .align_items(Alignment::Center)
        .height(Length::FillPortion(3)),
        footer::view(auto_clicker).height(Length::FillPortion(1))
    ]
    .spacing(10)
    .padding(20)
    .align_items(Alignment::Center)
    .into()
}

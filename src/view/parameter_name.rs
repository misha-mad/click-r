use crate::message::Message;
use iced::font::{Family, Stretch, Style, Weight};
use iced::widget::{row, text};
use iced::{Alignment, Font, Length};

pub fn view() -> iced::Element<'static, Message> {
    row![
        row![
            text("Parameter Name:")
                .font(Font {
                    family: Family::SansSerif,
                    weight: Weight::Bold,
                    stretch: Stretch::Normal,
                    style: Style::Normal,
                })
                .width(Length::FillPortion(1)),
            text("Current Value:")
                .font(Font {
                    family: Family::SansSerif,
                    weight: Weight::Bold,
                    stretch: Stretch::Normal,
                    style: Style::Normal,
                })
                .width(Length::FillPortion(1))
        ]
        .align_items(Alignment::Center)
        .spacing(10)
        .width(Length::FillPortion(1)),
        text("Input:")
            .font(Font {
                family: Family::SansSerif,
                weight: Weight::Bold,
                stretch: Stretch::Normal,
                style: Style::Normal,
            })
            .width(Length::FillPortion(2))
    ]
    .align_items(Alignment::Center)
    .spacing(10)
    .into()
}

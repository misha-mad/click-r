use crate::update::Message;
use crate::AutoClicker;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker, new_clicks_count: u8) -> Command<Message> {
    auto_clicker.clicks_count_slider_value = new_clicks_count;
    Command::none()
}

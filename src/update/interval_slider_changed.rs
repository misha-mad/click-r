use crate::update::Message;
use crate::AutoClicker;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker, new_interval: u8) -> Command<Message> {
    auto_clicker.click_interval_slider_value = new_interval;
    Command::none()
}

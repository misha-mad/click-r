use crate::update::Message;
use crate::AutoClicker;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker, new_seconds: u64) -> Command<Message> {
    auto_clicker.duration_seconds = new_seconds;
    Command::none()
}

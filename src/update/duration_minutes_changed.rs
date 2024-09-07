use crate::update::Message;
use crate::AutoClicker;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker, new_minutes: u64) -> Command<Message> {
    auto_clicker.duration_minutes = new_minutes;
    Command::none()
}

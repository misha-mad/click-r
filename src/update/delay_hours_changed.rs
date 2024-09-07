use crate::update::Message;
use crate::AutoClicker;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker, new_hours: u64) -> Command<Message> {
    auto_clicker.delay_hours = new_hours;
    Command::none()
}

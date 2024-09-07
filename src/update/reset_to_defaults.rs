use crate::update::Message;
use crate::AutoClicker;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker) -> Command<Message> {
    *auto_clicker = AutoClicker::default();
    Command::none()
}

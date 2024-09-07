use crate::update::Message;
use crate::AutoClicker;
use iced::{Command, Theme};

pub fn handle(auto_clicker: &mut AutoClicker, theme: Theme) -> Command<Message> {
    auto_clicker.theme = theme;
    Command::none()
}

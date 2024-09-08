use crate::update::{start, stop, Message};
use crate::AutoClicker;
use iced::keyboard::key::Named;
use iced::keyboard::Key;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker, key_code: Key) -> Command<Message> {
    match key_code {
        Key::Named(name) => match name {
            Named::F6 => {
                let _ = start::handle(auto_clicker);
                Command::none()
            }

            Named::F7 => {
                let _ = stop::handle(auto_clicker);
                Command::none()
            }
            _ => Command::none(),
        },
        _ => {
            Command::none()
        },
    }
}

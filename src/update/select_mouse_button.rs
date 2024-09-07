use crate::update::Message;
use crate::AutoClicker;
use enigo::Button as MouseButton;
use iced::Command;
use std::sync::Arc;
use std::sync::Mutex;

pub fn handle(auto_clicker: &mut AutoClicker, button: MouseButton) -> Command<Message> {
    auto_clicker.selected_mouse_button = Arc::new(Mutex::new(button));
    Command::none()
}

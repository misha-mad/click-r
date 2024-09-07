use crate::update::Message;
use crate::AutoClicker;
use iced::Command;
use std::fs;

pub fn handle(auto_clicker: &mut AutoClicker) -> Command<Message> {
    let settings = serde_json::to_string(auto_clicker).unwrap();
    fs::write("settings.json", settings).expect("Unable to write settings to file");
    Command::none()
}

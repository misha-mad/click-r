use crate::update::Message;
use crate::AutoClicker;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker) -> Command<Message> {
    if let Some(sender) = auto_clicker.stop_sender.take() {
        if sender.send(()).is_ok() {
            if let Some(handle) = auto_clicker.click_thread.take() {
                handle.thread().unpark();
                handle.join().unwrap();
            }
        }
    }

    Command::none()
}

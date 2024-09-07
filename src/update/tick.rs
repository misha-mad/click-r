use crate::update::Message;
use crate::AutoClicker;
use iced::Command;

pub fn handle(auto_clicker: &mut AutoClicker) -> Command<Message> {
    if *auto_clicker.is_running.lock().unwrap() {
        auto_clicker.ticks_count += 1;

        let delay_before_start = (auto_clicker.delay_hours * 3600)
            + (auto_clicker.delay_minutes * 60)
            + auto_clicker.delay_seconds;

        if auto_clicker.ticks_count > delay_before_start {
            auto_clicker.time_running += 1;
        } else {
            auto_clicker.delay_timer += 1;
        }
    }

    Command::none()
}

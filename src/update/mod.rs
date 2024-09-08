mod click_count_slider_changed;
mod delay_hours_changed;
mod delay_minutes_changed;
mod delay_seconds_changed;
mod duration_hours_changed;
mod duration_minutes_changed;
mod duration_seconds_changed;
mod interval_slider_changed;
mod key_pressed;
mod reset_to_defaults;
mod save_settings;
mod select_mouse_button;
mod start;
mod stop;
mod theme_changed;
mod tick;

use crate::message::Message;
use crate::AutoClicker;
use iced::Command;

pub fn update_handler(auto_clicker: &mut AutoClicker, message: Message) -> Command<Message> {
    match message {
        Message::ThemeChanged(theme) => theme_changed::handle(auto_clicker, theme),
        Message::SelectMouseButton(button) => select_mouse_button::handle(auto_clicker, button),
        Message::Start => start::handle(auto_clicker),
        Message::Stop => stop::handle(auto_clicker),
        Message::Tick => tick::handle(auto_clicker),
        Message::ResetToDefaults => reset_to_defaults::handle(auto_clicker),
        Message::IntervalSliderChanged(new_interval) => {
            interval_slider_changed::handle(auto_clicker, new_interval)
        }
        Message::ClickCountSliderChanged(new_clicks_count) => {
            click_count_slider_changed::handle(auto_clicker, new_clicks_count)
        }
        Message::DelayHoursChanged(new_hours) => {
            delay_hours_changed::handle(auto_clicker, new_hours)
        }
        Message::DelayMinutesChanged(new_minutes) => {
            delay_minutes_changed::handle(auto_clicker, new_minutes)
        }
        Message::DelaySecondsChanged(new_seconds) => {
            delay_seconds_changed::handle(auto_clicker, new_seconds)
        }
        Message::DurationHoursChanged(new_hours) => {
            duration_hours_changed::handle(auto_clicker, new_hours)
        }
        Message::DurationMinutesChanged(new_minutes) => {
            duration_minutes_changed::handle(auto_clicker, new_minutes)
        }
        Message::DurationSecondsChanged(new_seconds) => {
            duration_seconds_changed::handle(auto_clicker, new_seconds)
        }
        Message::SaveSettings => save_settings::handle(auto_clicker),
        Message::KeyPressed(key) => key_pressed::handle(auto_clicker, key),
        Message::None => Command::none(),
    }
}

use crate::update::Message;
use crate::AutoClicker;
use enigo::{Direction::Click, Enigo, Mouse, Settings as EnigoSettings};
use iced::Command;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

pub fn handle(auto_clicker: &mut AutoClicker) -> Command<Message> {
    *auto_clicker.is_running.lock().unwrap() = true;
    auto_clicker.delay_timer = 0;
    auto_clicker.time_running = 0;
    auto_clicker.ticks_count = 0;
    auto_clicker.total_clicks = Arc::new(Mutex::new(0));
    let interval = auto_clicker.click_interval_slider_value;
    let clicks_count = auto_clicker.clicks_count_slider_value;
    let (tx, rx) = mpsc::channel();
    let total_clicks = Arc::clone(&auto_clicker.total_clicks);
    let selected_mouse_button = Arc::clone(&auto_clicker.selected_mouse_button);
    let is_running = Arc::clone(&auto_clicker.is_running);

    let delay_before_start = auto_clicker.delay_seconds
        + auto_clicker.delay_minutes * 60
        + auto_clicker.delay_hours * 3600;

    let duration = if auto_clicker.duration_seconds == 0
        && auto_clicker.duration_minutes == 0
        && auto_clicker.duration_hours == 0
    {
        None
    } else {
        Some(
            auto_clicker.duration_seconds
                + auto_clicker.duration_minutes * 60
                + auto_clicker.duration_hours * 3600,
        )
    };

    auto_clicker.stop_sender = Some(tx);

    let handle = thread::spawn(move || {
        thread::park_timeout(Duration::from_secs(delay_before_start));
        let mut enigo = Enigo::new(&EnigoSettings::default()).unwrap();
        let button = selected_mouse_button.lock().unwrap().clone();
        let end_time = std::time::Instant::now() + Duration::from_secs(duration.unwrap_or(0));

        loop {
            if rx.try_recv().is_ok() {
                *is_running.lock().unwrap() = false;
                break;
            }

            if duration.is_some() && std::time::Instant::now() >= end_time {
                *is_running.lock().unwrap() = false;
                break;
            }

            for _ in 0..clicks_count {
                enigo.button(button, Click).unwrap();
                *total_clicks.lock().unwrap() += 1;
            }

            thread::park_timeout(Duration::from_secs(interval as u64));
        }
    });

    auto_clicker.click_thread = Some(handle);
    Command::none()
}

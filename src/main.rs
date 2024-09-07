mod message;
mod theme;
mod update;
mod utils;
mod view;

use crate::message::Message;
use crate::theme::ThemeDef;
use crate::update::update_handler;
use crate::utils::{deserialize_mouse_button, serialize_mouse_button};
use crate::view::view_handler;
use enigo::Button as MouseButton;
use iced::theme::Theme;
use iced::{executor, Application, Command, Element, Settings as IcedSettings, Subscription};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::{mpsc, Arc, Mutex};
use std::time::Duration;
use std::{fs, thread};

#[derive(Serialize, Deserialize)]
struct AutoClicker {
    click_interval_slider_value: u8,
    #[serde(skip)]
    click_thread: Option<thread::JoinHandle<()>>,
    clicks_count_slider_value: u8,
    delay_hours: u64,
    delay_minutes: u64,
    delay_seconds: u64,
    duration_hours: u64,
    duration_minutes: u64,
    duration_seconds: u64,
    #[serde(skip)]
    delay_timer: u64,
    #[serde(skip)]
    time_running: u64,
    #[serde(skip)]
    is_running: Arc<Mutex<bool>>,
    #[serde(
        serialize_with = "serialize_mouse_button",
        deserialize_with = "deserialize_mouse_button"
    )]
    selected_mouse_button: Arc<Mutex<MouseButton>>,
    #[serde(skip)]
    stop_sender: Option<mpsc::Sender<()>>,
    #[serde(with = "ThemeDef")]
    theme: Theme,
    #[serde(skip)]
    ticks_count: u64,
    #[serde(skip)]
    total_clicks: Arc<Mutex<u32>>,
}

impl Application for AutoClicker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let default_settings = Self::default();

        if Path::new("settings.json").exists() {
            match fs::read_to_string("settings.json") {
                Ok(settings) => match serde_json::from_str(&settings) {
                    Ok(new_settings) => (new_settings, Command::none()),
                    Err(_) => (default_settings, Command::none()),
                },
                Err(_) => (default_settings, Command::none()),
            }
        } else {
            (default_settings, Command::none())
        }
    }

    fn title(&self) -> String {
        String::from("Click-R")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        update_handler(self, message)
    }

    fn view(&self) -> Element<Self::Message> {
        view_handler(self)
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if *self.is_running.lock().unwrap() {
            iced::time::every(Duration::from_millis(1000)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}

impl Default for AutoClicker {
    fn default() -> Self {
        Self {
            click_interval_slider_value: 1,
            click_thread: None,
            clicks_count_slider_value: 1,
            delay_hours: 0,
            delay_minutes: 0,
            delay_seconds: 0,
            delay_timer: 0,
            duration_hours: 0,
            duration_minutes: 0,
            duration_seconds: 0,
            time_running: 0,
            is_running: Arc::new(Mutex::new(false)),
            selected_mouse_button: Arc::new(Mutex::new(MouseButton::Left)),
            stop_sender: None,
            theme: Theme::Oxocarbon,
            ticks_count: 0,
            total_clicks: Arc::new(Mutex::new(0)),
        }
    }
}

fn main() -> iced::Result {
    AutoClicker::run(IcedSettings::default())
}

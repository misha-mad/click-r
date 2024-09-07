mod update;
mod utils;

use crate::update::{update_handler, Message};
use crate::utils::{deserialize_mouse_button, serialize_mouse_button, ThemeDef};
use enigo::Button as MouseButton;
use iced::alignment;
use iced::font::{Family, Stretch, Style, Weight};
use iced::theme::{Button, Theme};
use iced::widget::{button, column, horizontal_rule, pick_list, row, slider, text, text_input};
use iced::Alignment::Center;
use iced::Font;
use iced::Length::{Fill, FillPortion};
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
        let content = column![
            // The `Parameter Name` section
            row![
                row![
                    text("Parameter Name:")
                        .font(Font {
                            family: Family::SansSerif,
                            weight: Weight::Bold,
                            stretch: Stretch::Normal,
                            style: Style::Normal,
                        })
                        .width(FillPortion(1)),
                    text("Current Value:")
                        .font(Font {
                            family: Family::SansSerif,
                            weight: Weight::Bold,
                            stretch: Stretch::Normal,
                            style: Style::Normal,
                        })
                        .width(FillPortion(1))
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                text("Input:")
                    .font(Font {
                        family: Family::SansSerif,
                        weight: Weight::Bold,
                        stretch: Stretch::Normal,
                        style: Style::Normal,
                    })
                    .width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Theme` section
            row![
                row![
                    text("Theme:").width(FillPortion(1)),
                    text(format!("{:?}", self.theme)).width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged)
                    .width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Interval` section
            row![
                row![
                    text("Interval:").width(FillPortion(1)),
                    text(format!("{:?}s", self.click_interval_slider_value)).width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                slider(
                    1..=100,
                    self.click_interval_slider_value,
                    Message::IntervalSliderChanged,
                )
                .width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Clicks` section
            row![
                row![
                    text("Click counts between intervals:").width(FillPortion(1)),
                    text(format!(
                        "{:?} {}",
                        self.clicks_count_slider_value,
                        if self.clicks_count_slider_value == 1 {
                            "click"
                        } else {
                            "clicks"
                        }
                    ))
                    .width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                slider(
                    1..=100,
                    self.clicks_count_slider_value,
                    Message::ClickCountSliderChanged,
                )
                .width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Delay Before Start` section
            row![
                row![
                    text("Delay before start:").width(FillPortion(1)),
                    text(format!(
                        "{}s",
                        self.delay_seconds + self.delay_minutes * 60 + self.delay_hours * 3600
                    ))
                    .width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                row![
                    text("Hours:").width(FillPortion(1)),
                    text_input("Hours", &self.delay_hours.to_string())
                        .on_input(|s| {
                            if let Ok(value) = s.parse::<u64>() {
                                Message::DelayHoursChanged(value.min(23))
                            } else {
                                Message::DelayHoursChanged(0)
                            }
                        })
                        .width(FillPortion(1))
                        .width(FillPortion(2)),
                    text("Minutes:").width(FillPortion(1)),
                    text_input("Minutes", &self.delay_minutes.to_string())
                        .on_input(|s| {
                            if let Ok(value) = s.parse::<u64>() {
                                Message::DelayMinutesChanged(value.min(59))
                            } else {
                                Message::DelayMinutesChanged(0)
                            }
                        })
                        .width(FillPortion(1))
                        .width(FillPortion(2)),
                    text("Seconds:").width(FillPortion(1)),
                    text_input("Seconds", &self.delay_seconds.to_string())
                        .on_input(|s| {
                            if let Ok(value) = s.parse::<u64>() {
                                Message::DelaySecondsChanged(value.min(59))
                            } else {
                                Message::DelaySecondsChanged(0)
                            }
                        })
                        .width(FillPortion(1))
                        .width(FillPortion(2)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(2)),
            ],
            horizontal_rule(20),
            // The `Duration` section
            row![
                row![
                    text("Duration:").width(FillPortion(1)),
                    text(
                        if self.duration_seconds == 0
                            && self.duration_minutes == 0
                            && self.duration_hours == 0
                        {
                            "∞".to_string()
                        } else {
                            format!(
                                "{}s",
                                (self.duration_seconds
                                    + self.duration_minutes * 60
                                    + self.duration_hours * 3600)
                                    .to_string()
                            )
                        },
                    )
                    .width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                row![
                    text("Hours:").width(FillPortion(1)),
                    text_input("Hours", &self.duration_hours.to_string())
                        .on_input(|s| {
                            if let Ok(value) = s.parse::<u64>() {
                                Message::DurationHoursChanged(value.min(23))
                            } else {
                                Message::DurationHoursChanged(0)
                            }
                        })
                        .width(FillPortion(2)),
                    text("Minutes:").width(FillPortion(1)),
                    text_input("Minutes", &self.duration_minutes.to_string())
                        .on_input(|s| {
                            if let Ok(value) = s.parse::<u64>() {
                                Message::DurationMinutesChanged(value.min(59))
                            } else {
                                Message::DurationMinutesChanged(0)
                            }
                        })
                        .width(FillPortion(2)),
                    text("Seconds:").width(FillPortion(1)),
                    text_input("Seconds", &self.duration_seconds.to_string())
                        .on_input(|s| {
                            if let Ok(value) = s.parse::<u64>() {
                                Message::DurationSecondsChanged(value.min(59))
                            } else {
                                Message::DurationSecondsChanged(0)
                            }
                        })
                        .width(FillPortion(2)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(2)),
            ],
            horizontal_rule(20),
            // The `Mouse Button` section
            row![
                row![
                    text("Choose mouse button:").width(FillPortion(1)),
                    text(format!("{:?}", self.selected_mouse_button.lock().unwrap()))
                        .width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                row![
                    row![
                        button(text("Left").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::SelectMouseButton(MouseButton::Left))
                            .style(
                                if *self.selected_mouse_button.lock().unwrap() == MouseButton::Left
                                {
                                    Button::Primary
                                } else {
                                    Button::Secondary
                                },
                            )
                            .width(FillPortion(1)),
                        button(text("Middle").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::SelectMouseButton(MouseButton::Middle))
                            .style(
                                if *self.selected_mouse_button.lock().unwrap()
                                    == MouseButton::Middle
                                {
                                    Button::Primary
                                } else {
                                    Button::Secondary
                                },
                            )
                            .width(FillPortion(1)),
                        button(text("Right").horizontal_alignment(alignment::Horizontal::Center))
                            .on_press(Message::SelectMouseButton(MouseButton::Right))
                            .style(
                                if *self.selected_mouse_button.lock().unwrap() == MouseButton::Right
                                {
                                    Button::Primary
                                } else {
                                    Button::Secondary
                                },
                            )
                            .width(FillPortion(1))
                    ]
                    .spacing(10)
                    .align_items(Center)
                    .width(FillPortion(1)),
                    row![].width(FillPortion(1)),
                ]
                .width(FillPortion(2)),
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
        ]
        .spacing(10)
        .padding(20)
        .align_items(Center);

        let footer = column![
            horizontal_rule(20),
            row![
                text(format!("Delay Timer: {}s", self.delay_timer)),
                text(format!("Time Running: {}s", self.time_running)),
                text(format!(
                    "Total Clicks: {}",
                    *self.total_clicks.lock().unwrap()
                ))
            ]
            .align_items(Center)
            .spacing(10)
            .height(FillPortion(1)),
            horizontal_rule(20),
            row![
                row![
                    button(text("Start").horizontal_alignment(alignment::Horizontal::Center))
                        .on_press_maybe(if *self.is_running.lock().unwrap() {
                            None
                        } else {
                            Some(Message::Start)
                        })
                        .width(FillPortion(1)),
                    button(text("Stop").horizontal_alignment(alignment::Horizontal::Center))
                        .on_press_maybe(if *self.is_running.lock().unwrap() {
                            Some(Message::Stop)
                        } else {
                            None
                        })
                        .width(FillPortion(1)),
                ]
                .spacing(10),
                row![].width(Fill),
                row![
                    button(
                        text("Save Settings").horizontal_alignment(alignment::Horizontal::Center)
                    )
                    .on_press(Message::SaveSettings)
                    .style(Button::Primary)
                    .on_press(Message::SaveSettings)
                    .style(Button::Positive)
                    .width(FillPortion(1)),
                    button(
                        text("Reset to Defaults")
                            .horizontal_alignment(alignment::Horizontal::Center)
                    )
                    .on_press(Message::ResetToDefaults)
                    .style(Button::Destructive)
                    .on_press(Message::ResetToDefaults)
                    .width(FillPortion(1)),
                ]
                .spacing(10),
            ]
            .align_items(Center)
            .spacing(10)
            .height(FillPortion(1)),
        ]
        .spacing(10)
        .padding(20)
        .align_items(Center);

        let page = column![
            content.height(FillPortion(3)),
            footer.height(FillPortion(1))
        ]
        .spacing(10)
        .padding(20)
        .align_items(Center);

        page.into()
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

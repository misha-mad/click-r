use enigo::{Button as MouseButton, Direction::Click, Enigo, Mouse, Settings as EnigoSettings};
use iced::font::{Family, Stretch, Style, Weight};
use iced::theme::{Button, Theme};
use iced::widget::{button, column, horizontal_rule, pick_list, row, slider, text, text_input};
use iced::Alignment::Center;
use iced::Font;
use iced::Length::FillPortion;
use iced::{executor, Application, Command, Element, Settings as IcedSettings, Subscription};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct AutoClicker {
    click_interval_slider_value: u8,
    click_thread: Option<thread::JoinHandle<()>>,
    clicks_count_slider_value: u8,
    delay_hours: u64,
    delay_minutes: u64,
    delay_seconds: u64,
    duration_hours: u64,
    duration_minutes: u64,
    duration_seconds: u64,
    delay_timer: u64,
    time_running: u64,
    is_running: Arc<Mutex<bool>>,
    selected_mouse_button: Arc<Mutex<MouseButton>>,
    stop_sender: Option<mpsc::Sender<()>>,
    theme: Theme,
    ticks_count: u64,
    total_clicks: Arc<Mutex<u32>>,
}

#[derive(Debug, Clone)]
enum Message {
    ClickCountSliderChanged(u8),
    DelayHoursChanged(u64),
    DelayMinutesChanged(u64),
    DelaySecondsChanged(u64),
    DurationHoursChanged(u64),
    DurationMinutesChanged(u64),
    DurationSecondsChanged(u64),
    IntervalSliderChanged(u8),
    SelectMouseButton(MouseButton),
    Start,
    Stop,
    ThemeChanged(Theme),
    Tick,
}

impl Application for AutoClicker {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (Self::default(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Click-R")
    }

    fn update(&mut self, message: Message) -> Command<Self::Message> {
        match message {
            Message::ThemeChanged(theme) => {
                self.theme = theme;
                Command::none()
            }
            Message::SelectMouseButton(button) => {
                self.selected_mouse_button = Arc::new(Mutex::new(button));
                Command::none()
            }
            Message::Start => {
                println!("Starting the auto clicker");
                *self.is_running.lock().unwrap() = true;
                self.delay_timer = 0;
                self.time_running = 0;
                self.ticks_count = 0;
                self.total_clicks = Arc::new(Mutex::new(0));
                println!("click_interval_secs {}", self.click_interval_slider_value);
                let interval = self.click_interval_slider_value;
                let clicks_count = self.clicks_count_slider_value;
                let (tx, rx) = mpsc::channel();
                let total_clicks = Arc::clone(&self.total_clicks);
                let selected_mouse_button = Arc::clone(&self.selected_mouse_button);
                let is_running = Arc::clone(&self.is_running);

                let delay_before_start =
                    self.delay_seconds + self.delay_minutes * 60 + self.delay_hours * 3600;

                let duration = if self.duration_seconds == 0
                    && self.duration_minutes == 0
                    && self.duration_hours == 0
                {
                    None
                } else {
                    Some(
                        self.duration_seconds
                            + self.duration_minutes * 60
                            + self.duration_hours * 3600,
                    )
                };

                self.stop_sender = Some(tx);

                let handle = thread::spawn(move || {
                    println!("interval {}", interval);
                    thread::park_timeout(Duration::from_secs(delay_before_start));
                    let mut enigo = Enigo::new(&EnigoSettings::default()).unwrap();
                    let button = selected_mouse_button.lock().unwrap().clone();

                    let end_time =
                        std::time::Instant::now() + Duration::from_secs(duration.unwrap_or(0));

                    loop {
                        if rx.try_recv().is_ok() {
                            *is_running.lock().unwrap() = false;
                            println!("Stop signal received");
                            break;
                        }

                        if duration.is_some() && std::time::Instant::now() >= end_time {
                            *is_running.lock().unwrap() = false;
                            println!("Duration elapsed");
                            break;
                        }

                        println!("Clicking every {} seconds", interval);

                        for _ in 0..clicks_count {
                            println!("Clicking");
                            enigo.button(button, Click).unwrap();
                            *total_clicks.lock().unwrap() += 1;
                        }

                        thread::park_timeout(Duration::from_secs(interval as u64));
                    }
                });

                self.click_thread = Some(handle);
                Command::none()
            }
            Message::Stop => {
                println!("Stopping the auto clicker");

                if let Some(sender) = self.stop_sender.take() {
                    if sender.send(()).is_ok() {
                        if let Some(handle) = self.click_thread.take() {
                            handle.thread().unpark();
                            handle.join().unwrap();
                        }
                    }
                }

                Command::none()
            }
            Message::Tick => {
                if *self.is_running.lock().unwrap() {
                    self.ticks_count += 1;

                    let delay_before_start =
                        (self.delay_hours * 3600) + (self.delay_minutes * 60) + self.delay_seconds;

                    if self.ticks_count > delay_before_start {
                        self.time_running += 1;
                    } else {
                        self.delay_timer += 1;
                    }
                }

                Command::none()
            }
            Message::IntervalSliderChanged(new_interval) => {
                println!("Setting click_interval_secs to {} seconds", new_interval);
                self.click_interval_slider_value = new_interval;
                Command::none()
            }
            Message::ClickCountSliderChanged(new_clicks_count) => {
                println!("Setting clicks_count_slider_value to {}", new_clicks_count);
                self.clicks_count_slider_value = new_clicks_count;
                Command::none()
            }
            Message::DelayHoursChanged(new_hours) => {
                println!("Setting delay_hours to {}", new_hours);
                self.delay_hours = new_hours;
                Command::none()
            }
            Message::DelayMinutesChanged(new_minutes) => {
                println!("Setting delay_minutes to {}", new_minutes);
                self.delay_minutes = new_minutes;
                Command::none()
            }
            Message::DelaySecondsChanged(new_seconds) => {
                println!("Setting delay_seconds to {}", new_seconds);
                self.delay_seconds = new_seconds;
                Command::none()
            }
            Message::DurationHoursChanged(new_hours) => {
                println!("Setting duration_hours to {}", new_hours);
                self.duration_hours = new_hours;
                Command::none()
            }
            Message::DurationMinutesChanged(new_minutes) => {
                println!("Setting duration_minutes to {}", new_minutes);
                self.duration_minutes = new_minutes;
                Command::none()
            }
            Message::DurationSecondsChanged(new_seconds) => {
                println!("Setting duration_seconds to {}", new_seconds);
                self.duration_seconds = new_seconds;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let parameter_name_text = text("Parameter Name:").font(Font {
            family: Family::SansSerif,
            weight: Weight::Bold,
            stretch: Stretch::Normal,
            style: Style::Normal,
        });

        let current_value_text = text("Current Value:").font(Font {
            family: Family::SansSerif,
            weight: Weight::Bold,
            stretch: Stretch::Normal,
            style: Style::Normal,
        });

        let input_text = text("Input:").font(Font {
            family: Family::SansSerif,
            weight: Weight::Bold,
            stretch: Stretch::Normal,
            style: Style::Normal,
        });

        let theme_text = text("Theme:");
        let theme_current_value_text = text(format!("{:?}", self.theme));
        let pick_list = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);
        let interval_text = text("Interval:");
        let interval_current_value_text = text(format!("{:?}", self.click_interval_slider_value));

        let interval_slider = slider(
            1..=100,
            self.click_interval_slider_value,
            Message::IntervalSliderChanged,
        );

        let clicks_count_text = text("Clicks per interval:");
        let clicks_count_current_value_text = text(format!("{:?}", self.clicks_count_slider_value));

        let clicks_count_slider = slider(
            1..=100,
            self.clicks_count_slider_value,
            Message::ClickCountSliderChanged,
        );

        let delay_before_start_text = text("Delay before start: {}s");

        let delay_before_start_current_value_text = text(format!(
            "{}s",
            self.delay_seconds + self.delay_minutes * 60 + self.delay_hours * 3600
        ));

        let delay_hours_text = text("Hours:");

        let delay_hours_input = text_input("Hours", &self.delay_hours.to_string())
            .on_input(|s| {
                if let Ok(value) = s.parse::<u64>() {
                    Message::DelayHoursChanged(value.min(23))
                } else {
                    Message::DelayHoursChanged(0)
                }
            })
            .width(FillPortion(1));

        let delay_minutes_text = text("Minutes:");

        let delay_minutes_input = text_input("Minutes", &self.delay_minutes.to_string())
            .on_input(|s| {
                if let Ok(value) = s.parse::<u64>() {
                    Message::DelayMinutesChanged(value.min(59))
                } else {
                    Message::DelayMinutesChanged(0)
                }
            })
            .width(FillPortion(1));

        let delay_seconds_text = text("Seconds:");

        let delay_seconds_input = text_input("Seconds", &self.delay_seconds.to_string())
            .on_input(|s| {
                if let Ok(value) = s.parse::<u64>() {
                    Message::DelaySecondsChanged(value.min(59))
                } else {
                    Message::DelaySecondsChanged(0)
                }
            })
            .width(FillPortion(1));

        let duration_text = text("Duration:");

        let duration_current_value_text = text(
            if self.duration_seconds == 0 && self.duration_minutes == 0 && self.duration_hours == 0
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
        );

        let duration_hours_text = text("Hours:");

        let duration_hours_input = text_input("Hours", &self.duration_hours.to_string())
            .on_input(|s| {
                if let Ok(value) = s.parse::<u64>() {
                    Message::DurationHoursChanged(value.min(23))
                } else {
                    Message::DurationHoursChanged(0)
                }
            })
            .width(FillPortion(1));

        let duration_minutes_text = text("Minutes:");

        let duration_minutes_input = text_input("Minutes", &self.duration_minutes.to_string())
            .on_input(|s| {
                if let Ok(value) = s.parse::<u64>() {
                    Message::DurationMinutesChanged(value.min(59))
                } else {
                    Message::DurationMinutesChanged(0)
                }
            })
            .width(FillPortion(1));

        let duration_seconds_text = text("Seconds:");

        let duration_seconds_input = text_input("Seconds", &self.duration_seconds.to_string())
            .on_input(|s| {
                if let Ok(value) = s.parse::<u64>() {
                    Message::DurationSecondsChanged(value.min(59))
                } else {
                    Message::DurationSecondsChanged(0)
                }
            })
            .width(FillPortion(1));

        let choose_mouse_button_text = text("Choose mouse button:");

        let choose_mouse_button_current_value_text =
            text(format!("{:?}", self.selected_mouse_button.lock().unwrap()));

        let left_button = button(text("Left"))
            .on_press(Message::SelectMouseButton(MouseButton::Left))
            .style(
                if *self.selected_mouse_button.lock().unwrap() == MouseButton::Left {
                    Button::Primary
                } else {
                    Button::Secondary
                },
            );

        let middle_button = button(text("Middle"))
            .on_press(Message::SelectMouseButton(MouseButton::Middle))
            .style(
                if *self.selected_mouse_button.lock().unwrap() == MouseButton::Middle {
                    Button::Primary
                } else {
                    Button::Secondary
                },
            );

        let right_button = button(text("Right"))
            .on_press(Message::SelectMouseButton(MouseButton::Right))
            .style(
                if *self.selected_mouse_button.lock().unwrap() == MouseButton::Right {
                    Button::Primary
                } else {
                    Button::Secondary
                },
            );

        let total_clicks = self.total_clicks.lock().unwrap();
        let total_clicks_text = text(format!("Total Clicks: {}", *total_clicks));
        let delay_timer_text = text(format!("Delay Timer: {}s", self.delay_timer));
        let time_running_text = text(format!("Time Running: {}s", self.time_running));
        let start_button = button(text("Start"));
        let stop_button = button(text("Stop"));

        let content = column![
            // The `Parameter Name` section
            row![
                row![
                    parameter_name_text.width(FillPortion(1)),
                    current_value_text.width(FillPortion(1))
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                input_text.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Theme` section
            row![
                row![
                    theme_text.width(FillPortion(1)),
                    theme_current_value_text.width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                pick_list.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Interval` section
            row![
                row![
                    interval_text.width(FillPortion(1)),
                    interval_current_value_text.width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                interval_slider.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Clicks` section
            row![
                row![
                    clicks_count_text.width(FillPortion(1)),
                    clicks_count_current_value_text.width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                clicks_count_slider.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Delay Before Start` section
            row![
                row![
                    delay_before_start_text.width(FillPortion(1)),
                    delay_before_start_current_value_text.width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                row![
                    delay_hours_text.width(FillPortion(1)),
                    delay_hours_input.width(FillPortion(2)),
                    delay_minutes_text.width(FillPortion(1)),
                    delay_minutes_input.width(FillPortion(2)),
                    delay_seconds_text.width(FillPortion(1)),
                    delay_seconds_input.width(FillPortion(2)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(2)),
            ],
            horizontal_rule(20),
            // The `Duration` section
            row![
                row![
                    duration_text.width(FillPortion(1)),
                    duration_current_value_text.width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                row![
                    duration_hours_text.width(FillPortion(1)),
                    duration_hours_input.width(FillPortion(2)),
                    duration_minutes_text.width(FillPortion(1)),
                    duration_minutes_input.width(FillPortion(2)),
                    duration_seconds_text.width(FillPortion(1)),
                    duration_seconds_input.width(FillPortion(2)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(2)),
            ],
            horizontal_rule(20),
            // The `Mouse Button` section
            row![
                row![
                    choose_mouse_button_text.width(FillPortion(1)),
                    choose_mouse_button_current_value_text.width(FillPortion(1)),
                ]
                .align_items(Center)
                .spacing(10)
                .width(FillPortion(1)),
                row![left_button, middle_button, right_button]
                    .width(FillPortion(2))
                    .align_items(Center)
                    .spacing(10),
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
            row![delay_timer_text, time_running_text, total_clicks_text]
                .align_items(Center)
                .spacing(10)
                .height(FillPortion(1)),
            horizontal_rule(20),
            row![
                start_button.on_press_maybe(if *self.is_running.lock().unwrap() {
                    None
                } else {
                    Some(Message::Start)
                }),
                stop_button.on_press_maybe(if *self.is_running.lock().unwrap() {
                    Some(Message::Stop)
                } else {
                    None
                }),
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

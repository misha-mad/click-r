use enigo::{Button as MouseButton, Direction::Click, Enigo, Mouse, Settings as EnigoSettings};
use iced::theme::{Button, Theme};
use iced::widget::{button, column, horizontal_rule, pick_list, row, slider, text};
use iced::Alignment::Center;
use iced::Length::FillPortion;
use iced::{executor, Application, Command, Element, Settings as IcedSettings, Subscription};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

struct AutoClicker {
    click_interval_slider_value: u8,
    click_thread: Option<thread::JoinHandle<()>>,
    clicks_count_slider_value: u8,
    delay_before_start_value: u8,
    duration_value: u8,
    elapsed_time: u64,
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
    DelayBeforeStartSliderChanged(u8),
    DurationSliderChanged(u8),
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
                self.elapsed_time = 0;
                self.total_clicks = Arc::new(Mutex::new(0));
                println!("click_interval_secs {}", self.click_interval_slider_value);
                let interval = self.click_interval_slider_value;
                let clicks_count = self.clicks_count_slider_value;
                let duration = self.duration_value;
                let (tx, rx) = mpsc::channel();
                let total_clicks = Arc::clone(&self.total_clicks);
                let selected_mouse_button = Arc::clone(&self.selected_mouse_button);
                let is_running = Arc::clone(&self.is_running);
                let delay_before_start = self.delay_before_start_value;
                let end_time = std::time::Instant::now()
                    + Duration::from_secs(duration as u64)
                    + Duration::from_secs(delay_before_start as u64);
                self.stop_sender = Some(tx);

                let handle = thread::spawn(move || {
                    println!("interval {}", interval);
                    thread::park_timeout(Duration::from_secs(delay_before_start as u64));
                    let mut enigo = Enigo::new(&EnigoSettings::default()).unwrap();
                    let button = selected_mouse_button.lock().unwrap().clone();

                    loop {
                        if rx.try_recv().is_ok() || std::time::Instant::now() >= end_time {
                            *is_running.lock().unwrap() = false;
                            println!("Stop signal received or duration elapsed");
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

                    if self.ticks_count > self.delay_before_start_value as u64 {
                        self.elapsed_time += 1;
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
            Message::DelayBeforeStartSliderChanged(new_delay) => {
                println!("Setting delay_before_start_value to {} seconds", new_delay);
                self.delay_before_start_value = new_delay;
                Command::none()
            }
            Message::DurationSliderChanged(new_duration) => {
                println!("Setting duration_value to {} seconds", new_duration);
                self.duration_value = new_duration;
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let theme_text = text("Theme:");
        let pick_list = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);

        let delay_before_start_text = text(format!(
            "Delay before start: {}s",
            self.delay_before_start_value
        ));

        let delay_before_start_slider = slider(
            0..=100,
            self.delay_before_start_value,
            Message::DelayBeforeStartSliderChanged,
        );

        let interval_text = text(format!("Interval: {}s", self.click_interval_slider_value));

        let interval_slider = slider(
            1..=100,
            self.click_interval_slider_value,
            Message::IntervalSliderChanged,
        );

        let clicks_count_text = text(format!("Clicks: {}", self.clicks_count_slider_value));

        let clicks_count_slider = slider(
            1..=100,
            self.clicks_count_slider_value,
            Message::ClickCountSliderChanged,
        );

        let duration_text = text(format!("Duration: {}s", self.duration_value));
        let duration_slider = slider(1..=100, self.duration_value, Message::DurationSliderChanged);
        let choose_mouse_button_text = text("Choose mouse button:");

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
        let timer_text = text(format!("Timer: {}s", self.elapsed_time));
        let start_button = button(text("Start"));
        let stop_button = button(text("Stop"));

        let content = column![
            // The `Theme` section
            row![
                theme_text.width(FillPortion(1)),
                pick_list.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Delay Before Start` section
            row![
                delay_before_start_text.width(FillPortion(1)),
                delay_before_start_slider.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Interval` section
            row![
                interval_text.width(FillPortion(1)),
                interval_slider.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Clicks` section
            row![
                clicks_count_text.width(FillPortion(1)),
                clicks_count_slider.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Duration` section
            row![
                duration_text.width(FillPortion(1)),
                duration_slider.width(FillPortion(2))
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Mouse Button` section
            row![
                choose_mouse_button_text.width(FillPortion(1)),
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
            row![timer_text, total_clicks_text]
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
            delay_before_start_value: 0,
            duration_value: 10,
            elapsed_time: 0,
            is_running: Arc::new(Mutex::new(false)),
            selected_mouse_button: Arc::new(Mutex::new(MouseButton::Left)),
            stop_sender: None,
            theme: Theme::Light,
            ticks_count: 0,
            total_clicks: Arc::new(Mutex::new(0)),
        }
    }
}

fn main() -> iced::Result {
    AutoClicker::run(IcedSettings::default())
}

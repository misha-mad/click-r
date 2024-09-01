use enigo::{Button as MouseButton, Direction::Click, Enigo, Mouse, Settings as EnigoSettings};
use iced::theme::Theme;
use iced::widget::{column, horizontal_rule, pick_list, row, slider, text, Button, Text};
use iced::Alignment::Center;
use iced::Length::FillPortion;
use iced::{executor, Application, Command, Element, Settings as IcedSettings, Subscription};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

struct AutoClicker {
    theme: Theme,
    click_interval_slider_value: u8,
    is_running: bool,
    elapsed_time: u64,
    click_thread: Option<thread::JoinHandle<()>>,
    stop_sender: Option<mpsc::Sender<()>>,
    clicks_count_slider_value: u8,
    selected_mouse_button: MouseButton,
}

#[derive(Debug, Clone)]
enum Message {
    ThemeChanged(Theme),
    IntervalSliderChanged(u8),
    ClickCountSliderChanged(u8),
    Tick,
    Start,
    Stop,
    SelectMouseButton(MouseButton),
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
                self.selected_mouse_button = button;
                Command::none()
            }
            Message::Start => {
                println!("Starting the auto clicker");
                self.is_running = true;
                self.elapsed_time = 0;
                println!("click_interval_secs {}", self.click_interval_slider_value);
                let interval = self.click_interval_slider_value;
                let clicks_count = self.clicks_count_slider_value;
                let (tx, rx) = mpsc::channel();
                self.stop_sender = Some(tx);

                let handle = thread::spawn(move || {
                    println!("interval {}", interval);
                    let mut enigo = Enigo::new(&EnigoSettings::default()).unwrap();

                    loop {
                        if rx.try_recv().is_ok() {
                            println!("Stop signal received");
                            break;
                        }

                        println!("Clicking every {} seconds", interval);

                        for _ in 0..clicks_count {
                            println!("Clicking");
                            enigo.button(MouseButton::Left, Click).unwrap();
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

                self.is_running = false;
                Command::none()
            }
            Message::Tick => {
                if self.is_running {
                    self.elapsed_time += 1;
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
        }
    }

    fn view(&self) -> Element<Self::Message> {
        let theme_text = text("Theme:");
        let pick_list = pick_list(Theme::ALL, Some(&self.theme), Message::ThemeChanged);
        let interval_text = Text::new(format!("Interval: {}s", self.click_interval_slider_value));

        let interval_slider = slider(
            1..=100,
            self.click_interval_slider_value,
            Message::IntervalSliderChanged,
        );

        let clicks_count_text = Text::new(format!("Clicks: {}", self.clicks_count_slider_value));

        let clicks_count_slider = slider(
            1..=100,
            self.clicks_count_slider_value,
            Message::ClickCountSliderChanged,
        );

        let choose_mouse_button_text = Text::new("Choose mouse button:");

        let left_button =
            Button::new(Text::new("Left")).on_press(Message::SelectMouseButton(MouseButton::Left));

        let right_button = Button::new(Text::new("Right"))
            .on_press(Message::SelectMouseButton(MouseButton::Right));

        let middle_button = Button::new(Text::new("Middle"))
            .on_press(Message::SelectMouseButton(MouseButton::Middle));

        let timer_text = Text::new(format!("Timer: {}s", self.elapsed_time));
        let start_button = Button::new(Text::new("Start")).on_press(Message::Start);
        let stop_button = Button::new(Text::new("Stop")).on_press(Message::Stop);

        let content = column![
            // The `Theme` section
            row![
                theme_text.width(FillPortion(1)),
                pick_list.width(FillPortion(2))
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
            // The `Mouse Button` section
            row![
                choose_mouse_button_text.width(FillPortion(1)),
                row![left_button, right_button, middle_button]
                    .width(FillPortion(2))
                    .align_items(Center)
                    .spacing(10),
            ]
            .align_items(Center)
            .spacing(10),
            horizontal_rule(20),
            // The `Timer` and `Start`/`Stop` buttons section
            timer_text,
            row![
                start_button.on_press_maybe(if self.is_running {
                    None
                } else {
                    Some(Message::Start)
                }),
                stop_button.on_press_maybe(if self.is_running {
                    Some(Message::Stop)
                } else {
                    None
                }),
            ]
            .align_items(Center)
            .spacing(10),
        ]
        .spacing(10)
        .padding(20)
        .align_items(Center);

        content.into()
    }

    fn theme(&self) -> Self::Theme {
        self.theme.clone()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        if self.is_running {
            iced::time::every(Duration::from_millis(1000)).map(|_| Message::Tick)
        } else {
            Subscription::none()
        }
    }
}

impl Default for AutoClicker {
    fn default() -> Self {
        Self {
            click_thread: None,
            elapsed_time: 0,
            is_running: false,
            click_interval_slider_value: 1,
            stop_sender: None,
            theme: Theme::Light,
            clicks_count_slider_value: 1,
            selected_mouse_button: MouseButton::Left,
        }
    }
}

fn main() -> iced::Result {
    AutoClicker::run(IcedSettings::default())
}

use enigo::Button;
use iced::Theme;

#[derive(Debug, Clone)]
pub enum Message {
    ClickCountSliderChanged(u8),
    DelayHoursChanged(u64),
    DelayMinutesChanged(u64),
    DelaySecondsChanged(u64),
    DurationHoursChanged(u64),
    DurationMinutesChanged(u64),
    DurationSecondsChanged(u64),
    IntervalSliderChanged(u8),
    ResetToDefaults,
    SaveSettings,
    SelectMouseButton(Button),
    Start,
    Stop,
    ThemeChanged(Theme),
    Tick,
}

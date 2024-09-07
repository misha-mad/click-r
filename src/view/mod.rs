mod parameter_name;
mod theme;
mod interval;
mod clicks_count;
mod delay_before_start;
mod duration;
mod mouse_button;
mod footer;
mod page;

use crate::AutoClicker;
use iced::Element;
use crate::message::Message;

pub fn view_handler(auto_clicker: &AutoClicker) -> Element<Message> {
    page::view(auto_clicker)
}

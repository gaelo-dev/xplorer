// to do!

use crate::bluetooth;
use iced::{Element, widget};

#[derive(Debug, Clone)]
pub enum Message {

}

#[derive(Debug, Clone)]
pub struct Disconnected {
    pub central: bluetooth::Central,
    pub peripherals: bluetooth::Peripherals,
}

impl Disconnected {
    pub fn view(&self) -> Element<Message> {
        widget::text("...").into()
    }
}

// ...

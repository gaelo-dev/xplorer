use crate::bluetooth;
use iced::{
    widget::{ button, column, row, text }, 
    Element,
};

pub enum Action {
    Connect(bluetooth::BDAddr),
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect(bluetooth::BDAddr),
}

#[derive(Debug, Clone)]
pub struct Disconnected {
    peripherals: bluetooth::Peripherals,
}

impl Disconnected {
    pub fn new(peripherals: bluetooth::Peripherals) -> Self {
        Self {
            peripherals,
        }
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::Connect(addr) => Action::Connect(addr),
        }
    } 

    pub fn view(&self) -> Element<Message> {
        column(self.peripherals.iter().map(|(_id, peripheral)| {
            let name = peripheral.local_name.as_deref().unwrap_or("(peripheral name unknown)");
            row![
                text(format!("{name} ({})", peripheral.address)),
                button("Connect")
                    .on_press(Message::Connect(peripheral.address))
            ].into()
        })).into()
    }
}

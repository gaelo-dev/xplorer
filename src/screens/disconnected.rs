use std::sync::Arc;

use crate::bluetooth;
use iced::{
    futures::{channel::mpsc, SinkExt, lock::Mutex}, 
    widget::{ 
        button, column, row, text 
    }, 
    Element, Subscription, Task
};

pub enum Action {
    Run(Task<Message>),
    Wait,
}

#[derive(Debug, Clone)]
pub enum Message {
    Connect(bluetooth::BDAddr),
    Wait,
}

#[derive(Debug, Clone)]
pub struct Disconnected {
    peripherals: bluetooth::Peripherals,
    sender: Arc<Mutex<mpsc::Sender<bluetooth::BDAddr>>>,
}

impl Disconnected {
    pub fn new(peripherals: bluetooth::Peripherals, tx: mpsc::Sender<bluetooth::BDAddr>) -> Self {
        Self {
            peripherals,
            sender: Arc::new(Mutex::new(tx)),
        }
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::Connect(addr) => {
                let sender = Arc::clone(&self.sender); 
                
                Action::Run(
                    Task::perform(
                        async move {
                            let mut sender = sender.lock().await;
                            let _ = sender.send(addr).await;
                        },
                        |_| Message::Wait,
                    )
                )
            },
            Message::Wait => Action::Wait,
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

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::none()
    }
}

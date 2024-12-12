// to do!
use std::sync::Arc;

use iced::{
    futures::{channel::mpsc, SinkExt, lock::Mutex}, 
    keyboard, widget, 
    Element, Subscription, Task 
};

use crate::bluetooth::{Command, car};

pub enum Action {
    Run(Task<Message>),
    None,
}

#[derive(Debug, Clone)]
pub enum Message {
    Command(Command),
    Ok,
}

#[derive(Debug, Clone)]
pub struct Connected {
    command: Command,
    sender: Arc<Mutex<mpsc::Sender<Command>>>,
    // speed: u8,
}

impl Connected {
    pub fn new(tx: mpsc::Sender<Command>) -> Self {
        Self { 
            command: Command::default(),
            sender: Arc::new(Mutex::new(tx)),
            // speed: 100,
        }
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::Command(cmd) => {
                if self.command != cmd {
                    self.command = cmd;
                    let sender = Arc::clone(&self.sender); 

                    Action::Run(
                        Task::perform(
                            async move {
                                let mut sender = sender.lock().await;
                                let _ = sender.send(cmd).await;
                            },
                            |_| Message::Ok,
                        )
                    )
                } else {
                    Action::None
                }
            },
            Message::Ok => Action::None,
        }
    }

    pub fn view(&self) -> Element<Message> {
        widget::text("conectado").into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            keyboard::on_key_press(|key, modifiers| {
                match key.as_ref() {
                    keyboard::Key::Character("w") => Some(car::forward()),
                    keyboard::Key::Character("s") => Some(car::backward()),
                    keyboard::Key::Character("a") => Some(car::leftward()),
                    keyboard::Key::Character("d") => Some(car::leftward()),
                    _ => None,
                }
                .map(|cmd| if modifiers.shift() { cmd + car::speed(255) } else { cmd })
                .map(Message::Command)
            }),

            keyboard::on_key_release(|key, _modifiers| {
                match key.as_ref() {
                    keyboard::Key::Character("w" | "s" | "a" | "d") => Some(car::speed(0)),
                    _ => None,
                }
                .map(Message::Command)
            }),
        ])
    }
}

// ...

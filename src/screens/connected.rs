// to do!
// use crate::bluetooth;
use iced::{
    futures::{channel::mpsc, SinkExt}, 
    keyboard, widget, 
    Element, Subscription, Task 
};

use crate::bluetooth::Command;

pub enum Action {
    Run(Task<Message>),
    None,
}

#[derive(Debug, Clone)]
pub enum Message {
    Forward,
    Backward,
    Rightward,
    Leftward,
    Stop
}

#[derive(Debug, Clone)]
pub struct Connected {
    // command: Command,
    sender: mpsc::Sender<Command>
}

impl Connected {
    pub fn new(sender: mpsc::Sender<Command>) -> Self {
        Self { 
            // command: Command::none(),
            sender,
        }
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::Forward => println!("avanzando"),
            Message::Backward => println!("retrocediendo"),
            Message::Leftward => println!("a la izquierda"),
            Message::Rightward => println!("a la derecha"),
            Message::Stop => println!("stop!"),
        }

        Action::None
    }

    pub fn view(&self) -> Element<Message> {
        widget::text("conectado").into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch([
            keyboard::on_key_press(|key, _modifiers| {
                match key.as_ref() {
                    keyboard::Key::Character("w") => Some(Message::Forward),
                    keyboard::Key::Character("s") => Some(Message::Backward),
                    keyboard::Key::Character("a") => Some(Message::Leftward),
                    keyboard::Key::Character("d") => Some(Message::Rightward),
                    _ => None,
                }
            }),

            keyboard::on_key_release(|key, _modifiers| {
                match key.as_ref() {
                    keyboard::Key::Character("w") | keyboard::Key::Character("s") | 
                    keyboard::Key::Character("a") | keyboard::Key::Character("d") => Some(Message::Stop),
                    _ => None,
                }
            }),
        ])
    }
}

// ...

// to do!
// use crate::bluetooth;
use iced::{Element, Task, widget};

pub enum Action {
    Run(Task<Message>),
    None,
}

#[derive(Debug, Clone)]
pub enum Message {

}

#[derive(Debug, Clone)]
pub struct Connected;

impl Connected {
    pub fn update(&mut self, _msg: Message) -> Action {
        todo!()
    }

    pub fn view(&self) -> Element<Message> {
        widget::text("conectado").into()
    }
}

// ...

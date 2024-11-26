// to do!

mod loading {
    use iced::{Element, widget};

    #[derive(Debug, Clone)]
    pub struct Loading;
    
    impl Loading {
        pub fn view<Msg>(&self) -> Element<Msg> {
            widget::text("...").into()
        }
    }

    // ...
}

mod connected;
mod disconnected;

use crate::bluetooth::ConnectionState;
use iced::{Element, Task};

pub enum Action {
    Run(Task<Message>),
    Changed(Screen),
    None,
}

#[derive(Debug, Clone)]
pub enum Message {
    Connected(connected::Message),
    Disconnected(disconnected::Message),
    Loading,
}

#[derive(Debug, Clone)]
pub enum Screen {
    Connected(connected::Connected),
    Disconnected(disconnected::Disconnected),
    Loading(loading::Loading),
}

impl Screen {
    pub fn loading() -> Self {
        Self::Loading(loading::Loading)
    }

    pub fn update(&mut self, msg: Message) -> Action {
        match msg {
            Message::Connected(msg) => {
                if let Screen::Connected(screen) = self {
                    todo!()
                } else {
                    Action::None
                }
            },
            Message::Disconnected(msg) => { 
                if let Screen::Disconnected(screen) = self {
                    todo!()
                } else {
                    Action::None

                }
            },
            _ => Action::None
        }
    }

    pub fn view(&self) -> Element<Message> {
        todo!()
    }
}

impl From<ConnectionState> for Screen {
    fn from(value: ConnectionState) -> Self {
        match value {
            ConnectionState::Connected { central, xplorer } => {
                Self::Connected(connected::Connected { central, xplorer })
            },
            ConnectionState::Disconnected { central, peripherals } => {
                Self::Disconnected(disconnected::Disconnected { central, peripherals })
            },
            ConnectionState::Loading => Self::loading(),
        }
    }
}

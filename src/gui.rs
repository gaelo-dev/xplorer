use crate::bluetooth;
use iced::{
    widget::{button, column, row, text, text_input}, 
    Element, Task
};

async fn connect(peripheral_name: String) -> State {
    let central = bluetooth::Central::new().await.unwrap();
    let peripherals_properties = central.scan().await.unwrap();

    for (id, properties) in peripherals_properties {
        let name = properties.local_name.unwrap_or("(peripheral name unknown)".to_string());
        println!("Peripheral: {name} ({})({})", properties.address, id);

        if name.contains(&peripheral_name) {
            let xplorer = central.connect(&id).await.unwrap();
            return State::Connected { central, xplorer }
            // xplorer.send("on13").await?;
        }
    }

    State::NotFoundPeripheral(central)
}

pub struct App {
    // central: Option<bluetooth::Central>,
    // xplorer: Option<bluetooth::Xplorer>,
    msg: String,
    state: State,
}

#[derive(Debug, Clone)]
pub enum State {
    Disconnected,
    NotFoundPeripheral(bluetooth::Central),
    // Connected(bluetooth::Xplorer),
    Connected {
        central: bluetooth::Central,
        xplorer: bluetooth::Xplorer,
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Connect,
    StateChanged(State),
    Ok,
    Send,
    On,
    Off,
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (
            Self {
                // central: None,
                msg: String::new(),
                state: State::Disconnected,
            },
            Task::none()
            // Task::perform(bluetooth::Central::new(), |central| Message::Central(central.unwrap()))
        )
    }


    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::InputChanged(s) => {
                self.msg = s;
                Task::none()
            },
            Message::Connect => {
                let msg = self.msg.clone();
                self.msg.clear();

                Task::perform(connect(msg), Message::StateChanged)
            },
            Message::StateChanged(state) => {
                self.state = state;
                Task::none()
            }
            Message::Send => {
                if let State::Connected { xplorer, .. } = &self.state{
                    let msg = self.msg.clone();
                    let xplorer = xplorer.clone();
                    self.msg.clear();

                    return Task::future(async move {
                        xplorer.send(msg).await.unwrap();
                        Message::Ok
                    })
                }

                Task::none()
            }
            Message::On => {
                self.msg = "on13".to_string();
                Task::done(Message::Send)
            }, 
            Message::Off => {
                self.msg = "off13".to_string();
                Task::done(Message::Send)
            }
            _ => Task::none()
        }
    }

    pub fn view(&self) -> Element<Message> {
        match &self.state {
            State::Disconnected => {
                column![
                    text_input("Peripheral name", &self.msg)
                        .on_input(Message::InputChanged)
                        .on_submit(Message::Connect),
                    button("Connect")
                        .on_press(Message::Connect)
                ].into()
            }
            State::NotFoundPeripheral(_) => {
                text("No se encontro el dispositivo").into()
            },
            State::Connected { central: _, xplorer: _ } => {
                column![
                    row![
                        text_input("Message", &self.msg)
                            .on_input(Message::InputChanged)
                            .on_submit(Message::Send),
                        button("Send")
                            .on_press(Message::Send), 
                    ],
                    button("On")
                        .on_press(Message::On),
                    button("Off")
                        .on_press(Message::Off)
                ].into()
            }
        }
    }
}

use crate::{bluetooth::{self, ConnectionState}, config::Config};
use iced::{
    widget::{button, column, row, text, text_input}, 
    Element, Task
};

pub struct App {
    msg: String,
    cfg: Config,
    state: ConnectionState,
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Connect(bluetooth::BDAddr),
    StateChanged(ConnectionState),
    Ok,
    Send,
    On,
    Off,
}

impl App {
    pub fn new(cfg: Config) -> Self {
        Self {
            cfg,
            msg: String::new(),
            state: ConnectionState::Loading,
        }
    }

    pub fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::InputChanged(s) => {
                self.msg = s;
                Task::none()
            },
            Message::Connect(ip) => {
                let state = self.state.clone();
                self.state = ConnectionState::Loading;
                self.cfg.addr = Some(ip);

                Task::perform(state.reconnect(ip), |state| Message::StateChanged(state.unwrap()))
            },
            Message::StateChanged(state) => {
                self.state = state;
                Task::none()
            }
            Message::Send => {
                if let ConnectionState::Connected { xplorer, .. } = &self.state{
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
            ConnectionState::Loading => text("...").into(),
            ConnectionState::Disconnected { peripherals, .. } => {
                iced::widget::column(peripherals.into_iter().map(|(_id, peripheral)| {
                    let name = peripheral.local_name.as_deref().unwrap_or("(peripheral name unknown)");
                    row![
                        text(format!("{name} ({})", peripheral.address)),
                        button("Connect")
                            .on_press(Message::Connect(peripheral.address))
                    ].into()
                })).into()
            }
            ConnectionState::Connected { central: _, xplorer: _ } => {
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

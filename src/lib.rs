pub mod bluetooth;
pub mod config;
pub mod screens;

use bluetooth::ConnectionState;
use screens::{Screen, connected, disconnected};
use config::Config;

use confy::ConfyError;
use iced::{Element, Task};

#[derive(Debug, Clone)]
pub enum Message {
    ChangedScreen(Screen),
    ChangedState(ConnectionState),
    Connected(connected::Message),
    Disconnected(disconnected::Message),
    Loading,
}

pub struct App {
    cfg: Config,
    screen: Screen,
    state: ConnectionState,
}

impl App {
    pub fn new() -> Result<Self, ConfyError> {
        let state = ConnectionState::Loading;
        
        Ok(Self { 
            cfg: Config::load()?, 
            screen: Screen::create(&state),
            state,
        })
    }

    pub fn run(self) -> Result<(), iced::Error> {
        let ip = self.cfg.addr;
        
        iced::application("Xplorer app", Self::update, Self::view)
           .run_with(move || {
                (
                    self,
                    Task::perform(bluetooth::start(ip), |state| {
                        Message::ChangedState(state.unwrap())
                    })
                )
           })?;
    
        Ok(())
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::ChangedScreen(screen) => {
                self.screen = screen;
                Task::none()
            },
            Message::ChangedState(state) => {
                self.state = state;

                let screen = Screen::create(&self.state);
                Task::done(Message::ChangedScreen(screen))
            },
            Message::Connected(msg) => {
                if let Screen::Connected(screen) = &mut self.screen {
                    let action = screen.update(msg);
                    // ...
                    
                    todo!()
                } else {
                    Task::none()
                }
            },
            Message::Disconnected(msg) => {
                if let Screen::Disconnected(screen) = &mut self.screen {
                    let action = screen.update(msg);
                    
                    match action {
                        disconnected::Action::Connect(addr) => {
                            self.cfg.addr = Some(addr);
                            let state = self.state.clone();

                            Task::done(Message::ChangedState(ConnectionState::Loading))
                                .chain(Task::perform(
                                    state.reconnect(addr),
                                    |state| Message::ChangedState(state.unwrap())
                                ))                      
                        },
                    }
                } else {
                    Task::none()
                }
            },
            Message::Loading => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Connected(screen) => screen.view().map(Message::Connected),
            Screen::Disconnected(screen) => screen.view().map(Message::Disconnected),
            Screen::Loading(screen) => screen.view()
        }
    }
}

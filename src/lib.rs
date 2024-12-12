pub mod bluetooth;
pub mod config;
mod screens;
mod subscription;

use screens::{connected, disconnected, loading, Screen};
use config::Config;
use subscription::Event;

use confy::ConfyError;
use iced::{
    futures::SinkExt, 
    Element, Subscription, Task,
};

#[derive(Debug, Clone)]
enum Message {
    BluetoothEvent(subscription::Event),
    ChangedScreen(Screen),
    Connected(connected::Message),
    Disconnected(disconnected::Message),
    Ok,
}

pub struct App {
    cfg: Config,
    screen: Screen,
}

impl App {
    pub fn new() -> Result<Self, ConfyError> {
        Ok(Self { 
            cfg: Config::load()?, 
            screen: loading::Loading.into(),
        })
    }

    pub fn run(self) -> Result<(), iced::Error> {
        iced::application("Xplorer app", Self::update, Self::view)
           .subscription(Self::subscribe)
           .run_with(move || {
                (
                    self,
                    // Task::perform(bluetooth::start(ip), |state| {
                    //     Message::ChangedState(state.unwrap())
                    //})
                    Task::none()
                )
           })?;
    
        Ok(())
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::BluetoothEvent(event) => {
                match event {
                    Event::Connected { addr, sender } => {
                        self.cfg.addr = Some(addr);

                        let screen = connected::Connected::new(sender).into();
                        Task::done(Message::ChangedScreen(screen))
                    },
                    Event::CommandReceived(_cmd) => {
                        todo!()
                    },
                    Event::Disconnected { peripherals, mut sender } => {
                        match &self.cfg.addr {
                            Some(addr) => {
                                let ip = addr.clone();
                                
                                Task::perform(
                                    async move {
                                        let _ = sender.send(ip).await;
                                    },
                                    |_| Message::Ok,
                                )
                            },
                            None => {
                                let screen = disconnected::Disconnected::new(peripherals, sender).into();
                                Task::done(Message::ChangedScreen(screen))
                            },
                        }
                    },
                    Event::Err(err) => {
                        eprintln!("{err}");
                        Task::none()
                    },
                }
            }
            Message::ChangedScreen(screen) => {
                self.screen = screen;
                Task::none()
            },
            Message::Connected(msg) => {
                if let Screen::Connected(screen) = &mut self.screen {
                    let action = screen.update(msg);
                    
                    match action {
                        connected::Action::Run(task) => task.map(Message::Connected),
                        connected::Action::None => Task::none(),
                    }                    
                } else {
                    Task::none()
                }
            },
            Message::Disconnected(msg) => {
                if let Screen::Disconnected(screen) = &mut self.screen {
                    let action = screen.update(msg);

                    match action {
                        disconnected::Action::Run(task) => task.map(Message::Disconnected),
                        disconnected::Action::Wait => {
                            let screen = loading::Loading.into();
                            Task::done(Message::ChangedScreen(screen))
                        }
                    }
                } else {
                    Task::none()
                }
            },
            Message::Ok => Task::none(),
        }
    }

    fn view(&self) -> Element<Message> {
        match &self.screen {
            Screen::Connected(screen) => screen.view().map(Message::Connected),
            Screen::Disconnected(screen) => screen.view().map(Message::Disconnected),
            Screen::Loading(screen) => screen.view()
        }
    }

    fn subscribe(&self) -> Subscription<Message> {
        Subscription::batch([
            Subscription::run(subscription::bluetooth_connection).map(Message::BluetoothEvent),
            match &self.screen {
                Screen::Connected(screen) => screen.subscription().map(Message::Connected),
                Screen::Disconnected(screen) => screen.subscription().map(Message::Disconnected),
                _ => Subscription::none(),
            }
        ])
    }
}

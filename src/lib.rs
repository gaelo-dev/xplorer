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
    BluetoothEvent(subscription::EventResult),
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
        log::info!("Running app with this config: {:?}", self.cfg);

        iced::application("Xplorer app", Self::update, Self::view)
           .subscription(Self::subscribe)
           .run_with(move || {
                (
                    self,
                    Task::none()
                )
           })?;

        Ok(())
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        log::debug!("New message generated: {msg:#?}");

        match msg {
            Message::BluetoothEvent(event) => {                
                match event {
                    Ok(Event::Connected { addr, sender }) => {
                        let screen = connected::Connected::new(sender).into();
                        self.cfg.addr = Some(addr);
                        
                        Task::done(Message::ChangedScreen(screen))
                    },
                    Ok(Event::CommandReceived(cmd)) => Task::done(Message::Connected(connected::Message::CommandReceived(cmd))),
                    Ok(Event::Disconnected { peripherals, mut sender }) => {
                        match &self.cfg.addr {
                            Some(addr) => {
                                let addr = *addr;
                                Task::perform(
                                    async move {
                                        let _ = sender.send(addr).await;
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
                    Err(err) => {
                        log::error!("Bluetooth Error -> {err}");
                        return Task::none();
                    },
                }
            }
            Message::ChangedScreen(screen) => {
                log::debug!("New screen: {screen:#?}");
                self.screen = screen;

                Task::none()
            },
            Message::Connected(msg) => {
                let Screen::Connected(screen) = &mut self.screen else {
                    return Task::none();
                };

                let action = screen.update(msg);

                match action {
                    connected::Action::Run(task) => task.map(Message::Connected),
                    connected::Action::None => Task::none(),
                }                    
            },
            Message::Disconnected(msg) => {
                let Screen::Disconnected(screen) = &mut self.screen else {
                    return Task::none();
                };

                let action = screen.update(msg);

                match action {
                    disconnected::Action::Run(task) => task.map(Message::Disconnected),
                    disconnected::Action::Wait => {
                        let screen = loading::Loading.into();
                        Task::done(Message::ChangedScreen(screen))
                    }
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

pub mod bluetooth;
pub mod config;
pub mod screens;
pub mod subscription;

use screens::{connected, disconnected, loading, Screen};
use config::Config;
use subscription::{Event, Input};

use std::sync::Arc;
use confy::ConfyError;
use iced::{
    futures::{channel::mpsc, SinkExt, lock::Mutex}, 
    Element, Subscription, Task
};

#[derive(Debug, Clone)]
pub enum Message {
    BluetoothEvent(subscription::Event),
    ChangedScreen(Screen),
    Connected(connected::Message),
    Disconnected(disconnected::Message),
    Ok,
}

pub struct App {
    cfg: Config,
    screen: Screen,
    sender: Option<Arc<Mutex<mpsc::Sender<Input>>>>,
}

impl App {
    pub fn new() -> Result<Self, ConfyError> {
        Ok(Self { 
            cfg: Config::load()?, 
            screen: loading::Loading.into(),
            sender: None,
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
                    Event::Connected => {
                        let screen = connected::Connected::new().into();
                        Task::done(Message::ChangedScreen(screen))
                    },
                    Event::CommandReceived(cmd) => {
                        todo!()
                    },
                    Event::Disconnected { peripherals } => {
                        let screen = disconnected::Disconnected::new(peripherals).into();
                        Task::done(Message::ChangedScreen(screen))
                    },
                    Event::Err(err) => {
                        eprint!("{err}");
                        Task::none()
                    },
                    Event::Ready(sender) => {
                        self.sender = Some(Arc::new(Mutex::new(sender)));
                        Task::none()
                    }

                }
            }
            Message::ChangedScreen(screen) => {
                self.screen = screen;
                Task::none()
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
                            let sender = Arc::clone(&self.sender.as_ref().unwrap()); 
                            
                            Task::perform(
                                async move {
                                    let mut sender = sender.lock().await;
                                    let _ = sender.send(Input::Connect(addr)).await;
                                },
                                |_| Message::Ok,
                            )
                        },
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
        Subscription::run(subscription::connection).map(Message::BluetoothEvent)
    }
}

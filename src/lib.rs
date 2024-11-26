pub mod bluetooth;
pub mod config;
pub mod screens;

// use bluetooth::ConnectionState;
use screens::{Screen, Message as ScreenMessage};
use config::Config;

use confy::ConfyError;
use iced::{
    // widget::{button, column, row, text, text_input}, 
    Element, Task
};

#[derive(Debug, Clone)]
pub enum Message {
    Screen(ScreenMessage),
    ChangedScreen(Screen),
}

pub struct App {
    // msg: String,
    
    cfg: Config,
    screen: Screen,

    // state: ConnectionState,
}

impl App {
    pub fn new() -> Result<Self, ConfyError> {
        Ok(Self { cfg: Config::load()?, screen: Screen::loading() })
    }

    pub fn run(self) -> Result<(), iced::Error> {
        let ip = self.cfg.addr;
        
        iced::application("Xplorer app", Self::update, Self::view)
           .run_with(move || {
                (
                    self,
                    iced::Task::perform(bluetooth::start(ip), |state| {
                        Message::ChangedScreen(state.unwrap().into())
                    })
                )
           })?;
    
        Ok(())
    }

    fn update(&mut self, msg: Message) -> Task<Message> {
        match msg {
            Message::Screen(msg) => {
                let action = self.screen.update(msg);

                // execute action ...
                todo!()
            },
            Message::ChangedScreen(screen) => {
                self.screen = screen;
                Task::none()
            },
        }
    }

    fn view(&self) -> Element<Message> {
        self.screen.view().map(Message::Screen)
    }
}

pub mod bluetooth;
pub mod config;
pub mod gui;

use std::error::Error;

pub fn run() -> Result<(), Box<dyn Error>> {
    let cfg = config::Config::load()?;
    let ip = cfg.addr;

    iced::application("Xplorer app", gui::App::update, gui::App::view)
       .run_with(move || {
            (
                gui::App::new(cfg),
                iced::Task::perform(bluetooth::start(ip), |state| gui::Message::StateChanged(state.unwrap()))
            )
       })?;

    Ok(())
}

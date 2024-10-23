pub mod bluetooth;
pub mod config;
pub mod gui;

pub fn run() -> iced::Result {
    let cfg = config::Config::load().unwrap();
    let ip = cfg.ip.clone();

    iced::application("Xplorer app", gui::App::update, gui::App::view)
       .run_with(move || {
            (
                gui::App::new(cfg),
                iced::Task::perform(bluetooth::start(ip), |state| gui::Message::StateChanged(state.unwrap()))
            )
       })
}

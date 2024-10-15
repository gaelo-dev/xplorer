pub mod bluetooth;
pub mod gui;

use gui::App;

pub fn run() -> iced::Result {
    iced::application("Xplorer app", App::update, App::view)
        .run_with(App::new)
}

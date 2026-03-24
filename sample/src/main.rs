mod button_style;
mod state;
mod task;
mod test_msg;
mod update;
mod view;

pub use button_style::*;
pub use state::{AppMessage, AppState};
pub use task::*;
pub use test_msg::*;
pub use update::update;
pub use view::view;

use iced::{Size, Task, application, window};

fn init() -> (AppState, Task<AppMessage>) {
    let app_state = AppState::new();
    (app_state, Task::none())
}

fn main() {
    application(init, update, view)
        .title("Bristo")
        .position(window::Position::Centered)
        .window(window::Settings {
            size: Size::new(600.0, 350.0),
            min_size: Some(Size::new(600.0, 350.0)),
            resizable: true,
            decorations: true,
            ..Default::default()
        })
        .run()
        .expect("Failed to run application");
}

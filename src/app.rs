pub mod controller;
pub mod ui;

use crate::app::ui::{build_ui, build_menu};
use druid::WindowDesc;
use druid::{AppLauncher, Data, Lens};

#[derive(Clone, Data, Lens)]
pub struct AppState {
    pub content: String,
    current_filepath: Option<String>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            content: String::new(),
            current_filepath: None,
        }
    }
}

pub fn create_app(initial_state: AppState) {
    let main_window = WindowDesc::new(build_ui())
        .title("Rustpad")
        .menu(build_menu)
        .window_size((600.0, 400.0))
        .resizable(true);

    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch");
}

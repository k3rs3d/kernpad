pub mod controller;
pub mod ui;

use crate::app::ui::{build_ui, build_menu};
use druid::WindowDesc;
use druid::{AppLauncher, Data, Lens};
use std::sync::Arc;

#[derive(Clone, Lens)]
pub struct AppState {
    pub content: String,
    pub current_filepath: Option<String>,
    undo_stack: Arc<Vec<String>>,
    redo_stack: Arc<Vec<String>>,
}

impl Data for AppState {
    fn same(&self, other: &Self) -> bool {
        self.content == other.content &&
        self.current_filepath == other.current_filepath &&
        Arc::ptr_eq(&self.undo_stack, &other.undo_stack) &&
        Arc::ptr_eq(&self.redo_stack, &other.redo_stack)
    }
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            content: String::new(),
            current_filepath: None,
            undo_stack: Arc::new(Vec::new()),
            redo_stack: Arc::new(Vec::new()),
        }
    }

    pub fn save_to_undo(&mut self) {
        Arc::make_mut(&mut self.undo_stack).push(self.content.clone());
        // Clear the redo stack
        //Arc::make_mut(&mut self.redo_stack).clear();
    }

    pub fn save_to_redo(&mut self) {
        Arc::make_mut(&mut self.redo_stack).push(self.content.clone());
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

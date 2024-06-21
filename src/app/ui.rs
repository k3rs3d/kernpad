use crate::app::{controller::AppController, controller::TextChangeController, AppState};
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{commands, Env, Menu, MenuItem, Selector, SysMods, Widget, WidgetExt, WindowId};

pub const SAVE_FILE: Selector<()> = Selector::new("rustpad.save-file");
pub const SAVE_FILE_AS: Selector<()> = Selector::new("rustpad.save-file-as");
pub const LOAD_FILE: Selector<()> = Selector::new("rustpad.load-file");
pub const SHOW_ABOUT: Selector<()> = Selector::new("show-about");

pub fn build_ui() -> impl Widget<AppState> {
    let editor = TextBox::multiline()
        .with_placeholder("Start typing...")
        .expand()
        .controller(TextChangeController)
        .lens(AppState::content); 
    
    let status_label = Label::new(|data: &AppState, _env: &_| match &data.current_filepath {
        Some(file) => format!("File: {}", file),
        None => "No file loaded".to_string(),
    })
    .padding(5.0)
    .expand_width();

    Flex::column()
        .with_flex_child(editor, 1.0)
        .with_child(Align::left(status_label))
        .controller(AppController)
}

pub fn help_menu() -> Menu<AppState> {
    Menu::new("Help")
    .entry(MenuItem::new("About")
    .command(SHOW_ABOUT)
    .hotkey(SysMods::Cmd, "i")
)
}

pub fn build_menu(_window_id: Option<WindowId>, _state: &AppState, _env: &Env) -> Menu<AppState> {
    Menu::empty().entry(file_menu()).entry(edit_menu()).entry(help_menu())
}

pub fn file_menu() -> Menu<AppState> {
    Menu::new("File")
        .entry(
            MenuItem::new("Load File")
                .command(LOAD_FILE)
                .hotkey(SysMods::Cmd, "o"),
        )
        .entry(
            MenuItem::new("Save File")
                .command(SAVE_FILE)
                .hotkey(SysMods::Cmd, "s"),
        )
        .entry(
            MenuItem::new("Save File As")
                .command(SAVE_FILE_AS)
                .hotkey(SysMods::CmdShift, "S"),
        )
        .separator()
        .entry(
            MenuItem::new("Quit")
                .command(commands::QUIT_APP)
                .hotkey(SysMods::Cmd, "q"),
        )
}

pub fn edit_menu() -> Menu<AppState> {
    Menu::new("Edit")
        .entry(
            MenuItem::new("Undo")
                .command(commands::UNDO)
                .hotkey(SysMods::Cmd, "z"),
        ) // TODO: UNDO & REDO!!!
        .entry(
            MenuItem::new("Redo")
                .command(commands::REDO)
                .hotkey(SysMods::Cmd, "y"),
        )
        .separator()
        .entry(
            MenuItem::new("Cut")
                .command(commands::CUT)
                .hotkey(SysMods::Cmd, "x"),
        )
        .entry(
            MenuItem::new("Copy")
                .command(commands::COPY)
                .hotkey(SysMods::Cmd, "c"),
        )
        .entry(
            MenuItem::new("Paste")
                .command(commands::PASTE)
                .hotkey(SysMods::Cmd, "v"),
        )
        .entry(
            MenuItem::new("Select All")
                .command(commands::SELECT_ALL)
                .hotkey(SysMods::Cmd, "a"),
        )
}

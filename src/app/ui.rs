use crate::app::{controller::AppController, AppState};
use druid::widget::{Align, Flex, Label, TextBox};
use druid::{Env, Menu, MenuItem, Selector, SysMods, Widget, WidgetExt, WindowId};

pub const SAVE_FILE: Selector<()> = Selector::new("rustpad.save-file");
pub const SAVE_FILE_AS: Selector<()> = Selector::new("rustpad.save-file-as");
pub const LOAD_FILE: Selector<()> = Selector::new("rustpad.load-file");

pub fn build_ui() -> impl Widget<AppState> {
    let editor = TextBox::multiline()
        .with_placeholder("Start typing...")
        .expand()
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

pub fn build_menu(_window_id: Option<WindowId>, _state: &AppState, _env: &Env) -> Menu<AppState> {
        Menu::new("")
            .entry(file_menu())
    }

// TODO: Fix hotkey names appended to menu label (eg "Save FileCtrl+S" instead of just "Save File")
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
                .hotkey(SysMods::CmdShift, "s"),
        )
        .entry(
            MenuItem::new("Quit")
                .command(druid::commands::QUIT_APP)
                .hotkey(SysMods::Cmd, "q"),
        )
}

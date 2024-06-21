use crate::app::{ui::LOAD_FILE, ui::SAVE_FILE, ui::SAVE_FILE_AS, AppState};
use druid::widget::Controller;
use druid::{commands,Env, Event, EventCtx, Widget};
use rfd::FileDialog;
use std::sync::Arc;

pub struct AppController;
impl<W: Widget<AppState>> Controller<AppState, W> for AppController {
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        match event {
            Event::Command(cmd) if cmd.is(commands::UNDO) => {
                if let Some(last_content) = Arc::make_mut(&mut data.undo_stack).pop() {
                    data.save_to_redo();
                    data.content = last_content;
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(commands::REDO) => {
                if let Some(last_undone_content) = Arc::make_mut(&mut data.redo_stack).pop() {
                    data.save_to_undo();
                    data.content = last_undone_content;
                    // BUG: Cursor stays in place when text is added due to redo...
                }
                ctx.set_handled();
            }
            Event::Command(ref cmd) if cmd.is(UNSAVED_CONTENT) => {
                let new_content = cmd.get_unchecked(UNSAVED_CONTENT).clone();
                data.save_to_undo();
                data.content = new_content;
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(SAVE_FILE) => {
                if let Some(file_path) = &data.current_filepath {
                    std::fs::write(file_path, &data.content).expect("Could not save file");
                } else {
                    let options = FileDialog::new();
                    if let Some(path) = options.save_file() {
                        std::fs::write(&path, &data.content).expect("Could not save file");
                        data.current_filepath = Some(path.to_string_lossy().into_owned());
                    }
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(SAVE_FILE_AS) => {
                let options = FileDialog::new();
                if let Some(path) = options.save_file() {
                    std::fs::write(&path, &data.content).expect("Could not save file");
                    data.current_filepath = Some(path.to_string_lossy().into_owned());
                }
                ctx.set_handled();
            }
            Event::Command(cmd) if cmd.is(LOAD_FILE) => {
                let mut options = FileDialog::new();
                if let Some(current_path) = &data.current_filepath {
                    if let Some(parent) = std::path::Path::new(current_path).parent() {
                        options = options.set_directory(parent);
                    }
                }
                if let Some(path) = options.pick_file() {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        data.save_to_undo();
                        data.content = content;
                        data.current_filepath = Some(path.to_string_lossy().into_owned());
                    }
                }
                ctx.set_handled();
            }
            _ => child.event(ctx, event, data, env),
        }
    }
}


// TODO: Refactor
const UNSAVED_CONTENT: druid::Selector<String> = druid::Selector::new("unsaved_content");
pub struct TextChangeController;
impl<W: Widget<String>> Controller<String, W> for TextChangeController {
    fn update(&mut self, child: &mut W, ctx: &mut druid::UpdateCtx, old_data: &String, data: &String, env: &Env) {
        if old_data != data {
            ctx.submit_command(druid::Command::new(UNSAVED_CONTENT, data.clone(), druid::Target::Auto));
        }
        child.update(ctx, old_data, data, env);
    }
}
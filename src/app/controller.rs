use druid::{Env, Event, EventCtx, Widget};
    use druid::widget::Controller;
    use crate::app::{AppState, ui::LOAD_FILE, ui::SAVE_FILE, ui::SAVE_FILE_AS};
use rfd::FileDialog;

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
                    let options = FileDialog::new();
                    if let Some(path) = options.pick_file() {
                        if let Ok(content) = std::fs::read_to_string(&path) {
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
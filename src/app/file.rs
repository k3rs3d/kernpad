use rfd::FileDialog;
use std::fs;

pub fn save_file(content: &str) -> Option<String> {
    let options = FileDialog::new();
    if let Some(path) = options.save_file() {
        fs::write(&path, content).expect("Could not save file");
        return Some(path.to_string_lossy().into_owned());
    }
    None
}

pub fn load_file() -> Option<(String, String)> {
    let options = FileDialog::new();
    if let Some(path) = options.pick_file() {
        if let Ok(content) = fs::read_to_string(&path) {
            return Some((path.to_string_lossy().into_owned(), content));
        }
    }
    None
}
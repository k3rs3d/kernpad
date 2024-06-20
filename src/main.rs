mod app;
use app::{AppState, create_app};

fn main() {
    let initial_state = AppState::new();
    create_app(initial_state);
}
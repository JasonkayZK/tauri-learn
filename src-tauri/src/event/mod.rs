use tauri::App;

use crate::event::emit::register_all_emit;
use crate::event::listener::register_all_listener;

pub mod emit;
pub mod listener;

pub fn register_events(app: &mut App) {
    register_all_listener(app);
    register_all_emit(app);
}

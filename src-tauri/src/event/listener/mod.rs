use tauri::App;

use crate::event::listener::global::listen_global;

pub mod global;

pub fn register_all_listener(app: &mut App) {
    listen_global(app);
}

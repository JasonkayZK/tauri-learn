use tauri::App;

use crate::event::emit::global::emit_global;

pub mod global;

pub fn register_all_emit(app: &mut App) {
    emit_global(app)
}

use crate::event::emit::global::emit_global;
use tauri::App;

pub mod global;

pub fn register_all_emit(app: &mut App) {
    emit_global(app);
}

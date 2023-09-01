use tauri::{Builder, Wry};
use crate::menu::all::register_all_menu;

pub mod all;

pub fn register_menus(app: Builder<Wry>) -> Builder<Wry> {
    register_all_menu(app)
}

use tauri::{App, Builder, Wry};

use crate::window::demo::demo_window;
use crate::window::global::global_menu;
use crate::window::main::main_window;

pub mod demo;
pub mod global;
pub mod main;

pub fn register_global_menu(app: Builder<Wry>) -> Builder<Wry> {
    global_menu(app)
}

pub fn register_all_windows(app: &mut App) {
    demo_window(app);
    main_window(app);
}

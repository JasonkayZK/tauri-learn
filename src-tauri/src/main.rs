// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::command::register_all_commands;
use crate::menu::register_menus;

mod command;
mod event;
mod menu;
mod window;

fn main() {
    let app = tauri::Builder::default();
    let app = register_menus(app);
    let app = register_all_commands(app);
    app.setup(|app| {
        // Don't emit messages when setup, the message may lost!!!!
        event::register_events(app);
        Ok(())
    }).
        run(tauri::generate_context!()).
        expect("error while running tauri application");
}

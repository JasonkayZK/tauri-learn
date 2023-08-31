// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::command::async_command::async_command;
use crate::command::error_handle::error_handle_command;
use crate::command::get_window::get_window_command;
use crate::command::greet::greet;
use crate::event::emit::register_all_emit;
use crate::event::listener::register_all_listener;

mod command;
mod event;

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            register_all_listener(app);
            register_all_emit(app);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet,
            error_handle_command,
            async_command,
            get_window_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

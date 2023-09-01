// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::command::register_all_commands;
use crate::tray::register_tray;
use crate::window::register_global_menu;

mod command;
mod event;
mod window;
mod tray;

fn main() {
    let app = tauri::Builder::default().on_window_event(|event| match event.event() {
        // Keep the Frontend Running in the Background
        tauri::WindowEvent::CloseRequested { api, .. } => {
            event.window().hide().unwrap();
            api.prevent_close();
        }
        _ => {}
    });
    let app = register_tray(app);
    let app = register_global_menu(app);
    let mut app = register_all_commands(app)
        .build(tauri::generate_context!())
        .expect("error while running tauri application");

    window::register_all_windows(&mut app);

    // Don't emit messages when setup, the message may lost!!!!
    event::register_events(&mut app);

    // This will start the app and no other code below this will run.
    app.run(|_app_handle, event| match event {
        // Keep the Backend Running in the Background
        tauri::RunEvent::ExitRequested { api, .. } => {
            api.prevent_exit();
        }
        _ => {}
    });
}

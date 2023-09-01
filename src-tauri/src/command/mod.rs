use tauri::{Builder, Wry};

pub mod async_command;
pub mod error_handle;
pub mod get_window;
pub mod greet;

pub fn register_all_commands(app: Builder<Wry>) -> Builder<Wry> {
    app.invoke_handler(tauri::generate_handler![
            greet::greet,
            error_handle::error_handle_command,
            async_command::async_command,
            get_window::get_window_command,
        ])
}

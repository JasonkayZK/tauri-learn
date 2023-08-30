// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn error_handle_command() -> Result<String, String> {
    // If something fails
    Err("This failed!".into())

    // If it worked
    // Ok("This worked!".into())
}

// Return a Result<String, ()> to bypass the borrowing issue
#[tauri::command]
async fn async_command(value: &str, bound: i32) -> Result<String, ()> {
    // Call another async function and wait for it to finish
    let res = some_async_function(bound).await;

    // Note that the return value must be wrapped in `Ok()` now.
    Ok(format!("{}, res: {}", value, res))
}

async fn some_async_function(bound: i32) -> i32 {
    let mut res = 0;
    for i in 0..=bound {
        res += i;
    }
    res
}

#[tauri::command]
async fn get_window_command(window: tauri::Window) {
    println!("Window: {}", window.label());
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            error_handle_command,
            async_command,
            get_window_command,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
pub async fn get_window_command(window: tauri::Window) {
    println!("Window: {}", window.label());
}

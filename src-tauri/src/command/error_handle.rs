#[tauri::command]
pub fn error_handle_command() -> Result<String, String> {
    // If something fails
    Err("This failed!".into())

    // If it worked
    // Ok("This worked!".into())
}

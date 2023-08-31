use tauri::{App, Manager};

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    message: String,
}

pub fn emit_global(app: &mut App) {
    // emit the `event-name` event to all webview windows on the frontend
    app.emit_all("global-event-front", Payload { message: "Tauri is awesome!".into() }).unwrap();
}

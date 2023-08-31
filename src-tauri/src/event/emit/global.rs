use tauri::{App, Manager};

const EVENT_NAME: &str = "global-event-front";

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    message: String,
}

pub fn emit_global(app: &mut App) {
    // emit the `event-name` event to all webview windows on the frontend
    app.emit_all(
        EVENT_NAME,
        Payload {
            message: "Tauri is good!".into(),
        },
    )
    .unwrap();
}

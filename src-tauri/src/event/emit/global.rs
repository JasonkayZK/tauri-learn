use std::{thread, time};

use tauri::{App, Manager};

const EVENT_NAME: &str = "global-event-front";

#[derive(Clone, serde::Serialize)]
pub struct Payload {
    message: String,
}

pub fn emit_global(app: &mut App) {

    let window = app.get_window("main").unwrap();

    thread::spawn(move || {
        let mut cnt = 0;
        loop {
            // emit the `event-name` event to all webview windows on the frontend
            window.emit_all(
                EVENT_NAME,
                Payload {
                    message: format!("Tauri is good {}!", cnt).into(),
                },
            )
                .unwrap();
            thread::sleep(time::Duration::from_secs(1));
            println!("emit global-event-front ok {}!", cnt);

            cnt += 1;
            if cnt >= 5 {
                break;
            }
        }
    });
}

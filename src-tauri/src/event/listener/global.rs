use tauri::{App, Manager};

const EVENT_NAME: &str = "global-emit";

pub fn listen_global(app: &mut App) {
    // listen to the `event-name` (emitted on any window)
    let _id = app.listen_global(EVENT_NAME, |event| {
        println!("got {:?} with payload {:?}", EVENT_NAME, event.payload());
    });
    // unlisten to the event using the `id` returned on the `listen_global` function
    // a `once_global` API is also exposed on the `App` struct
    // app.unlisten(id);
}

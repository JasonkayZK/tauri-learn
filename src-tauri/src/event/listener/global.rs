use tauri::{App, Manager};

pub fn listen_global(app: &App) {
    // listen to the `event-name` (emitted on any window)
    let _id = app.listen_global("global-click", |event| {
        println!("got event-name with payload {:?}", event.payload());
    });
    // unlisten to the event using the `id` returned on the `listen_global` function
    // a `once_global` API is also exposed on the `App` struct
    // app.unlisten(id);
}

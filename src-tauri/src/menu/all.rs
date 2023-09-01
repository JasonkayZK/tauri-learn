use tauri::{CustomMenuItem, Menu, MenuItem, Submenu};
use tauri::{Builder, Wry};

pub fn register_all_menu(app: Builder<Wry>) -> Builder<Wry> {
    let quit = CustomMenuItem::new("quit".to_string(), "Quit");
    let close = CustomMenuItem::new("close".to_string(), "Close");
    let submenu = Submenu::new("File", Menu::new().add_item(quit).add_item(close));
    let menu = Menu::new()
        .add_native_item(MenuItem::Copy)
        .add_item(CustomMenuItem::new("hide", "Hide"))
        .add_submenu(submenu);

    app.menu(menu).on_menu_event(|event| {
        match event.menu_item_id() {
            "quit" => {
                std::process::exit(0);
            }
            "close" => {
                event.window().close().unwrap();
            }
            _ => {}
        }
    })
}
